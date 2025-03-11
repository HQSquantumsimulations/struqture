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
use crate::{OperateOnDensityMatrix, StruqtureError};
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

use indexmap::map::{Entry, Iter};
use indexmap::IndexMap;

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
#[serde(try_from = "MixedLindbladNoiseOperatorSerialize")]
#[serde(into = "MixedLindbladNoiseOperatorSerialize")]
pub struct MixedLindbladNoiseOperator {
    /// The internal map representing the noise terms
    internal_map: IndexMap<(MixedDecoherenceProduct, MixedDecoherenceProduct), CalculatorComplex>,
    /// Number of Spin subsystems
    pub(crate) n_spins: usize,
    /// Number of Boson subsystems
    pub(crate) n_bosons: usize,
    /// Number of Fermion subsystems
    pub(crate) n_fermions: usize,
}

impl crate::SerializationSupport for MixedLindbladNoiseOperator {
    fn struqture_type() -> crate::StruqtureType {
        crate::StruqtureType::MixedLindbladNoiseOperator
    }
}
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
    serialisation_meta: crate::StruqtureSerialisationMeta,
}

impl TryFrom<MixedLindbladNoiseOperatorSerialize> for MixedLindbladNoiseOperator {
    type Error = StruqtureError;
    fn try_from(value: MixedLindbladNoiseOperatorSerialize) -> Result<Self, Self::Error> {
        let target_serialisation_meta =
            <Self as crate::SerializationSupport>::target_serialisation_meta();
        crate::check_can_be_deserialised(&target_serialisation_meta, &value.serialisation_meta)?;
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
        Ok(new_noise_op)
    }
}

impl From<MixedLindbladNoiseOperator> for MixedLindbladNoiseOperatorSerialize {
    fn from(value: MixedLindbladNoiseOperator) -> Self {
        let serialisation_meta = crate::SerializationSupport::struqture_serialisation_meta(&value);
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
        Self {
            items: new_noise_op,
            n_spins: value.n_spins,
            n_bosons: value.n_bosons,
            n_fermions: value.n_fermions,
            serialisation_meta,
        }
    }
}

impl<'a> OperateOnDensityMatrix<'a> for MixedLindbladNoiseOperator {
    type Index = (MixedDecoherenceProduct, MixedDecoherenceProduct);
    type Value = CalculatorComplex;

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
                Entry::Occupied(val) => Ok(Some(val.shift_remove())),
                Entry::Vacant(_) => Ok(None),
            }
        }
    }
}
impl OperateOnMixedSystems<'_> for MixedLindbladNoiseOperator {
    // From trait
    fn current_number_spins(&self) -> Vec<usize> {
        let mut current_number_spins: Vec<usize> = (0..self.n_spins).map(|_| 0).collect();
        if !self.internal_map.is_empty() {
            for (key_left, key_right) in self.keys() {
                for (index, s) in key_left.spins().enumerate() {
                    let maxk = (s.current_number_spins()).max(s.current_number_spins());
                    if maxk > current_number_spins[index] {
                        current_number_spins[index] = maxk
                    }
                }
                for (index, s) in key_right.spins().enumerate() {
                    let maxk = (s.current_number_spins()).max(s.current_number_spins());
                    if maxk > current_number_spins[index] {
                        current_number_spins[index] = maxk
                    }
                }
            }
        }
        current_number_spins
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
            internal_map: IndexMap::new(),
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
            internal_map: IndexMap::with_capacity(capacity),
            n_spins,
            n_bosons,
            n_fermions,
        }
    }

    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_export")]
    pub fn to_struqture_1(
        &self,
    ) -> Result<struqture_1::mixed_systems::MixedLindbladNoiseSystem, StruqtureError> {
        let mut new_mixed_system = struqture_1::mixed_systems::MixedLindbladNoiseSystem::new(
            vec![None; self.n_spins],
            vec![None; self.n_bosons],
            vec![None; self.n_fermions],
        );
        for (key, val) in self.iter() {
            let one_key_left = key.0.to_struqture_1()?;
            let one_key_right = key.1.to_struqture_1()?;
            let _ = struqture_1::OperateOnDensityMatrix::set(
                &mut new_mixed_system,
                (one_key_left, one_key_right),
                val.clone(),
            );
        }
        Ok(new_mixed_system)
    }

    /// Import from struqture_1 format.
    #[cfg(feature = "struqture_1_import")]
    pub fn from_struqture_1(
        value: &struqture_1::mixed_systems::MixedLindbladNoiseSystem,
    ) -> Result<Self, StruqtureError> {
        let mut new_operator = Self::new(
            struqture_1::mixed_systems::OperateOnMixedSystems::current_number_spins(value).len(),
            struqture_1::mixed_systems::OperateOnMixedSystems::current_number_bosonic_modes(value)
                .len(),
            struqture_1::mixed_systems::OperateOnMixedSystems::current_number_fermionic_modes(
                value,
            )
            .len(),
        );
        for (key, val) in struqture_1::OperateOnDensityMatrix::iter(value) {
            let self_key_left = MixedDecoherenceProduct::from_struqture_1(&key.0)?;
            let self_key_right = MixedDecoherenceProduct::from_struqture_1(&key.1)?;
            let _ = new_operator.set((self_key_left, self_key_right), val.clone());
        }
        Ok(new_operator)
    }
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
        let mut internal = IndexMap::with_capacity(self.len());
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
        let mut internal = IndexMap::with_capacity(self.len());
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
    type IntoIter = indexmap::map::IntoIter<
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
    use crate::STRUQTURE_VERSION;
    use serde_test::{assert_tokens, Configure, Token};
    use std::str::FromStr;

    // Test the Clone and PartialEq traits of MixedLindbladNoiseOperator
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
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "MixedLindbladNoiseOperator".to_string(),
                min_version: (2, 0, 0),
                version: STRUQTURE_VERSION.to_string(),
            },
        };
        let mut so = MixedLindbladNoiseOperator::new(1, 1, 1);
        so.set((pp.clone(), pp), CalculatorComplex::from(0.5))
            .unwrap();

        assert_eq!(
            MixedLindbladNoiseOperator::try_from(sos.clone()).unwrap(),
            so
        );
        assert_eq!(MixedLindbladNoiseOperatorSerialize::from(so), sos);
    }
    // Test the Clone and PartialEq traits of MixedLindbladNoiseOperator
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
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "MixedLindbladNoiseOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
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
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "MixedLindbladNoiseOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
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
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "MixedLindbladNoiseOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };
        assert!(sos_1 == sos);
        assert!(sos == sos_1);
        assert!(sos_2 != sos);
        assert!(sos != sos_2);
    }

    // Test the Debug trait of MixedLindbladNoiseOperator
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
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "MixedLindbladNoiseOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_eq!(
            format!("{:?}", sos),
            "MixedLindbladNoiseOperatorSerialize { items: [(MixedDecoherenceProduct { spins: [DecoherenceProduct { items: [(0, X)] }], bosons: [BosonProduct { creators: [0], annihilators: [3] }], fermions: [FermionProduct { creators: [0], annihilators: [3] }] }, MixedDecoherenceProduct { spins: [DecoherenceProduct { items: [(0, X)] }], bosons: [BosonProduct { creators: [0], annihilators: [3] }], fermions: [FermionProduct { creators: [0], annihilators: [3] }] }, Float(0.5), Float(0.0))], n_spins: 1, n_bosons: 1, n_fermions: 1, serialisation_meta: StruqtureSerialisationMeta { type_name: \"MixedLindbladNoiseOperator\", min_version: (2, 0, 0), version: \"2.0.0\" } }"
        );
    }

    /// Test MixedLindbladNoiseOperator Serialization and Deserialization traits (readable)
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
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "MixedLindbladNoiseOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
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
                Token::Str("serialisation_meta"),
                Token::Struct {
                    name: "StruqtureSerialisationMeta",
                    len: 3,
                },
                Token::Str("type_name"),
                Token::Str("MixedLindbladNoiseOperator"),
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

    /// Test MixedLindbladNoiseOperator Serialization and Deserialization traits (compact)
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
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "MixedLindbladNoiseOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
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
                Token::Str("serialisation_meta"),
                Token::Struct {
                    name: "StruqtureSerialisationMeta",
                    len: 3,
                },
                Token::Str("type_name"),
                Token::Str("MixedLindbladNoiseOperator"),
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
