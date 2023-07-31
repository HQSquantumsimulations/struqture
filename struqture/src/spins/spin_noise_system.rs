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

use super::{DecoherenceProduct, ToSparseMatrixSuperOperator};
use crate::fermions::FermionLindbladNoiseSystem;
use crate::mappings::JordanWignerSpinToFermion;
use crate::spins::{OperateOnSpins, SpinIndex, SpinLindbladNoiseOperator};
use crate::{CooSparseMatrix, OperateOnDensityMatrix, StruqtureError};
use num_complex::Complex64;
use qoqo_calculator::CalculatorComplex;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Iter, Keys, Values};
use std::iter::{FromIterator, IntoIterator};
use std::{
    fmt::{self, Write},
    ops,
};

/// SpinLindbladNoiseSystems are representations of systems of spins, with a SpinLindbladNoiseOperator to represent the hamiltonian of the spin system, and an optional number of spins.
///
/// In the Lindblad equation, Linblad noise operator L_i are not limited to [crate::spins::DecoherenceProduct] style operators.
/// We use ([crate::spins::DecoherenceProduct], [crate::spins::DecoherenceProduct]) as a unique basis.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::spins::{OperateOnSpins, DecoherenceProduct, SpinLindbladNoiseSystem};
///
/// let mut system = SpinLindbladNoiseSystem::new(Some(2));
///
/// // Representing the hamiltonian $ 1/2 \sigma_0^{x} \sigma_1^{x} + 1/5 \sigma_0^{z} $
/// let pp_0x1x = DecoherenceProduct::new().x(0).x(1);
/// let pp_0z = DecoherenceProduct::new().z(0);
/// system.set((pp_0x1x.clone(), pp_0x1x.clone()), CalculatorComplex::from(0.5)).unwrap();
/// system.set((pp_0z.clone(), pp_0z.clone()), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(system.number_spins(), 2_usize);
/// assert_eq!(system.get(&(pp_0x1x.clone(), pp_0x1x.clone())), &CalculatorComplex::from(0.5));
/// assert_eq!(system.get(&(pp_0z.clone(), pp_0z.clone())), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
pub struct SpinLindbladNoiseSystem {
    /// The number of spins in the SpinLindbladNoiseSystem.
    pub(crate) number_spins: Option<usize>,
    /// The SpinLindbladNoiseOperator representing the Lindblad noise terms of the SpinLindbladNoiseSystem.
    pub(crate) operator: SpinLindbladNoiseOperator,
}

impl crate::MinSupportedVersion for SpinLindbladNoiseSystem {}

impl<'a> OperateOnDensityMatrix<'a> for SpinLindbladNoiseSystem {
    type Value = CalculatorComplex;
    type Index = (DecoherenceProduct, DecoherenceProduct);
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
                number_spins: self.number_spins,
                operator: SpinLindbladNoiseOperator::with_capacity(cap),
            },
            None => Self {
                number_spins: self.number_spins,
                operator: SpinLindbladNoiseOperator::new(),
            },
        }
    }

    /// Overwrites an existing entry or sets a new entry in the SpinLindbladNoiseSystem with the given ((DecoherenceProduct, DecoherenceProduct) key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The (DecoherenceProduct, DecoherenceProduct) key to set in the SpinLindbladNoiseSystem.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the SpinLindbladNoiseSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(CalculatorComplex))` - The key existed, this is the value it had before it was set with the value input.
    /// * `Ok(None)` - The key did not exist, it has been set with its corresponding value.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Index of (DecoherenceProduct, DecoherenceProduct) exceeds that of the SpinLindbladNoiseSystem.
    fn set(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<Option<Self::Value>, StruqtureError> {
        match self.number_spins {
            Some(x) => {
                if key.0.current_number_spins() <= x && key.1.current_number_spins() <= x {
                    self.operator.set(key, value)
                } else {
                    Err(StruqtureError::NumberSpinsExceeded)
                }
            }
            None => self.operator.set(key, value),
        }
    }

    /// Adds a new ((DecoherenceProduct, DecoherenceProduct) key, CalculatorComplex value) pair to the SpinLindbladNoiseSystem.
    ///
    /// # Arguments
    ///
    /// * `key` - The (DecoherenceProduct, DecoherenceProduct) key to added to the SpinLindbladNoiseSystem.
    /// * `value` - The corresponding CalculatorComplex value to add for the key in the SpinLindbladNoiseSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The (key, value) pair was successfully added.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Index of (DecoherenceProduct, DecoherenceProduct) exceeds that of the SpinLindbladNoiseSystem.
    fn add_operator_product(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<(), StruqtureError> {
        match self.number_spins {
            Some(x) => {
                if key.0.current_number_spins() <= x && key.1.current_number_spins() <= x {
                    self.operator.add_operator_product(key, value)
                } else {
                    Err(StruqtureError::NumberSpinsExceeded)
                }
            }
            None => self.operator.add_operator_product(key, value),
        }
    }
}

impl<'a> OperateOnSpins<'a> for SpinLindbladNoiseSystem {
    /// Gets the number_spins input of the SpinLindbladNoiseSystem or returns the current_number_spins, if number_spins is None.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of spins in the SpinLindbladNoiseSystem.
    fn number_spins(&self) -> usize {
        match self.number_spins {
            Some(spins) => spins,
            None => self.operator.current_number_spins(),
        }
    }

    // From trait
    fn current_number_spins(&self) -> usize {
        self.operator.current_number_spins()
    }
}

impl<'a> ToSparseMatrixSuperOperator<'a> for SpinLindbladNoiseSystem {
    // From trait
    fn sparse_matrix_superoperator_entries_on_row(
        &'a self,
        row: usize,
        number_spins: usize,
    ) -> Result<std::collections::HashMap<usize, Complex64>, StruqtureError> {
        self.operator
            .sparse_matrix_superoperator_entries_on_row(row, number_spins)
    }

    // From trait
    fn unitary_sparse_matrix_coo(&'a self) -> Result<CooSparseMatrix, StruqtureError> {
        Ok((vec![], (vec![], vec![])) as CooSparseMatrix)
    }

    // From trait
    fn sparse_lindblad_entries(
        &'a self,
    ) -> Result<Vec<(CooSparseMatrix, CooSparseMatrix, Complex64)>, StruqtureError> {
        let mut coo_matrices =
            Vec::<(CooSparseMatrix, CooSparseMatrix, Complex64)>::with_capacity(self.len());
        for ((left, right), val) in self.iter() {
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

/// Functions for the SpinLindbladNoiseSystem.
///
impl SpinLindbladNoiseSystem {
    /// Creates a new SpinLindbladNoiseSystem.
    ///
    /// # Arguments
    ///
    /// * `number_spins` - The number of spins in the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new SpinLindbladNoiseSystem with the input number of spins.
    pub fn new(number_spins: Option<usize>) -> Self {
        SpinLindbladNoiseSystem {
            number_spins,
            operator: SpinLindbladNoiseOperator::new(),
        }
    }

    /// Creates a new SpinLindbladNoiseSystem with pre-allocated capacity and given number of spins.
    ///
    /// # Arguments
    ///
    /// * `number_spins` - The number of spins in the system.
    /// * `capacity` - The pre-allocated capacity of the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new SpinLindbladNoiseSystem with the input number of spins and capacity.
    pub fn with_capacity(number_spins: Option<usize>, capacity: usize) -> Self {
        SpinLindbladNoiseSystem {
            number_spins,
            operator: SpinLindbladNoiseOperator::with_capacity(capacity),
        }
    }

    /// Returns the SpinLindbladNoiseOperator of the SpinLindbladNoiseSystem.
    ///
    /// # Returns
    ///
    /// * `&SpinLindbladNoiseOperator` - The SpinLindbladNoiseOperator of the SpinLindbladNoiseSystem.
    pub fn operator(&self) -> &SpinLindbladNoiseOperator {
        &self.operator
    }

    /// Creates a SpinLindbladNoiseSystem from a SpinLindbladNoiseOperator and an optional number of spins.
    ///
    /// # Arguments
    ///
    /// * `operator` - The SpinLindbladNoiseOperator to create the SpinSytem from.
    /// * `number_spins` - The optional number of spins for the SpinLindbladNoiseSystem to be created.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The SpinLindbladNoiseSystem created from the inputs.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Number of spins in entry exceeds number of spins in system.
    pub fn from_operator(
        operator: SpinLindbladNoiseOperator,
        number_spins: Option<usize>,
    ) -> Result<Self, StruqtureError> {
        match number_spins {
            Some(x) => {
                if operator.current_number_spins() <= x {
                    Ok(SpinLindbladNoiseSystem {
                        number_spins: Some(x),
                        operator,
                    })
                } else {
                    Err(StruqtureError::NumberSpinsExceeded)
                }
            }
            None => Ok(SpinLindbladNoiseSystem {
                number_spins: None,
                operator,
            }),
        }
    }

    /// Separate self into an operator with the terms of given number of spins and an operator with the remaining operations
    ///
    /// # Arguments
    ///
    /// * `number_spins_left` - Number of spins to filter for in the left term of the keys.
    /// * `number_spins_right` - Number of spins to filter for in the right term of the keys.
    ///
    /// # Returns
    ///
    /// `Ok((separated, remainder))` - Operator with the noise terms where number_spins_left and number_spins_right match the number of spins the left and right noise operator product acts on and Operator with all other contributions.
    pub fn separate_into_n_terms(
        &self,
        number_spins_left: usize,
        number_spins_right: usize,
    ) -> Result<(Self, Self), StruqtureError> {
        let mut separated = Self::default();
        let mut remainder = Self::default();
        for ((prod_l, prod_r), val) in self.iter() {
            if prod_l.iter().len() == number_spins_left && prod_r.iter().len() == number_spins_right
            {
                separated.add_operator_product((prod_l.clone(), prod_r.clone()), val.clone())?;
            } else {
                remainder.add_operator_product((prod_l.clone(), prod_r.clone()), val.clone())?;
            }
        }
        Ok((separated, remainder))
    }
}

/// Implements the negative sign function of SpinLindbladNoiseSystem.
///
impl ops::Neg for SpinLindbladNoiseSystem {
    type Output = Self;
    /// Implement minus sign for SpinLindbladNoiseSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The SpinLindbladNoiseSystem * -1.
    fn neg(mut self) -> Self {
        self.operator = self.operator.neg();
        self
    }
}

/// Implements the plus function of SpinLindbladNoiseSystem by SpinLindbladNoiseSystem.
///
impl<T, V> ops::Add<T> for SpinLindbladNoiseSystem
where
    T: IntoIterator<Item = ((DecoherenceProduct, DecoherenceProduct), V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `+` (add) for two SpinLindbladNoiseSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The SpinLindbladNoiseSystem to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two SpinLindbladNoiseSystem added together.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Index of (DecoherenceProduct, DecoherenceProduct) exceeds that of the SpinLindbladNoiseSystem.
    fn add(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value))?;
        }
        Ok(self)
    }
}

/// Implements the minus function of SpinLindbladNoiseSystem by SpinLindbladNoiseSystem.
///
impl<T, V> ops::Sub<T> for SpinLindbladNoiseSystem
where
    T: IntoIterator<Item = ((DecoherenceProduct, DecoherenceProduct), V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `-` (subtract) for two SpinLindbladNoiseSystem.
    ///
    /// # Arguments
    ///
    /// * `other` - The SpinLindbladNoiseSystem to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two SpinLindbladNoiseSystem subtracted.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Index of (DecoherenceProduct, DecoherenceProduct) exceeds that of the SpinLindbladNoiseSystem.
    fn sub(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value) * -1.0)?;
        }
        Ok(self)
    }
}

/// Implements the multiplication function of SpinLindbladNoiseSystem by CalculatorComplex/CalculatorFloat.
///
impl<T> ops::Mul<T> for SpinLindbladNoiseSystem
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for SpinLindbladNoiseSystem and CalculatorComplex/CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex or CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The SpinLindbladNoiseSystem multiplied by the CalculatorComplex/CalculatorFloat.
    fn mul(mut self, other: T) -> Self {
        self.operator = self.operator * other;
        self
    }
}

/// Implements the into_iter function (IntoIterator trait) of SpinLindbladNoiseSystem.
///
impl IntoIterator for SpinLindbladNoiseSystem {
    type Item = ((DecoherenceProduct, DecoherenceProduct), CalculatorComplex);
    type IntoIter = std::collections::hash_map::IntoIter<
        (DecoherenceProduct, DecoherenceProduct),
        CalculatorComplex,
    >;
    /// Returns the SpinLindbladNoiseSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The SpinLindbladNoiseSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.operator.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference SpinLindbladNoiseSystem.
///
impl<'a> IntoIterator for &'a SpinLindbladNoiseSystem {
    type Item = (
        &'a (DecoherenceProduct, DecoherenceProduct),
        &'a CalculatorComplex,
    );
    type IntoIter = Iter<'a, (DecoherenceProduct, DecoherenceProduct), CalculatorComplex>;

    /// Returns the reference SpinLindbladNoiseSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference SpinLindbladNoiseSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.operator.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of SpinLindbladNoiseSystem.
///
impl FromIterator<((DecoherenceProduct, DecoherenceProduct), CalculatorComplex)>
    for SpinLindbladNoiseSystem
{
    /// Returns the object in SpinLindbladNoiseSystem form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the SpinLindbladNoiseSystem.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in SpinLindbladNoiseSystem form.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from_iter<
        I: IntoIterator<Item = ((DecoherenceProduct, DecoherenceProduct), CalculatorComplex)>,
    >(
        iter: I,
    ) -> Self {
        let mut so = SpinLindbladNoiseSystem::new(None);
        for (pp, cc) in iter {
            so.add_operator_product(pp.clone(), cc.clone())
                .expect("Internal error in add_operator_product");
        }
        so
    }
}

/// Implements the extend function (Extend trait) of SpinLindbladNoiseSystem.
///
impl Extend<((DecoherenceProduct, DecoherenceProduct), CalculatorComplex)>
    for SpinLindbladNoiseSystem
{
    /// Extends the SpinLindbladNoiseSystem by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the SpinLindbladNoiseSystem.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn extend<
        I: IntoIterator<Item = ((DecoherenceProduct, DecoherenceProduct), CalculatorComplex)>,
    >(
        &mut self,
        iter: I,
    ) {
        for (pp, cc) in iter {
            self.add_operator_product(pp, cc)
                .expect("Internal error in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of SpinLindbladNoiseSystem.
///
impl fmt::Display for SpinLindbladNoiseSystem {
    /// Formats the SpinLindbladNoiseSystem using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted SpinLindbladNoiseSystem.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("SpinLindbladNoiseSystem({}){{\n", self.number_spins());
        for (key, val) in self.iter() {
            writeln!(output, "({}, {}): {},", key.0, key.1, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}

impl JordanWignerSpinToFermion for SpinLindbladNoiseSystem {
    type Output = FermionLindbladNoiseSystem;

    /// Implements JordanWignerSpinToSpin for a SpinLindbladNoiseSystem.
    ///
    /// The convention used is that |0> represents an empty fermionic state (spin-orbital),
    /// and |1> represents an occupied fermionic state.
    ///
    /// # Returns
    ///
    /// `FermionLindbladNoiseSystem` - The fermion noise system that results from the transformation.
    ///
    /// # Panics
    ///
    /// * Internal error in jordan_wigner() for SpinLindbladNoiseOperator.
    fn jordan_wigner(&self) -> Self::Output {
        FermionLindbladNoiseSystem::from_operator(
            self.operator().jordan_wigner(),
            self.number_spins,
        )
            .expect("Internal bug in jordan_wigner() for SpinLindbladNoiseOperator. The number of modes in the resulting fermionic noise operator should equal the number of spins of the spin noise operator.")
    }
}
