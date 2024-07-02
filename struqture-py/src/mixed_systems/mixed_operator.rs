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

use crate::mixed_systems::MixedProductWrapper;
use bincode::deserialize;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator::CalculatorComplex;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::mixed_systems::{MixedOperator, OperateOnMixedSystems};
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture::{OperateOnDensityMatrix, OperateOnState};
use struqture_py_macros::noiseless_system_wrapper;

/// These are representations of systems of mixed_systems.
///
/// MixedOperators are characterized by a MixedOperator to represent the hamiltonian of the spin system
/// and an optional number of mixed_systems.
///
/// Args:
///     number_spins (int): The number of spin subsystems in the MixedOperator.
///     number_bosons (int): The number of boson subsystems in the MixedOperator.
///     number_fermions (int): The number of fermion subsystems in the MixedOperator.
///
/// Returns:
///     self: The new (empty) MixedOperator.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.mixed_systems import MixedOperator, MixedProduct
///     from struqture_py.spins import PauliProduct
///     from struqture_py.bosons import BosonProduct
///     from struqture_py.fermions import FermionProduct
///
///     ssystem = MixedOperator([2], [2], [2])
///     pp = MixedProduct([PauliProduct().z(0)], [BosonProduct([0], [1])], [FermionProduct([0], [0])])
///     ssystem.add_operator_product(pp, 5.0)
///     npt.assert_equal(ssystem.current_number_spins(), [2])
///     npt.assert_equal(ssystem.get(pp), CalculatorComplex(5))
///
#[pyclass(name = "MixedOperator", module = "struqture_py.mixed_systems")]
#[derive(Clone, Debug, PartialEq)]
pub struct MixedOperatorWrapper {
    /// Internal storage of [struqture::mixed_systems::MixedOperator]
    pub internal: MixedOperator,
}

#[noiseless_system_wrapper(
    OperateOnMixedSystems,
    HermitianOperateOnMixedSystems,
    OperateOnState,
    OperateOnDensityMatrix,
    HermitianCalculus
)]
impl MixedOperatorWrapper {
    /// Create an empty MixedOperator.
    ///
    /// Args:
    ///     number_spins (int): The number of spin subsystems in the MixedOperator.
    ///     number_bosons (int): The number of boson subsystems in the MixedOperator.
    ///     number_fermions (int): The number of fermion subsystems in the MixedOperator.
    ///
    /// Returns:
    ///     self: The new (empty) MixedOperator.
    #[new]
    #[pyo3(signature = (
        number_spins,
        number_bosons,
        number_fermions,
    ))]
    pub fn new(number_spins: usize, number_bosons: usize, number_fermions: usize) -> Self {
        Self {
            internal: MixedOperator::new(number_spins, number_bosons, number_fermions),
        }
    }

    /// Implement `*` for MixedOperator and MixedOperator/CalculatorComplex/CalculatorFloat.
    ///
    /// Args:
    ///     value (Union[MixedOperator, CalculatorComplex, CalculatorFloat]): value by which to multiply the self MixedOperator
    ///
    /// Returns:
    ///     MixedOperator: The MixedOperator multiplied by the value.
    ///
    /// Raises:
    ///     ValueError: The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor MixedOperator.
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
                                let new_self = (self.clone().internal * x).map_err(|err| {
                                    PyValueError::new_err(format!(
                                        "MixedOperators could not be multiplied: {:?}",
                                        err
                                    ))
                                })?;
                                Ok(Self { internal: new_self })
                            },
                            Err(err) => Err(PyValueError::new_err(format!(
                                "The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor MixedOperator: {:?}",
                                err)))
                        }
                    }
                }
            }
        }
    }
}
