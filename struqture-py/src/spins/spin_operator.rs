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

use crate::fermions::FermionOperatorWrapper;
use crate::spins::PauliProductWrapper;
use crate::{to_py_coo, PyCooMatrix};
use bincode::deserialize;
use num_complex::Complex64;
use pyo3::exceptions::{PyRuntimeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator::CalculatorComplex;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
#[cfg(feature = "unstable_struqture_2_import")]
use std::str::FromStr;
use struqture::mappings::JordanWignerSpinToFermion;
#[cfg(feature = "unstable_struqture_2_import")]
use struqture::spins::PauliProduct;
use struqture::spins::{
    OperateOnSpins, SpinOperator, ToSparseMatrixOperator, ToSparseMatrixSuperOperator,
};
use struqture::StruqtureError;
#[cfg(feature = "json_schema")]
use struqture::{MinSupportedVersion, STRUQTURE_VERSION};
use struqture::{OperateOnDensityMatrix, OperateOnState};
use struqture_py_macros::{mappings, noiseless_system_wrapper};

/// These are representations of systems of spins.
///
/// SpinOperators are characterized by a SpinOperator to represent the hamiltonian of the spin system
/// and an optional number of spins.
///
///  Args:
///     number_spins (Optional[int]): The number of spins in the SpinSystem.
///
/// Returns:
///     self: The new SpinSystem with the input number of spins.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.spins import SpinOperator, PauliProduct
///
///     ssystem = SpinOperator(2)
///     pp = PauliProduct().z(0)
///     ssystem.add_operator_product(pp, 5.0)
///     npt.assert_equal(ssystem.number_spins(), 2)
///     npt.assert_equal(ssystem.get(pp), CalculatorComplex(5))
///     npt.assert_equal(ssystem.keys(), [pp])
///     dimension = 4**ssystem.number_spins()
///     matrix = sp.coo_matrix(ssystem.sparse_matrix_superoperator_coo(), shape=(dimension, dimension))
///
#[pyclass(name = "SpinOperator", module = "struqture_py.spins")]
#[derive(Clone, Debug, PartialEq)]
pub struct SpinOperatorWrapper {
    /// Internal storage of [struqture::spins::SpinOperator]
    pub internal: SpinOperator,
}

#[mappings(JordanWignerSpinToFermion)]
#[noiseless_system_wrapper(
    OperateOnSpins,
    OperateOnState,
    ToSparseMatrixOperator,
    ToSparseMatrixSuperOperator,
    OperateOnDensityMatrix,
    Calculus
)]
impl SpinOperatorWrapper {
    /// Create an empty SpinOperator.
    ///
    /// Returns:
    ///     self: The new SpinOperator with the input number of spins.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: SpinOperator::new(),
        }
    }

    /// Fallible conversion of generic python object that is implemented in struqture 1.x.
    #[cfg(feature = "struqture_1_import")]
    pub fn from_pyany_struquture_one(input: Py<PyAny>) -> PyResult<SpinOperator> {
        Python::with_gil(|py| -> PyResult<SpinOperator> {
            let one_import = struqture_one_py::spins::SpinSystemWrapper::from_pyany(input)?;
            let spin_operator = SpinOperator::from_struqture_1(&one_import).map_err(
                |err| PyValueError::new_err(format!("Trying to obtain struqture 2.x SpinOperator from struqture 1.x SpinSystem. Conversion failed. Was the right type passed to all functions?")
            ))?;
            Ok(spin_operator)
        })
    }

    /// Implement `*` for SpinOperator and SpinOperator/CalculatorComplex/CalculatorFloat.
    ///
    /// Args:
    ///     value (Union[SpinOperator, CalculatorComplex, CalculatorFloat]): value by which to multiply the self SpinOperator
    ///
    /// Returns:
    ///     SpinOperator: The SpinOperator multiplied by the value.
    ///
    /// Raises:
    ///     ValueError: The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor SpinOperator.
    pub fn __mul__(&self, value: &PyAny) -> PyResult<Self> {
        let cf_value = qoqo_calculator_pyo3::convert_into_calculator_float(value);
        match cf_value {
            Ok(x) => Ok(Self {
                internal: self.clone().internal * CalculatorComplex::from(x),
            }),
            Err(_) => {
                let cc_value = qoqo_calculator_pyo3::convert_into_calculator_complex(value);
                match cc_value {
                    Ok(x) => Ok(Self {
                        internal: self.clone().internal * x,
                    }),
                    Err(_) => {
                        let bhs_value = Self::from_pyany(value);
                        match bhs_value {
                            Ok(x) => {
                                let new_self = self.clone().internal * x;
                                Ok(Self { internal: new_self })
                            },
                            Err(err) => Err(PyValueError::new_err(format!(
                                "The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor SpinOperator: {:?}",
                                err)))
                        }
                    }
                }
            }
        }
    }

    /// Converts a json struqture 2.x PauliOperator to a struqture 1.x SpinSystem.
    ///
    /// Args:
    ///     input (PauliOperator): The struqture 2.x PauliOperator to convert to struqture 1.x.
    ///
    /// Returns:
    ///     SpinSystem: The struqture 1.x SpinSystem created from the struqture 2.x PauliOperator.
    ///
    /// Raises:
    ///     TypeError: If the input is not a struqture 2.x PauliOperator.
    ///     ValueError: Conversion failed.
    #[staticmethod]
    #[cfg(feature = "unstable_struqture_2_import")]
    pub fn from_json_struqture_2(input: String) -> PyResult<SpinSystemWrapper> {
        let operator: struqture_2::spins::PauliOperator =
            serde_json::from_str(&input).map_err(|err| {
                PyValueError::new_err(format!(
                    "Input cannot be deserialized from json to struqture 2.x: {}",
                    err
                ))
            })?;
        let mut new_operator = SpinSystem::new(None);
        for (key, val) in struqture_2::OperateOnDensityMatrix::iter(&operator) {
            let self_key =
                PauliProduct::from_str(&format!("{}", key).to_string()).map_err(|err| {
                    PyValueError::new_err(format!(
                        "Struqture 2.x PauliProduct cannot be converted to struqture 1.x: {}",
                        err
                    ))
                })?;
            let _ = new_operator.set(self_key, val.clone());
        }
        Ok(SpinSystemWrapper {
            internal: new_operator,
        })
    }
}

impl Default for SpinOperatorWrapper {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SpinOperatorWrapper {
    fn default() -> Self {
        Self::new()
    }
}
