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

//! Module for representing bosonic physical systems
//!
//! This module can be used to represent bosonic quantum operators, bosonic quantum Hamiltonians and bosonic open quantum systems.
//!
//! In general the enduser should use the high-level [crate::bosons::BosonSystem] and [crate::bosons::BosonHamiltonian] structs
//! to represent bosonic quantum Operators and bosonic Hamiltonians respectively.
//!
//! Open Quantum Systems should be represented using [crate::bosons::BosonLindbladOpenSystem].

mod bosonic_hamiltonian;
mod bosonic_hamiltonian_system;
mod bosonic_indices;
mod bosonic_noise_operator;
mod bosonic_noise_system;
mod bosonic_open_system;
mod bosonic_operator;
mod bosonic_system;
use std::str::FromStr;

pub use bosonic_hamiltonian::BosonHamiltonian;
pub use bosonic_hamiltonian_system::BosonHamiltonianSystem;
pub use bosonic_noise_operator::BosonLindbladNoiseOperator;
pub use bosonic_noise_system::BosonLindbladNoiseSystem;
pub use bosonic_open_system::BosonLindbladOpenSystem;
pub use bosonic_operator::BosonOperator;
pub use bosonic_system::BosonSystem;

use crate::{ModeIndex, OperateOnDensityMatrix, SymmetricIndex};
pub use bosonic_indices::{BosonProduct, HermitianBosonProduct};
use qoqo_calculator::CalculatorComplex;

/// Signal Trait for specifying that a type can be used a bosonic index.
///
/// Implies the type implements the general [crate::SymmetricIndex] trait
/// for creator-annihilator indices.
pub trait BosonIndex:
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

/// Trait for operations on bosons.
///
/// All operators acting on bosons can always be represented as a sum over products of boson creation and annihilation operators.
/// This trait provides the corresponding functions to obtain the complex prefactor of the products in an operator
/// and iterate over products with non-zero prefactors.
///
/// # Example
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use std::collections::HashMap;
/// use struqture::bosons::{OperateOnBosons, BosonProduct, BosonOperator};
///
/// let mut bo = BosonOperator::new();
/// let bp_0c = BosonProduct::new([0], []).unwrap();
/// bo.set(bp_0c.clone(), CalculatorComplex::from(0.2)).unwrap();
/// // Inspecting the created object
/// println!("{}", &bo);
/// // Iterating over keys and values :
/// for (key, val) in bo.iter() {
///     println!("{}:{}", key, val);
/// }
/// ```
///
pub trait OperateOnBosons<'a>:
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
