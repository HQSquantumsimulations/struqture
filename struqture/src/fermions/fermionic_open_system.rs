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

use super::{FermionHamiltonian, FermionLindbladNoiseOperator};
use crate::mappings::JordanWignerFermionToSpin;
use crate::spins::PauliLindbladOpenSystem;
use crate::{OpenSystem, OperateOnDensityMatrix, OperateOnModes, StruqtureError};
use qoqo_calculator::CalculatorFloat;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Write};
use std::ops;

/// FermionLindbladOpenSystems are representations of open systems of fermions, where a system (FermionHamiltonian) interacts with the environment via noise (FermionLindbladNoiseOperator).
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::fermions::{FermionProduct, HermitianFermionProduct, FermionLindbladOpenSystem, FermionHamiltonian};
///
/// let mut system = FermionLindbladOpenSystem::new();
///
/// let bp_0_1 = FermionProduct::new([0], [1]).unwrap();
/// let bp_0 = HermitianFermionProduct::new([], [0]).unwrap();
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
pub struct FermionLindbladOpenSystem {
    /// The FermionHamiltonian representing the system terms of the open system
    system: FermionHamiltonian,
    /// The FermionLindbladNoiseOperator representing the noise terms of the open system
    noise: FermionLindbladNoiseOperator,
}

impl crate::SerializationSupport for FermionLindbladOpenSystem {
    fn struqture_type() -> crate::StruqtureType {
        crate::StruqtureType::FermionLindbladOpenSystem
    }
}
impl OpenSystem<'_> for FermionLindbladOpenSystem {
    type System = FermionHamiltonian;
    type Noise = FermionLindbladNoiseOperator;

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

    /// Takes a tuple of a system (FermionHamiltonian) and a noise term (FermionLindbladNoiseOperator) and combines them to be a FermionLindbladOpenSystem.
    ///
    /// # Arguments
    ///
    /// * `system` - The FermionHamiltonian to have in the FermionLindbladOpenSystem.
    /// * `noise` - The FermionLindbladNoiseOperator to have in the FermionLindbladOpenSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The FermionLindbladOpenSystem with input system and noise terms.
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

impl OperateOnModes<'_> for FermionLindbladOpenSystem {
    /// Gets the maximum current_number_modes of the FermionHamiltonian/FermionLindbladNoiseOperator.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of fermions in the FermionLindbladOpenSystem.
    fn current_number_modes(&self) -> usize {
        self.system
            .current_number_modes()
            .max(self.noise.current_number_modes())
    }
}

/// Functions for the FermionLindbladOpenSystem
///
impl FermionLindbladOpenSystem {
    /// Creates a new FermionLindbladOpenSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) FermionLindbladOpenSystem.
    pub fn new() -> Self {
        FermionLindbladOpenSystem {
            system: FermionHamiltonian::new(),
            noise: FermionLindbladNoiseOperator::new(),
        }
    }

    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_export")]
    pub fn to_struqture_1(
        &self,
    ) -> Result<struqture_1::fermions::FermionLindbladOpenSystem, StruqtureError> {
        let new_system = self.system().to_struqture_1()?;
        let new_noise = self.noise().to_struqture_1()?;

        struqture_1::OpenSystem::group(new_system, new_noise).map_err(
            |err| StruqtureError::GenericError { msg:
                format!("Could not convert struqture 2.x FermionLindbladOpenSystem to 1.x FermionLindbladOpenSystem, group function failed: {:?}.", err)
            }
        )
    }

    /// Import from struqture_1 format.
    #[cfg(feature = "struqture_1_import")]
    pub fn from_struqture_1(
        value: &struqture_1::fermions::FermionLindbladOpenSystem,
    ) -> Result<Self, StruqtureError> {
        let (system_one, noise_one) = struqture_1::OpenSystem::ungroup(value.clone());
        let new_system = FermionHamiltonian::from_struqture_1(&system_one)?;
        let new_noise = FermionLindbladNoiseOperator::from_struqture_1(&noise_one)?;
        Self::group(new_system, new_noise)
    }
}

/// Implements the negative sign function of FermionLindbladOpenSystem.
///
impl ops::Neg for FermionLindbladOpenSystem {
    type Output = Self;
    /// Implement minus sign for FermionLindbladOpenSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The FermionLindbladOpenSystem * -1.
    fn neg(self) -> Self {
        let (self_sys, self_noise) = self.ungroup();
        Self {
            system: self_sys.neg(),
            noise: self_noise.neg(),
        }
    }
}

/// Implements the plus function of FermionLindbladOpenSystem by FermionLindbladOpenSystem.
///
impl ops::Add<FermionLindbladOpenSystem> for FermionLindbladOpenSystem {
    type Output = Result<Self, StruqtureError>;
    /// Implements `+` (add) for two FermionLindbladOpenSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The FermionLindbladOpenSystem to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two FermionLindbladOpenSystems added together.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of HermitianFermionProduct exceeds that of the FermionHamiltonian.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of (FermionProduct, FermionProduct) exceeds that of the FermionLindbladNoiseOperator.
    fn add(self, other: FermionLindbladOpenSystem) -> Self::Output {
        let (self_sys, self_noise) = self.ungroup();
        let (other_sys, other_noise) = other.ungroup();
        Self::group((self_sys + other_sys)?, self_noise + other_noise)
    }
}

/// Implements the minus function of FermionLindbladOpenSystem by FermionLindbladOpenSystem.
///
impl ops::Sub<FermionLindbladOpenSystem> for FermionLindbladOpenSystem {
    type Output = Result<Self, StruqtureError>;
    /// Implements `-` (subtract) for two FermionLindbladOpenSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The FermionLindbladOpenSystem to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two FermionLindbladOpenSystems subtracted.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of HermitianFermionProduct exceeds that of the FermionHamiltonian.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of (FermionProduct, FermionProduct) exceeds that of the FermionLindbladNoiseOperator.
    fn sub(self, other: FermionLindbladOpenSystem) -> Self::Output {
        let (self_sys, self_noise) = self.ungroup();
        let (other_sys, other_noise) = other.ungroup();
        Self::group((self_sys - other_sys)?, self_noise - other_noise)
    }
}

/// Implements the multiplication function of FermionLindbladOpenSystem by CalculatorFloat.
///
impl ops::Mul<CalculatorFloat> for FermionLindbladOpenSystem {
    type Output = Self;
    /// Implement `*` for FermionLindbladOpenSystem and CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The FermionLindbladNoiseOperator multiplied by the CalculatorFloat.
    fn mul(self, rhs: CalculatorFloat) -> Self::Output {
        Self {
            system: self.system * rhs.clone(),
            noise: self.noise * rhs,
        }
    }
}

/// Implements the format function (Display trait) of FermionLindbladOpenSystem.
///
impl fmt::Display for FermionLindbladOpenSystem {
    /// Formats the FermionLindbladOpenSystem using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted FermionLindbladOpenSystem.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "FermionLindbladOpenSystem{\n".to_string();
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

impl JordanWignerFermionToSpin for FermionLindbladOpenSystem {
    type Output = PauliLindbladOpenSystem;

    /// Implements JordanWignerFermionToSpin for a FermionLindbladOpenSystem.
    ///
    /// The convention used is that |0> represents an empty fermionic state (spin-orbital),
    /// and |1> represents an occupied fermionic state.
    ///
    /// # Returns
    ///
    /// `PauliLindbladOpenSystem` - The spin open system that results from the transformation.
    fn jordan_wigner(&self) -> Self::Output {
        let jw_system = self.system().jordan_wigner();
        let jw_noise = self.noise().jordan_wigner();
        PauliLindbladOpenSystem::group(jw_system, jw_noise)
            .expect("Internal bug in jordan_wigner() for FermionHamiltonian or FermionLindbladNoiseOperator. The number of modes in the fermionic system should equal the number of spins in the spin system.")
    }
}
