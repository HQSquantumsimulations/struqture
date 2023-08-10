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

use super::{BosonHamiltonianSystem, BosonLindbladNoiseSystem};
use crate::{OpenSystem, OperateOnDensityMatrix, OperateOnModes, StruqtureError};
use qoqo_calculator::CalculatorFloat;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Write};
use std::ops;

/// BosonLindbladOpenSystems are representations of open systems of bosons, where a system (BosonHamiltonianSystem) interacts with the environment via noise (BosonLindbladNoiseSystem).
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::bosons::{BosonProduct, HermitianBosonProduct, BosonLindbladOpenSystem};
///
/// let mut system = BosonLindbladOpenSystem::new(None);
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
    /// The BosonHamiltonianSystem representing the system terms of the open system
    system: BosonHamiltonianSystem,
    /// The BosonLindbladNoiseSystem representing the noise terms of the open system
    noise: BosonLindbladNoiseSystem,
}

impl crate::MinSupportedVersion for BosonLindbladOpenSystem {}

impl<'a> OpenSystem<'a> for BosonLindbladOpenSystem {
    type System = BosonHamiltonianSystem;
    type Noise = BosonLindbladNoiseSystem;

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

    /// Takes a tuple of a system (BosonHamiltonianSystem) and a noise term (BosonLindbladNoiseSystem) and combines them to be a BosonLindbladOpenSystem.
    ///
    /// # Arguments
    ///
    /// * `system` - The BosonHamiltonianSystem to have in the BosonLindbladOpenSystem.
    /// * `noise` - The BosonLindbladNoiseSystem to have in the BosonLindbladOpenSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The BosonLindbladOpenSystem with input system and noise terms.
    /// * `Err(StruqtureError::MissmatchedNumberModes)` - The system and noise do not have the same number of modes.
    fn group(system: Self::System, noise: Self::Noise) -> Result<Self, StruqtureError> {
        let (system, noise) = if system.number_modes != noise.number_modes {
            match (system.number_modes, noise.number_modes) {
                (Some(n), None) => {
                    if n >= noise.number_modes() {
                        let mut noise = noise;
                        noise.number_modes = Some(n);
                        (system, noise)
                    } else {
                        return Err(StruqtureError::MissmatchedNumberModes);
                    }
                }
                (None, Some(n)) => {
                    if n >= system.number_modes() {
                        let mut system = system;
                        system.number_modes = Some(n);
                        (system, noise)
                    } else {
                        return Err(StruqtureError::MissmatchedNumberModes);
                    }
                }
                (Some(_), Some(_)) => {
                    return Err(StruqtureError::MissmatchedNumberModes);
                }
                _ => panic!("Unexpected missmatch of number modes"),
            }
        } else {
            (system, noise)
        };
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
    /// Gets the maximum number_modes of the BosonHamiltonianSystem/BosonLindbladNoiseSystem.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of bosons in the BosonLindbladOpenSystem.
    fn number_modes(&self) -> usize {
        self.system.number_modes().max(self.noise.number_modes())
    }

    /// Return maximum index in BosonLindbladOpenSystem.
    ///
    /// # Returns
    ///
    /// * `usize` - Maximum index.
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
    /// # Arguments
    ///
    /// * `number_modes` - The number of modes in the open system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) BosonLindbladOpenSystem.
    pub fn new(number_modes: Option<usize>) -> Self {
        BosonLindbladOpenSystem {
            system: BosonHamiltonianSystem::new(number_modes),
            noise: BosonLindbladNoiseSystem::new(number_modes),
        }
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
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of HermitianBosonProduct exceeds that of the BosonHamiltonianSystem.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of (BosonProduct, BosonProduct) exceeds that of the BosonLindbladNoiseSystem.
    fn add(self, other: BosonLindbladOpenSystem) -> Self::Output {
        let (self_sys, self_noise) = self.ungroup();
        let (other_sys, other_noise) = other.ungroup();
        Self::group((self_sys + other_sys)?, (self_noise + other_noise)?)
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
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of HermitianBosonProduct exceeds that of the BosonHamiltonianSystem.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of (BosonProduct, BosonProduct) exceeds that of the BosonLindbladNoiseSystem.
    fn sub(self, other: BosonLindbladOpenSystem) -> Self::Output {
        let (self_sys, self_noise) = self.ungroup();
        let (other_sys, other_noise) = other.ungroup();
        Self::group((self_sys - other_sys)?, (self_noise - other_noise)?)
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
    /// * `Self` - The BosonLindbladNoiseSystem multiplied by the CalculatorFloat.
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
        let mut output = format!("BosonLindbladOpenSystem({}){{\n", self.number_modes());
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
