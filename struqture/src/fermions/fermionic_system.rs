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

use super::{FermionOperator, OperateOnFermions};
use crate::fermions::FermionProduct;
use crate::mappings::JordanWignerFermionToSpin;
use crate::spins::SpinSystem;
use crate::{ModeIndex, OperateOnDensityMatrix, OperateOnModes, OperateOnState, StruqtureError};
use qoqo_calculator::CalculatorComplex;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Iter, Keys, Values};
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

/// FermionSystems are FermionOperators with a certain number of modes. When constructing it, the `new` function takes a `number_modes` input, and therefore
/// when the user adds a set of FermionProducts with specific CalculatorComplex coefficients, their indices must not exceed the number of modes in the FermionSystem.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::fermions::{OperateOnFermions, FermionProduct};
/// use struqture::fermions::FermionSystem;
/// let mut fo = FermionSystem::new(Some(2));
///
/// // Representing the opetator $ 1/2 f_0^{dagger} + 1/5 f_1 $
/// // Creating a FermionProduct with a creation operator acting on mode 0 and no annihilation operators
/// let fp_0 = FermionProduct::new([0],[]).unwrap();
/// // Creating a FermionProduct with a annihilation operator acting on mode 1 and no creation operators
/// let fp_1 = FermionProduct::new([],[1]).unwrap();
/// fo.set(fp_0.clone(), CalculatorComplex::from(0.5)).unwrap();
/// fo.set(fp_1.clone(), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(fo.get(&fp_0), &CalculatorComplex::from(0.5));
/// assert_eq!(fo.get(&fp_1), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
pub struct FermionSystem {
    /// The number of modes in the FermionSystem.
    pub(crate) number_modes: Option<usize>,
    /// The FermionOperator representing the operator of the FermionSystem.
    pub(crate) operator: FermionOperator,
}

impl crate::MinSupportedVersion for FermionSystem {}

impl<'a> OperateOnDensityMatrix<'a> for FermionSystem {
    type Index = FermionProduct;
    type Value = CalculatorComplex;
    type IteratorType = Iter<'a, Self::Index, Self::Value>;
    type KeyIteratorType = Keys<'a, Self::Index, Self::Value>;
    type ValueIteratorType = Values<'a, Self::Index, Self::Value>;

    // From trait
    fn get(&self, key: &FermionProduct) -> &CalculatorComplex {
        self.operator.get(key)
    }

    // From trait
    fn iter(&'a self) -> Self::IteratorType {
        self.operator.iter()
    }

    // From trait
    fn keys(&'a self) -> Self::KeyIteratorType {
        self.operator.keys()
    }

    // From trait
    fn values(&'a self) -> Self::ValueIteratorType {
        self.operator.values()
    }

    // From trait
    fn remove(&mut self, key: &Self::Index) -> Option<Self::Value> {
        self.operator.remove(key)
    }

    // From trait
    fn empty_clone(&self, capacity: Option<usize>) -> Self {
        match capacity {
            Some(cap) => Self {
                number_modes: self.number_modes,
                operator: FermionOperator::with_capacity(cap),
            },
            None => Self {
                number_modes: self.number_modes,
                operator: FermionOperator::new(),
            },
        }
    }

    /// Overwrites an existing entry or sets a new entry in the FermionOperator with the given (FermionProduct key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The FermionProduct key to set in the FermionOperator.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the FermionOperator.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(CalculatorComplex))` - The key existed, this is the value it had before it was set with the value input.
    /// * `Ok(None)` - The key did not exist, it has been set with its corresponding value.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of FermionProduct exceeds that of the FermionSystem.
    fn set(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<Option<Self::Value>, StruqtureError> {
        match self.number_modes {
            Some(x) => {
                if key.current_number_modes() <= x {
                    self.operator.set(key, value)
                } else {
                    Err(StruqtureError::NumberModesExceeded)
                }
            }
            None => self.operator.set(key, value),
        }
    }

    /// Adds a new (FermionProduct key, CalculatorComplex value) pair to the FermionSystem.
    ///
    /// # Arguments
    ///
    /// * `key` - The FermionProduct key to added to the FermionSystem.
    /// * `value` - The corresponding CalculatorComplex value to add for the key in the FermionSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The (key, value) pair was successfully added.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of FermionProduct exceeds that of the FermionSystem.
    fn add_operator_product(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<(), StruqtureError> {
        match self.number_modes {
            Some(x) => {
                if key.current_number_modes() <= x {
                    self.operator.add_operator_product(key, value)
                } else {
                    Err(StruqtureError::NumberModesExceeded)
                }
            }
            None => self.operator.add_operator_product(key, value),
        }
    }
}

impl<'a> OperateOnState<'a> for FermionSystem {}

impl<'a> OperateOnModes<'a> for FermionSystem {
    /// Gets the number_modes input of the FermionSystem, or the current_number_modes if number_modes is None.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of fermions in the FermionSystem.
    fn number_modes(&'a self) -> usize {
        match self.number_modes {
            Some(spins) => spins,
            None => self.operator.current_number_modes(),
        }
    }

    // From trait
    fn current_number_modes(&'a self) -> usize {
        self.operator.current_number_modes()
    }
}

impl<'a> OperateOnFermions<'a> for FermionSystem {}

/// Functions for the FermionSystem
///
impl FermionSystem {
    /// Creates a new FermionSystem.
    ///
    /// # Arguments
    ///
    /// * `number_modes` - The number of modes of the FermionSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) FermionSystem.
    pub fn new(number_modes: Option<usize>) -> Self {
        FermionSystem {
            number_modes,
            operator: FermionOperator::new(),
        }
    }

    /// Creates a new FermionSystem with pre-allocated capacity and given number of modes.
    ///
    /// # Arguments
    ///
    /// * `number_modes` - The number of modes of the system.
    /// * `capacity` - The pre-allocated capacity of the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) FermionSystem with the given number of modes and capacity.
    pub fn with_capacity(number_modes: Option<usize>, capacity: usize) -> Self {
        FermionSystem {
            number_modes,
            operator: FermionOperator::with_capacity(capacity),
        }
    }

    /// Returns the FermionOperator of the FermionSystem.
    ///
    /// # Returns
    ///
    /// * `&FermionOperator` - The FermionOperator of the FermionSystem.
    pub fn operator(&self) -> &FermionOperator {
        &self.operator
    }

    /// Creates a FermionSystem from a FermionOperator and an optional number of fermionic modes.
    ///
    /// # Arguments
    ///
    /// * `operator` - The FermionOperator to create the SpinSytem from.
    /// * `number_modes` - The optional number of fermionic modes for the FermionSystem to be created.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The FermionSystem created from the inputs.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Number of fermionic modes in entry exceeds number of fermionic modes in system.
    pub fn from_operator(
        operator: FermionOperator,
        number_modes: Option<usize>,
    ) -> Result<Self, StruqtureError> {
        match number_modes {
            Some(x) => {
                if operator.current_number_modes() <= x {
                    Ok(FermionSystem {
                        number_modes: Some(x),
                        operator,
                    })
                } else {
                    Err(StruqtureError::NumberModesExceeded)
                }
            }
            None => Ok(FermionSystem {
                number_modes: None,
                operator,
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

/// Implements the negative sign function of FermionSystem.
///
impl ops::Neg for FermionSystem {
    type Output = FermionSystem;
    /// Implement minus sign for FermionSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The FermionSystem * -1.
    fn neg(mut self) -> Self {
        self.operator = self.operator.neg();
        self
    }
}

/// Implements the plus function of FermionSystem by FermionSystem.
///
impl<T, V> ops::Add<T> for FermionSystem
where
    T: IntoIterator<Item = (FermionProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `+` (add) for two FermionSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The FermionSystem to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two FermionSystems added together.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of FermionProduct exceeds that of the FermionSystem.
    fn add(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value))?;
        }
        Ok(self)
    }
}

/// Implements the minus function of FermionSystem by FermionSystem.
///
impl<T, V> ops::Sub<T> for FermionSystem
where
    T: IntoIterator<Item = (FermionProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `-` (subtract) for two FermionSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The FermionSystem to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two FermionSystems subtracted.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of FermionProduct exceeds that of the FermionSystem.
    fn sub(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value) * -1.0)?;
        }
        Ok(self)
    }
}

/// Implements the multiplication function of FermionSystem by CalculatorComplex/CalculatorFloat.
///
impl<T> ops::Mul<T> for FermionSystem
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for FermionSystem and CalculatorComplex/CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex or CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The FermionSystem multiplied by the CalculatorComplex/CalculatorFloat.
    fn mul(mut self, other: T) -> Self {
        self.operator = self.operator * other;
        self
    }
}

/// Implements the multiplication function of FermionSystem by FermionSystem.
///
impl ops::Mul<FermionSystem> for FermionSystem {
    type Output = Self;
    /// Implement `*` for FermionSystem and FermionSystem.
    ///
    /// # Arguments
    ///
    /// * `other` - The FermionSystem to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - The two FermionSystems multiplied.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn mul(self, other: FermionSystem) -> Self {
        FermionSystem {
            number_modes: Some(self.number_modes().max(other.number_modes())),
            operator: self.operator * other.operator,
        }
    }
}

/// Implements the into_iter function (IntoIterator trait) of FermionSystem.
///
impl IntoIterator for FermionSystem {
    type Item = (FermionProduct, CalculatorComplex);
    type IntoIter = std::collections::hash_map::IntoIter<FermionProduct, CalculatorComplex>;
    /// Returns the FermionSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The FermionSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.operator.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference FermionSystem.
///
impl<'a> IntoIterator for &'a FermionSystem {
    type Item = (&'a FermionProduct, &'a CalculatorComplex);
    type IntoIter = Iter<'a, FermionProduct, CalculatorComplex>;

    /// Returns the reference FermionSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference FermionSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.operator.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of FermionSystem.
///
impl FromIterator<(FermionProduct, CalculatorComplex)> for FermionSystem {
    /// Returns the object in FermionSystem form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the FermionSystem.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in FermionSystem form.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from_iter<I: IntoIterator<Item = (FermionProduct, CalculatorComplex)>>(iter: I) -> Self {
        let mut so = FermionSystem::new(None);
        for (pp, cc) in iter {
            so.add_operator_product(pp.clone(), cc.clone())
                .expect("Internal error in add_operator_product");
        }
        so
    }
}

/// Implements the extend function (Extend trait) of FermionSystem.
///
impl Extend<(FermionProduct, CalculatorComplex)> for FermionSystem {
    /// Extends the FermionSystem by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the FermionSystem.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn extend<I: IntoIterator<Item = (FermionProduct, CalculatorComplex)>>(&mut self, iter: I) {
        for (fp, cc) in iter {
            self.add_operator_product(fp, cc)
                .expect("Internal error in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of FermionSystem.
///
impl fmt::Display for FermionSystem {
    /// Formats the FermionSystem using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted FermionSystem.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("FermionSystem({}){{\n", self.number_modes());
        for (key, val) in self.iter() {
            writeln!(output, "{}: {},", key, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}

impl JordanWignerFermionToSpin for FermionSystem {
    type Output = SpinSystem;

    /// Implements JordanWignerFermionToSpin for a FermionSystem.
    ///
    /// The convention used is that |0> represents an empty fermionic state (spin-orbital),
    /// and |1> represents an occupied fermionic state.
    ///
    /// # Returns
    ///
    /// `SpinSystem` - The spin noise operator that results from the transformation.
    /// # Panics
    ///
    /// * Internal error in jordan_wigner transformation for FermionHamiltonian.
    fn jordan_wigner(&self) -> Self::Output {
        SpinSystem::from_operator(
            self.operator().jordan_wigner(),
            Some(self.number_modes()),
        )
            .expect("Internal bug in jordan_wigner for FermionSystem. The number of spins in the resulting SpinSystem should equal the number of modes of the FermionSystem.")
    }
}
