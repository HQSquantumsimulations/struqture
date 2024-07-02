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

use super::QubitLindbladNoiseOperator;
use crate::fermions::FermionLindbladOpenSystem;
use crate::mappings::JordanWignerSpinToFermion;
use crate::spins::{OperateOnSpins, QubitHamiltonian, ToSparseMatrixSuperOperator};
use crate::{CooSparseMatrix, OpenSystem, OperateOnDensityMatrix, StruqtureError};
use num_complex::Complex64;
use qoqo_calculator::CalculatorFloat;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::ops;

/// QubitLindbladOpenSystems are representations of open systems of spins, where a system (QubitHamiltonian) interacts with the environment via noise (QubitLindbladNoiseOperator).
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::spins::{DecoherenceProduct, QubitLindbladOpenSystem, QubitLindbladNoiseOperator, QubitHamiltonian};
///
/// let mut system = QubitLindbladOpenSystem::new();
///
/// // Representing the hamiltonian $ 1/2 \sigma_0^{X} \sigma_1^{X} + 1/5 \sigma_0^{z} $
/// let pp_01 = DecoherenceProduct::new().x(0).x(1);
/// let pp_0 = DecoherenceProduct::new().z(0);
/// system.noise_mut().set((pp_01.clone(), pp_01.clone()), CalculatorComplex::from(0.5)).unwrap();
/// system.noise_mut().set((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(system.noise().get(&(pp_01.clone(), pp_01.clone())), &CalculatorComplex::from(0.5));
/// assert_eq!(system.noise().get(&(pp_0.clone(), pp_0.clone())), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
pub struct QubitLindbladOpenSystem {
    /// The QubitHamiltonian representing the system terms of the open system
    system: QubitHamiltonian,
    /// The QubitLindbladNoiseOperator representing the noise terms of the open system
    noise: QubitLindbladNoiseOperator,
}

impl crate::SerializationSupport for QubitLindbladOpenSystem {
    fn struqture_type() -> crate::StruqtureType {
        crate::StruqtureType::QubitLindbladOpenSystem
    }
}
impl<'a> OpenSystem<'a> for QubitLindbladOpenSystem {
    type System = QubitHamiltonian;
    type Noise = QubitLindbladNoiseOperator;

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

    /// Takes a tuple of a system (QubitHamiltonian) and a noise term (QubitLindbladNoiseOperator) and combines them to be a QubitLindbladOpenSystem.
    ///
    /// # Arguments
    ///
    /// * `system` - The QubitHamiltonian to have in the QubitLindbladOpenSystem.
    /// * `noise` - The QubitLindbladNoiseOperator to have in the QubitLindbladOpenSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The QubitLindbladOpenSystem with input system and noise terms.
    /// * `Err(StruqtureError::MissmatchedNumberSpins)` - The system and noise do not have the same number of modes.
    fn group(system: Self::System, noise: Self::Noise) -> Result<Self, StruqtureError> {
        Ok(Self { system, noise })
    }

    // From trait
    fn empty_clone(&self) -> Self {
        Self::group(self.system.empty_clone(None), self.noise.empty_clone(None)).expect(
            "Internal error: Number of spins in system and noise unexpectedly does not match.",
        )
    }
}

impl<'a> OperateOnSpins<'a> for QubitLindbladOpenSystem {
    /// Gets the maximum number_spins of the QubitHamiltonian/QubitLindbladNoiseOperator.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of spins in the QubitLindbladOpenSystem.
    fn current_number_spins(&self) -> usize {
        self.system
            .current_number_spins()
            .max(self.noise.current_number_spins())
    }
}

impl<'a> ToSparseMatrixSuperOperator<'a> for QubitLindbladOpenSystem {
    // From trait
    fn sparse_matrix_superoperator_entries_on_row(
        &'a self,
        row: usize,
        number_spins: usize,
    ) -> Result<HashMap<usize, Complex64>, StruqtureError> {
        let mut system_row = self
            .system
            .sparse_matrix_superoperator_entries_on_row(row, number_spins)
            .unwrap();
        let noise_row = self
            .noise
            .sparse_matrix_superoperator_entries_on_row(row, number_spins)
            .unwrap();
        for (key, val) in noise_row.into_iter() {
            match system_row.get_mut(&key) {
                Some(x) => *x += val,
                None => {
                    system_row.insert(key, val);
                }
            }
        }
        Ok(system_row)
    }

    // From trait
    fn unitary_sparse_matrix_coo(
        &'a self,
        number_spins: Option<usize>,
    ) -> Result<CooSparseMatrix, StruqtureError> {
        self.system.unitary_sparse_matrix_coo(number_spins)
    }

    // From trait
    fn sparse_lindblad_entries(
        &'a self,
    ) -> Result<Vec<(CooSparseMatrix, CooSparseMatrix, Complex64)>, StruqtureError> {
        let mut coo_matrices =
            Vec::<(CooSparseMatrix, CooSparseMatrix, Complex64)>::with_capacity(self.noise.len());
        for ((left, right), val) in self.noise.iter() {
            coo_matrices.push((
                left.to_coo(self.current_number_spins()).unwrap(),
                right.to_coo(self.current_number_spins()).unwrap(),
                Complex64 {
                    re: *val.re.float()?,
                    im: *val.im.float()?,
                },
            ))
        }
        Ok(coo_matrices)
    }
}

/// Functions for the QubitLindbladOpenSystem
///
impl QubitLindbladOpenSystem {
    /// Creates a new QubitLindbladOpenSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) QubitLindbladOpenSystem.
    pub fn new() -> Self {
        QubitLindbladOpenSystem {
            system: QubitHamiltonian::new(),
            noise: QubitLindbladNoiseOperator::new(),
        }
    }

    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_export")]
    pub fn to_struqture_1(
        &self,
    ) -> Result<struqture_1::spins::SpinLindbladOpenSystem, StruqtureError> {
        let new_system = self.system().to_struqture_1()?;
        let new_noise = self.noise().to_struqture_1()?;

        struqture_1::OpenSystem::group(new_system, new_noise).map_err(
            |err| StruqtureError::GenericError { msg:
                format!("Could not convert struqture 2.x QubitLindbladOpenSystem to 1.x SpinLindbladOpenSystem, group function failed: {:?}.", err)
            }
        )
    }

    /// Import from struqture_1 format.
    #[cfg(feature = "struqture_1_import")]
    pub fn from_struqture_1(
        value: &struqture_1::spins::SpinLindbladOpenSystem,
    ) -> Result<Self, StruqtureError> {
        let (system_one, noise_one) = struqture_1::OpenSystem::ungroup(value.clone());
        let new_system = QubitHamiltonian::from_struqture_1(&system_one)?;
        let new_noise = QubitLindbladNoiseOperator::from_struqture_1(&noise_one)?;
        Self::group(new_system, new_noise)
    }
}

/// Implements the negative sign function of QubitLindbladOpenSystem.
///
impl ops::Neg for QubitLindbladOpenSystem {
    type Output = Self;
    /// Implement minus sign for QubitLindbladOpenSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The QubitLindbladOpenSystem * -1.
    fn neg(self) -> Self {
        let (self_sys, self_noise) = self.ungroup();
        Self {
            system: self_sys.neg(),
            noise: self_noise.neg(),
        }
    }
}

/// Implements the plus function of QubitLindbladOpenSystem by QubitLindbladOpenSystem.
///
impl ops::Add<QubitLindbladOpenSystem> for QubitLindbladOpenSystem {
    type Output = Result<Self, StruqtureError>;
    /// Implements `+` (add) for two QubitLindbladOpenSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The QubitLindbladOpenSystem to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two QubitLindbladOpenSystems added together.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Index of PauliProduct exceeds that of the QubitHamiltonian.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Index of (DecoherenceProduct, DecoherenceProduct) exceeds that of the QubitLindbladNoiseOperator.
    fn add(self, other: QubitLindbladOpenSystem) -> Self::Output {
        let (self_sys, self_noise) = self.ungroup();
        let (other_sys, other_noise) = other.ungroup();
        Self::group(self_sys + other_sys, self_noise + other_noise)
    }
}

/// Implements the minus function of QubitLindbladOpenSystem by QubitLindbladOpenSystem.
///
impl ops::Sub<QubitLindbladOpenSystem> for QubitLindbladOpenSystem {
    type Output = Result<Self, StruqtureError>;
    /// Implements `-` (subtract) for two QubitLindbladOpenSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The QubitLindbladOpenSystem to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two QubitLindbladOpenSystems subtracted.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Index of PauliProduct exceeds that of the QubitHamiltonian.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Index of (DecoherenceProduct, DecoherenceProduct) exceeds that of the QubitLindbladNoiseOperator.
    fn sub(self, other: QubitLindbladOpenSystem) -> Self::Output {
        let (self_sys, self_noise) = self.ungroup();
        let (other_sys, other_noise) = other.ungroup();
        Self::group(self_sys - other_sys, self_noise - other_noise)
    }
}

/// Implements the multiplication function of QubitLindbladOpenSystem by CalculatorFloat.
///
impl ops::Mul<CalculatorFloat> for QubitLindbladOpenSystem {
    type Output = Self;

    /// Implement `*` for QubitLindbladOpenSystem and CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorFloat by which to multiply..
    ///
    /// # Returns
    ///
    /// * `Self` - The QubitLindbladOpenSystem multiplied by the CalculatorFloat.
    fn mul(self, rhs: CalculatorFloat) -> Self::Output {
        Self {
            system: self.system * rhs.clone(),
            noise: self.noise * rhs,
        }
    }
}

/// Implements the format function (Display trait) of QubitLindbladOpenSystem.
///
impl fmt::Display for QubitLindbladOpenSystem {
    /// Formats the QubitLindbladOpenSystem using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted QubitLindbladOpenSystem.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "QubitLindbladOpenSystem{\n".to_string();
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

impl JordanWignerSpinToFermion for QubitLindbladOpenSystem {
    type Output = FermionLindbladOpenSystem;

    /// Implements JordanWignerSpinToSpin for a QubitLindbladOpenSystem.
    ///
    /// The convention used is that |0> represents an empty fermionic state (spin-orbital),
    /// and |1> represents an occupied fermionic state.
    ///
    /// # Returns
    ///
    /// `FermionLindbladOpenSystem` - The fermion open system that results from the transformation.
    ///
    /// # Panics
    ///
    /// * Internal error in jordan_wigner() for QubitHamiltonian or QubitLindbladNoiseOperator.
    fn jordan_wigner(&self) -> Self::Output {
        let jw_system = self.system().jordan_wigner();
        let jw_noise = self.noise().jordan_wigner();
        FermionLindbladOpenSystem::group(jw_system, jw_noise)
            .expect("Internal bug in jordan_wigner() for QubitHamiltonian or QubitLindbladNoiseOperator. The number of modes in the fermionic system should equal the number of spins in the spin system.")
    }
}
