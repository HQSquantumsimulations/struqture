// Copyright © 2021-2022 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use crate::spins::DecoherenceProductWrapper;
use crate::{to_py_coo, PyCooMatrix};
use bincode::deserialize;
use num_complex::Complex64;
use pyo3::exceptions::{PyRuntimeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::spins::{OperateOnSpins, SpinLindbladNoiseSystem, ToSparseMatrixSuperOperator};
use struqture::{OperateOnDensityMatrix, StruqtureError};
use struqture_py_macros::noisy_system_wrapper;

/// These are representations of noisy systems of spins.
///
/// In a SpinLindbladNoiseSystem is characterized by a SpinLindbladNoiseOperator to represent the hamiltonian of the spin system, and an optional number of spins.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.spins import SpinLindbladNoiseSystem, DecoherenceProduct
///
///     slns = SpinLindbladNoiseSystem()
///     dp = DecoherenceProduct().z(0).x(1)
///     slns.add_operator_product((dp, dp), 2.0)
///     npt.assert_equal(slns.current_number_spins(), 2)
///     npt.assert_equal(slns.get((dp, dp)), CalculatorComplex(2))
///     npt.assert_equal(slns.keys(), [(dp, dp)])
///     dimension = 4**slns.number_spins()
///     matrix = sp.coo_matrix(slns.sparse_matrix_superoperator_coo(), shape=(dimension, dimension))
///
#[pyclass(name = "SpinLindbladNoiseSystem", module = "struqture_py.spins")]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SpinLindbladNoiseSystemWrapper {
    /// Internal storage of [struqture::spins::SpinLindbladNoiseSystem]
    pub internal: SpinLindbladNoiseSystem,
}

#[noisy_system_wrapper(
    OperateOnSpins,
    OperateOnDensityMatrix,
    ToSparseMatrixSuperOperator,
    Calculus
)]
impl SpinLindbladNoiseSystemWrapper {
    /// Create a new SpinLindbladNoiseSystem.
    ///
    /// Args:
    ///     number_spins (Optional[int]): The number of spins in the SpinLindbladNoiseSystem.
    ///
    /// Returns:
    ///     self: The new SpinLindbladNoiseSystem with the input number of spins.
    #[new]
    #[pyo3(signature = (number_spins = None))]
    pub fn new(number_spins: Option<usize>) -> Self {
        Self {
            internal: SpinLindbladNoiseSystem::new(number_spins),
        }
    }
}
