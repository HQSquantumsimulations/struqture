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

use super::MixedOperatorWrapper;
use crate::mixed_systems::MixedPlusMinusProductWrapper;
use bincode::deserialize;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator::CalculatorComplex;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::mixed_systems::{MixedOperator, MixedPlusMinusOperator, OperateOnMixedSystems};
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture::{OperateOnDensityMatrix, OperateOnState};
use struqture_py_macros::noiseless_system_wrapper;

/// These are representations of systems of mixed_systems.
///
/// MixedPlusMinusOperators are characterized by a MixedOperator to represent the hamiltonian of the spin system
/// and an optional number of mixed_systems.
///
/// Args:
///     number_spins (int): The number of spin subsystems in the MixedPlusMinusOperator.
///     number_bosons (int): The number of boson subsystems in the MixedPlusMinusOperator.
///     number_fermions (int): The number of fermion subsystems in the MixedPlusMinusOperator.
///
/// Returns:
///     self: The new (empty) MixedPlusMinusOperator.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.mixed_systems import MixedPlusMinusOperator, MixedPlusMinusProduct
///     from struqture_py.spins import PauliProduct
///     from struqture_py.bosons import BosonProduct
///     from struqture_py.fermions import FermionProduct
///
///     system = MixedPlusMinusOperator(1, 1, 1)
///     pp = MixedPlusMinusProduct([PauliProduct().z(0)], [BosonProduct([0], [1])], [FermionProduct([0], [0])])
///     system.add_operator_product(pp, 5.0)
///     npt.assert_equal(system.current_number_spins(), [2])
///     npt.assert_equal(system.get(pp), CalculatorComplex(5))
///
#[pyclass(name = "MixedPlusMinusOperator", module = "struqture_py.mixed_systems")]
#[derive(Clone, Debug, PartialEq)]
pub struct MixedPlusMinusOperatorWrapper {
    /// Internal storage of [struqture::mixed_systems::MixedPlusMinusOperator]
    pub internal: MixedPlusMinusOperator,
}

#[noiseless_system_wrapper(
    OperateOnMixedSystems,
    OperateOnState,
    OperateOnDensityMatrix,
    HermitianCalculus
)]
impl MixedPlusMinusOperatorWrapper {
    /// Create an empty MixedPlusMinusOperator.
    ///
    /// Args:
    ///     number_spins (int): The number of spin subsystems in the MixedPlusMinusOperator.
    ///     number_bosons (int): The number of boson subsystems in the MixedPlusMinusOperator.
    ///     number_fermions (int): The number of fermion subsystems in the MixedPlusMinusOperator.
    ///
    /// Returns:
    ///     self: The new (empty) MixedPlusMinusOperator.
    #[new]
    pub fn new(number_spins: usize, number_bosons: usize, number_fermions: usize) -> Self {
        Self {
            internal: MixedPlusMinusOperator::new(number_spins, number_bosons, number_fermions),
        }
    }

    /// Implement `*` for MixedPlusMinusOperator and MixedPlusMinusOperator/CalculatorComplex/CalculatorFloat.
    ///
    /// Args:
    ///     value (Union[MixedPlusMinusOperator, CalculatorComplex, CalculatorFloat]): value by which to multiply the self MixedPlusMinusOperator
    ///
    /// Returns:
    ///     MixedPlusMinusOperator: The MixedPlusMinusOperator multiplied by the value.
    ///
    /// Raises:
    ///     ValueError: The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor MixedPlusMinusOperator.
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
                    Err(err) => Err(PyValueError::new_err(format!(
                                "The rhs of the multiplication is neither CalculatorFloat nor CalculatorComplex: {:?}",
                                err)))
                        }
            }
        }
    }

    /// Convert a MixedOperator into a MixedPlusMinusOperator.
    ///
    /// Args:
    ///     value (MixedOperator): The MixedOperator to create the MixedPlusMinusOperator from.
    ///
    /// Returns:
    ///     MixedPlusMinusOperator: The operator created from the input MixedOperator.
    ///
    /// Raises:
    ///     ValueError: Could not create MixedOperator from input.
    #[staticmethod]
    pub fn from_mixed_operator(value: &Bound<PyAny>) -> PyResult<MixedPlusMinusOperatorWrapper> {
        let system = MixedOperatorWrapper::from_pyany(value)
            .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?;
        Ok(MixedPlusMinusOperatorWrapper {
            internal: MixedPlusMinusOperator::from(system.clone()),
        })
    }

    /// Convert a MixedPlusMinusOperator into a MixedOperator.
    ///
    /// Returns:
    ///     MixedOperator: The operator created from the input MixedPlusMinusOperator and optional number of spins.
    ///
    /// Raises:
    ///     ValueError: Could not create MixedOperator from MixedPlusMinusOperator.
    ///     ValueError: Could not create MixedOperator from MixedOperator.
    pub fn to_mixed_operator(&self) -> PyResult<MixedOperatorWrapper> {
        let result: MixedOperator = MixedOperator::try_from(self.internal.clone())
            .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?;
        Ok(MixedOperatorWrapper { internal: result })
    }
}
