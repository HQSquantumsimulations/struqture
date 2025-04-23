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

use super::{DecoherenceProductWrapper, PauliProductWrapper};
use super::{PauliHamiltonianWrapper, PauliLindbladNoiseOperatorWrapper};
use crate::fermions::FermionLindbladOpenSystemWrapper;
use crate::{to_py_coo, PyCooMatrix};
use bincode::deserialize;
use pyo3::exceptions::{PyRuntimeError, PyTypeError, PyValueError};
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator_pyo3::{CalculatorComplexWrapper, CalculatorFloatWrapper};
use struqture::mappings::JordanWignerSpinToFermion;
use struqture::spins::{OperateOnSpins, PauliLindbladOpenSystem, ToSparseMatrixSuperOperator};
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture::{OpenSystem, OperateOnDensityMatrix, StruqtureError};
use struqture_py_macros::{mappings, noisy_system_wrapper};

/// These are representations of noisy systems of spins.
///
/// In a PauliLindbladOpenSystem is characterized by a SpinLindbladOpenOperator to represent the hamiltonian of the system, and an optional number of spins.
///
/// Returns:
///     SpinLindbladOpenSystem: The new SpinLindbladOpenSystem.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     import scipy.sparse as sp
///     from qoqo_calculator_pyo3 import CalculatorComplex, CalculatorFloat
///     from struqture_py.spins import PauliLindbladOpenSystem, DecoherenceProduct
///
///     slns = PauliLindbladOpenSystem()
///     dp = DecoherenceProduct().z(0).x(1)
///     slns.system_add_operator_product(dp, 2.0)
///     npt.assert_equal(slns.current_number_spins(), 2)
///     npt.assert_equal(slns.system().get(dp), CalculatorFloat(2))
///     dimension = 4**slns.current_number_spins()
///     matrix = sp.coo_matrix(slns.sparse_matrix_superoperator_coo(slns.current_number_spins()), shape=(dimension, dimension))
///
#[pyclass(name = "PauliLindbladOpenSystem", module = "struqture_py.spins")]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct PauliLindbladOpenSystemWrapper {
    /// Internal storage of [struqture::spins::PauliLindbladOpenSystem]
    pub internal: PauliLindbladOpenSystem,
}

#[mappings(JordanWignerSpinToFermion)]
#[noisy_system_wrapper(
    OpenSystem,
    OperateOnSpins,
    ToSparseMatrixSuperOperator,
    HermitianCalculus
)]
impl PauliLindbladOpenSystemWrapper {
    /// Create a new PauliLindbladOpenSystem.
    ///
    /// Returns:
    ///     PauliLindbladOpenSystem: The new PauliLindbladOpenSystem.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: PauliLindbladOpenSystem::new(),
        }
    }
}
