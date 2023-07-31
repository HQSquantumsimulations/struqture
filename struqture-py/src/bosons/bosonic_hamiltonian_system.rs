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

use crate::bosons::{BosonSystemWrapper, HermitianBosonProductWrapper};
use bincode::deserialize;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator::CalculatorComplex;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::bosons::BosonHamiltonianSystem;
#[cfg(feature = "json_schema")]
use struqture::{MinSupportedVersion, STRUQTURE_VERSION};
use struqture::{OperateOnDensityMatrix, OperateOnModes, OperateOnState};
use struqture_py_macros::noiseless_system_wrapper;

/// These are representations of systems of bosons.
///
/// BosonHamiltonianSystems are characterized by a BosonOperator to represent the hamiltonian of the spin system
/// and an optional number of bosons.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.bosons import BosonHamiltonianSystem, HermitianBosonProduct
///     
///     ssystem = BosonHamiltonianSystem(2)
///     pp = HermitianBosonProduct([0], [0])
///     ssystem.add_operator_product(pp, 5.0)
///     npt.assert_equal(ssystem.number_modes(), 2)
///     npt.assert_equal(ssystem.get(pp), CalculatorComplex(5))
///     npt.assert_equal(ssystem.keys(), [pp])
///
#[pyclass(name = "BosonHamiltonianSystem", module = "struqture_py.bosons")]
#[derive(Clone, Debug, PartialEq)]
pub struct BosonHamiltonianSystemWrapper {
    /// Internal storage of [struqture::bosons::BosonHamiltonianSystem]
    pub internal: BosonHamiltonianSystem,
}

#[noiseless_system_wrapper(
    OperateOnBosons,
    OperateOnState,
    OperateOnModes,
    OperateOnDensityMatrix,
    Calculus
)]
impl BosonHamiltonianSystemWrapper {
    /// Create an empty BosonHamiltonianSystem.
    ///
    /// Args:
    ///     number_bosons (Optional[int]): The number of bosons in the BosonHamiltonianSystem.
    ///
    /// Returns:
    ///     self: The new BosonHamiltonianSystem with the input number of bosons.
    #[new]
    #[pyo3(signature = (number_bosons = None))]
    pub fn new(number_bosons: Option<usize>) -> Self {
        Self {
            internal: BosonHamiltonianSystem::new(number_bosons),
        }
    }

    /// Implement `*` for BosonHamiltonianSystem and BosonHamiltonianSystem/CalculatorComplex/CalculatorFloat.
    ///
    /// Args:
    ///     value (Union[BosonHamiltonianSystem, CalculatorComplex, CalculatorFloat]): value by which to multiply the self BosonHamiltonianSystem
    ///
    /// Returns:
    ///     BosonSystem: The BosonHamiltonianSystem multiplied by the value.
    ///
    /// Raises:
    ///     ValueError: The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor BosonHamiltonianSystem.
    pub fn __mul__(&self, value: &PyAny) -> PyResult<BosonSystemWrapper> {
        let cf_value = qoqo_calculator_pyo3::convert_into_calculator_float(value);
        match cf_value {
            Ok(x) => Ok(BosonSystemWrapper {
                internal: (self.clone().internal * CalculatorComplex::from(x)),
            }),
            Err(_) => {
                let cc_value = qoqo_calculator_pyo3::convert_into_calculator_complex(value);
                match cc_value {
                    Ok(x) => Ok(BosonSystemWrapper {
                        internal: (self.clone().internal * x),
                    }),
                    Err(_) => {
                        let bhs_value = Self::from_pyany(value.into());
                        match bhs_value {
                            Ok(x) => {
                                let new_self = (self.clone().internal * x).map_err(|err| {
                                    PyValueError::new_err(format!(
                                        "BosonHamiltonianSystems could not be multiplied: {:?}",
                                        err
                                    ))
                                })?;
                                Ok(BosonSystemWrapper { internal: new_self })
                            },
                            Err(err) => Err(PyValueError::new_err(format!(
                                "The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor BosonHamiltonianSystem: {:?}",
                                err))),
                        }
                    }
                }
            }
        }
    }
}
