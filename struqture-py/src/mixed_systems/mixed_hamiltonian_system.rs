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

use super::MixedSystemWrapper;
use crate::mixed_systems::HermitianMixedProductWrapper;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
#[cfg(feature = "unstable_struqture_2_import")]
use std::str::FromStr;
#[cfg(feature = "unstable_struqture_2_import")]
use struqture::mixed_systems::HermitianMixedProduct;
use struqture::mixed_systems::{
    GetValueMixed, MixedHamiltonianSystem, MixedProduct, MixedSystem, OperateOnMixedSystems,
};
#[cfg(feature = "json_schema")]
use struqture::{MinSupportedVersion, STRUQTURE_VERSION};
use struqture::{OperateOnDensityMatrix, OperateOnState, SymmetricIndex};
use struqture_py_macros::noiseless_system_wrapper;

/// These are representations of systems of mixed_systems.
///
/// MixedHamiltonianSystems are characterized by a MixedOperator to represent the hamiltonian of the spin system
/// and an optional number of mixed_systems.
///
/// Args:
///     number_spins (List[Optional[int]]): The number of spin subsystems in the MixedHamiltonianSystem.
///     number_bosons (List[Optional[int]]): The number of boson subsystems in the MixedHamiltonianSystem.
///     number_fermions (List[Optional[int]]): The number of fermion subsystems in the MixedHamiltonianSystem.
///
/// Returns:
///     self: The new (empty) MixedHamiltonianSystem.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.mixed_systems import MixedHamiltonianSystem, HermitianMixedProduct
///     from struqture_py.spins import PauliProduct
///     from struqture_py.bosons import BosonProduct
///     from struqture_py.fermions import FermionProduct
///
///     ssystem = MixedHamiltonianSystem([2], [2], [2])
///     pp = HermitianMixedProduct([PauliProduct().z(0)], [BosonProduct([0], [1])], [FermionProduct([0], [0])])
///     ssystem.add_operator_product(pp, 5.0)
///     npt.assert_equal(ssystem.number_spins(), [2])
///     npt.assert_equal(ssystem.get(pp), CalculatorComplex(5))
///
#[pyclass(name = "MixedHamiltonianSystem", module = "struqture_py.mixed_systems")]
#[derive(Clone, Debug, PartialEq)]
pub struct MixedHamiltonianSystemWrapper {
    /// Internal storage of [struqture::mixed_systems::MixedHamiltonianSystem]
    pub internal: MixedHamiltonianSystem,
}

#[noiseless_system_wrapper(
    OperateOnMixedSystems,
    HermitianOperateOnMixedSystems,
    OperateOnState,
    OperateOnDensityMatrix,
    Calculus
)]
impl MixedHamiltonianSystemWrapper {
    /// Create an empty MixedHamiltonianSystem.
    ///
    /// Args:
    ///     number_spins (List[Optional[int]]): The number of spin subsystems in the MixedHamiltonianSystem.
    ///     number_bosons (List[Optional[int]]): The number of boson subsystems in the MixedHamiltonianSystem.
    ///     number_fermions (List[Optional[int]]): The number of fermion subsystems in the MixedHamiltonianSystem.
    ///
    /// Returns:
    ///     self: The new (empty) MixedHamiltonianSystem.
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
            internal: MixedHamiltonianSystem::new(number_spins, number_bosons, number_fermions),
        }
    }

    /// Implement `*` for MixedHamiltonianSystem and MixedHamiltonianSystem/CalculatorComplex/CalculatorFloat.
    ///
    /// Args:
    ///     value (Union[MixedHamiltonianSystem, CalculatorComplex, CalculatorFloat]): value by which to multiply the self MixedHamiltonianSystem
    ///
    /// Returns:
    ///     MixedSystem: The MixedHamiltonianSystem multiplied by the value.
    ///
    /// Raises:
    ///     ValueError: The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor MixedHamiltonianSystem.
    pub fn __mul__(&self, value: &Bound<PyAny>) -> PyResult<MixedSystemWrapper> {
        let mut new_spins: Vec<Option<usize>> = Vec::new();
        for spin in self.internal.number_spins() {
            new_spins.push(Some(spin))
        }
        let mut new_bosons: Vec<Option<usize>> = Vec::new();
        for boson in self.internal.number_bosonic_modes() {
            new_bosons.push(Some(boson))
        }
        let mut new_fermions: Vec<Option<usize>> = Vec::new();
        for fermion in self.internal.number_fermionic_modes() {
            new_fermions.push(Some(fermion))
        }
        let mut mixed_system = MixedSystem::new(new_spins, new_bosons, new_fermions);
        for (key, val) in self.internal.clone().into_iter() {
            let bp = MixedProduct::get_key(&key);
            mixed_system
                .add_operator_product(bp.clone(), val.clone())
                .expect("Internal bug in add_operator_product");
            if !key.is_natural_hermitian() {
                let bp_conj = bp.hermitian_conjugate();
                mixed_system
                    .add_operator_product(MixedProduct::get_key(&bp_conj.0), val * bp_conj.1)
                    .expect("Internal error in add_operator_product");
            }
        }
        let cc_value = qoqo_calculator_pyo3::convert_into_calculator_complex(value);
        match cc_value {
            Ok(x) => Ok(MixedSystemWrapper {
                internal: mixed_system * x,
            }),
            Err(_) => {
                let bhs_value = Self::from_pyany(value);
                match bhs_value {
                    Ok(x) => {
                        let new_self = (self.clone().internal * x).map_err(|err| {
                            PyValueError::new_err(format!(
                                "MixedHamiltonianSystems could not be multiplied: {err:?}"
                            ))
                        })?;
                        Ok(MixedSystemWrapper { internal: new_self })
                    },
                    Err(err) => Err(PyValueError::new_err(format!(
                        "The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor MixedHamiltonianSystem: {err:?}")))
                }
            }
        }
    }

    /// Converts a json corresponding to struqture 2.x MixedHamiltonian to a struqture 1.x MixedHamiltonianSystem.
    ///
    /// Args:
    ///     input (str): the json of the struqture 2.x MixedHamiltonian to convert to struqture 1.x.
    ///
    /// Returns:
    ///     MixedHamiltonianSystem: The struqture 1.x MixedHamiltonianSystem created from the struqture 2.x MixedHamiltonian.
    ///
    /// Raises:
    ///     ValueError: Input could not be deserialised from json to struqture 2.x.
    ///     ValueError: Struqture 2.x object could not be converted to struqture 1.x.
    #[staticmethod]
    #[cfg(feature = "unstable_struqture_2_import")]
    pub fn from_json_struqture_2(input: String) -> PyResult<MixedHamiltonianSystemWrapper> {
        let operator: struqture_2::mixed_systems::MixedHamiltonian = serde_json::from_str(&input)
            .map_err(|err| {
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
        let mut new_operator = MixedHamiltonianSystem::new(
            number_spin_systems,
            number_boson_systems,
            number_fermion_systems,
        );
        for (key, val) in struqture_2::OperateOnDensityMatrix::iter(&operator) {
            let self_key = HermitianMixedProduct::from_str(&format!("{}", key).to_string())
                .map_err(|err| {
                    PyValueError::new_err(format!(
                    "Struqture 2.x HermitianMixedProduct cannot be converted to struqture 1.x: {}",
                    err
                ))
                })?;
            let _ = new_operator.set(self_key, val.clone());
        }
        Ok(MixedHamiltonianSystemWrapper {
            internal: new_operator,
        })
    }
}
