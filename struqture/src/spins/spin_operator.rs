// Copyright © 2021-2023 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.

use super::{ToSparseMatrixOperator, ToSparseMatrixSuperOperator};
use crate::fermions::FermionOperator;
use crate::mappings::JordanWignerSpinToFermion;
use crate::spins::{OperateOnSpins, PauliProduct, SpinHamiltonian, SpinIndex};
use crate::{
    CooSparseMatrix, GetValue, OperateOnDensityMatrix, OperateOnState, StruqtureError,
    StruqtureVersionSerializable, SymmetricIndex, MINIMUM_STRUQTURE_VERSION,
};
use num_complex::Complex64;
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Entry, Iter, Keys, Values};
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

/// SpinOperators are combinations of PauliProducts with specific CalculatorComplex coefficients.
///
/// This is a representation of sums of pauli products with weightings, in order to build a full hamiltonian.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::spins::{OperateOnSpins, PauliProduct, SpinOperator};
///
/// let mut so = SpinOperator::new();
///
/// // Representing the hamiltonian $ 1/2 \sigma_0^{x} \sigma_1^{x} + 1/5 \sigma_0^{z} $
/// let pp_0x1x = PauliProduct::new().x(0).x(1);
/// let pp_0z = PauliProduct::new().z(0);
/// so.add_operator_product(pp_0x1x.clone(), CalculatorComplex::from(0.5)).unwrap();
/// so.add_operator_product(pp_0z.clone(), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(so.get(&pp_0x1x), &CalculatorComplex::from(0.5));
/// assert_eq!(so.get(&pp_0z), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "SpinOperatorSerialize")]
#[serde(into = "SpinOperatorSerialize")]
pub struct SpinOperator {
    /// The internal HashMap of PauliProducts and coefficients (CalculatorComplex)
    internal_map: HashMap<PauliProduct, CalculatorComplex>,
}

impl crate::MinSupportedVersion for SpinOperator {}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for SpinOperator {
    fn schema_name() -> String {
        "SpinOperator".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <SpinOperatorSerialize>::json_schema(gen)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
///# SpinOperator
///
/// This is a representation of sums of pauli products with weightings, in order to build a full hamiltonian.
struct SpinOperatorSerialize {
    /// List of all non-zero entries in the SpinOperator in the form (PauliProduct, real part of weight, imaginary part of weight).
    items: Vec<(PauliProduct, CalculatorFloat, CalculatorFloat)>,
    /// Minimum struqture version required to de-serialize object
    _struqture_version: StruqtureVersionSerializable,
}

impl From<SpinOperatorSerialize> for SpinOperator {
    fn from(value: SpinOperatorSerialize) -> Self {
        let new_noise_op: SpinOperator = value
            .items
            .into_iter()
            .map(|(key, real, imag)| (key, CalculatorComplex { re: real, im: imag }))
            .collect();
        new_noise_op
    }
}

impl From<SpinOperator> for SpinOperatorSerialize {
    fn from(value: SpinOperator) -> Self {
        let new_noise_op: Vec<(PauliProduct, CalculatorFloat, CalculatorFloat)> = value
            .into_iter()
            .map(|(key, val)| (key, val.re, val.im))
            .collect();
        let current_version = StruqtureVersionSerializable {
            major_version: MINIMUM_STRUQTURE_VERSION.0,
            minor_version: MINIMUM_STRUQTURE_VERSION.1,
        };
        Self {
            items: new_noise_op,
            _struqture_version: current_version,
        }
    }
}

impl<'a> OperateOnDensityMatrix<'a> for SpinOperator {
    type IteratorType = Iter<'a, Self::Index, Self::Value>;
    type KeyIteratorType = Keys<'a, Self::Index, Self::Value>;
    type ValueIteratorType = Values<'a, Self::Index, Self::Value>;
    type Value = CalculatorComplex;
    type Index = PauliProduct;

    // From trait
    fn get(&self, key: &Self::Index) -> &Self::Value {
        match self.internal_map.get(key) {
            Some(value) => value,
            None => &CalculatorComplex::ZERO,
        }
    }

    // From trait
    fn iter(&'a self) -> Self::IteratorType {
        self.internal_map.iter()
    }

    // From trait
    fn keys(&'a self) -> Self::KeyIteratorType {
        self.internal_map.keys()
    }

    // From trait
    fn values(&'a self) -> Self::ValueIteratorType {
        self.internal_map.values()
    }

    // From trait
    fn remove(&mut self, key: &Self::Index) -> Option<Self::Value> {
        self.internal_map.remove(key)
    }

    // From trait
    fn empty_clone(&self, capacity: Option<usize>) -> Self {
        match capacity {
            Some(cap) => Self::with_capacity(cap),
            None => Self::new(),
        }
    }

    /// Overwrites an existing entry or sets a new entry in the SpinOperator with the given (PauliProduct key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The PauliProduct key to set in the SpinOperator.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the SpinOperator.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(CalculatorComplex))` - The key existed, this is the value it had before it was set with the value input.
    /// * `Ok(None)` - The key did not exist, it has been set with its corresponding value.
    fn set(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<Option<Self::Value>, StruqtureError> {
        if value != CalculatorComplex::ZERO {
            Ok(self.internal_map.insert(key, value))
        } else {
            match self.internal_map.entry(key) {
                Entry::Occupied(val) => Ok(Some(val.remove())),
                Entry::Vacant(_) => Ok(None),
            }
        }
    }
}

impl<'a> OperateOnState<'a> for SpinOperator {
    // From trait
    fn hermitian_conjugate(&self) -> Self {
        let mut new_operator = Self::with_capacity(self.len());
        for (pauli_product, value) in self.iter() {
            let (new_boson_product, prefactor) = pauli_product.hermitian_conjugate();
            new_operator
                .add_operator_product(new_boson_product, value.conj() * prefactor)
                .expect("Internal bug in add_operator_product");
        }
        new_operator
    }
}

impl<'a> OperateOnSpins<'a> for SpinOperator {
    // From trait
    fn current_number_spins(&self) -> usize {
        let mut max_mode: usize = 0;
        if !self.internal_map.is_empty() {
            for key in self.internal_map.keys() {
                if key.current_number_spins() > max_mode {
                    max_mode = key.current_number_spins()
                }
            }
        }
        max_mode
    }

    /// Gets the maximum index of the SpinOperator.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of spins in the SpinOperator.
    fn number_spins(&self) -> usize {
        self.current_number_spins()
    }
}

impl<'a> ToSparseMatrixOperator<'a> for SpinOperator {}
impl<'a> ToSparseMatrixSuperOperator<'a> for SpinOperator {
    // From trait
    fn sparse_matrix_superoperator_entries_on_row(
        &'a self,
        row: usize,
        number_spins: usize,
    ) -> Result<std::collections::HashMap<usize, Complex64>, StruqtureError> {
        <Self as ToSparseMatrixOperator>::sparse_matrix_superoperator_entries_on_row(
            self,
            row,
            number_spins,
        )
    }

    // From trait
    fn unitary_sparse_matrix_coo(&'a self) -> Result<CooSparseMatrix, StruqtureError> {
        self.sparse_matrix_coo(None)
    }

    // From trait
    fn sparse_lindblad_entries(
        &'a self,
    ) -> Result<Vec<(CooSparseMatrix, CooSparseMatrix, Complex64)>, StruqtureError> {
        let rate = Complex64::default();
        let left: CooSparseMatrix = (vec![], (vec![], vec![]));
        let right: CooSparseMatrix = (vec![], (vec![], vec![]));
        Ok(vec![(left, right, rate)])
    }
}

/// Implements the default function (Default trait) of SpinOperator (an empty SpinOperator).
///
impl Default for SpinOperator {
    fn default() -> Self {
        Self::new()
    }
}

/// Functions for the SpinOperator
///
impl SpinOperator {
    /// Creates a new SpinOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) SpinOperator.
    pub fn new() -> Self {
        SpinOperator {
            internal_map: HashMap::new(),
        }
    }

    /// Creates a new SpinOperator with pre-allocated capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The pre-allocated capacity of the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) SpinOperator.
    pub fn with_capacity(capacity: usize) -> Self {
        SpinOperator {
            internal_map: HashMap::with_capacity(capacity),
        }
    }

    /// Separate self into an operator with the terms of given number of spins and an operator with the remaining operations
    ///
    /// # Arguments
    ///
    /// * `number_spins` - Number of spins to filter for in the keys.
    ///
    /// # Returns
    ///
    /// `Ok((separated, remainder))` - Operator with the noise terms where number_spins matches the number of spins the operator product acts on and Operator with all other contributions.
    pub fn separate_into_n_terms(
        &self,
        number_spins: usize,
    ) -> Result<(Self, Self), StruqtureError> {
        let mut separated = Self::default();
        let mut remainder = Self::default();
        for (prod, val) in self.iter() {
            if prod.len() == number_spins {
                separated.add_operator_product(prod.clone(), val.clone())?;
            } else {
                remainder.add_operator_product(prod.clone(), val.clone())?;
            }
        }
        Ok((separated, remainder))
    }
}

impl From<SpinHamiltonian> for SpinOperator {
    /// Converts a SpinHamiltonian into a SpinOperator.
    ///
    /// # Arguments
    ///
    /// * `hamiltonian` - The SpinHamiltonian to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The SpinHamiltonian converted into a SpinOperator.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from(hamiltonian: SpinHamiltonian) -> Self {
        let mut internal = SpinOperator::new();
        for (key, value) in hamiltonian.into_iter() {
            let bp = PauliProduct::get_key(&key);
            internal
                .add_operator_product(bp, CalculatorComplex::from(value))
                .expect("Internal bug in add_operator_product");
        }
        internal
    }
}

/// Implements the negative sign function of SpinOperator.
///
impl ops::Neg for SpinOperator {
    type Output = SpinOperator;
    /// Implement minus sign for SpinOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The SpinOperator * -1.
    fn neg(self) -> Self {
        let mut internal = HashMap::with_capacity(self.len());
        for (key, val) in self {
            internal.insert(key.clone(), val.neg());
        }
        SpinOperator {
            internal_map: internal,
        }
    }
}

/// Implements the plus function of SpinOperator by SpinOperator.
///
impl<T, V> ops::Add<T> for SpinOperator
where
    T: IntoIterator<Item = (PauliProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implements `+` (add) for two SpinOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The SpinOperator to be added.
    ///
    /// # Returns
    ///
    /// * `Self` - The two SpinOperators added together.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn add(mut self, other: T) -> Self {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value))
                .expect("Internal bug in add_operator_product");
        }
        self
    }
}

/// Implements the minus function of SpinOperator by SpinOperator.
///
impl<T, V> ops::Sub<T> for SpinOperator
where
    T: IntoIterator<Item = (PauliProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implements `-` (subtract) for two SpinOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The SpinOperator to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Self` - The two SpinOperators subtracted.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn sub(mut self, other: T) -> Self {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value) * -1.0)
                .expect("Internal bug in add_operator_product");
        }
        self
    }
}

/// Implements the multiplication function of SpinOperator by CalculatorComplex/CalculatorFloat.
///
impl<T> ops::Mul<T> for SpinOperator
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for SpinOperator and CalculatorComplex/CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex or CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The SpinOperator multiplied by the CalculatorComplex/CalculatorFloat.
    fn mul(self, other: T) -> Self {
        let other_cc = Into::<CalculatorComplex>::into(other);
        let mut internal = HashMap::with_capacity(self.len());
        for (key, val) in self {
            internal.insert(key, val * other_cc.clone());
        }
        SpinOperator {
            internal_map: internal,
        }
    }
}

/// Implements the multiplication function of SpinOperator by SpinOperator.
///
impl ops::Mul<SpinOperator> for SpinOperator {
    type Output = Self;
    /// Implement `*` for SpinOperator and SpinOperator.
    ///
    /// # Arguments
    ///
    /// * `other` - The SpinOperator to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - The two SpinOperators multiplied.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn mul(self, other: SpinOperator) -> Self {
        let mut spin_op = SpinOperator::with_capacity(self.len() * other.len());
        for (pps, vals) in self {
            for (ppo, valo) in other.iter() {
                let (ppp, coefficient) = pps.clone() * ppo.clone();
                let coefficient =
                    Into::<CalculatorComplex>::into(valo) * coefficient * vals.clone();
                spin_op
                    .add_operator_product(ppp, coefficient)
                    .expect("Internal bug in add_operator_product");
            }
        }
        spin_op
    }
}

/// Implements the multiplication function of SpinOperator by PauliProduct.
///
impl ops::Mul<PauliProduct> for SpinOperator {
    type Output = Self;
    /// Implement `*` for SpinOperator and PauliProduct.
    ///
    /// # Arguments
    ///
    /// * `other` - PauliProduct
    ///
    /// # Returns
    ///
    /// * `Self` - The SpinOperator multiplied by the PauliProduct.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn mul(self, ppo: PauliProduct) -> Self {
        let mut spin_op = SpinOperator::with_capacity(self.len());
        for (pps, vals) in self {
            let (ppp, coefficient) = pps.clone() * ppo.clone();
            let coefficient = CalculatorComplex::from(coefficient) * vals.clone();
            spin_op
                .add_operator_product(ppp, coefficient)
                .expect("Internal bug in add_operator_product");
        }
        spin_op
    }
}

/// Implements the multiplication function of PauliProduct by SpinOperator.
///
impl ops::Mul<SpinOperator> for PauliProduct {
    type Output = SpinOperator;
    /// Implement `*` for PauliProduct and SpinOperator.
    ///
    /// # Arguments
    ///
    /// * `other` - The SpinOperator to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - A SpinOperator derived from the PauliProduct, SpinOperator multiplication.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn mul(self, other: SpinOperator) -> SpinOperator {
        let mut spin_op = SpinOperator::with_capacity(other.len());
        for (ppo, valo) in other.iter() {
            let (ppp, coefficient) = self.clone() * ppo.clone();
            let coefficient = valo.clone() * CalculatorComplex::from(coefficient);
            spin_op
                .add_operator_product(ppp, coefficient)
                .expect("Internal bug in add_operator_product");
        }
        spin_op
    }
}

/// Implements the into_iter function (IntoIterator trait) of SpinOperator.
///
impl IntoIterator for SpinOperator {
    type Item = (PauliProduct, CalculatorComplex);
    type IntoIter = std::collections::hash_map::IntoIter<PauliProduct, CalculatorComplex>;
    /// Returns the SpinOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The SpinOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference SpinOperator.
///
impl<'a> IntoIterator for &'a SpinOperator {
    type Item = (&'a PauliProduct, &'a CalculatorComplex);
    type IntoIter = Iter<'a, PauliProduct, CalculatorComplex>;

    /// Returns the reference SpinOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference SpinOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of SpinOperator.
///
impl FromIterator<(PauliProduct, CalculatorComplex)> for SpinOperator {
    /// Returns the object in SpinOperator form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the SpinOperator.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in SpinOperator form.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from_iter<I: IntoIterator<Item = (PauliProduct, CalculatorComplex)>>(iter: I) -> Self {
        let mut so = SpinOperator::new();
        for (pp, cc) in iter {
            so.add_operator_product(pp, cc)
                .expect("Internal bug in add_operator_product");
        }
        so
    }
}

/// Implements the extend function (Extend trait) of SpinOperator.
///
impl Extend<(PauliProduct, CalculatorComplex)> for SpinOperator {
    /// Extends the SpinOperator by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the SpinOperator.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn extend<I: IntoIterator<Item = (PauliProduct, CalculatorComplex)>>(&mut self, iter: I) {
        for (pp, cc) in iter {
            self.add_operator_product(pp, cc)
                .expect("Internal bug in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of SpinOperator.
///
impl fmt::Display for SpinOperator {
    /// Formats the SpinOperator using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted SpinOperator.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "SpinOperator{\n".to_string();
        for (key, val) in self.iter() {
            writeln!(output, "{}: {},", key, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}

impl JordanWignerSpinToFermion for SpinOperator {
    type Output = FermionOperator;

    /// Implements JordanWignerSpinToFermion for a SpinOperator.
    ///
    /// The convention used is that |0> represents an empty fermionic state (spin-orbital),
    /// and |1> represents an occupied fermionic state.
    ///
    /// # Returns
    ///
    /// `FermionOperator` - The fermionic operator that results from the transformation.
    fn jordan_wigner(&self) -> Self::Output {
        let mut out = FermionOperator::new();
        for pp in self.keys() {
            out = out + pp.jordan_wigner() * self.get(pp);
        }
        out
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_test::{assert_tokens, Configure, Token};

    // Test the Clone and PartialEq traits of SpinOperator
    #[test]
    fn so_from_sos() {
        let pp: PauliProduct = PauliProduct::new().z(0);
        let sos = SpinOperatorSerialize {
            items: vec![(pp.clone(), 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        let mut so = SpinOperator::new();
        so.set(pp, CalculatorComplex::from(0.5)).unwrap();

        assert_eq!(SpinOperator::from(sos.clone()), so);
        assert_eq!(SpinOperatorSerialize::from(so), sos);
    }
    // Test the Clone and PartialEq traits of SpinOperator
    #[test]
    fn clone_partial_eq() {
        let pp: PauliProduct = PauliProduct::new().z(0);
        let sos = SpinOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        // Test Clone trait
        assert_eq!(sos.clone(), sos);

        // Test PartialEq trait
        let pp_1: PauliProduct = PauliProduct::new().z(0);
        let sos_1 = SpinOperatorSerialize {
            items: vec![(pp_1, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        let pp_2: PauliProduct = PauliProduct::new().z(2);
        let sos_2 = SpinOperatorSerialize {
            items: vec![(pp_2, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        assert!(sos_1 == sos);
        assert!(sos == sos_1);
        assert!(sos_2 != sos);
        assert!(sos != sos_2);
    }

    // Test the Debug trait of SpinOperator
    #[test]
    fn debug() {
        let pp: PauliProduct = PauliProduct::new().z(0);
        let sos = SpinOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        assert_eq!(
            format!("{:?}", sos),
            "SpinOperatorSerialize { items: [(PauliProduct { items: [(0, Z)] }, Float(0.5), Float(0.0))], _struqture_version: StruqtureVersionSerializable { major_version: 1, minor_version: 0 } }"
        );
    }

    /// Test SpinOperator Serialization and Deserialization traits (readable)
    #[test]
    fn serde_readable() {
        let pp = PauliProduct::new().x(0);
        let sos = SpinOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        assert_tokens(
            &sos.readable(),
            &[
                Token::Struct {
                    name: "SpinOperatorSerialize",
                    len: 2,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 3 },
                Token::Str("0X"),
                Token::F64(0.5),
                Token::F64(0.0),
                Token::TupleEnd,
                Token::SeqEnd,
                Token::Str("_struqture_version"),
                Token::Struct {
                    name: "StruqtureVersionSerializable",
                    len: 2,
                },
                Token::Str("major_version"),
                Token::U32(1),
                Token::Str("minor_version"),
                Token::U32(0),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    /// Test SpinOperator Serialization and Deserialization traits (compact)
    #[test]
    fn serde_compact() {
        let pp = PauliProduct::new().x(0);
        let sos = SpinOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        assert_tokens(
            &sos.compact(),
            &[
                Token::Struct {
                    name: "SpinOperatorSerialize",
                    len: 2,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 3 },
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 2 },
                Token::U64(0),
                Token::UnitVariant {
                    name: "SingleSpinOperator",
                    variant: "X",
                },
                Token::TupleEnd,
                Token::SeqEnd,
                Token::NewtypeVariant {
                    name: "CalculatorFloat",
                    variant: "Float",
                },
                Token::F64(0.5),
                Token::NewtypeVariant {
                    name: "CalculatorFloat",
                    variant: "Float",
                },
                Token::F64(0.0),
                Token::TupleEnd,
                Token::SeqEnd,
                Token::Str("_struqture_version"),
                Token::Struct {
                    name: "StruqtureVersionSerializable",
                    len: 2,
                },
                Token::Str("major_version"),
                Token::U32(1),
                Token::Str("minor_version"),
                Token::U32(0),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
