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

use crate::spins::PauliOperatorWrapper;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use pyo3::types::PyType;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use struqture::fermions::*;
use struqture::mappings::JordanWignerFermionToSpin;
use struqture::prelude::*;
use struqture::SerializationSupport;
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture_py_macros::{mappings, product_wrapper};

/// A product of fermionic creation and annihilation operators.
///
/// The FermionProduct is used as an index for non-hermitian, normal ordered fermionic operators.
/// A fermionic operator can be written as a sum over normal ordered products of creation and annihilation operators.
/// The FermionProduct is used as an index when setting or adding new summands to a fermionic operator and when querrying the
/// weight of a product of operators in the sum.
///
/// Args:
///     creators (List[int]): List of creator sub-indices.
///     annihilators (List[int]): List of annihilator sub-indices.
///
/// Returns:
///     self: The new (empty) FermionProduct.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     from struqture_py.fermions import FermionProduct
///     import numpy.testing as npt
///     # For instance, to represent $c_0a_0$
///     fp = FermionProduct([0], [0])
///     npt.assert_equal(fp.creators(), [0])
///     npt.assert_equal(fp.annihilators(), [0])
///     
#[pyclass(name = "FermionProduct", module = "struqture_py.fermions")]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct FermionProductWrapper {
    pub internal: FermionProduct,
}

#[mappings(JordanWignerFermionToSpin)]
#[product_wrapper(FermionIndex, ModeIndex, SymmetricIndex)]
impl FermionProductWrapper {
    /// Create a new FermionProduct.
    ///
    /// Args:
    ///     creators (List[int]): List of creator sub-indices.
    ///     annihilators (List[int]): List of annihilator sub-indices.
    ///
    /// Returns:
    ///     self: The new (empty) FermionProduct.
    #[new]
    pub fn new(creators: Vec<usize>, annihilators: Vec<usize>) -> PyResult<Self> {
        Ok(Self {
            internal: FermionProduct::new(creators, annihilators).map_err(|err| {
                PyValueError::new_err(format!("Could not construct FermionProduct: {:?}", err))
            })?,
        })
    }

    /// Implement `*` for FermionProduct and FermionProduct.
    ///
    /// Args:
    ///     other (FermionProduct): value by which to multiply the self FermionProduct
    ///
    /// Returns:
    ///     List[Tuple[FermionProduct, float]]: The result of the multiplication.
    ///
    /// Raises:
    ///     ValueError: The rhs of the multiplication is not FermionProduct.
    pub fn __mul__(&self, other: Self) -> Vec<(Self, f64)> {
        let vec_object = self.internal.clone() * other.internal;
        let mut return_vector: Vec<(Self, f64)> = Vec::new();
        for obj in vec_object {
            return_vector.push((Self { internal: obj.0 }, obj.1));
        }
        return_vector
    }
}
