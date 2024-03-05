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

use super::FermionOperatorWrapper;
use crate::fermions::HermitianFermionProductWrapper;
use crate::spins::SpinHamiltonianWrapper;
use bincode::deserialize;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator::CalculatorComplex;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::fermions::FermionHamiltonian;
use struqture::mappings::JordanWignerFermionToSpin;
#[cfg(feature = "json_schema")]
use struqture::{MinSupportedVersion, STRUQTURE_VERSION};
use struqture::{OperateOnDensityMatrix, OperateOnModes, OperateOnState};
use struqture_py_macros::{mappings, noiseless_system_wrapper};

/// These are representations of systems of fermions.
///
/// FermionHamiltonians are characterized by a FermionOperator to represent the hamiltonian of the spin system
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
///     from struqture_py.fermions import FermionHamiltonian, HermitianFermionProduct
///
///     ssystem = FermionHamiltonian()
///     pp = HermitianFermionProduct([0], [0])
///     ssystem.add_operator_product(pp, 5.0)
///     npt.assert_equal(ssystem.number_modes(), 2)
///     npt.assert_equal(ssystem.get(pp), CalculatorComplex(5))
///     npt.assert_equal(ssystem.keys(), [pp])
///
#[pyclass(name = "FermionHamiltonian", module = "struqture_py.fermions")]
#[derive(Clone, Debug, PartialEq)]
pub struct FermionHamiltonianWrapper {
    /// Internal storage of [struqture::fermions::FermionHamiltonian]
    pub internal: FermionHamiltonian,
}

#[mappings(JordanWignerFermionToSpin)]
#[noiseless_system_wrapper(
    OperateOnFermions,
    OperateOnState,
    OperateOnModes,
    OperateOnDensityMatrix,
    HermitianCalculus
)]
impl FermionHamiltonianWrapper {
    /// Create an empty FermionHamiltonian.
    ///
    /// Returns:
    ///     self: The new FermionHamiltonian with the input number of fermions.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: FermionHamiltonian::new(),
        }
    }

    /// Implement `*` for FermionHamiltonian and FermionHamiltonian/CalculatorComplex/CalculatorFloat.
    ///
    /// Args:
    ///     value (Union[FermionHamiltonian, CalculatorComplex, CalculatorFloat]): value by which to multiply the self FermionHamiltonian
    ///
    /// Returns:
    ///     FermionOperator: The FermionHamiltonian multiplied by the value.
    ///
    /// Raises:
    ///     ValueError: The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor FermionHamiltonian.
    pub fn __mul__(&self, value: &PyAny) -> PyResult<FermionOperatorWrapper> {
        let cf_value = qoqo_calculator_pyo3::convert_into_calculator_float(value);
        match cf_value {
            Ok(x) => Ok(FermionOperatorWrapper {
                internal: (self.clone().internal * CalculatorComplex::from(x))
                    .map_err(|_| PyTypeError::new_err("Operator cannot be multiplied"))?,
            }),
            Err(_) => {
                let cc_value = qoqo_calculator_pyo3::convert_into_calculator_complex(value);
                match cc_value {
                    Ok(x) => Ok(FermionOperatorWrapper {
                        internal: (self.clone().internal * x)
                            .map_err(|_| PyTypeError::new_err("Operator cannot be multiplied"))?,
                    }),
                    Err(_) => {
                        let bhs_value = Self::from_pyany(value);
                        match bhs_value {
                            Ok(x) => {
                                let new_self = (self.clone().internal * x).map_err(|err| {
                                    PyValueError::new_err(format!(
                                        "FermionHamiltonians could not be multiplied: {:?}",
                                        err
                                    ))
                                })?;
                                Ok(FermionOperatorWrapper { internal: new_self })
                            },
                            Err(err) => Err(PyValueError::new_err(format!(
                                "The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor FermionHamiltonian: {:?}",
                                err)))
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

impl Default for FermionHamiltonianWrapper {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for FermionHamiltonianWrapper {
    fn default() -> Self {
        Self::new()
    }
}
