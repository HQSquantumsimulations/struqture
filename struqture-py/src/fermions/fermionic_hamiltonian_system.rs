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
use struqture::fermions::FermionHamiltonianSystem;
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
    pub fn __mul__(&self, value: &PyAny) -> PyResult<FermionSystemWrapper> {
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
                        let bhs_value = Self::from_pyany(value.into());
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
}
