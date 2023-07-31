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

use super::{HermitianOperateOnSpins, OperateOnSpins, SpinSystem};
use crate::fermions::FermionHamiltonianSystem;
use crate::mappings::JordanWignerSpinToFermion;
use crate::spins::{
    PauliProduct, SpinHamiltonian, ToSparseMatrixOperator, ToSparseMatrixSuperOperator,
};
use crate::{CooSparseMatrix, OperateOnDensityMatrix, OperateOnState, SpinIndex, StruqtureError};
use num_complex::Complex64;
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Iter, Keys, Values};
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

/// SpinHamiltonianSystems are representations of physical systems of spins, with a SpinHamiltonian to represent the hermitian hamiltonian of the system, and an optional number of spins.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorFloat;
/// use struqture::spins::{HermitianOperateOnSpins, PauliProduct, SpinHamiltonianSystem};
///
/// let mut system = SpinHamiltonianSystem::new(Some(2));
///
/// // Representing the hamiltonian $ 1/2 \sigma_0^{x} \sigma_1^{x} + 1/5 \sigma_0^{z} $
/// let pp_0x1x = PauliProduct::new().x(0).x(1);
/// let pp_0z = PauliProduct::new().z(0);
/// system.add_operator_product(pp_0x1x.clone(), CalculatorFloat::from(0.5)).unwrap();
/// system.add_operator_product(pp_0z.clone(), CalculatorFloat::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(system.number_spins(), 2_usize);
/// assert_eq!(system.get(&pp_0x1x), &CalculatorFloat::from(0.5));
/// assert_eq!(system.get(&pp_0z), &CalculatorFloat::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct SpinHamiltonianSystem {
    /// The number of spins in the SpinHamiltonianSystem
    pub(crate) number_spins: Option<usize>,
    /// The SpinHamiltonian representing the Hamiltonian of the SpinHamiltonianSystem
    pub(crate) hamiltonian: SpinHamiltonian,
}

impl crate::MinSupportedVersion for SpinHamiltonianSystem {}

impl<'a> OperateOnDensityMatrix<'a> for SpinHamiltonianSystem {
    type Index = PauliProduct;
    type Value = CalculatorFloat;
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
                number_spins: self.number_spins,
                hamiltonian: SpinHamiltonian::with_capacity(cap),
            },
            None => Self {
                number_spins: self.number_spins,
                hamiltonian: SpinHamiltonian::new(),
            },
        }
    }

    /// Overwrites an existing entry or sets a new entry in the SpinHamiltonianSystem with the given (PauliProduct key, CalculatorFloat value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The PauliProduct key to set in the SpinHamiltonianSystem.
    /// * `value` - The corresponding CalculatorFloat value to set for the key in the SpinHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(CalculatorFloat))` - The key existed, this is the value it had before it was set with the value input.
    /// * `Ok(None)` - The key did not exist, it has been set with its corresponding value.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Index of PauliProduct exceeds that of the SpinHamiltonianSystem.
    fn set(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<Option<Self::Value>, StruqtureError> {
        match self.number_spins {
            Some(x) => {
                if key.current_number_spins() <= x {
                    self.hamiltonian.set(key, value)
                } else {
                    Err(StruqtureError::NumberSpinsExceeded)
                }
            }
            None => self.hamiltonian.set(key, value),
        }
    }

    /// Adds a new (PauliProduct key, CalculatorFloat value) pair to the SpinHamiltonianSystem.
    ///
    /// # Arguments
    ///
    /// * `key` - The PauliProduct key to added to the SpinHamiltonianSystem.
    /// * `value` - The corresponding CalculatorFloat value to add for the key in the SpinHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The (key, value) pair was successfully added.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Index of PauliProduct exceeds that of the SpinHamiltonianSystem.
    fn add_operator_product(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<(), StruqtureError> {
        match self.number_spins {
            Some(x) => {
                if key.current_number_spins() <= x {
                    self.hamiltonian.add_operator_product(key, value)
                } else {
                    Err(StruqtureError::NumberSpinsExceeded)
                }
            }
            None => self.hamiltonian.add_operator_product(key, value),
        }
    }
}

impl<'a> OperateOnState<'a> for SpinHamiltonianSystem {
    // From trait
    fn hermitian_conjugate(&self) -> Self {
        self.clone()
    }
}

impl<'a> OperateOnSpins<'a> for SpinHamiltonianSystem {
    /// Gets the number_spins input of the SpinHamiltonianSystem, or the current_number_spins if number_spins is None.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of spins in the SpinHamiltonianSystem.
    fn number_spins(&self) -> usize {
        match self.number_spins {
            Some(spins) => spins,
            None => self.hamiltonian.current_number_spins(),
        }
    }

    // From trait
    fn current_number_spins(&self) -> usize {
        self.hamiltonian.current_number_spins()
    }
}

impl<'a> HermitianOperateOnSpins<'a> for SpinHamiltonianSystem {}

impl<'a> ToSparseMatrixOperator<'a> for SpinHamiltonianSystem {}
impl<'a> ToSparseMatrixSuperOperator<'a> for SpinHamiltonianSystem {
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
        self.sparse_matrix_coo(self.number_spins)
        // However this would also work: self.hamiltonian.sparse_matrix_coo(self.number_spins)
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

/// Functions for the SpinHamiltonianSystem
///
impl SpinHamiltonianSystem {
    /// Creates a new SpinHamiltonianSystem.
    ///
    /// # Arguments
    ///
    /// * `number_spins` - The number of spins in the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new SpinHamiltonianSystem with the input number of spins.
    pub fn new(number_spins: Option<usize>) -> Self {
        SpinHamiltonianSystem {
            number_spins,
            hamiltonian: SpinHamiltonian::new(),
        }
    }

    /// Creates a new SpinHamiltonianSystem with capacity.
    ///
    /// # Arguments
    ///
    /// * `number_spins` - The number of spins in the system
    /// * `capacity` - The pre-allocated capacity of the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new SpinHamiltonianSystem with the input number of spins.
    pub fn with_capacity(number_spins: Option<usize>, capacity: usize) -> Self {
        SpinHamiltonianSystem {
            number_spins,
            hamiltonian: SpinHamiltonian::with_capacity(capacity),
        }
    }

    /// Returns the SpinHamiltonian of the SpinHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `&SpinHamiltonian` - The SpinHamiltonian of the SpinHamiltonianSystem.
    #[deprecated(note = "Use the hamiltonian() method instead.")]
    pub fn operator(&self) -> &SpinHamiltonian {
        &self.hamiltonian
    }

    /// Returns the SpinHamiltonian of the SpinHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `&SpinHamiltonian` - The SpinHamiltonian of the SpinHamiltonianSystem.
    pub fn hamiltonian(&self) -> &SpinHamiltonian {
        &self.hamiltonian
    }

    /// Creates a SpinHamiltonianSystem from a SpinHamiltonian and an optional number of spins.
    ///
    /// # Arguments
    ///
    /// * `operator` - The SpinHamiltonian to create the SpinHamiltonianSystem from.
    /// * `number_spins` - The optional number of spins for the SpinHamiltonianSystem to be created.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The SpinHamiltonianSystem created from the inputs.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Number of spins in entry exceeds number of spins in system.
    pub fn from_hamiltonian(
        hamiltonian: SpinHamiltonian,
        number_spins: Option<usize>,
    ) -> Result<Self, StruqtureError> {
        match number_spins {
            Some(x) => {
                if hamiltonian.current_number_spins() <= x {
                    Ok(SpinHamiltonianSystem {
                        number_spins: Some(x),
                        hamiltonian,
                    })
                } else {
                    Err(StruqtureError::NumberSpinsExceeded)
                }
            }
            None => Ok(SpinHamiltonianSystem {
                number_spins: None,
                hamiltonian,
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

/// Implements the negative sign function of SpinHamiltonianSystem.
///
impl ops::Neg for SpinHamiltonianSystem {
    type Output = Self;
    /// Implement minus sign for SpinHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `Self` - The SpinHamiltonianSystem * -1.
    fn neg(mut self) -> Self {
        self.hamiltonian = self.hamiltonian.neg();
        self
    }
}

/// Implements the plus function of SpinHamiltonianSystem by SpinHamiltonianSystem.
///
impl<T, V> ops::Add<T> for SpinHamiltonianSystem
where
    T: IntoIterator<Item = (PauliProduct, V)>,
    V: Into<CalculatorFloat>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `+` (add) for two SpinHamiltonianSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The SpinHamiltonianSystem to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two SpinHamiltonianSystems added together.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Index of PauliProduct exceeds that of the SpinHamiltonianSystem.
    fn add(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorFloat>::into(value))?;
        }
        Ok(self)
    }
}

/// Implements the minus function of SpinHamiltonianSystem by SpinHamiltonianSystem.
///
impl<T, V> ops::Sub<T> for SpinHamiltonianSystem
where
    T: IntoIterator<Item = (PauliProduct, V)>,
    V: Into<CalculatorFloat>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `-` (subtract) for two SpinHamiltonianSystems.
    ///
    /// # Arguments
    ///
    /// * `other` - The SpinHamiltonianSystem to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two SpinHamiltonianSystems subtracted.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Index of PauliProduct exceeds that of the SpinHamiltonianSystem.
    fn sub(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorFloat>::into(value) * -1.0)?;
        }
        Ok(self)
    }
}

/// Implements the multiplication function of SpinHamiltonianSystem by CalculatorFloat.
///
impl ops::Mul<CalculatorFloat> for SpinHamiltonianSystem {
    type Output = Self;
    /// Implement `*` for SpinHamiltonianSystem and CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The SpinHamiltonianSystem multiplied by the CalculatorFloat.
    fn mul(mut self, other: CalculatorFloat) -> Self {
        self.hamiltonian = self.hamiltonian * other;
        self
    }
}

/// Implements the multiplication function of SpinHamiltonianSystem by CalculatorComplex.
///
impl ops::Mul<CalculatorComplex> for SpinHamiltonianSystem {
    type Output = SpinSystem;
    /// Implement `*` for SpinHamiltonianSystem and CalculatorComplex.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex by which to multiply.
    ///
    /// # Returns
    ///
    /// * `SpinSystem` - The SpinHamiltonianSystem multiplied by the CalculatorComplex.
    fn mul(self, other: CalculatorComplex) -> Self::Output {
        let mut system = SpinSystem::new(self.number_spins);
        system.operator = self.hamiltonian * other;
        system
    }
}

/// Implements the multiplication function of SpinHamiltonianSystem by SpinHamiltonianSystem.
///
impl ops::Mul<SpinHamiltonianSystem> for SpinHamiltonianSystem {
    type Output = Result<SpinSystem, StruqtureError>;
    /// Implement `*` for SpinHamiltonianSystem and SpinHamiltonianSystem.
    ///
    /// # Arguments
    ///
    /// * `other` - The SpinHamiltonianSystem to multiply by.
    ///
    /// # Returns
    ///
    /// * `Ok(SpinSystem)` - The two SpinHamiltonianSystems multiplied.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Index of PauliProduct exceeds that of the SpinSystem.
    fn mul(self, other: SpinHamiltonianSystem) -> Self::Output {
        Ok(SpinSystem {
            number_spins: Some(self.number_spins().max(other.number_spins())),
            operator: self.hamiltonian * other.hamiltonian,
        })
    }
}

/// Implements the into_iter function (IntoIterator trait) of SpinHamiltonianSystem.
///
impl IntoIterator for SpinHamiltonianSystem {
    type Item = (PauliProduct, CalculatorFloat);
    type IntoIter = std::collections::hash_map::IntoIter<PauliProduct, CalculatorFloat>;
    /// Returns the SpinHamiltonianSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The SpinHamiltonianSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.hamiltonian.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference SpinHamiltonianSystem.
///
impl<'a> IntoIterator for &'a SpinHamiltonianSystem {
    type Item = (&'a PauliProduct, &'a CalculatorFloat);
    type IntoIter = Iter<'a, PauliProduct, CalculatorFloat>;

    /// Returns the reference SpinHamiltonianSystem in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference SpinHamiltonianSystem in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.hamiltonian.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of SpinHamiltonianSystem.
///
impl FromIterator<(PauliProduct, CalculatorFloat)> for SpinHamiltonianSystem {
    /// Returns the object in SpinHamiltonianSystem form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the SpinHamiltonianSystem.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in SpinHamiltonianSystem form.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from_iter<I: IntoIterator<Item = (PauliProduct, CalculatorFloat)>>(iter: I) -> Self {
        let mut so = SpinHamiltonianSystem::new(None);
        for (pp, cc) in iter {
            so.add_operator_product(pp.clone(), cc.clone())
                .expect("Internal error in add_operator_product");
        }
        so
    }
}

/// Implements the extend function (Extend trait) of SpinHamiltonianSystem.
///
impl Extend<(PauliProduct, CalculatorFloat)> for SpinHamiltonianSystem {
    /// Extends the SpinHamiltonianSystem by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the SpinHamiltonianSystem.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn extend<I: IntoIterator<Item = (PauliProduct, CalculatorFloat)>>(&mut self, iter: I) {
        for (pp, cc) in iter {
            self.add_operator_product(pp, cc)
                .expect("Internal error in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of SpinHamiltonianSystem.
///
impl fmt::Display for SpinHamiltonianSystem {
    /// Formats the SpinHamiltonianSystem using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted SpinHamiltonianSystem.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("SpinHamiltonianSystem({}){{\n", self.number_spins());
        for (key, val) in self.iter() {
            writeln!(output, "{}: {},", key, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}

impl JordanWignerSpinToFermion for SpinHamiltonianSystem {
    type Output = FermionHamiltonianSystem;

    /// Implements JordanWignerSpinToSpin for a SpinHamiltonianSystem.
    ///
    /// The convention used is that |0> represents an empty fermionic state (spin-orbital),
    /// and |1> represents an occupied fermionic state.
    ///
    /// # Returns
    ///
    /// `FermionHamiltonianSystem` - The fermion hamiltonian system that results from the transformation.
    ///
    /// # Panics
    ///
    /// * Internal error in jordan_wigner() for SpinHamiltonian.
    fn jordan_wigner(&self) -> Self::Output {
        FermionHamiltonianSystem::from_hamiltonian(
            self.hamiltonian().jordan_wigner(),
            self.number_spins,
        )
            .expect("Internal bug in jordan_wigner() for SpinHamiltonian. The number of modes in the resulting fermionic Hamiltonian should equal the number of spins of the spin Hamiltonian.")
    }
}
