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

use crate::bosons::*;
use crate::fermions::*;
use crate::spins::*;
use num_complex::Complex64;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use pyo3::types::PyType;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use struqture::bosons::BosonProduct;
use struqture::fermions::FermionProduct;
use struqture::mixed_systems::*;
use struqture::spins::DecoherenceProduct;
use struqture::SymmetricIndex;
#[cfg(feature = "json_schema")]
use struqture::{MinSupportedVersion, STRUQTURE_VERSION};
use struqture_py_macros::product_wrapper;

/// A mixed product of pauli products and boson products.
///
/// A `DecoherenceProduct <struqture_py.spins.DecoherenceProduct>` is a representation of products of pauli matrices acting on qubits. It is used in order to build the corresponding spin terms of a hamiltonian.
///
/// A `BosonProduct <struqture_py.bosons.BosonProduct>` is a product of bosonic creation and annihilation operators.
/// It is used as an index for non-hermitian, normal ordered bosonic operators.
///
/// A `FermionProduct <struqture_py.fermions.FermionProduct>` is a product of bosonic creation and annihilation operators.
/// It is used as an index for non-hermitian, normal ordered bosonic operators.
///
/// Note: For a physical system, the `bosons` (BosonProduct) are usually considered
/// in presence of a `system-spin` part (DecoherenceProduct) and a `bath-spin` part (DecoherenceProduct),
/// as shown in the example below.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     from struqture_py.mixed_systems import MixedDecoherenceProduct
///     from struqture_py.spins import DecoherenceProduct
///     from struqture_py.bosons import BosonProduct
///     
///     # For instance, to represent the terms $\sigma_0^{x} \sigma_2^{x}$
///     # and $\sigma_1^{x} \sigma_2^{x}$
///     mp_spins_system = DecoherenceProduct().x(0).x(2)
///     mp_spins_bath = DecoherenceProduct().x(1).x(2)
///
///     # For instance, to represent $a_1*a_1$
///     mp_bosons = BosonProduct([1], [1])
///     
///     mp = MixedDecoherenceProduct([mp_spins_system, mp_spins_bath], [mp_bosons], [])
///     npt.assert_equal(mp.spins(), [mp_spins_system, mp_spins_bath])
///     npt.assert_equal(mp.bosons(), [mp_bosons])
///     
#[pyclass(
    name = "MixedDecoherenceProduct",
    module = "struqture_py.mixed_systems"
)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct MixedDecoherenceProductWrapper {
    // Internal storage of [struqture::mixed_systems::MixedDecoherenceProduct]
    pub internal: MixedDecoherenceProduct,
}

#[product_wrapper(SymmetricIndex, MixedIndex)]
impl MixedDecoherenceProductWrapper {
    /// Create a new MixedDecoherenceProduct.
    ///
    /// Args:
    ///     spins (List[DecoherenceProduct]): products of pauli matrices acting on qubits.
    ///     bosons (List[BosonProduct]): products of bosonic creation and annihilation operators.
    ///     fermions (List[FermionProduct]): products of fermionic creation and annihilation operators.
    ///
    /// Returns:
    ///     MixedDecoherenceProduct: a new MixedDecoherenceProduct with the input of spins, bosons and fermions.
    ///
    /// Raises:
    ///     ValueError: if MixedDecoherenceProduct can not be constructed from the input.
    #[new]
    pub fn new(
        spins: Vec<Py<PyAny>>,
        bosons: Vec<Py<PyAny>>,
        fermions: Vec<Py<PyAny>>,
    ) -> PyResult<Self> {
        let mut spinsv: Vec<DecoherenceProduct> = Vec::new();
        for s in spins {
            spinsv.push(DecoherenceProductWrapper::from_pyany(s)?);
        }
        let mut bosonsv: Vec<BosonProduct> = Vec::new();
        for b in bosons {
            bosonsv.push(BosonProductWrapper::from_pyany(b)?);
        }
        let mut fermionsv: Vec<FermionProduct> = Vec::new();
        for f in fermions {
            fermionsv.push(FermionProductWrapper::from_pyany(f)?);
        }
        Ok(Self {
            internal: MixedDecoherenceProduct::new(spinsv, bosonsv, fermionsv).map_err(|err| {
                PyValueError::new_err(format!(
                    "Could not construct MixedDecoherenceProduct: {:?}",
                    err
                ))
            })?,
        })
    }

    /// Create a pair (MixedDecoherenceProduct, CalculatorComplex).
    ///
    /// The first item is the valid MixedDecoherenceProduct created from the input creators and annihilators.
    /// The second term is the input CalculatorComplex transformed according to the valid order of creators and annihilators.
    ///
    /// Args:
    ///     creators: The creator indices to have in the MixedDecoherenceProduct.
    ///     annihilators: The annihilators indices to have in the MixedDecoherenceProduct.
    ///     value: The CalculatorComplex to transform.
    ///
    /// Returns:
    ///     Tuple[self, CalculatorComplex] - The valid MixedDecoherenceProduct and the corresponding transformed CalculatorComplex.
    ///
    /// Raises:
    ///     ValueError: Valid pair could not be constructed, spins couldn't be converted from string.
    ///     ValueError: Valid pair could not be constructed, bosons couldn't be converted from string.
    ///     ValueError: Valid pair could not be constructed, fermions couldn't be converted from string.
    ///     TypeError: Value cannot be converted to CalculatorComplex.
    ///     ValueError: Valid pair could not be constructed.
    #[classmethod]
    pub fn create_valid_pair(
        _cls: &PyType,
        spins: Vec<String>,
        bosons: Vec<String>,
        fermions: Vec<String>,
        value: &PyAny,
    ) -> PyResult<(Self, qoqo_calculator_pyo3::CalculatorComplexWrapper)> {
        let mut converted_spins: Vec<DecoherenceProduct> = Vec::new();
        for s in spins {
            match DecoherenceProduct::from_str(s.as_str()) {
                Ok(x) => converted_spins.push(x),
                Err(err) => return Err(PyValueError::new_err(format!("Valid pair could not be constructed, pauli spins couldn't be converted from string: {:?}", err)))
            }
        }
        let mut converted_bosons: Vec<BosonProduct> = Vec::new();
        for b in bosons {
            match BosonProduct::from_str(b.as_str()) {
                Ok(x) => converted_bosons.push(x),
                Err(err) => return Err(PyValueError::new_err(format!("Valid pair could not be constructed, bosons couldn't be converted from string: {:?}", err)))
            }
        }
        let mut converted_fermions: Vec<FermionProduct> = Vec::new();
        for f in fermions {
            match FermionProduct::from_str(f.as_str()) {
                Ok(x) => converted_fermions.push(x),
                Err(err) => return Err(PyValueError::new_err(format!("Valid pair could not be constructed, fermions couldn't be converted from string: {:?}", err)))
            }
        }

        let value = qoqo_calculator_pyo3::convert_into_calculator_complex(value)
            .map_err(|_| PyTypeError::new_err("Value is not CalculatorComplex"))?;
        let (index, value) = MixedDecoherenceProduct::create_valid_pair(
            converted_spins,
            converted_bosons,
            converted_fermions,
            value,
        )
        .map_err(|err| {
            PyValueError::new_err(format!("Valid pair could not be constructed: {:?}", err))
        })?;
        Ok((
            Self { internal: index },
            qoqo_calculator_pyo3::CalculatorComplexWrapper { internal: value },
        ))
    }

    /// Implement `*` for MixedDecoherenceProduct and MixedDecoherenceProduct.
    ///
    /// Args:
    ///     other (MixedDecoherenceProduct): value by which to multiply the self MixedDecoherenceProduct
    ///
    /// Returns:
    ///     MixedDecoherenceProduct: The MixedDecoherenceProduct multiplied by the value.
    ///
    /// Raises:
    ///     ValueError: The rhs of the multiplication not MixedDecoherenceProduct.
    pub fn __mul__(&self, other: Self) -> PyResult<Vec<(Self, Complex64)>> {
        let vec_object = (self.internal.clone() * other.internal).map_err(|err| {
            PyValueError::new_err(format!(
                "Could not multiply the two MixedDecoherenceProducts: {:?}",
                err
            ))
        })?;
        let mut return_vector: Vec<(Self, Complex64)> = Vec::new();
        for obj in vec_object {
            return_vector.push((Self { internal: obj.0 }, obj.1));
        }
        Ok(return_vector)
    }
}
