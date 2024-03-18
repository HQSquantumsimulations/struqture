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

use crate::mixed_systems::MixedDecoherenceProductWrapper;
use bincode::deserialize;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::mixed_systems::{MixedLindbladNoiseOperator, OperateOnMixedSystems};
use struqture::OperateOnDensityMatrix;
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture_py_macros::noisy_system_wrapper;

/// These are representations of noisy systems of mixed_systems.
///
/// In a MixedLindbladNoiseOperator is characterized by a MixedLindbladNoiseOperator to represent the hamiltonian of the system, and an optional number of mixed_systems.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex, CalculatorFloat
///     from struqture_py.mixed_systems import MixedLindbladNoiseOperator, MixedDecoherenceProduct
///     from struqture_py.spins import DecoherenceProduct
///     from struqture_py.bosons import BosonProduct
///     from struqture_py.fermions import FermionProduct
///
///     slns = MixedLindbladNoiseOperator(1, 1, 1)
///     dp = MixedDecoherenceProduct([DecoherenceProduct().z(0)], [BosonProduct([0], [1])], [FermionProduct([0], [0])])
///     slns.add_operator_product((dp, dp), 2.0)
///     npt.assert_equal(slns.current_number_spins(), [1])
///     npt.assert_equal(slns.get((dp, dp)), CalculatorFloat(2))
///
#[pyclass(
    name = "MixedLindbladNoiseOperator",
    module = "struqture_py.mixed_systems"
)]
#[derive(Clone, Debug, PartialEq)]
pub struct MixedLindbladNoiseOperatorWrapper {
    /// Internal storage of [struqture::mixed_systems::MixedLindbladNoiseOperator]
    pub internal: MixedLindbladNoiseOperator,
}

#[noisy_system_wrapper(OperateOnMixedSystems, OperateOnDensityMatrix, Calculus)]
impl MixedLindbladNoiseOperatorWrapper {
    /// Create a new MixedLindbladNoiseOperator.
    ///
    /// Args:
    ///     number_spins (int): The number of spin subsystems in the MixedSystem.
    ///     number_bosons (int): The number of boson subsystems in the MixedSystem.
    ///     number_fermions (int): The number of fermion subsystems in the MixedSystem.
    ///
    /// Returns:
    ///     self: The new MixedLindbladNoiseOperator.
    #[new]
    #[pyo3(signature = (
        number_spins,
        number_bosons,
        number_fermions,
    ))]
    pub fn new(number_spins: usize, number_bosons: usize, number_fermions: usize) -> Self {
        Self {
            internal: MixedLindbladNoiseOperator::new(number_spins, number_bosons, number_fermions),
        }
    }
}
