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

use super::FermionProductWrapper;
use crate::{create_subscript, spins::PauliHamiltonianWrapper};
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
/// The HermitianFermionProduct is used as an index for non-hermitian, normal ordered fermionic operators.
/// A fermionic operator can be written as a sum over normal ordered products of creation and annihilation operators.
/// The HermitianFermionProduct is used as an index when setting or adding new summands to a fermionic operator and when querrying the
/// weight of a product of operators in the sum.
///
/// Args:
///     creators (List[int]): List of creator sub-indices.
///     annihilators (List[int]): List of annihilator sub-indices.
///
/// Returns:
///     self: The new (empty) HermitianFermionProduct.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     from struqture_py.fermions import HermitianFermionProduct
///     import numpy.testing as npt
///     # For instance, to represent $c_0a_1$
///     fp = HermitianFermionProduct([0], [0])
///     npt.assert_equal(fp.creators(), [0])
///     npt.assert_equal(fp.annihilators(), [0])
///     
#[pyclass(name = "HermitianFermionProduct", module = "struqture_py.fermions")]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct HermitianFermionProductWrapper {
    pub internal: HermitianFermionProduct,
}

#[mappings(JordanWignerFermionToSpin)]
#[product_wrapper(FermionIndex, ModeIndex, SymmetricIndex)]
impl HermitianFermionProductWrapper {
    /// Create a new HermitianFermionProduct.
    ///
    /// Args:
    ///     creators (List[int]): List of creator sub-indices.
    ///     annihilators (List[int]): List of annihilator sub-indices.
    ///
    /// Returns:
    ///     self: The new (empty) HermitianFermionProduct.
    #[new]
    pub fn new(creators: Vec<usize>, annihilators: Vec<usize>) -> PyResult<Self> {
        Ok(Self {
            internal: HermitianFermionProduct::new(creators, annihilators).map_err(|err| {
                PyValueError::new_err(format!(
                    "Could not construct HermitianFermionProduct: {err:?}"
                ))
            })?,
        })
    }

    /// Implement `*` for HermitianFermionProduct and HermitianFermionProduct.
    ///
    /// Args:
    ///     other (HermitianFermionProduct): value by which to multiply the self HermitianFermionProduct
    ///
    /// Returns:
    ///     List[Tuple[HermitianFermionProduct, float]]: The result of the multiplication.
    ///
    /// Raises:
    ///     ValueError: The rhs of the multiplication is not HermitianFermionProduct.
    pub fn __mul__(&self, other: Self) -> Vec<(FermionProductWrapper, f64)> {
        let vec_object = self.internal.clone() * other.internal;
        let mut return_vector: Vec<(FermionProductWrapper, f64)> = Vec::new();
        for obj in vec_object {
            return_vector.push((FermionProductWrapper { internal: obj.0 }, obj.1));
        }
        return_vector
    }

    pub fn pprint(&self) -> String {
        let mut output = String::new();
        let mut hermitian_parts: Vec<String> = Vec::new();
        for creator in self.creators() {
            output.push('c');
            let mut hermitian_part = "c".to_string();
            let creator_string = format!("{creator}");
            for char in creator_string.chars() {
                output.push(create_subscript(char));
                hermitian_part.push(create_subscript(char));
            }
            hermitian_parts.push(hermitian_part);
            output.push('\u{2020}');
        }
        for annihilator in self.annihilators() {
            output.push('c');
            let mut hermitian_part = "c".to_string();
            let annihilator_string = format!("{annihilator}");
            for char in annihilator_string.chars() {
                output.push(create_subscript(char));
                hermitian_part.push(create_subscript(char));
            }
            hermitian_part.push('\u{2020}');
            hermitian_parts.push(hermitian_part);
        }

        if output.is_empty() {
            output.push('I');
        } else {
            hermitian_parts.reverse();
            output.push_str(" + ");
            for hermitian_part in hermitian_parts {
                output.push_str(&hermitian_part);
            }
        }

        output
    }
}
