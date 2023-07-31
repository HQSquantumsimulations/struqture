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

use crate::fermions::FermionLindbladNoiseSystemWrapper;
use crate::spins::PlusMinusProductWrapper;
use bincode::deserialize;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::fermions::FermionLindbladNoiseSystem;
use struqture::mappings::JordanWignerSpinToFermion;
use struqture::spins::{
    PlusMinusLindbladNoiseOperator, SpinLindbladNoiseOperator, SpinLindbladNoiseSystem,
};
use struqture::OperateOnDensityMatrix;
use struqture_py_macros::{mappings, noisy_system_wrapper};

use super::SpinLindbladNoiseSystemWrapper;
#[cfg(feature = "json_schema")]
use struqture::{MinSupportedVersion, STRUQTURE_VERSION};

/// These are representations of noisy systems of spins.
///
/// In a PlusMinusLindbladNoiseOperator is characterized by a SpinLindbladNoiseOperator to represent the hamiltonian of the spin system, and an optional number of spins.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.spins import PlusMinusLindbladNoiseOperator, PlusMinusProduct
///
///     slns = PlusMinusLindbladNoiseOperator()
///     dp = PlusMinusProduct().z(0).plus(1)
///     slns.add_operator_product((dp, dp), 2.0)
///     npt.assert_equal(slns.get((dp, dp)), CalculatorComplex(2))
///     npt.assert_equal(slns.keys(), [(dp, dp)])
///
#[pyclass(name = "PlusMinusLindbladNoiseOperator", module = "struqture_py.spins")]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct PlusMinusLindbladNoiseOperatorWrapper {
    /// Internal storage of [struqture::spins::PlusMinusLindbladNoiseOperator]
    pub internal: PlusMinusLindbladNoiseOperator,
}

#[mappings(JordanWignerSpinToFermion)]
#[noisy_system_wrapper(OperateOnDensityMatrix)]
impl PlusMinusLindbladNoiseOperatorWrapper {
    /// Create a new PlusMinusLindbladNoiseOperator.
    ///
    /// Returns:
    ///     self: The new PlusMinusLindbladNoiseOperator with the input number of spins.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: PlusMinusLindbladNoiseOperator::new(),
        }
    }
    /// Implement `-1` for self.
    ///
    /// Returns:
    ///     self: The object * -1.
    pub fn __neg__(&self) -> PlusMinusLindbladNoiseOperatorWrapper {
        PlusMinusLindbladNoiseOperatorWrapper {
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
    pub fn __add__(
        &self,
        other: PlusMinusLindbladNoiseOperatorWrapper,
    ) -> PlusMinusLindbladNoiseOperatorWrapper {
        let new_self = self.clone().internal + other.internal;
        PlusMinusLindbladNoiseOperatorWrapper { internal: new_self }
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
    pub fn __sub__(
        &self,
        other: PlusMinusLindbladNoiseOperatorWrapper,
    ) -> PlusMinusLindbladNoiseOperatorWrapper {
        let new_self = self.clone().internal - other.internal;
        PlusMinusLindbladNoiseOperatorWrapper { internal: new_self }
    }

    /// Separate self into an operator with the terms of given number of spins (left and right) and an operator with the remaining operations.
    ///
    /// Args
    ///     number_spins_left (int): Number of spin to filter for in the left key.
    ///     number_spins_right (int): Number of spin to filter for in the right key.
    ///
    /// Returns
    ///     Tuple[PlusMinusLindbladNoiseOperator, PlusMinusLindbladNoiseOperator]: Operator with the noise terms where number_spins (left and right) matches the number of spins the operator product acts on and Operator with all other contributions.
    ///
    /// Raises:
    ///     ValueError: Error in adding terms to return values.
    pub fn separate_into_n_terms(
        &self,
        number_spins_left: usize,
        number_spins_right: usize,
    ) -> PyResult<(
        PlusMinusLindbladNoiseOperatorWrapper,
        PlusMinusLindbladNoiseOperatorWrapper,
    )> {
        let result = self
            .internal
            .separate_into_n_terms(number_spins_left, number_spins_right)
            .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?;
        Ok((
            PlusMinusLindbladNoiseOperatorWrapper { internal: result.0 },
            PlusMinusLindbladNoiseOperatorWrapper { internal: result.1 },
        ))
    }

    /// Convert a SpinLindbladNoiseSystem into a PlusMinusLindbladNoiseOperator.
    ///
    /// Args:
    ///     value (SpinLindbladNoiseSystem): The SpinLindbladNoiseSystem to create the PlusMinusLindbladNoiseOperator from.
    ///
    /// Returns:
    ///     PlusMinusLindbladNoiseOperator: The operator created from the input SpinLindbladNoiseSystem.
    ///
    /// Raises:
    ///     ValueError: Could not create SpinLindbladNoiseSystem from input.
    #[staticmethod]
    pub fn from_spin_noise_system(
        value: Py<PyAny>,
    ) -> PyResult<PlusMinusLindbladNoiseOperatorWrapper> {
        let system = SpinLindbladNoiseSystemWrapper::from_pyany(value)
            .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?;
        Ok(PlusMinusLindbladNoiseOperatorWrapper {
            internal: PlusMinusLindbladNoiseOperator::from(system.operator().clone()),
        })
    }

    /// Convert a PlusMinusLindbladNoiseOperator into a SpinLindbladNoiseSystem.
    ///
    /// Args:
    ///     number_spinss (Optional[int]): The number of spins to initialize the SpinLindbladNoiseSystem with.
    ///
    /// Returns:
    ///     SpinLindbladNoiseSystem: The operator created from the input PlusMinusLindbladNoiseOperator and optional number of spins.
    ///
    /// Raises:
    ///     ValueError: Could not create SpinLindbladNoiseSystem from PlusMinusLindbladNoiseOperator.
    pub fn to_spin_noise_system(
        &self,
        number_spinss: Option<usize>,
    ) -> PyResult<SpinLindbladNoiseSystemWrapper> {
        let result: SpinLindbladNoiseOperator =
            SpinLindbladNoiseOperator::from(self.internal.clone());
        Ok(SpinLindbladNoiseSystemWrapper {
            internal: SpinLindbladNoiseSystem::from_operator(result, number_spinss)
                .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?,
        })
    }
}
