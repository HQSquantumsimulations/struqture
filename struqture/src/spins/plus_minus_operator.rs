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

use super::{DecoherenceOperator, DecoherenceProduct, PauliOperator, PauliProduct};
use crate::fermions::FermionOperator;
use crate::mappings::JordanWignerSpinToFermion;
use crate::spins::{PauliHamiltonian, PlusMinusProduct};
use crate::{OperateOnDensityMatrix, OperateOnState, StruqtureError, SymmetricIndex};
use num_complex::Complex64;
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde::{Deserialize, Serialize};

use indexmap::map::{Entry, Iter};
use indexmap::IndexMap;

use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

/// PlusMinusOperators are combinations of PlusMinusProducts with specific CalculatorComplex coefficients.
///
/// This is a representation of sums of pauli products with weightings, in order to build a full hamiltonian.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::spins::{OperateOnSpins, PlusMinusProduct, PlusMinusOperator};
///
/// let mut so = PlusMinusOperator::new();
///
/// // Representing the hamiltonian $ 1/2 \sigma_0^{+} \sigma_1^{+} + 1/5 \sigma_0^{z} $
/// let pp_0x1x = PlusMinusProduct::new().plus(0).plus(1);
/// let pp_0z = PlusMinusProduct::new().z(0);
/// so.add_operator_product(pp_0x1x.clone(), CalculatorComplex::from(0.5)).unwrap();
/// so.add_operator_product(pp_0z.clone(), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(so.get(&pp_0x1x), &CalculatorComplex::from(0.5));
/// assert_eq!(so.get(&pp_0z), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "PlusMinusOperatorSerialize")]
#[serde(into = "PlusMinusOperatorSerialize")]
pub struct PlusMinusOperator {
    // The internal HashMap of PlusMinusProducts and coefficients (CalculatorComplex)
    internal_map: IndexMap<PlusMinusProduct, CalculatorComplex>,
}

impl crate::SerializationSupport for PlusMinusOperator {
    fn struqture_type() -> crate::StruqtureType {
        crate::StruqtureType::PlusMinusOperator
    }
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for PlusMinusOperator {
    fn schema_name() -> String {
        "PlusMinusOperator".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <PlusMinusOperatorSerialize>::json_schema(gen)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
struct PlusMinusOperatorSerialize {
    items: Vec<(PlusMinusProduct, CalculatorFloat, CalculatorFloat)>,
    serialisation_meta: crate::StruqtureSerialisationMeta,
}

impl TryFrom<PlusMinusOperatorSerialize> for PlusMinusOperator {
    type Error = StruqtureError;
    fn try_from(value: PlusMinusOperatorSerialize) -> Result<Self, Self::Error> {
        let target_serialisation_meta =
            <Self as crate::SerializationSupport>::target_serialisation_meta();
        crate::check_can_be_deserialised(&target_serialisation_meta, &value.serialisation_meta)?;
        let new_noise_op: PlusMinusOperator = value
            .items
            .into_iter()
            .map(|(key, real, imag)| (key, CalculatorComplex { re: real, im: imag }))
            .collect();
        Ok(new_noise_op)
    }
}

impl From<PlusMinusOperator> for PlusMinusOperatorSerialize {
    fn from(value: PlusMinusOperator) -> Self {
        let serialisation_meta = crate::SerializationSupport::struqture_serialisation_meta(&value);

        let new_noise_op: Vec<(PlusMinusProduct, CalculatorFloat, CalculatorFloat)> = value
            .into_iter()
            .map(|(key, val)| (key, val.re, val.im))
            .collect();
        Self {
            items: new_noise_op,
            serialisation_meta,
        }
    }
}

impl<'a> OperateOnDensityMatrix<'a> for PlusMinusOperator {
    type Value = CalculatorComplex;
    type Index = PlusMinusProduct;

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

    /// Overwrites an existing entry or sets a new entry in the PlusMinusOperator with the given (PlusMinusProduct key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The PlusMinusProduct key to set in the PlusMinusOperator.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the PlusMinusOperator.
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

impl OperateOnState<'_> for PlusMinusOperator {
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

/// Implements the default function (Default trait) of PlusMinusOperator (an empty PlusMinusOperator).
///
impl Default for PlusMinusOperator {
    fn default() -> Self {
        Self::new()
    }
}

/// Functions for the PlusMinusOperator
///
impl PlusMinusOperator {
    /// Creates a new PlusMinusOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) PlusMinusOperator.
    pub fn new() -> Self {
        PlusMinusOperator {
            internal_map: IndexMap::new(),
        }
    }

    /// Creates a new PlusMinusOperator with pre-allocated capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The pre-allocated capacity of the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) PlusMinusOperator.
    pub fn with_capacity(capacity: usize) -> Self {
        PlusMinusOperator {
            internal_map: IndexMap::with_capacity(capacity),
        }
    }

    /// Gets the maximum index of the PlusMinusOperator.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of spins in the PlusMinusOperator.
    pub fn current_number_spins(&self) -> usize {
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

    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_export")]
    pub fn to_struqture_1(&self) -> Result<struqture_1::spins::PlusMinusOperator, StruqtureError> {
        let mut new_pm_system = struqture_1::spins::PlusMinusOperator::new();
        for (key, val) in self.iter() {
            let one_key = key.to_struqture_1()?;
            let _ =
                struqture_1::OperateOnDensityMatrix::set(&mut new_pm_system, one_key, val.clone());
        }
        Ok(new_pm_system)
    }

    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_import")]
    pub fn from_struqture_1(
        value: &struqture_1::spins::PlusMinusOperator,
    ) -> Result<Self, StruqtureError> {
        let mut new_operator = Self::new();
        for (key, val) in struqture_1::OperateOnDensityMatrix::iter(value) {
            let self_key = PlusMinusProduct::from_struqture_1(key)?;
            let _ = new_operator.set(self_key, val.clone());
        }
        Ok(new_operator)
    }
}

impl From<PlusMinusOperator> for PauliOperator {
    /// Converts a PlusMinusOperator into a PauliOperator.
    ///
    /// # Arguments
    ///
    /// * `value` - The PlusMinusOperator to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The PlusMinusOperator converted into a PauliOperator.
    fn from(value: PlusMinusOperator) -> Self {
        let mut new_operator = PauliOperator::with_capacity(2 * value.len());
        for (product, val) in value.into_iter() {
            let transscribed_vector: Vec<(PauliProduct, Complex64)> = product.into();
            for (transscribed_product, prefactor) in transscribed_vector {
                new_operator
                    .add_operator_product(transscribed_product, val.clone() * prefactor)
                    .expect("Unexpected error adding operators. Internal struqture error");
            }
        }
        new_operator
    }
}

impl From<PauliOperator> for PlusMinusOperator {
    /// Converts a PauliOperator into a PlusMinusOperator.
    ///
    /// # Arguments
    ///
    /// * `value` - The PauliOperator to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The PauliOperator converted into a PlusMinusOperator.
    fn from(value: PauliOperator) -> Self {
        let mut new_operator = PlusMinusOperator::with_capacity(2 * value.len());
        for (product, val) in value.into_iter() {
            let transscribed_vector: Vec<(PlusMinusProduct, Complex64)> = product.into();
            for (transscribed_product, prefactor) in transscribed_vector {
                new_operator
                    .add_operator_product(transscribed_product, val.clone() * prefactor)
                    .expect("Unexpected error adding operators. Internal struqture error");
            }
        }
        new_operator
    }
}

impl From<PlusMinusOperator> for DecoherenceOperator {
    /// Converts a PlusMinusOperator into a DecoherenceOperator.
    ///
    /// # Arguments
    ///
    /// * `value` - The PlusMinusOperator to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The PlusMinusOperator converted into a DecoherenceOperator.
    fn from(value: PlusMinusOperator) -> Self {
        let mut new_operator = DecoherenceOperator::with_capacity(2 * value.len());
        for (product, val) in value.into_iter() {
            let transscribed_vector: Vec<(DecoherenceProduct, Complex64)> = product.into();
            for (transscribed_product, prefactor) in transscribed_vector {
                new_operator
                    .add_operator_product(transscribed_product, val.clone() * prefactor)
                    .expect("Unexpected error adding operators. Internal struqture error");
            }
        }
        new_operator
    }
}

impl From<DecoherenceOperator> for PlusMinusOperator {
    /// Converts a DecoherenceOperator into a PlusMinusOperator.
    ///
    /// # Arguments
    ///
    /// * `value` - The DecoherenceOperator to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The DecoherenceOperator converted into a PlusMinusOperator.
    fn from(value: DecoherenceOperator) -> Self {
        let mut new_operator = PlusMinusOperator::with_capacity(2 * value.len());
        for (product, val) in value.into_iter() {
            let transscribed_vector: Vec<(PlusMinusProduct, Complex64)> = product.into();
            for (transscribed_product, prefactor) in transscribed_vector {
                new_operator
                    .add_operator_product(transscribed_product, val.clone() * prefactor)
                    .expect("Unexpected error adding operators. Internal struqture error");
            }
        }
        new_operator
    }
}

impl TryFrom<PlusMinusOperator> for PauliHamiltonian {
    type Error = StruqtureError;

    /// Tries to converts a PlusMinusOperator into a PauliHamiltonian.
    ///
    /// # Arguments
    ///
    /// * `value` - The PlusMinusOperator to try to convert.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The PlusMinusOperator converted into a PauliHamiltonian.
    /// * `Err(StruqtureError::NonHermitianOperator)` - Key is naturally hermitian (on-diagonal term), but its corresponding value is not real.
    fn try_from(value: PlusMinusOperator) -> Result<Self, Self::Error> {
        let tmp_operator = PauliOperator::from(value).truncate(1e-16);
        PauliHamiltonian::try_from(tmp_operator)
    }
}

impl From<PauliHamiltonian> for PlusMinusOperator {
    /// Converts a PauliHamiltonian into a PlusMinusOperator.
    ///
    /// # Arguments
    ///
    /// * `value` - The PauliHamiltonian to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The PauliHamiltonian converted into a PlusMinusOperator.
    fn from(value: PauliHamiltonian) -> Self {
        let mut new_operator = PlusMinusOperator::with_capacity(2 * value.len());
        for (product, val) in value.into_iter() {
            let transscribed_vector: Vec<(PlusMinusProduct, Complex64)> = product.into();
            for (transscribed_product, prefactor) in transscribed_vector {
                new_operator
                    .add_operator_product(
                        transscribed_product,
                        CalculatorComplex::from(val.clone()) * prefactor,
                    )
                    .expect("Unexpected error adding operators. Internal struqture error");
            }
        }
        new_operator.truncate(1e-16)
    }
}

/// Implements the negative sign function of PlusMinusOperator.
///
impl ops::Neg for PlusMinusOperator {
    type Output = PlusMinusOperator;
    /// Implement minus sign for PlusMinusOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The PlusMinusOperator * -1.
    fn neg(self) -> Self {
        let mut internal = IndexMap::with_capacity(self.len());
        for (key, val) in self {
            internal.insert(key.clone(), val.neg());
        }
        PlusMinusOperator {
            internal_map: internal,
        }
    }
}

/// Implements the plus function of PlusMinusOperator by PlusMinusOperator.
///
impl<T, V> ops::Add<T> for PlusMinusOperator
where
    T: IntoIterator<Item = (PlusMinusProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implements `+` (add) for two PlusMinusOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The PlusMinusOperator to be added.
    ///
    /// # Returns
    ///
    /// * `Self` - The two PlusMinusOperators added together.
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

/// Implements the minus function of PlusMinusOperator by PlusMinusOperator.
///
impl<T, V> ops::Sub<T> for PlusMinusOperator
where
    T: IntoIterator<Item = (PlusMinusProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implements `-` (subtract) for two PlusMinusOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The PlusMinusOperator to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Self` - The two PlusMinusOperators subtracted.
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

/// Implements the multiplication function of PlusMinusOperator by CalculatorComplex/CalculatorFloat.
///
impl<T> ops::Mul<T> for PlusMinusOperator
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for PlusMinusOperator and CalculatorComplex/CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex or CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The PlusMinusOperator multiplied by the CalculatorComplex/CalculatorFloat.
    fn mul(self, other: T) -> Self {
        let other_cc = Into::<CalculatorComplex>::into(other);
        let mut internal = IndexMap::with_capacity(self.len());
        for (key, val) in self {
            internal.insert(key, val * other_cc.clone());
        }
        PlusMinusOperator {
            internal_map: internal,
        }
    }
}

/// Implements the into_iter function (IntoIterator trait) of PlusMinusOperator.
///
impl IntoIterator for PlusMinusOperator {
    type Item = (PlusMinusProduct, CalculatorComplex);
    type IntoIter = indexmap::map::IntoIter<PlusMinusProduct, CalculatorComplex>;

    /// Returns the PlusMinusOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The PlusMinusOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference PlusMinusOperator.
///
impl<'a> IntoIterator for &'a PlusMinusOperator {
    type Item = (&'a PlusMinusProduct, &'a CalculatorComplex);
    type IntoIter = Iter<'a, PlusMinusProduct, CalculatorComplex>;

    /// Returns the reference PlusMinusOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference PlusMinusOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of PlusMinusOperator.
///
impl FromIterator<(PlusMinusProduct, CalculatorComplex)> for PlusMinusOperator {
    /// Returns the object in PlusMinusOperator form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the PlusMinusOperator.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in PlusMinusOperator form.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from_iter<I: IntoIterator<Item = (PlusMinusProduct, CalculatorComplex)>>(iter: I) -> Self {
        let mut so = PlusMinusOperator::new();
        for (pp, cc) in iter {
            so.add_operator_product(pp, cc)
                .expect("Internal bug in add_operator_product");
        }
        so
    }
}

/// Implements the extend function (Extend trait) of PlusMinusOperator.
///
impl Extend<(PlusMinusProduct, CalculatorComplex)> for PlusMinusOperator {
    /// Extends the PlusMinusOperator by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the PlusMinusOperator.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn extend<I: IntoIterator<Item = (PlusMinusProduct, CalculatorComplex)>>(&mut self, iter: I) {
        for (pp, cc) in iter {
            self.add_operator_product(pp, cc)
                .expect("Internal bug in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of PlusMinusOperator.
///
impl fmt::Display for PlusMinusOperator {
    /// Formats the PlusMinusOperator using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted PlusMinusOperator.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "PlusMinusOperator{\n".to_string();
        for (key, val) in self.iter() {
            writeln!(output, "{}: {},", key, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}

impl JordanWignerSpinToFermion for PlusMinusOperator {
    type Output = FermionOperator;

    /// Implements JordanWignerSpinToFermion for a PlusMinusOperator.
    ///
    /// The convention used is that |0> represents an empty fermionic state (spin-orbital),
    /// and |1> represents an occupied fermionic state.
    ///
    /// # Returns
    ///
    /// `FermionOperator` - The fermion operator that results from the transformation.
    fn jordan_wigner(&self) -> Self::Output {
        let mut out = FermionOperator::new();
        for pmp in self.keys() {
            out = out + pmp.jordan_wigner() * self.get(pmp);
        }
        out
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::STRUQTURE_VERSION;
    use serde_test::{assert_tokens, Configure, Token};

    // Test the Clone and PartialEq traits of PlusMinusOperator
    #[test]
    fn so_from_sos() {
        let pp: PlusMinusProduct = PlusMinusProduct::new().z(0);
        let sos = PlusMinusOperatorSerialize {
            items: vec![(pp.clone(), 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "PlusMinusOperator".to_string(),
                min_version: (2, 0, 0),
                version: STRUQTURE_VERSION.to_string(),
            },
        };
        let mut so = PlusMinusOperator::new();
        so.set(pp, CalculatorComplex::from(0.5)).unwrap();

        assert_eq!(PlusMinusOperator::try_from(sos.clone()).unwrap(), so);
        assert_eq!(PlusMinusOperatorSerialize::from(so), sos);
    }
    // Test the Clone and PartialEq traits of PlusMinusOperator
    #[test]
    fn clone_partial_eq() {
        let pp: PlusMinusProduct = PlusMinusProduct::new().z(0);
        let sos = PlusMinusOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "PlusMinusOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        // Test Clone trait
        assert_eq!(sos.clone(), sos);

        // Test PartialEq trait
        let pp_1: PlusMinusProduct = PlusMinusProduct::new().z(0);
        let sos_1 = PlusMinusOperatorSerialize {
            items: vec![(pp_1, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "PlusMinusOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };
        let pp_2: PlusMinusProduct = PlusMinusProduct::new().z(2);
        let sos_2 = PlusMinusOperatorSerialize {
            items: vec![(pp_2, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "PlusMinusOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };
        assert!(sos_1 == sos);
        assert!(sos == sos_1);
        assert!(sos_2 != sos);
        assert!(sos != sos_2);
    }

    // Test the Debug trait of PlusMinusOperator
    #[test]
    fn debug() {
        let pp: PlusMinusProduct = PlusMinusProduct::new().z(0);
        let sos = PlusMinusOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "PlusMinusOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_eq!(
            format!("{:?}", sos),
            "PlusMinusOperatorSerialize { items: [(PlusMinusProduct { items: [(0, Z)] }, Float(0.5), Float(0.0))], serialisation_meta: StruqtureSerialisationMeta { type_name: \"PlusMinusOperator\", min_version: (2, 0, 0), version: \"2.0.0\" } }"
        );
    }

    /// Test PlusMinusOperator Serialization and Deserialization traits (readable)
    #[test]
    fn serde_readable() {
        let pp = PlusMinusProduct::new().plus(0);
        let sos = PlusMinusOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "PlusMinusOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_tokens(
            &sos.readable(),
            &[
                Token::Struct {
                    name: "PlusMinusOperatorSerialize",
                    len: 2,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 3 },
                Token::Str("0+"),
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
                Token::Str("PlusMinusOperator"),
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

    /// Test PlusMinusOperator Serialization and Deserialization traits (compact)
    #[test]
    fn serde_compact() {
        let pp = PlusMinusProduct::new().plus(0);
        let sos = PlusMinusOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "PlusMinusOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_tokens(
            &sos.compact(),
            &[
                Token::Struct {
                    name: "PlusMinusOperatorSerialize",
                    len: 2,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 3 },
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 2 },
                Token::U64(0),
                Token::UnitVariant {
                    name: "SinglePlusMinusOperator",
                    variant: "Plus",
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
                Token::Str("PlusMinusOperator"),
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
