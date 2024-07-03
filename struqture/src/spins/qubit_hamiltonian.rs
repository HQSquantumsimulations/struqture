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

use super::{OperateOnSpins, QubitOperator, ToSparseMatrixOperator, ToSparseMatrixSuperOperator};
use crate::fermions::{FermionHamiltonian, FermionOperator};
use crate::mappings::JordanWignerSpinToFermion;
use crate::prelude::*;
use crate::spins::{HermitianOperateOnSpins, PauliProduct, SpinIndex};
use crate::{CooSparseMatrix, GetValue, OperateOnDensityMatrix, OperateOnState, StruqtureError};
use indexmap::map::{Entry, Iter};
use indexmap::IndexMap;
use num_complex::Complex64;
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

/// QubitHamiltonians are combinations of PauliProducts with specific CalculatorFloat coefficients.
///
/// This is a representation of sums of pauli products with weightings, in order to build a full hamiltonian.
/// QubitHamiltonian is the hermitian equivalent of QubitOperator.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorFloat;
/// use struqture::spins::{HermitianOperateOnSpins, PauliProduct, QubitHamiltonian};
///
/// let mut sh = QubitHamiltonian::new();
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
#[serde(try_from = "QubitHamiltonianSerialize")]
#[serde(into = "QubitHamiltonianSerialize")]
pub struct QubitHamiltonian {
    // The internal HashMap of PauliProducts and coefficients (CalculatorFloat)
    internal_map: IndexMap<PauliProduct, CalculatorFloat>,
}

impl crate::SerializationSupport for QubitHamiltonian {
    fn struqture_type() -> crate::StruqtureType {
        crate::StruqtureType::QubitHamiltonian
    }
}
#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for QubitHamiltonian {
    fn schema_name() -> String {
        "struqture::spins::QubitHamiltonian".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <QubitHamiltonianSerialize>::json_schema(gen)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
/// # QubitHamiltonian
/// QubitHamiltonians are combinations of PauliProducts with specific CalculatorFloat coefficients.
///
/// This is a representation of sums of pauli products with weightings, in order to build a full hamiltonian.
/// QubitHamiltonian is the hermitian equivalent of QubitOperator.
struct QubitHamiltonianSerialize {
    /// List of all non-zero entries in the QubitHamiltonian in the form (PauliProduct, real weight).
    items: Vec<(PauliProduct, CalculatorFloat)>,
    serialisation_meta: crate::StruqtureSerialisationMeta,
}

impl TryFrom<QubitHamiltonianSerialize> for QubitHamiltonian {
    type Error = StruqtureError;
    fn try_from(value: QubitHamiltonianSerialize) -> Result<Self, Self::Error> {
        let target_serialisation_meta =
            <Self as crate::SerializationSupport>::target_serialisation_meta();
        crate::check_can_be_deserialised(&target_serialisation_meta, &value.serialisation_meta)?;
        let new_noise_op: QubitHamiltonian = value.items.into_iter().collect();
        Ok(new_noise_op)
    }
}

impl From<QubitHamiltonian> for QubitHamiltonianSerialize {
    fn from(value: QubitHamiltonian) -> Self {
        let serialisation_meta = crate::SerializationSupport::struqture_serialisation_meta(&value);

        let new_noise_op: Vec<(PauliProduct, CalculatorFloat)> = value.into_iter().collect();
        Self {
            items: new_noise_op,
            serialisation_meta,
        }
    }
}

impl<'a> OperateOnDensityMatrix<'a> for QubitHamiltonian {
    type Index = PauliProduct;
    type Value = CalculatorFloat;

    // From trait
    fn get(&self, key: &Self::Index) -> &Self::Value {
        match self.internal_map.get(key) {
            Some(value) => value,
            None => &CalculatorFloat::ZERO,
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

    /// Overwrites an existing entry or sets a new entry in the QubitHamiltonian with the given (PauliProduct key, CalculatorFloat value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The PauliProduct key to set in the QubitHamiltonian.
    /// * `value` - The corresponding CalculatorFloat value to set for the key in the QubitHamiltonian.
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
                Entry::Occupied(val) => Ok(Some(val.shift_remove())),
                Entry::Vacant(_) => Ok(None),
            }
        }
    }
}

impl<'a> OperateOnState<'a> for QubitHamiltonian {
    // From trait
    fn hermitian_conjugate(&self) -> Self {
        self.clone()
    }
}

impl<'a> OperateOnSpins<'a> for QubitHamiltonian {
    /// Gets the maximum index of the QubitHamiltonian.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of spins in the QubitHamiltonian.
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

impl<'a> HermitianOperateOnSpins<'a> for QubitHamiltonian {}

impl<'a> ToSparseMatrixOperator<'a> for QubitHamiltonian {}
impl<'a> ToSparseMatrixSuperOperator<'a> for QubitHamiltonian {
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
    ) -> Result<crate::CooSparseMatrix, StruqtureError> {
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

/// Implements the default function (Default trait) of QubitHamiltonian (an empty QubitHamiltonian).
///
impl Default for QubitHamiltonian {
    fn default() -> Self {
        Self::new()
    }
}

/// Functions for the QubitHamiltonian
///
impl QubitHamiltonian {
    /// Creates a new QubitHamiltonian.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) QubitHamiltonian.
    pub fn new() -> Self {
        QubitHamiltonian {
            internal_map: IndexMap::new(),
        }
    }

    /// Creates a new QubitHamiltonian with pre-allocated capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The pre-allocated capacity of the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) QubitHamiltonian.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            internal_map: IndexMap::with_capacity(capacity),
        }
    }

    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_export")]
    pub fn to_struqture_1(
        &self,
    ) -> Result<struqture_1::spins::SpinHamiltonianSystem, StruqtureError> {
        let mut new_qubit_system = struqture_1::spins::SpinHamiltonianSystem::new(None);
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
        value: &struqture_1::spins::SpinHamiltonianSystem,
    ) -> Result<Self, StruqtureError> {
        let mut new_qubit_operator = Self::new();
        for (key, val) in struqture_1::OperateOnDensityMatrix::iter(value) {
            let self_key = PauliProduct::from_struqture_1(key)?;
            let _ = new_qubit_operator.set(self_key, val.clone());
        }
        Ok(new_qubit_operator)
    }
}

impl TryFrom<QubitOperator> for QubitHamiltonian {
    type Error = StruqtureError;
    /// Tries to convert a QubitOperator into a QubitHamiltonian.
    ///
    /// # Arguments
    ///
    /// * `hamiltonian` - The QubitOperator to try to convert.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The QubitOperator converted into a QubitHamiltonian.
    /// * `Err(StruqtureError::NonHermitianOperator)` - Key is naturally hermitian (on-diagonal term), but its corresponding value is not real.
    fn try_from(hamiltonian: QubitOperator) -> Result<Self, StruqtureError> {
        let mut internal = QubitHamiltonian::new();
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

/// Implements the negative sign function of QubitOperator.
///
impl ops::Neg for QubitHamiltonian {
    type Output = QubitHamiltonian;
    /// Implement minus sign for QubitHamiltonian.
    ///
    /// # Returns
    ///
    /// * `Self` - The QubitHamiltonian * -1.
    fn neg(self) -> Self {
        let mut internal = self.internal_map.clone();
        for key in self.keys() {
            internal.insert(key.clone(), internal[key].clone() * -1.0);
        }
        QubitHamiltonian {
            internal_map: internal,
        }
    }
}

/// Implements the plus function of QubitHamiltonian by QubitHamiltonian.
///
impl<T, V> ops::Add<T> for QubitHamiltonian
where
    T: IntoIterator<Item = (PauliProduct, V)>,
    V: Into<CalculatorFloat>,
{
    type Output = Self;
    /// Implements `+` (add) for two QubitHamiltonians.
    ///
    /// # Arguments
    ///
    /// * `other` - The QubitHamiltonian to be added.
    ///
    /// # Returns
    ///
    /// * `Self` - The two QubitHamiltonians added together.
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

/// Implements the minus function of QubitHamiltonian by QubitHamiltonian.
///
impl<T, V> ops::Sub<T> for QubitHamiltonian
where
    T: IntoIterator<Item = (PauliProduct, V)>,
    V: Into<CalculatorFloat>,
{
    type Output = Self;
    /// Implements `-` (subtract) for two QubitHamiltonians.
    ///
    /// # Arguments
    ///
    /// * `other` - The QubitHamiltonian to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Self` - The two QubitHamiltonians subtracted.
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

/// Implements the multiplication function of QubitHamiltonian by CalculatorFloat.
///
impl ops::Mul<CalculatorFloat> for QubitHamiltonian {
    type Output = Self;
    /// Implement `*` for QubitHamiltonian and CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The QubitHamiltonian multiplied by the CalculatorFloat.
    fn mul(self, other: CalculatorFloat) -> Self {
        let mut internal = self.internal_map.clone();
        for key in self.keys() {
            internal.insert(key.clone(), internal[key].clone() * other.clone());
        }
        QubitHamiltonian {
            internal_map: internal,
        }
    }
}

/// Implements the multiplication function of QubitHamiltonian by CalculatorComplex.
///
impl ops::Mul<CalculatorComplex> for QubitHamiltonian {
    type Output = QubitOperator;
    /// Implement `*` for QubitHamiltonian and CalculatorComplex.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex by which to multiply.
    ///
    /// # Returns
    ///
    /// * `QubitOperator` - The QubitHamiltonian multiplied by the CalculatorFloat.
    ///
    /// # Panics
    ///
    /// * Internal bug in set.
    fn mul(self, other: CalculatorComplex) -> Self::Output {
        let mut new_out = QubitOperator::with_capacity(self.len());
        for (key, val) in self {
            new_out
                .set(key, other.clone() * val)
                .expect("Internal bug in set");
        }
        new_out
    }
}

/// Implement `*` for QubitHamiltonian and QubitHamiltonian.
///
impl ops::Mul<QubitHamiltonian> for QubitHamiltonian {
    type Output = QubitOperator;
    /// Implement `*` for QubitHamiltonian and QubitHamiltonian.
    ///
    /// # Arguments
    ///
    /// * `other` - The QubitHamiltonian to multiply by.
    ///
    /// # Returns
    ///
    /// * `QubitOperator` - The two QubitHamiltonians multiplied.
    ///
    /// # Panics
    ///
    /// * Internal bug in add_operator_product.
    fn mul(self, other: QubitHamiltonian) -> Self::Output {
        let mut qubit_op = QubitOperator::with_capacity(self.len() * other.len());
        for (pps, vals) in self {
            for (ppo, valo) in other.iter() {
                let (ppp, coefficient) = pps.clone() * ppo.clone();
                let coefficient =
                    Into::<CalculatorComplex>::into(valo) * vals.clone() * coefficient;
                qubit_op
                    .add_operator_product(ppp, coefficient)
                    .expect("Internal bug in add_operator_product");
            }
        }
        qubit_op
    }
}

/// Implements the into_iter function (IntoIterator trait) of QubitHamiltonian.
///
impl IntoIterator for QubitHamiltonian {
    type Item = (PauliProduct, CalculatorFloat);
    type IntoIter = indexmap::map::IntoIter<PauliProduct, CalculatorFloat>;

    /// Returns the QubitHamiltonian in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The QubitHamiltonian in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference QubitHamiltonian.
///
impl<'a> IntoIterator for &'a QubitHamiltonian {
    type Item = (&'a PauliProduct, &'a CalculatorFloat);
    type IntoIter = Iter<'a, PauliProduct, CalculatorFloat>;

    /// Returns the reference QubitHamiltonian in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference QubitHamiltonian in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of QubitHamiltonian.
///
impl FromIterator<(PauliProduct, CalculatorFloat)> for QubitHamiltonian {
    /// Returns the object in QubitHamiltonian form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the QubitHamiltonian.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in QubitHamiltonian form.
    ///
    /// # Panics
    ///
    /// * Internal bug in add_operator_product.
    fn from_iter<I: IntoIterator<Item = (PauliProduct, CalculatorFloat)>>(iter: I) -> Self {
        let mut so = QubitHamiltonian::new();
        for (pp, cc) in iter {
            so.add_operator_product(pp, cc)
                .expect("Internal bug in add_operator_product");
        }
        so
    }
}

/// Implements the extend function (Extend trait) of QubitHamiltonian.
///
impl Extend<(PauliProduct, CalculatorFloat)> for QubitHamiltonian {
    /// Extends the QubitHamiltonian by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the QubitHamiltonian.
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

/// Implements the format function (Display trait) of QubitHamiltonian.
///
impl fmt::Display for QubitHamiltonian {
    /// Formats the QubitHamiltonian using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted QubitHamiltonian.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "QubitHamiltonian{\n".to_string();
        for (key, val) in self.iter() {
            writeln!(output, "{}: {},", key, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}

impl JordanWignerSpinToFermion for QubitHamiltonian {
    type Output = FermionHamiltonian;

    /// Implements JordanWignerSpinToFermion for a QubitHamiltonian.
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

    // Test the Clone and PartialEq traits of QubitHamiltonian
    #[test]
    fn sh_from_shs() {
        let pp: PauliProduct = PauliProduct::new().z(0);
        let shs = QubitHamiltonianSerialize {
            items: vec![(pp.clone(), 0.5.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "QubitHamiltonian".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0-alpha.0".to_string(),
            },
        };
        let mut sh = QubitHamiltonian::new();
        sh.set(pp, CalculatorFloat::from(0.5)).unwrap();

        assert_eq!(QubitHamiltonianSerialize::from(sh), shs);
    }
    // Test the Clone and PartialEq traits of QubitHamiltonian
    #[test]
    fn clone_partial_eq() {
        let pp: PauliProduct = PauliProduct::new().z(0);
        let shs = QubitHamiltonianSerialize {
            items: vec![(pp, 0.5.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "QubitHamiltonian".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        // Test Clone trait
        assert_eq!(shs.clone(), shs);

        // Test PartialEq trait
        let pp_1: PauliProduct = PauliProduct::new().z(0);
        let shs_1 = QubitHamiltonianSerialize {
            items: vec![(pp_1, 0.5.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "QubitHamiltonian".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };
        let pp_2: PauliProduct = PauliProduct::new().z(2);
        let shs_2 = QubitHamiltonianSerialize {
            items: vec![(pp_2, 0.5.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "QubitHamiltonian".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };
        assert!(shs_1 == shs);
        assert!(shs == shs_1);
        assert!(shs_2 != shs);
        assert!(shs != shs_2);
    }

    // Test the Debug trait of QubitHamiltonian
    #[test]
    fn debug() {
        let pp: PauliProduct = PauliProduct::new().z(0);
        let shs = QubitHamiltonianSerialize {
            items: vec![(pp, 0.5.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "QubitHamiltonian".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_eq!(
            format!("{:?}", shs),
            "QubitHamiltonianSerialize { items: [(PauliProduct { items: [(0, Z)] }, Float(0.5))], serialisation_meta: StruqtureSerialisationMeta { type_name: \"QubitHamiltonian\", min_version: (2, 0, 0), version: \"2.0.0\" } }"
        );
    }

    /// Test QubitHamiltonian Serialization and Deserialization traits (readable)
    #[test]
    fn serde_readable() {
        let pp = PauliProduct::new().x(0);
        let shs = QubitHamiltonianSerialize {
            items: vec![(pp, 0.5.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "QubitHamiltonian".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_tokens(
            &shs.readable(),
            &[
                Token::Struct {
                    name: "QubitHamiltonianSerialize",
                    len: 2,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 2 },
                Token::Str("0X"),
                Token::F64(0.5),
                Token::TupleEnd,
                Token::SeqEnd,
                Token::Str("serialisation_meta"),
                Token::Struct {
                    name: "StruqtureSerialisationMeta",
                    len: 3,
                },
                Token::Str("type_name"),
                Token::Str("QubitHamiltonian"),
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

    /// Test QubitHamiltonian Serialization and Deserialization traits (compact)
    #[test]
    fn serde_compact() {
        let pp = PauliProduct::new().x(0);
        let shs = QubitHamiltonianSerialize {
            items: vec![(pp, 0.5.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "QubitHamiltonian".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_tokens(
            &shs.compact(),
            &[
                Token::Struct {
                    name: "QubitHamiltonianSerialize",
                    len: 2,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 2 },
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
                Token::TupleEnd,
                Token::SeqEnd,
                Token::Str("serialisation_meta"),
                Token::Struct {
                    name: "StruqtureSerialisationMeta",
                    len: 3,
                },
                Token::Str("type_name"),
                Token::Str("QubitHamiltonian"),
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
