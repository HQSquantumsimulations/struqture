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

use super::SpinLindbladNoiseSystem;
use crate::fermions::FermionLindbladOpenSystem;
use crate::mappings::JordanWignerSpinToFermion;
use crate::spins::{OperateOnSpins, SpinHamiltonianSystem, ToSparseMatrixSuperOperator};
use crate::{CooSparseMatrix, OpenSystem, OperateOnDensityMatrix, StruqtureError};
use num_complex::Complex64;
use qoqo_calculator::CalculatorFloat;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::ops;

/// SpinLindbladOpenSystems are representations of open systems of spins, where a system (SpinHamiltonianSystem) interacts with the environment via noise (SpinLindbladNoiseSystem).
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::spins::{DecoherenceProduct, SpinLindbladOpenSystem, SpinLindbladNoiseSystem, SpinHamiltonianSystem};
///
/// let mut system = SpinLindbladOpenSystem::new(None);
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
pub struct SpinLindbladOpenSystem {
    /// The SpinHamiltonianSystem representing the system terms of the open system
    system: SpinHamiltonianSystem,
    /// The SpinLindbladNoiseSystem representing the noise terms of the open system
    noise: SpinLindbladNoiseSystem,
}

impl crate::MinSupportedVersion for SpinLindbladOpenSystem {}

impl<'a> OpenSystem<'a> for SpinLindbladOpenSystem {
    type System = SpinHamiltonianSystem;
    type Noise = SpinLindbladNoiseSystem;

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

    /// Takes a tuple of a system (SpinHamiltonianSystem) and a noise term (SpinLindbladNoiseSystem) and combines them to be a SpinLindbladOpenSystem.
    ///
    /// # Arguments
    ///
    /// * `system` - The SpinHamiltonianSystem to have in the SpinLindbladOpenSystem.
    /// * `noise` - The SpinLindbladNoiseSystem to have in the SpinLindbladOpenSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The SpinLindbladOpenSystem with input system and noise terms.
    /// * `Err(StruqtureError::MissmatchedNumberSpins)` - The system and noise do not have the same number of modes.
    fn group(system: Self::System, noise: Self::Noise) -> Result<Self, StruqtureError> {
        let (system, noise) = if system.number_spins != noise.number_spins {
            match (system.number_spins, noise.number_spins) {
                (Some(n), None) => {
                    if n >= noise.number_spins() {
                        let mut noise = noise;
                        noise.number_spins = Some(n);
                        (system, noise)
                    } else {
                        return Err(StruqtureError::MissmatchedNumberSpins);
                    }
                }
                (None, Some(n)) => {
                    if n >= system.number_spins() {
                        let mut system = system;
                        system.number_spins = Some(n);
                        (system, noise)
                    } else {
                        return Err(StruqtureError::MissmatchedNumberSpins);
                    }
                }
                (Some(_), Some(_)) => {
                    return Err(StruqtureError::MissmatchedNumberSpins);
                }
                _ => panic!("Unexpected missmatch of number spins"),
            }
        } else {
            (system, noise)
        };
        Ok(Self { system, noise })
    }

    // From trait
    fn empty_clone(&self) -> Self {
        Self::group(self.system.empty_clone(None), self.noise.empty_clone(None)).expect(
            "Internal error: Number of spins in system and noise unexpectedly does not match.",
        )
    }
}

impl<'a> OperateOnSpins<'a> for SpinLindbladOpenSystem {
    /// Gets the maximum number_spins of the SpinHamiltonianSystem/SpinLindbladNoiseSystem.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of spins in the SpinLindbladOpenSystem.
    fn number_spins(&self) -> usize {
        self.system.number_spins().max(self.noise.number_spins())
    }

    // From trait
    fn current_number_spins(&self) -> usize {
        self.system
            .current_number_spins()
            .max(self.noise.current_number_spins())
    }
}

impl<'a> ToSparseMatrixSuperOperator<'a> for SpinLindbladOpenSystem {
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
    fn unitary_sparse_matrix_coo(&'a self) -> Result<CooSparseMatrix, StruqtureError> {
        self.system.unitary_sparse_matrix_coo()
    }

    // From trait
    fn sparse_lindblad_entries(
        &'a self,
    ) -> Result<Vec<(CooSparseMatrix, CooSparseMatrix, Complex64)>, StruqtureError> {
        let mut coo_matrices =
            Vec::<(CooSparseMatrix, CooSparseMatrix, Complex64)>::with_capacity(self.noise.len());
        for ((left, right), val) in self.noise.iter() {
            coo_matrices.push((
                left.to_coo(self.number_spins()).unwrap(),
                right.to_coo(self.number_spins()).unwrap(),
                Complex64 {
                    re: *val.re.float()?,
                    im: *val.im.float()?,
                },
            ))
        }
        Ok(coo_matrices)
    }
}

/// Functions for the SpinLindbladOpenSystem
///
impl SpinLindbladOpenSystem {
    /// Creates a new SpinLindbladOpenSystem.
    ///
    /// # Arguments
    ///
    /// * `number_spins` - The number of spins in the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) SpinLindbladOpenSystem.
    pub fn new(number_spins: Option<usize>) -> Self {
        SpinLindbladOpenSystem {
            system: SpinHamiltonianSystem::new(number_spins),
            noise: SpinLindbladNoiseSystem::new(number_spins),
        }
    }
}

/// Implements the negative sign function of SpinLindbladOpenSystem.
///
impl ops::Neg for SpinLindbladOpenSystem {
    type Output = Self;
    /// Implement minus sign for SpinLindbladOpenSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The SpinLindbladOpenSystem * -1.
    fn neg(self) -> Self {
        let (self_sys, self_noise) = self.ungroup();
        Self {
            system: self_sys.neg(),
            noise: self_noise.neg(),
        }
    }
}

/// Implements the plus function of SpinLindbladOpenSystem by SpinLindbladOpenSystem.
///
impl ops::Add<SpinLindbladOpenSystem> for SpinLindbladOpenSystem {
    type Output = Result<Self, StruqtureError>;
    /// Implements `+` (add) for two SpinLindbladOpenSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The SpinLindbladOpenSystem to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two SpinLindbladOpenSystems added together.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Index of PauliProduct exceeds that of the SpinHamiltonianSystem.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Index of (DecoherenceProduct, DecoherenceProduct) exceeds that of the SpinLindbladNoiseSystem.
    fn add(self, other: SpinLindbladOpenSystem) -> Self::Output {
        let (self_sys, self_noise) = self.ungroup();
        let (other_sys, other_noise) = other.ungroup();
        Self::group((self_sys + other_sys)?, (self_noise + other_noise)?)
    }
}

/// Implements the minus function of SpinLindbladOpenSystem by SpinLindbladOpenSystem.
///
impl ops::Sub<SpinLindbladOpenSystem> for SpinLindbladOpenSystem {
    type Output = Result<Self, StruqtureError>;
    /// Implements `-` (subtract) for two SpinLindbladOpenSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The SpinLindbladOpenSystem to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two SpinLindbladOpenSystems subtracted.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Index of PauliProduct exceeds that of the SpinHamiltonianSystem.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Index of (DecoherenceProduct, DecoherenceProduct) exceeds that of the SpinLindbladNoiseSystem.
    fn sub(self, other: SpinLindbladOpenSystem) -> Self::Output {
        let (self_sys, self_noise) = self.ungroup();
        let (other_sys, other_noise) = other.ungroup();
        Self::group((self_sys - other_sys)?, (self_noise - other_noise)?)
    }
}

/// Implements the multiplication function of SpinLindbladOpenSystem by CalculatorFloat.
///
impl ops::Mul<CalculatorFloat> for SpinLindbladOpenSystem {
    type Output = Self;

    /// Implement `*` for SpinLindbladOpenSystem and CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorFloat by which to multiply..
    ///
    /// # Returns
    ///
    /// * `Self` - The SpinLindbladOpenSystem multiplied by the CalculatorFloat.
    fn mul(self, rhs: CalculatorFloat) -> Self::Output {
        Self {
            system: self.system * rhs.clone(),
            noise: self.noise * rhs,
        }
    }
}

/// Implements the format function (Display trait) of SpinLindbladOpenSystem.
///
impl fmt::Display for SpinLindbladOpenSystem {
    /// Formats the SpinLindbladOpenSystem using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted SpinLindbladOpenSystem.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("SpinLindbladOpenSystem({}){{\n", self.number_spins());
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

impl JordanWignerSpinToFermion for SpinLindbladOpenSystem {
    type Output = FermionLindbladOpenSystem;

    /// Implements JordanWignerSpinToSpin for a SpinLindbladOpenSystem.
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
    /// * Internal error in jordan_wigner() for SpinHamiltonianSystem or SpinLindbladNoiseSystem.
    fn jordan_wigner(&self) -> Self::Output {
        let jw_system = self.system().jordan_wigner();
        let jw_noise = self.noise().jordan_wigner();
        FermionLindbladOpenSystem::group(jw_system, jw_noise)
            .expect("Internal bug in jordan_wigner() for SpinHamiltonianSystem or SpinLindbladNoiseSystem. The number of modes in the fermionic system should equal the number of spins in the spin system.")
    }
}
