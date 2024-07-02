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

use super::{BosonHamiltonian, BosonLindbladNoiseOperator};
use crate::{OpenSystem, OperateOnDensityMatrix, OperateOnModes, StruqtureError};
use qoqo_calculator::CalculatorFloat;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Write};
use std::ops;

/// BosonLindbladOpenSystems are representations of open systems of bosons, where a system (BosonHamiltonian) interacts with the environment via noise (BosonLindbladNoiseOperator).
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::bosons::{BosonProduct, HermitianBosonProduct, BosonLindbladOpenSystem};
///
/// let mut system = BosonLindbladOpenSystem::new();
///
/// let bp_0_1 = BosonProduct::new([0], [1]).unwrap();
/// let bp_0 = HermitianBosonProduct::new([], [0]).unwrap();
/// system.noise_mut().set((bp_0_1.clone(), bp_0_1.clone()), CalculatorComplex::from(0.5)).unwrap();
/// system.system_mut().set(bp_0.clone(), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(system.noise().get(&(bp_0_1.clone(), bp_0_1.clone())), &CalculatorComplex::from(0.5));
/// assert_eq!(system.system().get(&bp_0.clone()), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
pub struct BosonLindbladOpenSystem {
    /// The BosonHamiltonian representing the system terms of the open system
    system: BosonHamiltonian,
    /// The BosonLindbladNoiseOperator representing the noise terms of the open system
    noise: BosonLindbladNoiseOperator,
}

impl crate::SerializationSupport for BosonLindbladOpenSystem {
    fn struqture_type() -> crate::StruqtureType {
        crate::StruqtureType::BosonLindbladOpenSystem
    }
}

impl<'a> OpenSystem<'a> for BosonLindbladOpenSystem {
    type System = BosonHamiltonian;
    type Noise = BosonLindbladNoiseOperator;

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

    /// Takes a tuple of a system (BosonHamiltonian) and a noise term (BosonLindbladNoiseOperator) and combines them to be a BosonLindbladOpenSystem.
    ///
    /// # Arguments
    ///
    /// * `system` - The BosonHamiltonian to have in the BosonLindbladOpenSystem.
    /// * `noise` - The BosonLindbladNoiseOperator to have in the BosonLindbladOpenSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The BosonLindbladOpenSystem with input system and noise terms.
    /// * `Err(StruqtureError::MissmatchedNumberModes)` - The system and noise do not have the same number of modes.
    fn group(system: Self::System, noise: Self::Noise) -> Result<Self, StruqtureError> {
        Ok(Self { system, noise })
    }

    // From trait
    fn empty_clone(&self) -> Self {
        Self::group(self.system.empty_clone(None), self.noise.empty_clone(None)).expect(
            "Internal error: Number of modes in system and noise unexpectedly does not match.",
        )
    }
}

impl<'a> OperateOnModes<'a> for BosonLindbladOpenSystem {
    /// Gets the maximum current_number_modes of the BosonHamiltonian/BosonLindbladNoiseOperator.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of bosons in the BosonLindbladOpenSystem.
    fn current_number_modes(&self) -> usize {
        self.system
            .current_number_modes()
            .max(self.noise.current_number_modes())
    }
}

/// Functions for the BosonLindbladOpenSystem
///
impl BosonLindbladOpenSystem {
    /// Creates a new BosonLindbladOpenSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) BosonLindbladOpenSystem.
    pub fn new() -> Self {
        BosonLindbladOpenSystem {
            system: BosonHamiltonian::new(),
            noise: BosonLindbladNoiseOperator::new(),
        }
    }

    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_export")]
    pub fn to_struqture_1(
        &self,
    ) -> Result<struqture_1::bosons::BosonLindbladOpenSystem, StruqtureError> {
        let new_system = self.system().to_struqture_1()?;
        let new_noise = self.noise().to_struqture_1()?;

        struqture_1::OpenSystem::group(new_system, new_noise).map_err(
            |err| StruqtureError::GenericError { msg:
                format!("Could not convert struqture 2.x BosonLindbladOpenSystem to 1.x BosonLindbladOpenSystem, group function failed: {:?}.", err)
            }
        )
    }

    /// Import from struqture_1 format.
    #[cfg(feature = "struqture_1_import")]
    pub fn from_struqture_1(
        value: &struqture_1::bosons::BosonLindbladOpenSystem,
    ) -> Result<Self, StruqtureError> {
        let (system_one, noise_one) = struqture_1::OpenSystem::ungroup(value.clone());
        let new_system = BosonHamiltonian::from_struqture_1(&system_one)?;
        let new_noise = BosonLindbladNoiseOperator::from_struqture_1(&noise_one)?;
        Self::group(new_system, new_noise)
    }
}

/// Implements the negative sign function of BosonLindbladOpenSystem.
///
impl ops::Neg for BosonLindbladOpenSystem {
    type Output = Self;
    /// Implement minus sign for BosonLindbladOpenSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The BosonLindbladOpenSystem * -1.
    fn neg(self) -> Self {
        let (self_sys, self_noise) = self.ungroup();
        Self {
            system: self_sys.neg(),
            noise: self_noise.neg(),
        }
    }
}

/// Implements the plus function of BosonLindbladOpenSystem by BosonLindbladOpenSystem.
///
impl ops::Add<BosonLindbladOpenSystem> for BosonLindbladOpenSystem {
    type Output = Result<Self, StruqtureError>;
    /// Implements `+` (add) for two BosonLindbladOpenSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The BosonLindbladOpenSystem to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two BosonLindbladOpenSystems added together.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of HermitianBosonProduct exceeds that of the BosonHamiltonian.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of (BosonProduct, BosonProduct) exceeds that of the BosonLindbladNoiseOperator.
    fn add(self, other: BosonLindbladOpenSystem) -> Self::Output {
        let (self_sys, self_noise) = self.ungroup();
        let (other_sys, other_noise) = other.ungroup();
        Self::group((self_sys + other_sys)?, self_noise + other_noise)
    }
}

/// Implements the minus function of BosonLindbladOpenSystem by BosonLindbladOpenSystem.
///
impl ops::Sub<BosonLindbladOpenSystem> for BosonLindbladOpenSystem {
    type Output = Result<Self, StruqtureError>;
    /// Implements `-` (subtract) for two BosonLindbladOpenSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The BosonLindbladOpenSystem to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two BosonLindbladOpenSystems added together.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of HermitianBosonProduct exceeds that of the BosonHamiltonian.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of (BosonProduct, BosonProduct) exceeds that of the BosonLindbladNoiseOperator.
    fn sub(self, other: BosonLindbladOpenSystem) -> Self::Output {
        let (self_sys, self_noise) = self.ungroup();
        let (other_sys, other_noise) = other.ungroup();
        Self::group((self_sys - other_sys)?, self_noise - other_noise)
    }
}

/// Implements the multiplication function of BosonLindbladOpenSystem by CalculatorFloat.
///
impl ops::Mul<CalculatorFloat> for BosonLindbladOpenSystem {
    type Output = Self;
    /// Implement `*` for BosonLindbladOpenSystem and CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The BosonLindbladNoiseOperator multiplied by the CalculatorFloat.
    fn mul(self, rhs: CalculatorFloat) -> Self::Output {
        Self {
            system: self.system * rhs.clone(),
            noise: self.noise * rhs,
        }
    }
}

/// Implements the format function (Display trait) of BosonLindbladOpenSystem.
///
impl fmt::Display for BosonLindbladOpenSystem {
    /// Formats the BosonLindbladOpenSystem using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted BosonLindbladOpenSystem.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "BosonLindbladOpenSystem{\n".to_string();
        output.push_str("System: {\n");
        for (key, val) in self.system.iter() {
            writeln!(output, "{}: {},", key, val)?;
        }
        output.push_str("}\n");
        output.push_str("Noise: {\n");
        for ((row, column), val) in self.noise.iter() {
            writeln!(output, "({}, {}): {},", row, column, val)?;
        }
        output.push_str("}\n");
        output.push('}');

        write!(f, "{}", output)
    }
}
