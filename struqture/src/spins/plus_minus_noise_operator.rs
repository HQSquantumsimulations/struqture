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

use crate::fermions::FermionLindbladNoiseOperator;
use crate::mappings::JordanWignerSpinToFermion;
use crate::spins::{PlusMinusOperator, PlusMinusProduct};
use crate::{OperateOnDensityMatrix, StruqtureError, StruqtureVersionSerializable};
use itertools::Itertools;
use num_complex::Complex64;
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Entry, Iter, Keys, Values};
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

use super::{DecoherenceProduct, SpinLindbladNoiseOperator};

/// PlusMinusLindbladNoiseOperators represent noise interactions in the Lindblad equation.
///
/// In the Lindblad equation, Linblad noise operator L_i are not limited to [crate::spins::PlusMinusProduct] style operators.
/// We use ([crate::spins::PlusMinusProduct], [crate::spins::PlusMinusProduct]) as a unique basis.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::spins::{PlusMinusProduct, PlusMinusLindbladNoiseOperator};
///
/// let mut system = PlusMinusLindbladNoiseOperator::new();
///
/// // Set noise terms:
/// let pp_01 = PlusMinusProduct::new().plus(0).plus(1);
/// let pp_0 = PlusMinusProduct::new().z(0);
/// system.set((pp_01.clone(), pp_01.clone()), CalculatorComplex::from(0.5)).unwrap();
/// system.set((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(system.get(&(pp_01.clone(), pp_01.clone())), &CalculatorComplex::from(0.5));
/// assert_eq!(system.get(&(pp_0.clone(), pp_0.clone())), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(from = "PlusMinusLindbladNoiseOperatorSerialize")]
#[serde(into = "PlusMinusLindbladNoiseOperatorSerialize")]
pub struct PlusMinusLindbladNoiseOperator {
    /// The internal map representing the noise terms
    internal_map: HashMap<(PlusMinusProduct, PlusMinusProduct), CalculatorComplex>,
}

impl crate::MinSupportedVersion for PlusMinusLindbladNoiseOperator {
    fn min_supported_version() -> (usize, usize, usize) {
        (1, 1, 0)
    }
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for PlusMinusLindbladNoiseOperator {
    fn schema_name() -> String {
        "PlusMinusLindbladNoiseOperator".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <PlusMinusLindbladNoiseOperatorSerialize>::json_schema(gen)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
struct PlusMinusLindbladNoiseOperatorSerialize {
    /// The vector representing the internal map of the PlusMinusLindbladNoiseOperator
    items: Vec<(
        PlusMinusProduct,
        PlusMinusProduct,
        CalculatorFloat,
        CalculatorFloat,
    )>,
    /// The struqture version
    _struqture_version: StruqtureVersionSerializable,
}

impl From<PlusMinusLindbladNoiseOperatorSerialize> for PlusMinusLindbladNoiseOperator {
    fn from(value: PlusMinusLindbladNoiseOperatorSerialize) -> Self {
        let new_noise_op: PlusMinusLindbladNoiseOperator = value
            .items
            .into_iter()
            .map(|(left, right, real, imag)| {
                ((left, right), CalculatorComplex { re: real, im: imag })
            })
            .collect();
        new_noise_op
    }
}

impl From<PlusMinusLindbladNoiseOperator> for PlusMinusLindbladNoiseOperatorSerialize {
    fn from(value: PlusMinusLindbladNoiseOperator) -> Self {
        let new_noise_op: Vec<(
            PlusMinusProduct,
            PlusMinusProduct,
            CalculatorFloat,
            CalculatorFloat,
        )> = value
            .into_iter()
            .map(|((left, right), val)| (left, right, val.re, val.im))
            .collect();
        let current_version = StruqtureVersionSerializable {
            major_version: 1,
            minor_version: 1,
        };
        Self {
            items: new_noise_op,
            _struqture_version: current_version,
        }
    }
}

impl<'a> OperateOnDensityMatrix<'a> for PlusMinusLindbladNoiseOperator {
    type Index = (PlusMinusProduct, PlusMinusProduct);
    type Value = CalculatorComplex;
    type IteratorType = Iter<'a, (PlusMinusProduct, PlusMinusProduct), CalculatorComplex>;
    type KeyIteratorType = Keys<'a, (PlusMinusProduct, PlusMinusProduct), CalculatorComplex>;
    type ValueIteratorType = Values<'a, (PlusMinusProduct, PlusMinusProduct), CalculatorComplex>;

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

    /// Overwrites an existing entry or sets a new entry in the PlusMinusLindbladNoiseOperator with the given ((PlusMinusProduct, PlusMinusProduct) key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The (PlusMinusProduct, PlusMinusProduct) key to set in the PlusMinusLindbladNoiseOperator.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the PlusMinusLindbladNoiseOperator.
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

/// Implements the default function (Default trait) of PlusMinusLindbladNoiseOperator (an empty PlusMinusLindbladNoiseOperator).
///
impl Default for PlusMinusLindbladNoiseOperator {
    fn default() -> Self {
        Self::new()
    }
}

/// Functions for the PlusMinusLindbladNoiseOperator
///
impl PlusMinusLindbladNoiseOperator {
    /// Creates a new PlusMinusLindbladNoiseOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) PlusMinusLindbladNoiseOperator.
    pub fn new() -> Self {
        PlusMinusLindbladNoiseOperator {
            internal_map: HashMap::new(),
        }
    }

    /// Creates a new PlusMinusLindbladNoiseOperator with pre-allocated capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The pre-allocated capacity of the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) PlusMinusLindbladNoiseOperator.
    pub fn with_capacity(capacity: usize) -> Self {
        PlusMinusLindbladNoiseOperator {
            internal_map: HashMap::with_capacity(capacity),
        }
    }

    /// Adds all noise entries corresponding to a ((PlusMinusOperator, PlusMinusOperator), CalculatorFloat).
    ///
    /// In the Lindblad equation, Linblad noise operator L_i are not limited to [crate::spins::PlusMinusProduct] style operators.
    /// We use ([crate::spins::PlusMinusProduct], [crate::spins::PlusMinusProduct]) as a unique basis.
    /// This function adds a Linblad-Term defined by a combination of Lindblad operators given as general [crate::spins::PlusMinusOperator]
    ///
    /// # Arguments
    ///
    /// * `left` - PlusMinusOperator that acts on the density matrix from the left in the Lindblad equation.
    /// * `value` -  PlusMinusOperator that acts on the density matrix from the right and in hermitian conjugated form in the Lindblad equation.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The noise was correctly added.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Number of spins in entry exceeds number of spins in system.
    pub fn add_noise_from_full_operators(
        &mut self,
        left: &PlusMinusOperator,
        right: &PlusMinusOperator,
        value: CalculatorComplex,
    ) -> Result<(), StruqtureError> {
        for ((decoherence_product_left, value_left), (decoherence_product_right, value_right)) in
            left.iter().cartesian_product(right.iter())
        {
            let value_complex = value_right.conj() * value_left;
            self.add_operator_product(
                (
                    decoherence_product_left.clone(),
                    decoherence_product_right.clone(),
                ),
                value_complex * value.clone(),
            )?;
        }
        Ok(())
    }

    /// Remaps the qubits in the PlusMinusLindbladNoiseOperator.
    ///
    /// # Arguments
    ///
    /// * `mapping` - HashMap containing the qubit remapping.
    ///
    /// # Returns
    ///
    /// * `Self` - The remapped PlusMinusLindbladNoiseOperator.
    pub fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Self {
        let mut new_noise = PlusMinusLindbladNoiseOperator::new();
        for ((left, right), rate) in self.iter() {
            let new_left = left.remap_qubits(mapping);
            let new_right = right.remap_qubits(mapping);
            new_noise
                .add_operator_product((new_left, new_right), rate.clone())
                .expect("Internal bug in add_operator_product");
        }
        new_noise
    }

    /// Separate self into an operator with the terms of given number of spins and an operator with the remaining operations
    ///
    /// # Arguments
    ///
    /// * `number_spins_left` - Number of spins to filter for in the left term of the keys.
    /// * `number_spins_right` - Number of spins to filter for in the right term of the keys.
    ///
    /// # Returns
    ///
    /// `Ok((separated, remainder))` - Operator with the noise terms where number_spins_left and number_spins_right match the number of spins the left and right noise operator product acts on and Operator with all other contributions.
    pub fn separate_into_n_terms(
        &self,
        number_spins_left: usize,
        number_spins_right: usize,
    ) -> Result<
        (
            PlusMinusLindbladNoiseOperator,
            PlusMinusLindbladNoiseOperator,
        ),
        StruqtureError,
    > {
        let mut separated = PlusMinusLindbladNoiseOperator::new();
        let mut remainder = PlusMinusLindbladNoiseOperator::new();
        for ((prod_l, prod_r), val) in self.iter() {
            if prod_l.iter().len() == number_spins_left && prod_r.iter().len() == number_spins_right
            {
                separated.add_operator_product((prod_l.clone(), prod_r.clone()), val.clone())?;
            } else {
                remainder.add_operator_product((prod_l.clone(), prod_r.clone()), val.clone())?;
            }
        }
        Ok((separated, remainder))
    }
}

impl From<PlusMinusLindbladNoiseOperator> for SpinLindbladNoiseOperator {
    /// Converts a PlusMinusLindbladNoiseOperator into a SpinLindbladNoiseOperator.
    ///
    /// # Arguments
    ///
    /// * `value` - The PlusMinusLindbladNoiseOperator to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The PlusMinusLindbladNoiseOperator converted into a SpinLindbladNoiseOperator.
    fn from(value: PlusMinusLindbladNoiseOperator) -> Self {
        let mut new_operator = SpinLindbladNoiseOperator::with_capacity(2 * value.len());
        for ((product_left, product_right), val) in value.into_iter() {
            let transscribed_vector_left: Vec<(DecoherenceProduct, Complex64)> =
                product_left.into();
            let transscribed_vector_right: Vec<(DecoherenceProduct, Complex64)> =
                product_right.into();
            for (transscribed_product_left, pref_left) in transscribed_vector_left {
                for (transscribed_product_right, pref_right) in transscribed_vector_right.clone() {
                    new_operator
                        .add_operator_product(
                            (
                                transscribed_product_left.clone(),
                                transscribed_product_right,
                            ),
                            val.clone() * pref_left * pref_right,
                        )
                        .expect("Unexpected error adding operators. Internal struqture error");
                }
            }
        }
        new_operator
    }
}

impl From<SpinLindbladNoiseOperator> for PlusMinusLindbladNoiseOperator {
    /// Converts a SpinLindbladNoiseOperator into a PlusMinusLindbladNoiseOperator.
    ///
    /// # Arguments
    ///
    /// * `value` - The SpinLindbladNoiseOperator to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The SpinLindbladNoiseOperator converted into a PlusMinusLindbladNoiseOperator.
    fn from(value: SpinLindbladNoiseOperator) -> Self {
        let mut new_operator = PlusMinusLindbladNoiseOperator::with_capacity(2 * value.len());
        for ((product_left, product_right), val) in value.into_iter() {
            let transscribed_vector_left: Vec<(PlusMinusProduct, Complex64)> = product_left.into();
            let transscribed_vector_right: Vec<(PlusMinusProduct, Complex64)> =
                product_right.into();
            for (transscribed_product_left, pref_left) in transscribed_vector_left {
                for (transscribed_product_right, pref_right) in transscribed_vector_right.clone() {
                    new_operator
                        .add_operator_product(
                            (
                                transscribed_product_left.clone(),
                                transscribed_product_right,
                            ),
                            val.clone() * pref_left * pref_right,
                        )
                        .expect("Unexpected error adding operators. Internal struqture error");
                }
            }
        }
        new_operator
    }
}

/// Implements the negative sign function of PlusMinusLindbladNoiseOperator.
///
impl ops::Neg for PlusMinusLindbladNoiseOperator {
    type Output = PlusMinusLindbladNoiseOperator;
    /// Implement minus sign for PlusMinusLindbladNoiseOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The PlusMinusLindbladNoiseOperator * -1.
    fn neg(self) -> Self {
        let mut internal = HashMap::with_capacity(self.len());
        for (key, val) in self {
            internal.insert(key.clone(), val.neg());
        }
        PlusMinusLindbladNoiseOperator {
            internal_map: internal,
        }
    }
}

/// Implements the plus function of PlusMinusLindbladNoiseOperator by PlusMinusLindbladNoiseOperator.
///
impl<T, V> ops::Add<T> for PlusMinusLindbladNoiseOperator
where
    T: IntoIterator<Item = ((PlusMinusProduct, PlusMinusProduct), V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implements `+` (add) for two PlusMinusLindbladNoiseOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The PlusMinusLindbladNoiseOperator to be added.
    ///
    /// # Returns
    ///
    /// * `Self` - The two PlusMinusLindbladNoiseOperators added together.
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

/// Implements the minus function of PlusMinusLindbladNoiseOperator by PlusMinusLindbladNoiseOperator.
///
impl<T, V> ops::Sub<T> for PlusMinusLindbladNoiseOperator
where
    T: IntoIterator<Item = ((PlusMinusProduct, PlusMinusProduct), V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implements `-` (subtract) for two PlusMinusLindbladNoiseOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The PlusMinusLindbladNoiseOperator to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Self` - The two PlusMinusLindbladNoiseOperators subtracted.
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

/// Implements the multiplication function of PlusMinusLindbladNoiseOperator by CalculatorComplex/CalculatorFloat.
///
impl<T> ops::Mul<T> for PlusMinusLindbladNoiseOperator
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for PlusMinusLindbladNoiseOperator and CalculatorComplex/CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex or CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The PlusMinusLindbladNoiseOperator multiplied by the CalculatorComplex/CalculatorFloat.
    fn mul(self, other: T) -> Self {
        let other_cc = Into::<CalculatorComplex>::into(other);
        let mut internal = HashMap::with_capacity(self.len());
        for (key, val) in self {
            internal.insert(key, val * other_cc.clone());
        }
        PlusMinusLindbladNoiseOperator {
            internal_map: internal,
        }
    }
}

/// Implements the into_iter function (IntoIterator trait) of PlusMinusLindbladNoiseOperator.
///
impl IntoIterator for PlusMinusLindbladNoiseOperator {
    type Item = ((PlusMinusProduct, PlusMinusProduct), CalculatorComplex);
    type IntoIter = std::collections::hash_map::IntoIter<
        (PlusMinusProduct, PlusMinusProduct),
        CalculatorComplex,
    >;
    /// Returns the PlusMinusLindbladNoiseOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The PlusMinusLindbladNoiseOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference PlusMinusLindbladNoiseOperator.
///
impl<'a> IntoIterator for &'a PlusMinusLindbladNoiseOperator {
    type Item = (
        &'a (PlusMinusProduct, PlusMinusProduct),
        &'a CalculatorComplex,
    );
    type IntoIter = Iter<'a, (PlusMinusProduct, PlusMinusProduct), CalculatorComplex>;

    /// Returns the reference PlusMinusLindbladNoiseOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference PlusMinusLindbladNoiseOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of PlusMinusLindbladNoiseOperator.
///
impl FromIterator<((PlusMinusProduct, PlusMinusProduct), CalculatorComplex)>
    for PlusMinusLindbladNoiseOperator
{
    /// Returns the object in PlusMinusLindbladNoiseOperator form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the PlusMinusLindbladNoiseOperator.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in PlusMinusLindbladNoiseOperator form.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from_iter<
        I: IntoIterator<Item = ((PlusMinusProduct, PlusMinusProduct), CalculatorComplex)>,
    >(
        iter: I,
    ) -> Self {
        let mut slno = PlusMinusLindbladNoiseOperator::new();
        for (pair, cc) in iter {
            slno.add_operator_product(pair, cc)
                .expect("Internal bug in add_operator_product");
        }
        slno
    }
}

/// Implements the extend function (Extend trait) of PlusMinusLindbladNoiseOperator.
///
impl Extend<((PlusMinusProduct, PlusMinusProduct), CalculatorComplex)>
    for PlusMinusLindbladNoiseOperator
{
    /// Extends the PlusMinusLindbladNoiseOperator by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the PlusMinusLindbladNoiseOperator.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn extend<I: IntoIterator<Item = ((PlusMinusProduct, PlusMinusProduct), CalculatorComplex)>>(
        &mut self,
        iter: I,
    ) {
        for (pair, cc) in iter {
            self.add_operator_product(pair, cc)
                .expect("Internal bug in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of PlusMinusLindbladNoiseOperator.
///
impl fmt::Display for PlusMinusLindbladNoiseOperator {
    /// Formats the PlusMinusLindbladNoiseOperator using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted PlusMinusLindbladNoiseOperator.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "PlusMinusLindbladNoiseOperator{\n".to_string();
        for (key, val) in self.iter() {
            writeln!(output, "({}, {}): {},", key.0, key.1, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}

impl JordanWignerSpinToFermion for PlusMinusLindbladNoiseOperator {
    type Output = FermionLindbladNoiseOperator;

    /// Implements JordanWignerSpinToFermion for a PlusMinusLindbladNoiseOperator.
    ///
    /// The convention used is that |0> represents an empty fermionic state (spin-orbital),
    /// and |1> represents an occupied fermionic state.
    ///
    /// # Returns
    ///
    /// `FermionLindbladNoiseOperator` - The fermionic noise operator that results from the transformation.
    fn jordan_wigner(&self) -> Self::Output {
        let mut out = FermionLindbladNoiseOperator::new();

        for key in self.keys() {
            let fermion_operator_left = key.0.jordan_wigner();
            let fermion_operator_right = key.1.jordan_wigner();

            out.add_noise_from_full_operators(
                &fermion_operator_left,
                &fermion_operator_right,
                self.get(key).into(),
            )
            .expect("Internal bug in add_noise_from_full_operators");
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
        let pp: PlusMinusProduct = PlusMinusProduct::new().z(0);
        let sos = PlusMinusLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp.clone(), 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 1,
            },
        };
        let mut so = PlusMinusLindbladNoiseOperator::new();
        so.set((pp.clone(), pp), CalculatorComplex::from(0.5))
            .unwrap();

        assert_eq!(PlusMinusLindbladNoiseOperator::from(sos.clone()), so);
        assert_eq!(PlusMinusLindbladNoiseOperatorSerialize::from(so), sos);
    }
    // Test the Clone and PartialEq traits of SpinOperator
    #[test]
    fn clone_partial_eq() {
        let pp: PlusMinusProduct = PlusMinusProduct::new().z(0);
        let sos = PlusMinusLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 1,
            },
        };

        // Test Clone trait
        assert_eq!(sos.clone(), sos);

        // Test PartialEq trait
        let pp_1: PlusMinusProduct = PlusMinusProduct::new().z(0);
        let sos_1 = PlusMinusLindbladNoiseOperatorSerialize {
            items: vec![(pp_1.clone(), pp_1, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 1,
            },
        };
        let pp_2: PlusMinusProduct = PlusMinusProduct::new().z(2);
        let sos_2 = PlusMinusLindbladNoiseOperatorSerialize {
            items: vec![(pp_2.clone(), pp_2, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 1,
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
        let pp: PlusMinusProduct = PlusMinusProduct::new().z(0);
        let sos = PlusMinusLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 1,
            },
        };

        assert_eq!(
            format!("{:?}", sos),
            "PlusMinusLindbladNoiseOperatorSerialize { items: [(PlusMinusProduct { items: [(0, Z)] }, PlusMinusProduct { items: [(0, Z)] }, Float(0.5), Float(0.0))], _struqture_version: StruqtureVersionSerializable { major_version: 1, minor_version: 1 } }"
        );
    }

    /// Test SpinOperator Serialization and Deserialization traits (readable)
    #[test]
    fn serde_readable() {
        let pp = PlusMinusProduct::new().minus(0);
        let sos = PlusMinusLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 1,
            },
        };

        assert_tokens(
            &sos.readable(),
            &[
                Token::Struct {
                    name: "PlusMinusLindbladNoiseOperatorSerialize",
                    len: 2,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 4 },
                Token::Str("0-"),
                Token::Str("0-"),
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
                Token::U32(1),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    /// Test SpinOperator Serialization and Deserialization traits (compact)
    #[test]
    fn serde_compact() {
        let pp = PlusMinusProduct::new().plus(0);
        let sos = PlusMinusLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 1,
            },
        };

        assert_tokens(
            &sos.compact(),
            &[
                Token::Struct {
                    name: "PlusMinusLindbladNoiseOperatorSerialize",
                    len: 2,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 4 },
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 2 },
                Token::U64(0),
                Token::UnitVariant {
                    name: "SinglePlusMinusOperator",
                    variant: "Plus",
                },
                Token::TupleEnd,
                Token::SeqEnd,
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
                Token::Str("_struqture_version"),
                Token::Struct {
                    name: "StruqtureVersionSerializable",
                    len: 2,
                },
                Token::Str("major_version"),
                Token::U32(1),
                Token::Str("minor_version"),
                Token::U32(1),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
