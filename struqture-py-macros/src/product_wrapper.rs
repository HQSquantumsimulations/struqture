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

//! struqture-py-macros
//!
//! Attribute proc-macros for the traits of struqture-py [struqture-py].

use crate::{strip_python_wrapper_name, AttributeMacroArguments};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemImpl};

pub fn productwrapper(
    metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as ItemImpl);
    let ident = parsed_input.self_ty;
    let items = parsed_input.items;
    let attribute_arguments = parse_macro_input!(metadata as AttributeMacroArguments);
    let (struct_name, struct_ident) = strip_python_wrapper_name(&ident);
    let (struqture_1_module, struqture_1_ident) = if struct_name.contains("PauliProduct") {
        (
            quote::format_ident!("spins"),
            quote::format_ident!("PauliProduct"),
        )
    } else if struct_name.contains("MixedDecoherenceProduct") {
        (
            quote::format_ident!("mixed_systems"),
            quote::format_ident!("MixedDecoherenceProduct"),
        )
    } else if struct_name.contains("HermitianMixedProduct") {
        (
            quote::format_ident!("mixed_systems"),
            quote::format_ident!("HermitianMixedProduct"),
        )
    } else if struct_name.contains("MixedPlusMinusProduct") {
        (
            quote::format_ident!("mixed_systems"),
            quote::format_ident!("MixedPlusMinusProduct"),
        )
    } else if struct_name.contains("DecoherenceProduct") {
        (
            quote::format_ident!("spins"),
            quote::format_ident!("DecoherenceProduct"),
        )
    } else if struct_name.contains("HermitianBosonProduct") {
        (
            quote::format_ident!("bosons"),
            quote::format_ident!("HermitianBosonProduct"),
        )
    } else if struct_name.contains("BosonProduct") {
        (
            quote::format_ident!("bosons"),
            quote::format_ident!("BosonProduct"),
        )
    } else if struct_name.contains("HermitianFermionProduct") {
        (
            quote::format_ident!("fermions"),
            quote::format_ident!("HermitianFermionProduct"),
        )
    } else if struct_name.contains("FermionProduct") {
        (
            quote::format_ident!("fermions"),
            quote::format_ident!("FermionProduct"),
        )
    } else if struct_name.contains("PlusMinusProduct") {
        (
            quote::format_ident!("spins"),
            quote::format_ident!("PlusMinusProduct"),
        )
    } else {
        (
            quote::format_ident!("mixed_systems"),
            quote::format_ident!("MixedProduct"),
        )
    };
    // ------------
    // Start the generating part of the macro
    let symmetric_index_quote = if attribute_arguments.contains("SymmetricIndex") {
        quote! {
                /// Return the hermitian conjugate of self and its prefactor.
                ///
                /// Returns:
                ///     (self, float): The hermitian conjugate of self and the potential sign it has picked up.
                pub fn hermitian_conjugate(&self) -> (#ident, f64) {
                    (#ident {
                        internal: self.internal.hermitian_conjugate().0
                    },
                    self.internal.hermitian_conjugate().1
                )
                }

                /// Return whether self is naturally hermitian.
                ///
                /// For spin objects, this is true when applying the hermitian conjugation does not change the sign.
                /// For bosonic and fermionic objects, this is true when creators == annihilators.
                /// For mixed objects, this is true when all of the spin, bosonic and fermionic parts' `is_naturally_hermitian` functions evaluate to true.
                ///
                /// Returns:
                ///     bool: Whether self is naturally hermitian or not.
                pub fn is_natural_hermitian(&self) -> bool {
                    self.internal.is_natural_hermitian()
                }
        }
    } else {
        TokenStream::new()
    };
    let mode_index_quote = if attribute_arguments.contains("ModeIndex") {
        quote! {
                /// Get the number of creator indices of self.
                ///
                /// Returns:
                ///     int: The number of creator indices in self.
                pub fn number_creators(&self) -> usize {
                    self.internal.number_creators()
                }

                /// Get the number of annihilator indices of self.
                ///
                /// Returns:
                ///     int: The number of annihilator indices in self.
                pub fn number_annihilators(&self) -> usize {
                    self.internal.number_annihilators()
                }

                /// Returns the maximal number of modes self acts on.
                ///
                /// Self acts on a state space of unknown dimension.
                /// There is only a lower bound of the dimension or number of modes based on the
                /// maximal mode the product of operators in the index acts on.
                /// For example an index consisting of one creator acting on mode 0 would have
                /// a current_number_modes of one. An index consisting of one annhihilator acting on 3
                /// would have current_number_modes of four.
                ///
                /// Returns:
                ///     int: The maximal number of modes self acts on.
                pub fn current_number_modes(&self) -> usize {
                    self.internal.current_number_modes()
                }

                /// Return list of creator indices.
                ///
                /// Returns:
                ///     List[int]: A list of the corresponding creator indices.
                pub fn creators(&self) -> Vec<usize> {
                    self.internal.creators().cloned().collect()
                }

                /// Return list of annihilator indices.
                ///
                /// Returns:
                ///     List[int]: A list of the corresponding annihilator indices.
                pub fn annihilators(&self) -> Vec<usize> {
                    self.internal.annihilators().cloned().collect()
                }

                /// Remap modes according to an input dictionary.
                ///
                /// Args:
                ///    reordering_dictionary (dict) - The dictionary specifying the remapping. It must represent a permutation.
                ///
                /// Returns:
                ///   (Self, CalculatorComplex) - The instance of Self with modes remapped, and the sign resulting from symmetry/antisymmetry.
                ///
                /// Raises:
                ///    ValueError: Input reordering dictionary is not a permutation of the indices.
                pub fn remap_modes(&self, reordering_dictionary: &Bound<PyAny>) -> PyResult<(#ident, qoqo_calculator_pyo3::CalculatorComplexWrapper)> {
                    let remap_dict = reordering_dictionary.as_gil_ref().extract::<HashMap<usize, usize>>()?;
                    let (index, value) = self.internal.remap_modes(&remap_dict).map_err(|err| PyValueError::new_err(format!("{:?}", err)))?;
                    Ok((#ident{internal: index}, qoqo_calculator_pyo3::CalculatorComplexWrapper{internal: value}))
                }


                /// Create valid pair of index and value to be set in an operator.
                ///
                /// The first item is the valid instance of self created from the input creators and annihilators.
                /// The second term is the input CalculatorComplex transformed according to the valid order of creators and annihilators.
                ///
                /// Args:
                ///    creators (List[int]): The creator indices to have in the instance of self.
                ///    annihilators (List[int]): The annihilators indices to have in the instance of self.
                ///    value (CalculatorComplex): The CalculatorComplex to transform.
                ///
                /// Returns:
                ///    (self, CalculatorComplex): The valid instance of self and the corresponding transformed CalculatorComplex.
                ///
                /// Raises:
                ///     TypeError: Value is not CalculatorComplex.
                ///     ValueError: Indices given in either creators or annihilators contain a double index specification (only applicable to fermionic objects).
                #[classmethod]
                pub fn create_valid_pair(_cls: Bound<PyType>, creators: Vec<usize>, annihilators: Vec<usize>, value: &Bound<PyAny>) -> PyResult<(#ident, qoqo_calculator_pyo3::CalculatorComplexWrapper)> {
                    let value = qoqo_calculator_pyo3::convert_into_calculator_complex(value).map_err(|_| PyTypeError::new_err("Value is not CalculatorComplex"))?;
                    let (index, value) = #struct_ident::create_valid_pair(creators, annihilators, value).map_err(|err| PyValueError::new_err(format!("Valid pair could not be constructed: {:?}", err)))?;
                    Ok((#ident{internal: index}, qoqo_calculator_pyo3::CalculatorComplexWrapper{internal: value}))
                }
        }
    } else {
        TokenStream::new()
    };
    let spin_index_quote = if attribute_arguments.contains("SpinIndex") {
        quote! {
                /// Get the pauli matrix corresponding to the index.
                ///
                /// Args:
                ///     index (int): Index of get object.
                ///
                /// Returns:
                ///     Optional[str]: The key's corresponding value (if it exists).
                pub fn get(&self, index: usize) -> Option<String> {
                    match self.internal.get(&index) {
                        Some(x) => Some(format!("{}", x)),
                        None => None
                    }
                }

                /// Return a list of the unsorted keys in self.
                ///
                /// Returns:
                ///     List[int]: The sequence of qubit index keys of self.
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
                    self.internal.len()
                }

                /// Return whether self is empty or not.
                ///
                /// Returns:
                ///     bool: Whether self is empty or not.
                pub fn is_empty(&self) -> bool {
                    self.internal.is_empty()
                }

                /// Remap the qubits in a new instance of self (returned).
                ///
                /// Args:
                ///     mapping (Dict[int, int]): The map containing the {qubit: qubit} mapping to use.
                ///
                /// Returns:
                ///     self: The new instance of self with the qubits remapped.
                pub fn remap_qubits(&self, mapping: HashMap<usize, usize>) -> #ident {
                    #ident {
                        internal: self.internal.remap_qubits(&mapping)
                    }
                }

                /// Return the concatenation of two objects of type `self` with no overlapping qubits.
                ///
                /// Args:
                ///     other (self): The object to concatenate self with.
                ///
                /// Returns:
                ///     List[int]: A list of the corresponding creator indices.
                ///
                /// Raises:
                ///     ValueError: The two objects could not be concatenated.
                pub fn concatenate(&self, other: #ident) -> PyResult<#ident> {
                    let concatenated = self.internal.concatenate(other.internal).map_err(|err| PyValueError::new_err(format!("The two objects could not be concatenated: {:?}", err)))?;
                    Ok(#ident {
                        internal: concatenated
                    })
                }
        }
    } else {
        TokenStream::new()
    };
    let calculus_quote = if attribute_arguments.contains("Calculus") {
        quote! {
                /// Multiplication function for a self-typed object by a self-typed object.
                ///
                /// Args:
                ///     left (self): Left-hand self typed object to be multiplied.
                ///     right (self): Right-hand self typed object to be multiplied.
                ///
                /// Returns:
                ///     (self, complex):  The multiplied objects and the resulting prefactor.
                #[staticmethod]
                pub fn multiply(left: #ident, right: #ident) -> (#ident, Complex64) {
                    let (index, value) = #struct_ident::multiply(left.internal, right.internal);
                    (#ident{internal: index}, value)
                }
        }
    } else {
        TokenStream::new()
    };
    let mixed_index_quote = if struct_name.contains("Mixed") {
        let spin_type = if struct_name.contains("Decoherence") {
            quote::format_ident!("DecoherenceProductWrapper")
        } else if struct_name.contains("PlusMinus") {
            quote::format_ident!("PlusMinusProductWrapper")
        } else {
            quote::format_ident!("PauliProductWrapper")
        };
        quote! {
                /// Get the spin products of self.
                ///
                /// Returns:
                ///     List[str]: The spin products of self.
                pub fn spins(&self) -> Vec<#spin_type> {
                    let spins: Vec<#spin_type> = self
                        .internal
                        .spins()
                        .cloned()
                        .map(|x| #spin_type { internal: x })
                        .collect();
                    spins
                }

                /// Get the boson products of self.
                ///
                /// Returns:
                ///     List[str]: The boson products of self.
                pub fn bosons(&self) -> Vec<BosonProductWrapper> {
                    let bosons: Vec<BosonProductWrapper> = self
                        .internal
                        .bosons()
                        .cloned()
                        .map(|x| BosonProductWrapper { internal: x })
                        .collect();
                    bosons
                }

                /// Get the fermion products of self.
                ///
                /// Returns:
                ///     List[str]: The fermion products of self.
                pub fn fermions(&self) -> Vec<FermionProductWrapper> {
                    let fermions: Vec<FermionProductWrapper> = self
                        .internal
                        .fermions()
                        .cloned()
                        .map(|x| FermionProductWrapper { internal: x })
                        .collect();
                    fermions
                }

                /// Return the current number of spins each subsystem acts upon.
                ///
                /// Returns:
                ///     List[int]: Number of spins in each spin sub-system.
                pub fn current_number_spins(&self) -> Vec<usize> {
                    self.internal.current_number_spins()
                }

                /// Return the current number of bosonic modes each subsystem acts upon.
                ///
                /// Returns:
                ///     List[int]: Number of bosonic modes in each spin sub-system.
                pub fn current_number_bosonic_modes(&self) -> Vec<usize> {
                    self.internal.current_number_bosonic_modes()
                }

                /// Return the current number of fermionic modes each subsystem acts upon.
                ///
                /// Returns:
                ///     List[int]: Number of fermionic modes in each spin sub-system.
                pub fn current_number_fermionic_modes(&self) -> Vec<usize> {
                    self.internal.current_number_fermionic_modes()
                }
        }
    } else {
        TokenStream::new()
    };
    let q = quote! {

        impl #ident {
            /// Fallible conversion of generic python object..
            pub fn from_pyany(input: &Bound<PyAny>) -> PyResult<#struct_ident> {
                Python::with_gil(|py| -> PyResult<#struct_ident> {
                    if let Ok(try_downcast) = input.extract::<#ident>() {
                        return Ok(try_downcast.internal);
                    } else {
                        let input = input.as_ref();
                        let get_str = input.call_method0("__str__").map_err(|_| {
                            PyTypeError::new_err("Type conversion failed".to_string())
                        })?;
                        let string = get_str.extract::<String>().map_err(|_| {
                            PyTypeError::new_err("Type conversion failed".to_string())
                        })?;
                        let res = #struct_ident::from_str(string.as_str()).map_err(|err|
                            PyTypeError::new_err(format!(
                                "Type conversion failed: {}",
                                err
                            )))?;
                        let source_serialisation_meta = res.struqture_serialisation_meta();

                        let target_serialisation_meta = <#struct_ident as struqture::SerializationSupport>::target_serialisation_meta();

                        struqture::check_can_be_deserialised(&target_serialisation_meta, &source_serialisation_meta).map_err(|err| {
                            PyTypeError::new_err(err.to_string())
                        })?;

                        Ok(res)

                    }
                })
            }

            /// Fallible conversion of generic python object that is implemented in struqture 1.x.
            #[cfg(feature = "struqture_1_import")]
            pub fn from_pyany_struqture_1(input: &Bound<PyAny>) -> PyResult<#struct_ident> {
                Python::with_gil(|py| -> PyResult<#struct_ident> {
                    let input = input.as_ref();
                    let get_str = input.call_method0("__str__").map_err(|_| {
                        PyTypeError::new_err("Type conversion failed".to_string())
                    })?;
                    let string = get_str.extract::<String>().map_err(|_| {
                        PyTypeError::new_err("Type conversion failed".to_string())
                    })?;
                    let one_import = struqture_1::#struqture_1_module::#struqture_1_ident::from_str(string.as_str()).map_err(|err|
                        PyTypeError::new_err(format!(
                            "Type conversion failed: {}",
                            err
                        )))?;

                    let qubit_operator: #struct_ident = #struct_ident::from_struqture_1(&one_import).map_err(
                        |err| PyValueError::new_err(format!("Trying to obtain struqture 2.x object from struqture 1.x object. Conversion failed. Was the right type passed to all functions? {:?}", err)
                    ))?;
                    Ok(qubit_operator)
                })
            }

            /// Fallible conversion of generic python object that is implemented in struqture 1.x.
            #[cfg(feature = "struqture_1_export")]
            pub fn from_pyany_to_struqture_1(input: &Bound<PyAny>) -> PyResult<struqture_1::#struqture_1_module::#struqture_1_ident> {
                let res = <#ident>::from_pyany(input)?;
                <#struct_ident>::to_struqture_1(&res).map_err(
                    |err| PyValueError::new_err(format!("Trying to obtain struqture 2.x object from struqture 1.x object. Conversion failed. Was the right type passed to all functions? Error message: {:?}", err)
                ))
            }
        }

        #[pymethods]
        impl #ident {

            #(#items)*

            #symmetric_index_quote
            #mode_index_quote
            #spin_index_quote
            #mixed_index_quote
            #calculus_quote

            // ----------------------------------
            // Default pyo3 implementations

            // add in a function converting struqture_1 (not py) to struqture 2
            // take a pyany, implement from_pyany by hand (or use from_pyany_struqture_1 internally) and wrap the result in a struqture 2 spin operator wrapper
            #[cfg(feature = "struqture_1_import")]
            #[staticmethod]
            pub fn from_struqture_1(input: &Bound<PyAny>) -> PyResult<#ident> {
                let qubit_operator: #struct_ident = #ident::from_pyany_struqture_1(input)?;
                Ok(#ident {
                    internal: qubit_operator,
                })
            }

            // add in a function converting struqture_1 (not py) to struqture 2
            // take a pyany, implement from_pyany by hand (or use from_pyany_struqture_1 internally) and wrap the result in a struqture 2 spin operator wrapper
            #[cfg(feature = "struqture_1_import")]
            #[pyo3(text_signature = "(input)")]
            #[staticmethod]
            pub fn from_json_struqture_1(input: String) -> PyResult<#ident> {
                let qubit_operator: struqture_1::#struqture_1_module::#struqture_1_ident =
                    serde_json::from_str(&input).map_err(|err| {
                        PyValueError::new_err(format!(
                            "Input cannot be deserialized from json to struqture 1.x: {}",
                            err
                        ))
                    })?;
                Ok(#ident {
                    internal: #struct_ident::from_struqture_1(&qubit_operator).map_err(|err| {
                        PyValueError::new_err(format!(
                            "Trying to obtain struqture 2.x object from struqture 1.x object. Conversion failed. Was the right type passed to all functions? {:?}", err
                        ))
                    })?,
                })
            }

            /// Return a copy of self (copy here produces a deepcopy).
            ///
            /// Returns:
            ///     self: A deep copy of Self.
            pub fn __copy__(&self) -> #ident {
                self.clone()
            }

            /// Return a deep copy of self.
            ///
            /// Returns:
            ///     self: A deep copy of Self.
            pub fn __deepcopy__(&self, _memodict: &Bound<PyAny>) -> #ident {
                self.clone()
            }

            /// Convert the bincode representation of the object to an instance using the [bincode] crate.
            ///
            /// Args:
            ///     input (bytearray): The serialized object (in [bincode] form).
            ///
            /// Returns:
            ///    The deserialized Spin System.
            ///
            /// Raises:
            ///     TypeError: Input cannot be converted to byte array.
            ///     ValueError: Input cannot be deserialized.
            #[staticmethod]
            pub fn from_bincode(input: &Bound<PyAny>) -> PyResult<#ident> {
                let bytes = input
                    .as_gil_ref()
                    .extract::<Vec<u8>>()
                    .map_err(|_| PyTypeError::new_err("Input cannot be converted to byte array"))?;

                Ok(#ident {
                    internal: bincode::deserialize(&bytes[..]).map_err(|err| {
                        PyValueError::new_err(format!(
                            "Input cannot be deserialized from bytes. {}",
                            err
                        ))
                    })?,
                })
            }

            /// Return the bincode representation of the object using the [bincode] crate.
            ///
            /// Returns:
            ///     bytearray: The serialized object (in [bincode] form).
            ///
            /// Raises:
            ///     ValueError: Cannot serialize object to bytes.
            pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
                let serialized = bincode::serialize(&self.internal).map_err(|_| {
                    PyValueError::new_err("Cannot serialize object to bytes")
                })?;
                let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
                    PyByteArray::new_bound(py, &serialized[..]).into()
                });
                Ok(b)
            }

            /// Return the json representation of the object.
            ///
            /// Returns:
            ///     str: The serialized form of the object.
            ///
            /// Raises:
            ///     ValueError: Cannot serialize object to json.
            pub fn to_json(&self) -> PyResult<String> {
                let serialized = serde_json::to_string(&self.internal)
                    .map_err(|_| PyValueError::new_err("Cannot serialize object to json".to_string()))?;
                Ok(serialized)
            }

            /// Convert the json representation of the object to an instance.
            ///
            /// Args:
            ///     input (str): The serialized object in json form.
            ///
            /// Returns:
            ///     The deserialized object.
            ///
            /// Raises:
            ///     ValueError: Input cannot be deserialized.
            #[staticmethod]
            #[pyo3(text_signature = "(input)")]
            pub fn from_json(input: String) -> PyResult<#ident> {
                Ok(#ident {
                    internal: serde_json::from_str(&input).map_err(|err| {
                        PyValueError::new_err(format!(
                            "Input cannot be deserialized {}",
                            err
                        ))
                    })?,
                })
            }

            /// Convert a string representation of the object to an instance.
            ///
            /// Args:
            ///     input (str): The serialized index in str representation.
            ///
            /// Returns:
            ///     self: The converted object.
            ///
            /// Raises:
            ///     ValueError: Input cannot be converted from str.
            #[staticmethod]
            #[pyo3(text_signature = "(input)")]
            pub fn from_string(input: String) -> PyResult<#ident> {
                Ok(#ident {
                    internal: #struct_ident::from_str(&input).map_err(|err| {
                        PyValueError::new_err(format!(
                            "Input cannot be deserialized: {}",
                            err
                        ))
                    })?,
                })
            }

            /// Return a string containing a printable representation of the index.
            ///
            /// Returns:
            ///     str: The printable string representation of the index.
            pub fn __str__(&self) -> String {
                format!("{}", self.internal)
            }

            /// Return a string containing a printable representation of the index.
            ///
            /// Returns:
            ///     str: The printable string representation of the index.
            pub fn __repr__(&self) -> String {
                format!("{}", self.internal)
            }

            /// Return the __richcmp__ magic method to perform rich comparison operations on mixed index.
            ///
            /// Args:
            ///     other: The object to compare self to.
            ///     op: Whether they should be equal or not.
            ///
            /// Returns:
            ///     Whether the two operations compared evaluated to True or False
            ///
            /// Raises:
            ///     NotImplementedError: Other comparison not implemented.
            pub fn __richcmp__(&self, other: &Bound<PyAny>, op: pyo3::class::basic::CompareOp) -> PyResult<bool> {
                let other = Self::from_pyany(other);
                    match op {
                        pyo3::class::basic::CompareOp::Eq => match other {
                            Ok(pauli) => Ok(self.internal == pauli),
                            _ => Ok(false),
                        },
                        pyo3::class::basic::CompareOp::Ne => match other {
                            Ok(pauli) => Ok(self.internal != pauli),
                            _ => Ok(true),
                        },
                        _ => Err(pyo3::exceptions::PyNotImplementedError::new_err(
                            "Other comparison not implemented",
                        )),
                    }
            }

            /// Return the __hash__ magic method.
            ///
            /// Returns:
            ///     integer: Hash
            pub fn __hash__(&self) -> PyResult<isize> {
                let mut hasher = DefaultHasher::new();
                self.internal.hash(&mut hasher);
                Ok(hasher.finish() as isize)
            }

            #[cfg(feature = "json_schema")]
            /// Returns the current version of the struqture library .
            ///
            /// Returns:
            ///     str: The current version of the library.
            #[staticmethod]
            pub fn current_version() -> String {
                return STRUQTURE_VERSION.to_string();
            }

            #[cfg(feature = "json_schema")]
            /// Return the minimum version of struqture that supports this object.
            ///
            /// Returns:
            ///     str: The minimum version of the struqture library to deserialize this object.
            pub fn min_supported_version(&self) -> String {
                let min_version: (usize, usize, usize) = struqture::SerializationSupport::min_supported_version(&self.internal);
                return format!("{}.{}.{}", min_version.0, min_version.1, min_version.2);
            }

            /// Returns the StruqtureSerialisationMeta of the object.
            fn _get_serialisation_meta(&self) -> PyResult<String>{
                let meta = struqture::SerializationSupport::struqture_serialisation_meta(&self.internal);
                let string = serde_json::to_string(&meta).map_err(|err| PyValueError::new_err(err.to_string()))?;
                Ok(string)
            }

            #[cfg(feature = "json_schema")]
            /// Return the JsonSchema for the json serialisation of the class.
            ///
            /// Returns:
            ///     str: The json schema serialized to json
            #[staticmethod]
            pub fn json_schema() -> String {
                let schema = schemars::schema_for!(#struct_ident);
                serde_json::to_string_pretty(&schema).expect("Unexpected failure to serialize schema")
            }
        }

    };
    q.into()
}
