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
    let (index_type, struqture_1_module, struqture_1_ident) =
        if struct_name.contains("PauliLindbladNoiseOperator") {
            (
                quote::format_ident!("DecoherenceProductWrapper"),
                quote::format_ident!("spins"),
                quote::format_ident!("SpinLindbladNoiseSystem"),
            )
        } else if struct_name.contains("PauliLindbladOpenSystem") {
            (
                quote::format_ident!("DecoherenceProductWrapper"),
                quote::format_ident!("spins"),
                quote::format_ident!("SpinLindbladOpenSystem"),
            )
        } else if struct_name.contains("PlusMinusLindbladNoiseOperator") {
            (
                quote::format_ident!("PlusMinusProductWrapper"),
                quote::format_ident!("spins"),
                quote::format_ident!("PlusMinusLindbladNoiseOperator"),
            )
        } else if struct_name.contains("MixedLindbladNoiseOperator") {
            (
                quote::format_ident!("MixedDecoherenceProductWrapper"),
                quote::format_ident!("mixed_systems"),
                quote::format_ident!("MixedLindbladNoiseSystem"),
            )
        } else if struct_name.contains("MixedLindbladOpenSystem") {
            (
                quote::format_ident!("MixedDecoherenceProductWrapper"),
                quote::format_ident!("mixed_systems"),
                quote::format_ident!("MixedLindbladOpenSystem"),
            )
        } else if struct_name.contains("BosonLindbladNoiseOperator") {
            (
                quote::format_ident!("BosonProductWrapper"),
                quote::format_ident!("bosons"),
                quote::format_ident!("BosonLindbladNoiseSystem"),
            )
        } else if struct_name.contains("BosonLindbladOpenSystem") {
            (
                quote::format_ident!("BosonProductWrapper"),
                quote::format_ident!("bosons"),
                quote::format_ident!("BosonLindbladOpenSystem"),
            )
        } else if struct_name.contains("FermionLindbladNoiseOperator") {
            (
                quote::format_ident!("FermionProductWrapper"),
                quote::format_ident!("fermions"),
                quote::format_ident!("FermionLindbladNoiseSystem"),
            )
        } else {
            (
                quote::format_ident!("FermionProductWrapper"),
                quote::format_ident!("fermions"),
                quote::format_ident!("FermionLindbladOpenSystem"),
            )
        };
    // ------------
    // Start the generating part of the macro
    let operate_on_density_matrix_quote = if attribute_arguments.contains("OperateOnDensityMatrix")
    {
        quote! {
                /// Get the coefficient corresponding to the key.
                ///
                /// Args:
                ///     key (Tuple[Product type, Product type]): Product to get the value of.
                ///
                /// Returns:
                ///     CalculatorComplex: Value at key (or 0.0).
                ///
                /// Raises:
                ///     ValueError: Left-hand product could not be constructed from key.
                ///     ValueError: Right-hand product could not be constructed from key.
                pub fn get(&self, key: (Py<PyAny>, Py<PyAny>)) -> PyResult<CalculatorComplexWrapper> {
                    Python::with_gil(|py| -> PyResult<CalculatorComplexWrapper> {
                        let (converted_left, converted_right) = (
                            #index_type::from_pyany(key.0.bind(py)).map_err(|err| {
                                PyValueError::new_err(format!(
                                    "Product could not be constructed: {:?}",
                                    err
                                ))
                            })?,
                            #index_type::from_pyany(key.1.bind(py)).map_err(|err| {
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
                    })
                }

                /// Remove the value of the input object key.
                ///
                /// Args:
                ///     key (Tuple[Product type, Product type]): The key of the value to remove.
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
                    Python::with_gil(|py| -> PyResult<Option<CalculatorComplexWrapper>> {
                        let (converted_left, converted_right) = (
                            #index_type::from_pyany(key.0.bind(py)).map_err(|err| {
                                PyValueError::new_err(format!(
                                    "Product could not be constructed: {:?}",
                                    err
                                ))
                            })?,
                            #index_type::from_pyany(key.1.bind(py)).map_err(|err| {
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
                    })
                }

                /// Overwrite an existing entry or set a new entry in self.
                ///
                /// Args:
                ///     key (Tuple[Product type, Product type]): The key of the value to set.
                ///     value (CalculatorComplex): The value to set.
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
                    value: &Bound<PyAny>,
                ) -> PyResult<Option<CalculatorComplexWrapper>> {
                    Python::with_gil(|py| -> PyResult<Option<CalculatorComplexWrapper>> {
                        let value = qoqo_calculator_pyo3::convert_into_calculator_complex(value)
                            .map_err(|_| PyTypeError::new_err("Value is not CalculatorComplex"))?;
                        let (converted_left, converted_right) = (
                            #index_type::from_pyany(key.0.bind(py)).map_err(|err| {
                                PyValueError::new_err(format!(
                                    "Product could not be constructed: {:?}",
                                    err
                                ))
                            })?,
                            #index_type::from_pyany(key.1.bind(py)).map_err(|err| {
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
                                PyValueError::new_err(format!("Error in set function of FermionOperator: {:?}", err))
                            })? {
                            Some(x) => Ok(Some(CalculatorComplexWrapper { internal: x })),
                            None => Ok(None),
                        }
                    })
                }

                /// Adds a new (key object, CalculatorComplex) pair to existing entries.
                ///
                /// Args:
                ///     key (Tuple[Product type, Product type]): The key of the value to add.
                ///     value (CalculatorComplex): The value to add.
                ///
                /// Raises:
                ///     TypeError: Value is not CalculatorComplex or CalculatorFloat.
                ///     ValueError: Left-hand product could not be constructed.
                ///     ValueError: Right-hand product could not be constructed.
                ///     ValueError: Error in add_operator_product function of self.
                pub fn add_operator_product(
                    &mut self,
                    key: (Py<PyAny>, Py<PyAny>),
                    value: &Bound<PyAny>,
                ) -> PyResult<()> {
                    Python::with_gil(|py| -> PyResult<()> {
                        let value = qoqo_calculator_pyo3::convert_into_calculator_complex(value)
                            .map_err(|_| PyTypeError::new_err("Value is not CalculatorComplex"))?;
                        let (converted_left, converted_right) = (
                            #index_type::from_pyany(key.0.bind(py)).map_err(|err| {
                                PyValueError::new_err(format!(
                                    "Product could not be constructed: {:?}",
                                    err
                                ))
                            })?,
                            #index_type::from_pyany(key.1.bind(py)).map_err(|err| {
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
                                    "Error in add_operator_product function of Operator: {:?}",
                                    err
                                ))
                            })
                    })
                }

                /// Return unsorted keys in self.
                ///
                /// Returns:
                ///     List[(OperatorProduct, OperatorProduct)]: The sequence of keys of self.
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
                ///     List[CalculatorComplex]: The sequence of values of self.
                pub fn values(&self) -> Vec<CalculatorComplexWrapper> {
                    let mut system_values: Vec<CalculatorComplexWrapper> = Vec::new();
                    for val in self.internal.values() {
                        system_values.push(
                            CalculatorComplexWrapper { internal: val.clone() },
                        );
                    }
                    system_values
                }

                /// Return unsorted keys in self.
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
                pub fn __mul__(&self, value: &Bound<PyAny>) -> PyResult<#ident> {
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
            /// Return the current_number_modes input of self.
            ///
            /// Returns:
            ///     int: The number of modes in self.
            pub fn current_number_modes(&self) -> usize {
                self.internal.current_number_modes()
            }
        }
    } else {
        TokenStream::new()
    };
    let operate_on_spins_quote = if attribute_arguments.contains("OperateOnSpins") {
        quote! {
            /// Return the current_number_spins input of self.
            ///
            /// Returns:
            ///     int: The number of spins in self.
            pub fn current_number_spins(&self) -> usize {
                self.internal.current_number_spins()
            }

            /// Return maximum index in self.
            ///
            /// Returns:
            ///     int: Maximum index.
            pub fn number_spins(&self) -> usize {
                Python::with_gil(|py| {
                    py.run(c_str!("import warnings; warnings.warn(\"The 'number_spins' method has been deprecated, as the total number of spins can no longer be set. Please use the 'current_number_spins' method instead. The 'number_spins' method will be removed in future.\", category=DeprecationWarning, stacklevel=2)"), None, None).unwrap();
                });
                self.internal.current_number_spins()
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
                ///     number_spins (int): The number of spins in self.
                ///
                /// Returns:
                ///     Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]]: The matrix little endian representation of self.
                ///
                /// Raises:
                ///     ValueError: CalculatorError.
                ///     RuntimeError: Could not convert to complex superoperator matrix.
                pub fn sparse_matrix_superoperator_coo(&self, number_spins: usize) -> PyResult<PyCooMatrix> {
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
        }
    } else {
        TokenStream::new()
    };
    let operate_on_mixedsystems_quote = if attribute_arguments.contains("OperateOnMixedSystems") {
        quote! {
                /// Return the current_number_spins input of each spin subsystem of self.
                ///
                /// Returns:
                ///     int: The number of spins in each spin subsystem of self.
                pub fn current_number_spins(&self) -> Vec<usize> {
                    self.internal.current_number_spins()
                }

                /// Return the number of bosonic modes in each bosonic subsystem of self.
                ///
                /// Returns:
                ///     list[int]: The number of bosonic modes in each bosonic subsystem of self.
                pub fn current_number_bosonic_modes(&self) -> Vec<usize> {
                    self.internal.current_number_bosonic_modes()
                }

                /// Return the number of fermionic modes in each fermionic subsystem of self.
                ///
                /// Returns:
                ///     list[int]: The number of fermionic modes in each fermionic subsystem of self.
                pub fn current_number_fermionic_modes(&self) -> Vec<usize> {
                    self.internal.current_number_fermionic_modes()
                }
        }
    } else {
        TokenStream::new()
    };
    let open_system_quote = if attribute_arguments.contains("OpenSystem") {
        let (system_type, system_index_type, value_type, noise_type) =
            if struct_name.contains("Pauli") {
                (
                    quote::format_ident!("PauliHamiltonianWrapper"),
                    quote::format_ident!("PauliProductWrapper"),
                    quote::format_ident!("CalculatorFloatWrapper"),
                    quote::format_ident!("PauliLindbladNoiseOperatorWrapper"),
                )
            } else if struct_name.contains("Boson") {
                (
                    quote::format_ident!("BosonHamiltonianWrapper"),
                    quote::format_ident!("HermitianBosonProductWrapper"),
                    quote::format_ident!("CalculatorComplexWrapper"),
                    quote::format_ident!("BosonLindbladNoiseOperatorWrapper"),
                )
            } else if struct_name.contains("Fermion") {
                (
                    quote::format_ident!("FermionHamiltonianWrapper"),
                    quote::format_ident!("HermitianFermionProductWrapper"),
                    quote::format_ident!("CalculatorComplexWrapper"),
                    quote::format_ident!("FermionLindbladNoiseOperatorWrapper"),
                )
            } else {
                (
                    quote::format_ident!("MixedHamiltonianWrapper"),
                    quote::format_ident!("HermitianMixedProductWrapper"),
                    quote::format_ident!("CalculatorComplexWrapper"),
                    quote::format_ident!("MixedLindbladNoiseOperatorWrapper"),
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
            pub fn group(system: &Bound<PyAny>, noise: &Bound<PyAny>) -> PyResult<Self> {
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
                key: &Bound<PyAny>,
                value: &Bound<PyAny>,
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
            ///     key (Tuple[Product type, Product type]): Tuple of Products of set object.
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
                value: &Bound<PyAny>,
            ) -> PyResult<#ident> {
                Python::with_gil(|py| -> PyResult<#ident> {
                    let dp_left = #index_type::from_pyany(key.0.bind(py))?;
                    let dp_right = #index_type::from_pyany(key.1.bind(py))?;
                    let value = CalculatorComplexWrapper::from_pyany(value)
                        .map_err(|_| PyTypeError::new_err("Value is not CalculatorComplex"))?;

                    self.internal.noise_mut().set((dp_left, dp_right), value).map_err(|_| PyTypeError::new_err("Couldn't set key and value combination"))?;

                    Ok(#ident {
                        internal: self.internal.clone(),
                    })
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
                key: &Bound<PyAny>,
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
            ///     key (Tuple[Product type, Product type]): Tuple of Products of set object.
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
                Python::with_gil(|py| -> PyResult<CalculatorComplexWrapper> {
                    let dp_left = #index_type::from_pyany(key.0.bind(py))?;
                    let dp_right = #index_type::from_pyany(key.1.bind(py))?;
                    let get_value = self.internal.noise().get(&(dp_left, dp_right));

                    Ok(CalculatorComplexWrapper {
                        internal: get_value.into(),
                    })
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
                key: &Bound<PyAny>,
                value: &Bound<PyAny>,
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
            ///     key (Tuple[Product type, Product type]): Tuple of Products of set object.
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
                value: &Bound<PyAny>,
            ) -> PyResult<#ident> {
                Python::with_gil(|py| -> PyResult<#ident> {
                    let dp_left = #index_type::from_pyany(key.0.bind(py))?;
                    let dp_right = #index_type::from_pyany(key.1.bind(py))?;
                    let value = CalculatorComplexWrapper::from_pyany(value)
                        .map_err(|_| PyTypeError::new_err("Value is not CalculatorComplex"))?;

                    self.internal.noise_mut().add_operator_product((dp_left, dp_right), value).map_err(|_| PyTypeError::new_err("Number of spins exceeded"))?;

                    Ok(#ident {
                        internal: self.internal.clone(),
                    })
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
                pub fn __mul__(&self, value: &Bound<PyAny>) -> PyResult<#ident> {
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
                let new_self = (self.clone().internal + other.internal);
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
                let new_self = (self.clone().internal - other.internal);
                Ok(#ident {
                    internal: new_self
                })
            }
        }
    } else {
        TokenStream::new()
    };
    let hermitian_calculus_quote = if attribute_arguments.contains("HermitianCalculus") {
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
            pub fn from_pyany(input: &Bound<PyAny>) -> PyResult<#struct_ident> {
                Python::with_gil(|py| -> PyResult<#struct_ident> {
                    let source_serialisation_meta = input.call_method0("_get_serialisation_meta").map_err(|_| {
                        PyTypeError::new_err("Trying to use Python object as a struqture-py object that does not behave as struqture-py object. Are you sure you have the right type to all functions?".to_string())
                    })?;
                    let source_serialisation_meta: String = source_serialisation_meta.extract().map_err(|_| {
                        PyTypeError::new_err("Trying to use Python object as a struqture-py object that does not behave as struqture-py object. Are you sure you have the right type to all functions?".to_string())
                    })?;

                    let source_serialisation_meta: struqture::StruqtureSerialisationMeta = serde_json::from_str(&source_serialisation_meta).map_err(|_| {
                        PyTypeError::new_err("Trying to use Python object as a struqture-py object that does not behave as struqture-py object. Are you sure you have the right type to all functions?".to_string())
                    })?;

                    let target_serialisation_meta = <#struct_ident as struqture::SerializationSupport>::target_serialisation_meta();

                    struqture::check_can_be_deserialised(&target_serialisation_meta, &source_serialisation_meta).map_err(|err| {
                        PyTypeError::new_err(err.to_string())
                    })?;

                    let input = input.as_ref();
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
                })
            }

            /// Fallible conversion of generic python object that is implemented in struqture 1.x.
            #[cfg(feature = "struqture_1_import")]
            pub fn from_pyany_struqture_1(input: &Bound<PyAny>) -> PyResult<#struct_ident> {
                Python::with_gil(|py| -> PyResult<#struct_ident> {
                    let input = input.as_ref();
                    let get_bytes = input
                        .call_method0("to_bincode")
                        .map_err(|_| PyTypeError::new_err("Serialisation failed".to_string()))?;
                    let bytes = get_bytes
                        .extract::<Vec<u8>>()
                        .map_err(|_| PyTypeError::new_err("Deserialisation failed".to_string()))?;
                    let one_import = deserialize(&bytes[..])
                        .map_err(|err| PyTypeError::new_err(format!("Type conversion failed: {}", err)))?;
                    let qubit_operator: #struct_ident = #struct_ident::from_struqture_1(&one_import).map_err(
                        |err| PyValueError::new_err(format!("Trying to obtain struqture 2.x object from struqture 1.x object. Conversion failed. Was the right type passed to all functions? {:?}", err)
                    ))?;
                    Ok(qubit_operator)
                })
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
            #hermitian_calculus_quote

            // ----------------------------------
            // Default pyo3 implementations

            /// Convert a json corresponding to a struqture 1 object to the equivalent object in struqture 2.
            ///
            /// Args:
            ///     input (Any): the json of the struqture 1 object to convert.
            ///
            /// Returns:
            ///     Any: the input object in struqture 2 form.
            ///
            /// Raises:
            ///     ValueError: Input could not be deserialised form json.
            ///     ValueError: Struqture 1 object could not be converted to struqture 2.
            #[cfg(feature = "struqture_1_import")]
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

            /// Return a copy (copy here produces a deepcopy).
            ///
            /// Returns:
            ///     Operator: A deep copy of self.
            pub fn __copy__(&self) -> #ident {
                self.clone()
            }

            /// Return a deep copy .
            ///
            /// Returns:
            ///     Operator: A deep copy of self.
            pub fn __deepcopy__(&self, _memodict: Py<PyAny>) -> #ident {
                self.clone()
            }

            /// Convert the bincode representation of the object to an instance using the [bincode] crate.
            ///
            /// Args:
            ///     input (bytearray): The serialized object (in [bincode] form).
            ///
            /// Returns:
            ///    The deserialized object.
            ///
            /// Raises:
            ///     TypeError: Input cannot be converted to byte array.
            ///     ValueError: Input cannot be deserialized.
            #[staticmethod]
            pub fn from_bincode(input: &Bound<PyAny>) -> PyResult<#ident> {
                let bytes = input
                    .as_ref()
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
