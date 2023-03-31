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

use crate::spins::PlusMinusProductWrapper;
use bincode::deserialize;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::spins::{
    PlusMinusLindbladNoiseOperator, SpinLindbladNoiseOperator, SpinLindbladNoiseSystem,
};
use struqture::OperateOnDensityMatrix;
use struqture_py_macros::noisy_system_wrapper;

use super::SpinLindbladNoiseSystemWrapper;

/// These are representations of noisy systems of spins.
///
/// In a PlusMinusLindbladNoiseOperator is characterized by a SpinLindbladNoiseOperator to represent the hamiltonian of the spin system, and an optional number of spins.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.spins import PlusMinusLindbladNoiseOperator, PlusMinusProduct
///
///     slns = PlusMinusLindbladNoiseOperator()
///     dp = PlusMinusProduct().z(0).x(1)
///     slns.add_operator_product((dp, dp), 2.0)
///     npt.assert_equal(slns.current_number_spins(), 2)
///     npt.assert_equal(slns.get((dp, dp)), CalculatorComplex(2))
///     npt.assert_equal(slns.keys(), [(dp, dp)])
///     dimension = 4**slns.number_spins()
///     matrix = sp.coo_matrix(slns.sparse_matrix_superoperator_coo(), shape=(dimension, dimension))
///
#[pyclass(name = "PlusMinusLindbladNoiseOperator", module = "struqture_py.spins")]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct PlusMinusLindbladNoiseOperatorWrapper {
    /// Internal storage of [struqture::spins::PlusMinusLindbladNoiseOperator]
    pub internal: PlusMinusLindbladNoiseOperator,
}

#[noisy_system_wrapper(OperateOnDensityMatrix)]
impl PlusMinusLindbladNoiseOperatorWrapper {
    /// Create a new PlusMinusLindbladNoiseOperator.
    ///
    /// Returns:
    ///     self: The new PlusMinusLindbladNoiseOperator with the input number of spins.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: PlusMinusLindbladNoiseOperator::new(),
        }
    }
    /// Implement `-1` for self.
    ///
    /// Returns:
    ///     self: The object * -1.
    pub fn __neg__(&self) -> PlusMinusLindbladNoiseOperatorWrapper {
        PlusMinusLindbladNoiseOperatorWrapper {
            internal: -self.clone().internal,
        }
    }

    /// Implement `+` for self with self-type.
    ///
    /// Args:
    ///     other (self): value by which to add to self.
    ///
    /// Returns:
    ///     self: The two objects added.
    ///
    /// Raises:
    ///     ValueError: Objects could not be added.
    pub fn __add__(
        &self,
        other: PlusMinusLindbladNoiseOperatorWrapper,
    ) -> PlusMinusLindbladNoiseOperatorWrapper {
        let new_self = self.clone().internal + other.internal;
        PlusMinusLindbladNoiseOperatorWrapper { internal: new_self }
    }

    /// Implement `-` for self with self-type.
    ///
    /// Args:
    ///     other (self): value by which to subtract from self.
    ///
    /// Returns:
    ///     self: The two objects subtracted.
    ///
    /// Raises:
    ///     ValueError: Objects could not be subtracted.
    pub fn __sub__(
        &self,
        other: PlusMinusLindbladNoiseOperatorWrapper,
    ) -> PlusMinusLindbladNoiseOperatorWrapper {
        let new_self = self.clone().internal - other.internal;
        PlusMinusLindbladNoiseOperatorWrapper { internal: new_self }
    }

    /// Separate self into an operator with the terms of given number of qubits and an operator with the remaining operations
    ///
    /// # Arguments
    ///
    /// * `number_of_spins_left` - Number of spins to filter for in the keys.
    ///
    /// # Returns
    ///
    /// (separated, remainder) - Operator with the noise terms where number_of_spins matches the number of spins the operator product acts on and Operator with all other contributions.
    pub fn separate_into_n_spin_terms(
        &self,
        number_of_spins_left: usize,
        number_of_spins_right: usize,
    ) -> PyResult<(
        PlusMinusLindbladNoiseOperatorWrapper,
        PlusMinusLindbladNoiseOperatorWrapper,
    )> {
        let result = self
            .internal
            .separate_into_n_spin_terms(number_of_spins_left, number_of_spins_right)
            .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?;
        Ok((
            PlusMinusLindbladNoiseOperatorWrapper { internal: result.0 },
            PlusMinusLindbladNoiseOperatorWrapper { internal: result.1 },
        ))
    }

    /// Return the concatenation of two objects of type `self` with no overlapping qubits.
    ///
    /// Args:
    ///     other (self): The object to concatenate self with.
    ///
    /// Returns:
    ///     list[int]: A list of the corresponding creator indices.
    ///
    /// Raises:
    ///     ValueError: The two objects could not be concatenated.
    #[staticmethod]
    pub fn from(value: Py<PyAny>) -> PyResult<PlusMinusLindbladNoiseOperatorWrapper> {
        let system = SpinLindbladNoiseSystemWrapper::from_pyany(value)
            .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?;
        Ok(PlusMinusLindbladNoiseOperatorWrapper {
            internal: PlusMinusLindbladNoiseOperator::from(system.operator().clone()),
        })
    }

    /// Return the concatenation of two objects of type `self` with no overlapping qubits.
    ///
    /// Args:
    ///     other (self): The object to concatenate self with.
    ///
    /// Returns:
    ///     list[int]: A list of the corresponding creator indices.
    ///
    /// Raises:
    ///     ValueError: The two objects could not be concatenated.
    pub fn to_spin_system(&self) -> PyResult<SpinLindbladNoiseSystemWrapper> {
        let result: SpinLindbladNoiseOperator =
            SpinLindbladNoiseOperator::from(self.internal.clone());
        Ok(SpinLindbladNoiseSystemWrapper {
            internal: SpinLindbladNoiseSystem::from_operator(result, None)
                .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?,
        })
    }
}
