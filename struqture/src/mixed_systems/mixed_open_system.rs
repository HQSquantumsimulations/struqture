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

use super::{MixedHamiltonianSystem, MixedLindbladNoiseSystem, OperateOnMixedSystems};
use crate::{OpenSystem, OperateOnDensityMatrix, StruqtureError};
use qoqo_calculator::CalculatorFloat;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops;
use tinyvec::TinyVec;

/// MixedLindbladOpenSystems are representations of open systems of spins, where a system (MixedHamiltonianSystem) interacts with the environment via noise (MixedLindbladNoiseSystem).
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::spins::{DecoherenceProduct, PauliProduct};
/// use struqture::bosons::BosonProduct;
/// use struqture::fermions::FermionProduct;
/// use struqture::mixed_systems::{MixedLindbladOpenSystem, MixedDecoherenceProduct, HermitianMixedProduct};
///
/// let mut system = MixedLindbladOpenSystem::new([Some(2_usize)], [Some(2_usize)], [Some(2_usize)]);
///
/// let pp_0x1x_a1_c0a1: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
///     [DecoherenceProduct::new().x(0).x(1)],
///     [BosonProduct::new([], [1]).unwrap()],
///     [FermionProduct::new([0], [1]).unwrap()],
/// )
/// .unwrap();
/// let pp_0z_c0a1_c0a0: HermitianMixedProduct = HermitianMixedProduct::new(
///     [PauliProduct::new().z(0)],
///     [BosonProduct::new([0], [1]).unwrap()],
///     [FermionProduct::new([0], [0]).unwrap()],
/// )
/// .unwrap();
/// system.noise_mut().set((pp_0x1x_a1_c0a1.clone(), pp_0x1x_a1_c0a1.clone()), CalculatorComplex::from(0.5)).unwrap();
/// system.system_mut().set(pp_0z_c0a1_c0a0.clone(), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(system.noise().get(&(pp_0x1x_a1_c0a1.clone(), pp_0x1x_a1_c0a1.clone())), &CalculatorComplex::from(0.5));
/// assert_eq!(system.system().get(&pp_0z_c0a1_c0a0), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
pub struct MixedLindbladOpenSystem {
    /// The MixedHamiltonianSystem representing the system terms of the open system
    system: MixedHamiltonianSystem,
    /// The MixedLindbladNoiseSystem representing the noise terms of the open system
    noise: MixedLindbladNoiseSystem,
}

impl crate::MinSupportedVersion for MixedLindbladOpenSystem {}

impl<'a> OpenSystem<'a> for MixedLindbladOpenSystem {
    type System = MixedHamiltonianSystem;
    type Noise = MixedLindbladNoiseSystem;

    // From trait
    fn noise(&self) -> &Self::Noise {
        &self.noise
    }

    // From trait
    fn system(&self) -> &Self::System {
        &self.system
    }

    // From trait
    fn noise_mut(&mut self) -> &mut Self::Noise {
        &mut self.noise
    }

    // From trait
    fn system_mut(&mut self) -> &mut Self::System {
        &mut self.system
    }

    // From trait
    fn ungroup(self) -> (Self::System, Self::Noise) {
        (self.system, self.noise)
    }

    /// Takes a tuple of a system (MixedHamiltonianSystem) and a noise term (MixedLindbladNoiseSystem) and combines them to be a MixedLindbladOpenSystem.
    ///
    /// # Arguments
    ///
    /// * `system` - The MixedHamiltonianSystem to have in the MixedLindbladOpenSystem.
    /// * `noise` - The MixedLindbladNoiseSystem to have in the MixedLindbladOpenSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The MixedLindbladOpenSystem with input system and noise terms.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and noise do not match.
    fn group(system: Self::System, noise: Self::Noise) -> Result<Self, StruqtureError> {
        if system.number_spins.len() != noise.number_spins.len()
            || system.number_bosons.len() != noise.number_bosons.len()
            || system.number_fermions.len() != noise.number_fermions.len()
        {
            return Err(StruqtureError::MissmatchedNumberSubsystems {
                target_number_spin_subsystems: system.number_spins.len(),
                target_number_boson_subsystems: system.number_bosons.len(),
                target_number_fermion_subsystems: system.number_fermions.len(),
                actual_number_spin_subsystems: noise.number_spins.len(),
                actual_number_boson_subsystems: noise.number_bosons.len(),
                actual_number_fermion_subsystems: noise.number_fermions.len(),
            });
        }

        let mut variable_number_spins = system.number_spins.clone();
        let noise_number_spins = noise.number_spins.clone();
        let noise_number_current_spins = noise.number_spins();
        let system_number_current_spins = system.number_spins();
        for (index, (system_spins, noise_spins)) in variable_number_spins
            .iter_mut()
            .zip(noise_number_spins.iter())
            .enumerate()
        {
            if system_spins != noise_spins {
                match (*system_spins, noise_spins) {
                    (Some(n), None) => {
                        if n < noise_number_current_spins[index] {
                            return Err(StruqtureError::MissmatchedNumberSpins);
                        }
                    }
                    (None, Some(n)) => {
                        if *n >= system_number_current_spins[index] {
                            *system_spins = Some(*n);
                        } else {
                            return Err(StruqtureError::MissmatchedNumberSpins);
                        }
                    }
                    (Some(_), Some(_)) => {
                        return Err(StruqtureError::MissmatchedNumberSpins);
                    }
                    _ => panic!("Unexpected missmatch of number modes"),
                }
            }
        }
        let mut system = system;
        let mut noise = noise;
        system.number_spins = variable_number_spins.clone();
        noise.number_spins = variable_number_spins;

        // Checking boson compatibility

        let mut variable_number_bosons = system.number_bosons.clone();
        let noise_number_bosons = noise.number_bosons.clone();
        let noise_number_current_bosons = noise.number_bosonic_modes();
        let system_number_current_bosons = system.number_bosonic_modes();
        for (index, (system_bosons, noise_bosons)) in variable_number_bosons
            .iter_mut()
            .zip(noise_number_bosons.iter())
            .enumerate()
        {
            if system_bosons != noise_bosons {
                match (*system_bosons, noise_bosons) {
                    (Some(n), None) => {
                        if n < noise_number_current_bosons[index] {
                            return Err(StruqtureError::MissmatchedNumberModes);
                        }
                    }
                    (None, Some(n)) => {
                        if *n >= system_number_current_bosons[index] {
                            *system_bosons = Some(*n);
                        } else {
                            return Err(StruqtureError::MissmatchedNumberModes);
                        }
                    }
                    (Some(_), Some(_)) => {
                        return Err(StruqtureError::MissmatchedNumberModes);
                    }
                    _ => panic!("Unexpected missmatch of number modes"),
                }
            }
        }

        system.number_bosons = variable_number_bosons.clone();
        noise.number_bosons = variable_number_bosons;

        // Checking Fermion compatibility

        let mut variable_number_fermions = system.number_fermions.clone();
        let noise_number_fermions = noise.number_fermions.clone();
        let noise_number_current_fermions = noise.number_bosonic_modes();
        let system_number_current_fermions = system.number_bosonic_modes();
        for (index, (system_fermions, noise_fermions)) in variable_number_fermions
            .iter_mut()
            .zip(noise_number_fermions.iter())
            .enumerate()
        {
            if system_fermions != noise_fermions {
                match (*system_fermions, noise_fermions) {
                    (Some(n), None) => {
                        if n < noise_number_current_fermions[index] {
                            return Err(StruqtureError::MissmatchedNumberModes);
                        }
                    }
                    (None, Some(n)) => {
                        if *n >= system_number_current_fermions[index] {
                            *system_fermions = Some(*n);
                        } else {
                            return Err(StruqtureError::MissmatchedNumberModes);
                        }
                    }
                    (Some(_), Some(_)) => {
                        return Err(StruqtureError::MissmatchedNumberModes);
                    }
                    _ => panic!("Unexpected missmatch of number modes"),
                }
            }
        }

        system.number_fermions = variable_number_fermions.clone();
        noise.number_fermions = variable_number_fermions;

        Ok(Self { system, noise })
    }

    // From trait
    fn empty_clone(&self) -> Self {
        Self::group(self.system.empty_clone(None), self.noise.empty_clone(None)).expect(
            "Internal error: Number of spins in system and noise unexpectedly does not match.",
        )
    }
}

impl<'a> OperateOnMixedSystems<'a> for MixedLindbladOpenSystem {
    // From trait
    fn number_spins(&self) -> Vec<usize> {
        self.system
            .number_spins()
            .iter()
            .zip(self.noise.number_spins().iter())
            .map(|(s, n)| *(s.max(n)))
            .collect()
    }

    // From trait
    fn current_number_spins(&self) -> Vec<usize> {
        self.system
            .current_number_spins()
            .iter()
            .zip(self.noise.current_number_spins().iter())
            .map(|(s, n)| *(s.max(n)))
            .collect()
    }

    // From trait
    fn number_bosonic_modes(&self) -> Vec<usize> {
        self.system
            .number_bosonic_modes()
            .iter()
            .zip(self.noise.number_bosonic_modes().iter())
            .map(|(s, n)| *(s.max(n)))
            .collect()
    }

    // From trait
    fn current_number_bosonic_modes(&self) -> Vec<usize> {
        self.system
            .current_number_bosonic_modes()
            .iter()
            .zip(self.noise.current_number_bosonic_modes().iter())
            .map(|(s, n)| *(s.max(n)))
            .collect()
    }

    // From trait
    fn number_fermionic_modes(&self) -> Vec<usize> {
        self.system
            .number_fermionic_modes()
            .iter()
            .zip(self.noise.number_fermionic_modes().iter())
            .map(|(s, n)| *(s.max(n)))
            .collect()
    }

    // From trait
    fn current_number_fermionic_modes(&self) -> Vec<usize> {
        self.system
            .current_number_fermionic_modes()
            .iter()
            .zip(self.noise.current_number_fermionic_modes().iter())
            .map(|(s, n)| *(s.max(n)))
            .collect()
    }
}

/// Functions for the MixedLindbladOpenSystem
///
impl MixedLindbladOpenSystem {
    /// Creates a new MixedLindbladOpenSystem.
    ///
    /// # Arguments
    ///
    /// * `number_spins` - The number of spins in each spin subsystem.
    /// * `number_bosons` - The number of bosons in each bosonic subsystem.
    /// * `number_fermions` - The number of fermions in each fermionic subsystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) MixedLindbladOpenSystem.
    pub fn new(
        number_spins: impl IntoIterator<Item = Option<usize>>,
        number_bosons: impl IntoIterator<Item = Option<usize>>,
        number_fermions: impl IntoIterator<Item = Option<usize>>,
    ) -> Self {
        let number_spins: TinyVec<[Option<usize>; 2]> = number_spins.into_iter().collect();
        let number_bosons: TinyVec<[Option<usize>; 2]> = number_bosons.into_iter().collect();
        let number_fermions: TinyVec<[Option<usize>; 2]> = number_fermions.into_iter().collect();
        MixedLindbladOpenSystem {
            system: MixedHamiltonianSystem::new(
                number_spins.clone(),
                number_bosons.clone(),
                number_fermions.clone(),
            ),
            noise: MixedLindbladNoiseSystem::new(number_spins, number_bosons, number_fermions),
        }
    }
}

/// Implements the negative sign function of MixedLindbladOpenSystem.
///
impl ops::Neg for MixedLindbladOpenSystem {
    type Output = Self;
    /// Implement minus sign for MixedLindbladOpenSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The MixedLindbladOpenSystem * -1.
    fn neg(self) -> Self {
        let (self_sys, self_noise) = self.ungroup();
        Self {
            system: self_sys.neg(),
            noise: self_noise.neg(),
        }
    }
}

/// Implements the plus function of MixedLindbladOpenSystem by MixedLindbladOpenSystem.
///
impl ops::Add<MixedLindbladOpenSystem> for MixedLindbladOpenSystem {
    type Output = Result<Self, StruqtureError>;
    /// Implements `+` (add) for two MixedLindbladOpenSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The MixedLindbladOpenSystem to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two MixedLindbladOpenSystems added together.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in MixedHamiltonianSystem and key do not match.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in MixedLindbladNoiseSystem and key do not match.
    fn add(self, other: MixedLindbladOpenSystem) -> Self::Output {
        let (self_sys, self_noise) = self.ungroup();
        let (other_sys, other_noise) = other.ungroup();
        Self::group((self_sys + other_sys)?, (self_noise + other_noise)?)
    }
}

/// Implements the minus function of MixedLindbladOpenSystem by MixedLindbladOpenSystem.
///
impl ops::Sub<MixedLindbladOpenSystem> for MixedLindbladOpenSystem {
    type Output = Result<Self, StruqtureError>;
    /// Implements `-` (subtract) for two MixedLindbladOpenSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The MixedLindbladOpenSystem to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two MixedLindbladOpenSystems subtracted.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in MixedHamiltonianSystem and key do not match.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in MixedLindbladNoiseSystem and key do not match.
    fn sub(self, other: MixedLindbladOpenSystem) -> Self::Output {
        let (self_sys, self_noise) = self.ungroup();
        let (other_sys, other_noise) = other.ungroup();
        Self::group((self_sys - other_sys)?, (self_noise - other_noise)?)
    }
}

/// Implements the multiplication function of MixedLindbladOpenSystem by CalculatorFloat.
///
impl ops::Mul<CalculatorFloat> for MixedLindbladOpenSystem {
    type Output = Self;

    /// Implement `*` for MixedLindbladOpenSystem and CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The MixedLindbladNoiseSystem multiplied by the CalculatorFloat.
    fn mul(self, rhs: CalculatorFloat) -> Self::Output {
        Self {
            system: self.system * rhs.clone(),
            noise: self.noise * rhs,
        }
    }
}

/// Implements the format function (Display trait) of MixedLindbladOpenSystem.
///
impl fmt::Display for MixedLindbladOpenSystem {
    /// Formats the MixedLindbladOpenSystem using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted MixedLindbladOpenSystem.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "MixedLindbladOpenSystem{\n".to_string();
        output.push_str("System: {\n");
        output.push_str(format!("{}", self.system()).as_str());
        output.push_str("}\n");
        output.push_str("Noise: {\n");
        output.push_str(format!("{}", self.noise()).as_str());

        output.push_str("}\n");
        output.push('}');

        write!(f, "{}", output)
    }
}
