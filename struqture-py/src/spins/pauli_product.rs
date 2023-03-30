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

use num_complex::Complex64;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use struqture::spins::{PauliProduct, SingleSpinOperator};
use struqture::{SpinIndex, SymmetricIndex};
use struqture_py_macros::product_wrapper;

/// PauliProducts are combinations of SingleSpinOperators on specific qubits.
///
/// PauliProducts can be used in either noise-free or a noisy system.
/// They are representations of products of pauli matrices acting on qubits,
/// in order to build the terms of a hamiltonian.
/// For instance, to represent the term :math:`\sigma_0^{x}` :math:`\sigma_2^{x}` :
///
/// `PauliProduct().x(0).x(2)`.
///
/// PauliProduct is  supposed to be used as input for the function `set_pauli_product`,
/// for instance in the spin system classes SpinLindbladOpenSystem, SpinHamiltonianSystem or SpinSystem,
/// or in the mixed systems as part of `MixedProduct <mixed_systems.MixedProduct>`
/// or as part of `HermitianMixedProduct <mixed_systems.HermitianMixedProduct>`.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     from struqture_py.spins import PauliProduct
///     pp = PauliProduct().x(0).y(1).z(2)
///     pp = pp.set_pauli(3, "X")
///     npt.assert_equal(pp.get(0), "X")
///     npt.assert_equal(pp.keys(), [0, 1, 2, 3])
///
#[pyclass(name = "PauliProduct", module = "struqture_py.spins")]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PauliProductWrapper {
    /// Internal storage of [struqture::spins::PauliProduct]
    pub internal: PauliProduct,
}

#[product_wrapper(SpinIndex, SymmetricIndex)]
impl PauliProductWrapper {
    /// Create an empty PauliProduct.
    ///
    /// Returns:
    ///     self: The new, empty PauliProduct.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: PauliProduct::new(),
        }
    }

    /// Set a new entry for SingleSpinOperator X in the internal dictionary.
    ///
    /// Args:
    ///     index (int): Index of set object.
    ///
    /// Returns:
    ///     PauliProduct: The PauliProduct with the new entry.
    pub fn x(&self, index: usize) -> PauliProductWrapper {
        Self {
            internal: self.clone().internal.x(index),
        }
    }

    /// Set a new entry for SingleSpinOperator Y in the internal dictionary.
    ///
    /// Args:
    ///     index (int): Index of set object.
    ///
    /// Returns:
    ///     PauliProduct: The PauliProduct with the new entry.
    pub fn y(&self, index: usize) -> PauliProductWrapper {
        Self {
            internal: self.clone().internal.y(index),
        }
    }

    /// Set a new entry for SingleSpinOperator Z in the internal dictionary.
    ///
    /// Args:
    ///     index (int): Index of set object.
    ///
    /// Returns:
    ///     PauliProduct: The PauliProduct with the new entry.
    pub fn z(&self, index: usize) -> PauliProductWrapper {
        Self {
            internal: self.clone().internal.z(index),
        }
    }

    /// Set a new entry in the internal_map. This function consumes self.
    ///
    /// Args:
    ///     index (int): Index of set object.
    ///     pauli (str): Value of set object.
    ///
    /// Returns:
    ///     self: The entry was correctly set and the PauliProduct is returned.
    pub fn set_pauli(&self, index: usize, pauli: String) -> PyResult<Self> {
        let converted_pauli = SingleSpinOperator::from_str(pauli.as_str()).map_err(|err| {
            PyValueError::new_err(format!(
                "pauli could not be converted to X, Y, Z: {:?}",
                err
            ))
        })?;
        Ok(Self {
            internal: self.internal.clone().set_pauli(index, converted_pauli),
        })
    }
}
