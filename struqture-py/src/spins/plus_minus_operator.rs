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

use crate::fermions::FermionSystemWrapper;
use crate::spins::{PlusMinusProductWrapper, SpinSystemWrapper};
use bincode::deserialize;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator::CalculatorComplex;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::fermions::FermionSystem;
use struqture::mappings::JordanWignerSpinToFermion;
use struqture::spins::{
    PlusMinusOperator, SpinHamiltonian, SpinHamiltonianSystem, SpinOperator, SpinSystem,
};
#[cfg(feature = "json_schema")]
use struqture::{MinSupportedVersion, STRUQTURE_VERSION};
use struqture::{OperateOnDensityMatrix, OperateOnState};
use struqture_py_macros::{mappings, noiseless_system_wrapper};

use super::SpinHamiltonianSystemWrapper;

/// These are representations of systems of spins.
///
/// PlusMinusOperators are characterized by a SpinOperator to represent the hamiltonian of the spin system
/// and an optional number of spins.
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
#[noiseless_system_wrapper(OperateOnState, OperateOnDensityMatrix)]
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
    pub fn __mul__(&self, value: &PyAny) -> PyResult<Self> {
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

    /// Implement `-1` for self.
    ///
    /// Returns:
    ///     self: The object * -1.
    pub fn __neg__(&self) -> PlusMinusOperatorWrapper {
        PlusMinusOperatorWrapper {
            internal: -self.clone().internal,
        }
    }

    /// Implement `+` for self with self-type.
    ///
    /// Args:
    ///     other (self): value by which to add to self.
    ///
    /// Returns:
    ///     self: The two objects added.
    ///
    /// Raises:
    ///     ValueError: Objects could not be added.
    pub fn __add__(&self, other: PlusMinusOperatorWrapper) -> PlusMinusOperatorWrapper {
        PlusMinusOperatorWrapper {
            internal: self.clone().internal + other.internal,
        }
    }

    /// Implement `-` for self with self-type.
    ///
    /// Args:
    ///     other (self): value by which to subtract from self.
    ///
    /// Returns:
    ///     self: The two objects subtracted.
    ///
    /// Raises:
    ///     ValueError: Objects could not be subtracted.
    pub fn __sub__(&self, other: PlusMinusOperatorWrapper) -> PlusMinusOperatorWrapper {
        PlusMinusOperatorWrapper {
            internal: self.clone().internal - other.internal,
        }
    }

    /// Separate self into an operator with the terms of given number of spins and an operator with the remaining operations
    ///
    /// Args
    ///     number_spins (int): Number of spins to filter for in the keys.
    ///
    /// Returns
    ///     (PlusMinusOperator, PlusMinusOperator): Operator with the terms where number_spins matches the number of spins the operator product acts on and Operator with all other contributions.
    ///
    /// Raises:
    ///     ValueError: Error in adding terms to return values.
    pub fn separate_into_n_terms(
        &self,
        number_spins: usize,
    ) -> PyResult<(PlusMinusOperatorWrapper, PlusMinusOperatorWrapper)> {
        let result = self
            .internal
            .separate_into_n_terms(number_spins)
            .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?;
        Ok((
            PlusMinusOperatorWrapper { internal: result.0 },
            PlusMinusOperatorWrapper { internal: result.1 },
        ))
    }

    /// Convert a SpinSystem into a PlusMinusOperator.
    ///
    /// Args:
    ///     value (SpinSystem): The SpinSystem to create the PlusMinusOperator from.
    ///
    /// Returns:
    ///     PlusMinusOperator: The operator created from the input SpinSystem.
    ///
    /// Raises:
    ///     ValueError: Could not create SpinSystem from input.
    #[staticmethod]
    pub fn from_spin_system(value: Py<PyAny>) -> PyResult<PlusMinusOperatorWrapper> {
        let system = SpinSystemWrapper::from_pyany(value)
            .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?;
        Ok(PlusMinusOperatorWrapper {
            internal: PlusMinusOperator::from(system.operator().clone()),
        })
    }

    /// Convert a SpinHamiltonianSystem into a PlusMinusOperator.
    ///
    /// Args:
    ///     value (SpinHamiltonianSystem): The SpinHamiltonianSystem to create the PlusMinusOperator from.
    ///
    /// Returns:
    ///     PlusMinusOperator: The operator created from the input SpinSystem.
    ///
    /// Raises:
    ///     ValueError: Could not create SpinHamiltonianSystem from input.
    #[staticmethod]
    pub fn from_spin_hamiltonian_system(value: Py<PyAny>) -> PyResult<PlusMinusOperatorWrapper> {
        let system = SpinHamiltonianSystemWrapper::from_pyany(value)
            .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?;
        Ok(PlusMinusOperatorWrapper {
            internal: PlusMinusOperator::from(system.hamiltonian().clone()),
        })
    }

    /// Convert a PlusMinusOperator into a SpinSystem.
    ///
    /// Args:
    ///     number_spins (Optional[int]): The number of spins to initialize the SpinSystem with.
    ///
    /// Returns:
    ///     SpinSystem: The operator created from the input PlusMinusOperator and optional number of spins.
    ///
    /// Raises:
    ///     ValueError: Could not create SpinSystem from PlusMinusOperator.
    pub fn to_spin_system(&self, number_spins: Option<usize>) -> PyResult<SpinSystemWrapper> {
        let result: SpinOperator = SpinOperator::from(self.internal.clone());
        Ok(SpinSystemWrapper {
            internal: SpinSystem::from_operator(result, number_spins)
                .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?,
        })
    }

    /// Convert a PlusMinusOperator into a SpinHamiltonianSystem.
    ///
    /// Args:
    ///     number_spins (Optional[int]): The number of spins to initialize the SpinHamiltonianSystem with.
    ///
    /// Returns:
    ///     SpinHamiltonianSystem: The operator created from the input PlusMinusOperator and optional number of spins.
    ///
    /// Raises:
    ///     ValueError: Could not create SpinHamiltonianSystem from PlusMinusOperator.
    pub fn to_spin_hamiltonian_system(
        &self,
        number_spins: Option<usize>,
    ) -> PyResult<SpinHamiltonianSystemWrapper> {
        let result: SpinHamiltonian = SpinHamiltonian::try_from(self.internal.clone())
            .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?;
        Ok(SpinHamiltonianSystemWrapper {
            internal: SpinHamiltonianSystem::from_hamiltonian(result, number_spins)
                .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?,
        })
    }
}

impl Default for PlusMinusOperatorWrapper {
    fn default() -> Self {
        Self::new()
    }
}
