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

use super::{BosonLindbladNoiseOperator, OperateOnBosons};
use crate::{ModeIndex, OperateOnDensityMatrix, OperateOnModes, StruqtureError};
use qoqo_calculator::CalculatorComplex;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Iter, Keys, Values};
use std::iter::{FromIterator, IntoIterator};
use std::{
    fmt::{self, Write},
    ops,
};

use super::BosonProduct;

/// BosonLindbladNoiseSystems are BosonLindbladNoiseOperators with a certain number of modes. When constructing it, the `new` function takes a `number_modes` input, and therefore
/// when the user adds a set of (BosonProduct, BosonProduct) with specific CalculatorComplex coefficients, their indices must not exceed the number of modes in the BosonLindbladNoiseSystem.
///
/// In the Lindblad equation, Linblad noise operator L_i are not limited to [crate::bosons::BosonProduct] style operators.
/// We use ([crate::bosons::BosonProduct], [crate::bosons::BosonProduct]) as a unique basis.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::bosons::{OperateOnBosons, BosonProduct, BosonLindbladNoiseSystem};
///
/// let mut system = BosonLindbladNoiseSystem::new(Some(2));
///
/// let bp_0_1 = BosonProduct::new([0], [1]).unwrap();
/// let bp_0 = BosonProduct::new([], [0]).unwrap();
/// system.set((bp_0_1.clone(), bp_0_1.clone()), CalculatorComplex::from(0.5)).unwrap();
/// system.set((bp_0.clone(),bp_0.clone()), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(system.number_modes(), 2_usize);
/// assert_eq!(system.get(&(bp_0_1.clone(), bp_0_1.clone())), &CalculatorComplex::from(0.5));
/// assert_eq!(system.get(&(bp_0.clone(),bp_0.clone())), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
pub struct BosonLindbladNoiseSystem {
    /// The number of modes in the BosonLindbladNoiseSystem.
    pub(crate) number_modes: Option<usize>,
    /// The BosonLindbladNoiseOperator representing the Lindblad terms of the BosonLindbladNoiseSystem.
    pub(crate) operator: BosonLindbladNoiseOperator,
}

impl crate::MinSupportedVersion for BosonLindbladNoiseSystem {}

impl<'a> OperateOnDensityMatrix<'a> for BosonLindbladNoiseSystem {
    type Value = CalculatorComplex;
    type Index = (BosonProduct, BosonProduct);
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
                operator: BosonLindbladNoiseOperator::with_capacity(cap),
            },
            None => Self {
                number_modes: self.number_modes,
                operator: BosonLindbladNoiseOperator::new(),
            },
        }
    }

    /// Overwrites an existing entry or sets a new entry in the BosonLindbladNoiseSystem with the given ((BosonProduct, BosonProduct) key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The (BosonProduct, BosonProduct) key to set in the BosonLindbladNoiseSystem.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the BosonLindbladNoiseSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(CalculatorComplex))` - The key existed, this is the value it had before it was set with the value input.
    /// * `Ok(None)` - The key did not exist, it has been set with its corresponding value.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of (BosonProduct, BosonProduct) exceeds that of the BosonLindbladNoiseSystem.
    fn set(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<Option<Self::Value>, StruqtureError> {
        match self.number_modes {
            Some(x) => {
                if key.0.current_number_modes() <= x && key.1.current_number_modes() <= x {
                    self.operator.set(key, value)
                } else {
                    Err(StruqtureError::NumberModesExceeded)
                }
            }
            None => self.operator.set(key, value),
        }
    }

    /// Adds a new ((BosonProduct, BosonProduct) key, CalculatorComplex value) pair to the BosonLindbladNoiseSystem.
    ///
    /// # Arguments
    ///
    /// * `key` - The (BosonProduct, BosonProduct) key to added to the BosonLindbladNoiseSystem.
    /// * `value` - The corresponding CalculatorComplex value to add for the key in the BosonLindbladNoiseSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The (key, value) pair was successfully added.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of (BosonProduct, BosonProduct) exceeds that of the BosonLindbladNoiseSystem.
    fn add_operator_product(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<(), StruqtureError> {
        match self.number_modes {
            Some(x) => {
                if key.0.current_number_modes() <= x && key.1.current_number_modes() <= x {
                    self.operator.add_operator_product(key, value)
                } else {
                    Err(StruqtureError::NumberModesExceeded)
                }
            }
            None => self.operator.add_operator_product(key, value),
        }
    }
}

impl<'a> OperateOnModes<'a> for BosonLindbladNoiseSystem {
    // From trait
    fn current_number_modes(&'a self) -> usize {
        match self.number_modes {
            Some(modes) => modes,
            None => self.operator.current_number_modes(),
        }
    }

    /// Gets the number_modes input of the BosonLindbladNoiseSystem, or the current_number_modes if number_modes is None.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of bosons in the BosonLindbladNoiseSystem.
    fn number_modes(&'a self) -> usize {
        match self.number_modes {
            Some(modes) => modes,
            None => self.operator.current_number_modes(),
        }
    }
}

impl<'a> OperateOnBosons<'a> for BosonLindbladNoiseSystem {}

/// Functions for the BosonLindbladNoiseSystem.
///
impl BosonLindbladNoiseSystem {
    /// Creates a new BosonLindbladNoiseSystem.
    ///
    /// # Arguments
    ///
    /// * `number_modes` - The number of modes in the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new BosonLindbladNoiseSystem with the input number of modes.
    pub fn new(number_modes: Option<usize>) -> Self {
        BosonLindbladNoiseSystem {
            number_modes,
            operator: BosonLindbladNoiseOperator::new(),
        }
    }

    /// Creates a new BosonLindbladNoiseSystem with pre-allocated capacity and given number of modes.
    ///
    /// # Arguments
    ///
    /// * `number_modes` - The number of modes in the system.
    /// * `capacity` - The pre-allocated capacity of the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new BosonLindbladNoiseSystem with the input number of modes and capacity.
    pub fn with_capacity(number_modes: Option<usize>, capacity: usize) -> Self {
        BosonLindbladNoiseSystem {
            number_modes,
            operator: BosonLindbladNoiseOperator::with_capacity(capacity),
        }
    }

    /// Returns the BosonLindbladNoiseOperator of the BosonLindbladNoiseSystem.
    ///
    /// # Returns
    ///
    /// * `&BosonLindbladNoiseOperator` - The BosonLindbladNoiseOperator of the BosonLindbladNoiseSystem.
    pub fn operator(&self) -> &BosonLindbladNoiseOperator {
        &self.operator
    }

    /// Creates a BosonLindbladNoiseSystem from a BosonLindbladNoiseOperator and an optional number of modes.
    ///
    /// # Arguments
    ///
    /// * `operator` - The BosonLindbladNoiseOperator to create the BosonLindbladNoiseSystem from.
    /// * `number_modes` - The optional number of modes for the BosonLindbladNoiseSystem to be created.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The BosonLindbladNoiseSystem created from the inputs.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Number of modes in entry exceeds number of modes in system.
    pub fn from_operator(
        operator: BosonLindbladNoiseOperator,
        number_modes: Option<usize>,
    ) -> Result<Self, StruqtureError> {
        match number_modes {
            Some(x) => {
                if operator.current_number_modes() <= x {
                    Ok(BosonLindbladNoiseSystem {
                        number_modes: Some(x),
                        operator,
                    })
                } else {
                    Err(StruqtureError::NumberModesExceeded)
                }
            }
            None => Ok(BosonLindbladNoiseSystem {
                number_modes: None,
                operator,
            }),
        }
    }

    /// Separate self into an operator with the terms of given number of creation and annihilation operators and an operator with the remaining operations
    ///
    /// # Arguments
    ///
    /// * `number_creators_annihilators_left` - Number of creators and number of annihilators to filter for in the left term of the keys.
    /// * `number_creators_annihilators_right` - Number of creators and number of annihilators to filter for in the right term of the keys.
    ///
    /// # Returns
    ///
    /// `Ok((separated, remainder))` - Operator with the noise terms where the number of creation and annihilation operators matches the number of spins the operator product acts on and Operator with all other contributions.
    pub fn separate_into_n_terms(
        &self,
        number_creators_annihilators_left: (usize, usize),
        number_creators_annihilators_right: (usize, usize),
    ) -> Result<(Self, Self), StruqtureError> {
        let mut separated = Self::default();
        let mut remainder = Self::default();
        for ((prod_l, prod_r), val) in self.iter() {
            if (prod_l.creators().len(), prod_l.annihilators().len())
                == number_creators_annihilators_left
                && (prod_r.creators().len(), prod_r.annihilators().len())
                    == number_creators_annihilators_right
            {
                separated.add_operator_product((prod_l.clone(), prod_r.clone()), val.clone())?;
            } else {
                remainder.add_operator_product((prod_l.clone(), prod_r.clone()), val.clone())?;
            }
        }
        Ok((separated, remainder))
    }
}

/// Implements the negative sign function of BosonLindbladNoiseSystem.
///
impl ops::Neg for BosonLindbladNoiseSystem {
    type Output = Self;
    /// Implement minus sign for BosonLindbladNoiseSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The BosonLindbladNoiseSystem * -1.
    fn neg(mut self) -> Self {
        self.operator = self.operator.neg();
        self
    }
}

/// Implements the plus function of BosonLindbladNoiseSystem by BosonLindbladNoiseSystem.
///
impl<T, V> ops::Add<T> for BosonLindbladNoiseSystem
where
    T: IntoIterator<Item = ((BosonProduct, BosonProduct), V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `+` (add) for two BosonLindbladNoiseSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The BosonLindbladNoiseSystem to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two BosonLindbladNoiseSystems added together.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of (BosonProduct, BosonProduct) exceeds that of the BosonLindbladNoiseSystem.
    fn add(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value))?;
        }
        Ok(self)
    }
}

/// Implements the minus function of BosonLindbladNoiseSystem by BosonLindbladNoiseSystem.
///
impl<T, V> ops::Sub<T> for BosonLindbladNoiseSystem
where
    T: IntoIterator<Item = ((BosonProduct, BosonProduct), V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `-` (subtract) for two BosonLindbladNoiseSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The BosonLindbladNoiseSystem to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two BosonLindbladNoiseSystems subtracted.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of (BosonProduct, BosonProduct) exceeds that of the BosonLindbladNoiseSystem.
    fn sub(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value) * -1.0)?;
        }
        Ok(self)
    }
}

/// Implements the multiplication function of BosonLindbladNoiseSystem by CalculatorFloat.
///
impl<T> ops::Mul<T> for BosonLindbladNoiseSystem
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for BosonLindbladNoiseSystem and CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The BosonLindbladNoiseSystem multiplied by the CalculatorFloat.
    fn mul(mut self, other: T) -> Self {
        self.operator = self.operator * other;
        self
    }
}

/// Implements the into_iter function (IntoIterator trait) of BosonLindbladNoiseSystem.
///
impl IntoIterator for BosonLindbladNoiseSystem {
    type Item = ((BosonProduct, BosonProduct), CalculatorComplex);
    type IntoIter =
        std::collections::hash_map::IntoIter<(BosonProduct, BosonProduct), CalculatorComplex>;
    /// Returns the BosonLindbladNoiseSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The BosonLindbladNoiseSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.operator.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference BosonLindbladNoiseSystem.
///
impl<'a> IntoIterator for &'a BosonLindbladNoiseSystem {
    type Item = (&'a (BosonProduct, BosonProduct), &'a CalculatorComplex);
    type IntoIter = Iter<'a, (BosonProduct, BosonProduct), CalculatorComplex>;

    /// Returns the BosonLindbladNoiseSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference BosonLindbladNoiseSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.operator.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of BosonLindbladNoiseSystem.
///
impl FromIterator<((BosonProduct, BosonProduct), CalculatorComplex)> for BosonLindbladNoiseSystem {
    /// Returns the object in BosonLindbladNoiseSystem form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the BosonLindbladNoiseSystem.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in BosonLindbladNoiseSystem form.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from_iter<I: IntoIterator<Item = ((BosonProduct, BosonProduct), CalculatorComplex)>>(
        iter: I,
    ) -> Self {
        let mut so = BosonLindbladNoiseSystem::new(None);
        for (pp, cc) in iter {
            so.add_operator_product(pp.clone(), cc.clone())
                .expect("Internal error in add_operator_product");
        }
        so
    }
}

/// Implements the extend function (Extend trait) of BosonLindbladNoiseSystem.
///
impl Extend<((BosonProduct, BosonProduct), CalculatorComplex)> for BosonLindbladNoiseSystem {
    /// Extends the BosonLindbladNoiseSystem by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the BosonLindbladNoiseSystem.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn extend<I: IntoIterator<Item = ((BosonProduct, BosonProduct), CalculatorComplex)>>(
        &mut self,
        iter: I,
    ) {
        for (pp, cc) in iter {
            self.add_operator_product(pp, cc)
                .expect("Internal error in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of BosonLindbladNoiseSystem.
///
impl fmt::Display for BosonLindbladNoiseSystem {
    /// Formats the BosonLindbladNoiseSystem using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted BosonLindbladNoiseSystem.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("BosonLindbladNoiseSystem({}){{\n", self.number_modes());
        for (key, val) in self.iter() {
            writeln!(output, "({}, {}): {},", key.0, key.1, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}
