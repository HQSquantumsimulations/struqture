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

use super::{
    BosonHamiltonianWrapper, BosonLindbladNoiseOperatorWrapper, BosonProductWrapper,
    HermitianBosonProductWrapper,
};
use bincode::deserialize;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::bosons::BosonLindbladOpenSystem;
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture::{OpenSystem, OperateOnDensityMatrix, OperateOnModes};
use struqture_py_macros::noisy_system_wrapper;

/// These are representations of noisy systems of bosons.
///
/// In a BosonLindbladOpenSystem is characterized by a BosonLindbladOpenOperator to represent the hamiltonian of the system, and an optional number of bosons.
///
/// Returns:
///     self: The new BosonLindbladOpenSystem with the input number of bosons.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex, CalculatorFloat
///     from struqture_py.bosons import BosonLindbladOpenSystem, BosonProduct
///
///     slns = BosonLindbladOpenSystem()
///     dp = BosonProduct([0], [1])
///     slns.system_add_operator_product(dp, 2.0)
///     npt.assert_equal(slns.current_number_modes(), 2)
///     npt.assert_equal(slns.system().get(dp), CalculatorFloat(2))
///
#[pyclass(name = "BosonLindbladOpenSystem", module = "struqture_py.bosons")]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct BosonLindbladOpenSystemWrapper {
    /// Internal storage of [struqture::bosons::BosonLindbladOpenSystem]
    pub internal: BosonLindbladOpenSystem,
}

#[noisy_system_wrapper(OpenSystem, OperateOnModes, HermitianCalculus)]
impl BosonLindbladOpenSystemWrapper {
    /// Create a new BosonLindbladOpenSystem.
    ///
    /// Returns:
    ///     self: The new BosonLindbladOpenSystem with the input number of bosons.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: BosonLindbladOpenSystem::new(),
        }
    }
}
