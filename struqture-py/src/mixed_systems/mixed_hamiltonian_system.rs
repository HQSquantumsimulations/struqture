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
use bincode::deserialize;
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
#[cfg(feature = "unstable_struqture_2_import")]
use struqture::StruqtureError;
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
                                "MixedHamiltonianSystems could not be multiplied: {:?}",
                                err
                            ))
                        })?;
                        Ok(MixedSystemWrapper { internal: new_self })
                    },
                    Err(err) => Err(PyValueError::new_err(format!(
                        "The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor MixedHamiltonianSystem: {:?}",
                        err)))
                }
            }
        }
    }

    /// Converts a struqture 2.x MixedHamiltonian to a struqture 1.x MixedHamiltonianSystem.
    ///
    /// Args:
    ///     input (MixedHamiltonian): The struqture 2.x MixedHamiltonian to convert to struqture 1.x.
    ///
    /// Returns:
    ///     MixedHamiltonianSystem: The struqture 1.x MixedHamiltonianSystem created from the struqture 2.x MixedHamiltonian.
    ///
    /// Raises:
    ///     TypeError: If the input is not a struqture 2.x MixedHamiltonian.
    ///     ValueError: Conversion failed.
    #[staticmethod]
    #[cfg(feature = "unstable_struqture_2_import")]
    pub fn from_struqture_2(input: &Bound<PyAny>) -> PyResult<MixedHamiltonianSystemWrapper> {
        Python::with_gil(|_| -> PyResult<MixedHamiltonianSystemWrapper> {
            let error_message = "Trying to use Python object as a struqture-py object that does not behave as struqture-py object. Are you sure you have the right type?".to_string();
            let source_serialisation_meta = input
                .call_method0("_get_serialisation_meta")
                .map_err(|_| PyTypeError::new_err(error_message.clone()))?;
            let source_serialisation_meta: String = source_serialisation_meta
                .extract()
                .map_err(|_| PyTypeError::new_err(error_message.clone()))?;

            let source_serialisation_meta: struqture_2::StruqtureSerialisationMeta =
                serde_json::from_str(&source_serialisation_meta)
                    .map_err(|_| PyTypeError::new_err(error_message))?;

            let target_serialisation_meta = <struqture_2::mixed_systems::MixedHamiltonian as struqture_2::SerializationSupport>::target_serialisation_meta();

            struqture_2::check_can_be_deserialised(
                &target_serialisation_meta,
                &source_serialisation_meta,
            )
            .map_err(|err| PyTypeError::new_err(err.to_string()))?;

            let get_bytes = input
                .call_method0("to_bincode")
                .map_err(|_| PyTypeError::new_err("Serialisation failed".to_string()))?;
            let bytes = get_bytes
                .extract::<Vec<u8>>()
                .map_err(|_| PyTypeError::new_err("Deserialisation failed".to_string()))?;
            let two_import: struqture_2::mixed_systems::MixedHamiltonian = deserialize(&bytes[..])
                .map_err(|err| PyTypeError::new_err(format!("Type conversion failed: {}", err)))?;
            let number_spins: usize = <struqture_2::mixed_systems::MixedHamiltonian as struqture_2::mixed_systems::OperateOnMixedSystems>::current_number_spins(&two_import).len();
            let spin_systems: Vec<Option<usize>> = vec![None; number_spins];
            let number_bosons: usize = <struqture_2::mixed_systems::MixedHamiltonian as struqture_2::mixed_systems::OperateOnMixedSystems>::current_number_bosonic_modes(&two_import).len();
            let bosonic_systems: Vec<Option<usize>> = vec![None; number_bosons];
            let number_fermions: usize = <struqture_2::mixed_systems::MixedHamiltonian as struqture_2::mixed_systems::OperateOnMixedSystems>::current_number_fermionic_modes(&two_import).len();
            let fermionic_systems: Vec<Option<usize>> = vec![None; number_fermions];
            let mut fermion_system: MixedHamiltonianSystem = MixedHamiltonianSystem::new(
                spin_systems.iter().cloned(),
                bosonic_systems.iter().cloned(),
                fermionic_systems.iter().cloned(),
            );
            for (key, val) in struqture_2::OperateOnDensityMatrix::iter(&two_import) {
                let value_string = key.to_string();
                let self_key = HermitianMixedProduct::from_str(&value_string).map_err(
                    |_err: StruqtureError| PyValueError::new_err(
                        "Trying to obtain struqture 1.x HermitianMixedProduct from struqture 2.x HermitianMixedProduct. Conversion failed. Was the right type passed to all functions?".to_string()
                ))?;

                fermion_system
                    .set(self_key, val.clone())
                    .map_err(|_err: StruqtureError| {
                        PyValueError::new_err(
                            "Could not set key in resulting 1.x MixedHamiltonianSystem".to_string(),
                        )
                    })?;
            }

            Ok(MixedHamiltonianSystemWrapper {
                internal: fermion_system,
            })
        })
    }
}
