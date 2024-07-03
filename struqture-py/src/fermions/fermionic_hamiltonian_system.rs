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

use super::FermionSystemWrapper;
use crate::fermions::HermitianFermionProductWrapper;
use crate::spins::SpinHamiltonianSystemWrapper;
use bincode::deserialize;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator::CalculatorComplex;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use std::str::FromStr;
use struqture::fermions::{FermionHamiltonianSystem, HermitianFermionProduct};
use struqture::mappings::JordanWignerFermionToSpin;
#[cfg(feature = "json_schema")]
use struqture::{MinSupportedVersion, STRUQTURE_VERSION};
use struqture::{OperateOnDensityMatrix, OperateOnModes, OperateOnState, StruqtureError};
use struqture_py_macros::{mappings, noiseless_system_wrapper};

/// These are representations of systems of fermions.
///
/// FermionHamiltonianSystems are characterized by a FermionOperator to represent the hamiltonian of the spin system
/// and an optional number of fermions.
///
/// Args:
///     number_fermions (Optional[int]): The number of fermions in the FermionHamiltonianSystem.
///
/// Returns:
///     self: The new FermionHamiltonianSystem with the input number of fermions.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.fermions import FermionHamiltonianSystem, HermitianFermionProduct
///
///     ssystem = FermionHamiltonianSystem(2)
///     pp = HermitianFermionProduct([0], [0])
///     ssystem.add_operator_product(pp, 5.0)
///     npt.assert_equal(ssystem.number_modes(), 2)
///     npt.assert_equal(ssystem.get(pp), CalculatorComplex(5))
///     npt.assert_equal(ssystem.keys(), [pp])
///
#[pyclass(name = "FermionHamiltonianSystem", module = "struqture_py.fermions")]
#[derive(Clone, Debug, PartialEq)]
pub struct FermionHamiltonianSystemWrapper {
    /// Internal storage of [struqture::fermions::FermionHamiltonianSystem]
    pub internal: FermionHamiltonianSystem,
}

#[mappings(JordanWignerFermionToSpin)]
#[noiseless_system_wrapper(
    OperateOnFermions,
    OperateOnState,
    OperateOnModes,
    OperateOnDensityMatrix,
    Calculus
)]
impl FermionHamiltonianSystemWrapper {
    /// Create an empty FermionHamiltonianSystem.
    ///
    /// Args:
    ///     number_fermions (Optional[int]): The number of fermions in the FermionHamiltonianSystem.
    ///
    /// Returns:
    ///     self: The new FermionHamiltonianSystem with the input number of fermions.
    #[new]
    #[pyo3(signature = (number_fermions = None))]
    pub fn new(number_fermions: Option<usize>) -> Self {
        Self {
            internal: FermionHamiltonianSystem::new(number_fermions),
        }
    }

    /// Implement `*` for FermionHamiltonianSystem and FermionHamiltonianSystem/CalculatorComplex/CalculatorFloat.
    ///
    /// Args:
    ///     value (Union[FermionHamiltonianSystem, CalculatorComplex, CalculatorFloat]): value by which to multiply the self FermionHamiltonianSystem
    ///
    /// Returns:
    ///     FermionSystem: The FermionHamiltonianSystem multiplied by the value.
    ///
    /// Raises:
    ///     ValueError: The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor FermionHamiltonianSystem.
    pub fn __mul__(&self, value: &Bound<PyAny>) -> PyResult<FermionSystemWrapper> {
        let cf_value = qoqo_calculator_pyo3::convert_into_calculator_float(value);
        match cf_value {
            Ok(x) => Ok(FermionSystemWrapper {
                internal: (self.clone().internal * CalculatorComplex::from(x))
                    .map_err(|_| PyTypeError::new_err("System cannot be multiplied"))?,
            }),
            Err(_) => {
                let cc_value = qoqo_calculator_pyo3::convert_into_calculator_complex(value);
                match cc_value {
                    Ok(x) => Ok(FermionSystemWrapper {
                        internal: (self.clone().internal * x)
                            .map_err(|_| PyTypeError::new_err("System cannot be multiplied"))?,
                    }),
                    Err(_) => {
                        let bhs_value = Self::from_pyany(value);
                        match bhs_value {
                            Ok(x) => {
                                let new_self = (self.clone().internal * x).map_err(|err| {
                                    PyValueError::new_err(format!(
                                        "FermionHamiltonianSystems could not be multiplied: {:?}",
                                        err
                                    ))
                                })?;
                                Ok(FermionSystemWrapper { internal: new_self })
                            },
                            Err(err) => Err(PyValueError::new_err(format!(
                                "The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor FermionHamiltonianSystem: {:?}",
                                err)))
                        }
                    }
                }
            }
        }
    }

    // add in a function converting struqture_one (not py) to struqture 2
    // take a pyany, implement from_pyany by hand (or use from_pyany_struqture_one internally) and wrap the result in a struqture 2 spin operator wrapper
    // #[cfg(feature = "struqture_2_import")]
    #[staticmethod]
    pub fn from_struqture_two(input: &Bound<PyAny>) -> PyResult<FermionHamiltonianSystemWrapper> {
        Python::with_gil(|_| -> PyResult<FermionHamiltonianSystemWrapper> {
            let source_serialisation_meta = input.call_method0("_get_serialisation_meta").map_err(|_| {
                PyTypeError::new_err("Trying to use Python object as a struqture-py object that does not behave as struqture-py object. Are you sure you have the right type to all functions?".to_string())
            })?;
            let source_serialisation_meta: String = source_serialisation_meta.extract().map_err(|_| {
                PyTypeError::new_err("Trying to use Python object as a struqture-py object that does not behave as struqture-py object. Are you sure you have the right type to all functions?".to_string())
            })?;

            let source_serialisation_meta: struqture_two::StruqtureSerialisationMeta = serde_json::from_str(&source_serialisation_meta).map_err(|_| {
                PyTypeError::new_err("Trying to use Python object as a struqture-py object that does not behave as struqture-py object. Are you sure you have the right type to all functions?".to_string())
            })?;

            let target_serialisation_meta = <struqture_two::fermions::FermionHamiltonian as struqture_two::SerializationSupport>::target_serialisation_meta();

            struqture_two::check_can_be_deserialised(
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
            let two_import: struqture_two::fermions::FermionHamiltonian = deserialize(&bytes[..])
                .map_err(|err| {
                PyTypeError::new_err(format!("Type conversion failed: {}", err))
            })?;
            let mut fermion_system = FermionHamiltonianSystem::new(None);
            for (key, val) in struqture_two::OperateOnDensityMatrix::iter(&two_import) {
                let value_string = key.to_string();
                let self_key = HermitianFermionProduct::from_str(&value_string).map_err(
                    |_err: StruqtureError| PyValueError::new_err(
                        "Trying to obtain struqture 1.x FermionHamiltonianSystem from struqture 2.x FermionHamiltonian. Conversion failed. Was the right type passed to all functions?".to_string()
                ))?;

                let _ = fermion_system.set(self_key, val.clone());
            }

            Ok(FermionHamiltonianSystemWrapper {
                internal: fermion_system,
            })
        })
    }
}
