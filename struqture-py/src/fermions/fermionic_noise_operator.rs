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

use crate::fermions::FermionProductWrapper;
use crate::spins::QubitLindbladNoiseOperatorWrapper;
use bincode::deserialize;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::fermions::FermionLindbladNoiseOperator;
use struqture::mappings::JordanWignerFermionToSpin;
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture::{OperateOnDensityMatrix, OperateOnModes};
use struqture_py_macros::{mappings, noisy_system_wrapper};

/// These are representations of noisy systems of fermions.
///
/// In a FermionLindbladNoiseOperator is characterized by a FermionLindbladNoiseOperator to represent the hamiltonian of the system, and an optional number of fermions.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.fermions import FermionLindbladNoiseOperator, FermionProduct
///
///     slns = FermionLindbladNoiseOperator()
///     dp = FermionProduct([0], [1])
///     slns.add_operator_product((dp, dp), 2.0)
///     npt.assert_equal(slns.current_number_modes(), 2)
///     npt.assert_equal(slns.get((dp, dp)), CalculatorComplex(2))
///
#[pyclass(
    name = "FermionLindbladNoiseOperator",
    module = "struqture_py.fermions"
)]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct FermionLindbladNoiseOperatorWrapper {
    /// Internal storage of [struqture::fermions::FermionLindbladNoiseOperator]
    pub internal: FermionLindbladNoiseOperator,
}

#[mappings(JordanWignerFermionToSpin)]
#[noisy_system_wrapper(OperateOnModes, OperateOnFermions, OperateOnDensityMatrix, Calculus)]
impl FermionLindbladNoiseOperatorWrapper {
    /// Create a new FermionLindbladNoiseOperator.
    ///
    /// Returns:
    ///     self: The new FermionLindbladNoiseOperator with the input number of fermions.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: FermionLindbladNoiseOperator::new(),
        }
    }
}
