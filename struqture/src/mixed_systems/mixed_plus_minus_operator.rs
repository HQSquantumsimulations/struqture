// Copyright © 2021-2022 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use super::{MixedOperator, MixedPlusMinusProduct, MixedProduct, OperateOnMixedSystems};
use crate::{
    ModeIndex, OperateOnDensityMatrix, OperateOnState, StruqtureError,
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

/// MixedOperators are combinations of MixedProducts with specific CalculatorComplex coefficients.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::spins::PlusMinusProduct;
/// use struqture::bosons::BosonProduct;
/// use struqture::fermions::FermionProduct;
/// use struqture::mixed_systems::{MixedPlusMinusProduct, MixedPlusMinusOperator};
///
/// let mut sh = MixedPlusMinusOperator::new(1, 1, 1);
///
/// let mp_1: MixedPlusMinusProduct = MixedPlusMinusProduct::new([PlusMinusProduct::new().plus(0),], [BosonProduct::new([0], [1]).unwrap()], [FermionProduct::new([0], [1]).unwrap()]);
/// let mp_0: MixedPlusMinusProduct = MixedPlusMinusProduct::new([PlusMinusProduct::new().z(0),], [BosonProduct::new([0], [1]).unwrap()], [FermionProduct::new([0], [1]).unwrap()]);
/// sh.set(mp_1.clone(), CalculatorComplex::from(0.5)).unwrap();
/// sh.set(mp_0.clone(), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(sh.get(&mp_1), &CalculatorComplex::from(0.5));
/// assert_eq!(sh.get(&mp_0), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "MixedPlusMinusOperatorSerialize")]
#[serde(into = "MixedPlusMinusOperatorSerialize")]
pub struct MixedPlusMinusOperator {
    /// The internal HashMap of MixedProducts and coefficients (CalculatorComplex)
    internal_map: HashMap<MixedPlusMinusProduct, CalculatorComplex>,
    /// Number of Spin subsystems
    n_spins: usize,
    /// Number of Boson subsystems
    n_bosons: usize,
    /// Number of Fermion subsystems
    n_fermions: usize,
}

impl crate::MinSupportedVersion for MixedPlusMinusOperator {
    fn min_supported_version() -> (usize, usize, usize) {
        (1, 2, 0)
    }
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for MixedPlusMinusOperator {
    fn schema_name() -> String {
        "MixedPlusMinusOperator".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <MixedPlusMinusOperatorSerialize>::json_schema(gen)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
struct MixedPlusMinusOperatorSerialize {
    items: Vec<(MixedPlusMinusProduct, CalculatorFloat, CalculatorFloat)>,
    n_spins: usize,
    n_bosons: usize,
    n_fermions: usize,
    _struqture_version: StruqtureVersionSerializable,
}

impl From<MixedPlusMinusOperatorSerialize> for MixedPlusMinusOperator {
    fn from(value: MixedPlusMinusOperatorSerialize) -> Self {
        let mut new_noise_op =
            MixedPlusMinusOperator::new(value.n_spins, value.n_bosons, value.n_fermions);
        for (key, real, imag) in value.items.iter() {
            let _ =
                new_noise_op.add_operator_product(key.clone(), CalculatorComplex::new(real, imag));
        }
        new_noise_op
    }
}

impl From<MixedPlusMinusOperator> for MixedPlusMinusOperatorSerialize {
    fn from(value: MixedPlusMinusOperator) -> Self {
        let new_noise_op: Vec<(MixedPlusMinusProduct, CalculatorFloat, CalculatorFloat)> = value
            .clone()
            .into_iter()
            .map(|(key, val)| (key, val.re, val.im))
            .collect();
        let current_version = StruqtureVersionSerializable {
            major_version: MINIMUM_STRUQTURE_VERSION.0,
            minor_version: MINIMUM_STRUQTURE_VERSION.1,
        };
        Self {
            items: new_noise_op,
            n_spins: value.n_spins,
            n_bosons: value.n_bosons,
            n_fermions: value.n_fermions,
            _struqture_version: current_version,
        }
    }
}

impl<'a> OperateOnDensityMatrix<'a> for MixedPlusMinusOperator {
    type Index = MixedPlusMinusProduct;
    type Value = CalculatorComplex;
    type IteratorType = Iter<'a, Self::Index, Self::Value>;
    type KeyIteratorType = Keys<'a, Self::Index, Self::Value>;
    type ValueIteratorType = Values<'a, Self::Index, Self::Value>;

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
    fn remove(&mut self, key: &MixedPlusMinusProduct) -> Option<CalculatorComplex> {
        self.internal_map.remove(key)
    }

    // From trait
    fn empty_clone(&self, capacity: Option<usize>) -> Self {
        match capacity {
            Some(cap) => Self::with_capacity(self.n_spins, self.n_bosons, self.n_fermions, cap),
            None => Self::new(self.n_spins, self.n_bosons, self.n_fermions),
        }
    }

    /// Overwrites an existing entry or sets a new entry in the MixedPlusMinusOperator with the given (MixedPlusMinusProduct key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The MixedPlusMinusProduct key to set in the MixedPlusMinusOperator.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the MixedPlusMinusOperator.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(CalculatorComplex))` - The key existed, this is the value it had before it was set with the value input.
    /// * `Ok(None)` - The key did not exist, it has been set with its corresponding value.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and key do not match.
    fn set(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<Option<Self::Value>, StruqtureError> {
        if key.spins().len() != self.n_spins
            || key.bosons().len() != self.n_bosons
            || key.fermions().len() != self.n_fermions
        {
            return Err(StruqtureError::MissmatchedNumberSubsystems {
                target_number_spin_subsystems: self.n_spins,
                target_number_boson_subsystems: self.n_bosons,
                target_number_fermion_subsystems: self.n_fermions,
                actual_number_spin_subsystems: key.spins().len(),
                actual_number_boson_subsystems: key.bosons().len(),
                actual_number_fermion_subsystems: key.fermions().len(),
            });
        }
        if value.re != CalculatorFloat::ZERO || value.im != CalculatorFloat::ZERO {
            // Catch on diagonals with non-zero imaginary values
            Ok(self.internal_map.insert(key, value))
        } else {
            match self.internal_map.entry(key) {
                Entry::Occupied(val) => Ok(Some(val.remove())),
                Entry::Vacant(_) => Ok(None),
            }
        }
    }
}

impl<'a> OperateOnState<'a> for MixedPlusMinusOperator {
    // From trait
    fn hermitian_conjugate(&self) -> Self {
        self.clone()
    }
}

impl<'a> OperateOnMixedSystems<'a> for MixedPlusMinusOperator {
    // From trait
    fn number_spins(&self) -> Vec<usize> {
        self.current_number_spins()
    }

    // From trait
    fn current_number_spins(&self) -> Vec<usize> {
        let mut number_spins: Vec<usize> = (0..self.n_spins).map(|_| 0).collect();
        for key in self.keys() {
            for (index, s) in key.spins().enumerate() {
                let maxk = s.current_number_spins();
                if maxk > number_spins[index] {
                    number_spins[index] = maxk
                }
            }
        }
        number_spins
    }

    // From trait
    fn number_bosonic_modes(&self) -> Vec<usize> {
        self.current_number_bosonic_modes()
    }

    // From trait
    fn current_number_bosonic_modes(&self) -> Vec<usize> {
        let mut number_bosons: Vec<usize> = (0..self.n_bosons).map(|_| 0).collect();
        for key in self.keys() {
            for (index, s) in key.bosons().enumerate() {
                let maxk = s.current_number_modes();
                if maxk > number_bosons[index] {
                    number_bosons[index] = maxk
                }
            }
        }
        number_bosons
    }

    // From trait
    fn number_fermionic_modes(&self) -> Vec<usize> {
        self.current_number_fermionic_modes()
    }

    // From trait
    fn current_number_fermionic_modes(&self) -> Vec<usize> {
        let mut number_fermions: Vec<usize> = (0..self.n_fermions).map(|_| 0).collect();
        for key in self.keys() {
            for (index, s) in key.fermions().enumerate() {
                let maxk = s.current_number_modes();
                if maxk > number_fermions[index] {
                    number_fermions[index] = maxk
                }
            }
        }
        number_fermions
    }
}

/// Implements the default function (Default trait) of MixedPlusMinusOperator (an empty MixedPlusMinusOperator).
///
impl Default for MixedPlusMinusOperator {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

/// Functions for the MixedPlusMinusOperator
///
impl MixedPlusMinusOperator {
    /// Creates a new MixedPlusMinusOperator.
    ///
    /// # Arguments:
    ///
    /// * `n_spins` - Number of spin sub-systems
    /// * `n_bosons` - Number of bosonic sub-systems
    /// * `n_fermions` - Number of fermionic sub-systems
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) MixedPlusMinusOperator.
    pub fn new(n_spins: usize, n_bosons: usize, n_fermions: usize) -> Self {
        MixedPlusMinusOperator {
            internal_map: HashMap::new(),
            n_spins,
            n_bosons,
            n_fermions,
        }
    }

    /// Creates a new MixedPlusMinusOperator with capacity.
    ///
    /// # Arguments
    ///
    /// * `n_spins` - The number of spin sub-systems.
    /// * `n_bosons` - The number of boson sub-systems.
    /// * `n_fermions` - The number of fermion sub-systems.
    /// * `capacity` - The pre-allocated capacity of the operator.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) MixedPlusMinusOperator.
    pub fn with_capacity(
        n_spins: usize,
        n_bosons: usize,
        n_fermions: usize,
        capacity: usize,
    ) -> Self {
        Self {
            internal_map: HashMap::with_capacity(capacity),
            n_spins,
            n_bosons,
            n_fermions,
        }
    }

    // /// Separate self into an operator with the terms of given number of spins, bosons and fermions and an operator with the remaining operations
    // ///
    // /// # Arguments
    // ///
    // /// * `number_particles` - Number of spins, bosons and fermions to filter for in the keys.
    // ///
    // /// # Returns
    // ///
    // /// `Ok((separated, remainder))` - Operator with the noise terms where number_particles matches the number of spins the operator product acts on and Operator with all other contributions.
    // pub fn separate_into_n_terms(
    //     &self,
    //     number_particles: (usize, usize, usize),
    // ) -> Result<(Self, Self), StruqtureError> {
    //     let mut separated = Self::default();
    //     let mut remainder = Self::default();
    //     for (prod, val) in self.iter() {
    //         if (
    //             prod.spins().len(),
    //             prod.bosons().len(),
    //             prod.fermions().len(),
    //         ) == number_particles
    //         {
    //             separated.add_operator_product(prod.clone(), val.clone())?;
    //         } else {
    //             remainder.add_operator_product(prod.clone(), val.clone())?;
    //         }
    //     }
    //     Ok((separated, remainder))
    // }
}

impl TryFrom<MixedPlusMinusOperator> for MixedOperator {
    type Error = StruqtureError;
    /// Converts a MixedPlusMinusOperator into a MixedOperator.
    ///
    /// # Arguments
    ///
    /// * `value` - The MixedPlusMinusOperator to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The MixedPlusMinusOperator converted into a MixedOperator.
    fn try_from(value: MixedPlusMinusOperator) -> Result<Self, Self::Error> {
        let mut new_operator = MixedOperator::with_capacity(
            value.n_spins,
            value.n_bosons,
            value.n_fermions,
            2 * value.len(),
        );
        for (product, val) in value.into_iter() {
            let transscribed_vector: Vec<(MixedProduct, Complex64)> = product.try_into()?;
            for (transscribed_product, prefactor) in transscribed_vector {
                new_operator
                    .add_operator_product(transscribed_product, val.clone() * prefactor)
                    .expect("Unexpected error adding operators. Internal struqture error");
            }
        }
        Ok(new_operator)
    }
}

impl From<MixedOperator> for MixedPlusMinusOperator {
    /// Converts a MixedOperator into a MixedPlusMinusOperator.
    ///
    /// # Arguments
    ///
    /// * `value` - The MixedOperator to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The MixedOperator converted into a MixedPlusMinusOperator.
    fn from(value: MixedOperator) -> Self {
        let mut new_operator = MixedPlusMinusOperator::with_capacity(
            value.current_number_spins().len(),
            value.current_number_bosonic_modes().len(),
            value.current_number_fermionic_modes().len(),
            2 * value.len(),
        );
        for (product, val) in value.into_iter() {
            let transscribed_vector: Vec<(MixedPlusMinusProduct, Complex64)> = product.into();
            for (transscribed_product, prefactor) in transscribed_vector {
                new_operator
                    .add_operator_product(transscribed_product, val.clone() * prefactor)
                    .expect("Unexpected error adding operators. Internal struqture error");
            }
        }
        new_operator
    }
}

/// Implements the negative sign function of MixedPlusMinusOperator.
///
impl ops::Neg for MixedPlusMinusOperator {
    type Output = MixedPlusMinusOperator;
    /// Implement minus sign for MixedPlusMinusOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The MixedPlusMinusOperator * -1.
    fn neg(self) -> Self {
        let mut internal = self.internal_map.clone();
        let n_spins = self.n_spins;
        let n_bosons = self.n_bosons;
        let n_fermions = self.n_fermions;
        for key in self.keys() {
            internal.insert(key.clone(), internal[key].clone() * -1.0);
        }
        MixedPlusMinusOperator {
            internal_map: internal,
            n_spins,
            n_bosons,
            n_fermions,
        }
    }
}

/// Implements the plus function of MixedPlusMinusOperator by MixedPlusMinusOperator.
///
impl<T, V> ops::Add<T> for MixedPlusMinusOperator
where
    T: IntoIterator<Item = (MixedPlusMinusProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `+` (add) for two MixedOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The MixedPlusMinusOperator to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two MixedOperators added together.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and key do not match.
    fn add(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value))?;
        }
        Ok(self)
    }
}

/// Implements the minus function of MixedPlusMinusOperator by MixedPlusMinusOperator.
///
impl<T, V> ops::Sub<T> for MixedPlusMinusOperator
where
    T: IntoIterator<Item = (MixedPlusMinusProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `-` (subtract) for two MixedOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The MixedPlusMinusOperator to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two MixedOperators subtracted.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and key do not match.
    fn sub(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value) * -1.0)?;
        }
        Ok(self)
    }
}

/// Implements the multiplication function of MixedPlusMinusOperator by CalculatorComplex/CalculatorFloat.
///
impl<T> ops::Mul<T> for MixedPlusMinusOperator
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for MixedPlusMinusOperator and CalculatorComplex/CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex or CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The MixedPlusMinusOperator multiplied by the CalculatorComplex/CalculatorFloat.
    fn mul(self, other: T) -> Self {
        let other_cc = Into::<CalculatorComplex>::into(other);
        let mut internal = self.internal_map.clone();
        let n_spins = self.n_spins;
        let n_bosons = self.n_bosons;
        let n_fermions = self.n_fermions;
        for key in self.keys() {
            internal.insert(key.clone(), internal[key].clone() * other_cc.clone());
        }
        MixedPlusMinusOperator {
            internal_map: internal,
            n_spins,
            n_bosons,
            n_fermions,
        }
    }
}

/// Implements the into_iter function (IntoIterator trait) of MixedPlusMinusOperator.
///
impl IntoIterator for MixedPlusMinusOperator {
    type Item = (MixedPlusMinusProduct, CalculatorComplex);
    type IntoIter = std::collections::hash_map::IntoIter<MixedPlusMinusProduct, CalculatorComplex>;
    /// Returns the MixedPlusMinusOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The MixedPlusMinusOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference MixedPlusMinusOperator.
///
impl<'a> IntoIterator for &'a MixedPlusMinusOperator {
    type Item = (&'a MixedPlusMinusProduct, &'a CalculatorComplex);
    type IntoIter = Iter<'a, MixedPlusMinusProduct, CalculatorComplex>;

    /// Returns the reference MixedPlusMinusOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference MixedPlusMinusOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of MixedPlusMinusOperator.
///
impl FromIterator<(MixedPlusMinusProduct, CalculatorComplex)> for MixedPlusMinusOperator {
    /// Returns the object in MixedPlusMinusOperator form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the MixedPlusMinusOperator.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in MixedPlusMinusOperator form.
    ///
    /// # Panics
    ///
    /// * Internal error in set.
    /// * Internal error in add_operator_product.
    fn from_iter<I: IntoIterator<Item = (MixedPlusMinusProduct, CalculatorComplex)>>(
        iter: I,
    ) -> Self {
        let mut iterator = iter.into_iter();
        match iterator.next() {
            Some(first_element) => {
                let spins = first_element.0.spins().len();
                let bosons = first_element.0.bosons().len();
                let fermions = first_element.0.fermions().len();
                let mut mpmo = MixedPlusMinusOperator::new(spins, bosons, fermions);
                mpmo.set(first_element.0, first_element.1)
                    .expect("Internal error in set");
                for (pair, cc) in iterator {
                    mpmo.add_operator_product(pair, cc)
                        .expect("Internal error in add_operator_product");
                }
                mpmo
            }
            None => MixedPlusMinusOperator::new(0, 0, 0),
        }
    }
}

/// Implements the extend function (Extend trait) of MixedPlusMinusOperator.
///
impl Extend<(MixedPlusMinusProduct, CalculatorComplex)> for MixedPlusMinusOperator {
    /// Extends the MixedPlusMinusOperator by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the MixedPlusMinusOperator.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn extend<I: IntoIterator<Item = (MixedPlusMinusProduct, CalculatorComplex)>>(
        &mut self,
        iter: I,
    ) {
        for (pp, cc) in iter {
            self.add_operator_product(pp, cc)
                .expect("Internal error in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of MixedPlusMinusOperator.
///
impl fmt::Display for MixedPlusMinusOperator {
    /// Formats the MixedPlusMinusOperator using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted MixedPlusMinusOperator.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "MixedPlusMinusOperator{\n".to_string();
        for (key, val) in self.iter() {
            writeln!(output, "{}: {},", key, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bosons::BosonProduct;
    use crate::fermions::FermionProduct;
    use crate::spins::PlusMinusProduct;
    use serde_test::{assert_tokens, Configure, Token};

    // Test the Clone and PartialEq traits of MixedOperator
    #[test]
    fn mpmo_from_mpmos() {
        let pp: MixedPlusMinusProduct = MixedPlusMinusProduct::new(
            [PlusMinusProduct::new().z(2)],
            [BosonProduct::new([0], [3]).unwrap()],
            [FermionProduct::new([0], [2]).unwrap()],
        );
        let mpmos = MixedPlusMinusOperatorSerialize {
            items: vec![(pp.clone(), 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        let mut mpmo = MixedPlusMinusOperator::new(1, 1, 1);
        mpmo.set(pp, CalculatorComplex::from(0.5)).unwrap();

        assert_eq!(MixedPlusMinusOperator::from(mpmos.clone()), mpmo);
        assert_eq!(MixedPlusMinusOperatorSerialize::from(mpmo), mpmos);
    }
    // Test the Clone and PartialEq traits of MixedOperator
    #[test]
    fn clone_partial_eq() {
        let pp: MixedPlusMinusProduct = MixedPlusMinusProduct::new(
            [PlusMinusProduct::new().z(2)],
            [BosonProduct::new([0], [3]).unwrap()],
            [FermionProduct::new([0], [2]).unwrap()],
        );
        let mpmos = MixedPlusMinusOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        // Test Clone trait
        assert_eq!(mpmos.clone(), mpmos);

        // Test PartialEq trait
        let pp_1: MixedPlusMinusProduct = MixedPlusMinusProduct::new(
            [PlusMinusProduct::new().z(2)],
            [BosonProduct::new([0], [3]).unwrap()],
            [FermionProduct::new([0], [2]).unwrap()],
        );
        let mpmos_1 = MixedPlusMinusOperatorSerialize {
            items: vec![(pp_1, 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        let pp_2: MixedPlusMinusProduct = MixedPlusMinusProduct::new(
            [PlusMinusProduct::new().z(0)],
            [BosonProduct::new([0], [3]).unwrap()],
            [FermionProduct::new([0], [2]).unwrap()],
        );
        let mpmos_2 = MixedPlusMinusOperatorSerialize {
            items: vec![(pp_2, 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        assert!(mpmos_1 == mpmos);
        assert!(mpmos == mpmos_1);
        assert!(mpmos_2 != mpmos);
        assert!(mpmos != mpmos_2);
    }

    // Test the Debug trait of MixedOperator
    #[test]
    fn debug() {
        let pp: MixedPlusMinusProduct = MixedPlusMinusProduct::new(
            [PlusMinusProduct::new().z(2)],
            [BosonProduct::new([0], [3]).unwrap()],
            [FermionProduct::new([0], [2]).unwrap()],
        );
        let mpmos = MixedPlusMinusOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        assert_eq!(
            format!("{:?}", mpmos),
            "MixedPlusMinusOperatorSerialize { items: [(MixedPlusMinusProduct { spins: [PlusMinusProduct { items: [(2, Z)] }], bosons: [BosonProduct { creators: [0], annihilators: [3] }], fermions: [FermionProduct { creators: [0], annihilators: [2] }] }, Float(0.5), Float(0.0))], n_spins: 1, n_bosons: 1, n_fermions: 1, _struqture_version: StruqtureVersionSerializable { major_version: 1, minor_version: 0 } }"
        );
    }

    /// Test MixedOperator Serialization and Deserialization traits (readable)
    #[test]
    fn serde_readable() {
        let pp: MixedPlusMinusProduct = MixedPlusMinusProduct::new(
            [PlusMinusProduct::new().z(2)],
            [BosonProduct::new([0], [3]).unwrap()],
            [FermionProduct::new([0], [2]).unwrap()],
        );
        let mpmos = MixedPlusMinusOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        assert_tokens(
            &mpmos.readable(),
            &[
                Token::Struct {
                    name: "MixedPlusMinusOperatorSerialize",
                    len: 5,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 3 },
                Token::Str("S2Z:Bc0a3:Fc0a2:"),
                Token::F64(0.5),
                Token::F64(0.0),
                Token::TupleEnd,
                Token::SeqEnd,
                Token::Str("n_spins"),
                Token::U64(1),
                Token::Str("n_bosons"),
                Token::U64(1),
                Token::Str("n_fermions"),
                Token::U64(1),
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

    /// Test MixedOperator Serialization and Deserialization traits (compact)
    #[test]
    fn serde_compact() {
        let pp: MixedPlusMinusProduct = MixedPlusMinusProduct::new(
            [PlusMinusProduct::new().z(2)],
            [BosonProduct::new([0], [3]).unwrap()],
            [FermionProduct::new([0], [2]).unwrap()],
        );
        let mpmos = MixedPlusMinusOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        assert_tokens(
            &mpmos.compact(),
            &[
                Token::Struct {
                    name: "MixedPlusMinusOperatorSerialize",
                    len: 5,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 3 },
                Token::Tuple { len: 3 },
                Token::Seq { len: Some(1) },
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 2 },
                Token::U64(2),
                Token::UnitVariant {
                    name: "SinglePlusMinusOperator",
                    variant: "Z",
                },
                Token::TupleEnd,
                Token::SeqEnd,
                Token::SeqEnd,
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 2 },
                Token::Seq { len: Some(1) },
                Token::U64(0),
                Token::SeqEnd,
                Token::Seq { len: Some(1) },
                Token::U64(3),
                Token::SeqEnd,
                Token::TupleEnd,
                Token::SeqEnd,
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 2 },
                Token::Seq { len: Some(1) },
                Token::U64(0),
                Token::SeqEnd,
                Token::Seq { len: Some(1) },
                Token::U64(2),
                Token::SeqEnd,
                Token::TupleEnd,
                Token::SeqEnd,
                Token::TupleEnd,
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
                Token::Str("n_spins"),
                Token::U64(1),
                Token::Str("n_bosons"),
                Token::U64(1),
                Token::Str("n_fermions"),
                Token::U64(1),
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
