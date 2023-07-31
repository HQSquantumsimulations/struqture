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

pub fn noiselesswrapper(
    metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as ItemImpl);
    let ident = parsed_input.self_ty;
    let items = parsed_input.items;
    let attribute_arguments = parse_macro_input!(metadata as AttributeMacroArguments);
    let (struct_name, struct_ident) = strip_python_wrapper_name(&ident);
    let index_type = if struct_name.contains("Spin") {
        quote::format_ident!("PauliProductWrapper")
    } else if struct_name.contains("MixedPlusMinusOperator") {
        quote::format_ident!("MixedPlusMinusProductWrapper")
    } else if struct_name.contains("PlusMinusOperator") {
        quote::format_ident!("PlusMinusProductWrapper")
    } else if struct_name.contains("BosonHamiltonian") {
        quote::format_ident!("HermitianBosonProductWrapper")
    } else if struct_name.contains("Boson") {
        quote::format_ident!("BosonProductWrapper")
    } else if struct_name.contains("FermionHamiltonian") {
        quote::format_ident!("HermitianFermionProductWrapper")
    } else if struct_name.contains("Fermion") {
        quote::format_ident!("FermionProductWrapper")
    } else if struct_name.contains("MixedHamiltonian") {
        quote::format_ident!("HermitianMixedProductWrapper")
    } else {
        quote::format_ident!("MixedProductWrapper")
    };
    let value_type = if struct_name.contains("SpinHamiltonian") {
        quote::format_ident!("CalculatorFloatWrapper")
    } else {
        quote::format_ident!("CalculatorComplexWrapper")
    };
    // ------------
    // Start the generating part of the macro
    let operate_on_density_matrix_quote = if attribute_arguments.contains("OperateOnDensityMatrix")
    {
        quote! {
                /// Return a list of the unsorted keys in self.
                ///
                /// Returns:
                ///     list[OperatorProduct]: The sequence of keys of the self.
                pub fn keys(&self) -> Vec<#index_type> {
                    let mut system_keys: Vec<#index_type> = Vec::new();
                    for key in self.internal.keys() {
                        system_keys.push(
                            #index_type { internal: key.clone() },
                        );
                    }
                    system_keys
                }

                /// Return number of entries in self.
                ///
                /// Returns:
                ///     int: The length of the content of self.
                pub fn __len__(&self) -> usize {
                    self.internal.len()
                }

                /// Return an instance of self that has no entries but clones all other properties, with the given capacity.
                ///
                /// Args:
                ///     capacity (Optional[int]): The capacity of the new instance to create.
                ///
                /// Returns:
                ///     self: An empty clone with the same properties as self, with the given capacity.
                #[pyo3(signature = (capacity = None))]
                pub fn empty_clone(&self, capacity: Option<usize>) -> #ident {
                    #ident {
                        internal: self.internal.empty_clone(capacity)
                    }
                }

                /// Return true if self contains no values.
                ///
                /// Returns:
                ///     bool: Whether self is empty or not.
                pub fn is_empty(&self) -> bool {
                    self.internal.is_empty()
                }

                /// Truncate self by returning a copy without entries under a threshold.
                ///
                /// Args:
                ///     threshold: The threshold for inclusion.
                ///
                /// Returns:
                ///     self: The truncated version of self.
                pub fn truncate(&self, threshold: f64) -> #ident {
                    #ident { internal: self.internal.truncate(threshold) }
                }

                /// Get the coefficient corresponding to the key.
                ///
                /// Args:
                ///     key: Product to get the value of.
                ///
                /// Returns:
                ///     CalculatorComplex: Value at key (or 0.0).
                ///
                /// Raises:
                ///     ValueError: Product could not be constructed from key.
                pub fn get(&self, key: Py<PyAny>) -> PyResult<#value_type> {
                    let converted_key = #index_type::from_pyany(key).map_err(|err| {
                        PyValueError::new_err(format!(
                            "Product could not be constructed: {:?}",
                            err
                        ))
                    })?;
                    Ok(#value_type {
                        internal: self.clone().internal.get(&converted_key).clone(),
                    })
                }

                /// Remove the value of the input key.
                ///
                /// Returns:
                ///     Optional[Union[CalculatorComplex, CalculatorFloat]]: Key existed if this is not None, and this is the value it had before it was removed.
                ///
                /// Raises:
                ///     ValueError: Product could not be constructed.
                pub fn remove(&mut self, key: Py<PyAny>) -> PyResult<Option<#value_type>> {
                    let converted_key = #index_type::from_pyany(key).map_err(|err| {
                        PyValueError::new_err(format!(
                            "Product could not be constructed: {:?}",
                            err
                        ))
                    })?;
                    match self.internal.remove(&converted_key) {
                        Some(x) => Ok(Some(#value_type { internal: x })),
                        None => Ok(None),
                    }
                }

                /// Overwrite an existing entry or set a new entry in self.
                ///
                /// Returns:
                ///     Optional[Union[CalculatorComplex, CalculatorFloat]]: Key existed if this is not None, and this is the value it had before it was overwritten.
                ///
                /// Raises:
                ///     ValueError: Product could not be constructed.
                pub fn set(
                    &mut self,
                    key: Py<PyAny>,
                    value: Py<PyAny>,
                ) -> PyResult<Option<#value_type>> {
                    let value = #value_type::from_pyany(value)
                        .map_err(|_| PyTypeError::new_err("Value is not CalculatorComplex or CalculatorFloat"))?;
                    let converted_key = #index_type::from_pyany(key).map_err(|err| {
                        PyValueError::new_err(format!(
                            "Product could not be constructed: {:?}",
                            err
                        ))
                    })?;
                    match self.internal.set(converted_key, value).map_err(|err| {
                        PyValueError::new_err(format!(
                            "Error in set function of System: {:?}",
                            err
                        ))
                    })? {
                        Some(x) => Ok(Some(#value_type { internal: x })),
                        None => Ok(None),
                    }
                }

                /// Add a new (key object, value Union[CalculatorComplex, CalculatorFloat]) pair to existing entries.
                ///
                /// Raises:
                ///     TypeError: Value is not CalculatorComplex or CalculatorFloat.
                ///     ValueError: Product could not be constructed.
                ///     ValueError: Error in add_operator_product function of self.
                pub fn add_operator_product(&mut self, key: Py<PyAny>, value: Py<PyAny>) -> PyResult<()> {
                    let value = #value_type::from_pyany(value)
                        .map_err(|_| PyTypeError::new_err("Value is not CalculatorComplex or CalculatorFloat"))?;
                    let converted_key = #index_type::from_pyany(key).map_err(|err| {
                        PyValueError::new_err(format!(
                            "Product could not be constructed: {:?}",
                            err
                        ))
                    })?;
                    self.internal
                        .add_operator_product(converted_key, value)
                        .map_err(|err| {
                            PyValueError::new_err(format!(
                                "Error in add_operator_product function of System: {:?}",
                                err
                            ))
                        })
                }

                /// Return unsorted values in self.
                ///
                /// Returns:
                ///     list[Union[CalculatorComplex, CalculatorFloat]]: The sequence of values of self.
                pub fn values(&self) -> Vec<#value_type> {
                    let mut system_values: Vec<#value_type> = Vec::new();
                    for val in self.internal.values() {
                        system_values.push(
                            #value_type { internal: val.clone() },
                        );
                    }
                    system_values
                }
        }
    } else {
        TokenStream::new()
    };
    let operate_on_state_quote = if attribute_arguments.contains("OperateOnState") {
        quote! {
                /// Return the hermitian conjugate of self.
                ///
                /// Returns:
                ///     self: The hermitian conjugate of self.
                pub fn hermitian_conjugate(&self) -> #ident {
                    #ident {
                        internal: self.internal.hermitian_conjugate()
                    }
                }
        }
    } else {
        TokenStream::new()
    };
    let operate_on_modes_quote = if attribute_arguments.contains("OperateOnModes") {
        quote! {
            /// Return maximum index in self.
            ///
            /// Returns:
            ///     int: Maximum index.
            pub fn current_number_modes(&self) -> usize {
                self.internal.current_number_modes()
            }

            /// Return the number_modes input of self.
            ///
            /// Returns:
            ///     int: The number of modes in self.
            pub fn number_modes(&self) -> usize {
                self.internal.number_modes()
            }

            /// Separate self into an operator with the terms of given number of creation and annihilation operators and an operator with the remaining operations.
            ///
            /// Args:
            ///     number_creators_annihilators (Tuple[int, int]): Number of modes to filter for in the keys.
            ///
            /// Returns:
            ///     Tuple[Self, Self]: Operator with the noise terms where the number of creation and annihilation operators matches the number of spins the operator product acts on and Operator with all other contributions.
            ///
            /// Raises:
            ///     ValueError: Error in adding terms to return values.
            pub fn separate_into_n_terms(&self, number_creators_annihilators: (usize, usize)) -> PyResult<(#ident, #ident)> {
                let (separated, remainder) = self.internal.separate_into_n_terms(number_creators_annihilators).map_err(|err| PyValueError::new_err(format!("{:?}", err)))?;
                Ok((
                    #ident { internal: separated },
                    #ident { internal: remainder }
                ))
            }
        }
    } else {
        TokenStream::new()
    };
    let operate_on_spins_quote = if attribute_arguments.contains("OperateOnSpins") {
        quote! {
                /// Return maximum spin index in self.
                ///
                /// Returns:
                ///     int: Maximum index.
                pub fn current_number_spins(&self) -> usize {
                    self.internal.current_number_spins()
                }

                /// Return the number_spins input of self.
                ///
                /// Returns:
                ///     int: The number of spins in self.
                pub fn number_spins(&self) -> usize {
                    self.internal.number_spins()
                }

                /// Separate self into an operator with the terms of given number of spins and an operator with the remaining operations.
                ///
                /// Args:
                ///     number_spins (int): Number of spins to filter for in the keys.
                ///
                /// Returns:
                ///     Tuple[Self, Self]: Operator with the noise terms where the number of spins matches the number of spins the operator product acts on and Operator with all other contributions.
                ///
                /// Raises:
                ///     ValueError: Error in adding terms to return values.
        pub fn separate_into_n_terms(&self, number_spins: usize) -> PyResult<(#ident, #ident)> {
                    let (separated, remainder) = self.internal.separate_into_n_terms(number_spins).map_err(|err| PyValueError::new_err(format!("{:?}", err)))?;
                    Ok((
                        #ident { internal: separated },
                        #ident { internal: remainder }
                    ))
                }
            }
    } else {
        TokenStream::new()
    };
    let to_sparse_matrix_operator_quote = if attribute_arguments.contains("ToSparseMatrixOperator")
    {
        quote! {
                /// Constructs the sparse matrix representation of self as a scipy COO matrix with a given number of spins.
                ///
                /// Args:
                ///     number_spins: The number of spins in self.
                ///
                /// Returns:
                ///     Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]]: The matrix representation of self.
                ///
                /// Raises:
                ///     ValueError: CalculatorError.
                ///     RuntimeError: Could not convert to complex superoperator matrix.
                #[pyo3(signature = (number_spins = None))]
                pub fn sparse_matrix_coo(&self, number_spins: Option<usize>) -> PyResult<PyCooMatrix> {
                    let coo = self
                        .internal
                        .sparse_matrix_coo(number_spins)
                        .map_err(|err| match err {
                            StruqtureError::CalculatorError(c_err) => {
                                PyValueError::new_err(format!("{}", c_err))
                            }
                            _ => PyRuntimeError::new_err(
                                "Could not convert to complex superoperator matrix".to_string(),
                            ),
                        })?;
                    to_py_coo(coo)
                }
        }
    } else {
        TokenStream::new()
    };
    let to_sparse_matrix_superoperator_quote = if attribute_arguments
        .contains("ToSparseMatrixSuperOperator")
    {
        quote! {
                /// Construct the sparse matrix representation of the superoperator in COO representation.
                ///
                /// The superoperator for the operator O is defined as the Matrix S so that
                /// `flatten(-i [O, p]) = S flatten(p)` wher `[,]` is the commutator, `p` is a matrix
                /// and `flatten` flattens a matrix into a vector in row-major form.
                ///
                /// Args:
                ///     number_spins: The number of spins to construct the matrix for.
                ///
                /// Returns:
                ///     Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]]: The matrix representation of self.
                ///
                /// Raises:
                ///     ValueError: CalculatorError.
                ///     RuntimeError: Could not convert to complex superoperator matrix.
                #[pyo3(signature = (number_spins = None))]
                pub fn sparse_matrix_superoperator_coo(&self, number_spins: Option<usize>) -> PyResult<PyCooMatrix> {
                    let coo = self
                        .internal
                        .sparse_matrix_superoperator_coo(number_spins)
                        .map_err(|err| match err {
                            StruqtureError::CalculatorError(c_err) => {
                                PyValueError::new_err(format!("{}", c_err))
                            }
                            _ => PyRuntimeError::new_err(
                                "Could not convert to complex superoperator matrix".to_string(),
                            ),
                        })?;
                    to_py_coo(coo)
                }

                /// Return the unitary part of the superoperator in the sparse COO format.
                ///
                /// Returns:
                ///     Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]]: The matrix representation of the unitary part of self.
                ///
                /// Raises:
                ///     ValueError: CalculatorError.
                ///     RuntimeError: Could not convert to complex superoperator matrix.
                pub fn unitary_sparse_matrix_coo(&self) -> PyResult<PyCooMatrix> {
                    let coo = self
                        .internal
                        .unitary_sparse_matrix_coo()
                        .map_err(|err| match err {
                            StruqtureError::CalculatorError(c_err) => {
                                PyValueError::new_err(format!("{}", c_err))
                            }
                            _ => PyRuntimeError::new_err(
                                "Could not convert to complex superoperator matrix".to_string(),
                            ),
                        })?;
                    to_py_coo(coo)
                }

                /// Output the Lindblad entries in the form (left, right, rate) where left/right are the left and right lindblad operators, and rate is the lindblad rate respectively.
                ///
                /// Returns:
                ///     list[Tuple[Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]], Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray], complex]]: The matrix representation of the noise part of self.
                ///
                /// Raises:
                ///     ValueError: CalculatorError.
                ///     RuntimeError: Could not convert to complex superoperator matrix.
                pub fn sparse_lindblad_entries(&self) -> PyResult<Vec<(PyCooMatrix, PyCooMatrix, Complex64)>> {
                    let coo = self
                        .internal
                        .sparse_lindblad_entries()
                        .map_err(|err| match err {
                            StruqtureError::CalculatorError(c_err) => {
                                PyValueError::new_err(format!("{}", c_err))
                            }
                            _ => PyRuntimeError::new_err(
                                "Could not convert to complex superoperator matrix".to_string(),
                            ),
                        })?;
                    let mut res_vec: Vec<(PyCooMatrix, PyCooMatrix, Complex64)> = Vec::new();
                    for mat in coo {
                        let left = to_py_coo(mat.0).map_err(|err| match err {
                            _ => PyRuntimeError::new_err(
                                "Could not convert to complex superoperator matrix".to_string(),
                            ),
                        })?;
                        let right = to_py_coo(mat.1).map_err(|err| match err {
                            _ => PyRuntimeError::new_err(
                                "Could not convert to complex superoperator matrix".to_string(),
                            ),
                        })?;
                        res_vec.push((left, right, mat.2));
                    }
                    Ok(res_vec)
                }
        }
    } else {
        TokenStream::new()
    };
    let operate_on_mixedsystems_quote = if attribute_arguments.contains("OperateOnMixedSystems") {
        quote! {
                /// Return the number_spins input of each spin subsystem of self.
                ///
                /// Returns:
                ///     int: The number of spins in each spin subsystem of self.
                pub fn number_spins(&self) -> Vec<usize> {
                    self.internal.number_spins()
                }

                /// Return maximum spin index in each spin subsystem of self.
                ///
                /// Returns:
                ///     int: Maximum index in each spin subsystem of self.
                pub fn current_number_spins(&self) -> Vec<usize> {
                    self.internal.current_number_spins()
                }
                /// Return the number of bosonic modes in each bosonic subsystem of self.
                ///
                /// Returns:
                ///     list[int]: The number of bosonic modes in each bosonic subsystem of self.
                pub fn number_bosonic_modes(&self) -> Vec<usize> {
                    self.internal.number_bosonic_modes()
                }

                /// Return the number of bosonic modes each bosonic subsystem of self acts on.
                ///
                /// Returns:
                ///     list[int]: Maximum bosonic mode index currently used in each bosonic subsystem of self.
                pub fn current_number_bosonic_modes(&self) -> Vec<usize> {
                    self.internal.current_number_bosonic_modes()
                }

                /// Return the number of fermionic modes in each fermionic subsystem of self.
                ///
                /// Returns:
                ///     list[int]: The number of fermionic modes in each fermionic subsystem of self.
                pub fn number_fermionic_modes(&self) -> Vec<usize> {
                    self.internal.number_fermionic_modes()
                }

                /// Return the number of fermionic modes each fermionic subsystem of self acts on.
                ///
                /// Returns:
                ///     list[int]: Maximum fermionic mode index currently used in each fermionic subsystem of self.
                pub fn current_number_fermionic_modes(&self) -> Vec<usize> {
                    self.internal.current_number_fermionic_modes()
                }

                // /// Separate self into an operator with the terms of given number of qubits and an operator with the remaining operations.
                // ///
                // /// Args:
                // ///     number_particles (Tuple[int, int, int]): Number of particles to filter for in the keys.
                // ///
                // /// Returns:
                // ///     int: The number of modes in self.
                // ///
                // /// Raises:
                // ///     ValueError: Operator with the noise terms where number_particles matches the number of spins the operator product acts on and Operator with all other contributions.
                // pub fn separate_into_n_terms(&self, number_particles: (usize, usize, usize)) -> PyResult<(#ident, #ident)> {
                //     let (separated, remainder) = self.internal.separate_into_n_terms(number_particles).map_err(|err| PyValueError::new_err(format!("{:?}", err)))?;
                //     Ok((
                //         #ident { internal: separated },
                //         #ident { internal: remainder }
                //     ))
                // }
        }
    } else {
        TokenStream::new()
    };
    let calculus_quote = if attribute_arguments.contains("Calculus") {
        quote! {
            /// Implement `-1` for self.
            ///
            /// Returns:
            ///     self: The object * -1.
            pub fn __neg__(&self) -> #ident {
                #ident {
                    internal: -self.clone().internal
                }
            }

            /// Implement `+` for self with self-type.
            ///
            /// Args:
            ///     other (self): value by which to add to self.
            ///
            /// Returns:
            ///     self: The two objects added.
            ///
            /// Raises:
            ///     ValueError: Objects could not be added.
            pub fn __add__(&self, other: #ident) -> PyResult<#ident> {
                let new_self = (self.clone().internal + other.internal).map_err(|err| PyValueError::new_err(format!("Objects could not be added: {:?}", err)))?;
                Ok(#ident {
                    internal: new_self
                })
            }

            /// Implement `-` for self with self-type.
            ///
            /// Args:
            ///     other (self): value by which to subtract from self.
            ///
            /// Returns:
            ///     self: The two objects subtracted.
            ///
            /// Raises:
            ///     ValueError: Objects could not be subtracted.
            pub fn __sub__(&self, other: #ident) -> PyResult<#ident> {
                let new_self = (self.clone().internal - other.internal).map_err(|err| PyValueError::new_err(format!("Objects could not be subtracted: {:?}", err)))?;
                Ok(#ident {
                    internal: new_self
                })
            }
        }
    } else {
        TokenStream::new()
    };
    let q = quote! {

        impl #ident {
            /// Fallible conversion of generic python object.
            pub fn from_pyany(input: Py<PyAny>
            ) -> PyResult<#struct_ident> {
                Python::with_gil(|py| -> PyResult<#struct_ident> {
                    let input = input.as_ref(py);
                    if let Ok(try_downcast) = input.extract::<#ident>() {
                        return Ok(try_downcast.internal);
                    } else {
                    let get_bytes = input.call_method0("to_bincode").map_err(|_| {
                        PyTypeError::new_err("Serialisation failed".to_string())
                    })?;
                    let bytes = get_bytes.extract::<Vec<u8>>().map_err(|_| {
                        PyTypeError::new_err("Deserialisation failed".to_string())
                    })?;
                    deserialize(&bytes[..]).map_err(|err| {
                        PyTypeError::new_err(format!(
                            "Type conversion failed: {}",
                            err
                        ))}
                    )

                    }
                }

                )
        }
    }
        #[pymethods]
        impl #ident {

            #(#items)*

            #operate_on_density_matrix_quote
            #operate_on_state_quote
            #operate_on_modes_quote
            #operate_on_spins_quote
            #to_sparse_matrix_operator_quote
            #to_sparse_matrix_superoperator_quote
            #operate_on_mixedsystems_quote
            #calculus_quote

            // ----------------------------------
            // Default pyo3 implementations

            /// Return a copy of self (copy here produces a deepcopy).
            ///
            /// Returns:
            ///     self: A deep copy of self.
            pub fn __copy__(&self) -> #ident {
                self.clone()
            }

            /// Return a deep copy of self.
            ///
            /// Returns:
            ///     self: A deep copy of self.
            pub fn __deepcopy__(&self, _memodict: Py<PyAny>) -> #ident {
                self.clone()
            }

            /// Convert the bincode representation of self to an instance using the [bincode] crate.
            ///
            /// Args:
            ///     input (ByteArray): The serialized object (in [bincode] form).
            ///
            /// Returns:
            ///    The deserialized object.
            ///
            /// Raises:
            ///     TypeError: Input cannot be converted to byte array.
            ///     ValueError: Input cannot be deserialized.
            #[staticmethod]
            pub fn from_bincode(input: &PyAny) -> PyResult<#ident> {
                let bytes = input
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

            /// Return the bincode representation of self using the [bincode] crate.
            ///
            /// Returns:
            ///     ByteArray: The serialized object (in [bincode] form).
            ///
            /// Raises:
            ///     ValueError: Cannot serialize object to bytes.
            pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
                let serialized = bincode::serialize(&self.internal).map_err(|_| {
                    PyValueError::new_err("Cannot serialize object to bytes")
                })?;
                let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
                    PyByteArray::new(py, &serialized[..]).into()
                });
                Ok(b)
            }

            /// Return the json representation of self.
            ///
            /// Returns:
            ///     str: The serialized form of self.
            ///
            /// Raises:
            ///     ValueError: Cannot serialize object to json.
            pub fn to_json(&self) -> PyResult<String> {
                let serialized = serde_json::to_string(&self.internal)
                    .map_err(|_| PyValueError::new_err("Cannot serialize object to json".to_string()))?;
                Ok(serialized)
            }

            /// Convert the json representation of self to an instance.
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
                            "Input cannot be deserialized: {}",
                            err
                        ))
                    })?,
                })
            }

            /// Return a string containing a printable representation of self.
            ///
            /// Returns:
            ///     str: The printable string representation of self.
            pub fn __str__(&self) -> String {
                format!("{}", self.internal)
            }

            /// Return a string containing a printable representation of self.
            ///
            /// Returns:
            ///     str: The printable string representation of self.
            pub fn __repr__(&self) -> String {
                format!("{}", self.internal)
            }

            /// Return the __richcmp__ magic method to perform rich comparison operations on object.
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
            pub fn __richcmp__(&self, other: Py<PyAny>, op: pyo3::class::basic::CompareOp) -> PyResult<bool> {
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
                let min_version: (usize, usize, usize) = #struct_ident::min_supported_version();
                return format!("{}.{}.{}", min_version.0, min_version.1, min_version.2);
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
