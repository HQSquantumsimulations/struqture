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
use struqture::mixed_systems::{MixedLindbladNoiseSystem, OperateOnMixedSystems};
use struqture::OperateOnDensityMatrix;
#[cfg(feature = "json_schema")]
use struqture::{MinSupportedVersion, STRUQTURE_VERSION};
use struqture_py_macros::noisy_system_wrapper;

/// These are representations of noisy systems of mixed_systems.
///
/// In a MixedLindbladNoiseSystem is characterized by a MixedLindbladNoiseOperator to represent the hamiltonian of the system, and an optional number of mixed_systems.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex, CalculatorFloat
///     from struqture_py.mixed_systems import MixedLindbladNoiseSystem, MixedDecoherenceProduct
///     from struqture_py.spins import DecoherenceProduct
///     from struqture_py.bosons import BosonProduct
///     from struqture_py.fermions import FermionProduct
///
///     slns = MixedLindbladNoiseSystem()
///     dp = MixedDecoherenceProduct([DecoherenceProduct().z(0)], [BosonProduct([0], [1])], [FermionProduct([0], [0])])
///     slns.add_operator_product((dp, dp), 2.0)
///     npt.assert_equal(slns.current_number_spins(), [1])
///     npt.assert_equal(slns.get((dp, dp)), CalculatorFloat(2))
///
#[pyclass(
    name = "MixedLindbladNoiseSystem",
    module = "struqture_py.mixed_systems"
)]
#[derive(Clone, Debug, PartialEq)]
pub struct MixedLindbladNoiseSystemWrapper {
    /// Internal storage of [struqture::mixed_systems::MixedLindbladNoiseSystem]
    pub internal: MixedLindbladNoiseSystem,
}

#[noisy_system_wrapper(OperateOnMixedSystems, OperateOnDensityMatrix, Calculus)]
impl MixedLindbladNoiseSystemWrapper {
    /// Create a new MixedLindbladNoiseSystem.
    ///
    /// Args:
    ///     number_spins (List[Optional[int]]): The number of spin subsystems in the MixedSystem.
    ///     number_bosons (List[Optional[int]]): The number of boson subsystems in the MixedSystem.
    ///     number_fermions (List[Optional[int]]): The number of fermion subsystems in the MixedSystem.
    ///
    /// Returns:
    ///     self: The new MixedLindbladNoiseSystem.
    #[new]
    #[pyo3(signature = (
        number_spins = vec![None],
        number_bosons = vec![None],
        number_fermions = vec![None],
    ))]
    pub fn new(
        number_spins: Vec<Option<usize>>,
        number_bosons: Vec<Option<usize>>,
        number_fermions: Vec<Option<usize>>,
    ) -> Self {
        Self {
            internal: MixedLindbladNoiseSystem::new(number_spins, number_bosons, number_fermions),
        }
    }

    // /// Separate self into an operator with the terms of given number of qubits and an operator with the remaining operations.
    // ///
    // /// Args:
    // ///     number_particles_left (Tuple[int, int, int]): Number of particles to filter for in the left term of the keys.
    // ///     number_particles_right (Tuple[int, int, int]): Number of particles to filter for in the right term of the keys.
    // ///
    // /// Returns:
    // ///     int: The number of modes in self.
    // ///
    // /// Raises:
    // ///     ValueError: Operator with the noise terms where number_particles matches the number of spins the operator product acts on and Operator with all other contributions.
    // pub fn separate_into_n_terms(
    //     &self,
    //     number_particles_left: (usize, usize, usize),
    //     number_particles_right: (usize, usize, usize),
    // ) -> PyResult<(Self, Self)> {
    //     let (separated, remainder) = self
    //         .internal
    //         .separate_into_n_terms(number_particles_left, number_particles_right)
    //         .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?;
    //     Ok((
    //         Self {
    //             internal: separated,
    //         },
    //         Self {
    //             internal: remainder,
    //         },
    //     ))
    // }
}
