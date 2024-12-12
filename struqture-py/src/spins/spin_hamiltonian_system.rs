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

use super::SpinSystemWrapper;
use crate::fermions::FermionHamiltonianSystemWrapper;
use crate::spins::PauliProductWrapper;
use crate::{to_py_coo, PyCooMatrix};
use bincode::deserialize;
use num_complex::Complex64;
use pyo3::exceptions::{PyRuntimeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator::CalculatorComplex;
use qoqo_calculator_pyo3::CalculatorFloatWrapper;
#[cfg(feature = "unstable_struqture_2_import")]
use std::str::FromStr;
use struqture::mappings::JordanWignerSpinToFermion;
#[cfg(feature = "unstable_struqture_2_import")]
use struqture::spins::PauliProduct;
use struqture::spins::{
    OperateOnSpins, SpinHamiltonianSystem, ToSparseMatrixOperator, ToSparseMatrixSuperOperator,
};
use struqture::StruqtureError;
#[cfg(feature = "json_schema")]
use struqture::{MinSupportedVersion, STRUQTURE_VERSION};
use struqture::{OperateOnDensityMatrix, OperateOnState};
use struqture_py_macros::{mappings, noiseless_system_wrapper};
/// These are representations of systems of spins.
///
/// SpinHamiltonianSystems are characterized by a SpinOperator to represent the hamiltonian of the spin system
/// and an optional number of spins.
///
/// Args:
///     number_spins (Optional[int]): The number of spins in the SpinHamiltonianSystem.
///
/// Returns:
///     self: The new SpinHamiltonianSystem with the input number of spins.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.spins import SpinHamiltonianSystem, PauliProduct
///
///     ssystem = SpinHamiltonianSystem(2)
///     pp = PauliProduct().z(0)
///     ssystem.add_operator_product(pp, 5.0)
///     npt.assert_equal(ssystem.number_spins(), 2)
///     npt.assert_equal(ssystem.get(pp), CalculatorComplex(5))
///     npt.assert_equal(ssystem.keys(), [pp])
///     dimension = 4**ssystem.number_spins()
///     matrix = sp.coo_matrix(ssystem.sparse_matrix_superoperator_coo(), shape=(dimension, dimension))
///
#[pyclass(name = "SpinHamiltonianSystem", module = "struqture_py.spins")]
#[derive(Clone, Debug, PartialEq)]
pub struct SpinHamiltonianSystemWrapper {
    /// Internal storage of [struqture::spins::SpinHamiltonianSystem]
    pub internal: SpinHamiltonianSystem,
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
impl SpinHamiltonianSystemWrapper {
    /// Create an empty SpinHamiltonianSystem.
    ///
    /// Args:
    ///     number_spins (Optional[int]): The number of spins in the SpinHamiltonianSystem.
    ///
    /// Returns:
    ///     self: The new SpinHamiltonianSystem with the input number of spins.
    #[new]
    #[pyo3(signature = (number_spins = None))]
    pub fn new(number_spins: Option<usize>) -> Self {
        Self {
            internal: SpinHamiltonianSystem::new(number_spins),
        }
    }

    /// Implement `*` for SpinHamiltonianSystem and SpinHamiltonianSystem/CalculatorComplex/CalculatorFloat.
    ///
    /// Args:
    ///     value (Union[SpinHamiltonianSystem, CalculatorComplex, CalculatorFloat]): value by which to multiply the self SpinHamiltonianSystem
    ///
    /// Returns:
    ///     SpinSystem: The SpinHamiltonianSystem multiplied by the value.
    ///
    /// Raises:
    ///     ValueError: The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor SpinHamiltonianSystem.
    pub fn __mul__(&self, value: &Bound<PyAny>) -> PyResult<SpinSystemWrapper> {
        let cf_value = qoqo_calculator_pyo3::convert_into_calculator_float(value);
        match cf_value {
            Ok(x) => Ok(SpinSystemWrapper {
                internal: self.clone().internal * CalculatorComplex::from(x),
            }),
            Err(_) => {
                let cc_value = qoqo_calculator_pyo3::convert_into_calculator_complex(value);
                match cc_value {
                    Ok(x) => Ok(SpinSystemWrapper {
                        internal: self.clone().internal * x,
                    }),
                    Err(_) => {
                        let bhs_value = Self::from_pyany(value);
                        match bhs_value {
                            Ok(x) => {
                                let new_self = (self.clone().internal * x).map_err(|err| {
                                    PyValueError::new_err(format!(
                                        "SpinHamiltonianSystems could not be multiplied: {:?}",
                                        err
                                    ))
                                })?;
                                Ok(SpinSystemWrapper { internal: new_self })
                            },
                            Err(err) => Err(PyValueError::new_err(format!(
                                "The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor SpinHamiltonianSystem: {:?}",
                                err)))
                        }
                    }
                }
            }
        }
    }

    /// Converts a struqture 2.x QubitHamiltonian to a struqture 1.x SpinHamiltonianSystem.
    ///
    /// Args:
    ///     input (QubitHamiltonian): The struqture 2.x QubitHamiltonian to convert to struqture 1.x.
    ///
    /// Returns:
    ///     SpinHamiltonianSystem: The struqture 1.x SpinHamiltonianSystem created from the struqture 2.x QubitHamiltonian.
    ///
    /// Raises:
    ///     TypeError: If the input is not a struqture 2.x QubitHamiltonian.
    ///     ValueError: Conversion failed.
    #[staticmethod]
    #[cfg(feature = "unstable_struqture_2_import")]
    pub fn from_struqture_2(input: &Bound<PyAny>) -> PyResult<SpinHamiltonianSystemWrapper> {
        Python::with_gil(|_| -> PyResult<SpinHamiltonianSystemWrapper> {
            let error_message = "Trying to use Python object as a struqture-py object that does not behave as struqture-py object. Are you sure you have the right type?".to_string();
            let source_serialisation_meta = input
                .call_method0("_get_serialisation_meta")
                .map_err(|_| PyTypeError::new_err(error_message.clone()))?;
            let source_serialisation_meta: String = source_serialisation_meta
                .extract()
                .map_err(|_| PyTypeError::new_err(error_message.clone()))?;

            let source_serialisation_meta: struqture_2::StruqtureSerialisationMeta =
                serde_json::from_str(&source_serialisation_meta)
                    .map_err(|_| PyTypeError::new_err(error_message))?;

            let target_serialisation_meta = <struqture_2::spins::QubitHamiltonian as struqture_2::SerializationSupport>::target_serialisation_meta();

            struqture_2::check_can_be_deserialised(
                &target_serialisation_meta,
                &source_serialisation_meta,
            )
            .map_err(|err| PyTypeError::new_err(err.to_string()))?;

            let get_bytes = input
                .call_method0("to_bincode")
                .map_err(|_| PyTypeError::new_err("Serialisation failed".to_string()))?;
            let bytes = get_bytes
                .extract::<Vec<u8>>()
                .map_err(|_| PyTypeError::new_err("Deserialisation failed".to_string()))?;
            let two_import: struqture_2::spins::QubitHamiltonian = deserialize(&bytes[..])
                .map_err(|err| PyTypeError::new_err(format!("Type conversion failed: {}", err)))?;
            let mut spin_system = SpinHamiltonianSystem::new(None);
            for (key, val) in struqture_2::OperateOnDensityMatrix::iter(&two_import) {
                let value_string = key.to_string();
                let self_key = PauliProduct::from_str(&value_string).map_err(
                    |_err: StruqtureError| PyValueError::new_err(
                        "Trying to obtain struqture 1.x PauliProduct from struqture 2.x PauliProduct. Conversion failed. Was the right type passed to all functions?".to_string()
                ))?;

                spin_system
                    .set(self_key, val.clone())
                    .map_err(|_err: StruqtureError| {
                        PyValueError::new_err(
                            "Could not set key in resulting 1.x SpinHamiltonianSystem".to_string(),
                        )
                    })?;
            }

            Ok(SpinHamiltonianSystemWrapper {
                internal: spin_system,
            })
        })
    }
}
