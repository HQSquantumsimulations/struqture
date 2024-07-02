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
use crate::spins::{OperateOnSpins, PauliProduct, QubitHamiltonian, SpinIndex};
use crate::{
    CooSparseMatrix, GetValue, OperateOnDensityMatrix, OperateOnState, StruqtureError,
    SymmetricIndex,
};
use indexmap::map::{Entry, Iter};
use indexmap::IndexMap;
use num_complex::Complex64;
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

/// QubitOperators are combinations of PauliProducts with specific CalculatorComplex coefficients.
///
/// This is a representation of sums of pauli products with weightings, in order to build a full hamiltonian.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::spins::{OperateOnSpins, PauliProduct, QubitOperator};
///
/// let mut so = QubitOperator::new();
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
#[serde(try_from = "QubitOperatorSerialize")]
#[serde(into = "QubitOperatorSerialize")]
pub struct QubitOperator {
    // The internal HashMap of PauliProducts and coefficients (CalculatorComplex)
    internal_map: IndexMap<PauliProduct, CalculatorComplex>,
}

impl crate::SerializationSupport for QubitOperator {
    fn struqture_type() -> crate::StruqtureType {
        crate::StruqtureType::QubitOperator
    }
}
#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for QubitOperator {
    fn schema_name() -> String {
        "QubitOperator".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <QubitOperatorSerialize>::json_schema(gen)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
///# QubitOperator
///
/// This is a representation of sums of pauli products with weightings, in order to build a full hamiltonian.
struct QubitOperatorSerialize {
    /// List of all non-zero entries in the QubitOperator in the form (PauliProduct, real part of weight, imaginary part of weight).
    items: Vec<(PauliProduct, CalculatorFloat, CalculatorFloat)>,
    /// Minimum struqture version required to de-serialize object
    serialisation_meta: crate::StruqtureSerialisationMeta,
}

impl TryFrom<QubitOperatorSerialize> for QubitOperator {
    type Error = StruqtureError;
    fn try_from(value: QubitOperatorSerialize) -> Result<Self, Self::Error> {
        let target_serialisation_meta =
            <Self as crate::SerializationSupport>::target_serialisation_meta();
        crate::check_can_be_deserialised(&target_serialisation_meta, &value.serialisation_meta)?;
        let new_noise_op: QubitOperator = value
            .items
            .into_iter()
            .map(|(key, real, imag)| (key, CalculatorComplex { re: real, im: imag }))
            .collect();
        Ok(new_noise_op)
    }
}

impl From<QubitOperator> for QubitOperatorSerialize {
    fn from(value: QubitOperator) -> Self {
        let serialisation_meta = crate::SerializationSupport::struqture_serialisation_meta(&value);
        let new_noise_op: Vec<(PauliProduct, CalculatorFloat, CalculatorFloat)> = value
            .into_iter()
            .map(|(key, val)| (key, val.re, val.im))
            .collect();
        Self {
            items: new_noise_op,
            serialisation_meta,
        }
    }
}

impl<'a> OperateOnDensityMatrix<'a> for QubitOperator {
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
    fn iter(&'a self) -> impl ExactSizeIterator<Item = (&'a Self::Index, &'a Self::Value)> {
        self.internal_map.iter()
    }

    // From trait
    fn keys(&'a self) -> impl ExactSizeIterator<Item = &'a Self::Index> {
        self.internal_map.keys()
    }

    // From trait
    fn values(&'a self) -> impl ExactSizeIterator<Item = &'a Self::Value> {
        self.internal_map.values()
    }

    // From trait
    fn remove(&mut self, key: &Self::Index) -> Option<Self::Value> {
        self.internal_map.shift_remove(key)
    }

    // From trait
    fn empty_clone(&self, capacity: Option<usize>) -> Self {
        match capacity {
            Some(cap) => Self::with_capacity(cap),
            None => Self::new(),
        }
    }

    /// Overwrites an existing entry or sets a new entry in the QubitOperator with the given (PauliProduct key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The PauliProduct key to set in the QubitOperator.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the QubitOperator.
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
                Entry::Occupied(val) => Ok(Some(val.shift_remove())),
                Entry::Vacant(_) => Ok(None),
            }
        }
    }
}

impl<'a> OperateOnState<'a> for QubitOperator {
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

impl<'a> OperateOnSpins<'a> for QubitOperator {
    /// Gets the maximum index of the QubitOperator.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of spins in the QubitOperator.
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

impl<'a> ToSparseMatrixOperator<'a> for QubitOperator {}
impl<'a> ToSparseMatrixSuperOperator<'a> for QubitOperator {
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
    fn unitary_sparse_matrix_coo(
        &'a self,
        number_spins: Option<usize>,
    ) -> Result<CooSparseMatrix, StruqtureError> {
        self.sparse_matrix_coo(number_spins)
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

/// Implements the default function (Default trait) of QubitOperator (an empty QubitOperator).
///
impl Default for QubitOperator {
    fn default() -> Self {
        Self::new()
    }
}

/// Functions for the QubitOperator
///
impl QubitOperator {
    /// Creates a new QubitOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) QubitOperator.
    pub fn new() -> Self {
        QubitOperator {
            internal_map: IndexMap::new(),
        }
    }

    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_export")]
    pub fn to_struqture_1(&self) -> Result<struqture_1::spins::SpinSystem, StruqtureError> {
        let mut new_qubit_system = struqture_1::spins::SpinSystem::new(None);
        for (key, val) in self.iter() {
            let one_key = key.to_struqture_1()?;
            let _ = struqture_1::OperateOnDensityMatrix::set(
                &mut new_qubit_system,
                one_key,
                val.clone(),
            );
        }
        Ok(new_qubit_system)
    }

    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_import")]
    pub fn from_struqture_1(
        value: &struqture_1::spins::SpinSystem,
    ) -> Result<Self, StruqtureError> {
        let mut new_qubit_operator = Self::new();
        for (key, val) in struqture_1::OperateOnDensityMatrix::iter(value) {
            let self_key = PauliProduct::from_struqture_1(key)?;
            let _ = new_qubit_operator.set(self_key, val.clone());
        }
        Ok(new_qubit_operator)
    }

    /// Creates a new QubitOperator with pre-allocated capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The pre-allocated capacity of the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) QubitOperator.
    pub fn with_capacity(capacity: usize) -> Self {
        QubitOperator {
            internal_map: IndexMap::with_capacity(capacity),
        }
    }
}

impl From<QubitHamiltonian> for QubitOperator {
    /// Converts a QubitHamiltonian into a QubitOperator.
    ///
    /// # Arguments
    ///
    /// * `hamiltonian` - The QubitHamiltonian to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The QubitHamiltonian converted into a QubitOperator.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from(hamiltonian: QubitHamiltonian) -> Self {
        let mut internal = QubitOperator::new();
        for (key, value) in hamiltonian.into_iter() {
            let bp = PauliProduct::get_key(&key);
            internal
                .add_operator_product(bp, CalculatorComplex::from(value))
                .expect("Internal bug in add_operator_product");
        }
        internal
    }
}

/// Implements the negative sign function of QubitOperator.
///
impl ops::Neg for QubitOperator {
    type Output = QubitOperator;
    /// Implement minus sign for QubitOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The QubitOperator * -1.
    fn neg(self) -> Self {
        let mut internal = IndexMap::with_capacity(self.len());
        for (key, val) in self {
            internal.insert(key.clone(), val.neg());
        }
        QubitOperator {
            internal_map: internal,
        }
    }
}

/// Implements the plus function of QubitOperator by QubitOperator.
///
impl<T, V> ops::Add<T> for QubitOperator
where
    T: IntoIterator<Item = (PauliProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implements `+` (add) for two QubitOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The QubitOperator to be added.
    ///
    /// # Returns
    ///
    /// * `Self` - The two QubitOperators added together.
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

/// Implements the minus function of QubitOperator by QubitOperator.
///
impl<T, V> ops::Sub<T> for QubitOperator
where
    T: IntoIterator<Item = (PauliProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implements `-` (subtract) for two QubitOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The QubitOperator to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Self` - The two QubitOperators subtracted.
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

/// Implements the multiplication function of QubitOperator by CalculatorComplex/CalculatorFloat.
///
impl<T> ops::Mul<T> for QubitOperator
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for QubitOperator and CalculatorComplex/CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex or CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The QubitOperator multiplied by the CalculatorComplex/CalculatorFloat.
    fn mul(self, other: T) -> Self {
        let other_cc = Into::<CalculatorComplex>::into(other);
        let mut internal = self.internal_map.clone();
        for (key, val) in self {
            internal.insert(key, val * other_cc.clone());
        }
        QubitOperator {
            internal_map: internal,
        }
    }
}

/// Implements the multiplication function of QubitOperator by QubitOperator.
///
impl ops::Mul<QubitOperator> for QubitOperator {
    type Output = Self;
    /// Implement `*` for QubitOperator and QubitOperator.
    ///
    /// # Arguments
    ///
    /// * `other` - The QubitOperator to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - The two QubitOperators multiplied.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn mul(self, other: QubitOperator) -> Self {
        let mut qubit_op = QubitOperator::with_capacity(self.len() * other.len());
        for (pps, vals) in self {
            for (ppo, valo) in other.iter() {
                let (ppp, coefficient) = pps.clone() * ppo.clone();
                let coefficient =
                    Into::<CalculatorComplex>::into(valo) * coefficient * vals.clone();
                qubit_op
                    .add_operator_product(ppp, coefficient)
                    .expect("Internal bug in add_operator_product");
            }
        }
        qubit_op
    }
}

/// Implements the multiplication function of QubitOperator by PauliProduct.
///
impl ops::Mul<PauliProduct> for QubitOperator {
    type Output = Self;
    /// Implement `*` for QubitOperator and PauliProduct.
    ///
    /// # Arguments
    ///
    /// * `other` - PauliProduct
    ///
    /// # Returns
    ///
    /// * `Self` - The QubitOperator multiplied by the PauliProduct.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn mul(self, ppo: PauliProduct) -> Self {
        let mut qubit_op = QubitOperator::with_capacity(self.len());
        for (pps, vals) in self {
            let (ppp, coefficient) = pps.clone() * ppo.clone();
            let coefficient = CalculatorComplex::from(coefficient) * vals.clone();
            qubit_op
                .add_operator_product(ppp, coefficient)
                .expect("Internal bug in add_operator_product");
        }
        qubit_op
    }
}

/// Implements the multiplication function of PauliProduct by QubitOperator.
///
impl ops::Mul<QubitOperator> for PauliProduct {
    type Output = QubitOperator;
    /// Implement `*` for PauliProduct and QubitOperator.
    ///
    /// # Arguments
    ///
    /// * `other` - The QubitOperator to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - A QubitOperator derived from the PauliProduct, QubitOperator multiplication.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn mul(self, other: QubitOperator) -> QubitOperator {
        let mut qubit_op = QubitOperator::with_capacity(other.len());
        for (ppo, valo) in other.iter() {
            let (ppp, coefficient) = self.clone() * ppo.clone();
            let coefficient = valo.clone() * CalculatorComplex::from(coefficient);
            qubit_op
                .add_operator_product(ppp, coefficient)
                .expect("Internal bug in add_operator_product");
        }
        qubit_op
    }
}

/// Implements the into_iter function (IntoIterator trait) of QubitOperator.
///
impl IntoIterator for QubitOperator {
    type Item = (PauliProduct, CalculatorComplex);
    type IntoIter = indexmap::map::IntoIter<PauliProduct, CalculatorComplex>;
    /// Returns the QubitOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The QubitOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference QubitOperator.
///
impl<'a> IntoIterator for &'a QubitOperator {
    type Item = (&'a PauliProduct, &'a CalculatorComplex);
    type IntoIter = Iter<'a, PauliProduct, CalculatorComplex>;

    /// Returns the reference QubitOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference QubitOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of QubitOperator.
///
impl FromIterator<(PauliProduct, CalculatorComplex)> for QubitOperator {
    /// Returns the object in QubitOperator form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the QubitOperator.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in QubitOperator form.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from_iter<I: IntoIterator<Item = (PauliProduct, CalculatorComplex)>>(iter: I) -> Self {
        let mut so = QubitOperator::new();
        for (pp, cc) in iter {
            so.add_operator_product(pp, cc)
                .expect("Internal bug in add_operator_product");
        }
        so
    }
}

/// Implements the extend function (Extend trait) of QubitOperator.
///
impl Extend<(PauliProduct, CalculatorComplex)> for QubitOperator {
    /// Extends the QubitOperator by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the QubitOperator.
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

/// Implements the format function (Display trait) of QubitOperator.
///
impl fmt::Display for QubitOperator {
    /// Formats the QubitOperator using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted QubitOperator.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "QubitOperator{\n".to_string();
        for (key, val) in self.iter() {
            writeln!(output, "{}: {},", key, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}

impl JordanWignerSpinToFermion for QubitOperator {
    type Output = FermionOperator;

    /// Implements JordanWignerSpinToFermion for a QubitOperator.
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
    use crate::StruqtureSerialisationMeta;
    use serde_test::{assert_tokens, Configure, Token};

    // Test the Clone and PartialEq traits of QubitOperator
    #[test]
    fn so_from_sos() {
        let pp: PauliProduct = PauliProduct::new().z(0);
        let sos = QubitOperatorSerialize {
            items: vec![(pp.clone(), 0.5.into(), 0.0.into())],
            serialisation_meta: StruqtureSerialisationMeta {
                type_name: "QubitOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0-alpha.0".to_string(),
            },
        };
        let mut so = QubitOperator::new();
        so.set(pp, CalculatorComplex::from(0.5)).unwrap();

        assert_eq!(QubitOperator::try_from(sos.clone()).unwrap(), so);
        assert_eq!(QubitOperatorSerialize::from(so), sos);
    }
    // Test the Clone and PartialEq traits of QubitOperator
    #[test]
    fn clone_partial_eq() {
        let pp: PauliProduct = PauliProduct::new().z(0);
        let sos = QubitOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            serialisation_meta: StruqtureSerialisationMeta {
                type_name: "QubitOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        // Test Clone trait
        assert_eq!(sos.clone(), sos);

        // Test PartialEq trait
        let pp_1: PauliProduct = PauliProduct::new().z(0);
        let sos_1 = QubitOperatorSerialize {
            items: vec![(pp_1, 0.5.into(), 0.0.into())],
            serialisation_meta: StruqtureSerialisationMeta {
                type_name: "QubitOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };
        let pp_2: PauliProduct = PauliProduct::new().z(2);
        let sos_2 = QubitOperatorSerialize {
            items: vec![(pp_2, 0.5.into(), 0.0.into())],
            serialisation_meta: StruqtureSerialisationMeta {
                type_name: "QubitOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };
        assert!(sos_1 == sos);
        assert!(sos == sos_1);
        assert!(sos_2 != sos);
        assert!(sos != sos_2);
    }

    // Test the Debug trait of QubitOperator
    #[test]
    fn debug() {
        let pp: PauliProduct = PauliProduct::new().z(0);
        let sos = QubitOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            serialisation_meta: StruqtureSerialisationMeta {
                type_name: "QubitOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_eq!(
            format!("{:?}", sos),
            "QubitOperatorSerialize { items: [(PauliProduct { items: [(0, Z)] }, Float(0.5), Float(0.0))], serialisation_meta: StruqtureSerialisationMeta { type_name: \"QubitOperator\", min_version: (2, 0, 0), version: \"2.0.0\" } }"
        );
    }

    /// Test QubitOperator Serialization and Deserialization traits (readable)
    #[test]
    fn serde_readable() {
        let pp = PauliProduct::new().x(0);
        let sos = QubitOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            serialisation_meta: StruqtureSerialisationMeta {
                type_name: "QubitOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_tokens(
            &sos.readable(),
            &[
                Token::Struct {
                    name: "QubitOperatorSerialize",
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
                Token::Str("serialisation_meta"),
                Token::Struct {
                    name: "StruqtureSerialisationMeta",
                    len: 3,
                },
                Token::Str("type_name"),
                Token::Str("QubitOperator"),
                Token::Str("min_version"),
                Token::Tuple { len: 3 },
                Token::U64(2),
                Token::U64(0),
                Token::U64(0),
                Token::TupleEnd,
                Token::Str("version"),
                Token::Str("2.0.0"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    /// Test QubitOperator Serialization and Deserialization traits (compact)
    #[test]
    fn serde_compact() {
        let pp = PauliProduct::new().x(0);
        let sos = QubitOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            serialisation_meta: StruqtureSerialisationMeta {
                type_name: "QubitOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_tokens(
            &sos.compact(),
            &[
                Token::Struct {
                    name: "QubitOperatorSerialize",
                    len: 2,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 3 },
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 2 },
                Token::U64(0),
                Token::UnitVariant {
                    name: "SingleQubitOperator",
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
                Token::Str("serialisation_meta"),
                Token::Struct {
                    name: "StruqtureSerialisationMeta",
                    len: 3,
                },
                Token::Str("type_name"),
                Token::Str("QubitOperator"),
                Token::Str("min_version"),
                Token::Tuple { len: 3 },
                Token::U64(2),
                Token::U64(0),
                Token::U64(0),
                Token::TupleEnd,
                Token::Str("version"),
                Token::Str("2.0.0"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
