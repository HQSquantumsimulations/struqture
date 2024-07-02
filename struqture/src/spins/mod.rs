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

//! Module for representing spin physical systems

use crate::{OperateOnDensityMatrix, SpinIndex, StruqtureError};
use num_complex::{Complex, Complex64};
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use std::collections::HashMap;
use std::convert::TryInto;
use std::iter::IntoIterator;
use std::ops::{Add, Mul, Sub};

mod decoherence_product;
pub use decoherence_product::*;

mod pauli_product;
pub use pauli_product::*;

mod decoherence_operator;
pub use decoherence_operator::*;

mod qubit_operator;
pub use qubit_operator::*;

mod qubit_hamiltonian;
pub use qubit_hamiltonian::*;

mod qubit_noise_operator;
pub use qubit_noise_operator::*;

mod qubit_open_system;
pub use qubit_open_system::*;

mod plus_minus_product;
pub use plus_minus_product::*;

mod plus_minus_operator;
pub use plus_minus_operator::*;

mod plus_minus_noise_operator;
pub use plus_minus_noise_operator::*;

use crate::CooSparseMatrix;

/// Trait for non-Hermitian operations on spins.
///
/// # Example
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use std::collections::HashMap;
/// use struqture::spins::{OperateOnSpins, PauliProduct, QubitOperator};
///
/// let mut so = QubitOperator::new();
/// let pp_0z = PauliProduct::new().z(0);
/// so.add_operator_product(pp_0z.clone(), CalculatorComplex::from(0.2)).unwrap();
/// let mut mapping: HashMap<PauliProduct, CalculatorComplex> = HashMap::new();
/// mapping.insert(pp_0z.clone(), CalculatorComplex::from(0.2));
///
/// // Functions provided in this :
/// assert_eq!(so.get(&pp_0z), &CalculatorComplex::from(0.2));
/// for (item_so, item_map) in so.iter().zip(mapping.iter()) {
///     assert_eq!(item_so, item_map);
/// }
/// for (key_so, key_map) in so.keys().zip(mapping.keys()) {
///     assert_eq!(key_so, key_map);
/// }
/// for (val_so, val_map) in so.values().zip(mapping.values()) {
///     assert_eq!(val_so, val_map);
/// }
/// assert_eq!(so.len(), 1_usize);
/// assert_eq!(so.is_empty(), false);
/// ```
///
///
pub trait OperateOnSpins<'a>: PartialEq + Clone + Mul<CalculatorFloat> + Add + Sub {
    // Document locally
    fn current_number_spins(&self) -> usize;
}

pub trait ToSparseMatrixOperator<'a>:
    ToSparseMatrixSuperOperator<'a>
    + OperateOnSpins<'a>
    + OperateOnDensityMatrix<'a>
    + IntoIterator<Item = (Self::Index, Self::Value)>
    + PartialEq
    + Clone
where
    SingleQubitOperator:
        From<<<Self as OperateOnDensityMatrix<'a>>::Index as SpinIndex>::SingleSpinType>,
    CalculatorComplex: From<<Self as OperateOnDensityMatrix<'a>>::Value>,
    &'a Self: IntoIterator<Item = (&'a Self::Index, &'a Self::Value)>,
    Self::Index: SpinIndex,
    Self::Value: Into<CalculatorComplex>,
{
    /// Constructs the sparse matrix representation of Self as a HashMap with a given number of spins.
    ///
    /// # Arguments
    ///
    /// * `number_spins` - The number of spins for which to construct the sparse matrix.
    ///
    /// # Returns
    ///
    /// * `Ok(HashMap<(usize, usize), CalculatorComplex>)` - The little endian matrix representation of the operator-like object.
    /// * `Err(CalculatorError)` - CalculatorFloat could not be converted to f64.
    fn sparse_matrix(
        &'a self,
        number_spins: Option<usize>,
    ) -> Result<HashMap<(usize, usize), Complex64>, StruqtureError> {
        let dimension = match number_spins {
            None => 2usize.pow(self.current_number_spins() as u32),
            Some(num_spins) => 2usize.pow(num_spins as u32),
        };
        let mut matrix: HashMap<(usize, usize), Complex64> = HashMap::new();
        for row in 0..dimension {
            for (column, val) in self.sparse_matrix_entries_on_row(row)?.into_iter() {
                matrix.insert((row, column), val);
            }
        }
        Ok(matrix)
    }

    /// Constructs the sparse matrix representation of the operator-like object as a scipy COO matrix with a given number of spins.
    ///
    /// # Arguments
    ///
    /// * `number_spins` - The number of spins for which to construct the sparse matrix in COO form.
    ///
    /// # Returns
    ///
    /// * `Ok((Vec<Complex64>, (Vec<usize>, Vec<usize>)))` - The little endian matrix representation of the operator-like object.
    /// * `Err(CalculatorError)` - CalculatorFloat could not be converted to f64.
    fn sparse_matrix_coo(
        &'a self,
        number_spins: Option<usize>,
    ) -> Result<CooSparseMatrix, StruqtureError> {
        let dimension = match number_spins {
            None => 2usize.pow(self.current_number_spins() as u32),
            Some(num_spins) => 2usize.pow(num_spins as u32),
        };

        let capacity = dimension;
        let mut values: Vec<Complex64> = Vec::with_capacity(capacity);
        let mut rows: Vec<usize> = Vec::with_capacity(capacity);
        let mut columns: Vec<usize> = Vec::with_capacity(capacity);

        for row in 0..dimension {
            for (col, val) in self.sparse_matrix_entries_on_row(row)?.into_iter() {
                rows.push(row);
                columns.push(col);
                values.push(val);
            }
        }
        Ok((values, (rows, columns)))
    }

    /// Constructs the sparse matrix entries for one row of the sparse matrix.
    ///
    /// # Arguments
    ///
    /// * `row` - The row for which to get the entries.
    /// * `number_spins` - The number of spins for which to construct the sparse matrix entries.
    ///
    /// # Returns
    ///
    /// * `Ok(HashMap<(usize, usize), CalculatorComplex>)` - The little endian matrix representation of Self.
    /// * `Err(CalculatorError)` - CalculatorFloat could not be converted to f64.
    fn sparse_matrix_entries_on_row(
        &'a self,
        row: usize,
    ) -> Result<HashMap<usize, Complex<f64>>, StruqtureError> {
        let mut entries: HashMap<usize, Complex<f64>> = HashMap::with_capacity(self.len());
        for (index, value) in self.iter() {
            let mut column = row;
            let mut prefac: Complex<f64> = 1.0.into();
            for (spin_op_index, pauliop) in index.iter() {
                match SingleQubitOperator::from(*pauliop) {
                    SingleQubitOperator::X => {
                        match row.div_euclid(2usize.pow(*spin_op_index as u32)) % 2 {
                            0 => column += 2usize.pow(*spin_op_index as u32),
                            1 => column -= 2usize.pow(*spin_op_index as u32),
                            _ => panic!("Internal error in constructing matrix"),
                        }
                    }
                    SingleQubitOperator::Y => {
                        match row.div_euclid(2usize.pow(*spin_op_index as u32)) % 2 {
                            0 => {
                                column += 2usize.pow(*spin_op_index as u32);
                                prefac *= Complex::<f64>::new(0.0, -1.0);
                            }
                            1 => {
                                column -= 2usize.pow(*spin_op_index as u32);
                                prefac *= Complex::<f64>::new(0.0, 1.0);
                            }
                            _ => panic!("Internal error in constructing matrix"),
                        };
                    }
                    SingleQubitOperator::Z => {
                        match row.div_euclid(2usize.pow(*spin_op_index as u32)) % 2 {
                            0 => {
                                prefac *= Complex::<f64>::new(1.0, 0.0);
                            }
                            1 => {
                                prefac *= Complex::<f64>::new(-1.0, 0.0);
                            }
                            _ => panic!("Internal error in constructing matrix"),
                        };
                    }
                    SingleQubitOperator::Identity => (),
                }
            }
            let mut_value = entries.get_mut(&column);
            let ri_value = CalculatorComplex::from(value.clone());
            let real_value: f64 = ri_value.re.try_into()?;
            let imag_value: f64 = ri_value.im.try_into()?;
            let complex_value = Complex::<f64>::new(real_value, imag_value);
            match mut_value {
                Some(x) => *x += prefac * complex_value,
                None => {
                    entries.insert(column, prefac * complex_value);
                }
            }
        }
        Ok(entries)
    }

    /// Constructs the sparse matrix entries for one row of the sparse matrix superoperator.
    ///
    /// # Arguments
    ///
    /// * `row` - The row for which to get the superoperator entries.
    /// * `number_spins` - The number of spins for which to construct the sparse matrix entries.
    ///
    /// # Returns
    ///
    /// * `Ok(HashMap<(usize, usize), CalculatorComplex>)` - The little endian matrix representation of the operator-like object.
    /// * `Err(CalculatorError)` - CalculatorFloat could not be converted to f64.
    fn sparse_matrix_superoperator_entries_on_row(
        &'a self,
        row: usize,
        number_spins: usize,
    ) -> Result<HashMap<usize, Complex<f64>>, StruqtureError> {
        let mut entries: HashMap<usize, Complex<f64>> = HashMap::new();
        let dimension = 2_usize.pow(number_spins as u32);
        let constant_prefactor = Complex64::new(0.0, -1.0);
        for (index, value) in self.iter() {
            // iterate over terms corresponding to -i H p => -i H.kron(I) flatten(p) and i p H => i I.kron(H.T)
            for (row_adjusted, commutator_prefactor, shift) in [
                (row.div_euclid(dimension), 1.0, number_spins),
                (row % dimension, -1.0, 0),
            ] {
                let mut column = row;
                let mut prefac: Complex<f64> = 1.0.into();
                // first the terms corresponding to -i H p => -i H.kron(I) flatten(p)
                for (spin_op_index, pauliop) in index.iter() {
                    match SingleQubitOperator::from(*pauliop) {
                        SingleQubitOperator::X => {
                            match row_adjusted.div_euclid(2usize.pow(*spin_op_index as u32)) % 2 {
                                0 => column += 2usize.pow((*spin_op_index + shift) as u32),
                                1 => column -= 2usize.pow((*spin_op_index + shift) as u32),
                                _ => panic!("Internal error in constructing matrix"),
                            }
                        }
                        SingleQubitOperator::Y => {
                            match row_adjusted.div_euclid(2usize.pow(*spin_op_index as u32)) % 2 {
                                0 => {
                                    column += 2usize.pow((*spin_op_index + shift) as u32);
                                    // due to the transpose in i p H => i I.kron(H.T) only the Y Pauli operator picks up an extra
                                    // sign equal to the commutator_prefactor
                                    prefac *= Complex::<f64>::new(0.0, -1.0) * commutator_prefactor;
                                }
                                1 => {
                                    column -= 2usize.pow((*spin_op_index + shift) as u32);
                                    prefac *= Complex::<f64>::new(0.0, 1.0) * commutator_prefactor;
                                }
                                _ => panic!("Internal error in constructing matrix"),
                            };
                        }
                        SingleQubitOperator::Z => {
                            match row_adjusted.div_euclid(2usize.pow(*spin_op_index as u32)) % 2 {
                                0 => {
                                    prefac *= Complex::<f64>::new(1.0, 0.0);
                                }
                                1 => {
                                    prefac *= Complex::<f64>::new(-1.0, 0.0);
                                }
                                _ => panic!("Internal error in constructing matrix"),
                            };
                        }
                        SingleQubitOperator::Identity => (),
                    }
                }
                prefac *= commutator_prefactor * constant_prefactor;
                let mut_value = entries.get_mut(&column);
                let ri_value = CalculatorComplex::from(value.clone());
                let real_value: f64 = ri_value.re.try_into()?;
                let imag_value: f64 = ri_value.im.try_into()?;
                let complex_value = Complex::<f64>::new(real_value, imag_value);
                if complex_value != Complex64::new(0.0, 0.0) {
                    match mut_value {
                        Some(x) => {
                            if *x + prefac * complex_value == Complex64::new(0.0, 0.0) {
                                entries.remove(&column);
                            } else {
                                *x += prefac * complex_value;
                            }
                        }
                        None => {
                            entries.insert(column, prefac * complex_value);
                        }
                    }
                }
            }
        }
        Ok(entries)
    }
}

pub trait ToSparseMatrixSuperOperator<'a>: OperateOnSpins<'a> + PartialEq + Clone {
    /// Constructs the sparse matrix representation of the superoperator as a HashMap.
    ///
    /// The superoperator for the operator O is defined as the Matrix S so that
    /// `flatten(-i [O, p]) = S flatten(p)` wher `[,]` is the commutator, `p` is a matrix
    /// and `flatten` flattens a matrix into a vector in row-major form.
    ///
    /// # Arguments
    ///
    /// * `number_spins` - The number of spins for which to construct the sparse matrix.
    ///
    /// # Returns
    ///
    /// * `HashMap<(usize, usize), CalculatorComplex>` - The little endian matrix representation of Self.
    /// * `Err(CalculatorError)` - CalculatorFloat could not be converted to f64.
    fn sparse_matrix_superoperator(
        &'a self,
        number_spins: Option<usize>,
    ) -> Result<HashMap<(usize, usize), Complex64>, StruqtureError> {
        let dimension = match number_spins {
            None => 2usize.pow(self.current_number_spins() as u32),
            Some(num_spins) => 2usize.pow(num_spins as u32),
        };
        let number_spins = match number_spins {
            None => self.current_number_spins(),
            Some(num_spins) => num_spins,
        };
        let mut matrix: HashMap<(usize, usize), Complex64> = HashMap::new();
        for row in 0..dimension.pow(2) {
            for (column, val) in self
                .sparse_matrix_superoperator_entries_on_row(row, number_spins)?
                .into_iter()
            {
                matrix.insert((row, column), val);
            }
        }
        Ok(matrix)
    }

    /// Constructs the sparse matrix representation of the superoperator in COO representation.
    ///
    /// The superoperator for the operator O is defined as the Matrix S so that
    /// `flatten(-i [O, p]) = S flatten(p)` wher `[,]` is the commutator, `p` is a matrix
    /// and `flatten` flattens a matrix into a vector in row-major form.
    ///
    /// # Arguments
    ///
    /// * `number_spins` - The number of spins for which to construct the sparse matrix in COO form.
    ///
    /// # Returns
    ///
    /// * `(Vec<Complex64>, (Vec<usize>, Vec<usize>)` - The little endian matrix representation of Self.
    /// * `Err(CalculatorError)` - CalculatorFloat could not be converted to f64.
    fn sparse_matrix_superoperator_coo(
        &'a self,
        number_spins: Option<usize>,
    ) -> Result<CooSparseMatrix, StruqtureError> {
        let dimension = match number_spins {
            None => 2usize.pow(self.current_number_spins() as u32),
            Some(num_spins) => 2usize.pow(num_spins as u32),
        };
        let number_spins = match number_spins {
            None => self.current_number_spins(),
            Some(num_spins) => num_spins,
        };
        let capacity = dimension;
        let mut values: Vec<Complex64> = Vec::with_capacity(capacity);
        let mut rows: Vec<usize> = Vec::with_capacity(capacity);
        let mut columns: Vec<usize> = Vec::with_capacity(capacity);

        for row in 0..dimension.pow(2) {
            for (col, val) in self
                .sparse_matrix_superoperator_entries_on_row(row, number_spins)?
                .into_iter()
            {
                rows.push(row);
                columns.push(col);
                values.push(val);
            }
        }
        Ok((values, (rows, columns)))
    }

    /// Constructs the sparse matrix entries for one row of the sparse matrix superoperator.
    ///
    /// # Arguments
    ///
    /// * `row` - The row for which to get the superoperator entries.
    /// * `number_spins` - The number of spins for which to construct the sparse matrix entries.
    ///
    /// # Returns
    ///
    /// * `Ok(HashMap<(usize, usize), CalculatorComplex>)` - The little endian matrix representation of Self.
    /// * `Err(CalculatorError)` - CalculatorFloat could not be converted to f64.
    fn sparse_matrix_superoperator_entries_on_row(
        &'a self,
        row: usize,
        number_spins: usize,
    ) -> Result<HashMap<usize, Complex<f64>>, StruqtureError>;

    /// Return the unitary part of the superoperator in the sparse COO format.
    ///
    /// # Arguments
    ///
    /// * `number_spins` - The number of spins for which to construct the unitary sparse matrix.
    ///
    /// # Returns
    ///
    /// * `Ok((Vec<Complex64>, (Vec<usize>, Vec<usize>))` - The little endian matrix representation of Self.
    /// * `Err(CalculatorError)` - CalculatorFloat could not be converted to f64.
    fn unitary_sparse_matrix_coo(
        &'a self,
        number_spins: Option<usize>,
    ) -> Result<CooSparseMatrix, StruqtureError>;

    /// Output the Lindblad entries in the form (left, right, rate) where left/right are the left and right lindblad operators, and rate is the lindblad rate respectively.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<((Vec<Complex64>, (Vec<usize>, Vec<usize>), (Vec<Complex64>, (Vec<usize>, Vec<usize>), Complex64)>)` - The little endian matrix representation of Self.
    /// * `Err(CalculatorError)` - CalculatorFloat could not be converted to f64.
    fn sparse_lindblad_entries(
        &'a self,
    ) -> Result<Vec<(CooSparseMatrix, CooSparseMatrix, Complex64)>, StruqtureError>;
}

/// Trait for Hermitian operations on spins.
///
/// # Example
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorFloat;
/// use std::collections::HashMap;
/// use struqture::spins::{HermitianOperateOnSpins, PauliProduct, QubitHamiltonian};
///
/// let mut sh = QubitHamiltonian::new();
/// let pp_0z = PauliProduct::new().z(0);
/// sh.add_operator_product(pp_0z.clone(), CalculatorFloat::from(0.2)).unwrap();
/// let mut mapping: HashMap<PauliProduct, CalculatorFloat> = HashMap::new();
/// mapping.insert(pp_0z.clone(), CalculatorFloat::from(0.2));
///
/// // Functions provided in this :
/// assert_eq!(sh.get(&pp_0z), &CalculatorFloat::from(0.2));
/// for (item_sh, item_map) in sh.iter().zip(mapping.iter()) {
///     assert_eq!(item_sh, item_map);
/// }
/// for (key_sh, key_map) in sh.keys().zip(mapping.keys()) {
///     assert_eq!(key_sh, key_map);
/// }
/// for (val_sh, val_map) in sh.values().zip(mapping.values()) {
///     assert_eq!(val_sh, val_map);
/// }
/// assert_eq!(sh.len(), 1_usize);
/// assert_eq!(sh.is_empty(), false);
/// ```
///
pub trait HermitianOperateOnSpins<'a>:
    OperateOnSpins<'a>
    + OperateOnDensityMatrix<'a>
    + IntoIterator<Item = (Self::Index, Self::Value)>
    + PartialEq
    + Clone
where
    &'a Self: IntoIterator<Item = (&'a Self::Index, &'a Self::Value)>,

    SingleQubitOperator:
        From<<<Self as OperateOnDensityMatrix<'a>>::Index as SpinIndex>::SingleSpinType>,
    <Self as OperateOnDensityMatrix<'a>>::Value: Into<CalculatorFloat>,

    Self::Index: SpinIndex,
{
}
