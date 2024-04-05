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

use crate::bosons::BosonProductWrapper;
use bincode::deserialize;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::bosons::BosonLindbladNoiseOperator;
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture::{OperateOnDensityMatrix, OperateOnModes};
use struqture_py_macros::noisy_system_wrapper;

/// These are representations of noisy systems of bosons.
///
/// In a BosonLindbladNoiseOperator is characterized by a BosonLindbladNoiseOperator to represent the hamiltonian of the system, and an optional number of bosons.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.bosons import BosonLindbladNoiseOperator, BosonProduct
///
///     slns = BosonLindbladNoiseOperator()
///     dp = BosonProduct([0], [1])
///     slns.add_operator_product((dp, dp), 2.0)
///     npt.assert_equal(slns.current_number_modes(), 2)
///     npt.assert_equal(slns.get((dp, dp)), CalculatorComplex(2))
///
#[pyclass(name = "BosonLindbladNoiseOperator", module = "struqture_py.bosons")]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct BosonLindbladNoiseOperatorWrapper {
    /// Internal storage of [struqture::bosons::BosonLindbladNoiseOperator]
    pub internal: BosonLindbladNoiseOperator,
}

#[noisy_system_wrapper(OperateOnModes, OperateOnBosons, OperateOnDensityMatrix, Calculus)]
impl BosonLindbladNoiseOperatorWrapper {
    /// Create a new BosonLindbladNoiseOperator.
    ///
    /// Returns:
    ///     self: The new BosonLindbladNoiseOperator with the input number of bosons.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: BosonLindbladNoiseOperator::new(),
        }
    }
}
