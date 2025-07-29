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
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator::CalculatorComplex;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
#[cfg(feature = "unstable_struqture_2_import")]
use std::str::FromStr;
use struqture::fermions::FermionHamiltonianSystem;
#[cfg(feature = "unstable_struqture_2_import")]
use struqture::fermions::HermitianFermionProduct;
use struqture::mappings::JordanWignerFermionToSpin;
#[cfg(feature = "json_schema")]
use struqture::{MinSupportedVersion, STRUQTURE_VERSION};
use struqture::{OperateOnDensityMatrix, OperateOnModes, OperateOnState};
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
                                        "FermionHamiltonianSystems could not be multiplied: {err:?}"
                                    ))
                                })?;
                                Ok(FermionSystemWrapper { internal: new_self })
                            },
                            Err(err) => Err(PyValueError::new_err(format!(
                                "The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor FermionHamiltonianSystem: {err:?}")))
                        }
                    }
                }
            }
        }
    }

    /// Converts a json corresponding to struqture 2.x FermionHamiltonian to a struqture 1.x FermionHamiltonianSystem.
    ///
    /// Args:
    ///     input (str): the json of the struqture 2.x FermionHamiltonian to convert to struqture 1.x.
    ///
    /// Returns:
    ///     FermionHamiltonianSystem: The struqture 1.x FermionHamiltonianSystem created from the struqture 2.x FermionHamiltonian.
    ///
    /// Raises:
    ///     ValueError: Input could not be deserialised from json to struqture 2.x.
    ///     ValueError: Struqture 2.x object could not be converted to struqture 1.x.
    #[staticmethod]
    #[cfg(feature = "unstable_struqture_2_import")]
    pub fn from_json_struqture_2(input: String) -> PyResult<FermionHamiltonianSystemWrapper> {
        let operator: struqture_2::fermions::FermionHamiltonian = serde_json::from_str(&input)
            .map_err(|err| {
                PyValueError::new_err(format!(
                    "Input cannot be deserialized from json to struqture 2.x: {}",
                    err
                ))
            })?;
        let mut new_operator = FermionHamiltonianSystem::new(None);
        for (key, val) in struqture_2::OperateOnDensityMatrix::iter(&operator) {
            let self_key = HermitianFermionProduct::from_str(&format!("{}", key).to_string()).map_err(
                |err| {
                    PyValueError::new_err(format!(
                        "Struqture 2.x HermitianFermionProduct cannot be converted to struqture 1.x: {}",
                        err
                    ))
                },
            )?;
            let _ = new_operator.set(self_key, val.clone());
        }
        Ok(FermionHamiltonianSystemWrapper {
            internal: new_operator,
        })
    }
}
