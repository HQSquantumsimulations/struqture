// Copyright © 2021-2022 HQS Quantum Simulations GmbH. All Rights Reserved.
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
use bincode::deserialize;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::fermions::FermionLindbladNoiseSystem;
use struqture::{OperateOnDensityMatrix, OperateOnModes};
use struqture_py_macros::noisy_system_wrapper;

/// These are representations of noisy systems of fermions.
///
/// In a FermionLindbladNoiseSystem is characterized by a FermionLindbladNoiseOperator to represent the hamiltonian of the system, and an optional number of fermions.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.fermions import FermionLindbladNoiseSystem, FermionProduct
///
///     slns = fermions.FermionLindbladNoiseSystem()
///     dp = fermions.FermionProduct([0], [0])
///     slns = slns.add_operator_product((dp, dp), 2.0)
///     npt.assert_equal(slns.current_number_fermions(), 2)
///     npt.assert_equal(slns.get((dp, dp)), CalculatorComplex(2))
///
#[pyclass(name = "FermionLindbladNoiseSystem", module = "struqture_py.fermions")]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct FermionLindbladNoiseSystemWrapper {
    /// Internal storage of [struqture::fermions::FermionLindbladNoiseSystem]
    pub internal: FermionLindbladNoiseSystem,
}

#[noisy_system_wrapper(OperateOnModes, OperateOnFermions, OperateOnDensityMatrix, Calculus)]
impl FermionLindbladNoiseSystemWrapper {
    /// Create a new FermionLindbladNoiseSystem.
    ///
    /// Args:
    ///     number_fermions (Optional(int)): The number of fermions in the FermionLindbladNoiseSystem.
    ///
    /// Returns:
    ///     self: The new FermionLindbladNoiseSystem with the input number of fermions.
    #[new]
    #[args(number_fermions = "None")]
    pub fn new(number_fermions: Option<usize>) -> Self {
        Self {
            internal: FermionLindbladNoiseSystem::new(number_fermions),
        }
    }
}
