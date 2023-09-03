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

use crate::fermions::FermionSystemWrapper;
use num_complex::Complex64;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyByteArray};
use qoqo_calculator::CalculatorComplex;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use struqture::fermions::FermionSystem;
use struqture::mappings::JordanWignerSpinToFermion;
use struqture::spins::{
    DecoherenceProduct, PauliProduct, PlusMinusProduct, SinglePlusMinusOperator,
};
use struqture::SymmetricIndex;
use struqture_py_macros::{mappings, product_wrapper};

use super::{DecoherenceProductWrapper, PauliProductWrapper};
#[cfg(feature = "json_schema")]
use struqture::{MinSupportedVersion, STRUQTURE_VERSION};

/// PlusMinusProducts are combinations of SinglePlusMinusOperators on specific qubits.
///
/// PlusMinusProducts can be used in either noise-free or a noisy system.
/// They are representations of products of pauli matrices acting on qubits,
/// in order to build the terms of a hamiltonian.
/// For instance, to represent the term :math:`\sigma_0^{+}` :math:`\sigma_2^{+}` :
///
/// `PlusMinusProduct().plus(0).plus(2)`.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     from struqture_py.spins import PlusMinusProduct
///
///     pp = PlusMinusProduct().plus(0).minus(1).z(2)
///     pp = pp.set_pauli(3, "+")
///     npt.assert_equal(pp.get(0), "+")
///     npt.assert_equal(pp.keys(), [0, 1, 2, 3])
///
#[pyclass(name = "PlusMinusProduct", module = "struqture_py.spins")]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PlusMinusProductWrapper {
    /// Internal storage of [struqture::spins::PlusMinusProduct]
    pub internal: PlusMinusProduct,
}

#[mappings(JordanWignerSpinToFermion)]
#[product_wrapper(SymmetricIndex)]
impl PlusMinusProductWrapper {
    /// Create an empty PlusMinusProduct.
    ///
    /// Returns:
    ///     self: The new, empty PlusMinusProduct.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: PlusMinusProduct::new(),
        }
    }

    /// Set a new entry for SinglePlusMinusOperator X in the internal dictionary.
    ///
    /// Args:
    ///     index (int): Index of set object.
    ///
    /// Returns:
    ///     PlusMinusProduct: The PlusMinusProduct with the new entry.
    pub fn plus(&self, index: usize) -> PlusMinusProductWrapper {
        Self {
            internal: self.clone().internal.plus(index),
        }
    }

    /// Set a new entry for SinglePlusMinusOperator Y in the internal dictionary.
    ///
    /// Args:
    ///     index (int): Index of set object.
    ///
    /// Returns:
    ///     PlusMinusProduct: The PlusMinusProduct with the new entry.
    pub fn minus(&self, index: usize) -> PlusMinusProductWrapper {
        Self {
            internal: self.clone().internal.minus(index),
        }
    }

    /// Set a new entry for SinglePlusMinusOperator Z in the internal dictionary.
    ///
    /// Args:
    ///     index (int): Index of set object.
    ///
    /// Returns:
    ///     PlusMinusProduct: The PlusMinusProduct with the new entry.
    pub fn z(&self, index: usize) -> PlusMinusProductWrapper {
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
    ///     self: The entry was correctly set and the PlusMinusProduct is returned.
    pub fn set_pauli(&self, index: usize, pauli: String) -> PyResult<Self> {
        let converted_pauli = SinglePlusMinusOperator::from_str(pauli.as_str()).map_err(|err| {
            PyValueError::new_err(format!(
                "pauli could not be converted to X, Y, Z: {:?}",
                err
            ))
        })?;
        Ok(Self {
            internal: self.internal.clone().set_pauli(index, converted_pauli),
        })
    }

    /// Get the pauli matrix corresponding to the index.
    ///
    /// Args:
    ///     index (int): Index of get object.
    ///
    /// Returns:
    ///     Optional[str]: The key's corresponding value (if it exists).
    pub fn get(&self, index: usize) -> Option<String> {
        self.internal.get(&index).map(|x| format!("{}", x))
    }

    /// Return a list of the unsorted keys in self.
    ///
    /// Returns:
    ///     list[int]: The sequence of qubit index keys of self.
    pub fn keys(&self) -> Vec<usize> {
        let keys: Vec<usize> = self.internal.iter().map(|(k, _)| k).copied().collect();
        keys
    }

    /// Return maximum index in self.
    ///
    /// Returns:
    ///     int: Maximum index.
    pub fn current_number_spins(&self) -> usize {
        self.internal.current_number_spins()
    }

    /// Return number of entries in object.
    ///
    /// Returns:
    ///     int: The length of the content of the object.
    pub fn __len__(&self) -> usize {
        self.internal.iter().len()
    }

    /// Remap the qubits in a new instance of self (returned).
    ///
    /// Args:
    ///     mapping (dict[int, int]): The map containing the {qubit: qubit} mapping to use.
    ///
    /// Returns:
    ///     self: The new instance of self with the qubits remapped.
    pub fn remap_qubits(&self, mapping: HashMap<usize, usize>) -> PlusMinusProductWrapper {
        PlusMinusProductWrapper {
            internal: self.internal.remap_qubits(&mapping),
        }
    }

    /// Return the concatenation of two objects of type `self` with no overlapping qubits.
    ///
    /// Args:
    ///     other (self): The object to concatenate self with.
    ///
    /// Returns:
    ///     list[int]: A list of the corresponding creator indices.
    ///
    /// Raises:
    ///     ValueError: The two objects could not be concatenated.
    pub fn concatenate(&self, other: PlusMinusProductWrapper) -> PyResult<PlusMinusProductWrapper> {
        let concatenated = self.internal.concatenate(other.internal).map_err(|err| {
            PyValueError::new_err(format!(
                "The two objects could not be concatenated: {:?}",
                err
            ))
        })?;
        Ok(PlusMinusProductWrapper {
            internal: concatenated,
        })
    }

    /// Creates a list of corresponding (PlusMinusProduct, CalculatorComplex) tuples from the input PauliProduct or DecoherenceProduct.
    ///
    /// Args:
    ///     value (PauliProduct or DecoherenceProduct): The input object to convert.
    ///
    /// Returns:
    ///     list[tuple[(PlusMinusProduct, CalculatorComplex)]]: The converted input.
    ///
    /// Raises:
    ///     ValueError: Input is neither a PauliProduct nor a DecoherenceProduct.
    #[staticmethod]
    pub fn from_product(
        value: Py<PyAny>,
    ) -> PyResult<Vec<(PlusMinusProductWrapper, CalculatorComplexWrapper)>> {
        match PauliProductWrapper::from_pyany(value.clone()) {
            Ok(x) => {
                let result: Vec<(PlusMinusProduct, Complex64)> =
                    Vec::<(PlusMinusProduct, Complex64)>::from(x);
                let result_pyo3: Vec<(PlusMinusProductWrapper, CalculatorComplexWrapper)> = result
                    .iter()
                    .map(|(key, val)| {
                        (
                            PlusMinusProductWrapper {
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
            Err(_) => match DecoherenceProductWrapper::from_pyany(value) {
                Ok(x) => {
                    let result: Vec<(PlusMinusProduct, Complex64)> =
                        Vec::<(PlusMinusProduct, Complex64)>::from(x);
                    let result_pyo3: Vec<(PlusMinusProductWrapper, CalculatorComplexWrapper)> =
                        result
                            .iter()
                            .map(|(key, val)| {
                                (
                                    PlusMinusProductWrapper {
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
                Err(_) => Err(PyValueError::new_err(
                    "Input is neither PauliProduct nor DecoherenceProduct",
                )),
            },
        }
    }

    /// DEPRECATED: Convert `self` into a list of (PauliProduct, CalculatorComplex) tuples.
    ///
    /// This function is deprecated, please use `to_pauli_product_list`
    ///
    /// Returns:
    ///     list[tuple[(PauliProduct, CalculatorComplex)]]: A list of the terms `self` corresponds to.
    pub fn to_pauli_product(&self) -> Vec<(PauliProductWrapper, CalculatorComplexWrapper)> {
        self.to_pauli_product_list()
    }

    /// Convert `self` into a list of (PauliProduct, CalculatorComplex) tuples.
    ///
    /// Returns:
    ///     list[tuple[(PauliProduct, CalculatorComplex)]]: A list of the terms `self` corresponds to.
    pub fn to_pauli_product_list(&self) -> Vec<(PauliProductWrapper, CalculatorComplexWrapper)> {
        let result: Vec<(PauliProduct, Complex64)> =
            Vec::<(PauliProduct, Complex64)>::from(self.internal.clone());
        let result_pyo3: Vec<(PauliProductWrapper, CalculatorComplexWrapper)> = result
            .iter()
            .map(|(key, val)| {
                (
                    PauliProductWrapper {
                        internal: key.clone(),
                    },
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::new(val.re, val.im),
                    },
                )
            })
            .collect();
        result_pyo3
    }

    /// DEPRECATED: Convert `self` into a list of (DecoherenceProduct, CalculatorComplex) tuples.
    ///
    /// This function is deprecated, please use `to_decoherence_product_list`
    ///
    /// Returns:
    ///     list[tuple[(DecoherenceProduct, CalculatorComplex)]]: A list of the terms `self` corresponds to.
    pub fn to_decoherence_product(
        &self,
    ) -> Vec<(DecoherenceProductWrapper, CalculatorComplexWrapper)> {
        self.to_decoherence_product_list()
    }

    /// Convert `self` into a list of (DecoherenceProduct, CalculatorComplex) tuples.
    ///
    /// Returns:
    ///     list[tuple[(DecoherenceProduct, CalculatorComplex)]]: A list of the terms `self` corresponds to.
    pub fn to_decoherence_product_list(
        &self,
    ) -> Vec<(DecoherenceProductWrapper, CalculatorComplexWrapper)> {
        let result: Vec<(DecoherenceProduct, Complex64)> =
            Vec::<(DecoherenceProduct, Complex64)>::from(self.internal.clone());
        let result_pyo3: Vec<(DecoherenceProductWrapper, CalculatorComplexWrapper)> = result
            .iter()
            .map(|(key, val)| {
                (
                    DecoherenceProductWrapper {
                        internal: key.clone(),
                    },
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::new(val.re, val.im),
                    },
                )
            })
            .collect();
        result_pyo3
    }
}
