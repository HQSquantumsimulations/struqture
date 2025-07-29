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

use crate::fermions::FermionLindbladNoiseSystemWrapper;
use crate::spins::DecoherenceProductWrapper;
use crate::{to_py_coo, PyCooMatrix};
use num_complex::Complex64;
use pyo3::exceptions::{PyRuntimeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
#[cfg(feature = "unstable_struqture_2_import")]
use std::str::FromStr;
use struqture::mappings::JordanWignerSpinToFermion;
#[cfg(feature = "unstable_struqture_2_import")]
use struqture::spins::DecoherenceProduct;
use struqture::spins::{OperateOnSpins, SpinLindbladNoiseSystem, ToSparseMatrixSuperOperator};
#[cfg(feature = "json_schema")]
use struqture::{MinSupportedVersion, STRUQTURE_VERSION};
use struqture::{OperateOnDensityMatrix, StruqtureError};
use struqture_py_macros::{mappings, noisy_system_wrapper};
/// These are representations of noisy systems of spins.
///
/// In a SpinLindbladNoiseSystem is characterized by a SpinLindbladNoiseOperator to represent the hamiltonian of the spin system, and an optional number of spins.
///  
/// Args:
///     number_spins (Optional[int]): The number of spins in the SpinLindbladNoiseSystem.
///
/// Returns:
///     self: The new SpinLindbladNoiseSystem with the input number of spins.
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

#[mappings(JordanWignerSpinToFermion)]
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

    /// Separate self into an operator with the terms of given number of spins and an operator with the remaining operations.
    ///
    /// Args:
    ///     number_spins_left (int): Number of spins to filter for in the left term of the keys.
    ///     number_spins_right (int): Number of spins to filter for in the right term of the keys.
    ///
    /// Returns:
    ///     Tuple[SpinLindbladNoiseSystem, SpinLindbladNoiseSystem]: Operator with the noise terms where the number of spins matches the number of spins the operator product acts on and Operator with all other contributions.
    ///
    /// Raises:
    ///     ValueError: Error in adding terms to return values.
    pub fn separate_into_n_terms(
        &self,
        number_spins_left: usize,
        number_spins_right: usize,
    ) -> PyResult<(Self, Self)> {
        let (separated, remainder) = self
            .internal
            .separate_into_n_terms(number_spins_left, number_spins_right)
            .map_err(|err| PyValueError::new_err(format!("{err:?}")))?;
        Ok((
            Self {
                internal: separated,
            },
            Self {
                internal: remainder,
            },
        ))
    }

    /// Converts a json struqture 2.x PauliLindbladNoiseOperator to a struqture 1.x SpinLindbladNoiseSystem.
    ///
    /// Args:
    ///     input (PauliLindbladNoiseOperator): The struqture 2.x PauliLindbladNoiseOperator to convert to struqture 1.x.
    ///
    /// Returns:
    ///     SpinLindbladNoiseSystem: The struqture 1.x SpinLindbladNoiseSystem created from the struqture 2.x PauliLindbladNoiseOperator.
    ///
    /// Raises:
    ///     TypeError: If the input is not a struqture 2.x PauliLindbladNoiseOperator.
    ///     ValueError: Conversion failed.
    #[staticmethod]
    #[cfg(feature = "unstable_struqture_2_import")]
    pub fn from_json_struqture_2(input: String) -> PyResult<SpinLindbladNoiseSystemWrapper> {
        let operator: struqture_2::spins::PauliLindbladNoiseOperator = serde_json::from_str(&input)
            .map_err(|err| {
                PyValueError::new_err(format!(
                    "Input cannot be deserialized from json to struqture 2.x: {}",
                    err
                ))
            })?;
        let mut new_operator = SpinLindbladNoiseSystem::new(None);
        for (key, val) in struqture_2::OperateOnDensityMatrix::iter(&operator) {
            let self_key_left = DecoherenceProduct::from_str(&format!("{}", key.0).to_string())
                .map_err(|err| {
                    PyValueError::new_err(format!(
                        "Struqture 2.x DecoherenceProduct cannot be converted to struqture 1.x: {}",
                        err
                    ))
                })?;
            let self_key_right = DecoherenceProduct::from_str(&format!("{}", key.1).to_string())
                .map_err(|err| {
                    PyValueError::new_err(format!(
                        "Struqture 2.x DecoherenceProduct cannot be converted to struqture 1.x: {}",
                        err
                    ))
                })?;
            let _ = new_operator.set((self_key_left, self_key_right), val.clone());
        }
        Ok(SpinLindbladNoiseSystemWrapper {
            internal: new_operator,
        })
    }
}
