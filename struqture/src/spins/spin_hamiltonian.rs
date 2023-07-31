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

use super::{OperateOnSpins, SpinOperator, ToSparseMatrixOperator, ToSparseMatrixSuperOperator};
use crate::fermions::{FermionHamiltonian, FermionOperator};
use crate::mappings::JordanWignerSpinToFermion;
use crate::prelude::*;
use crate::spins::{HermitianOperateOnSpins, PauliProduct, SpinIndex};
use crate::{
    CooSparseMatrix, GetValue, OperateOnDensityMatrix, OperateOnState, StruqtureError,
    StruqtureVersionSerializable, MINIMUM_STRUQTURE_VERSION,
};
use num_complex::Complex64;
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Entry, Iter, Keys, Values};
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

/// SpinHamiltonians are combinations of PauliProducts with specific CalculatorFloat coefficients.
///
/// This is a representation of sums of pauli products with weightings, in order to build a full hamiltonian.
/// SpinHamiltonian is the hermitian equivalent of SpinOperator.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorFloat;
/// use struqture::spins::{HermitianOperateOnSpins, PauliProduct, SpinHamiltonian};
///
/// let mut sh = SpinHamiltonian::new();
///
/// // Representing the hamiltonian $ 1/2 \sigma_0^{x} \sigma_1^{x} + 1/5 \sigma_0^{z} $
/// let pp_0x1x = PauliProduct::new().x(0).x(1);
/// let pp_0z = PauliProduct::new().z(0);
/// sh.add_operator_product(pp_0x1x.clone(), CalculatorFloat::from(0.5)).unwrap();
/// sh.add_operator_product(pp_0z.clone(), CalculatorFloat::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(sh.get(&pp_0x1x), &CalculatorFloat::from(0.5));
/// assert_eq!(sh.get(&pp_0z), &CalculatorFloat::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "SpinHamiltonianSerialize")]
#[serde(into = "SpinHamiltonianSerialize")]
pub struct SpinHamiltonian {
    /// The internal HashMap of PauliProducts and coefficients (CalculatorFloat)
    internal_map: HashMap<PauliProduct, CalculatorFloat>,
}

impl crate::MinSupportedVersion for SpinHamiltonian {}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for SpinHamiltonian {
    fn schema_name() -> String {
        "struqture::spins::SpinHamiltonian".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <SpinHamiltonianSerialize>::json_schema(gen)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
/// # SpinHamiltonian
/// SpinHamiltonians are combinations of PauliProducts with specific CalculatorFloat coefficients.
///
/// This is a representation of sums of pauli products with weightings, in order to build a full hamiltonian.
/// SpinHamiltonian is the hermitian equivalent of SpinOperator.
struct SpinHamiltonianSerialize {
    /// List of all non-zero entries in the SpinHamiltonian in the form (PauliProduct, real weight).
    items: Vec<(PauliProduct, CalculatorFloat)>,
    _struqture_version: StruqtureVersionSerializable,
}

impl From<SpinHamiltonianSerialize> for SpinHamiltonian {
    fn from(value: SpinHamiltonianSerialize) -> Self {
        let new_noise_op: SpinHamiltonian = value.items.into_iter().collect();
        new_noise_op
    }
}

impl From<SpinHamiltonian> for SpinHamiltonianSerialize {
    fn from(value: SpinHamiltonian) -> Self {
        let new_noise_op: Vec<(PauliProduct, CalculatorFloat)> =
            value.into_iter().map(|(key, val)| (key, val)).collect();
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

impl<'a> OperateOnDensityMatrix<'a> for SpinHamiltonian {
    type Index = PauliProduct;
    type Value = CalculatorFloat;
    type IteratorType = Iter<'a, Self::Index, Self::Value>;
    type KeyIteratorType = Keys<'a, Self::Index, Self::Value>;
    type ValueIteratorType = Values<'a, Self::Index, Self::Value>;

    // From trait
    fn get(&self, key: &Self::Index) -> &Self::Value {
        match self.internal_map.get(key) {
            Some(value) => value,
            None => &CalculatorFloat::ZERO,
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

    /// Overwrites an existing entry or sets a new entry in the SpinHamiltonian with the given (PauliProduct key, CalculatorFloat value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The PauliProduct key to set in the SpinHamiltonian.
    /// * `value` - The corresponding CalculatorFloat value to set for the key in the SpinHamiltonian.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(CalculatorFloat))` - The key existed, this is the value it had before it was set with the value input.
    /// * `Ok(None)` - The key did not exist, it has been set with its corresponding value.
    fn set(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<Option<Self::Value>, StruqtureError> {
        if value != CalculatorFloat::ZERO {
            Ok(self.internal_map.insert(key, value))
        } else {
            match self.internal_map.entry(key) {
                Entry::Occupied(val) => Ok(Some(val.remove())),
                Entry::Vacant(_) => Ok(None),
            }
        }
    }
}

impl<'a> OperateOnState<'a> for SpinHamiltonian {
    // From trait
    fn hermitian_conjugate(&self) -> Self {
        self.clone()
    }
}

impl<'a> OperateOnSpins<'a> for SpinHamiltonian {
    /// Gets the maximum index of the SpinHamiltonian.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of spins in the SpinHamiltonian.
    fn number_spins(&self) -> usize {
        self.current_number_spins()
    }

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
}

impl<'a> HermitianOperateOnSpins<'a> for SpinHamiltonian {}

impl<'a> ToSparseMatrixOperator<'a> for SpinHamiltonian {}
impl<'a> ToSparseMatrixSuperOperator<'a> for SpinHamiltonian {
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
    fn unitary_sparse_matrix_coo(&'a self) -> Result<crate::CooSparseMatrix, StruqtureError> {
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

/// Implements the default function (Default trait) of SpinHamiltonian (an empty SpinHamiltonian).
///
impl Default for SpinHamiltonian {
    fn default() -> Self {
        Self::new()
    }
}

/// Functions for the SpinHamiltonian
///
impl SpinHamiltonian {
    /// Creates a new SpinHamiltonian.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) SpinHamiltonian.
    pub fn new() -> Self {
        SpinHamiltonian {
            internal_map: HashMap::new(),
        }
    }

    /// Creates a new SpinHamiltonian with pre-allocated capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The pre-allocated capacity of the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) SpinHamiltonian.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
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

impl TryFrom<SpinOperator> for SpinHamiltonian {
    type Error = StruqtureError;
    /// Tries to convert a SpinOperator into a SpinHamiltonian.
    ///
    /// # Arguments
    ///
    /// * `hamiltonian` - The SpinOperator to try to convert.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The SpinOperator converted into a SpinHamiltonian.
    /// * `Err(StruqtureError::NonHermitianOperator)` - Key is naturally hermitian (on-diagonal term), but its corresponding value is not real.
    fn try_from(hamiltonian: SpinOperator) -> Result<Self, StruqtureError> {
        let mut internal = SpinHamiltonian::new();
        for (key, value) in hamiltonian.into_iter() {
            if value.im != CalculatorFloat::ZERO {
                return Err(StruqtureError::NonHermitianOperator {});
            } else {
                let pp = PauliProduct::get_key(&key);
                internal.add_operator_product(pp, value.re)?;
            }
        }
        Ok(internal)
    }
}

/// Implements the negative sign function of SpinOperator.
///
impl ops::Neg for SpinHamiltonian {
    type Output = SpinHamiltonian;
    /// Implement minus sign for SpinHamiltonian.
    ///
    /// # Returns
    ///
    /// * `Self` - The SpinHamiltonian * -1.
    fn neg(self) -> Self {
        let mut internal = self.internal_map.clone();
        for key in self.keys() {
            internal.insert(key.clone(), internal[key].clone() * -1.0);
        }
        SpinHamiltonian {
            internal_map: internal,
        }
    }
}

/// Implements the plus function of SpinHamiltonian by SpinHamiltonian.
///
impl<T, V> ops::Add<T> for SpinHamiltonian
where
    T: IntoIterator<Item = (PauliProduct, V)>,
    V: Into<CalculatorFloat>,
{
    type Output = Self;
    /// Implements `+` (add) for two SpinHamiltonians.
    ///
    /// # Arguments
    ///
    /// * `other` - The SpinHamiltonian to be added.
    ///
    /// # Returns
    ///
    /// * `Self` - The two SpinHamiltonians added together.
    ///
    /// # Panics
    ///
    /// * Internal bug in add_operator_product.
    fn add(mut self, other: T) -> Self {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorFloat>::into(value))
                .expect("Internal bug in add_operator_product");
        }
        self
    }
}

/// Implements the minus function of SpinHamiltonian by SpinHamiltonian.
///
impl<T, V> ops::Sub<T> for SpinHamiltonian
where
    T: IntoIterator<Item = (PauliProduct, V)>,
    V: Into<CalculatorFloat>,
{
    type Output = Self;
    /// Implements `-` (subtract) for two SpinHamiltonians.
    ///
    /// # Arguments
    ///
    /// * `other` - The SpinHamiltonian to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Self` - The two SpinHamiltonians subtracted.
    ///
    /// # Panics
    ///
    /// * Internal bug in add_operator_product.
    fn sub(mut self, other: T) -> Self {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorFloat>::into(value) * -1.0)
                .expect("Internal bug in add_operator_product");
        }
        self
    }
}

/// Implements the multiplication function of SpinHamiltonian by CalculatorFloat.
///
impl ops::Mul<CalculatorFloat> for SpinHamiltonian {
    type Output = Self;
    /// Implement `*` for SpinHamiltonian and CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The SpinHamiltonian multiplied by the CalculatorFloat.
    fn mul(self, other: CalculatorFloat) -> Self {
        let mut internal = self.internal_map.clone();
        for key in self.keys() {
            internal.insert(key.clone(), internal[key].clone() * other.clone());
        }
        SpinHamiltonian {
            internal_map: internal,
        }
    }
}

/// Implements the multiplication function of SpinHamiltonian by CalculatorComplex.
///
impl ops::Mul<CalculatorComplex> for SpinHamiltonian {
    type Output = SpinOperator;
    /// Implement `*` for SpinHamiltonian and CalculatorComplex.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex by which to multiply.
    ///
    /// # Returns
    ///
    /// * `SpinOperator` - The SpinHamiltonian multiplied by the CalculatorFloat.
    ///
    /// # Panics
    ///
    /// * Internal bug in set.
    fn mul(self, other: CalculatorComplex) -> Self::Output {
        let mut new_out = SpinOperator::with_capacity(self.len());
        for (key, val) in self {
            new_out
                .set(key, other.clone() * val)
                .expect("Internal bug in set");
        }
        new_out
    }
}

/// Implement `*` for SpinHamiltonian and SpinHamiltonian.
///
impl ops::Mul<SpinHamiltonian> for SpinHamiltonian {
    type Output = SpinOperator;
    /// Implement `*` for SpinHamiltonian and SpinHamiltonian.
    ///
    /// # Arguments
    ///
    /// * `other` - The SpinHamiltonian to multiply by.
    ///
    /// # Returns
    ///
    /// * `SpinOperator` - The two SpinHamiltonians multiplied.
    ///
    /// # Panics
    ///
    /// * Internal bug in add_operator_product.
    fn mul(self, other: SpinHamiltonian) -> Self::Output {
        let mut spin_op = SpinOperator::with_capacity(self.len() * other.len());
        for (pps, vals) in self {
            for (ppo, valo) in other.iter() {
                let (ppp, coefficient) = pps.clone() * ppo.clone();
                let coefficient =
                    Into::<CalculatorComplex>::into(valo) * vals.clone() * coefficient;
                spin_op
                    .add_operator_product(ppp, coefficient)
                    .expect("Internal bug in add_operator_product");
            }
        }
        spin_op
    }
}

/// Implements the into_iter function (IntoIterator trait) of SpinHamiltonian.
///
impl IntoIterator for SpinHamiltonian {
    type Item = (PauliProduct, CalculatorFloat);
    type IntoIter = std::collections::hash_map::IntoIter<PauliProduct, CalculatorFloat>;
    /// Returns the SpinHamiltonian in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The SpinHamiltonian in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference SpinHamiltonian.
///
impl<'a> IntoIterator for &'a SpinHamiltonian {
    type Item = (&'a PauliProduct, &'a CalculatorFloat);
    type IntoIter = Iter<'a, PauliProduct, CalculatorFloat>;

    /// Returns the reference SpinHamiltonian in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference SpinHamiltonian in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of SpinHamiltonian.
///
impl FromIterator<(PauliProduct, CalculatorFloat)> for SpinHamiltonian {
    /// Returns the object in SpinHamiltonian form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the SpinHamiltonian.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in SpinHamiltonian form.
    ///
    /// # Panics
    ///
    /// * Internal bug in add_operator_product.
    fn from_iter<I: IntoIterator<Item = (PauliProduct, CalculatorFloat)>>(iter: I) -> Self {
        let mut so = SpinHamiltonian::new();
        for (pp, cc) in iter {
            so.add_operator_product(pp, cc)
                .expect("Internal bug in add_operator_product");
        }
        so
    }
}

/// Implements the extend function (Extend trait) of SpinHamiltonian.
///
impl Extend<(PauliProduct, CalculatorFloat)> for SpinHamiltonian {
    /// Extends the SpinHamiltonian by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the SpinHamiltonian.
    ///
    /// # Panics
    ///
    /// * Internal bug in add_operator_product.
    fn extend<I: IntoIterator<Item = (PauliProduct, CalculatorFloat)>>(&mut self, iter: I) {
        for (pp, cc) in iter {
            self.add_operator_product(pp, cc)
                .expect("Internal bug in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of SpinHamiltonian.
///
impl fmt::Display for SpinHamiltonian {
    /// Formats the SpinHamiltonian using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted SpinHamiltonian.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "SpinHamiltonian{\n".to_string();
        for (key, val) in self.iter() {
            writeln!(output, "{}: {},", key, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}

impl JordanWignerSpinToFermion for SpinHamiltonian {
    type Output = FermionHamiltonian;

    /// Implements JordanWignerSpinToFermion for a SpinHamiltonian.
    ///
    /// The convention used is that |0> represents an empty fermionic state (spin-orbital),
    /// and |1> represents an occupied fermionic state.
    ///
    /// # Returns
    ///
    /// `FermionHamiltonian` - The fermionic Hamiltonian that results from the transformation.
    ///
    /// # Panics
    ///
    /// * Failed conversion of FermionOperator into FermionHamiltonian. Internal bug in jordan_wigner().
    fn jordan_wigner(&self) -> Self::Output {
        let mut out = FermionOperator::new();
        for pp in self.keys() {
            let mut new_term = pp.jordan_wigner();
            new_term = new_term * self.get(pp);
            out = out + new_term;
        }
        let filtered_fermion_operator = FermionOperator::from_iter(out.into_iter().filter(|x| {
            x.0.is_natural_hermitian() || x.0.creators().min() < x.0.annihilators().min()
        }));
        FermionHamiltonian::try_from(filtered_fermion_operator)
            .expect("Failed to convert FermionOperator into FermionHamiltonian.")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_test::{assert_tokens, Configure, Token};

    // Test the Clone and PartialEq traits of SpinHamiltonian
    #[test]
    fn sh_from_shs() {
        let pp: PauliProduct = PauliProduct::new().z(0);
        let shs = SpinHamiltonianSerialize {
            items: vec![(pp.clone(), 0.5.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        let mut sh = SpinHamiltonian::new();
        sh.set(pp, CalculatorFloat::from(0.5)).unwrap();

        assert_eq!(SpinHamiltonian::from(shs.clone()), sh);
        assert_eq!(SpinHamiltonianSerialize::from(sh), shs);
    }
    // Test the Clone and PartialEq traits of SpinHamiltonian
    #[test]
    fn clone_partial_eq() {
        let pp: PauliProduct = PauliProduct::new().z(0);
        let shs = SpinHamiltonianSerialize {
            items: vec![(pp, 0.5.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        // Test Clone trait
        assert_eq!(shs.clone(), shs);

        // Test PartialEq trait
        let pp_1: PauliProduct = PauliProduct::new().z(0);
        let shs_1 = SpinHamiltonianSerialize {
            items: vec![(pp_1, 0.5.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        let pp_2: PauliProduct = PauliProduct::new().z(2);
        let shs_2 = SpinHamiltonianSerialize {
            items: vec![(pp_2, 0.5.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        assert!(shs_1 == shs);
        assert!(shs == shs_1);
        assert!(shs_2 != shs);
        assert!(shs != shs_2);
    }

    // Test the Debug trait of SpinHamiltonian
    #[test]
    fn debug() {
        let pp: PauliProduct = PauliProduct::new().z(0);
        let shs = SpinHamiltonianSerialize {
            items: vec![(pp, 0.5.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        assert_eq!(
            format!("{:?}", shs),
            "SpinHamiltonianSerialize { items: [(PauliProduct { items: [(0, Z)] }, Float(0.5))], _struqture_version: StruqtureVersionSerializable { major_version: 1, minor_version: 0 } }"
        );
    }

    /// Test SpinHamiltonian Serialization and Deserialization traits (readable)
    #[test]
    fn serde_readable() {
        let pp = PauliProduct::new().x(0);
        let shs = SpinHamiltonianSerialize {
            items: vec![(pp, 0.5.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        assert_tokens(
            &shs.readable(),
            &[
                Token::Struct {
                    name: "SpinHamiltonianSerialize",
                    len: 2,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 2 },
                Token::Str("0X"),
                Token::F64(0.5),
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

    /// Test SpinHamiltonian Serialization and Deserialization traits (compact)
    #[test]
    fn serde_compact() {
        let pp = PauliProduct::new().x(0);
        let shs = SpinHamiltonianSerialize {
            items: vec![(pp, 0.5.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        assert_tokens(
            &shs.compact(),
            &[
                Token::Struct {
                    name: "SpinHamiltonianSerialize",
                    len: 2,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 2 },
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
