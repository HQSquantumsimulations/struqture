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
use qoqo_calculator::CalculatorComplex;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use struqture::bosons::BosonProduct;
use struqture::fermions::FermionProduct;
use struqture::mixed_systems::*;
use struqture::spins::PlusMinusProduct;
use struqture::SymmetricIndex;
use struqture_py_macros::product_wrapper;

use super::MixedProductWrapper;
use struqture::SerializationSupport;
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;

/// A mixed product of pauli products and boson products.
///
/// A `PlusMinusProduct <struqture_py.spins.PlusMinusProduct>` is a representation of products of pauli matrices acting on qubits. It is used in order to build the corresponding spin terms of a hamiltonian.
///
/// A `BosonProduct <struqture_py.bosons.BosonProduct>` is a product of bosonic creation and annihilation operators.
/// It is used as an index for non-hermitian, normal ordered bosonic operators.
///
/// A `FermionProduct <struqture_py.fermions.FermionProduct>` is a product of bosonic creation and annihilation operators.
/// It is used as an index for non-hermitian, normal ordered bosonic operators.
///
/// Note: For a physical system, the `bosons` (BosonProduct) are usually considered
/// in presence of a `system-spin` part (PlusMinusProduct) and a `bath-spin` part (PlusMinusProduct),
/// as shown in the example below.
///
/// Args:
///     spins (List[PlusMinusProduct]): Products of pauli operators acting on qubits.
///     bosons (List[BosonProduct]): Products of bosonic creation and annihilation operators.
///     fermions (List[FermionProduct]): Products of fermionic creation and annihilation operators.
///
/// Returns:
///     MixedPlusMinusProduct: a new MixedPlusMinusProduct with the input of spins, bosons and fermions.
///
/// Raises:
///     ValueError: MixedPlusMinusProduct can not be constructed from the input.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     from struqture_py.mixed_systems import MixedPlusMinusProduct
///     from struqture_py.spins import PlusMinusProduct
///     from struqture_py.bosons import BosonProduct
///     
///     # For instance, to represent the terms $\sigma_0^{x} \sigma_2^{x}$
///     # and $\sigma_1^{x} \sigma_2^{x}$
///     mp_spins_system = PlusMinusProduct().x(0).x(2)
///     mp_spins_bath = PlusMinusProduct().x(1).x(2)
///
///     # For instance, to represent $a_1*a_1$
///     mp_bosons = BosonProduct([1], [1])
///     
///     mp = MixedPlusMinusProduct([mp_spins_system, mp_spins_bath], [mp_bosons], [])
///     npt.assert_equal(mp.spins(), [mp_spins_system, mp_spins_bath])
///     npt.assert_equal(mp.bosons(), [mp_bosons])
///     
#[pyclass(name = "MixedPlusMinusProduct", module = "struqture_py.mixed_systems")]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct MixedPlusMinusProductWrapper {
    // Internal storage of [struqture::mixed_systems::MixedPlusMinusProduct]
    pub internal: MixedPlusMinusProduct,
}

#[product_wrapper(SymmetricIndex, MixedIndex)]
impl MixedPlusMinusProductWrapper {
    /// Create a new MixedPlusMinusProduct.
    ///
    /// Args:
    ///     spins (List[PlusMinusProduct]): Products of pauli operators acting on qubits.
    ///     bosons (List[BosonProduct]): Products of bosonic creation and annihilation operators.
    ///     fermions (List[FermionProduct]): Products of fermionic creation and annihilation operators.
    ///
    /// Returns:
    ///     MixedPlusMinusProduct: a new MixedPlusMinusProduct with the input of spins, bosons and fermions.
    ///
    /// Raises:
    ///     ValueError: MixedPlusMinusProduct can not be constructed from the input.
    #[new]
    pub fn new(
        spins: Vec<Py<PyAny>>,
        bosons: Vec<Py<PyAny>>,
        fermions: Vec<Py<PyAny>>,
    ) -> PyResult<Self> {
        let mut spinsv: Vec<PlusMinusProduct> = Vec::new();
        let mut bosonsv: Vec<BosonProduct> = Vec::new();
        let mut fermionsv: Vec<FermionProduct> = Vec::new();
        Python::with_gil(|py| -> PyResult<()> {
            for s in spins {
                spinsv.push(PlusMinusProductWrapper::from_pyany(s.bind(py))?);
            }
            for b in bosons {
                bosonsv.push(BosonProductWrapper::from_pyany(b.bind(py))?);
            }
            for f in fermions {
                fermionsv.push(FermionProductWrapper::from_pyany(f.bind(py))?);
            }
            Ok(())
        })?;
        Ok(Self {
            internal: MixedPlusMinusProduct::new(spinsv, bosonsv, fermionsv),
        })
    }

    /// Creates a list of corresponding (MixedPlusMinusProduct, CalculatorComplex) tuples from the input MixedProduct.
    ///
    /// Args:
    ///     value (MixedProduct): The MixedProduct object to convert.
    ///
    /// Returns:
    ///     List[Tuple[(MixedPlusMinusProduct, CalculatorComplex)]]: The converted input.
    ///
    /// Raises:
    ///     ValueError: Input is not a MixedProduct.
    #[staticmethod]
    pub fn from_mixed_product(
        value: &Bound<PyAny>,
    ) -> PyResult<Vec<(MixedPlusMinusProductWrapper, CalculatorComplexWrapper)>> {
        match MixedProductWrapper::from_pyany(value) {
            Ok(x) => {
                let result: Vec<(MixedPlusMinusProduct, Complex64)> =
                    Vec::<(MixedPlusMinusProduct, Complex64)>::from(x);
                let result_pyo3: Vec<(MixedPlusMinusProductWrapper, CalculatorComplexWrapper)> =
                    result
                        .iter()
                        .map(|(key, val)| {
                            (
                                MixedPlusMinusProductWrapper {
                                    internal: key.clone(),
                                },
                                CalculatorComplexWrapper {
                                    internal: CalculatorComplex::new(val.re, val.im),
                                },
                            )
                        })
                        .collect();
                Ok(result_pyo3)
            }
            Err(_) => Err(PyValueError::new_err("Input is not a MixedProduct")),
        }
    }

    /// Convert the `self` instance to the corresponding list of (MixedProduct, CalculatorComplex) instances.
    ///
    /// Returns:
    ///     List[Tuple[(MixedProduct, CalculatorComplex)]]: The converted MixedPlusMinusProduct.
    ///
    /// Raises:
    ///     ValueError: The conversion was not successful.
    pub fn to_mixed_product_list(
        &self,
    ) -> PyResult<Vec<(MixedProductWrapper, CalculatorComplexWrapper)>> {
        let result: Vec<(MixedProduct, Complex64)> =
            Vec::<(MixedProduct, Complex64)>::try_from(self.internal.clone()).map_err(|err| {
                PyValueError::new_err(format!("The conversion was not successful: {:?}", err))
            })?;
        let result_pyo3: Vec<(MixedProductWrapper, CalculatorComplexWrapper)> = result
            .iter()
            .map(|(key, val)| {
                (
                    MixedProductWrapper {
                        internal: key.clone(),
                    },
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::new(val.re, val.im),
                    },
                )
            })
            .collect();
        Ok(result_pyo3)
    }
}
