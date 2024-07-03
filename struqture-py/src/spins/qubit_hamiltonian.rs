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

use super::QubitOperatorWrapper;
use crate::fermions::FermionHamiltonianWrapper;
use crate::spins::PauliProductWrapper;
use crate::{to_py_coo, PyCooMatrix};
use bincode::deserialize;
use num_complex::Complex64;
use pyo3::exceptions::{PyRuntimeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator::CalculatorComplex;
use qoqo_calculator_pyo3::CalculatorFloatWrapper;
use struqture::mappings::JordanWignerSpinToFermion;
use struqture::spins::{
    OperateOnSpins, QubitHamiltonian, ToSparseMatrixOperator, ToSparseMatrixSuperOperator,
};
use struqture::StruqtureError;
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture::{OperateOnDensityMatrix, OperateOnState};
use struqture_py_macros::{mappings, noiseless_system_wrapper};

/// These are representations of systems of spins.
///
/// QubitHamiltonians are characterized by a QubitOperator to represent the hamiltonian of the spin system
/// and an optional number of spins.
///
/// Returns:
///     self: The new QubitHamiltonian.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.spins import QubitHamiltonian, PauliProduct
///
///     ssystem = QubitHamiltonian(2)
///     pp = PauliProduct().z(0)
///     ssystem.add_operator_product(pp, 5.0)
///     npt.assert_equal(ssystem.current_number_spins(), 2)
///     npt.assert_equal(ssystem.get(pp), CalculatorComplex(5))
///     npt.assert_equal(ssystem.keys(), [pp])
///     dimension = 4**ssystem.current_number_spins()
///     matrix = sp.coo_matrix(ssystem.sparse_matrix_superoperator_coo(), shape=(dimension, dimension))
///
#[pyclass(name = "QubitHamiltonian", module = "struqture_py.spins")]
#[derive(Clone, Debug, PartialEq)]
pub struct QubitHamiltonianWrapper {
    /// Internal storage of [struqture::spins::QubitHamiltonian]
    pub internal: QubitHamiltonian,
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
impl QubitHamiltonianWrapper {
    /// Create an empty QubitHamiltonian.
    ///
    /// Returns:
    ///     self: The new QubitHamiltonian.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: QubitHamiltonian::new(),
        }
    }

    /// Implement `*` for QubitHamiltonian and QubitHamiltonian/CalculatorComplex/CalculatorFloat.
    ///
    /// Args:
    ///     value (Union[QubitHamiltonian, CalculatorComplex, CalculatorFloat]): value by which to multiply the self QubitHamiltonian
    ///
    /// Returns:
    ///     QubitOperator: The QubitHamiltonian multiplied by the value.
    ///
    /// Raises:
    ///     ValueError: The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor QubitHamiltonian.
    pub fn __mul__(&self, value: &Bound<PyAny>) -> PyResult<QubitOperatorWrapper> {
        let cf_value = qoqo_calculator_pyo3::convert_into_calculator_float(value);
        match cf_value {
            Ok(x) => Ok(QubitOperatorWrapper {
                internal: self.clone().internal * CalculatorComplex::from(x),
            }),
            Err(_) => {
                let cc_value = qoqo_calculator_pyo3::convert_into_calculator_complex(value);
                match cc_value {
                    Ok(x) => Ok(QubitOperatorWrapper {
                        internal: self.clone().internal * x,
                    }),
                    Err(_) => {
                        let bhs_value = Self::from_pyany(value);
                        match bhs_value {
                            Ok(x) => {
                                let new_self = self.clone().internal * x;
                                Ok(QubitOperatorWrapper { internal: new_self })
                            },
                            Err(err) => Err(PyValueError::new_err(format!(
                                "The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor QubitHamiltonian: {:?}",
                                err)))
                        }
                    }
                }
            }
        }
    }
}

impl Default for QubitHamiltonianWrapper {
    fn default() -> Self {
        Self::new()
    }
}
