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
    HermitianMixedProduct, HermitianOperateOnMixedSystems, MixedHamiltonian, MixedSystem,
    OperateOnMixedSystems,
};
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

/// MixedHamiltonianSystems are representations of physical systems of spins, with a MixedHamiltonian to represent the hermitian hamiltonian of the system, and an optional number of spins.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::spins::PauliProduct;
/// use struqture::bosons::BosonProduct;
/// use struqture::fermions::FermionProduct;
/// use struqture::mixed_systems::{MixedHamiltonianSystem, HermitianMixedProduct};
///
/// let mut ms = MixedHamiltonianSystem::new([Some(2_usize)], [Some(2_usize)], [Some(2_usize)]);
///
/// let pp_0x1x_c0a1: HermitianMixedProduct = HermitianMixedProduct::new(
///     [PauliProduct::new().x(0).x(1)],
///     [BosonProduct::new([], [1]).unwrap()],
///     [FermionProduct::new([0], [1]).unwrap()],
/// )
/// .unwrap();
/// let pp_0z_c0a1: HermitianMixedProduct = HermitianMixedProduct::new(
///     [PauliProduct::new().z(0)],
///     [BosonProduct::new([0], [1]).unwrap()],
///     [FermionProduct::new([0], [0]).unwrap()],
/// )
/// .unwrap();
/// ms.set(pp_0x1x_c0a1.clone(), CalculatorComplex::from(0.5))
///     .unwrap();
/// ms.set(pp_0z_c0a1.clone(), CalculatorComplex::from(0.2))
///     .unwrap();
/// ```
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MixedHamiltonianSystem {
    /// The number of spins in each subsystem
    pub(crate) number_spins: TinyVec<[Option<usize>; 2]>,
    /// The number of bosons in each subsystem
    pub(crate) number_bosons: TinyVec<[Option<usize>; 2]>,
    /// The number of fermions in each subsystem
    pub(crate) number_fermions: TinyVec<[Option<usize>; 2]>,
    /// The MixedHamiltonian representing the Hamiltonian of the MixedHamiltonianSystem
    pub(crate) hamiltonian: MixedHamiltonian,
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for MixedHamiltonianSystem {
    fn schema_name() -> String {
        "MixedHamiltonianSystem".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <SchemaHelperMixedHamiltonianSystem>::json_schema(gen)
    }
}

#[cfg(feature = "json_schema")]
#[derive(schemars::JsonSchema)]
#[schemars(deny_unknown_fields)]
#[allow(dead_code)]
struct SchemaHelperMixedHamiltonianSystem {
    /// The number of spins in each subsystem
    #[serde(with = "TinyVecDef")]
    number_spins: TinyVec<[Option<usize>; 2]>,
    /// The number of bosons in each subsystem
    #[serde(with = "TinyVecDef")]
    number_bosons: TinyVec<[Option<usize>; 2]>,
    /// The number of fermions in each subsystem
    #[serde(with = "TinyVecDef")]
    number_fermions: TinyVec<[Option<usize>; 2]>,
    /// The MixedHamiltonian representing the Hamiltonian of the MixedHamiltonianSystem
    pub(crate) hamiltonian: MixedHamiltonian,
}

impl crate::MinSupportedVersion for MixedHamiltonianSystem {}

impl<'a> OperateOnDensityMatrix<'a> for MixedHamiltonianSystem {
    type Index = HermitianMixedProduct;
    type Value = CalculatorComplex;
    type IteratorType = Iter<'a, Self::Index, Self::Value>;
    type KeyIteratorType = Keys<'a, Self::Index, Self::Value>;
    type ValueIteratorType = Values<'a, Self::Index, Self::Value>;

    // From trait
    fn get(&self, key: &Self::Index) -> &Self::Value {
        self.hamiltonian.get(key)
    }

    // From trait
    fn iter(&'a self) -> Self::IteratorType {
        self.hamiltonian.iter()
    }

    // From trait
    fn keys(&'a self) -> Self::KeyIteratorType {
        self.hamiltonian.keys()
    }

    // From trait
    fn values(&'a self) -> Self::ValueIteratorType {
        self.hamiltonian.values()
    }

    // From trait
    fn remove(&mut self, key: &Self::Index) -> Option<Self::Value> {
        self.hamiltonian.remove(key)
    }

    // From trait
    fn empty_clone(&self, capacity: Option<usize>) -> Self {
        match capacity {
            Some(cap) => Self {
                number_spins: self.number_spins.clone(),
                number_bosons: self.number_bosons.clone(),
                number_fermions: self.number_fermions.clone(),
                hamiltonian: MixedHamiltonian::with_capacity(
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
                hamiltonian: MixedHamiltonian::new(
                    self.number_spins.len(),
                    self.number_bosons.len(),
                    self.number_fermions.len(),
                ),
            },
        }
    }

    /// Overwrites an existing entry or sets a new entry in the MixedHamiltonianSystem with the given (HermitianMixedProduct key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The HermitianMixedProduct key to set in the MixedHamiltonianSystem.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the MixedHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(CalculatorComplex))` - The key existed, this is the value it had before it was set with the value input.
    /// * `Ok(None)` - The key did not exist, it has been set with its corresponding value.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and key do not match.
    /// * `Err(StruqtureError::NonHermitianOperator)` - Key is naturally hermitian (on-diagonal term), but its corresponding value is not real.
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
        self.hamiltonian.set(key, value)
    }

    /// Adds a new (HermitianMixedProduct key, CalculatorComplex value) pair to the MixedHamiltonianSystem.
    ///
    /// # Arguments
    ///
    /// * `key` - The HermitianMixedProduct key to added to the MixedHamiltonianSystem.
    /// * `value` - The corresponding CalculatorComplex value to add for the key in the MixedHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The (key, value) pair was successfully added.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and key do not match.
    /// * `Err(StruqtureError::NonHermitianOperator)` - Key is naturally hermitian (on-diagonal term), but its corresponding value is not real.
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
        self.hamiltonian.add_operator_product(key, value)
    }
}

impl<'a> OperateOnState<'a> for MixedHamiltonianSystem {
    // From trait
    fn hermitian_conjugate(&self) -> Self {
        self.clone()
    }
}

impl<'a> OperateOnMixedSystems<'a> for MixedHamiltonianSystem {
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

impl<'a> HermitianOperateOnMixedSystems<'a> for MixedHamiltonianSystem {}

/// Functions for the MixedHamiltonianSystem
///
impl MixedHamiltonianSystem {
    /// Creates a new MixedHamiltonianSystem.
    ///
    /// # Arguments
    ///
    /// * `number_spins` - The number of spins in each spin subsystem
    /// * `number_bosons` - The number of boson modes in each bosonic subsystem
    /// * `number_fermions` - The number of fermion modes in each fermionic subsystem
    ///
    /// # Returns
    ///
    /// * `Self` - The new MixedHamiltonianSystem with the input number of spins and modes.
    pub fn new(
        number_spins: impl IntoIterator<Item = Option<usize>>,
        number_bosons: impl IntoIterator<Item = Option<usize>>,
        number_fermions: impl IntoIterator<Item = Option<usize>>,
    ) -> Self {
        let number_spins: TinyVec<[Option<usize>; 2]> = number_spins.into_iter().collect();
        let number_bosons: TinyVec<[Option<usize>; 2]> = number_bosons.into_iter().collect();
        let number_fermions: TinyVec<[Option<usize>; 2]> = number_fermions.into_iter().collect();
        let hamiltonian = MixedHamiltonian::new(
            number_spins.len(),
            number_bosons.len(),
            number_fermions.len(),
        );
        MixedHamiltonianSystem {
            number_spins,
            number_bosons,
            number_fermions,
            hamiltonian,
        }
    }

    /// Creates a new MixedHamiltonianSystem with pre-allocated capacity.
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
    /// * `Self` - The new MixedHamiltonianSystem with the input number of spins and modes.
    pub fn with_capacity(
        number_spins: impl IntoIterator<Item = Option<usize>>,
        number_bosons: impl IntoIterator<Item = Option<usize>>,
        number_fermions: impl IntoIterator<Item = Option<usize>>,
        capacity: usize,
    ) -> Self {
        let number_spins: TinyVec<[Option<usize>; 2]> = number_spins.into_iter().collect();
        let number_bosons: TinyVec<[Option<usize>; 2]> = number_bosons.into_iter().collect();
        let number_fermions: TinyVec<[Option<usize>; 2]> = number_fermions.into_iter().collect();
        let hamiltonian = MixedHamiltonian::with_capacity(
            number_spins.len(),
            number_bosons.len(),
            number_fermions.len(),
            capacity,
        );
        MixedHamiltonianSystem {
            number_spins,
            number_bosons,
            number_fermions,
            hamiltonian,
        }
    }

    /// Returns the MixedHamiltonian of the MixedHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `&MixedHamiltonian` - The MixedHamiltonian of the MixedHamiltonianSystem.
    pub fn hamiltonian(&self) -> &MixedHamiltonian {
        &self.hamiltonian
    }

    /// Creates a MixedHamiltonianSystem from a MixedHamiltonian and an optional number of spins/modes.
    ///
    /// # Arguments
    ///
    /// * `hamiltonian` - The MixedHamiltonian to create the MixedHamiltonianSystem from.
    /// * `number_spins` - The number of spins for the MixedHamiltonianSystem to be created.
    /// * `number_bosons` - The number of boson modes for the MixedHamiltonianSystem to be created.
    /// * `number_fermions` - The number of fermion modes for the MixedHamiltonianSystem to be created.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The MixedHamiltonianSystem created from the inputs.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Number of spins in entry exceeds number of spins in system.
    pub fn from_hamiltonian(
        hamiltonian: MixedHamiltonian,
        number_spins: impl IntoIterator<Item = Option<usize>>,
        number_bosons: impl IntoIterator<Item = Option<usize>>,
        number_fermions: impl IntoIterator<Item = Option<usize>>,
    ) -> Result<Self, StruqtureError> {
        let number_spins: TinyVec<[Option<usize>; 2]> = number_spins.into_iter().collect();
        let number_bosons: TinyVec<[Option<usize>; 2]> = number_bosons.into_iter().collect();
        let number_fermions: TinyVec<[Option<usize>; 2]> = number_fermions.into_iter().collect();

        if hamiltonian
            .current_number_spins()
            .iter()
            .zip(number_spins.iter())
            .all(|(current, target)| match target {
                Some(x) => current <= x,
                None => true,
            })
            && hamiltonian
                .current_number_bosonic_modes()
                .iter()
                .zip(number_bosons.iter())
                .all(|(current, target)| match target {
                    Some(x) => current <= x,
                    None => true,
                })
            && hamiltonian
                .current_number_fermionic_modes()
                .iter()
                .zip(number_fermions.iter())
                .all(|(current, target)| match target {
                    Some(x) => current <= x,
                    None => true,
                })
        {
            Ok(MixedHamiltonianSystem {
                number_spins,
                number_bosons,
                number_fermions,
                hamiltonian,
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

/// Implements the negative sign function of MixedHamiltonianSystem.
///
impl ops::Neg for MixedHamiltonianSystem {
    type Output = Self;
    /// Implement minus sign for MixedHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The MixedHamiltonianSystem * -1.
    fn neg(mut self) -> Self {
        self.hamiltonian = self.hamiltonian.neg();
        self
    }
}

/// Implements the plus function of MixedHamiltonianSystem by MixedHamiltonianSystem.
///
impl<T, V> ops::Add<T> for MixedHamiltonianSystem
where
    T: IntoIterator<Item = (HermitianMixedProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `+` (add) for two MixedHamiltonianSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The MixedHamiltonianSystem to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two MixedHamiltonianSystems added together.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and key do not match.
    /// * `Err(StruqtureError::NonHermitianOperator)` - Key is naturally hermitian (on-diagonal term), but its corresponding value is not real.
    fn add(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value))?;
        }
        Ok(self)
    }
}

/// Implements the minus function of MixedHamiltonianSystem by MixedHamiltonianSystem.
///
impl<T, V> ops::Sub<T> for MixedHamiltonianSystem
where
    T: IntoIterator<Item = (HermitianMixedProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `-` (subtract) for two MixedHamiltonianSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The MixedHamiltonianSystem to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two MixedHamiltonianSystems subtracted.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and key do not match.
    /// * `Err(StruqtureError::NonHermitianOperator)` - Key is naturally hermitian (on-diagonal term), but its corresponding value is not real.
    fn sub(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value) * -1.0)?;
        }
        Ok(self)
    }
}

/// Implements the multiplication function of MixedHamiltonianSystem by CalculatorComplex/CalculatorFloat.
///
impl<T> ops::Mul<T> for MixedHamiltonianSystem
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for MixedHamiltonianSystem and CalculatorComplex/CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex or CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The MixedHamiltonianSystem multiplied by the CalculatorComplex/CalculatorFloat.
    fn mul(mut self, other: T) -> Self {
        let other_cc = Into::<CalculatorComplex>::into(other);
        self.hamiltonian = self.hamiltonian * other_cc;
        self
    }
}

/// Implements the multiplication function of MixedHamiltonianSystem by MixedHamiltonianSystem.
///
impl ops::Mul<MixedHamiltonianSystem> for MixedHamiltonianSystem {
    type Output = Result<MixedSystem, StruqtureError>;
    /// Implement `*` for MixedHamiltonianSystem and MixedHamiltonianSystem.
    ///
    /// # Arguments
    ///
    /// * `other` - The MixedHamiltonianSystem to multiply by.
    ///
    /// # Returns
    ///
    /// * `Ok(MixedSystem)` - The two MixedHamiltonianSystems multiplied.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and key do not match.
    fn mul(self, other: MixedHamiltonianSystem) -> Self::Output {
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

/// Implements the into_iter function (IntoIterator trait) of MixedHamiltonianSystem.
///
impl IntoIterator for MixedHamiltonianSystem {
    type Item = (HermitianMixedProduct, CalculatorComplex);
    type IntoIter = std::collections::hash_map::IntoIter<HermitianMixedProduct, CalculatorComplex>;
    /// Returns the MixedHamiltonianSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The MixedHamiltonianSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.hamiltonian.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference MixedHamiltonianSystem.
///
impl<'a> IntoIterator for &'a MixedHamiltonianSystem {
    type Item = (&'a HermitianMixedProduct, &'a CalculatorComplex);
    type IntoIter = Iter<'a, HermitianMixedProduct, CalculatorComplex>;

    /// Returns the reference MixedHamiltonianSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference MixedHamiltonianSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.hamiltonian.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of MixedHamiltonianSystem.
///
impl FromIterator<(HermitianMixedProduct, CalculatorComplex)> for MixedHamiltonianSystem {
    /// Returns the object in MixedHamiltonianSystem form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the MixedHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in MixedHamiltonianSystem form.
    ///
    /// # Panics
    ///
    /// * Internal error in set.
    /// * Internal error in add_operator_product.
    fn from_iter<I: IntoIterator<Item = (HermitianMixedProduct, CalculatorComplex)>>(
        iter: I,
    ) -> Self {
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
                let mut slno =
                    MixedHamiltonianSystem::new(number_spins, number_bosons, number_fermions);
                slno.set(first_element.0, first_element.1)
                    .expect("Internal error in set");
                for (pair, cc) in iterator {
                    slno.add_operator_product(pair, cc)
                        .expect("Internal error in add_operator_product");
                }
                slno
            }
            None => MixedHamiltonianSystem::new([], [], []),
        }
    }
}

/// Implements the extend function (Extend trait) of MixedHamiltonianSystem.
///
impl Extend<(HermitianMixedProduct, CalculatorComplex)> for MixedHamiltonianSystem {
    /// Extends the MixedHamiltonianSystem by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the MixedHamiltonianSystem.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn extend<I: IntoIterator<Item = (HermitianMixedProduct, CalculatorComplex)>>(
        &mut self,
        iter: I,
    ) {
        for (pp, cc) in iter {
            self.add_operator_product(pp, cc)
                .expect("Internal error in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of MixedHamiltonianSystem.
///
impl fmt::Display for MixedHamiltonianSystem {
    /// Formats the MixedHamiltonianSystem using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted MixedHamiltonianSystem.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "MixedHamiltonianSystem(\n".to_string();
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
        let mut vec: Vec<(&HermitianMixedProduct, &CalculatorComplex)> = self.iter().collect();
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
