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

use super::{MixedDecoherenceProduct, MixedIndex, OperateOnMixedSystems};
use crate::prelude::*;
use crate::{
    OperateOnDensityMatrix, StruqtureError, StruqtureVersionSerializable, MINIMUM_STRUQTURE_VERSION,
};
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Entry, Iter, Keys, Values};
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

/// MixedLindbladNoiseOperators represent noise interactions in the Lindblad equation.
///
/// In the Lindblad equation, Linblad noise operator L_i are not limited to [crate::mixed_systems::MixedDecoherenceProduct] style operators.
/// We use ([crate::mixed_systems::MixedDecoherenceProduct], [crate::mixed_systems::MixedDecoherenceProduct]) as a unique basis.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::mixed_systems::{MixedDecoherenceProduct, MixedLindbladNoiseOperator};
/// use struqture::spins::DecoherenceProduct;
/// use struqture::bosons::BosonProduct;
/// use struqture::fermions::FermionProduct;
///
/// let mut system = MixedLindbladNoiseOperator::new(1, 1, 1);
///
/// // Set noise terms:
/// let pp_01: MixedDecoherenceProduct = MixedDecoherenceProduct::new([DecoherenceProduct::new().x(0),], [BosonProduct::new([0], [1]).unwrap()], [FermionProduct::new([0], [2]).unwrap()]).unwrap();
/// let pp_0: MixedDecoherenceProduct = MixedDecoherenceProduct::new([DecoherenceProduct::new().z(0),], [BosonProduct::new([0], [1]).unwrap()], [FermionProduct::new([0], [2]).unwrap()]).unwrap();
/// system.set((pp_01.clone(), pp_01.clone()), CalculatorComplex::from(0.5)).unwrap();
/// system.set((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(system.get(&(pp_01.clone(), pp_01.clone())), &CalculatorComplex::from(0.5));
/// assert_eq!(system.get(&(pp_0.clone(), pp_0.clone())), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(from = "MixedLindbladNoiseOperatorSerialize")]
#[serde(into = "MixedLindbladNoiseOperatorSerialize")]
pub struct MixedLindbladNoiseOperator {
    /// The internal map representing the noise terms
    internal_map: HashMap<(MixedDecoherenceProduct, MixedDecoherenceProduct), CalculatorComplex>,
    /// Number of Spin subsystems
    n_spins: usize,
    /// Number of Boson subsystems
    n_bosons: usize,
    /// Number of Fermion subsystems
    n_fermions: usize,
}

impl crate::MinSupportedVersion for MixedLindbladNoiseOperator {}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for MixedLindbladNoiseOperator {
    fn schema_name() -> String {
        "MixedLindbladNoiseOperator".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <MixedLindbladNoiseOperatorSerialize>::json_schema(gen)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
struct MixedLindbladNoiseOperatorSerialize {
    /// The internal map representing the noise terms
    items: Vec<(
        MixedDecoherenceProduct,
        MixedDecoherenceProduct,
        CalculatorFloat,
        CalculatorFloat,
    )>,
    n_spins: usize,
    n_bosons: usize,
    n_fermions: usize,
    /// The struqture version
    _struqture_version: StruqtureVersionSerializable,
}

impl From<MixedLindbladNoiseOperatorSerialize> for MixedLindbladNoiseOperator {
    fn from(value: MixedLindbladNoiseOperatorSerialize) -> Self {
        let mut new_noise_op =
            MixedLindbladNoiseOperator::new(value.n_spins, value.n_bosons, value.n_fermions);
        for (key_l, key_r, real, imag) in value.items.iter() {
            new_noise_op
                .add_operator_product(
                    (key_l.clone(), key_r.clone()),
                    CalculatorComplex::new(real, imag),
                )
                .expect("Internal bug in add_operator_product");
        }
        new_noise_op
    }
}

impl From<MixedLindbladNoiseOperator> for MixedLindbladNoiseOperatorSerialize {
    fn from(value: MixedLindbladNoiseOperator) -> Self {
        let new_noise_op: Vec<(
            MixedDecoherenceProduct,
            MixedDecoherenceProduct,
            CalculatorFloat,
            CalculatorFloat,
        )> = value
            .clone()
            .into_iter()
            .map(|((left, right), val)| (left, right, val.re, val.im))
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

impl<'a> OperateOnDensityMatrix<'a> for MixedLindbladNoiseOperator {
    type Index = (MixedDecoherenceProduct, MixedDecoherenceProduct);
    type Value = CalculatorComplex;
    type IteratorType =
        Iter<'a, (MixedDecoherenceProduct, MixedDecoherenceProduct), CalculatorComplex>;
    type KeyIteratorType =
        Keys<'a, (MixedDecoherenceProduct, MixedDecoherenceProduct), CalculatorComplex>;
    type ValueIteratorType =
        Values<'a, (MixedDecoherenceProduct, MixedDecoherenceProduct), CalculatorComplex>;

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
            Some(cap) => Self::with_capacity(self.n_spins, self.n_bosons, self.n_fermions, cap),
            None => Self::new(self.n_spins, self.n_bosons, self.n_fermions),
        }
    }

    /// Overwrites an existing entry or sets a new entry in the MixedLindbladNoiseOperator with the given ((MixedDecoherenceProduct, MixedDecoherenceProduct) key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The (MixedDecoherenceProduct, MixedDecoherenceProduct) key to set in the MixedLindbladNoiseOperator.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the MixedLindbladNoiseOperator.
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
        if key.0.spins().len() != self.n_spins
            || key.0.bosons().len() != self.n_bosons
            || key.0.fermions().len() != self.n_fermions
        {
            return Err(StruqtureError::MissmatchedNumberSubsystems {
                target_number_spin_subsystems: self.n_spins,
                target_number_boson_subsystems: self.n_bosons,
                target_number_fermion_subsystems: self.n_fermions,
                actual_number_spin_subsystems: key.0.spins().len(),
                actual_number_boson_subsystems: key.0.bosons().len(),
                actual_number_fermion_subsystems: key.0.fermions().len(),
            });
        }
        if key.1.spins().len() != self.n_spins
            || key.1.bosons().len() != self.n_bosons
            || key.1.fermions().len() != self.n_fermions
        {
            return Err(StruqtureError::MissmatchedNumberSubsystems {
                target_number_spin_subsystems: self.n_spins,
                target_number_boson_subsystems: self.n_bosons,
                target_number_fermion_subsystems: self.n_fermions,
                actual_number_spin_subsystems: key.1.spins().len(),
                actual_number_boson_subsystems: key.1.bosons().len(),
                actual_number_fermion_subsystems: key.1.fermions().len(),
            });
        }
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
impl<'a> OperateOnMixedSystems<'a> for MixedLindbladNoiseOperator {
    // From trait
    fn number_spins(&self) -> Vec<usize> {
        self.current_number_spins()
    }

    // From trait
    fn current_number_spins(&self) -> Vec<usize> {
        let mut number_spins: Vec<usize> = (0..self.n_spins).map(|_| 0).collect();
        if !self.internal_map.is_empty() {
            for (key_left, key_right) in self.keys() {
                for (index, s) in key_left.spins().enumerate() {
                    let maxk = (s.current_number_spins()).max(s.current_number_spins());
                    if maxk > number_spins[index] {
                        number_spins[index] = maxk
                    }
                }
                for (index, s) in key_right.spins().enumerate() {
                    let maxk = (s.current_number_spins()).max(s.current_number_spins());
                    if maxk > number_spins[index] {
                        number_spins[index] = maxk
                    }
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
        if !self.internal_map.is_empty() {
            for (key_left, key_right) in self.keys() {
                for (index, b) in key_left.bosons().enumerate() {
                    let maxk = (b.current_number_modes()).max(b.current_number_modes());
                    if maxk > number_bosons[index] {
                        number_bosons[index] = maxk
                    }
                }
                for (index, b) in key_right.bosons().enumerate() {
                    let maxk = (b.current_number_modes()).max(b.current_number_modes());
                    if maxk > number_bosons[index] {
                        number_bosons[index] = maxk
                    }
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
        if !self.internal_map.is_empty() {
            for (key_left, key_right) in self.keys() {
                for (index, f) in key_left.fermions().enumerate() {
                    let maxk = (f.current_number_modes()).max(f.current_number_modes());
                    if maxk > number_fermions[index] {
                        number_fermions[index] = maxk
                    }
                }
                for (index, f) in key_right.fermions().enumerate() {
                    let maxk = (f.current_number_modes()).max(f.current_number_modes());
                    if maxk > number_fermions[index] {
                        number_fermions[index] = maxk
                    }
                }
            }
        }
        number_fermions
    }
}

/// Implements the default function (Default trait) of MixedLindbladNoiseOperator (an empty MixedLindbladNoiseOperator).
///
impl Default for MixedLindbladNoiseOperator {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

/// Functions for the MixedLindbladNoiseOperator
///
impl MixedLindbladNoiseOperator {
    /// Creates a new MixedLindbladNoiseOperator.
    ///
    /// # Arguments:
    ///
    /// * `n_spins` - Number of spin sub-systems
    /// * `n_bosons` - Number of bosonic sub-systems
    /// * `n_fermions` - Number of fermionic sub-systems
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) MixedLindbladNoiseOperator.
    pub fn new(n_spins: usize, n_bosons: usize, n_fermions: usize) -> Self {
        MixedLindbladNoiseOperator {
            internal_map: HashMap::new(),
            n_spins,
            n_bosons,
            n_fermions,
        }
    }

    /// Creates a new MixedLindbladNoiseOperator with capacity.
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
    /// * `Self` - The new (empty) MixedLindbladNoiseOperator.
    pub fn with_capacity(
        n_spins: usize,
        n_bosons: usize,
        n_fermions: usize,
        capacity: usize,
    ) -> Self {
        MixedLindbladNoiseOperator {
            internal_map: HashMap::with_capacity(capacity),
            n_spins,
            n_bosons,
            n_fermions,
        }
    }

    // /// Separate self into an operator with the terms of given number of qubits and an operator with the remaining operations
    // ///
    // /// # Arguments
    // ///
    // /// * `number_particles_left` - Number of spins, bosons and fermions to filter for in the left term of the keys.
    // /// * `number_particles_right` - Number of spins, bosons and fermions to filter for in the right term of the keys.
    // ///
    // /// # Returns
    // ///
    // /// `Ok((separated, remainder))` - Operator with the noise terms where number_particles matches the number of spins the operator product acts on and Operator with all other contributions.
    // pub fn separate_into_n_terms(
    //     &self,
    //     number_particles_left: (usize, usize, usize),
    //     number_particles_right: (usize, usize, usize),
    // ) -> Result<(Self, Self), StruqtureError> {
    //     let mut separated = Self::default();
    //     let mut remainder = Self::default();
    //     for ((prod_l, prod_r), val) in self.iter() {
    //         if (
    //             prod_l.spins().len(),
    //             prod_l.bosons().len(),
    //             prod_l.fermions().len(),
    //         ) == number_particles_left
    //             && (
    //                 prod_r.spins().len(),
    //                 prod_r.bosons().len(),
    //                 prod_r.fermions().len(),
    //             ) == number_particles_right
    //         {
    //             separated.add_operator_product((prod_l.clone(), prod_r.clone()), val.clone())?;
    //         } else {
    //             remainder.add_operator_product((prod_l.clone(), prod_r.clone()), val.clone())?;
    //         }
    //     }
    //     Ok((separated, remainder))
    // }
}

/// Implements the negative sign function of MixedLindbladNoiseOperator.
///
impl ops::Neg for MixedLindbladNoiseOperator {
    type Output = MixedLindbladNoiseOperator;
    /// Implement minus sign for MixedLindbladNoiseOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The MixedLindbladNoiseOperator * -1.
    fn neg(self) -> Self {
        let mut internal = HashMap::with_capacity(self.len());
        let n_spins = self.n_spins;
        let n_bosons = self.n_bosons;
        let n_fermions = self.n_fermions;
        for (key, val) in self {
            internal.insert(key.clone(), val.neg());
        }
        MixedLindbladNoiseOperator {
            internal_map: internal,
            n_spins,
            n_bosons,
            n_fermions,
        }
    }
}

/// Implements the plus function of MixedLindbladNoiseOperator by MixedLindbladNoiseOperator.
///
impl<T, V> ops::Add<T> for MixedLindbladNoiseOperator
where
    T: IntoIterator<Item = ((MixedDecoherenceProduct, MixedDecoherenceProduct), V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implements `+` (add) for two MixedLindbladNoiseOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The MixedLindbladNoiseOperator to be added.
    ///
    /// # Returns
    ///
    /// * `Self` - The two MixedLindbladNoiseOperators added together.
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

/// Implements the minus function of MixedLindbladNoiseOperator by MixedLindbladNoiseOperator.
///
impl<T, V> ops::Sub<T> for MixedLindbladNoiseOperator
where
    T: IntoIterator<Item = ((MixedDecoherenceProduct, MixedDecoherenceProduct), V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implements `-` (subtract) for two MixedLindbladNoiseOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The MixedLindbladNoiseOperator to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Self` - The two MixedLindbladNoiseOperators subtracted.
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

/// Implements the multiplication function of MixedLindbladNoiseOperator by CalculatorComplex/CalculatorFloat.
///
impl<T> ops::Mul<T> for MixedLindbladNoiseOperator
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for MixedLindbladNoiseOperator and CalculatorComplex/CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex or CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The MixedLindbladNoiseOperator multiplied by the CalculatorComplex/CalculatorFloat.
    fn mul(self, other: T) -> Self {
        let other_cc = Into::<CalculatorComplex>::into(other);
        let mut internal = HashMap::with_capacity(self.len());
        let n_spins = self.n_spins;
        let n_bosons = self.n_bosons;
        let n_fermions = self.n_fermions;
        for (key, val) in self {
            internal.insert(key, val * other_cc.clone());
        }
        MixedLindbladNoiseOperator {
            internal_map: internal,
            n_spins,
            n_bosons,
            n_fermions,
        }
    }
}

/// Implements the into_iter function (IntoIterator trait) of MixedLindbladNoiseOperator.
///
impl IntoIterator for MixedLindbladNoiseOperator {
    type Item = (
        (MixedDecoherenceProduct, MixedDecoherenceProduct),
        CalculatorComplex,
    );
    type IntoIter = std::collections::hash_map::IntoIter<
        (MixedDecoherenceProduct, MixedDecoherenceProduct),
        CalculatorComplex,
    >;
    /// Returns the MixedLindbladNoiseOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The MixedLindbladNoiseOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference MixedLindbladNoiseOperator.
///
impl<'a> IntoIterator for &'a MixedLindbladNoiseOperator {
    type Item = (
        &'a (MixedDecoherenceProduct, MixedDecoherenceProduct),
        &'a CalculatorComplex,
    );
    type IntoIter = Iter<'a, (MixedDecoherenceProduct, MixedDecoherenceProduct), CalculatorComplex>;

    /// Returns the reference MixedLindbladNoiseOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference MixedLindbladNoiseOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of MixedLindbladNoiseOperator.
///
impl
    FromIterator<(
        (MixedDecoherenceProduct, MixedDecoherenceProduct),
        CalculatorComplex,
    )> for MixedLindbladNoiseOperator
{
    /// Returns the object in MixedLindbladNoiseOperator form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the MixedLindbladNoiseOperator.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in MixedLindbladNoiseOperator form.
    ///
    /// # Panics
    ///
    /// * Internal error in set.
    /// * Internal error in add_operator_product.
    fn from_iter<
        I: IntoIterator<
            Item = (
                (MixedDecoherenceProduct, MixedDecoherenceProduct),
                CalculatorComplex,
            ),
        >,
    >(
        iter: I,
    ) -> Self {
        let mut iterator = iter.into_iter();
        match iterator.next() {
            Some(first_element) => {
                let spins = first_element.0 .0.spins().len();
                let bosons = first_element.0 .0.bosons().len();
                let fermions = first_element.0 .0.fermions().len();
                let mut slno = MixedLindbladNoiseOperator::new(spins, bosons, fermions);
                slno.set(first_element.0, first_element.1)
                    .expect("Internal error in set");
                for (pair, cc) in iterator {
                    slno.add_operator_product(pair, cc)
                        .expect("Internal error in add_operator_product");
                }
                slno
            }
            None => MixedLindbladNoiseOperator::new(0, 0, 0),
        }
    }
}

/// Implements the extend function (Extend trait) of MixedLindbladNoiseOperator.
///
impl
    Extend<(
        (MixedDecoherenceProduct, MixedDecoherenceProduct),
        CalculatorComplex,
    )> for MixedLindbladNoiseOperator
{
    /// Extends the MixedLindbladNoiseOperator by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the MixedLindbladNoiseOperator.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn extend<
        I: IntoIterator<
            Item = (
                (MixedDecoherenceProduct, MixedDecoherenceProduct),
                CalculatorComplex,
            ),
        >,
    >(
        &mut self,
        iter: I,
    ) {
        for (pair, cc) in iter {
            self.add_operator_product(pair, cc)
                .expect("Internal error in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of MixedLindbladNoiseOperator.
///
impl fmt::Display for MixedLindbladNoiseOperator {
    /// Formats the MixedLindbladNoiseOperator using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted MixedLindbladNoiseOperator.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "MixedLindbladNoiseOperator{\n".to_string();
        for (key, val) in self.iter() {
            writeln!(output, "({}, {}): {},", key.0, key.1, val)?;
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
    use crate::spins::DecoherenceProduct;
    use serde_test::{assert_tokens, Configure, Token};
    use std::str::FromStr;

    // Test the Clone and PartialEq traits of SpinOperator
    #[test]
    fn so_from_sos() {
        let spins = DecoherenceProduct::from_str("0X").unwrap();
        let creators = &[0];
        let annihilators = &[3];
        let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
        let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
        let pp = MixedDecoherenceProduct::new([spins], [bosons], [fermions]).unwrap();

        let sos = MixedLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp.clone(), 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        let mut so = MixedLindbladNoiseOperator::new(1, 1, 1);
        so.set((pp.clone(), pp), CalculatorComplex::from(0.5))
            .unwrap();

        assert_eq!(MixedLindbladNoiseOperator::from(sos.clone()), so);
        assert_eq!(MixedLindbladNoiseOperatorSerialize::from(so), sos);
    }
    // Test the Clone and PartialEq traits of SpinOperator
    #[test]
    fn clone_partial_eq() {
        let spins = DecoherenceProduct::from_str("0X").unwrap();
        let creators = &[0];
        let annihilators = &[3];
        let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
        let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
        let pp = MixedDecoherenceProduct::new([spins], [bosons], [fermions]).unwrap();
        let sos = MixedLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp, 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        // Test Clone trait
        assert_eq!(sos.clone(), sos);

        // Test PartialEq trait
        let spins = DecoherenceProduct::from_str("0X").unwrap();
        let creators = &[0];
        let annihilators = &[3];
        let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
        let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
        let pp_1 = MixedDecoherenceProduct::new([spins], [bosons], [fermions]).unwrap();
        let sos_1 = MixedLindbladNoiseOperatorSerialize {
            items: vec![(pp_1.clone(), pp_1, 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        let spins = DecoherenceProduct::from_str("0X").unwrap();
        let creators = &[0];
        let annihilators = &[4];
        let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
        let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
        let pp_2 = MixedDecoherenceProduct::new([spins], [bosons], [fermions]).unwrap();
        let sos_2 = MixedLindbladNoiseOperatorSerialize {
            items: vec![(pp_2.clone(), pp_2, 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
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
        let spins = DecoherenceProduct::from_str("0X").unwrap();
        let creators = &[0];
        let annihilators = &[3];
        let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
        let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
        let pp = MixedDecoherenceProduct::new([spins], [bosons], [fermions]).unwrap();
        let sos = MixedLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp, 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        assert_eq!(
            format!("{:?}", sos),
            "MixedLindbladNoiseOperatorSerialize { items: [(MixedDecoherenceProduct { spins: [DecoherenceProduct { items: [(0, X)] }], bosons: [BosonProduct { creators: [0], annihilators: [3] }], fermions: [FermionProduct { creators: [0], annihilators: [3] }] }, MixedDecoherenceProduct { spins: [DecoherenceProduct { items: [(0, X)] }], bosons: [BosonProduct { creators: [0], annihilators: [3] }], fermions: [FermionProduct { creators: [0], annihilators: [3] }] }, Float(0.5), Float(0.0))], n_spins: 1, n_bosons: 1, n_fermions: 1, _struqture_version: StruqtureVersionSerializable { major_version: 1, minor_version: 0 } }"
        );
    }

    /// Test SpinOperator Serialization and Deserialization traits (readable)
    #[test]
    fn serde_readable() {
        let spins = DecoherenceProduct::from_str("0X").unwrap();
        let creators = &[0];
        let annihilators = &[3];
        let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
        let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
        let pp = MixedDecoherenceProduct::new([spins], [bosons], [fermions]).unwrap();
        let sos = MixedLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp, 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        assert_tokens(
            &sos.readable(),
            &[
                Token::Struct {
                    name: "MixedLindbladNoiseOperatorSerialize",
                    len: 5,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 4 },
                Token::Str("S0X:Bc0a3:Fc0a3:"),
                Token::Str("S0X:Bc0a3:Fc0a3:"),
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

    /// Test SpinOperator Serialization and Deserialization traits (compact)
    #[test]
    fn serde_compact() {
        let spins = DecoherenceProduct::from_str("0X").unwrap();
        let creators = &[0];
        let annihilators = &[3];
        let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
        let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
        let pp = MixedDecoherenceProduct::new([spins], [bosons], [fermions]).unwrap();
        let sos = MixedLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp, 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        assert_tokens(
            &sos.compact(),
            &[
                Token::Struct {
                    name: "MixedLindbladNoiseOperatorSerialize",
                    len: 5,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 4 },
                Token::Tuple { len: 3 },
                Token::Seq { len: Some(1) },
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 2 },
                Token::U64(0),
                Token::UnitVariant {
                    name: "SingleDecoherenceOperator",
                    variant: "X",
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
                Token::U64(3),
                Token::SeqEnd,
                Token::TupleEnd,
                Token::SeqEnd,
                Token::TupleEnd,
                Token::Tuple { len: 3 },
                Token::Seq { len: Some(1) },
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 2 },
                Token::U64(0),
                Token::UnitVariant {
                    name: "SingleDecoherenceOperator",
                    variant: "X",
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
                Token::U64(3),
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
