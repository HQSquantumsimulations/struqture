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

use crate::fermions::FermionProductWrapper;
use crate::spins::PauliOperatorWrapper;
use bincode::deserialize;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator::CalculatorComplex;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::fermions::FermionOperator;
use struqture::mappings::JordanWignerFermionToSpin;
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture::{OperateOnDensityMatrix, OperateOnModes, OperateOnState};
use struqture_py_macros::{mappings, noiseless_system_wrapper};

/// These are representations of systems of fermions.
///
/// FermionOperators are characterized by a FermionOperator to represent the hamiltonian of the spin system
/// and an optional number of fermions.
///
/// Returns:
///     self: The new FermionSystem with the input number of fermions.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.fermions import FermionOperator, FermionProduct
///
///     system = FermionOperator()
///     pp = FermionProduct([0], [0])
///     system.add_operator_product(pp, 5.0)
///     npt.assert_equal(system.current_number_modes(), 2)
///     npt.assert_equal(system.get(pp), CalculatorComplex(5))
///     npt.assert_equal(system.keys(), [pp])
///
#[pyclass(name = "FermionOperator", module = "struqture_py.fermions")]
#[derive(Clone, Debug, PartialEq)]
pub struct FermionOperatorWrapper {
    /// Internal storage of [struqture::fermions::FermionOperator]
    pub internal: FermionOperator,
}

#[mappings(JordanWignerFermionToSpin)]
#[noiseless_system_wrapper(
    OperateOnFermions,
    OperateOnState,
    OperateOnModes,
    OperateOnDensityMatrix,
    Calculus
)]
impl FermionOperatorWrapper {
    /// Create an empty FermionOperator.
    ///
    /// Returns:
    ///     self: The new FermionOperator with the input number of fermions.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: FermionOperator::new(),
        }
    }

    /// Implement `*` for FermionOperator and FermionOperator/CalculatorComplex/CalculatorFloat.
    ///
    /// Args:
    ///     value (Union[FermionOperator, CalculatorComplex, CalculatorFloat]): value by which to multiply the self FermionOperator
    ///
    /// Returns:
    ///     FermionOperator: The FermionOperator multiplied by the value.
    ///
    /// Raises:
    ///     ValueError: The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor FermionOperator.
    pub fn __mul__(&self, value: &Bound<PyAny>) -> PyResult<Self> {
        let cf_value = qoqo_calculator_pyo3::convert_into_calculator_float(value);
        match cf_value {
            Ok(x) => Ok(FermionOperatorWrapper {
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
                                "The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor FermionOperator: {:?}",
                                err)))
                        }
                    }
                }
            }
        }
    }
}

impl Default for FermionOperatorWrapper {
    fn default() -> Self {
        Self::new()
    }
}
