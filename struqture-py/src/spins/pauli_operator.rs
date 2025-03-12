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
use pyo3::exceptions::{PyRuntimeError, PyTypeError, PyValueError};
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator::CalculatorComplex;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::mappings::JordanWignerSpinToFermion;
use struqture::spins::{
    OperateOnSpins, PauliOperator, ToSparseMatrixOperator, ToSparseMatrixSuperOperator,
};
use struqture::StruqtureError;
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture::{OperateOnDensityMatrix, OperateOnState};
use struqture_py_macros::{mappings, noiseless_system_wrapper};

/// These are representations of systems of spins.
///
/// PauliOperators are characterized by a PauliOperator to represent the hamiltonian of the spin system
/// and an optional number of spins.
///
/// Returns:
///     self: The new PauliOperator.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.spins import PauliOperator, PauliProduct
///
///     system = PauliOperator()
///     pp = PauliProduct().z(0)
///     system.add_operator_product(pp, 5.0)
///     npt.assert_equal(system.current_number_spins(), 2)
///     npt.assert_equal(system.get(pp), CalculatorComplex(5))
///     npt.assert_equal(system.keys(), [pp])
///     dimension = 4**system.current_number_spins()
///     matrix = sp.coo_matrix(system.sparse_matrix_superoperator_coo(system.current_number_spins()), shape=(dimension, dimension))
///
#[pyclass(name = "PauliOperator", module = "struqture_py.spins")]
#[derive(Clone, Debug, PartialEq)]
pub struct PauliOperatorWrapper {
    /// Internal storage of [struqture::spins::PauliOperator]
    pub internal: PauliOperator,
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
impl PauliOperatorWrapper {
    /// Create an empty PauliOperator.
    ///
    /// Returns:
    ///     self: The new PauliOperator.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: PauliOperator::new(),
        }
    }

    /// Implement `*` for PauliOperator and PauliOperator/CalculatorComplex/CalculatorFloat.
    ///
    /// Args:
    ///     value (Union[PauliOperator, CalculatorComplex, CalculatorFloat]): value by which to multiply the self PauliOperator
    ///
    /// Returns:
    ///     PauliOperator: The PauliOperator multiplied by the value.
    ///
    /// Raises:
    ///     ValueError: The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor PauliOperator.
    pub fn __mul__(&self, value: &Bound<PyAny>) -> PyResult<Self> {
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
                                "The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor PauliOperator: {:?}",
                                err)))
                        }
                    }
                }
            }
        }
    }
}

impl Default for PauliOperatorWrapper {
    fn default() -> Self {
        Self::new()
    }
}
