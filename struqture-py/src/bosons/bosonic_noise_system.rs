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
use struqture::bosons::BosonLindbladNoiseSystem;
#[cfg(feature = "json_schema")]
use struqture::{MinSupportedVersion, STRUQTURE_VERSION};
use struqture::{OperateOnDensityMatrix, OperateOnModes};
use struqture_py_macros::noisy_system_wrapper;

/// These are representations of noisy systems of bosons.
///
/// In a BosonLindbladNoiseSystem is characterized by a BosonLindbladNoiseOperator to represent the hamiltonian of the system, and an optional number of bosons.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.bosons import BosonLindbladNoiseSystem, BosonProduct
///
///     slns = BosonLindbladNoiseSystem()
///     dp = BosonProduct([0], [1])
///     slns.add_operator_product((dp, dp), 2.0)
///     npt.assert_equal(slns.current_number_modes(), 2)
///     npt.assert_equal(slns.get((dp, dp)), CalculatorComplex(2))
///
#[pyclass(name = "BosonLindbladNoiseSystem", module = "struqture_py.bosons")]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct BosonLindbladNoiseSystemWrapper {
    /// Internal storage of [struqture::bosons::BosonLindbladNoiseSystem]
    pub internal: BosonLindbladNoiseSystem,
}

#[noisy_system_wrapper(OperateOnModes, OperateOnBosons, OperateOnDensityMatrix, Calculus)]
impl BosonLindbladNoiseSystemWrapper {
    /// Create a new BosonLindbladNoiseSystem.
    ///
    /// Args:
    ///     number_bosons (Optional(int)): The number of bosons in the BosonLindbladNoiseSystem.
    ///
    /// Returns:
    ///     self: The new BosonLindbladNoiseSystem with the input number of bosons.
    #[new]
    #[pyo3(signature = (number_bosons = None))]
    pub fn new(number_bosons: Option<usize>) -> Self {
        Self {
            internal: BosonLindbladNoiseSystem::new(number_bosons),
        }
    }

    /// Separate self into an operator with the terms of given number of creation and annihilation operators and an operator with the remaining operations.
    ///
    /// Args:
    ///     number_creators_annihilators_left (Tuple[int, int]): Number of creators and number of annihilators to filter for in the left term of the keys.
    ///     number_creators_annihilators_right (Tuple[int, int]): Number of creators and number of annihilators to filter for in the right term of the keys.
    ///
    /// Returns:
    ///     Tuple[BosonLindbladNoiseSystem, BosonLindbladNoiseSystem]: Operator with the noise terms where the number of creation and annihilation operators matches the number of spins the operator product acts on and Operator with all other contributions.
    ///
    /// Raises:
    ///     ValueError: Error in adding terms to return values.
    pub fn separate_into_n_terms(
        &self,
        number_creators_annihilators_left: (usize, usize),
        number_creators_annihilators_right: (usize, usize),
    ) -> PyResult<(Self, Self)> {
        let (separated, remainder) = self
            .internal
            .separate_into_n_terms(
                number_creators_annihilators_left,
                number_creators_annihilators_right,
            )
            .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?;
        Ok((
            Self {
                internal: separated,
            },
            Self {
                internal: remainder,
            },
        ))
    }
}
