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

//! Module for representing fermionic physical systems
//!
//! This module can be used to represent fermionic quantum operators, fermionic quantum Hamiltonians and fermionic open quantum systems.
//!
//! In general the enduser should use the high-level [crate::fermions::FermionSystem] and [crate::fermions::FermionHamiltonian] structs
//! to represent fermionic quantum Operators and fermionic Hamiltonians respectively.
//!
//! Open Quantum Systems should be represented using [crate::fermions::FermionLindbladOpenSystem].

mod fermionic_hamiltonian;
mod fermionic_indices;
mod fermionic_noise_operator;
mod fermionic_open_system;
mod fermionic_operator;
use std::str::FromStr;

pub use fermionic_hamiltonian::FermionHamiltonian;
pub use fermionic_noise_operator::FermionLindbladNoiseOperator;
pub use fermionic_open_system::FermionLindbladOpenSystem;
pub use fermionic_operator::FermionOperator;

use crate::{ModeIndex, OperateOnDensityMatrix, SymmetricIndex};
pub use fermionic_indices::{FermionProduct, HermitianFermionProduct};
use qoqo_calculator::CalculatorComplex;

/// Signal Trait for specifying that a type can be used a fermionic index.
///
/// Implies the type implements the general [crate::SymmetricIndex] trait
/// for creator-annihilator indices.
pub trait FermionIndex:
    ModeIndex
    + SymmetricIndex
    + std::hash::Hash
    + Eq
    + Sized
    + Clone
    + std::fmt::Debug
    + std::fmt::Display
    + FromStr
    + Default
{
}

/// Trait for operations on fermions.
///
/// All operators acting on fermions can always be represented as a sum over products of fermion creation and annihilation operators.
/// This trait provides the corresponding functions to obtain the complex prefactor of the products in an operator
/// and iterate over products with non-zero prefactors.
///
/// # Example
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use std::collections::HashMap;
/// use struqture::fermions::{OperateOnFermions, FermionProduct, FermionOperator};
///
/// let mut bo = FermionOperator::new();
/// let bp_0c = FermionProduct::new([0], []).unwrap();
/// bo.set(bp_0c.clone(), CalculatorComplex::from(0.2)).unwrap();
/// // Inspecting the created object
/// println!("{}", &bo);
/// // Iterating over keys and values :
/// for (key, val) in bo.iter() {
///     println!("{}:{}", key, val);
/// }
/// ```
///
pub trait OperateOnFermions<'a>:
    OperateOnDensityMatrix<'a>
    + IntoIterator
    + FromIterator<(Self::Index, CalculatorComplex)>
    + Extend<(Self::Index, CalculatorComplex)>
    + PartialEq
    + Clone
where
    &'a Self: IntoIterator,
{
}
