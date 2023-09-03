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

use super::{BosonOperator, OperateOnBosons};
use crate::bosons::BosonProduct;
use crate::{ModeIndex, OperateOnDensityMatrix, OperateOnModes, OperateOnState, StruqtureError};
use qoqo_calculator::CalculatorComplex;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Iter, Keys, Values};
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

/// BosonSystems are BosonOperators with a certain number of modes. When constructing it, the `new` function takes a `number_modes` input, and therefore
/// when the user adds a set of BosonProducts with specific CalculatorComplex coefficients, their indices must not exceed the number of modes in the BosonSystem.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::bosons::{OperateOnBosons, BosonProduct};
/// use struqture::bosons::BosonSystem;
/// let mut bo = BosonSystem::new(Some(2));
///
/// // Representing the opetator $ 1/2 b_0^{dagger} + 1/5 b_1 $
/// // Creating a BosonProduct with a creation operator acting on mode 0 and no annihilation operators
/// let bp_0 = BosonProduct::new([0],[]).unwrap();
/// // Creating a BosonProduct with a annihilation operator acting on mode 1 and no creation operators
/// let bp_1 = BosonProduct::new([],[1]).unwrap();
/// bo.set(bp_0.clone(), CalculatorComplex::from(0.5)).unwrap();
/// bo.set(bp_1.clone(), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(bo.get(&bp_0), &CalculatorComplex::from(0.5));
/// assert_eq!(bo.get(&bp_1), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
pub struct BosonSystem {
    /// The number of modes in the BosonSystem.
    pub(crate) number_modes: Option<usize>,
    /// The BosonOperator representing the operator of the BosonSystem.
    pub(crate) operator: BosonOperator,
}

impl crate::MinSupportedVersion for BosonSystem {}

impl<'a> OperateOnDensityMatrix<'a> for BosonSystem {
    type Index = BosonProduct;

    type Value = CalculatorComplex;
    type IteratorType = Iter<'a, Self::Index, Self::Value>;
    type KeyIteratorType = Keys<'a, Self::Index, Self::Value>;
    type ValueIteratorType = Values<'a, Self::Index, Self::Value>;

    // From trait
    fn get(&self, key: &Self::Index) -> &Self::Value {
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
                operator: BosonOperator::with_capacity(cap),
            },
            None => Self {
                number_modes: self.number_modes,
                operator: BosonOperator::new(),
            },
        }
    }

    /// Overwrites an existing entry or sets a new entry in the BosonSystem with the given (BosonProduct key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The BosonProduct key to set in the BosonSystem.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the BosonSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(CalculatorComplex))` - The key existed, this is the value it had before it was set with the value input.
    /// * `Ok(None)` - The key did not exist, it has been set with its corresponding value.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of BosonProduct exceeds that of the BosonSystem.
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

    /// Adds a new (BosonProduct key, CalculatorComplex value) pair to the BosonSystem.
    ///
    /// # Arguments
    ///
    /// * `key` - The BosonProduct key to added to the BosonSystem.
    /// * `value` - The corresponding CalculatorComplex value to add for the key in the BosonSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The (key, value) pair was successfully added.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of BosonProduct exceeds that of the BosonSystem.
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

impl<'a> OperateOnState<'a> for BosonSystem {}

impl<'a> OperateOnModes<'a> for BosonSystem {
    /// Gets the number_modes input of the BosonSystem, or the current_number_modes if number_modes is None.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of bosons in the BosonSystem.
    fn number_modes(&'a self) -> usize {
        match self.number_modes {
            Some(modes) => modes,
            None => self.operator.current_number_modes(),
        }
    }

    // From trait
    fn current_number_modes(&'a self) -> usize {
        self.operator.current_number_modes()
    }
}

impl<'a> OperateOnBosons<'a> for BosonSystem {}

/// Functions for the BosonSystem
///
impl BosonSystem {
    /// Creates a new BosonSystem.
    ///
    /// # Arguments
    ///
    /// * `number_modes` - The number of modes of the BosonSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) BosonSystem with the given number of modes.
    pub fn new(number_modes: Option<usize>) -> Self {
        BosonSystem {
            number_modes,
            operator: BosonOperator::new(),
        }
    }

    /// Creates a new BosonSystem with pre-allocated capacity and given number of modes.
    ///
    /// # Arguments
    ///
    /// * `number_modes` - The number of modes of the system.
    /// * `capacity` - The pre-allocated capacity of the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) BosonSystem with the given number of modes and capacity.
    pub fn with_capacity(number_modes: Option<usize>, capacity: usize) -> Self {
        BosonSystem {
            number_modes,
            operator: BosonOperator::with_capacity(capacity),
        }
    }

    /// Returns the BosonOperator of the BosonSystem.
    ///
    /// # Returns
    ///
    /// * `&BosonOperator` - The BosonOperator of the BosonSystem.
    pub fn operator(&self) -> &BosonOperator {
        &self.operator
    }

    /// Creates a BosonSystem from a BosonOperator and an optional number of bosonic modes.
    ///
    /// # Arguments
    ///
    /// * `operator` - The BosonOperator to create the BosonSystem from.
    /// * `number_modes` - The optional number of bosonic modes for the BosonSystem to be created.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The BosonSystem created from the inputs.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Number of bosonic modes in entry exceeds number of bosonic modes in system.
    pub fn from_operator(
        operator: BosonOperator,
        number_modes: Option<usize>,
    ) -> Result<Self, StruqtureError> {
        match number_modes {
            Some(x) => {
                if operator.current_number_modes() <= x {
                    Ok(BosonSystem {
                        number_modes: Some(x),
                        operator,
                    })
                } else {
                    Err(StruqtureError::NumberModesExceeded)
                }
            }
            None => Ok(BosonSystem {
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

/// Implements the negative sign function of BosonSystem.
///
impl ops::Neg for BosonSystem {
    type Output = BosonSystem;
    /// Implement minus sign for BosonSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The BosonSystem * -1.
    fn neg(mut self) -> Self {
        self.operator = self.operator.neg();
        self
    }
}

/// Implements the plus function of BosonSystem by BosonSystem.
///
impl<T, V> ops::Add<T> for BosonSystem
where
    T: IntoIterator<Item = (BosonProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `+` (add) for two BosonSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The BosonSystem to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two BosonSystems added together.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of BosonProduct exceeds that of the BosonSystem.
    fn add(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value))?;
        }
        Ok(self)
    }
}

/// Implements the minus function of BosonSystem by BosonSystem.
///
impl<T, V> ops::Sub<T> for BosonSystem
where
    T: IntoIterator<Item = (BosonProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `-` (subtract) for two BosonSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The BosonSystem to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two BosonSystems subtracted together.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of BosonProduct exceeds that of the BosonSystem.
    fn sub(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value) * -1.0)?;
        }
        Ok(self)
    }
}

/// Implements the multiplication function of BosonSystem by CalculatorComplex/CalculatorFloat.
///
impl<T> ops::Mul<T> for BosonSystem
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for BosonSystem and CalculatorComplex/CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex or CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The BosonSystem multiplied by the CalculatorComplex/CalculatorFloat.
    fn mul(mut self, other: T) -> Self {
        self.operator = self.operator * other;
        self
    }
}

/// Implements the multiplication function of BosonSystem by BosonSystem.
///
impl ops::Mul<BosonSystem> for BosonSystem {
    type Output = Self;
    /// Implement `*` for BosonSystem and BosonSystem.
    ///
    /// # Arguments
    ///
    /// * `other` - The BosonSystem to multiply self by.
    ///
    /// # Returns
    ///
    /// * `Self` - The two BosonSystems multiplied.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn mul(self, other: BosonSystem) -> Self {
        BosonSystem {
            number_modes: Some(self.number_modes().max(other.number_modes())),
            operator: self.operator * other.operator,
        }
    }
}

/// Implements the into_iter function (IntoIterator trait) of BosonSystem.
///
impl IntoIterator for BosonSystem {
    type Item = (BosonProduct, CalculatorComplex);
    type IntoIter = std::collections::hash_map::IntoIter<BosonProduct, CalculatorComplex>;
    /// Returns the BosonSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The BosonSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.operator.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference BosonSystem.
///
impl<'a> IntoIterator for &'a BosonSystem {
    type Item = (&'a BosonProduct, &'a CalculatorComplex);
    type IntoIter = Iter<'a, BosonProduct, CalculatorComplex>;

    /// Returns the reference BosonSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference BosonSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.operator.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of BosonSystem.
///
impl FromIterator<(BosonProduct, CalculatorComplex)> for BosonSystem {
    /// Returns the object in BosonSystem form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the BosonSystem.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in BosonSystem form.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from_iter<I: IntoIterator<Item = (BosonProduct, CalculatorComplex)>>(iter: I) -> Self {
        let mut so = BosonSystem::new(None);
        for (pp, cc) in iter {
            so.add_operator_product(pp.clone(), cc.clone())
                .expect("Internal error in add_operator_product");
        }
        so
    }
}

/// Implements the extend function (Extend trait) of BosonSystem.
///
impl Extend<(BosonProduct, CalculatorComplex)> for BosonSystem {
    /// Extends the BosonSystem by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the BosonSystem.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn extend<I: IntoIterator<Item = (BosonProduct, CalculatorComplex)>>(&mut self, iter: I) {
        for (bp, cc) in iter {
            self.add_operator_product(bp, cc)
                .expect("Internal error in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of BosonSystem.
///
impl fmt::Display for BosonSystem {
    /// Formats the BosonSystem using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted BosonSystem.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("BosonSystem({}){{\n", self.number_modes());
        for (key, val) in self.iter() {
            writeln!(output, "{}: {},", key, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}
