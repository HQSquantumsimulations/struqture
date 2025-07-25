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

use super::PauliHamiltonianWrapper;
use crate::{
    fermions::FermionOperatorWrapper,
    spins::{PauliOperatorWrapper, PlusMinusProductWrapper},
};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator::CalculatorComplex;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::mappings::JordanWignerSpinToFermion;
use struqture::spins::{PauliHamiltonian, PauliOperator, PlusMinusOperator};
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture::{OperateOnDensityMatrix, OperateOnState};
use struqture_py_macros::{mappings, noiseless_system_wrapper};

/// These are representations of systems of spins.
///
/// PlusMinusOperators are characterized by a PauliOperator to represent the hamiltonian of the spin system
/// and an optional number of spins.
///
/// Returns:
///     self: The new PlusMinusOperator.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.spins import PlusMinusOperator, PlusMinusProduct
///
///     system = PlusMinusOperator()
///     pp = PlusMinusProduct().z(0)
///     system.add_operator_product(pp, 5.0)
///     npt.assert_equal(system.get(pp), CalculatorComplex(5))
///     npt.assert_equal(system.keys(), [pp])
///
#[pyclass(name = "PlusMinusOperator", module = "struqture_py.spins")]
#[derive(Clone, Debug, PartialEq)]
pub struct PlusMinusOperatorWrapper {
    /// Internal storage of [struqture::spins::PlusMinusOperator]
    pub internal: PlusMinusOperator,
}

#[mappings(JordanWignerSpinToFermion)]
#[noiseless_system_wrapper(OperateOnState, OperateOnDensityMatrix, OperateOnSpins, Calculus)]
impl PlusMinusOperatorWrapper {
    /// Create an empty PlusMinusOperator.
    ///
    /// Returns:
    ///     self: The new PlusMinusOperator.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: PlusMinusOperator::new(),
        }
    }

    /// Implement `*` for PlusMinusOperator and PlusMinusOperator/CalculatorComplex/CalculatorFloat.
    ///
    /// Args:
    ///     value (Union[PlusMinusOperator, CalculatorComplex, CalculatorFloat]): value by which to multiply the self PlusMinusOperator
    ///
    /// Returns:
    ///     PlusMinusOperator: The PlusMinusOperator multiplied by the value.
    ///
    /// Raises:
    ///     ValueError: The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor PlusMinusOperator.
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
                    Err(err) => Err(PyValueError::new_err(format!("The rhs of the multiplication is neither CalculatorFloat nor CalculatorComplex: {err:?}")))
                }
            }
        }
    }

    /// Convert a PauliOperator into a PlusMinusOperator.
    ///
    /// Args:
    ///     value (PauliOperator): The PauliOperator to create the PlusMinusOperator from.
    ///
    /// Returns:
    ///     PlusMinusOperator: The operator created from the input PauliOperator.
    ///
    /// Raises:
    ///     ValueError: Could not create PauliOperator from input.
    #[staticmethod]
    pub fn from_pauli_operator(value: &Bound<PyAny>) -> PyResult<PlusMinusOperatorWrapper> {
        let system = PauliOperatorWrapper::from_pyany(value)
            .map_err(|err| PyValueError::new_err(format!("{err:?}")))?;
        Ok(PlusMinusOperatorWrapper {
            internal: PlusMinusOperator::from(system.clone()),
        })
    }

    /// Convert a PauliHamiltonian into a PlusMinusOperator.
    ///
    /// Args:
    ///     value (PauliHamiltonian): The PauliHamiltonian to create the PlusMinusOperator from.
    ///
    /// Returns:
    ///     PlusMinusOperator: The operator created from the input PauliOperator.
    ///
    /// Raises:
    ///     ValueError: Could not create PauliHamiltonian from input.
    #[staticmethod]
    pub fn from_pauli_hamiltonian(value: &Bound<PyAny>) -> PyResult<PlusMinusOperatorWrapper> {
        let system = PauliHamiltonianWrapper::from_pyany(value)
            .map_err(|err| PyValueError::new_err(format!("{err:?}")))?;
        Ok(PlusMinusOperatorWrapper {
            internal: PlusMinusOperator::from(system.clone()),
        })
    }

    /// Convert a PlusMinusOperator into a PauliOperator.
    ///
    /// Returns:
    ///     PauliOperator: The operator created from the input PlusMinusOperator and optional number of spins.
    ///
    /// Raises:
    ///     ValueError: Could not create PauliOperator from PlusMinusOperator.
    pub fn to_pauli_operator(&self) -> PyResult<PauliOperatorWrapper> {
        let result: PauliOperator = PauliOperator::from(self.internal.clone());
        Ok(PauliOperatorWrapper { internal: result })
    }

    /// Convert a PlusMinusOperator into a PauliHamiltonian.
    ///
    /// Returns:
    ///     PauliHamiltonian: The operator created from the input PlusMinusOperator and optional number of spins.
    ///
    /// Raises:
    ///     ValueError: Could not create PauliHamiltonian from PlusMinusOperator.
    pub fn to_pauli_hamiltonian(&self) -> PyResult<PauliHamiltonianWrapper> {
        let result: PauliHamiltonian = PauliHamiltonian::try_from(self.internal.clone())
            .map_err(|err| PyValueError::new_err(format!("{err:?}")))?;
        Ok(PauliHamiltonianWrapper { internal: result })
    }
}

impl Default for PlusMinusOperatorWrapper {
    fn default() -> Self {
        Self::new()
    }
}
