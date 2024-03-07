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

use crate::{
    fermions::FermionOperatorWrapper,
    spins::{PlusMinusProductWrapper, SpinOperatorWrapper},
};
use bincode::deserialize;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator::CalculatorComplex;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::mappings::JordanWignerSpinToFermion;
use struqture::spins::{PlusMinusOperator, SpinHamiltonian, SpinOperator};
#[cfg(feature = "json_schema")]
use struqture::{MinSupportedVersion, STRUQTURE_VERSION};
use struqture::{OperateOnDensityMatrix, OperateOnState};
use struqture_py_macros::{mappings, noiseless_system_wrapper};

use super::SpinHamiltonianWrapper;

/// These are representations of systems of spins.
///
/// PlusMinusOperators are characterized by a SpinOperator to represent the hamiltonian of the spin system
/// and an optional number of spins.
///
/// Returns:
///     self: The new PlusMinusOperator with the input number of spins.
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
///     ssystem = PlusMinusOperator()
///     pp = PlusMinusProduct().z(0)
///     ssystem.add_operator_product(pp, 5.0)
///     npt.assert_equal(ssystem.get(pp), CalculatorComplex(5))
///     npt.assert_equal(ssystem.keys(), [pp])
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
    ///     self: The new PlusMinusOperator with the input number of spins.
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
                    Err(err) => Err(PyValueError::new_err(format!("The rhs of the multiplication is neither CalculatorFloat nor CalculatorComplex: {:?}", err)))
                }
            }
        }
    }

    /// Convert a SpinOperator into a PlusMinusOperator.
    ///
    /// Args:
    ///     value (SpinOperator): The SpinOperator to create the PlusMinusOperator from.
    ///
    /// Returns:
    ///     PlusMinusOperator: The operator created from the input SpinOperator.
    ///
    /// Raises:
    ///     ValueError: Could not create SpinOperator from input.
    #[staticmethod]
    pub fn from_spin_system(value: Py<PyAny>) -> PyResult<PlusMinusOperatorWrapper> {
        let system = SpinOperatorWrapper::from_pyany(value)
            .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?;
        Ok(PlusMinusOperatorWrapper {
            internal: PlusMinusOperator::from(system.clone()),
        })
    }

    /// Convert a SpinHamiltonian into a PlusMinusOperator.
    ///
    /// Args:
    ///     value (SpinHamiltonian): The SpinHamiltonian to create the PlusMinusOperator from.
    ///
    /// Returns:
    ///     PlusMinusOperator: The operator created from the input SpinOperator.
    ///
    /// Raises:
    ///     ValueError: Could not create SpinHamiltonian from input.
    #[staticmethod]
    pub fn from_spin_hamiltonian_system(value: Py<PyAny>) -> PyResult<PlusMinusOperatorWrapper> {
        let system = SpinHamiltonianWrapper::from_pyany(value)
            .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?;
        Ok(PlusMinusOperatorWrapper {
            internal: PlusMinusOperator::from(system.clone()),
        })
    }

    /// Convert a PlusMinusOperator into a SpinOperator.
    ///
    /// Returns:
    ///     SpinOperator: The operator created from the input PlusMinusOperator and optional number of spins.
    ///
    /// Raises:
    ///     ValueError: Could not create SpinOperator from PlusMinusOperator.
    pub fn to_spin_system(&self) -> PyResult<SpinOperatorWrapper> {
        let result: SpinOperator = SpinOperator::from(self.internal.clone());
        Ok(SpinOperatorWrapper { internal: result })
    }

    /// Convert a PlusMinusOperator into a SpinHamiltonian.
    ///
    /// Returns:
    ///     SpinHamiltonian: The operator created from the input PlusMinusOperator and optional number of spins.
    ///
    /// Raises:
    ///     ValueError: Could not create SpinHamiltonian from PlusMinusOperator.
    pub fn to_spin_hamiltonian_system(&self) -> PyResult<SpinHamiltonianWrapper> {
        let result: SpinHamiltonian = SpinHamiltonian::try_from(self.internal.clone())
            .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?;
        Ok(SpinHamiltonianWrapper { internal: result })
    }
}

impl Default for PlusMinusOperatorWrapper {
    fn default() -> Self {
        Self::new()
    }
}
