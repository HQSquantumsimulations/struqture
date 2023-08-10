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

use super::{ToSparseMatrixOperator, ToSparseMatrixSuperOperator};
use crate::fermions::FermionSystem;
use crate::mappings::JordanWignerSpinToFermion;
use crate::spins::{OperateOnSpins, PauliProduct, SpinIndex, SpinOperator};
use crate::{
    CooSparseMatrix, OperateOnDensityMatrix, OperateOnState, StruqtureError, SymmetricIndex,
};
use num_complex::Complex64;
use qoqo_calculator::CalculatorComplex;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Iter, Keys, Values};
use std::iter::{FromIterator, IntoIterator};
use std::{
    fmt::{self, Write},
    ops,
};

/// SpinSystems are SpinOperators with a certain number of spins. When constructing it, the `new` function takes a `number_spins` input, and therefore
/// when the user adds a set of PauliProducts with specific CalculatorComplex coefficients, their indices must not exceed the number of spins in the SpinSystem.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::spins::{OperateOnSpins, PauliProduct, SpinSystem};
///
/// let mut system = SpinSystem::new(Some(2));
///
/// // Representing the hamiltonian $ 1/2 \sigma_0^{x} \sigma_1^{x} + 1/5 \sigma_0^{z} $
/// let pp_0x1x = PauliProduct::new().x(0).x(1);
/// let pp_0z = PauliProduct::new().z(0);
/// system.add_operator_product(pp_0x1x.clone(), CalculatorComplex::from(0.5)).unwrap();
/// system.add_operator_product(pp_0z.clone(), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(system.number_spins(), 2_usize);
/// assert_eq!(system.get(&pp_0x1x), &CalculatorComplex::from(0.5));
/// assert_eq!(system.get(&pp_0z), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct SpinSystem {
    /// The number of spins in the SpinSystem.
    pub(crate) number_spins: Option<usize>,
    /// The SpinOperator representing the Hamiltonian of the SpinSystem.
    pub(crate) operator: SpinOperator,
}

impl crate::MinSupportedVersion for SpinSystem {}

impl<'a> OperateOnDensityMatrix<'a> for SpinSystem {
    type Value = CalculatorComplex;
    type Index = PauliProduct;
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
                operator: SpinOperator::with_capacity(cap),
            },
            None => Self {
                number_spins: self.number_spins,
                operator: SpinOperator::new(),
            },
        }
    }

    /// Overwrites an existing entry or sets a new entry in the SpinSystem with the given (PauliProduct key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The PauliProduct key to set in the SpinSystem.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the SpinSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(CalculatorComplex))` - The key existed, this is the value it had before it was set with the value input.
    /// * `Ok(None)` - The key did not exist, it has been set with its corresponding value.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Index of PauliProduct exceeds that of the SpinSystem.
    fn set(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<Option<Self::Value>, StruqtureError> {
        match self.number_spins {
            Some(x) => {
                if key.current_number_spins() <= x {
                    self.operator.set(key, value)
                } else {
                    Err(StruqtureError::NumberSpinsExceeded)
                }
            }
            None => self.operator.set(key, value),
        }
    }

    /// Adds a new (PauliProduct key, CalculatorComplex value) pair to the SpinSystem.
    ///
    /// # Arguments
    ///
    /// * `key` - The PauliProduct key to added to the SpinSystem.
    /// * `value` - The corresponding CalculatorComplex value to add for the key in the SpinSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The (key, value) pair was successfully added.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Index of PauliProduct exceeds that of the SpinSystem.
    fn add_operator_product(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<(), StruqtureError> {
        match self.number_spins {
            Some(x) => {
                if key.current_number_spins() <= x {
                    self.operator.add_operator_product(key, value)
                } else {
                    Err(StruqtureError::NumberSpinsExceeded)
                }
            }
            None => self.operator.add_operator_product(key, value),
        }
    }
}

impl<'a> OperateOnState<'a> for SpinSystem {
    // From trait
    fn hermitian_conjugate(&self) -> Self {
        let mut new_operator = Self::with_capacity(self.number_spins, self.len());
        for (pauli_product, value) in self.iter() {
            let (new_spin_product, prefactor) = pauli_product.hermitian_conjugate();
            new_operator
                .add_operator_product(new_spin_product, value.conj() * prefactor)
                .expect("Internal bug in add_operator_product");
        }
        new_operator
    }
}

impl<'a> OperateOnSpins<'a> for SpinSystem {
    /// Gets the number_spins input of the SpinSystem or returns the current_number_spins, if number_spins is None.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of spins in the SpinSystem.
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

impl<'a> ToSparseMatrixOperator<'a> for SpinSystem {}
impl<'a> ToSparseMatrixSuperOperator<'a> for SpinSystem {
    // From trait
    fn sparse_matrix_superoperator_entries_on_row(
        &'a self,
        row: usize,
        number_spins: usize,
    ) -> Result<std::collections::HashMap<usize, Complex64>, StruqtureError> {
        <Self as ToSparseMatrixOperator>::sparse_matrix_superoperator_entries_on_row(
            self,
            row,
            number_spins,
        )
    }

    // From trait
    fn unitary_sparse_matrix_coo(&'a self) -> Result<CooSparseMatrix, StruqtureError> {
        self.operator.sparse_matrix_coo(self.number_spins)
    }

    // From trait
    fn sparse_lindblad_entries(
        &'a self,
    ) -> Result<Vec<(CooSparseMatrix, CooSparseMatrix, Complex64)>, StruqtureError> {
        let rate = Complex64::default();
        let left: CooSparseMatrix = (vec![], (vec![], vec![]));
        let right: CooSparseMatrix = (vec![], (vec![], vec![]));
        Ok(vec![(left, right, rate)])
    }
}

/// Functions for the SpinSystem.
///
impl SpinSystem {
    /// Creates a new SpinSystem.
    ///
    /// # Arguments
    ///
    /// * `number_spins` - The number of spins in the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new SpinSystem with the input number of spins.
    pub fn new(number_spins: Option<usize>) -> Self {
        SpinSystem {
            number_spins,
            operator: SpinOperator::new(),
        }
    }

    /// Creates a new SpinSystem with pre-allocated capacity and given number of spins.
    ///
    /// # Arguments
    ///
    /// * `number_spins` - The number of spins in the system.
    /// * `capacity` - The pre-allocated capacity of the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new SpinSystem with the given number of spins and capacity.
    pub fn with_capacity(number_spins: Option<usize>, capacity: usize) -> Self {
        SpinSystem {
            number_spins,
            operator: SpinOperator::with_capacity(capacity),
        }
    }

    /// Returns the SpinOperator of the SpinSystem.
    ///
    /// # Returns
    ///
    /// * `&SpinOperator` - The SpinOperator of the SpinSystem.
    pub fn operator(&self) -> &SpinOperator {
        &self.operator
    }

    /// Creates a SpinSystem from a SpinOperator and an optional number of spins.
    ///
    /// # Arguments
    ///
    /// * `operator` - The SpinOperator to create the SpinSytem from.
    /// * `number_spins` - The optional number of spins for the SpinSystem to be created.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The SpinSystem created from the inputs.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Number of spins in entry exceeds number of spins in system.
    pub fn from_operator(
        operator: SpinOperator,
        number_spins: Option<usize>,
    ) -> Result<Self, StruqtureError> {
        match number_spins {
            Some(x) => {
                if operator.current_number_spins() <= x {
                    Ok(SpinSystem {
                        number_spins: Some(x),
                        operator,
                    })
                } else {
                    Err(StruqtureError::NumberSpinsExceeded)
                }
            }
            None => Ok(SpinSystem {
                number_spins: None,
                operator,
            }),
        }
    }

    /// Separate self into an operator with the terms of given number of spins and an operator with the remaining operations
    ///
    /// # Arguments
    ///
    /// * `number_spins` - Number of spins to filter for in the keys.
    ///
    /// # Returns
    ///
    /// `Ok((separated, remainder))` - Operator with the noise terms where number_spins matches the number of spins the operator product acts on and Operator with all other contributions.
    pub fn separate_into_n_terms(
        &self,
        number_spins: usize,
    ) -> Result<(Self, Self), StruqtureError> {
        let mut separated = Self::default();
        let mut remainder = Self::default();
        for (prod, val) in self.iter() {
            if prod.len() == number_spins {
                separated.add_operator_product(prod.clone(), val.clone())?;
            } else {
                remainder.add_operator_product(prod.clone(), val.clone())?;
            }
        }
        Ok((separated, remainder))
    }
}

/// Implements the negative sign function of SpinSystem.
///
impl ops::Neg for SpinSystem {
    type Output = Self;
    /// Implement minus sign for SpinSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The SpinSystem * -1.
    fn neg(mut self) -> Self {
        self.operator = self.operator.neg();
        self
    }
}

/// Implements the plus function of SpinSystem by SpinSystem.
///
impl<T, V> ops::Add<T> for SpinSystem
where
    T: IntoIterator<Item = (PauliProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `+` (add) for two SpinSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The SpinSystem to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two SpinSystems added together.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of PauliProduct exceeds that of the SpinSystem.
    fn add(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value))?;
        }
        Ok(self)
    }
}

/// Implements the minus function of SpinSystem by SpinSystem.
///
impl<T, V> ops::Sub<T> for SpinSystem
where
    T: IntoIterator<Item = (PauliProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `-` (subtract) for two SpinSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The SpinSystem to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two SpinSystems subtracted.
    /// * `Err(StruqtureError::NumberModesExceeded)` - Index of PauliProduct exceeds that of the SpinSystem.
    fn sub(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value) * -1.0)?;
        }
        Ok(self)
    }
}

/// Implements the multiplication function of SpinSystem by CalculatorComplex/CalculatorFloat.
///
impl<T> ops::Mul<T> for SpinSystem
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implements `*` for SpinSystem and CalculatorComplex/CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex or CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The SpinSystem multiplied by the CalculatorComplex/CalculatorFloat.
    fn mul(mut self, other: T) -> Self {
        self.operator = self.operator * other;
        self
    }
}

/// Implements the multiplication function of SpinSystem by SpinSystem.
///
impl ops::Mul<SpinSystem> for SpinSystem {
    type Output = Self;
    /// Implement `*` for SpinSystem and SpinSystem.
    ///
    /// # Arguments
    ///
    /// * `other` - The SpinSystem to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - The two SpinSystems multiplied.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn mul(self, other: SpinSystem) -> Self::Output {
        SpinSystem {
            number_spins: Some(self.number_spins().max(other.number_spins())),
            operator: self.operator * other.operator,
        }
    }
}

/// Implements the into_iter function (IntoIterator trait) of SpinSystem.
///
impl IntoIterator for SpinSystem {
    type Item = (PauliProduct, CalculatorComplex);
    type IntoIter = std::collections::hash_map::IntoIter<PauliProduct, CalculatorComplex>;
    /// Returns the SpinSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The SpinSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.operator.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference SpinSystem.
///
impl<'a> IntoIterator for &'a SpinSystem {
    type Item = (&'a PauliProduct, &'a CalculatorComplex);
    type IntoIter = Iter<'a, PauliProduct, CalculatorComplex>;

    /// Returns the reference SpinSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference SpinSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.operator.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of SpinSystem.
///
impl FromIterator<(PauliProduct, CalculatorComplex)> for SpinSystem {
    /// Returns the object in SpinSystem form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the SpinSystem.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in SpinSystem form.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from_iter<I: IntoIterator<Item = (PauliProduct, CalculatorComplex)>>(iter: I) -> Self {
        let mut so = SpinSystem::new(None);
        for (pp, cc) in iter {
            so.add_operator_product(pp.clone(), cc.clone())
                .expect("Internal error in add_operator_product");
        }
        so
    }
}

/// Implements the extend function (Extend trait) of SpinSystem.
///
impl Extend<(PauliProduct, CalculatorComplex)> for SpinSystem {
    /// Extends the SpinSystem by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the SpinSystem.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn extend<I: IntoIterator<Item = (PauliProduct, CalculatorComplex)>>(&mut self, iter: I) {
        for (pp, cc) in iter {
            self.add_operator_product(pp, cc)
                .expect("Internal error in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of SpinSystem.
///
impl fmt::Display for SpinSystem {
    /// Formats the SpinSystem using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted SpinSystem.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("SpinSystem({}){{\n", self.number_spins());
        for (key, val) in self.iter() {
            writeln!(output, "{}: {},", key, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}

impl JordanWignerSpinToFermion for SpinSystem {
    type Output = FermionSystem;

    /// Implements JordanWignerSpinToFermion for a SpinSystem.
    ///
    /// The convention used is that |0> represents an empty fermionic state (spin-orbital),
    /// and |1> represents an occupied fermionic state.
    ///
    /// # Returns
    ///
    /// `FermionSystem` - The fermionic system that results from the transformation.
    ///
    /// # Panics
    ///
    /// * Internal bug in jordan_wigner() for SpinSystem.
    fn jordan_wigner(&self) -> Self::Output {
        FermionSystem::from_operator(
            self.operator().jordan_wigner(),
            Some(self.number_spins()),
        )
            .expect("Internal bug in jordan_wigner() for SpinSystem. The number of modes in the resulting FermionSystem should equal the number of spins of the SpinSystem.")
    }
}
