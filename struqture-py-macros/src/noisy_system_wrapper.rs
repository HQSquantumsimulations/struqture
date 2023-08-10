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

pub fn noisywrapper(
    metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as ItemImpl);
    let ident = parsed_input.self_ty;
    let items = parsed_input.items;
    let attribute_arguments = parse_macro_input!(metadata as AttributeMacroArguments);
    let (struct_name, struct_ident) = strip_python_wrapper_name(&ident);
    let index_type = if struct_name.contains("Spin") {
        quote::format_ident!("DecoherenceProductWrapper")
    } else if struct_name.contains("PlusMinus") {
        quote::format_ident!("PlusMinusProductWrapper")
    } else if struct_name.contains("Boson") {
        quote::format_ident!("BosonProductWrapper")
    } else if struct_name.contains("Fermion") {
        quote::format_ident!("FermionProductWrapper")
    } else {
        quote::format_ident!("MixedDecoherenceProductWrapper")
    };
    // ------------
    // Start the generating part of the macro
    let operate_on_density_matrix_quote = if attribute_arguments.contains("OperateOnDensityMatrix")
    {
        quote! {
                /// Get the coefficient corresponding to the key.
                ///
                /// Args:
                ///     key: Product to get the value of.
                ///
                /// Returns:
                ///     CalculatorComplex: Value at key (or 0.0).
                ///
                /// Raises:
                ///     ValueError: Left-hand product could not be constructed from key.
                ///     ValueError: Right-hand product could not be constructed from key.
                pub fn get(&self, key: (Py<PyAny>, Py<PyAny>)) -> PyResult<CalculatorComplexWrapper> {
                    let (converted_left, converted_right) = (
                        #index_type::from_pyany(key.0).map_err(|err| {
                            PyValueError::new_err(format!(
                                "Product could not be constructed: {:?}",
                                err
                            ))
                        })?,
                        #index_type::from_pyany(key.1).map_err(|err| {
                            PyValueError::new_err(format!(
                                "Product could not be constructed: {:?}",
                                err
                            ))
                        })?,
                    );
                    Ok(CalculatorComplexWrapper {
                        internal: self
                            .clone()
                            .internal
                            .get(&(converted_left, converted_right))
                            .clone(),
                    })
                }

                /// Remove the value of the input object key.
                ///
                /// Returns:
                ///     Optional[CalculatorComplex]: Key existed if this is not None, and this is the value it had before it was removed.
                ///
                /// Raises:
                ///     ValueError: Left-hand Product could not be constructed.
                ///     ValueError: Right-hand Product could not be constructed.
                pub fn remove(
                    &mut self,
                    key: (Py<PyAny>, Py<PyAny>),
                ) -> PyResult<Option<CalculatorComplexWrapper>> {
                    let (converted_left, converted_right) = (
                        #index_type::from_pyany(key.0).map_err(|err| {
                            PyValueError::new_err(format!(
                                "Product could not be constructed: {:?}",
                                err
                            ))
                        })?,
                        #index_type::from_pyany(key.1).map_err(|err| {
                            PyValueError::new_err(format!(
                                "Product could not be constructed: {:?}",
                                err
                            ))
                        })?,
                    );
                    match self.internal.remove(&(converted_left, converted_right)) {
                        Some(x) => Ok(Some(CalculatorComplexWrapper { internal: x })),
                        None => Ok(None),
                    }
                }

                /// Overwrite an existing entry or set a new entry in self.
                ///
                /// Returns:
                ///     Optional[CalculatorComplex]: Key existed if this is not None, and this is the value it had before it was overwritten.
                ///
                /// Raises:
                ///     ValueError: Left-hand Product could not be constructed.
                ///     ValueError: Right-hand Product could not be constructed.
                pub fn set(
                    &mut self,
                    key: (Py<PyAny>, Py<PyAny>),
                    value: &PyAny,
                ) -> PyResult<Option<CalculatorComplexWrapper>> {
                    let value = qoqo_calculator_pyo3::convert_into_calculator_complex(value)
                        .map_err(|_| PyTypeError::new_err("Value is not CalculatorComplex"))?;
                    let (converted_left, converted_right) = (
                        #index_type::from_pyany(key.0).map_err(|err| {
                            PyValueError::new_err(format!(
                                "Product could not be constructed: {:?}",
                                err
                            ))
                        })?,
                        #index_type::from_pyany(key.1).map_err(|err| {
                            PyValueError::new_err(format!(
                                "Product could not be constructed: {:?}",
                                err
                            ))
                        })?,
                    );
                    match self
                        .internal
                        .set((converted_left, converted_right), value)
                        .map_err(|err| {
                            PyValueError::new_err(format!("Error in set function of FermionSystem: {:?}", err))
                        })? {
                        Some(x) => Ok(Some(CalculatorComplexWrapper { internal: x })),
                        None => Ok(None),
                    }
                }

                /// Adds a new (key object, CalculatorComplex) pair to existing entries.
                ///
                /// Raises:
                ///     TypeError: Value is not CalculatorComplex or CalculatorFloat.
                ///     ValueError: Left-hand product could not be constructed.
                ///     ValueError: Right-hand product could not be constructed.
                ///     ValueError: Error in add_operator_product function of self.
                pub fn add_operator_product(
                    &mut self,
                    key: (Py<PyAny>, Py<PyAny>),
                    value: &PyAny,
                ) -> PyResult<()> {
                    let value = qoqo_calculator_pyo3::convert_into_calculator_complex(value)
                        .map_err(|_| PyTypeError::new_err("Value is not CalculatorComplex"))?;
                    let (converted_left, converted_right) = (
                        #index_type::from_pyany(key.0).map_err(|err| {
                            PyValueError::new_err(format!(
                                "Product could not be constructed: {:?}",
                                err
                            ))
                        })?,
                        #index_type::from_pyany(key.1).map_err(|err| {
                            PyValueError::new_err(format!(
                                "Product could not be constructed: {:?}",
                                err
                            ))
                        })?,
                    );
                    self.internal
                        .add_operator_product((converted_left, converted_right), value)
                        .map_err(|err| {
                            PyValueError::new_err(format!(
                                "Error in add_operator_product function of System: {:?}",
                                err
                            ))
                        })
                }

                /// Return unsorted keys in self.
                ///
                /// Returns:
                ///     list[(OperatorProduct, OperatorProduct)]: The sequence of keys of self.
                pub fn keys(&self) -> Vec<(#index_type, #index_type)> {
                    let mut system_keys: Vec<(#index_type, #index_type)> = Vec::new();
                    for (key_l, key_r) in self.internal.keys() {
                        system_keys.push(
                            (#index_type { internal: key_l.clone() }, #index_type { internal: key_r.clone() })
                        );
                }
                    system_keys
                }

                /// Return unsorted values in self.
                ///
                /// Returns:
                ///     list[CalculatorComplex]: The sequence of values of self.
                pub fn values(&self) -> Vec<CalculatorComplexWrapper> {
                    let mut system_values: Vec<CalculatorComplexWrapper> = Vec::new();
                    for val in self.internal.values() {
                        system_values.push(
                            CalculatorComplexWrapper { internal: val.clone() },
                        );
                    }
                    system_values
                }

                /// Return number of entries in object.
                ///
                /// Returns:
                ///     int: The length of the content of self.
                pub fn __len__(&self) -> usize {
                    self.internal.len()
                }

                /// Return an instance of self that has no entries but clones all other properties, with the given capacity.
                ///
                /// Args:
                ///     capacity: The capacity of the object to create.
                ///
                /// Returns:
                ///     self: An empty clone with the same properties as self, with the given capacity.
                #[pyo3(signature = (capacity = None))]
                pub fn empty_clone(&self, capacity: Option<usize>) -> #ident {
                    #ident {
                        internal: self.internal.empty_clone(capacity)
                    }
                }

                /// Return true if object contains no values.
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

                /// Implement `*` for self and Union[CalculatorComplex, CalculatorFloat].
                ///
                /// Args:
                ///     value (Union[CalculatorComplex, CalculatorFloat]): value by which to multiply self by.
                ///
                /// Returns:
                ///     self: The object multiplied by the value.
                ///
                /// Raises:
                ///     ValueError: The rhs of the multiplication is neither CalculatorFloat nor CalculatorComplex.
                pub fn __mul__(&self, value: &PyAny) -> PyResult<#ident> {
                    let cf_value = qoqo_calculator_pyo3::convert_into_calculator_float(value);
                    match cf_value {
                        Ok(x) => Ok(#ident {
                            internal: self.clone().internal * x,
                        }),
                        Err(_) => {
                            let cc_value = qoqo_calculator_pyo3::convert_into_calculator_complex(value);
                            match cc_value {
                                Ok(x) => Ok(#ident { internal: self.clone().internal * x }),
                                Err(err) => Err(PyValueError::new_err(format!(
                                    "The rhs of the multiplication is neither CalculatorFloat nor CalculatorComplex: {:?}",
                                    err)))
                            }
                        }
                    }
                }
        }
    } else {
        TokenStream::new()
    };
    let operate_on_modes_quote = if attribute_arguments.contains("OperateOnModes") {
        quote! {
            /// Return maximum index in object.
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
        }
    } else {
        TokenStream::new()
    };
    let operate_on_spins_quote = if attribute_arguments.contains("OperateOnSpins") {
        quote! {
            /// Return maximum spin index in object.
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
                ///     number_spins: The number of spins in self.
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
        }
    } else {
        TokenStream::new()
    };
    let open_system_quote = if attribute_arguments.contains("OpenSystem") {
        let (system_type, system_index_type, value_type, noise_type) =
            if struct_name.contains("Spin") {
                (
                    quote::format_ident!("SpinHamiltonianSystemWrapper"),
                    quote::format_ident!("PauliProductWrapper"),
                    quote::format_ident!("CalculatorFloatWrapper"),
                    quote::format_ident!("SpinLindbladNoiseSystemWrapper"),
                )
            } else if struct_name.contains("Boson") {
                (
                    quote::format_ident!("BosonHamiltonianSystemWrapper"),
                    quote::format_ident!("HermitianBosonProductWrapper"),
                    quote::format_ident!("CalculatorComplexWrapper"),
                    quote::format_ident!("BosonLindbladNoiseSystemWrapper"),
                )
            } else if struct_name.contains("Fermion") {
                (
                    quote::format_ident!("FermionHamiltonianSystemWrapper"),
                    quote::format_ident!("HermitianFermionProductWrapper"),
                    quote::format_ident!("CalculatorComplexWrapper"),
                    quote::format_ident!("FermionLindbladNoiseSystemWrapper"),
                )
            } else {
                (
                    quote::format_ident!("MixedHamiltonianSystemWrapper"),
                    quote::format_ident!("HermitianMixedProductWrapper"),
                    quote::format_ident!("CalculatorComplexWrapper"),
                    quote::format_ident!("MixedLindbladNoiseSystemWrapper"),
                )
            };
        quote! {
            /// Return the system part of self.
            ///
            /// Returns:
            ///     System type: The system of self.
            pub fn system(&self) -> #system_type {
                #system_type {
                    internal: self.internal.system().clone(),
                }
            }

            /// Return the noise part of self.
            ///
            /// Returns:
            ///     Noise type: The noise of self.
            pub fn noise(&self) -> #noise_type {
                #noise_type {
                    internal: self.internal.noise().clone(),
                }
            }

            /// Return a tuple of the system and the noise of self.
            ///
            /// Returns:
            ///     (System, Noise): The system and noise of self.
            pub fn ungroup(
                &self,
            ) -> (
                #system_type,
                #noise_type,
            ) {
                (self.system(), self.noise())
            }

            /// Take a tuple of a system term and a noise term and combines them to be a OpenSystem.
            ///
            /// Args:
            ///     system: The system to have in the new instance.
            ///     noise: The noise to have in the new instance.
            ///
            /// Returns:
            ///     self: The OpenSystem with input system and noise terms.
            ///
            /// Raises:
            ///     ValueError: System could not be constructed.
            ///     ValueError: Noise could not be constructed.
            ///     ValueError: Grouping could not be constructed.
            #[staticmethod]
            pub fn group(system: Py<PyAny>, noise: Py<PyAny>) -> PyResult<Self> {
                let system = #system_type::from_pyany(system).map_err(|err| {
                    PyValueError::new_err(format!("System could not be constructed: {:?}", err))
                })?;
                let noise = #noise_type::from_pyany(noise).map_err(|err| {
                    PyValueError::new_err(format!("Noise could not be constructed: {:?}", err))
                })?;
                let new_self = #struct_ident::group(system, noise).map_err(|err| {
                    PyValueError::new_err(format!("Grouping could not be constructed: {:?}", err))
                })?;
                Ok(Self { internal: new_self })
            }

            /// Return an instance of self that has no entries but clones all other properties, with the given capacity.
            ///
            /// Returns:
            ///     self: An empty clone with the same properties as self, with the given capacity.
            pub fn empty_clone(&self) -> #ident {
                #ident {
                    internal: self.internal.empty_clone()
                }
            }

            /// Truncate self by returning a copy without entries under a threshold.
            ///
            /// Args:
            ///     threshold: The threshold for inclusion.
            ///
            /// Returns:
            ///     self: The truncated version of self.
            pub fn truncate(&self, threshold: f64) -> #ident {
                #ident {
                    internal: self.internal.truncate(threshold)
                }
            }

            /// Set a new entry in the system of the open system.
            ///
            /// Args:
            ///     key (Product type): Product key of set object.
            ///     value (Union[CalculatorComplex, CalculatorFloat]): Value of set object.
            ///
            /// Returns:
            ///     OpenSystem: The OpenSystem with the new entry.
            ///
            /// Raises:
            ///     ValueError: key element cannot be converted to product.
            ///     TypeError: Value cannot be converted to Union[CalculatorComplex, CalculatorFloat].
            pub fn system_set(
                &mut self,
                key: Py<PyAny>,
                value: Py<PyAny>,
            ) -> PyResult<#ident> {
                let pp = #system_index_type::from_pyany(key)?;
                let value = #value_type::from_pyany(value)
                    .map_err(|_| PyTypeError::new_err("Value cannot be converted to Union[CalculatorComplex, CalculatorFloat]"))?;

                self.internal.system_mut().set(pp, value).map_err(|_| PyTypeError::new_err("Couldn't set key and value combination"))?;

                Ok(#ident {
                    internal: self.internal.clone(),
                })
            }

            /// Set a new entry in the noise of the open system.
            ///
            /// Args:
            ///     key (Tuple(Product type, Product type)): Tuple of Products of set object.
            ///     value (CalculatorComplex): CalculatorComplex value of set object.
            ///
            /// Returns:
            ///     OpenSystem: The OpenSystem with the new entry.
            ///
            /// Raises:
            ///     ValueError: Left key element cannot be converted to product.
            ///     ValueError: Right key element cannot be converted to product.
            ///     TypeError: Value cannot be converted to Union[CalculatorComplex, CalculatorFloat].
            pub fn noise_set(
                &mut self,
                key: (Py<PyAny>, Py<PyAny>),
                value: Py<PyAny>,
            ) -> PyResult<#ident> {
                let dp_left = #index_type::from_pyany(key.0)?;
                let dp_right = #index_type::from_pyany(key.1)?;
                let value = CalculatorComplexWrapper::from_pyany(value)
                    .map_err(|_| PyTypeError::new_err("Value is not CalculatorComplex"))?;

                self.internal.noise_mut().set((dp_left, dp_right), value).map_err(|_| PyTypeError::new_err("Couldn't set key and value combination"))?;

                Ok(#ident {
                    internal: self.internal.clone(),
                })
            }

            /// Get the CalculatorComplex or CalculatorFloat coefficient corresponding to the key.
            ///
            /// Args:
            ///     key (Product type): Product key of set object.
            ///
            /// Returns:
            ///     CalculatorComplex or CalculatorFloat: Value at key (or 0.0).
            ///
            /// Raises:
            ///     ValueError: key element cannot be converted to product.
            pub fn system_get(
                &mut self,
                key: Py<PyAny>,
            ) -> PyResult<#value_type> {
                let pp = #system_index_type::from_pyany(key)?;
                let get_value = self.internal.system().get(&pp);

                Ok(#value_type {
                    internal: get_value.into(),
                })
            }

            /// Get the CalculatorComplex coefficient corresponding to the key.
            ///
            /// Args:
            ///     key (Tuple(Product type, Product type)): Tuple of Products of set object.
            ///
            /// Returns:
            ///     CalculatorComplex: Value at key (or 0.0).
            ///
            /// Raises:
            ///     ValueError: Left key element cannot be converted to product.
            ///     ValueError: Right key element cannot be converted to product.
            pub fn noise_get(
                &mut self,
                key: (Py<PyAny>, Py<PyAny>),
            ) -> PyResult<CalculatorComplexWrapper> {
                let dp_left = #index_type::from_pyany(key.0)?;
                let dp_right = #index_type::from_pyany(key.1)?;
                let get_value = self.internal.noise().get(&(dp_left, dp_right));

                Ok(CalculatorComplexWrapper {
                    internal: get_value.into(),
                })
            }

            /// Add a new entry to the system of the open system.
            ///
            /// Args:
            ///     key (Product type): Product key of set object.
            ///     value (Union[CalculatorComplex, CalculatorFloat]): Value of set object.
            ///
            /// Returns:
            ///     OpenSystem: The OpenSystem with the new entry.
            ///
            /// Raises:
            ///     ValueError: key element cannot be converted to product.
            ///     TypeError: Value cannot be converted to Union[CalculatorComplex, CalculatorFloat].
            pub fn system_add_operator_product(
                &mut self,
                key: Py<PyAny>,
                value: Py<PyAny>,
            ) -> PyResult<#ident> {
                let pp = #system_index_type::from_pyany(key)?;
                let value = #value_type::from_pyany(value)
                    .map_err(|_| PyTypeError::new_err("Value cannot be converted to CalculatorComplex"))?;

                self.internal.system_mut().add_operator_product(pp, value).map_err(|_| PyTypeError::new_err("Couldn't add in key and value combination"))?;

                Ok(#ident {
                    internal: self.internal.clone(),
                })
            }

            /// Add a new entry to the system of the open system.
            ///
            /// Args:
            ///     key (Tuple(Product type, Product type)): Tuple of Products of set object.
            ///     value (CalculatorComplex): Value of set object.
            ///
            /// Returns:
            ///     OpenSystem: The OpenSystem with the new entry.
            ///
            /// Raises:
            ///     ValueError: Left key element cannot be converted to product.
            ///     ValueError: Right key element cannot be converted to product.
            ///     TypeError: Value cannot be converted to CalculatorComplex.
            pub fn noise_add_operator_product(
                &mut self,
                key: (Py<PyAny>, Py<PyAny>),
                value: Py<PyAny>,
            ) -> PyResult<#ident> {
                let dp_left = #index_type::from_pyany(key.0)?;
                let dp_right = #index_type::from_pyany(key.1)?;
                let value = CalculatorComplexWrapper::from_pyany(value)
                    .map_err(|_| PyTypeError::new_err("Value is not CalculatorComplex"))?;

                self.internal.noise_mut().add_operator_product((dp_left, dp_right), value).map_err(|_| PyTypeError::new_err("Number of spins exceeded"))?;

                Ok(#ident {
                    internal: self.internal.clone(),
                })
            }

                /// Implement `*` for self and CalculatorFloat.
                ///
                /// Args:
                ///     value (CalculatorFloat): value by which to multiply self by.
                ///
                /// Returns:
                ///     self: The object multiplied by the value.
                ///
                /// Raises:
                ///     ValueError: The rhs of the multiplication cannot be converted to CalculatorFloat.
                pub fn __mul__(&self, value: &PyAny) -> PyResult<#ident> {
                let cf_value = qoqo_calculator_pyo3::convert_into_calculator_float(value);
                match cf_value {
                    Ok(x) => Ok(#ident {
                        internal: self.clone().internal * x,
                    }),
                    Err(err) => Err(PyValueError::new_err(format!(
                        "The rhs of the multiplication is not a CalculatorFloat: {:?}",
                        err
                    ))),
                }
            }
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
            /// Fallible conversion of generic python object..
            pub fn from_pyany( input: Py<PyAny>
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
            #operate_on_modes_quote
            #operate_on_spins_quote
            #open_system_quote
            #to_sparse_matrix_superoperator_quote
            #operate_on_mixedsystems_quote
            #calculus_quote

            // ----------------------------------
            // Default pyo3 implementations
            /// Return a copy (copy here produces a deepcopy).
            ///
            /// Returns:
            ///     System: A deep copy of self.
            pub fn __copy__(&self) -> #ident {
                self.clone()
            }

            /// Return a deep copy .
            ///
            /// Returns:
            ///     System: A deep copy of self.
            pub fn __deepcopy__(&self, _memodict: Py<PyAny>) -> #ident {
                self.clone()
            }

            /// Convert the bincode representation of the object to an instance using the [bincode] crate.
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

            /// Return the bincode representation of the object using the [bincode] crate.
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
