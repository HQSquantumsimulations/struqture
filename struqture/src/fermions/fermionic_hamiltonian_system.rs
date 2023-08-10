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
    FermionHamiltonian, FermionSystem, HermitianFermionProduct, ModeIndex, OperateOnFermions,
};
use crate::mappings::JordanWignerFermionToSpin;
use crate::spins::SpinHamiltonianSystem;
use crate::{OperateOnDensityMatrix, OperateOnModes, OperateOnState, StruqtureError};
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Iter, Keys, Values};
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

/// FermionHamiltonianSystems are combinations of FermionProducts with specific CalculatorFloat coefficients.
///
/// This is a representation of sums of pauli products with weightings, in order to build a full hamiltonian.
/// FermionHamiltonianSystem is the hermitian equivalent of FermionOperator.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::fermions::{ HermitianFermionProduct, FermionHamiltonianSystem};
/// use struqture::prelude::*;
///
/// let mut fhs = FermionHamiltonianSystem::new(Some(2));
///
/// let fp_0_1 = HermitianFermionProduct::new([0], [1]).unwrap();
/// let fp_0 = HermitianFermionProduct::new([], [0]).unwrap();
/// fhs.set(fp_0_1.clone(), CalculatorComplex::from(0.5)).unwrap();
/// fhs.set(fp_0.clone(), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(fhs.get(&fp_0_1), &CalculatorComplex::from(0.5));
/// assert_eq!(fhs.get(&fp_0), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
pub struct FermionHamiltonianSystem {
    /// The number of fermions in the FermionHamiltonianSystem
    pub(crate) number_modes: Option<usize>,
    /// The FermionHamiltonian representing the Hamiltonian of the FermionHamiltonianSystem
    pub(crate) hamiltonian: FermionHamiltonian,
}

impl crate::MinSupportedVersion for FermionHamiltonianSystem {}

impl<'a> OperateOnDensityMatrix<'a> for FermionHamiltonianSystem {
    type Index = HermitianFermionProduct;
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
    fn remove(&mut self, key: &Self::Index) -> Option<CalculatorComplex> {
        self.hamiltonian.remove(key)
    }

    // From trait
    fn empty_clone(&self, capacity: Option<usize>) -> Self {
        match capacity {
            Some(cap) => Self {
                number_modes: self.number_modes,
                hamiltonian: FermionHamiltonian::with_capacity(cap),
            },
            None => Self {
                number_modes: self.number_modes,
                hamiltonian: FermionHamiltonian::new(),
            },
        }
    }

    /// Overwrites an existing entry or sets a new entry in the FermionHamiltonianSystem with the given (HermitianFermionProduct key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The HermitianFermionProduct key to set in the FermionHamiltonianSystem.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the FermionHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(CalculatorComplex))` - The key existed, this is the value it had before it was set with the value input.
    /// * `Ok(None)` - The key did not exist, it has been set with its corresponding value.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of HermitianFermionProduct exceeds that of the FermionHamiltonianSystem.
    fn set(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<Option<Self::Value>, StruqtureError> {
        match self.number_modes {
            Some(x) => {
                if key.current_number_modes() <= x {
                    self.hamiltonian.set(key, value)
                } else {
                    Err(StruqtureError::NumberModesExceeded)
                }
            }
            None => self.hamiltonian.set(key, value),
        }
    }

    /// Adds a new (HermitianFermionProduct key, CalculatorComplex value) pair to the FermionHamiltonianSystem.
    ///
    /// # Arguments
    ///
    /// * `key` - The HermitianFermionProduct key to added to the FermionHamiltonianSystem.
    /// * `value` - The corresponding CalculatorComplex value to add for the key in the FermionHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The (key, value) pair was successfully added.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of HermitianFermionProduct exceeds that of the FermionHamiltonianSystem.
    fn add_operator_product(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<(), StruqtureError> {
        match self.number_modes {
            Some(x) => {
                if key.current_number_modes() <= x {
                    self.hamiltonian.add_operator_product(key, value)
                } else {
                    Err(StruqtureError::NumberModesExceeded)
                }
            }
            None => self.hamiltonian.add_operator_product(key, value),
        }
    }
}

impl<'a> OperateOnState<'a> for FermionHamiltonianSystem {
    // From trait
    fn hermitian_conjugate(&self) -> Self {
        self.clone()
    }
}

impl<'a> OperateOnModes<'a> for FermionHamiltonianSystem {
    /// Gets the number_modes input of the FermionHamiltonianSystem, or the current_number_modes if number_modes is None.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of fermions in the FermionHamiltonianSystem.
    fn number_modes(&self) -> usize {
        match self.number_modes {
            Some(fermions) => fermions,
            None => self.hamiltonian.current_number_modes(),
        }
    }

    /// Return maximum index in FermionHamiltonianSystem internal_map.
    ///
    /// # Returns
    ///
    /// * `usize` - Maximum index.
    fn current_number_modes(&self) -> usize {
        self.hamiltonian.current_number_modes()
    }
}

impl<'a> OperateOnFermions<'a> for FermionHamiltonianSystem {}

/// Functions for the FermionHamiltonianSystem
///
impl FermionHamiltonianSystem {
    /// Creates a new FermionHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) FermionHamiltonianSystem.
    pub fn new(number_modes: Option<usize>) -> Self {
        FermionHamiltonianSystem {
            number_modes,
            hamiltonian: FermionHamiltonian::new(),
        }
    }

    /// Creates a new FermionHamiltonianSystem with pre-allocated capacity.
    ///
    /// # Arguments
    ///
    /// * `number_modes` - The number of modes of the system.
    /// * `capacity` - The pre-allocated capacity of the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) FermionHamiltonianSystem with the given number of modes and capacity.
    pub fn with_capacity(number_modes: Option<usize>, capacity: usize) -> Self {
        FermionHamiltonianSystem {
            number_modes,
            hamiltonian: FermionHamiltonian::with_capacity(capacity),
        }
    }

    /// Returns the FermionHamiltonian of the FermionHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `&FermionHamiltonian` - The FermionHamiltonian of the FermionHamiltonianSystem.
    pub fn hamiltonian(&self) -> &FermionHamiltonian {
        &self.hamiltonian
    }

    /// Creates a FermionHamiltonianSystem from a FermionHamiltonian and an optional number of fermionic modes.
    ///
    /// # Arguments
    ///
    /// * `operator` - The FermionHamiltonian to create the FermionHamiltonianSystem from.
    /// * `number_modes` - The optional number of fermionic modes for the FermionHamiltonianSystem to be created.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The FermionHamiltonianSystem created from the inputs.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Number of fermionic modes in entry exceeds number of fermionic modes in system.
    pub fn from_hamiltonian(
        hamiltonian: FermionHamiltonian,
        number_modes: Option<usize>,
    ) -> Result<Self, StruqtureError> {
        match number_modes {
            Some(x) => {
                if hamiltonian.current_number_modes() <= x {
                    Ok(FermionHamiltonianSystem {
                        number_modes: Some(x),
                        hamiltonian,
                    })
                } else {
                    Err(StruqtureError::NumberModesExceeded)
                }
            }
            None => Ok(FermionHamiltonianSystem {
                number_modes: None,
                hamiltonian,
            }),
        }
    }

    /// Separate self into an operator with the terms of given number of creation and annihilation operators and an operator with the remaining operations
    ///
    /// # Arguments
    ///
    /// * `number_creators_annihilators` - Number of creation and annihilation terms to filter for in the keys.
    ///
    /// # Returns
    ///
    /// `Ok((separated, remainder))` - Operator with the noise terms where number_creators_annihilators matches the number of spins the operator product acts on and Operator with all other contributions.
    pub fn separate_into_n_terms(
        &self,
        number_creators_annihilators: (usize, usize),
    ) -> Result<(Self, Self), StruqtureError> {
        let mut separated = Self::default();
        let mut remainder = Self::default();
        for (prod, val) in self.iter() {
            if (prod.creators().len(), prod.annihilators().len()) == number_creators_annihilators {
                separated.add_operator_product(prod.clone(), val.clone())?;
            } else {
                remainder.add_operator_product(prod.clone(), val.clone())?;
            }
        }
        Ok((separated, remainder))
    }
}

/// Implements the negative sign function of FermionHamiltonianSystem.
///
impl ops::Neg for FermionHamiltonianSystem {
    type Output = Self;
    /// Implement minus sign for FermionHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The FermionHamiltonianSystem * -1.
    fn neg(mut self) -> Self {
        self.hamiltonian = self.hamiltonian.neg();
        self
    }
}

/// Implements the plus function of FermionHamiltonianSystem by FermionHamiltonianSystem.
///
impl<T, V> ops::Add<T> for FermionHamiltonianSystem
where
    T: IntoIterator<Item = (HermitianFermionProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `+` (add) for two FermionHamiltonianSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The FermionHamiltonianSystem to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two FermionHamiltonianSystems added together.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Number of fermionic modes in entry exceeds number of fermionic modes in system.
    fn add(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value))?;
        }
        Ok(self)
    }
}

/// Implements the minus function of FermionHamiltonianSystem by FermionHamiltonianSystem.
///
impl<T, V> ops::Sub<T> for FermionHamiltonianSystem
where
    T: IntoIterator<Item = (HermitianFermionProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `-` (subtract) for two FermionHamiltonianSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The FermionHamiltonianSystem to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two FermionHamiltonianSystems subtracted.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Number of fermionic modes in entry exceeds number of fermionic modes in system.
    fn sub(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value) * -1.0)?;
        }
        Ok(self)
    }
}

/// Implements the multiplication function of FermionHamiltonianSystem by CalculatorFloat.
///
impl ops::Mul<CalculatorFloat> for FermionHamiltonianSystem {
    type Output = Self;
    /// Implement `*` for FermionHamiltonianSystem and CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The FermionHamiltonianSystem multiplied by the CalculatorFloat.
    fn mul(mut self, other: CalculatorFloat) -> Self {
        self.hamiltonian = self.hamiltonian * other;
        self
    }
}

/// Implements the multiplication function of FermionHamiltonianSystem by CalculatorComplex.
///
impl ops::Mul<CalculatorComplex> for FermionHamiltonianSystem {
    type Output = Result<FermionSystem, StruqtureError>;
    /// Implement `*` for FermionHamiltonianSystem and CalculatorComplex.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Ok(FermionSystem)` - The FermionHamiltonianSystem multiplied by the CalculatorComplex.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Number of fermionic modes in entry exceeds number of fermionic modes in system.
    fn mul(self, other: CalculatorComplex) -> Self::Output {
        let mut system = FermionSystem::new(self.number_modes);
        system.operator = (self.hamiltonian * other)?;
        Ok(system)
    }
}

/// Implements the multiplication function of FermionHamiltonianSystem by FermionHamiltonianSystem.
///
impl ops::Mul<FermionHamiltonianSystem> for FermionHamiltonianSystem {
    type Output = Result<FermionSystem, StruqtureError>;
    /// Implement `*` for FermionHamiltonianSystem and FermionHamiltonianSystem.
    ///
    /// # Arguments
    ///
    /// * `other` - The FermionHamiltonianSystem to multiply by.
    ///
    /// # Returns
    ///
    /// * `Ok(FermionSystem)` - The two FermionHamiltonianSystems multiplied.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Number of fermionic modes in entry exceeds number of fermionic modes in system.
    fn mul(self, other: FermionHamiltonianSystem) -> Self::Output {
        Ok(FermionSystem {
            number_modes: Some(self.number_modes().max(other.number_modes())),
            operator: (self.hamiltonian * other.hamiltonian)?,
        })
    }
}

/// Implements the into_iter function (IntoIterator trait) of FermionHamiltonianSystem.
///
impl IntoIterator for FermionHamiltonianSystem {
    type Item = (HermitianFermionProduct, CalculatorComplex);
    type IntoIter =
        std::collections::hash_map::IntoIter<HermitianFermionProduct, CalculatorComplex>;
    /// Returns the FermionHamiltonianSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The FermionHamiltonianSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.hamiltonian.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference FermionHamiltonianSystem.
///
impl<'a> IntoIterator for &'a FermionHamiltonianSystem {
    type Item = (&'a HermitianFermionProduct, &'a CalculatorComplex);
    type IntoIter = Iter<'a, HermitianFermionProduct, CalculatorComplex>;

    /// Returns the reference FermionHamiltonianSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference FermionHamiltonianSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.hamiltonian.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of FermionHamiltonianSystem.
///
impl FromIterator<(HermitianFermionProduct, CalculatorComplex)> for FermionHamiltonianSystem {
    /// Returns the object in FermionHamiltonianSystem form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the FermionHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in FermionHamiltonianSystem form.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from_iter<I: IntoIterator<Item = (HermitianFermionProduct, CalculatorComplex)>>(
        iter: I,
    ) -> Self {
        let mut bhs = FermionHamiltonianSystem::new(None);
        for (bp, cc) in iter {
            bhs.add_operator_product(bp.clone(), cc.clone())
                .expect("Internal error in add_operator_product");
        }
        bhs
    }
}

/// Implements the extend function (Extend trait) of FermionHamiltonianSystem.
///
impl Extend<(HermitianFermionProduct, CalculatorComplex)> for FermionHamiltonianSystem {
    /// Extends the FermionHamiltonianSystem by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the FermionHamiltonianSystem.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn extend<I: IntoIterator<Item = (HermitianFermionProduct, CalculatorComplex)>>(
        &mut self,
        iter: I,
    ) {
        for (bp, cc) in iter {
            self.add_operator_product(bp, cc)
                .expect("Internal error in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of FermionHamiltonianSystem.
///
impl fmt::Display for FermionHamiltonianSystem {
    /// Formats the FermionHamiltonianSystem using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted FermionHamiltonianSystem.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("FermionHamiltonianSystem({}){{\n", self.number_modes());
        for (key, val) in self.iter() {
            writeln!(output, "{}: {},", key, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}

impl JordanWignerFermionToSpin for FermionHamiltonianSystem {
    type Output = SpinHamiltonianSystem;

    /// Implements JordanWignerFermionToSpin for a FermionHamiltonianSystem.
    ///
    /// The convention used is that |0> represents an empty fermionic state (spin-orbital),
    /// and |1> represents an occupied fermionic state.
    ///
    /// # Returns
    ///
    /// `SpinHamiltonianSystem` - The spin noise operator that results from the transformation.
    ///
    /// # Panics
    ///
    /// * Internal error in jordan_wigner transformation for FermionHamiltonian.
    fn jordan_wigner(&self) -> Self::Output {
        SpinHamiltonianSystem::from_hamiltonian(
            self.hamiltonian().jordan_wigner(),
            self.number_modes,
        )
        .expect("Internal bug in jordan_wigner for FermionHamiltonian. The number of spins in the resulting Hamiltonian should equal the number of modes of the FermionHamiltonian.")
    }
}
