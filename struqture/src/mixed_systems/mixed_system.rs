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

use super::{HermitianOperateOnMixedSystems, MixedOperator, MixedProduct, OperateOnMixedSystems};
#[cfg(feature = "json_schema")]
use crate::mixed_systems::TinyVecDef;
use crate::prelude::*;
use crate::{OperateOnDensityMatrix, OperateOnState, StruqtureError};
use qoqo_calculator::CalculatorComplex;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Iter, Keys, Values};
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;
use tinyvec::TinyVec;

/// MixedSystems are representations of physical systems of spins, with a MixedOperator to represent the hermitian hamiltonian of the system, and an optional number of spins.
/// MixedSystems are MixedOperators with a certain number of spins, a certain number of bosonic modes and a certain number of fermionic modes. When constructing it, the `new`
/// function takes a `number_spins` input, a `number_bosons` input and a `number_fermions` input, and therefore when the user adds a set of MixedProducts with specific CalculatorComplex coefficients,
/// their indices must not exceed the number of modes in the MixedSystem.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::spins::PauliProduct;
/// use struqture::bosons::BosonProduct;
/// use struqture::fermions::FermionProduct;
/// use struqture::mixed_systems::{MixedSystem, MixedProduct};
///
/// let mut ms = MixedSystem::new([Some(2_usize)], [Some(2_usize)], [Some(2_usize)]);
///
/// let pp_0x1x_a1_c0a1: MixedProduct = MixedProduct::new(
///     [PauliProduct::new().x(0).x(1)],
///     [BosonProduct::new([], [1]).unwrap()],
///     [FermionProduct::new([0], [1]).unwrap()],
/// )
/// .unwrap();
/// let pp_0z_c0a1_c0a0: MixedProduct = MixedProduct::new(
///     [PauliProduct::new().z(0)],
///     [BosonProduct::new([0], [1]).unwrap()],
///     [FermionProduct::new([0], [0]).unwrap()],
/// )
/// .unwrap();
/// ms.set(pp_0x1x_a1_c0a1.clone(), CalculatorComplex::from(0.5))
///     .unwrap();
/// ms.set(pp_0z_c0a1_c0a0.clone(), CalculatorComplex::from(0.2))
///     .unwrap();
///
/// // Access what you set:
/// assert_eq!(ms.get(&pp_0x1x_a1_c0a1), &CalculatorComplex::from(0.5));
/// assert_eq!(ms.get(&pp_0z_c0a1_c0a0), &CalculatorComplex::from(0.2));
///
/// ```
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MixedSystem {
    /// The number of spins in each subsystem
    pub(crate) number_spins: TinyVec<[Option<usize>; 2]>,
    /// The number of bosons in each subsystem
    pub(crate) number_bosons: TinyVec<[Option<usize>; 2]>,
    /// The number of fermions in each subsystem
    pub(crate) number_fermions: TinyVec<[Option<usize>; 2]>,
    /// The MixedOperator representing the Hamiltonian of the MixedSystem
    pub(crate) operator: MixedOperator,
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for MixedSystem {
    fn schema_name() -> String {
        "MixedSystem".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <SchemaHelperMixedSystem>::json_schema(gen)
    }
}

#[cfg(feature = "json_schema")]
#[derive(schemars::JsonSchema)]
#[schemars(deny_unknown_fields)]
#[allow(dead_code)]
struct SchemaHelperMixedSystem {
    /// The number of spins in each subsystem
    #[serde(with = "TinyVecDef")]
    number_spins: TinyVec<[Option<usize>; 2]>,
    /// The number of bosons in each subsystem
    #[serde(with = "TinyVecDef")]
    number_bosons: TinyVec<[Option<usize>; 2]>,
    /// The number of fermions in each subsystem
    #[serde(with = "TinyVecDef")]
    number_fermions: TinyVec<[Option<usize>; 2]>,
    /// The MixedOperator representing the Hamiltonian of the MixedSystem
    pub(crate) operator: MixedOperator,
}

impl crate::MinSupportedVersion for MixedSystem {}

impl<'a> OperateOnDensityMatrix<'a> for MixedSystem {
    type Index = MixedProduct;
    type Value = CalculatorComplex;
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
                operator: MixedOperator::with_capacity(
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
                operator: MixedOperator::new(
                    self.number_spins.len(),
                    self.number_bosons.len(),
                    self.number_fermions.len(),
                ),
            },
        }
    }

    /// Overwrites an existing entry or sets a new entry in the MixedSystem with the given (MixedProduct key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The MixedProduct key to set in the MixedSystem.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the MixedSystem.
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
        if key.spins().len() != self.number_spins.len()
            || key.bosons().len() != self.number_bosons.len()
            || key.fermions().len() != self.number_fermions.len()
        {
            return Err(StruqtureError::MissmatchedNumberSubsystems {
                target_number_spin_subsystems: self.number_spins.len(),
                target_number_boson_subsystems: self.number_bosons.len(),
                target_number_fermion_subsystems: self.number_fermions.len(),
                actual_number_spin_subsystems: key.spins().len(),
                actual_number_boson_subsystems: key.bosons().len(),
                actual_number_fermion_subsystems: key.fermions().len(),
            });
        }
        for (x, y) in key.bosons().zip(self.number_bosons.clone()) {
            if let Some(max_number) = y {
                if x.current_number_modes() > max_number {
                    return Err(StruqtureError::MissmatchedNumberModes);
                }
            }
        }
        for (x, y) in key.fermions().zip(self.number_fermions.clone()) {
            if let Some(max_number) = y {
                if x.current_number_modes() > max_number {
                    return Err(StruqtureError::MissmatchedNumberModes);
                }
            }
        }
        for (x, y) in key.spins().zip(self.number_spins.clone()) {
            if let Some(max_number) = y {
                if x.current_number_spins() > max_number {
                    return Err(StruqtureError::MissmatchedNumberSpins);
                }
            }
        }
        self.operator.set(key, value)
    }

    /// Adds a new (MixedProduct key, CalculatorComplex value) pair to the MixedSystem.
    ///
    /// # Arguments
    ///
    /// * `key` - The MixedProduct key to added to the MixedSystem.
    /// * `value` - The corresponding CalculatorComplex value to add for the key in the MixedSystem.
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
        if key.spins().len() != self.number_spins.len()
            || key.bosons().len() != self.number_bosons.len()
            || key.fermions().len() != self.number_fermions.len()
        {
            return Err(StruqtureError::MissmatchedNumberSubsystems {
                target_number_spin_subsystems: self.number_spins.len(),
                target_number_boson_subsystems: self.number_bosons.len(),
                target_number_fermion_subsystems: self.number_fermions.len(),
                actual_number_spin_subsystems: key.spins().len(),
                actual_number_boson_subsystems: key.bosons().len(),
                actual_number_fermion_subsystems: key.fermions().len(),
            });
        }
        for (x, y) in key.bosons().zip(self.number_bosons.clone()) {
            if let Some(max_number) = y {
                if x.current_number_modes() > max_number {
                    return Err(StruqtureError::MissmatchedNumberModes);
                }
            }
        }
        for (x, y) in key.fermions().zip(self.number_fermions.clone()) {
            if let Some(max_number) = y {
                if x.current_number_modes() > max_number {
                    return Err(StruqtureError::MissmatchedNumberModes);
                }
            }
        }
        for (x, y) in key.spins().zip(self.number_spins.clone()) {
            if let Some(max_number) = y {
                if x.current_number_spins() > max_number {
                    return Err(StruqtureError::MissmatchedNumberSpins);
                }
            }
        }
        self.operator.add_operator_product(key, value)
    }
}

impl<'a> OperateOnState<'a> for MixedSystem {
    // From trait
    fn hermitian_conjugate(&self) -> Self {
        self.clone()
    }
}

impl<'a> OperateOnMixedSystems<'a> for MixedSystem {
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
        self.number_bosons
            .iter()
            .zip(self.current_number_bosonic_modes())
            .map(|(target, current)| target.unwrap_or_else(|| current))
            .collect()
    }

    // From trait
    fn current_number_bosonic_modes(&self) -> Vec<usize> {
        let mut number_bosons: Vec<usize> = (0..self.number_bosons.len()).map(|_| 0).collect();
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
        self.number_fermions
            .iter()
            .zip(self.current_number_fermionic_modes())
            .map(|(target, current)| target.unwrap_or_else(|| current))
            .collect()
    }

    // From trait
    fn current_number_fermionic_modes(&self) -> Vec<usize> {
        let mut number_fermions: Vec<usize> = (0..self.number_fermions.len()).map(|_| 0).collect();
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

impl<'a> HermitianOperateOnMixedSystems<'a> for MixedSystem {}

/// Functions for the MixedSystem
///
impl MixedSystem {
    /// Creates a new MixedSystem.
    ///
    /// # Arguments
    ///
    /// * `number_spins` - The number of spins in each spin subsystem.
    /// * `number_bosons` - The number of boson modes in each bosonic subsystem.
    /// * `number_fermions` - The number of fermion modes in each fermionic subsystem.
    ///
    ///
    /// # Returns
    ///
    /// * `Self` - The new MixedSystem with the input number of spins and modes.
    pub fn new(
        number_spins: impl IntoIterator<Item = Option<usize>>,
        number_bosons: impl IntoIterator<Item = Option<usize>>,
        number_fermions: impl IntoIterator<Item = Option<usize>>,
    ) -> Self {
        let number_spins: TinyVec<[Option<usize>; 2]> = number_spins.into_iter().collect();
        let number_bosons: TinyVec<[Option<usize>; 2]> = number_bosons.into_iter().collect();
        let number_fermions: TinyVec<[Option<usize>; 2]> = number_fermions.into_iter().collect();
        let operator = MixedOperator::new(
            number_spins.len(),
            number_bosons.len(),
            number_fermions.len(),
        );
        MixedSystem {
            number_spins,
            number_bosons,
            number_fermions,
            operator,
        }
    }

    /// Creates a new MixedSystem with capacity pre-allocated capacity and given inputs.
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
    /// * `Self` - The new MixedSystem with the input number of spins and modes.
    pub fn with_capacity(
        number_spins: impl IntoIterator<Item = Option<usize>>,
        number_bosons: impl IntoIterator<Item = Option<usize>>,
        number_fermions: impl IntoIterator<Item = Option<usize>>,
        capacity: usize,
    ) -> Self {
        let number_spins: TinyVec<[Option<usize>; 2]> = number_spins.into_iter().collect();
        let number_bosons: TinyVec<[Option<usize>; 2]> = number_bosons.into_iter().collect();
        let number_fermions: TinyVec<[Option<usize>; 2]> = number_fermions.into_iter().collect();
        let operator = MixedOperator::with_capacity(
            number_spins.len(),
            number_bosons.len(),
            number_fermions.len(),
            capacity,
        );
        MixedSystem {
            number_spins,
            number_bosons,
            number_fermions,
            operator,
        }
    }

    /// Returns the MixedOperator of the MixedSystem.
    ///
    /// # Returns
    ///
    /// * `&MixedOperator` - The MixedOperator of the MixedSystem.
    pub fn operator(&self) -> &MixedOperator {
        &self.operator
    }

    /// Creates a MixedSystem from a MixedOperator and an optional number of spins/modes.
    ///
    /// # Arguments
    ///
    /// * `operator` - The MixedOperator to create the MixedSystem from.
    /// * `number_spins` - The number of spins for each spin subsystem of the MixedSystem to be created.
    /// * `number_bosons` - The number of boson modes for each bosonic subsystem of the MixedSystem to be created.
    /// * `number_fermions` - The number of fermion modes for each fermionic subsystem of the MixedSystem to be created.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The MixedSystem created from the inputs.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Number of spins in entry exceeds number of spins in system.
    pub fn from_operator(
        operator: MixedOperator,
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
            Ok(MixedSystem {
                number_spins,
                number_bosons,
                number_fermions,
                operator,
            })
        } else {
            Err(StruqtureError::NumberSpinsExceeded)
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

/// Implements the negative sign function of MixedSystem.
///
impl ops::Neg for MixedSystem {
    type Output = Self;
    /// Implement minus sign for MixedSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The MixedSystem * -1.
    fn neg(mut self) -> Self {
        self.operator = self.operator.neg();
        self
    }
}

/// Implements the plus function of MixedSystem by MixedSystem.
///
impl<T, V> ops::Add<T> for MixedSystem
where
    T: IntoIterator<Item = (MixedProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `+` (add) for two MixedSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The MixedSystem to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two MixedSystems added together.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and key do not match.
    fn add(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value))?;
        }
        Ok(self)
    }
}

/// Implements the minus function of MixedSystem by MixedSystem.
///
impl<T, V> ops::Sub<T> for MixedSystem
where
    T: IntoIterator<Item = (MixedProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `-` (subtract) for two MixedSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The MixedSystem to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two MixedSystems subtracted.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and key do not match.
    fn sub(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value) * -1.0)?;
        }
        Ok(self)
    }
}

/// Implements the multiplication function of MixedSystem by CalculatorComplex/CalculatorFloat.
///
impl<T> ops::Mul<T> for MixedSystem
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for MixedSystem and CalculatorComplex/CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex or CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The MixedSystem multiplied by the CalculatorComplex/CalculatorFloat.
    fn mul(mut self, other: T) -> Self {
        let other_cc = Into::<CalculatorComplex>::into(other);
        self.operator = self.operator * other_cc;
        self
    }
}

/// Implements the multiplication function of MixedSystem by MixedSystem.
///
impl ops::Mul<MixedSystem> for MixedSystem {
    type Output = Result<MixedSystem, StruqtureError>;
    /// Implement `*` for MixedSystem and MixedSystem.
    ///
    /// # Arguments
    ///
    /// * `other` - The MixedSystem to multiply by.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two MixedSystems multiplied.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and key do not match.
    fn mul(self, other: MixedSystem) -> Self::Output {
        if self.number_spins.len() != other.number_spins.len()
            || self.number_bosons.len() != other.number_bosons.len()
            || self.number_fermions.len() != other.number_fermions.len()
        {
            return Err(StruqtureError::MissmatchedNumberSubsystems {
                target_number_spin_subsystems: self.number_spins.len(),
                target_number_boson_subsystems: self.number_bosons.len(),
                target_number_fermion_subsystems: self.number_fermions.len(),
                actual_number_spin_subsystems: other.number_spins.len(),
                actual_number_boson_subsystems: other.number_bosons.len(),
                actual_number_fermion_subsystems: other.number_fermions.len(),
            });
        }
        let capacity = self.len() * other.len();
        let mut spin_op = MixedSystem::with_capacity(
            self.number_spins.clone(),
            self.number_bosons.clone(),
            self.number_fermions.clone(),
            capacity,
        );
        for (pps, vals) in self {
            for (ppo, valo) in other.iter() {
                let products = (pps.clone() * ppo.clone())?;
                for (ppp, coefficient) in products {
                    let coefficient =
                        Into::<CalculatorComplex>::into(valo) * vals.clone() * coefficient;
                    spin_op.add_operator_product(ppp, coefficient)?;
                }
            }
        }
        Ok(spin_op)
    }
}

/// Implements the into_iter function (IntoIterator trait) of MixedSystem.
///
impl IntoIterator for MixedSystem {
    type Item = (MixedProduct, CalculatorComplex);
    type IntoIter = std::collections::hash_map::IntoIter<MixedProduct, CalculatorComplex>;
    /// Returns the MixedSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The MixedSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.operator.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference MixedSystem.
///
impl<'a> IntoIterator for &'a MixedSystem {
    type Item = (&'a MixedProduct, &'a CalculatorComplex);
    type IntoIter = Iter<'a, MixedProduct, CalculatorComplex>;

    /// Returns the reference MixedSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference MixedSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.operator.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of MixedSystem.
///
impl FromIterator<(MixedProduct, CalculatorComplex)> for MixedSystem {
    /// Returns the object in MixedSystem form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the MixedSystem.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in MixedSystem form.
    ///
    /// # Panics
    ///
    /// * Internal error in set.
    /// * Internal error in add_operator_product.
    fn from_iter<I: IntoIterator<Item = (MixedProduct, CalculatorComplex)>>(iter: I) -> Self {
        let mut iterator = iter.into_iter();
        match iterator.next() {
            Some(first_element) => {
                let number_spins: Vec<Option<usize>> =
                    (0..first_element.0.spins().len()).map(|_| None).collect();
                let number_bosons: Vec<Option<usize>> =
                    (0..first_element.0.bosons().len()).map(|_| None).collect();
                let number_fermions: Vec<Option<usize>> = (0..first_element.0.fermions().len())
                    .map(|_| None)
                    .collect();
                let mut slno = MixedSystem::new(number_spins, number_bosons, number_fermions);
                slno.set(first_element.0, first_element.1)
                    .expect("Internal error in set");
                for (pair, cc) in iterator {
                    slno.add_operator_product(pair, cc)
                        .expect("Internal error in add_operator_product");
                }
                slno
            }
            None => MixedSystem::new([], [], []),
        }
    }
}

/// Implements the extend function (Extend trait) of MixedSystem.
///
impl Extend<(MixedProduct, CalculatorComplex)> for MixedSystem {
    /// Extends the MixedSystem by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the MixedSystem.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn extend<I: IntoIterator<Item = (MixedProduct, CalculatorComplex)>>(&mut self, iter: I) {
        for (pp, cc) in iter {
            self.add_operator_product(pp, cc)
                .expect("Internal error in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of MixedSystem.
///
impl fmt::Display for MixedSystem {
    /// Formats the MixedSystem using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted MixedSystem.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "MixedSystem(\n".to_string();
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
        output.push_str(")\n");
        output.push('{');
        let mut vec: Vec<(&MixedProduct, &CalculatorComplex)> = self.iter().collect();
        vec.sort_unstable_by(|(left_index, _), (right_index, _)| {
            left_index
                .partial_cmp(right_index)
                .expect("Cannot compare two unsigned integers internal error in struqture.spins")
        });
        for (key, val) in vec {
            writeln!(output, "{}: {},", key, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}
