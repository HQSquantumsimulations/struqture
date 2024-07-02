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

use super::{MixedHamiltonian, MixedLindbladNoiseOperator, OperateOnMixedSystems};
use crate::{OpenSystem, OperateOnDensityMatrix, StruqtureError};
use qoqo_calculator::CalculatorFloat;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops;

/// MixedLindbladOpenSystems are representations of open systems of spins, where a system (MixedHamiltonian) interacts with the environment via noise (MixedLindbladNoiseOperator).
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
/// let mut system = MixedLindbladOpenSystem::new(1, 1, 1);
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
    /// The MixedHamiltonian representing the system terms of the open system
    system: MixedHamiltonian,
    /// The MixedLindbladNoiseOperator representing the noise terms of the open system
    noise: MixedLindbladNoiseOperator,
}

impl crate::SerializationSupport for MixedLindbladOpenSystem {
    fn struqture_type() -> crate::StruqtureType {
        crate::StruqtureType::MixedLindbladOpenSystem
    }
}
impl<'a> OpenSystem<'a> for MixedLindbladOpenSystem {
    type System = MixedHamiltonian;
    type Noise = MixedLindbladNoiseOperator;

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

    /// Takes a tuple of a system (MixedHamiltonian) and a noise term (MixedLindbladNoiseOperator) and combines them to be a MixedLindbladOpenSystem.
    ///
    /// # Arguments
    ///
    /// * `system` - The MixedHamiltonian to have in the MixedLindbladOpenSystem.
    /// * `noise` - The MixedLindbladNoiseOperator to have in the MixedLindbladOpenSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The MixedLindbladOpenSystem with input system and noise terms.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and noise do not match.
    fn group(system: Self::System, noise: Self::Noise) -> Result<Self, StruqtureError> {
        if system.n_spins != noise.n_spins
            || system.n_bosons != noise.n_bosons
            || system.n_fermions != noise.n_fermions
        {
            return Err(StruqtureError::MissmatchedNumberSubsystems {
                target_number_spin_subsystems: system.n_spins,
                target_number_boson_subsystems: system.n_bosons,
                target_number_fermion_subsystems: system.n_fermions,
                actual_number_spin_subsystems: noise.n_spins,
                actual_number_boson_subsystems: noise.n_bosons,
                actual_number_fermion_subsystems: noise.n_fermions,
            });
        }

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
    fn current_number_spins(&self) -> Vec<usize> {
        self.system
            .current_number_spins()
            .iter()
            .zip(self.noise.current_number_spins().iter())
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
    /// * `number_spins` - The number of spin subsystems.
    /// * `number_bosons` - The number of bosonic subsystems.
    /// * `number_fermions` - The number of fermionic subsystems.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) MixedLindbladOpenSystem.
    pub fn new(number_spins: usize, number_bosons: usize, number_fermions: usize) -> Self {
        MixedLindbladOpenSystem {
            system: MixedHamiltonian::new(number_spins, number_bosons, number_fermions),
            noise: MixedLindbladNoiseOperator::new(number_spins, number_bosons, number_fermions),
        }
    }

    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_export")]
    pub fn to_struqture_1(
        &self,
    ) -> Result<struqture_1::mixed_systems::MixedLindbladOpenSystem, StruqtureError> {
        let new_system = self.system().to_struqture_1()?;
        let new_noise = self.noise().to_struqture_1()?;

        struqture_1::OpenSystem::group(new_system, new_noise).map_err(
            |err| StruqtureError::GenericError { msg:
                format!("Could not convert struqture 2.x MixedLindbladOpenSystem to 1.x MixedLindbladOpenSystem, group function failed: {:?}.", err)
            }
        )
    }

    /// Import from struqture_1 format.
    #[cfg(feature = "struqture_1_import")]
    pub fn from_struqture_1(
        value: &struqture_1::mixed_systems::MixedLindbladOpenSystem,
    ) -> Result<Self, StruqtureError> {
        let (system_one, noise_one) = struqture_1::OpenSystem::ungroup(value.clone());
        let new_system = MixedHamiltonian::from_struqture_1(&system_one)?;
        let new_noise = MixedLindbladNoiseOperator::from_struqture_1(&noise_one)?;
        Self::group(new_system, new_noise)
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
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in MixedHamiltonian and key do not match.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in MixedLindbladNoiseOperator and key do not match.
    fn add(self, other: MixedLindbladOpenSystem) -> Self::Output {
        let (self_sys, self_noise) = self.ungroup();
        let (other_sys, other_noise) = other.ungroup();
        Self::group((self_sys + other_sys)?, self_noise + other_noise)
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
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in MixedHamiltonian and key do not match.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in MixedLindbladNoiseOperator and key do not match.
    fn sub(self, other: MixedLindbladOpenSystem) -> Self::Output {
        let (self_sys, self_noise) = self.ungroup();
        let (other_sys, other_noise) = other.ungroup();
        Self::group((self_sys - other_sys)?, self_noise - other_noise)
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
    /// * `Self` - The MixedLindbladNoiseOperator multiplied by the CalculatorFloat.
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
