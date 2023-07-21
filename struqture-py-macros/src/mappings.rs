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

pub fn mappings_macro(
    metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let impl_item = parse_macro_input!(input as ItemImpl);

    let ident = &impl_item.self_ty;
    let attribute_arguments = parse_macro_input!(metadata as AttributeMacroArguments);
    let (struct_name, _struct_ident) = strip_python_wrapper_name(ident);

    let jordan_wigner_spin_to_fermion_quote =
        jordan_wigner_spin_to_fermion_quote(&attribute_arguments, &struct_name);
    let jordan_wigner_fermion_to_spin_quote =
        jordan_wigner_fermion_to_spin_quote(&attribute_arguments, &struct_name);

    let q = quote! {

        #impl_item

        #[pymethods]
        impl #ident {

            #jordan_wigner_spin_to_fermion_quote
            #jordan_wigner_fermion_to_spin_quote
        }
    };
    q.into()
}

fn jordan_wigner_spin_to_fermion_quote(
    attribute_arguments: &AttributeMacroArguments,
    struct_name: &str,
) -> TokenStream {
    if attribute_arguments.contains("JordanWignerSpinToFermion") {
        let output_wrapper_type;
        let output_type;

        if struct_name.contains("System") {
            let mut output_wrapper_name = format!("{}Wrapper", struct_name);
            output_wrapper_name = output_wrapper_name.replace("Spin", "Fermion");
            output_wrapper_type = quote::format_ident!("{}", output_wrapper_name);

            quote! {
                /// Transform the given spin object into a fermionic object using
                /// the Jordan Wigner mapping.
                pub fn jordan_wigner(&self) -> #output_wrapper_type {
                    #output_wrapper_type {
                        internal: self.internal.jordan_wigner()
                    }
                }
            }
        } else {
            if struct_name == "PauliProduct"
                || struct_name == "DecoherenceProduct"
                || struct_name == "PlusMinusProduct"
                || struct_name == "PlusMinusOperator"
            {
                output_wrapper_type = quote::format_ident!("FermionSystemWrapper");
                output_type = quote::format_ident!("FermionSystem");
            } else if struct_name == "PlusMinusLindbladNoiseOperator" {
                output_wrapper_type = quote::format_ident!("FermionLindbladNoiseSystemWrapper");
                output_type = quote::format_ident!("FermionLindbladNoiseSystem");
            } else {
                panic!("JordanWignerSpinToFermion can only be implemented for spin types!")
            };

            quote! {
                /// Transform the given spin object into a fermionic object using
                /// the Jordan Wigner mapping.
                pub fn jordan_wigner(&self) -> #output_wrapper_type {
                    #output_wrapper_type {
                        internal: #output_type::from_operator(
                            self.internal.jordan_wigner(), None)
                            .expect("Internal bug when creating fermionic system object from fermionic operator object.")
                    }
                }
            }
        }
    } else {
        TokenStream::new()
    }
}

fn jordan_wigner_fermion_to_spin_quote(
    attribute_arguments: &AttributeMacroArguments,
    struct_name: &str,
) -> TokenStream {
    if attribute_arguments.contains("JordanWignerFermionToSpin") {
        let output_wrapper_type;
        let output_type;
        let from_method;

        if struct_name.contains("System") {
            let mut output_wrapper_name = format!("{}Wrapper", struct_name);
            output_wrapper_name = output_wrapper_name.replace("Fermion", "Spin");
            output_wrapper_type = quote::format_ident!("{}", output_wrapper_name);

            quote! {
                /// Transform the given fermionic object into a spin object using
                /// the Jordan Wigner mapping.
                pub fn jordan_wigner(&self) -> #output_wrapper_type {
                    #output_wrapper_type {
                        internal: self.internal.jordan_wigner()
                    }
                }
            }
        } else {
            if struct_name == "FermionProduct" {
                output_wrapper_type = quote::format_ident!("SpinSystemWrapper");
                output_type = quote::format_ident!("SpinSystem");
                from_method = quote::format_ident!("from_operator");
            } else if struct_name == "HermitianFermionProduct" {
                output_wrapper_type = quote::format_ident!("SpinHamiltonianSystemWrapper");
                output_type = quote::format_ident!("SpinHamiltonianSystem");
                from_method = quote::format_ident!("from_hamiltonian");
            } else {
                panic!("JordanWignerFermionToSpin can only be implemented for fermionic types!")
            };

            quote! {
                /// Transform the given fermionic object into a spin object using
                /// the Jordan Wigner mapping.
                pub fn jordan_wigner(&self) -> #output_wrapper_type {
                    #output_wrapper_type {
                        internal: #output_type::#from_method(
                            self.internal.jordan_wigner(), None)
                            .expect("Internal bug when creating spin system object from spin operator object.")
                    }
                }
            }
        }
    } else {
        TokenStream::new()
    }
}
