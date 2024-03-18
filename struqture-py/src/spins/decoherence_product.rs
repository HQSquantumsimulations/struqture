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

use crate::fermions::FermionOperatorWrapper;
use num_complex::Complex64;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use struqture::mappings::JordanWignerSpinToFermion;
use struqture::spins::{DecoherenceProduct, SingleDecoherenceOperator};
use struqture::SerializationSupport;
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture::{SpinIndex, SymmetricIndex};
use struqture_py_macros::{mappings, product_wrapper};

/// These are combinations of SingleDecoherenceOperators on specific qubits.
///
/// DecoherenceProducts act in a noisy system. They are representation of products of decoherence
/// matrices acting on qubits in order to build the terms of a hamiltonian.
/// For instance, to represent the term :math:`\sigma_0^{x}` :math:`\sigma_2^{z}`:
///
/// `DecoherenceProduct().x(0).z(2)`.
///
/// DecoherenceProduct is  supposed to be used as input for the function `add_noise`,
/// for instance in the spin system classes QubitLindbladOpenSystem, SpinLindbladNoiseSystem or QubitLindbladNoiseOperator,
/// or in the mixed systems as part of `MixedDecoherenceProduct <mixed_systems.MixedDecoherenceProduct>`.
///
/// Returns:
///     self: The new, empty DecoherenceProduct.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     from struqture_py.spins import DecoherenceProduct
///     dp = DecoherenceProduct().x(0).iy(1).z(2)
///     dp = dp.set_pauli(3, "X")
///     npt.assert_equal(dp.get(1), "iY")
///     npt.assert_equal(dp.keys(), [0, 1, 2, 3])
///
#[pyclass(name = "DecoherenceProduct", module = "struqture_py.spins")]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct DecoherenceProductWrapper {
    /// Internal storage of [struqture::spins::DecoherenceProduct]
    pub internal: DecoherenceProduct,
}

#[mappings(JordanWignerSpinToFermion)]
#[product_wrapper(SpinIndex, SymmetricIndex, Calculus)]
impl DecoherenceProductWrapper {
    /// Create an empty DecoherenceProduct.
    ///
    /// Returns:
    ///     self: The new, empty DecoherenceProduct.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: DecoherenceProduct::new(),
        }
    }

    /// Set a new entry for SingleDecoherenceOperator X in the internal dictionary.
    ///
    /// Args:
    ///     index (int): Index of set object.
    ///
    /// Returns:
    ///     DecoherenceProduct: The DecoherenceProduct with the new entry.
    pub fn x(&self, index: usize) -> DecoherenceProductWrapper {
        Self {
            internal: self.clone().internal.x(index),
        }
    }

    /// Set a new entry for SingleDecoherenceOperator iY in the internal dictionary.
    ///
    /// Args:
    ///     index (int): Index of set object.
    ///
    /// Returns:
    ///     DecoherenceProduct: The DecoherenceProduct with the new entry.
    pub fn iy(&self, index: usize) -> DecoherenceProductWrapper {
        Self {
            internal: self.clone().internal.iy(index),
        }
    }

    /// Set a new entry for SingleDecoherenceOperator Z in the internal dictionary.
    ///
    /// Args:
    ///     index (int): Index of set object.
    ///
    /// Returns:
    ///     DecoherenceProduct: The DecoherenceProduct with the new entry.
    pub fn z(&self, index: usize) -> DecoherenceProductWrapper {
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
    ///     self: The entry was correctly set and the DecoherenceProduct is returned.
    pub fn set_pauli(&self, index: usize, pauli: String) -> PyResult<Self> {
        let converted_pauli =
            SingleDecoherenceOperator::from_str(pauli.as_str()).map_err(|err| {
                PyValueError::new_err(format!(
                    "pauli could not be converted to X, iY, Z: {:?}",
                    err
                ))
            })?;
        Ok(Self {
            internal: self.internal.clone().set_pauli(index, converted_pauli),
        })
    }
}
