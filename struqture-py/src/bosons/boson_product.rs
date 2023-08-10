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

use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use pyo3::types::PyType;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use struqture::bosons::*;
use struqture::prelude::*;
#[cfg(feature = "json_schema")]
use struqture::{MinSupportedVersion, STRUQTURE_VERSION};
use struqture_py_macros::product_wrapper;

/// A product of bosonic creation and annihilation operators.
///
/// The BosonProduct is used as an index for non-hermitian, normal ordered bosonic operators.
/// A bosonic operator can be written as a sum over normal ordered products of creation and annihilation operators.
/// The BosonProduct is used as an index when setting or adding new summands to a bosonic operator and when querrying the
/// weight of a product of operators in the sum.
///
/// Example:
/// --------
///
/// .. code-block:: python
///
///     from struqture_py.bosons import BosonProduct
///     import numpy.testing as npt
///     # For instance, to represent $c_0a_0$
///     b_product = BosonProduct([0], [0])
///     npt.assert_equal(b_product.creators(), [0])
///     npt.assert_equal(b_product.annihilators(), [0])
///     
#[pyclass(name = "BosonProduct", module = "struqture_py.bosons")]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct BosonProductWrapper {
    pub internal: BosonProduct,
}

#[product_wrapper(BosonIndex, ModeIndex, SymmetricIndex)]
impl BosonProductWrapper {
    /// Create a new BosonProduct.
    ///
    /// Args:
    ///     creators (List[int]): List of creator sub-indices.
    ///     annihilators (List[int]): List of annihilator sub-indices.
    ///
    /// Returns:
    ///     self: The new (empty) BosonProduct.
    #[new]
    pub fn new(creators: Vec<usize>, annihilators: Vec<usize>) -> PyResult<Self> {
        Ok(Self {
            internal: BosonProduct::new(creators, annihilators).unwrap(),
        })
    }

    /// Implement `*` for BosonProduct and BosonProduct.
    ///
    /// Args:
    ///     other (BosonProduct): value by which to multiply the self BosonProduct
    ///
    /// Returns:
    ///     List[BosonProduct]: The result of the multiplication.
    ///
    /// Raises:
    ///     ValueError: The rhs of the multiplication is not BosonProduct.
    pub fn __mul__(&self, other: Self) -> Vec<Self> {
        let vec_object = self.internal.clone() * other.internal;
        let mut return_vector: Vec<Self> = Vec::new();
        for obj in vec_object {
            return_vector.push(Self { internal: obj });
        }
        return_vector
    }
}
