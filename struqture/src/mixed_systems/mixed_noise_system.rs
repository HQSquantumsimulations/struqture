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

use super::{
    MixedDecoherenceProduct, MixedIndex, MixedLindbladNoiseOperator, OperateOnMixedSystems,
};
use crate::prelude::*;
use crate::{OperateOnDensityMatrix, StruqtureError};
use qoqo_calculator::CalculatorComplex;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Iter, Keys, Values};
use std::iter::{FromIterator, IntoIterator};
use std::{
    fmt::{self, Write},
    ops,
};
use tinyvec::TinyVec;

#[cfg(feature = "json_schema")]
#[derive(schemars::JsonSchema)]
#[serde(remote = "TinyVec<[Option<usize>; 2]>")]
#[serde(transparent)]
pub(crate) struct TinyVecDef(Vec<Option<usize>>);

/// MixedLindbladNoiseSystems are representations of systems of spins, with a MixedLindbladNoiseOperator to represent the hamiltonian of the spin system, and an optional number of spins.
///
/// In the Lindblad equation, Linblad noise operator L_i are not limited to [crate::mixed_systems::MixedDecoherenceProduct] style operators.
/// We use ([crate::mixed_systems::MixedDecoherenceProduct], [crate::mixed_systems::MixedDecoherenceProduct]) as a unique basis.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::spins::DecoherenceProduct;
/// use struqture::bosons::BosonProduct;
/// use struqture::fermions::FermionProduct;
/// use struqture::mixed_systems::{MixedLindbladNoiseSystem, MixedDecoherenceProduct};
///
/// let mut system = MixedLindbladNoiseSystem::new([Some(2_usize)], [Some(2_usize)], [Some(2_usize)]);
///
/// let pp_0x1x_c0a1: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
///     [DecoherenceProduct::new().x(0).x(1)],
///     [BosonProduct::new([], [1]).unwrap()],
///     [FermionProduct::new([1], [1]).unwrap()],
/// )
/// .unwrap();
/// let pp_0z_c0a1: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
///     [DecoherenceProduct::new().z(0)],
///     [BosonProduct::new([0], [1]).unwrap()],
///     [FermionProduct::new([0], [0]).unwrap()],
/// )
/// .unwrap();
/// system.set((pp_0x1x_c0a1.clone(), pp_0x1x_c0a1.clone()), CalculatorComplex::from(0.5)).unwrap();
/// system.set((pp_0z_c0a1.clone(), pp_0z_c0a1.clone()), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(system.get(&(pp_0x1x_c0a1.clone(), pp_0x1x_c0a1.clone())), &CalculatorComplex::from(0.5));
/// assert_eq!(system.get(&(pp_0z_c0a1.clone(), pp_0z_c0a1.clone())), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MixedLindbladNoiseSystem {
    /// The number of spins in each subsystem
    pub(crate) number_spins: TinyVec<[Option<usize>; 2]>,
    /// The number of bosons in each subsystem
    pub(crate) number_bosons: TinyVec<[Option<usize>; 2]>,
    /// The number of fermions in each subsystem
    pub(crate) number_fermions: TinyVec<[Option<usize>; 2]>,
    /// The MixedLindbladNoiseOperator representing the Lindblad noise terms of the MixedLindbladNoiseSystem.
    pub(crate) operator: MixedLindbladNoiseOperator,
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for MixedLindbladNoiseSystem {
    fn schema_name() -> String {
        "MixedLindbladNoiseSystem".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <SchemaHelperMixedLindbladNoiseSystem>::json_schema(gen)
    }
}

#[cfg(feature = "json_schema")]
#[derive(schemars::JsonSchema)]
#[schemars(deny_unknown_fields)]
#[allow(dead_code)]
struct SchemaHelperMixedLindbladNoiseSystem {
    /// The number of spins in each subsystem
    #[serde(with = "TinyVecDef")]
    number_spins: TinyVec<[Option<usize>; 2]>,
    /// The number of bosons in each subsystem
    #[serde(with = "TinyVecDef")]
    number_bosons: TinyVec<[Option<usize>; 2]>,
    /// The number of fermions in each subsystem
    #[serde(with = "TinyVecDef")]
    number_fermions: TinyVec<[Option<usize>; 2]>,
    /// The MixedLindbladNoiseOperator representing the Lindblad noise terms of the MixedLindbladNoiseSystem.
    pub(crate) operator: MixedLindbladNoiseOperator,
}

impl crate::MinSupportedVersion for MixedLindbladNoiseSystem {}

impl<'a> OperateOnDensityMatrix<'a> for MixedLindbladNoiseSystem {
    type Value = CalculatorComplex;
    type Index = (MixedDecoherenceProduct, MixedDecoherenceProduct);
    type IteratorType = Iter<'a, Self::Index, Self::Value>;
    type KeyIteratorType = Keys<'a, Self::Index, Self::Value>;
    type ValueIteratorType = Values<'a, Self::Index, Self::Value>;

    // From trait
    fn get(&self, key: &Self::Index) -> &Self::Value {
        self.operator.get(key)
    }

    // From trait
    fn iter(&'a self) -> Self::IteratorType {
        self.operator.iter()
    }

    // From trait
    fn keys(&'a self) -> Self::KeyIteratorType {
        self.operator.keys()
    }

    // From trait
    fn values(&'a self) -> Self::ValueIteratorType {
        self.operator.values()
    }

    // From trait
    fn remove(&mut self, key: &Self::Index) -> Option<Self::Value> {
        self.operator.remove(key)
    }

    // From trait
    fn empty_clone(&self, capacity: Option<usize>) -> Self {
        match capacity {
            Some(cap) => Self {
                number_spins: self.number_spins.clone(),
                number_bosons: self.number_bosons.clone(),
                number_fermions: self.number_fermions.clone(),
                operator: MixedLindbladNoiseOperator::with_capacity(
                    self.number_spins.len(),
                    self.number_bosons.len(),
                    self.number_fermions.len(),
                    cap,
                ),
            },
            None => Self {
                number_spins: self.number_spins.clone(),
                number_bosons: self.number_bosons.clone(),
                number_fermions: self.number_fermions.clone(),
                operator: MixedLindbladNoiseOperator::new(
                    self.number_spins.len(),
                    self.number_bosons.len(),
                    self.number_fermions.len(),
                ),
            },
        }
    }

    /// Overwrites an existing entry or sets a new entry in the MixedLindbladNoiseSystem with the given ((MixedDecoherenceProduct, MixedDecoherenceProduct) key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The MixedDecoherenceProduct pair key to set in the MixedLindbladNoiseSystem.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the MixedLindbladNoiseSystem.
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
        if key.0.spins().len() != self.number_spins.len()
            || key.0.bosons().len() != self.number_bosons.len()
            || key.0.fermions().len() != self.number_fermions.len()
        {
            return Err(StruqtureError::MissmatchedNumberSubsystems {
                target_number_spin_subsystems: self.number_spins.len(),
                target_number_boson_subsystems: self.number_bosons.len(),
                target_number_fermion_subsystems: self.number_fermions.len(),
                actual_number_spin_subsystems: key.0.spins().len(),
                actual_number_boson_subsystems: key.0.bosons().len(),
                actual_number_fermion_subsystems: key.0.fermions().len(),
            });
        }
        if key.1.spins().len() != self.number_spins.len()
            || key.1.bosons().len() != self.number_bosons.len()
            || key.1.fermions().len() != self.number_fermions.len()
        {
            return Err(StruqtureError::MissmatchedNumberSubsystems {
                target_number_spin_subsystems: self.number_spins.len(),
                target_number_boson_subsystems: self.number_bosons.len(),
                target_number_fermion_subsystems: self.number_fermions.len(),
                actual_number_spin_subsystems: key.1.spins().len(),
                actual_number_boson_subsystems: key.1.bosons().len(),
                actual_number_fermion_subsystems: key.1.fermions().len(),
            });
        }
        for (x, y) in key.0.bosons().zip(self.number_bosons.clone()) {
            if let Some(max_number) = y {
                if x.current_number_modes() > max_number {
                    return Err(StruqtureError::MissmatchedNumberModes);
                }
            }
        }
        for (x, y) in key.0.fermions().zip(self.number_fermions.clone()) {
            if let Some(max_number) = y {
                if x.current_number_modes() > max_number {
                    return Err(StruqtureError::MissmatchedNumberModes);
                }
            }
        }
        for (x, y) in key.0.spins().zip(self.number_spins.clone()) {
            if let Some(max_number) = y {
                if x.current_number_spins() > max_number {
                    return Err(StruqtureError::MissmatchedNumberSpins);
                }
            }
        }
        for (x, y) in key.1.bosons().zip(self.number_bosons.clone()) {
            if let Some(max_number) = y {
                if x.current_number_modes() > max_number {
                    return Err(StruqtureError::MissmatchedNumberModes);
                }
            }
        }
        for (x, y) in key.1.fermions().zip(self.number_fermions.clone()) {
            if let Some(max_number) = y {
                if x.current_number_modes() > max_number {
                    return Err(StruqtureError::MissmatchedNumberModes);
                }
            }
        }
        for (x, y) in key.1.spins().zip(self.number_spins.clone()) {
            if let Some(max_number) = y {
                if x.current_number_spins() > max_number {
                    return Err(StruqtureError::MissmatchedNumberSpins);
                }
            }
        }

        self.operator.set(key, value)
    }

    /// Adds a new ((MixedDecoherenceProduct, MixedDecoherenceProduct) key, CalculatorComplex value) pair to the MixedLindbladNoiseSystem.
    ///
    /// # Arguments
    ///
    /// * `key` - The (MixedDecoherenceProduct, MixedDecoherenceProduct) key to added to the MixedLindbladNoiseSystem.
    /// * `value` - The corresponding CalculatorComplex value to add for the key in the MixedLindbladNoiseSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The (key, value) pair was successfully added.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and key do not match.
    fn add_operator_product(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<(), StruqtureError> {
        if key.0.spins().len() != self.number_spins.len()
            || key.0.bosons().len() != self.number_bosons.len()
            || key.0.fermions().len() != self.number_fermions.len()
        {
            return Err(StruqtureError::MissmatchedNumberSubsystems {
                target_number_spin_subsystems: self.number_spins.len(),
                target_number_boson_subsystems: self.number_bosons.len(),
                target_number_fermion_subsystems: self.number_fermions.len(),
                actual_number_spin_subsystems: key.0.spins().len(),
                actual_number_boson_subsystems: key.0.bosons().len(),
                actual_number_fermion_subsystems: key.0.fermions().len(),
            });
        }
        if key.1.spins().len() != self.number_spins.len()
            || key.1.bosons().len() != self.number_bosons.len()
            || key.1.fermions().len() != self.number_fermions.len()
        {
            return Err(StruqtureError::MissmatchedNumberSubsystems {
                target_number_spin_subsystems: self.number_spins.len(),
                target_number_boson_subsystems: self.number_bosons.len(),
                target_number_fermion_subsystems: self.number_fermions.len(),
                actual_number_spin_subsystems: key.1.spins().len(),
                actual_number_boson_subsystems: key.1.bosons().len(),
                actual_number_fermion_subsystems: key.1.fermions().len(),
            });
        }
        for (x, y) in key.0.bosons().zip(self.number_bosons.clone()) {
            if let Some(max_number) = y {
                if x.current_number_modes() > max_number {
                    return Err(StruqtureError::MissmatchedNumberModes);
                }
            }
        }
        for (x, y) in key.0.fermions().zip(self.number_fermions.clone()) {
            if let Some(max_number) = y {
                if x.current_number_modes() > max_number {
                    return Err(StruqtureError::MissmatchedNumberModes);
                }
            }
        }
        for (x, y) in key.0.spins().zip(self.number_spins.clone()) {
            if let Some(max_number) = y {
                if x.current_number_spins() > max_number {
                    return Err(StruqtureError::MissmatchedNumberSpins);
                }
            }
        }
        for (x, y) in key.1.bosons().zip(self.number_bosons.clone()) {
            if let Some(max_number) = y {
                if x.current_number_modes() > max_number {
                    return Err(StruqtureError::MissmatchedNumberModes);
                }
            }
        }
        for (x, y) in key.1.fermions().zip(self.number_fermions.clone()) {
            if let Some(max_number) = y {
                if x.current_number_modes() > max_number {
                    return Err(StruqtureError::MissmatchedNumberModes);
                }
            }
        }
        for (x, y) in key.1.spins().zip(self.number_spins.clone()) {
            if let Some(max_number) = y {
                if x.current_number_spins() > max_number {
                    return Err(StruqtureError::MissmatchedNumberSpins);
                }
            }
        }

        self.operator.add_operator_product(key, value)
    }
}

impl<'a> OperateOnMixedSystems<'a> for MixedLindbladNoiseSystem {
    // From trait
    fn number_spins(&self) -> Vec<usize> {
        self.number_spins
            .iter()
            .zip(self.current_number_spins())
            .map(|(target, current)| target.unwrap_or_else(|| current))
            .collect()
    }

    // From trait
    fn current_number_spins(&self) -> Vec<usize> {
        let mut number_spins: Vec<usize> = (0..self.number_spins.len()).map(|_| 0).collect();
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

        number_spins
    }

    // From trait
    fn number_bosonic_modes(&self) -> Vec<usize> {
        self.number_bosons
            .iter()
            .zip(self.current_number_bosonic_modes())
            .map(|(target, current)| target.unwrap_or_else(|| current))
            .collect()
    }

    // From trait
    fn current_number_bosonic_modes(&self) -> Vec<usize> {
        let mut number_bosons: Vec<usize> = (0..self.number_bosons.len()).map(|_| 0).collect();
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
        number_bosons
    }

    // From trait
    fn number_fermionic_modes(&self) -> Vec<usize> {
        self.number_fermions
            .iter()
            .zip(self.current_number_fermionic_modes())
            .map(|(target, current)| target.unwrap_or_else(|| current))
            .collect()
    }

    // From trait
    fn current_number_fermionic_modes(&self) -> Vec<usize> {
        let mut number_fermions: Vec<usize> = (0..self.number_fermions.len()).map(|_| 0).collect();
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
        number_fermions
    }
}

/// Functions for the MixedLindbladNoiseSystem.
///
impl MixedLindbladNoiseSystem {
    /// Creates a new MixedLindbladNoiseSystem.
    ///
    /// # Arguments
    ///
    /// * `number_spins` - The number of spins in each spin subsystem.
    /// * `number_bosons` - The number of bosons in each bosonic subsystem.
    /// * `number_fermions` - The number of fermions in each fermionic subsystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The new MixedLindbladNoiseSystem with the input number of spins, bosons and fermions.
    pub fn new(
        number_spins: impl IntoIterator<Item = Option<usize>>,
        number_bosons: impl IntoIterator<Item = Option<usize>>,
        number_fermions: impl IntoIterator<Item = Option<usize>>,
    ) -> Self {
        let number_spins: TinyVec<[Option<usize>; 2]> = number_spins.into_iter().collect();
        let number_bosons: TinyVec<[Option<usize>; 2]> = number_bosons.into_iter().collect();
        let number_fermions: TinyVec<[Option<usize>; 2]> = number_fermions.into_iter().collect();
        let operator = MixedLindbladNoiseOperator::new(
            number_spins.len(),
            number_bosons.len(),
            number_fermions.len(),
        );
        MixedLindbladNoiseSystem {
            number_spins,
            number_bosons,
            number_fermions,
            operator,
        }
    }

    /// Creates a new MixedLindbladNoiseSystem.
    ///
    /// # Arguments
    ///
    /// * `number_spins` - The number of spins in each spin subsystem
    /// * `number_bosons` - The number of boson modes in each bosonic subsystem
    /// * `number_fermions` - The number of fermion modes in each fermionic subsystem
    /// * `capacity` - The pre-allocated capacity of the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new MixedLindbladNoiseSystem with the input number of spins, bosonic modes and fermionic modes.
    pub fn with_capacity(
        number_spins: impl IntoIterator<Item = Option<usize>>,
        number_bosons: impl IntoIterator<Item = Option<usize>>,
        number_fermions: impl IntoIterator<Item = Option<usize>>,
        capacity: usize,
    ) -> Self {
        let number_spins: TinyVec<[Option<usize>; 2]> = number_spins.into_iter().collect();
        let number_bosons: TinyVec<[Option<usize>; 2]> = number_bosons.into_iter().collect();
        let number_fermions: TinyVec<[Option<usize>; 2]> = number_fermions.into_iter().collect();
        let operator = MixedLindbladNoiseOperator::with_capacity(
            number_spins.len(),
            number_bosons.len(),
            number_fermions.len(),
            capacity,
        );

        MixedLindbladNoiseSystem {
            number_spins,
            number_bosons,
            number_fermions,
            operator,
        }
    }

    /// Returns the MixedLindbladNoiseOperator of the MixedLindbladNoiseSystem.
    ///
    /// # Returns
    ///
    /// * `&MixedLindbladNoiseOperator` - The MixedLindbladNoiseOperator of the MixedLindbladNoiseSystem.
    pub fn operator(&self) -> &MixedLindbladNoiseOperator {
        &self.operator
    }

    /// Creates a MixedLindbladNoiseSystem from a MixedLindbladNoiseOperator and an optional number of spins.
    ///
    /// # Arguments
    ///
    /// * `operator` - The MixedLindbladNoiseOperator to create the SpinSytem from.
    /// * `number_spins` - The optional number of spins of each spin subsystem in the MixedLindbladNoiseSystem to be created.
    /// * `number_bosons` - The optional number of boson modes of each bosonic subsystem in the MixedLindbladNoiseSystem to be created.
    /// * `number_fermions` - The optional number of fermion modes for of each fermionic subsystem in the MixedLindbladNoiseSystem to be created.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The MixedLindbladNoiseSystem created from the inputs.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Number of spins in entry exceeds number of spins in system.
    pub fn from_operator(
        operator: MixedLindbladNoiseOperator,
        number_spins: impl IntoIterator<Item = Option<usize>>,
        number_bosons: impl IntoIterator<Item = Option<usize>>,
        number_fermions: impl IntoIterator<Item = Option<usize>>,
    ) -> Result<Self, StruqtureError> {
        let number_spins: TinyVec<[Option<usize>; 2]> = number_spins.into_iter().collect();
        let number_bosons: TinyVec<[Option<usize>; 2]> = number_bosons.into_iter().collect();
        let number_fermions: TinyVec<[Option<usize>; 2]> = number_fermions.into_iter().collect();

        if operator
            .current_number_spins()
            .iter()
            .zip(number_spins.iter())
            .all(|(current, target)| match target {
                Some(x) => current <= x,
                None => true,
            })
            && operator
                .current_number_bosonic_modes()
                .iter()
                .zip(number_bosons.iter())
                .all(|(current, target)| match target {
                    Some(x) => current <= x,
                    None => true,
                })
            && operator
                .current_number_fermionic_modes()
                .iter()
                .zip(number_fermions.iter())
                .all(|(current, target)| match target {
                    Some(x) => current <= x,
                    None => true,
                })
        {
            Ok(MixedLindbladNoiseSystem {
                number_spins,
                number_bosons,
                number_fermions,
                operator,
            })
        } else {
            Err(StruqtureError::NumberSpinsExceeded)
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

/// Implements the negative sign function of MixedLindbladNoiseSystem.
///
impl ops::Neg for MixedLindbladNoiseSystem {
    type Output = Self;
    /// Implement minus sign for MixedLindbladNoiseSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The MixedLindbladNoiseSystem * -1.
    fn neg(mut self) -> Self {
        self.operator = self.operator.neg();
        self
    }
}

/// Implements the plus function of MixedLindbladNoiseSystem by MixedLindbladNoiseSystem.
///
impl<T, V> ops::Add<T> for MixedLindbladNoiseSystem
where
    T: IntoIterator<Item = ((MixedDecoherenceProduct, MixedDecoherenceProduct), V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `+` (add) for two MixedLindbladNoiseSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The MixedLindbladNoiseSystem to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two MixedLindbladNoiseSystems added together.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and key do not match.
    fn add(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value))?;
        }
        Ok(self)
    }
}

/// Implements the minus function of MixedLindbladNoiseSystem by MixedLindbladNoiseSystem.
///
impl<T, V> ops::Sub<T> for MixedLindbladNoiseSystem
where
    T: IntoIterator<Item = ((MixedDecoherenceProduct, MixedDecoherenceProduct), V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `-` (subtract) for two MixedLindbladNoiseSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The MixedLindbladNoiseSystem to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two MixedLindbladNoiseSystems subtracted.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and key do not match.
    fn sub(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value) * -1.0)?;
        }
        Ok(self)
    }
}

/// Implements the multiplication function of MixedLindbladNoiseSystem by CalculatorComplex/CalculatorFloat.
///
impl<T> ops::Mul<T> for MixedLindbladNoiseSystem
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for MixedLindbladNoiseSystem and CalculatorComplex/CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex or CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The MixedLindbladNoiseSystem multiplied by the CalculatorComplex/CalculatorFloat.
    fn mul(mut self, other: T) -> Self {
        let other_cc = Into::<CalculatorComplex>::into(other);
        self.operator = self.operator * other_cc;
        self
    }
}

/// Implements the into_iter function (IntoIterator trait) of MixedLindbladNoiseSystem.
///
impl IntoIterator for MixedLindbladNoiseSystem {
    type Item = (
        (MixedDecoherenceProduct, MixedDecoherenceProduct),
        CalculatorComplex,
    );
    type IntoIter = std::collections::hash_map::IntoIter<
        (MixedDecoherenceProduct, MixedDecoherenceProduct),
        CalculatorComplex,
    >;
    /// Returns the MixedLindbladNoiseSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The MixedLindbladNoiseSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.operator.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference MixedLindbladNoiseSystem.
///
impl<'a> IntoIterator for &'a MixedLindbladNoiseSystem {
    type Item = (
        &'a (MixedDecoherenceProduct, MixedDecoherenceProduct),
        &'a CalculatorComplex,
    );
    type IntoIter = Iter<'a, (MixedDecoherenceProduct, MixedDecoherenceProduct), CalculatorComplex>;

    /// Returns the reference MixedLindbladNoiseSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference MixedLindbladNoiseSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.operator.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of MixedLindbladNoiseSystem.
///
impl
    FromIterator<(
        (MixedDecoherenceProduct, MixedDecoherenceProduct),
        CalculatorComplex,
    )> for MixedLindbladNoiseSystem
{
    /// Returns the object in MixedLindbladNoiseSystem form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the MixedLindbladNoiseSystem.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in MixedLindbladNoiseSystem form.
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
                let number_spins: Vec<Option<usize>> = (0..first_element.0 .0.spins().len())
                    .map(|_| None)
                    .collect();
                let number_bosons: Vec<Option<usize>> = (0..first_element.0 .0.bosons().len())
                    .map(|_| None)
                    .collect();
                let number_fermions: Vec<Option<usize>> = (0..first_element.0 .0.bosons().len())
                    .map(|_| None)
                    .collect();
                let mut slno =
                    MixedLindbladNoiseSystem::new(number_spins, number_bosons, number_fermions);
                slno.set(first_element.0, first_element.1)
                    .expect("Internal error in set");
                for (pair, cc) in iterator {
                    slno.add_operator_product(pair, cc)
                        .expect("Internal error in add_operator_product");
                }
                slno
            }
            None => MixedLindbladNoiseSystem::new([], [], []),
        }
    }
}

/// Implements the extend function (Extend trait) of MixedLindbladNoiseSystem.
///
impl
    Extend<(
        (MixedDecoherenceProduct, MixedDecoherenceProduct),
        CalculatorComplex,
    )> for MixedLindbladNoiseSystem
{
    /// Extends the MixedLindbladNoiseSystem by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the MixedLindbladNoiseSystem.
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
        for (pp, cc) in iter {
            self.add_operator_product(pp, cc)
                .expect("Internal error in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of MixedLindbladNoiseSystem.
///
impl fmt::Display for MixedLindbladNoiseSystem {
    /// Formats the MixedLindbladNoiseSystem using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted MixedLindbladNoiseSystem.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "MixedLindbladNoiseSystem(\n".to_string();
        output.push_str("number_spins: ");
        for n in self.number_spins() {
            write!(output, "{}, ", n)?;
        }
        output.push('\n');
        output.push_str("number_bosons: ");
        for n in self.number_bosonic_modes() {
            write!(output, "{}, ", n)?;
        }
        output.push('\n');
        output.push_str("number_fermions: ");
        for n in self.number_fermionic_modes() {
            write!(output, "{}, ", n)?;
        }
        output.push_str(")\n{");
        for (key, val) in self.iter() {
            writeln!(output, "({}, {}): {},", key.0, key.1, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}
