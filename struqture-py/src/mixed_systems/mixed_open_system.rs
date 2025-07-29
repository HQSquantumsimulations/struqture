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

use super::{
    HermitianMixedProductWrapper, MixedDecoherenceProductWrapper, MixedHamiltonianSystemWrapper,
    MixedLindbladNoiseSystemWrapper,
};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
#[cfg(feature = "unstable_struqture_2_import")]
use std::str::FromStr;
#[cfg(feature = "unstable_struqture_2_import")]
use struqture::mixed_systems::{HermitianMixedProduct, MixedDecoherenceProduct};
use struqture::mixed_systems::{MixedLindbladOpenSystem, OperateOnMixedSystems};
#[cfg(feature = "json_schema")]
use struqture::{MinSupportedVersion, STRUQTURE_VERSION};
use struqture::{OpenSystem, OperateOnDensityMatrix};
use struqture_py_macros::noisy_system_wrapper;

/// These are representations of noisy systems of mixed_systems.
///
/// In a MixedLindbladOpenSystem is characterized by a MixedLindbladOpenOperator to represent the hamiltonian of the system, and an optional number of mixed_systems.
///
/// Args:
///     number_spins (List[Optional[int]]): The number of spin subsystems in the MixedLindbladOpenSystem.
///     number_bosons (List[Optional[int]]): The number of boson subsystems in the MixedLindbladOpenSystem.
///     number_fermions (List[Optional[int]]): The number of fermion subsystems in the MixedLindbladOpenSystem.
///
/// Returns:
///     self: The new MixedLindbladOpenSystem.
///
/// Examples
/// --------
///
/// .. code-block:: python
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex, CalculatorFloat
///     from struqture_py.mixed_systems import MixedLindbladOpenSystem
///     from struqture_py.spins import DecoherenceProduct
///     from struqture_py.bosons import BosonProduct
///     from struqture_py.fermions import FermionProduct
///
///     slns = MixedLindbladOpenSystem()
///     dp = MixedDecoherenceProduct([DecoherenceProduct().z(0)], [BosonProduct([0], [1])], [FermionProduct([0], [0])])
///     slns.noise_add_operator_product((dp, dp), 2.0)
///     npt.assert_equal(slns.current_number_spins(), [1])
///     npt.assert_equal(slns.noise().get((dp, dp)), CalculatorFloat(2))
///
#[pyclass(
    name = "MixedLindbladOpenSystem",
    module = "struqture_py.mixed_systems"
)]
#[derive(Clone, Debug, PartialEq)]
pub struct MixedLindbladOpenSystemWrapper {
    /// Internal storage of [struqture::mixed_systems::MixedLindbladOpenSystem]
    pub internal: MixedLindbladOpenSystem,
}

#[noisy_system_wrapper(OpenSystem, OperateOnMixedSystems, Calculus)]
impl MixedLindbladOpenSystemWrapper {
    /// Create a new MixedLindbladOpenSystem.
    ///
    /// Args:
    ///     number_spins (List[Optional[int]]): The number of spin subsystems in the MixedLindbladOpenSystem.
    ///     number_bosons (List[Optional[int]]): The number of boson subsystems in the MixedLindbladOpenSystem.
    ///     number_fermions (List[Optional[int]]): The number of fermion subsystems in the MixedLindbladOpenSystem.
    ///
    /// Returns:
    ///     self: The new MixedLindbladOpenSystem.
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
            internal: MixedLindbladOpenSystem::new(number_spins, number_bosons, number_fermions),
        }
    }

    /// Converts a json corresponding to struqture 2.x MixedLindbladOpenSystem to a struqture 1.x MixedLindbladOpenSystem.
    ///
    /// Args:
    ///     input (str): the json of the struqture 2.x MixedLindbladOpenSystem to convert to struqture 1.x.
    ///
    /// Returns:
    ///     MixedLindbladOpenSystem: The struqture 1.x MixedLindbladOpenSystem created from the struqture 2.x MixedLindbladOpenSystem.
    ///
    /// Raises:
    ///     ValueError: Input could not be deserialised from json to struqture 2.x.
    ///     ValueError: Struqture 2.x object could not be converted to struqture 1.x.
    #[staticmethod]
    #[cfg(feature = "unstable_struqture_2_import")]
    pub fn from_json_struqture_2(input: String) -> PyResult<MixedLindbladOpenSystemWrapper> {
        let operator: struqture_2::mixed_systems::MixedLindbladOpenSystem =
            serde_json::from_str(&input).map_err(|err| {
                PyValueError::new_err(format!(
                    "Input cannot be deserialized from json to struqture 2.x: {}",
                    err
                ))
            })?;
        let number_spin_systems =
            struqture_2::mixed_systems::OperateOnMixedSystems::current_number_spins(&operator)
                .into_iter()
                .map(|_| None);
        let number_boson_systems =
            struqture_2::mixed_systems::OperateOnMixedSystems::current_number_bosonic_modes(
                &operator,
            )
            .into_iter()
            .map(|_| None);
        let number_fermion_systems =
            struqture_2::mixed_systems::OperateOnMixedSystems::current_number_fermionic_modes(
                &operator,
            )
            .into_iter()
            .map(|_| None);
        let mut new_operator = MixedLindbladOpenSystem::new(
            number_spin_systems,
            number_boson_systems,
            number_fermion_systems,
        );
        let system = struqture_2::OpenSystem::system(&operator);
        for (key, val) in struqture_2::OperateOnDensityMatrix::iter(system) {
            let self_key = HermitianMixedProduct::from_str(&format!("{}", key).to_string()).map_err(
                |err| {
                    PyValueError::new_err(format!(
                        "Struqture 2.x HermitianMixedProduct cannot be converted to struqture 1.x: {}",
                        err
                    ))
                },
            )?;
            let _ = new_operator.system_mut().set(self_key, val.clone());
        }
        let noise = struqture_2::OpenSystem::noise(&operator);
        for (key, val) in struqture_2::OperateOnDensityMatrix::iter(noise) {
            let self_key_left =
                MixedDecoherenceProduct::from_str(&format!("{}", key.0).to_string()).map_err(
                    |err| {
                        PyValueError::new_err(format!(
                            "Struqture 2.x MixedDecoherenceProduct cannot be converted to struqture 1.x: {}",
                            err
                        ))
                    },
                )?;
            let self_key_right =
                MixedDecoherenceProduct::from_str(&format!("{}", key.1).to_string()).map_err(
                    |err| {
                        PyValueError::new_err(format!(
                            "Struqture 2.x MixedDecoherenceProduct cannot be converted to struqture 1.x: {}",
                            err
                        ))
                    },
                )?;
            let _ = new_operator
                .noise_mut()
                .set((self_key_left, self_key_right), val.clone());
        }
        Ok(MixedLindbladOpenSystemWrapper {
            internal: new_operator,
        })
    }
}
