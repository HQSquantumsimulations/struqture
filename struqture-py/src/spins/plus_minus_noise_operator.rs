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

use super::QubitLindbladNoiseOperatorWrapper;
use crate::fermions::FermionLindbladNoiseOperatorWrapper;
use crate::spins::PlusMinusProductWrapper;
use bincode::deserialize;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::mappings::JordanWignerSpinToFermion;
use struqture::spins::{PlusMinusLindbladNoiseOperator, QubitLindbladNoiseOperator};
use struqture::OperateOnDensityMatrix;
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture_py_macros::{mappings, noisy_system_wrapper};

/// These are representations of noisy systems of spins.
///
/// In a PlusMinusLindbladNoiseOperator is characterized by a QubitLindbladNoiseOperator to represent the hamiltonian of the spin system, and an optional number of spins.
///
/// Returns:
///     self: The new PlusMinusLindbladNoiseOperator.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.spins import PlusMinusLindbladNoiseOperator, PlusMinusProduct
///
///     slns = PlusMinusLindbladNoiseOperator()
///     dp = PlusMinusProduct().z(0).plus(1)
///     slns.add_operator_product((dp, dp), 2.0)
///     npt.assert_equal(slns.get((dp, dp)), CalculatorComplex(2))
///     npt.assert_equal(slns.keys(), [(dp, dp)])
///
#[pyclass(name = "PlusMinusLindbladNoiseOperator", module = "struqture_py.spins")]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct PlusMinusLindbladNoiseOperatorWrapper {
    /// Internal storage of [struqture::spins::PlusMinusLindbladNoiseOperator]
    pub internal: PlusMinusLindbladNoiseOperator,
}

#[mappings(JordanWignerSpinToFermion)]
#[noisy_system_wrapper(OperateOnDensityMatrix, Calculus)]
impl PlusMinusLindbladNoiseOperatorWrapper {
    /// Create a new PlusMinusLindbladNoiseOperator.
    ///
    /// Returns:
    ///     self: The new PlusMinusLindbladNoiseOperator.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: PlusMinusLindbladNoiseOperator::new(),
        }
    }

    /// Convert a QubitLindbladNoiseOperator into a PlusMinusLindbladNoiseOperator.
    ///
    /// Args:
    ///     value (QubitLindbladNoiseOperator): The QubitLindbladNoiseOperator to create the PlusMinusLindbladNoiseOperator from.
    ///
    /// Returns:
    ///     PlusMinusLindbladNoiseOperator: The operator created from the input QubitLindbladNoiseOperator.
    ///
    /// Raises:
    ///     ValueError: Could not create QubitLindbladNoiseOperator from input.
    #[staticmethod]
    pub fn from_qubit_noise_operator(
        value: &Bound<PyAny>,
    ) -> PyResult<PlusMinusLindbladNoiseOperatorWrapper> {
        let system = QubitLindbladNoiseOperatorWrapper::from_pyany(value)
            .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?;
        Ok(PlusMinusLindbladNoiseOperatorWrapper {
            internal: PlusMinusLindbladNoiseOperator::from(system.clone()),
        })
    }

    /// Convert a PlusMinusLindbladNoiseOperator into a QubitLindbladNoiseOperator.
    ///
    /// Returns:
    ///     QubitLindbladNoiseOperator: The operator created from the input PlusMinusLindbladNoiseOperator and optional number of spins.
    ///
    /// Raises:
    ///     ValueError: Could not create QubitLindbladNoiseOperator from PlusMinusLindbladNoiseOperator.
    pub fn to_qubit_noise_operator(&self) -> PyResult<QubitLindbladNoiseOperatorWrapper> {
        let result: QubitLindbladNoiseOperator =
            QubitLindbladNoiseOperator::from(self.internal.clone());
        Ok(QubitLindbladNoiseOperatorWrapper { internal: result })
    }
}
