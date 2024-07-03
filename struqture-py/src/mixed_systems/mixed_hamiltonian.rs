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
use crate::mixed_systems::HermitianMixedProductWrapper;
use bincode::deserialize;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::mixed_systems::{
    GetValueMixed, MixedHamiltonian, MixedOperator, MixedProduct, OperateOnMixedSystems,
};
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture::{OperateOnDensityMatrix, OperateOnState, SymmetricIndex};
use struqture_py_macros::noiseless_system_wrapper;

/// These are representations of systems of mixed_systems.
///
/// MixedHamiltonians are characterized by a MixedOperator to represent the hamiltonian of the spin system
/// and an optional number of mixed_systems.
///
/// Args:
///     number_spins (int): The number of spin subsystems in the MixedHamiltonian.
///     number_bosons (int): The number of boson subsystems in the MixedHamiltonian.
///     number_fermions (int): The number of fermion subsystems in the MixedHamiltonian.
///
/// Returns:
///     self: The new (empty) MixedHamiltonian.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.mixed_systems import MixedHamiltonian, HermitianMixedProduct
///     from struqture_py.spins import PauliProduct
///     from struqture_py.bosons import BosonProduct
///     from struqture_py.fermions import FermionProduct
///
///     ssystem = MixedHamiltonian(1, 1, 1)
///     pp = HermitianMixedProduct([PauliProduct().z(0)], [BosonProduct([0], [1])], [FermionProduct([0], [0])])
///     ssystem.add_operator_product(pp, 5.0)
///     npt.assert_equal(ssystem.current_number_spins(), [2])
///     npt.assert_equal(ssystem.get(pp), CalculatorComplex(5))
///
#[pyclass(name = "MixedHamiltonian", module = "struqture_py.mixed_systems")]
#[derive(Clone, Debug, PartialEq)]
pub struct MixedHamiltonianWrapper {
    /// Internal storage of [struqture::mixed_systems::MixedHamiltonian]
    pub internal: MixedHamiltonian,
}

#[noiseless_system_wrapper(
    OperateOnMixedSystems,
    HermitianOperateOnMixedSystems,
    OperateOnState,
    OperateOnDensityMatrix,
    HermitianCalculus
)]
impl MixedHamiltonianWrapper {
    /// Create an empty MixedHamiltonian.
    ///
    /// Args:
    ///     number_spins (int): The number of spin subsystems in the MixedHamiltonian.
    ///     number_bosons (int): The number of boson subsystems in the MixedHamiltonian.
    ///     number_fermions (int): The number of fermion subsystems in the MixedHamiltonian.
    ///
    /// Returns:
    ///     self: The new (empty) MixedHamiltonian.
    #[new]
    #[pyo3(signature = (
        number_spins,
        number_bosons,
        number_fermions,
    ))]
    pub fn new(number_spins: usize, number_bosons: usize, number_fermions: usize) -> Self {
        Self {
            internal: MixedHamiltonian::new(number_spins, number_bosons, number_fermions),
        }
    }

    /// Implement `*` for MixedHamiltonian and MixedHamiltonian/CalculatorComplex/CalculatorFloat.
    ///
    /// Args:
    ///     value (Union[MixedHamiltonian, CalculatorComplex, CalculatorFloat]): value by which to multiply the self MixedHamiltonian
    ///
    /// Returns:
    ///     MixedOperator: The MixedHamiltonian multiplied by the value.
    ///
    /// Raises:
    ///     ValueError: The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor MixedHamiltonian.
    pub fn __mul__(&self, value: &Bound<PyAny>) -> PyResult<MixedOperatorWrapper> {
        let mut mixed_system = MixedOperator::new(
            self.current_number_spins().len(),
            self.current_number_bosonic_modes().len(),
            self.current_number_fermionic_modes().len(),
        );
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
            Ok(x) => Ok(MixedOperatorWrapper {
                internal: mixed_system * x,
            }),
            Err(_) => {
                let bhs_value = Self::from_pyany(value);
                match bhs_value {
                    Ok(x) => {
                        let new_self = (self.clone().internal * x).map_err(|err| {
                            PyValueError::new_err(format!(
                                "MixedHamiltonians could not be multiplied: {:?}",
                                err
                            ))
                        })?;
                        Ok(MixedOperatorWrapper { internal: new_self })
                    },
                    Err(err) => Err(PyValueError::new_err(format!(
                        "The rhs of the multiplication is neither CalculatorFloat, CalculatorComplex, nor MixedHamiltonian: {:?}",
                        err)))
                }
            }
        }
    }
}
