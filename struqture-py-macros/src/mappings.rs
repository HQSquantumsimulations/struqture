// Copyright © 2021-2022 HQS Quantum Simulations GmbH. All Rights Reserved.
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

pub fn mappings_quotes(
    metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as ItemImpl);
    let ident = parsed_input.self_ty;
    let items = parsed_input.items;
    let attribute_arguments = parse_macro_input!(metadata as AttributeMacroArguments);
    let (struct_name, struct_ident) = strip_python_wrapper_name(&ident);
    // ------------
    // Start the generating part of the macro
    let jordan_wigner_spin_to_fermion_quote = if attribute_arguments
        .contains("JordanWignerSpinToFermion")
    {
        let output_wrapper_type;
        let output_type;
        let from_method;
            if struct_name == "PauliProduct" ||
            struct_name == "DecoherenceProduct" ||
            struct_name == "DecoherenceOperator" ||
            struct_name == "PlusMinusProduct" ||
            struct_name == "PlusMinusOperator" 
        {
            output_wrapper_type = quote::format_ident!("FermionSystemWrapper");
            output_type = quote::format_ident!("FermionSystem");
            from_method = quote::format_ident!("from_operator");
        } else if
            struct_name == "PlusMinusNoiseOperator"
        {
            output_wrapper_type = quote::format_ident!("FermionLindbladNoiseSystemWrapper");
            output_type = quote::format_ident!("FermionLindbladNoiseSystem");
            from_method = quote::format_ident!("from_operator");
        } else {
            panic!("JordanWignerSpinToFermion can only be implemented for spin types!")
        };
        quote! {
            /// Transform the given spin object into a fermionic object using
            /// the Jordan Wigner mapping.
            pub fn jordan_wigner(&self) -> #output_wrapper_type {
                #output_wrapper_type {
                    internal: #output_type::#from_method(
                        self.internal.jordan_wigner(), None)
                        .expect("Internal bug when creating fermion (hamiltonian) system from fermion operator (hamiltonian).")
                }
            }
        }
    } else {
        TokenStream::new()
    };
    let jordan_wigner_fermion_to_spin_quote = if attribute_arguments
        .contains("JordanWignerFermionToSpin")
    {
        let output_wrapper_type;
        let output_type;
        let from_method;
        // structs that implement FermionIndex
        if struct_name == "FermionProduct" {
            output_wrapper_type = quote::format_ident!("SpinSystemWrapper");
            output_type = quote::format_ident!("SpinSystem");
            from_method = quote::format_ident!("from_operator");
        } else if struct_name == "HermitianFermionProduct" {
            output_wrapper_type = quote::format_ident!("SpinHamiltonianSystemWrapper");
            output_type = quote::format_ident!("SpinHamiltonianSystem");
            from_method = quote::format_ident!("from_hamiltonian");
        } else {
            panic!("JordanWignerFermionToSpin can only be implemented for fermion types!")
        };

        quote! {
            /// Transform the given spin object into a fermionic object using
            /// the Jordan Wigner mapping.
            pub fn jordan_wigner(&self) -> #output_wrapper_type {
                #output_wrapper_type {
                    internal: #output_type::#from_method(
                        self.internal.jordan_wigner(), None)
                        .expect("Internal bug when creating FermionSystem from FermionOperator.")
                }
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
                    Ok(try_downcast.internal)
                }
                else {
                let get_str = input.call_method0("__str__").map_err(|_| {
                    PyTypeError::new_err("Type conversion failed".to_string())
                })?;
                let string = get_str.extract::<String>().map_err(|_| {
                    PyTypeError::new_err("Type conversion failed".to_string())
                })?;
                #struct_ident::from_str(string.as_str()).map_err(|err|
                    PyTypeError::new_err(format!(
                        "Type conversion failed: {}",
                        err
                    )))

            }
                }

            )
        }
    }
        #[pymethods]
        impl #ident {

            #(#items)*

            #jordan_wigner_spin_to_fermion_quote
            #jordan_wigner_fermion_to_spin_quote

            // ----------------------------------
            // Default pyo3 implementations

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
            pub fn __deepcopy__(&self, _memodict: Py<PyAny>) -> #ident {
                self.clone()
            }

            /// Convert the bincode representation of the object to an instance using the [bincode] crate.
            ///
            /// Args:
            ///     input (ByteArray): The serialized object (in [bincode] form).
            ///
            /// Returns:
            ///    The deserialized Spin System.
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

            /// Return the __hash__ magic method.
            ///
            /// Returns:
            ///     integer: Hash
            pub fn __hash__(&self) -> PyResult<isize> {
                let mut hasher = DefaultHasher::new();
                self.internal.hash(&mut hasher);
                Ok(hasher.finish() as isize)
            }

        }

    };
    q.into()
}
