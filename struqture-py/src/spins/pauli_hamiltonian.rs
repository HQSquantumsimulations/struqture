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

use super::PauliOperatorWrapper;
use crate::fermions::FermionHamiltonianWrapper;
use crate::spins::PauliProductWrapper;
use crate::{to_py_coo, PyCooMatrix};
use bincode::deserialize;
use pyo3::exceptions::{PyRuntimeError, PyTypeError, PyValueError};
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator::CalculatorComplex;
use qoqo_calculator_pyo3::CalculatorFloatWrapper;
use struqture::mappings::JordanWignerSpinToFermion;
use struqture::spins::{
    OperateOnSpins, PauliHamiltonian, ToSparseMatrixOperator, ToSparseMatrixSuperOperator,
};
use struqture::StruqtureError;
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture::{OperateOnDensityMatrix, OperateOnState};
use struqture_py_macros::{mappings, noiseless_system_wrapper};

/// These are representations of systems of spins.
///
/// PauliHamiltonians are characterized by a PauliOperator to represent the hamiltonian of the spin system
/// and an optional number of spins.
///
/// Returns:
///     self: The new PauliHamiltonian.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.spins import PauliHamiltonian, PauliProduct
///
///     system = PauliHamiltonian()
///     pp = PauliProduct().z(0)
///     system.add_operator_product(pp, 5.0)
///     npt.assert_equal(system.current_number_spins(), 2)
///     npt.assert_equal(system.get(pp), CalculatorComplex(5))
///     npt.assert_equal(system.keys(), [pp])
///     dimension = 4**system.current_number_spins()
///     matrix = sp.coo_matrix(system.sparse_matrix_superoperator_coo(system.current_number_spins()), shape=(dimension, dimension))
///
#[pyclass(name = "PauliHamiltonian", module = "struqture_py.spins")]
#[derive(Clone, Debug, PartialEq)]
pub struct PauliHamiltonianWrapper {
    /// Internal storage of [struqture::spins::PauliHamiltonian]
    pub internal: PauliHamiltonian,
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
impl PauliHamiltonianWrapper {
    /// Create an empty PauliHamiltonian.
    ///
    /// Returns:
    ///     self: The new PauliHamiltonian.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: PauliHamiltonian::new(),
        }
    }

    /// Implement `*` for PauliHamiltonian and PauliHamiltonian/CalculatorComplex/CalculatorFloat.
    ///
    /// Args:
    ///     value (Union[PauliHamiltonian, CalculatorComplex, CalculatorFloat]): value by which to multiply the self PauliHamiltonian
    ///
    /// Returns:
    ///     PauliOperator: The PauliHamiltonian multiplied by the value.
    ///
    /// Raises:
    ///     ValueError: The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor PauliHamiltonian.
    pub fn __mul__(&self, value: &Bound<PyAny>) -> PyResult<PauliOperatorWrapper> {
        let cf_value = qoqo_calculator_pyo3::convert_into_calculator_float(value);
        match cf_value {
            Ok(x) => Ok(PauliOperatorWrapper {
                internal: self.clone().internal * CalculatorComplex::from(x),
            }),
            Err(_) => {
                let cc_value = qoqo_calculator_pyo3::convert_into_calculator_complex(value);
                match cc_value {
                    Ok(x) => Ok(PauliOperatorWrapper {
                        internal: self.clone().internal * x,
                    }),
                    Err(_) => {
                        let bhs_value = Self::from_pyany(value);
                        match bhs_value {
                            Ok(x) => {
                                let new_self = self.clone().internal * x;
                                Ok(PauliOperatorWrapper { internal: new_self })
                            },
                            Err(err) => Err(PyValueError::new_err(format!(
                                "The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor PauliHamiltonian: {:?}",
                                err)))
                        }
                    }
                }
            }
        }
    }
}

impl Default for PauliHamiltonianWrapper {
    fn default() -> Self {
        Self::new()
    }
}
