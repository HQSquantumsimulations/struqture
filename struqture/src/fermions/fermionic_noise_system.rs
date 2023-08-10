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

use super::{FermionLindbladNoiseOperator, OperateOnFermions};
use crate::mappings::JordanWignerFermionToSpin;
use crate::spins::SpinLindbladNoiseSystem;
use crate::{ModeIndex, OperateOnDensityMatrix, OperateOnModes, StruqtureError};
use qoqo_calculator::CalculatorComplex;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Iter, Keys, Values};
use std::iter::{FromIterator, IntoIterator};
use std::{
    fmt::{self, Write},
    ops,
};

use super::FermionProduct;

/// FermionLindbladNoiseSystems are representations of systems of fermions, with a FermionLindbladNoiseOperator to represent the hamiltonian of the spin system, and an optional number of fermions.
///
/// In the Lindblad equation, Linblad noise operator L_i are not limited to [crate::fermions::FermionProduct] style operators.
/// We use ([crate::fermions::FermionProduct], [crate::fermions::FermionProduct]) as a unique basis.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::fermions::{OperateOnFermions, FermionProduct, FermionLindbladNoiseSystem};
///
/// let mut system = FermionLindbladNoiseSystem::new(Some(2));
///
/// let bp_0_1 = FermionProduct::new([0], [1]).unwrap();
/// let bp_0 = FermionProduct::new([], [0]).unwrap();
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
pub struct FermionLindbladNoiseSystem {
    /// The number of fermions in the FermionLindbladNoiseSystem.
    pub(crate) number_modes: Option<usize>,
    /// The FermionLindbladNoiseOperator representing the Lindblad noise terms of the FermionLindbladNoiseSystem.
    pub(crate) operator: FermionLindbladNoiseOperator,
}

impl crate::MinSupportedVersion for FermionLindbladNoiseSystem {}

impl<'a> OperateOnDensityMatrix<'a> for FermionLindbladNoiseSystem {
    type Value = CalculatorComplex;
    type Index = (FermionProduct, FermionProduct);
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
                operator: FermionLindbladNoiseOperator::with_capacity(cap),
            },
            None => Self {
                number_modes: self.number_modes,
                operator: FermionLindbladNoiseOperator::new(),
            },
        }
    }

    /// Overwrites an existing entry or sets a new entry in the FermionLindbladNoiseSystem with the given ((FermionProduct, FermionProduct) key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The (FermionProduct, FermionProduct) key to set in the FermionLindbladNoiseSystem.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the FermionLindbladNoiseSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(CalculatorComplex))` - The key existed, this is the value it had before it was set with the value input.
    /// * `Ok(None)` - The key did not exist, it has been set with its corresponding value.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of key exceeds that of the FermionLindbladNoiseSystem.
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

    /// Adds a new ((FermionProduct, FermionProduct) key, CalculatorComplex value) pair to the FermionLindbladNoiseSystem.
    ///
    /// # Arguments
    ///
    /// * `key` - The (FermionProduct, FermionProduct) key to added to the FermionLindbladNoiseSystem.
    /// * `value` - The corresponding CalculatorComplex value to add for the key in the FermionLindbladNoiseSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The (key, value) pair was successfully added.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of (FermionProduct, FermionProduct) exceeds that of the FermionLindbladNoiseSystem.
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

impl<'a> OperateOnModes<'a> for FermionLindbladNoiseSystem {
    // From trait
    fn current_number_modes(&'a self) -> usize {
        match self.number_modes {
            Some(modes) => modes,
            None => self.operator.current_number_modes(),
        }
    }

    /// Gets the number_modes input of the FermionLindbladNoiseSystem, or the current_number_modes if number_modes is None.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of fermions in the FermionLindbladNoiseSystem.
    fn number_modes(&'a self) -> usize {
        match self.number_modes {
            Some(modes) => modes,
            None => self.operator.current_number_modes(),
        }
    }
}

impl<'a> OperateOnFermions<'a> for FermionLindbladNoiseSystem {}

/// Functions for the FermionLindbladNoiseSystem.
///
impl FermionLindbladNoiseSystem {
    /// Creates a new FermionLindbladNoiseSystem.
    ///
    /// # Arguments
    ///
    /// * `number_modes` - The number of fermions in the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new FermionLindbladNoiseSystem with the input number of fermions.
    pub fn new(number_modes: Option<usize>) -> Self {
        FermionLindbladNoiseSystem {
            number_modes,
            operator: FermionLindbladNoiseOperator::new(),
        }
    }

    /// Creates a new FermionLindbladNoiseSystem with pre-allocated capacity.
    ///
    /// # Arguments
    ///
    /// * `number_modes` - The number of modes in the system.
    /// * `capacity` - The pre-allocated capacity of the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new FermionLindbladNoiseSystem with the input number of fermions.
    pub fn with_capacity(number_modes: Option<usize>, capacity: usize) -> Self {
        FermionLindbladNoiseSystem {
            number_modes,
            operator: FermionLindbladNoiseOperator::with_capacity(capacity),
        }
    }

    /// Returns the FermionLindbladNoiseOperator of the FermionLindbladNoiseSystem.
    ///
    /// # Returns
    ///
    /// * `&FermionLindbladNoiseOperator` - The FermionLindbladNoiseOperator of the FermionLindbladNoiseSystem.
    pub fn operator(&self) -> &FermionLindbladNoiseOperator {
        &self.operator
    }

    /// Creates a FermionLindbladNoiseSystem from a FermionLindbladNoiseOperator and an optional number of modes.
    ///
    /// # Arguments
    ///
    /// * `operator` - The FermionLindbladNoiseOperator to create the FermionSytem from.
    /// * `number_modes` - The optional number of modes for the FermionLindbladNoiseSystem to be created.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The FermionLindbladNoiseSystem created from the inputs.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Number of modes in entry exceeds number of modes in system.
    pub fn from_operator(
        operator: FermionLindbladNoiseOperator,
        number_modes: Option<usize>,
    ) -> Result<Self, StruqtureError> {
        match number_modes {
            Some(x) => {
                if operator.current_number_modes() <= x {
                    Ok(FermionLindbladNoiseSystem {
                        number_modes: Some(x),
                        operator,
                    })
                } else {
                    Err(StruqtureError::NumberModesExceeded)
                }
            }
            None => Ok(FermionLindbladNoiseSystem {
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
    /// `Ok((separated, remainder))` - Operator with the noise terms where number_creators_annihilators matches the number of spins the operator product acts on and Operator with all other contributions.
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

/// Implements the negative sign function of FermionLindbladNoiseSystem.
///
impl ops::Neg for FermionLindbladNoiseSystem {
    type Output = Self;
    /// Implement minus sign for FermionLindbladNoiseSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The FermionLindbladNoiseSystem * -1.
    fn neg(mut self) -> Self {
        self.operator = self.operator.neg();
        self
    }
}

/// Implements the plus function of FermionLindbladNoiseSystem by FermionLindbladNoiseSystem.
///
impl<T, V> ops::Add<T> for FermionLindbladNoiseSystem
where
    T: IntoIterator<Item = ((FermionProduct, FermionProduct), V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `+` (add) for two FermionLindbladNoiseSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The FermionLindbladNoiseSystem to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two FermionLindbladNoiseSystems added together.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of (FermionProduct, FermionProduct) exceeds that of the FermionLindbladNoiseSystem.
    fn add(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value))?;
        }
        Ok(self)
    }
}

/// Implements the minus function of FermionLindbladNoiseSystem by FermionLindbladNoiseSystem.
///
impl<T, V> ops::Sub<T> for FermionLindbladNoiseSystem
where
    T: IntoIterator<Item = ((FermionProduct, FermionProduct), V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `-` (subtract) for two FermionLindbladNoiseSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The FermionLindbladNoiseSystem to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two FermionLindbladNoiseSystems subtracted.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of (FermionProduct, FermionProduct) exceeds that of the FermionLindbladNoiseSystem.
    fn sub(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value) * -1.0)?;
        }
        Ok(self)
    }
}

/// Implements the multiplication function of FermionLindbladNoiseSystem by CalculatorFloat.
///
impl<T> ops::Mul<T> for FermionLindbladNoiseSystem
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for FermionLindbladNoiseSystem and CalculatorComplex.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The FermionLindbladNoiseSystem multiplied by the CalculatorComplex.
    fn mul(mut self, other: T) -> Self {
        self.operator = self.operator * other;
        self
    }
}

/// Implements the into_iter function (IntoIterator trait) of FermionLindbladNoiseSystem.
///
impl IntoIterator for FermionLindbladNoiseSystem {
    type Item = ((FermionProduct, FermionProduct), CalculatorComplex);
    type IntoIter =
        std::collections::hash_map::IntoIter<(FermionProduct, FermionProduct), CalculatorComplex>;
    /// Returns the FermionLindbladNoiseSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The FermionLindbladNoiseSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.operator.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference FermionLindbladNoiseSystem.
///
impl<'a> IntoIterator for &'a FermionLindbladNoiseSystem {
    type Item = (&'a (FermionProduct, FermionProduct), &'a CalculatorComplex);
    type IntoIter = Iter<'a, (FermionProduct, FermionProduct), CalculatorComplex>;

    /// Returns the FermionLindbladNoiseSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference FermionLindbladNoiseSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.operator.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of FermionLindbladNoiseSystem.
///
impl FromIterator<((FermionProduct, FermionProduct), CalculatorComplex)>
    for FermionLindbladNoiseSystem
{
    /// Returns the object in FermionLindbladNoiseSystem form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the FermionLindbladNoiseSystem.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in FermionLindbladNoiseSystem form.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from_iter<I: IntoIterator<Item = ((FermionProduct, FermionProduct), CalculatorComplex)>>(
        iter: I,
    ) -> Self {
        let mut so = FermionLindbladNoiseSystem::new(None);
        for (pp, cc) in iter {
            so.add_operator_product(pp.clone(), cc.clone())
                .expect("Internal error in add_operator_product");
        }
        so
    }
}

/// Implements the extend function (Extend trait) of FermionLindbladNoiseSystem.
///
impl Extend<((FermionProduct, FermionProduct), CalculatorComplex)> for FermionLindbladNoiseSystem {
    /// Extends the FermionLindbladNoiseSystem by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the FermionLindbladNoiseSystem.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn extend<I: IntoIterator<Item = ((FermionProduct, FermionProduct), CalculatorComplex)>>(
        &mut self,
        iter: I,
    ) {
        for (pp, cc) in iter {
            self.add_operator_product(pp, cc)
                .expect("Internal error in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of FermionLindbladNoiseSystem.
///
impl fmt::Display for FermionLindbladNoiseSystem {
    /// Formats the FermionLindbladNoiseSystem using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted FermionLindbladNoiseSystem.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("FermionLindbladNoiseSystem({}){{\n", self.number_modes());
        for (key, val) in self.iter() {
            writeln!(output, "({}, {}): {},", key.0, key.1, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}

impl JordanWignerFermionToSpin for FermionLindbladNoiseSystem {
    type Output = SpinLindbladNoiseSystem;

    /// Implements JordanWignerFermionToSpin for a FermionLindbladNoiseSystem.
    ///
    /// The convention used is that |0> represents an empty fermionic state (spin-orbital),
    /// and |1> represents an occupied fermionic state.
    ///
    /// # Returns
    ///
    /// `SpinLindbladNoiseSystem` - The spin noise operator that results from the transformation.
    ///
    /// # Panics
    ///
    /// * Internal error in jordan_wigner transformation for FermionLindbladNoiseOperator.
    fn jordan_wigner(&self) -> Self::Output {
        SpinLindbladNoiseSystem::from_operator(
            self.operator().jordan_wigner(),
            self.number_modes,
        )
            .expect("Internal bug in jordan_wigner for FermionLindbladNoiseOperator. The number of spins in the resulting SpinLindbladNoiseOperator should equal the number of modes of the FermionLindbladNoiseOperator.")
    }
}
