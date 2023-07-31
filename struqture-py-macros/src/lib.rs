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

use quote::format_ident;
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, Token, Type, TypePath};

mod product_wrapper;
use product_wrapper::productwrapper;

/// Attribute macro for constructing the pyo3 implementation for mixed indices.
#[proc_macro_attribute]
pub fn product_wrapper(
    metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    productwrapper(metadata, input)
}

mod noiseless_system_wrapper;
use noiseless_system_wrapper::noiselesswrapper;

/// Attribute macro for constructing the pyo3 implementation for noiseless systems.
#[proc_macro_attribute]
pub fn noiseless_system_wrapper(
    metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    noiselesswrapper(metadata, input)
}

mod noisy_system_wrapper;
use noisy_system_wrapper::noisywrapper;

/// Attribute macro for constructing the pyo3 implementation for noisy systems.
#[proc_macro_attribute]
pub fn noisy_system_wrapper(
    metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    noisywrapper(metadata, input)
}

mod mappings;
use mappings::mappings_macro;

/// Attribute macro for constructing the pyo3 implementation for mappings.
#[proc_macro_attribute]
pub fn mappings(
    metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    mappings_macro(metadata, input)
}

// Helper functions
// Struct for parsed derive macro arguments. Used to identify structs belonging to enums
#[derive(Debug)]
struct AttributeMacroArguments(HashSet<String>);

impl AttributeMacroArguments {
    pub fn contains(&self, st: &str) -> bool {
        self.0.contains(st)
    }
    pub fn _ids(&self) -> Vec<Ident> {
        self.0
            .clone()
            .into_iter()
            .map(|s| format_ident!("{}", s))
            .collect()
    }
}

fn strip_python_wrapper_name(ident: &Type) -> (String, proc_macro2::Ident) {
    // get name of the interal struct (not the wrapper)
    let type_path = match ident.clone() {
        Type::Path(TypePath { path: p, .. }) => p,
        _ => panic!("Trait only supports newtype variants with normal types of form path"),
    };
    let type_string = match type_path.get_ident() {
        Some(ident_path) => ident_path.to_string(),
        _ => match type_path.segments.last() {
            Some(segment) => segment.ident.to_string(),
            None => panic!("Can't extract string."),
        },
    };
    // Cut off "Wrapper" at the end of the Impl name
    let struct_name = type_string
        .as_str()
        .strip_suffix("Wrapper")
        .expect("Not conform to Wrapper naming scheme.");
    let struct_ident = format_ident!("{}", struct_name);
    (struct_name.to_string(), struct_ident)
}

impl Parse for AttributeMacroArguments {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        // Parse arguments as comma separated list of idents
        let arguments = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Self(
            arguments.into_iter().map(|id| id.to_string()).collect(),
        ))
    }
}
