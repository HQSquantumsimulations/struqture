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

use super::{BosonHamiltonian, BosonSystem, HermitianBosonProduct, ModeIndex, OperateOnBosons};
use crate::{OperateOnDensityMatrix, OperateOnModes, OperateOnState, StruqtureError};
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Iter, Keys, Values};
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

/// BosonHamiltonianSystems are BosonHamiltonians with a certain number of modes. When constructing it, the `new` function takes a `number_modes` input, and therefore
/// when the user adds a set of HermitianBosonProducts with specific CalculatorComplex coefficients, their indices must not exceed the number of modes in the BosonHamiltonianSystem.
///
/// This is a representation of sums of creation and annihilation operators with weightings, in order to build a full hamiltonian.
/// BosonHamiltonianSystem is the hermitian equivalent of BosonSystem.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::bosons::{ HermitianBosonProduct, BosonHamiltonianSystem};
/// use struqture::prelude::*;
///
/// let mut sh = BosonHamiltonianSystem::new(Some(2));
///
/// let bp_0 = HermitianBosonProduct::new([0], [1]).unwrap();
/// let bp_1 = HermitianBosonProduct::new([], [0]).unwrap();
/// sh.set(bp_0.clone(), CalculatorComplex::from(0.5)).unwrap();
/// sh.set(bp_1.clone(), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(sh.get(&bp_0), &CalculatorComplex::from(0.5));
/// assert_eq!(sh.get(&bp_1), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
pub struct BosonHamiltonianSystem {
    /// The number of modes in the BosonHamiltonianSystem
    pub(crate) number_modes: Option<usize>,
    /// The BosonHamiltonian representing the Hamiltonian of the BosonHamiltonianSystem
    pub(crate) hamiltonian: BosonHamiltonian,
}

impl crate::MinSupportedVersion for BosonHamiltonianSystem {}

impl<'a> OperateOnDensityMatrix<'a> for BosonHamiltonianSystem {
    type Index = HermitianBosonProduct;
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
                number_modes: self.number_modes,
                hamiltonian: BosonHamiltonian::with_capacity(cap),
            },
            None => Self {
                number_modes: self.number_modes,
                hamiltonian: BosonHamiltonian::new(),
            },
        }
    }

    /// Overwrites an existing entry or sets a new entry in the BosonHamiltonianSystem with the given (HermitianBosonProduct key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The HermitianBosonProduct key to set in the BosonHamiltonianSystem.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the BosonHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(CalculatorComplex))` - The key existed, this is the value it had before it was set with the value input.
    /// * `Ok(None)` - The key did not exist, it has been set with its corresponding value.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of HermitianBosonProduct exceeds that of the BosonHamiltonianSystem.
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

    /// Adds a new (HermitianBosonProduct key, CalculatorComplex value) pair to the BosonHamiltonianSystem.
    ///
    /// # Arguments
    ///
    /// * `key` - The HermitianBosonProduct key to added to the BosonHamiltonianSystem.
    /// * `value` - The corresponding CalculatorComplex value to add for the key in the BosonHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The (key, value) pair was successfully added.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of HermitianBosonProduct exceeds that of the BosonHamiltonianSystem.
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

impl<'a> OperateOnState<'a> for BosonHamiltonianSystem {
    // From trait
    fn hermitian_conjugate(&self) -> Self {
        self.clone()
    }
}

impl<'a> OperateOnModes<'a> for BosonHamiltonianSystem {
    /// Gets the number_modes input of the BosonHamiltonianSystem, or the current_number_modes if number_modes is None.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of modes in the BosonHamiltonianSystem.
    fn number_modes(&self) -> usize {
        match self.number_modes {
            Some(modes) => modes,
            None => self.hamiltonian.current_number_modes(),
        }
    }

    /// Return maximum index in BosonHamiltonianSystem hamiltonian's internal_map.
    ///
    /// # Returns
    ///
    /// * `usize` - Maximum index.
    fn current_number_modes(&self) -> usize {
        self.hamiltonian.current_number_modes()
    }
}

impl<'a> OperateOnBosons<'a> for BosonHamiltonianSystem {}

/// Functions for the BosonHamiltonianSystem
///
impl BosonHamiltonianSystem {
    /// Creates a new BosonHamiltonianSystem.
    ///
    /// # Arguments
    ///
    /// * `number_modes` - The number of modes of the BosonHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) BosonHamiltonianSystem with the given number of modes.
    pub fn new(number_modes: Option<usize>) -> Self {
        BosonHamiltonianSystem {
            number_modes,
            hamiltonian: BosonHamiltonian::new(),
        }
    }

    /// Creates a new BosonHamiltonianSystem with pre-allocated capacity.
    ///
    /// # Arguments
    ///
    /// * `number_modes` - The number of modes of the system.
    /// * `capacity` - The pre-allocated capacity of the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) BosonHamiltonianSystem with the given number of modes and capacity.
    pub fn with_capacity(number_modes: Option<usize>, capacity: usize) -> Self {
        BosonHamiltonianSystem {
            number_modes,
            hamiltonian: BosonHamiltonian::with_capacity(capacity),
        }
    }

    /// Returns the BosonHamiltonian of the BosonHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `&BosonHamiltonian` - The BosonHamiltonian of the BosonHamiltonianSystem.
    pub fn hamiltonian(&self) -> &BosonHamiltonian {
        &self.hamiltonian
    }

    /// Creates a BosonHamiltonianSystem from a BosonHamiltonian and an optional number of bosonic modes.
    ///
    /// # Arguments
    ///
    /// * `operator` - The BosonHamiltonian to create the BosonHamiltonianSystem from.
    /// * `number_modes` - The optional number of bosonic modes for the BosonHamiltonianSystem to be created.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The BosonHamiltonianSystem created from the inputs.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Number of bosonic modes in entry exceeds number of bosonic modes in system.
    pub fn from_hamiltonian(
        hamiltonian: BosonHamiltonian,
        number_modes: Option<usize>,
    ) -> Result<Self, StruqtureError> {
        match number_modes {
            Some(x) => {
                if hamiltonian.current_number_modes() <= x {
                    Ok(BosonHamiltonianSystem {
                        number_modes: Some(x),
                        hamiltonian,
                    })
                } else {
                    Err(StruqtureError::NumberModesExceeded)
                }
            }
            None => Ok(BosonHamiltonianSystem {
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

/// Implements the negative sign function of BosonHamiltonianSystem.
///
impl ops::Neg for BosonHamiltonianSystem {
    type Output = Self;
    /// Implement minus sign for BosonHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The BosonHamiltonianSystem * -1.
    fn neg(mut self) -> Self {
        self.hamiltonian = self.hamiltonian.neg();
        self
    }
}

/// Implements the plus function of BosonHamiltonianSystem by BosonHamiltonianSystem.
///
impl<T, V> ops::Add<T> for BosonHamiltonianSystem
where
    T: IntoIterator<Item = (HermitianBosonProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `+` (add) for two BosonHamiltonianSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The BosonHamiltonianSystem to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two BosonHamiltonianSystems added together.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of HermitianBosonProduct exceeds that of the BosonHamiltonianSystem.
    fn add(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value))?;
        }
        Ok(self)
    }
}

/// Implements the minus function of BosonHamiltonianSystem by BosonHamiltonianSystem.
///
impl<T, V> ops::Sub<T> for BosonHamiltonianSystem
where
    T: IntoIterator<Item = (HermitianBosonProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `-` (subtract) for two BosonHamiltonianSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The BosonHamiltonianSystem to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two BosonHamiltonianSystems added together.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of HermitianBosonProduct exceeds that of the BosonHamiltonianSystem.
    fn sub(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value) * -1.0)?;
        }
        Ok(self)
    }
}

/// Implements the multiplication function of BosonHamiltonianSystem by CalculatorFloat.
///
impl ops::Mul<CalculatorFloat> for BosonHamiltonianSystem {
    type Output = Self;
    /// Implement `*` for BosonHamiltonianSystem and CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The BosonHamiltonianSystem multiplied by the CalculatorFloat.
    fn mul(mut self, other: CalculatorFloat) -> Self {
        self.hamiltonian = self.hamiltonian * other;
        self
    }
}

/// Implements the multiplication function of BosonHamiltonianSystem by CalculatorComplex.
///
impl ops::Mul<CalculatorComplex> for BosonHamiltonianSystem {
    type Output = BosonSystem;
    /// Implement `*` for BosonHamiltonianSystem and CalculatorComplex.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex by which to multiply.
    ///
    /// # Returns
    ///
    /// * `BosonSystem` - The BosonHamiltonianSystem multiplied by the CalculatorComplex.
    fn mul(self, other: CalculatorComplex) -> BosonSystem {
        let mut system = BosonSystem::new(self.number_modes);
        system.operator = self.hamiltonian * other;
        system
    }
}

/// Implements the multiplication function of BosonHamiltonianSystem by BosonHamiltonianSystem.
///
impl ops::Mul<BosonHamiltonianSystem> for BosonHamiltonianSystem {
    type Output = Result<BosonSystem, StruqtureError>;
    /// Implement `*` for BosonHamiltonianSystem and BosonHamiltonianSystem.
    ///
    /// # Arguments
    ///
    /// * `other` - The BosonHamiltonianSystem to multiply by.
    ///
    /// # Returns
    ///
    /// * `Ok(BosonSystem)` - The two BosonHamiltonianSystems multiplied.
    fn mul(self, other: BosonHamiltonianSystem) -> Self::Output {
        Ok(BosonSystem {
            number_modes: Some(self.number_modes().max(other.number_modes())),
            operator: self.hamiltonian * other.hamiltonian,
        })
    }
}

/// Implements the into_iter function (IntoIterator trait) of BosonHamiltonianSystem.
///
impl IntoIterator for BosonHamiltonianSystem {
    type Item = (HermitianBosonProduct, CalculatorComplex);
    type IntoIter = std::collections::hash_map::IntoIter<HermitianBosonProduct, CalculatorComplex>;
    /// Returns the BosonHamiltonianSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The BosonHamiltonianSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.hamiltonian.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference BosonHamiltonianSystem.
///
impl<'a> IntoIterator for &'a BosonHamiltonianSystem {
    type Item = (&'a HermitianBosonProduct, &'a CalculatorComplex);
    type IntoIter = Iter<'a, HermitianBosonProduct, CalculatorComplex>;

    /// Returns the reference BosonHamiltonianSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference BosonHamiltonianSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.hamiltonian.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of BosonHamiltonianSystem.
///
impl FromIterator<(HermitianBosonProduct, CalculatorComplex)> for BosonHamiltonianSystem {
    /// Returns the object in BosonHamiltonianSystem form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the BosonHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in BosonHamiltonianSystem form.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from_iter<I: IntoIterator<Item = (HermitianBosonProduct, CalculatorComplex)>>(
        iter: I,
    ) -> Self {
        let mut bhs = BosonHamiltonianSystem::new(None);
        for (bp, cc) in iter {
            bhs.add_operator_product(bp.clone(), cc.clone())
                .expect("Internal error in add_operator_product");
        }
        bhs
    }
}

/// Implements the extend function (Extend trait) of BosonHamiltonianSystem.
///
impl Extend<(HermitianBosonProduct, CalculatorComplex)> for BosonHamiltonianSystem {
    /// Extends the BosonHamiltonianSystem by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the BosonHamiltonianSystem.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn extend<I: IntoIterator<Item = (HermitianBosonProduct, CalculatorComplex)>>(
        &mut self,
        iter: I,
    ) {
        for (bp, cc) in iter {
            self.add_operator_product(bp, cc)
                .expect("Internal error in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of BosonHamiltonianSystem.
///
impl fmt::Display for BosonHamiltonianSystem {
    /// Formats the BosonHamiltonianSystem using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted BosonHamiltonianSystem.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("BosonHamiltonianSystem({}){{\n", self.number_modes());
        for (key, val) in self.iter() {
            writeln!(output, "{}: {},", key, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}
