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
use qoqo_calculator::CalculatorComplex;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::bosons::BosonOperator;
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture::{OperateOnDensityMatrix, OperateOnModes, OperateOnState};
use struqture_py_macros::noiseless_system_wrapper;

/// These are representations of systems of bosons.
///
/// BosonOperators are characterized by a BosonOperator to represent the hamiltonian of the spin system
/// and an optional number of bosons.
///
/// Returns:
///     self: The new BosonSystem with the input number of bosons.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.bosons import BosonOperator, BosonProduct
///
///     ssystem = BosonOperator(2)
///     pp = BosonProduct([0], [1])
///     ssystem.add_operator_product(pp, 5.0)
///     npt.assert_equal(ssystem.current_number_modes(), 2)
///     npt.assert_equal(ssystem.get(pp), CalculatorComplex(5))
///     npt.assert_equal(ssystem.keys(), [pp])
///
#[pyclass(name = "BosonOperator", module = "struqture_py.bosons")]
#[derive(Clone, Debug, PartialEq)]
pub struct BosonOperatorWrapper {
    /// Internal storage of [struqture::bosons::BosonOperator]
    pub internal: BosonOperator,
}

#[noiseless_system_wrapper(
    OperateOnBosons,
    OperateOnState,
    OperateOnModes,
    OperateOnDensityMatrix,
    Calculus
)]
impl BosonOperatorWrapper {
    /// Create an empty BosonOperator.
    ///
    /// Returns:
    ///     self: The new BosonOperator with the input number of bosons.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: BosonOperator::new(),
        }
    }

    /// Implement `*` for BosonOperator and BosonOperator/CalculatorComplex/CalculatorFloat.
    ///
    /// Args:
    ///     value (Union[BosonOperator, CalculatorComplex, CalculatorFloat]): value by which to multiply the self BosonOperator
    ///
    /// Returns:
    ///     BosonOperator: The BosonOperator multiplied by the value.
    ///
    /// Raises:
    ///     ValueError: The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor BosonOperator.
    pub fn __mul__(&self, value: &Bound<PyAny>) -> PyResult<Self> {
        let cf_value = qoqo_calculator_pyo3::convert_into_calculator_float(value);
        match cf_value {
            Ok(x) => Ok(BosonOperatorWrapper {
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
                                "The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor BosonOperator: {:?}",
                                err)))
                        }
                    }
                }
            }
        }
    }
}

impl Default for BosonOperatorWrapper {
    fn default() -> Self {
        Self::new()
    }
}
