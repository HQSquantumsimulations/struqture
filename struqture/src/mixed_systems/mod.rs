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

//! Module for representing mixed physical systems
//!
//! A mixed physical system can contain any combination of none, one, or several subsystems of spin, bosonic, or fermionic types.
//! For example a mixed system with two spin-subsystems or a mixed system with a spin-subsystem and a bosonic-subsystem would both be valid.
//!
//! This module can be used to represent mixed quantum operators, mixed quantum Hamiltonians and mixed open quantum systems.
//!
//! In general the enduser should use the high-level [crate::mixed_systems::MixedSystem] and [crate::mixed_systems::MixedHamiltonianSystem] structs
//! to represent mixed quantum Operators and mixed Hamiltonians respectively.
//!
//! Open Quantum Systems should be represented using [crate::mixed_systems::MixedLindbladOpenSystem].
//!
//!

mod mixed_decoherence_product;
mod mixed_hamiltonian;
mod mixed_hamiltonian_system;
mod mixed_hermitian_product;
mod mixed_noise_operator;
mod mixed_noise_system;
mod mixed_open_system;
mod mixed_operator;
mod mixed_plus_minus_operator;
mod mixed_plus_minus_product;
mod mixed_product;
mod mixed_system;

use crate::{
    bosons::BosonIndex, fermions::FermionIndex, ModeIndex, OperateOnDensityMatrix, SpinIndex,
    StruqtureError,
};
#[cfg(feature = "json_schema")]
use mixed_noise_system::TinyVecDef;
use qoqo_calculator::CalculatorComplex;
use std::str::FromStr;

pub use mixed_decoherence_product::MixedDecoherenceProduct;
pub use mixed_hamiltonian::MixedHamiltonian;
pub use mixed_hamiltonian_system::MixedHamiltonianSystem;
pub use mixed_hermitian_product::HermitianMixedProduct;
pub use mixed_noise_operator::MixedLindbladNoiseOperator;
pub use mixed_noise_system::MixedLindbladNoiseSystem;
pub use mixed_open_system::MixedLindbladOpenSystem;
pub use mixed_operator::MixedOperator;
pub use mixed_plus_minus_operator::MixedPlusMinusOperator;
pub use mixed_plus_minus_product::MixedPlusMinusProduct;
pub use mixed_product::MixedProduct;
pub use mixed_system::MixedSystem;

/// Trait for all index types requires converting between index types
pub trait MixedIndex:
    std::hash::Hash
    + Eq
    + Sized
    + Clone
    + std::fmt::Debug
    + std::fmt::Display
    + FromStr
    + Default
    + serde::Serialize
where
    Self::SpinIndexType: SpinIndex,
    Self::BosonicIndexType: BosonIndex,
    Self::FermionicIndexType: FermionIndex,
{
    type SpinIndexType;
    type BosonicIndexType;
    type FermionicIndexType;

    // Document locally
    fn new(
        spins: impl IntoIterator<Item = Self::SpinIndexType>,
        bosons: impl IntoIterator<Item = Self::BosonicIndexType>,
        fermions: impl IntoIterator<Item = Self::FermionicIndexType>,
    ) -> Result<Self, StruqtureError>;

    /// Gets the spin Products of Self.
    ///
    /// # Returns
    ///
    /// * `Iter<Self::SpinIndexType>` - The spin Products in Self.
    fn spins(&self) -> std::slice::Iter<Self::SpinIndexType>;

    /// Gets the boson Products of Self.
    ///
    /// # Returns
    ///
    /// * `Iter<Self::BosonicIndexType>` - The boson Products in Self.
    fn bosons(&self) -> std::slice::Iter<Self::BosonicIndexType>;

    /// Gets the fermion Products of Self.
    ///
    /// # Returns
    ///
    /// * `Iter<Self::FermionicIndexType>` - The fermion Products in Self.
    fn fermions(&self) -> std::slice::Iter<Self::FermionicIndexType>;

    /// Returns the current number of spins each subsystem acts upon.
    ///
    /// # Returns
    ///
    /// * `Vec<usize>` - Number of spins in each spin sub-system.
    fn current_number_spins(&self) -> Vec<usize> {
        self.spins().map(|s| s.current_number_spins()).collect()
    }

    /// Returns the current number of bosonic modes each subsystem acts upon.
    ///
    /// # Returns
    ///
    /// * `Vec<usize>` - Number of bosons in each boson sub-system.
    fn current_number_bosonic_modes(&self) -> Vec<usize> {
        self.bosons().map(|b| b.current_number_modes()).collect()
    }

    /// Returns the current number of fermionic modes each subsystem acts upon.
    ///
    /// # Returns
    ///
    /// * `Vec<usize>` - Number of fermions in each fermion sub-system.
    fn current_number_fermionic_modes(&self) -> Vec<usize> {
        self.fermions().map(|f| f.current_number_modes()).collect()
    }

    // Document locally
    fn create_valid_pair(
        spins: impl IntoIterator<Item = Self::SpinIndexType>,
        bosons: impl IntoIterator<Item = Self::BosonicIndexType>,
        fermions: impl IntoIterator<Item = Self::FermionicIndexType>,
        value: CalculatorComplex,
    ) -> Result<(Self, CalculatorComplex), StruqtureError>;
}

/// Trait for transforming value stored at index I when using index of different type T to read out value
/// e.g. Hermitian Hamiltonian H but we access H[NOIndex(2,1)] -> H[HermitianIndex(1,2)].conj()
pub trait GetValueMixed<'a, T>: MixedIndex
where
    T: MixedIndex,
    T::SpinIndexType: SpinIndex,
    T::BosonicIndexType: BosonIndex,
    T::FermionicIndexType: FermionIndex,
{
    // Document locally
    fn get_key(index: &T) -> Self;

    // Document locally
    fn get_transform(index: &T, value: CalculatorComplex) -> CalculatorComplex;
}

/// Trait for operations on mixed systems.
///
/// # Example
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::spins::PauliProduct;
/// use struqture::bosons::BosonProduct;
/// use struqture::fermions::FermionProduct;
/// use struqture::mixed_systems::{OperateOnMixedSystems, MixedProduct, MixedOperator};
///
/// let mut sh = MixedOperator::new(1, 1, 1);
///
/// let mp_1: MixedProduct = MixedProduct::new([PauliProduct::new().x(0),], [BosonProduct::new([0], [1]).unwrap()], [FermionProduct::new([0], [1]).unwrap()]).unwrap();
/// let mp_0: MixedProduct = MixedProduct::new([PauliProduct::new().z(0),], [BosonProduct::new([0], [1]).unwrap()], [FermionProduct::new([0], [1]).unwrap()]).unwrap();
/// sh.set(mp_1.clone(), CalculatorComplex::from(0.5)).unwrap();
/// sh.set(mp_0.clone(), CalculatorComplex::from(0.2)).unwrap();
///
/// assert_eq!(sh.number_spins(), vec![1]);
/// assert_eq!(sh.current_number_spins(), vec![1]);
/// assert_eq!(sh.number_bosonic_modes(), vec![2]);
/// assert_eq!(sh.current_number_bosonic_modes(), vec![2]);
/// assert_eq!(sh.number_fermionic_modes(), vec![2]);
/// assert_eq!(sh.current_number_fermionic_modes(), vec![2]);
/// ```
///
pub trait OperateOnMixedSystems<'a>: PartialEq + Clone {
    /// Returns the number of spins in each spin sub-system of Self.
    ///
    /// # Returns
    ///
    /// * `Vec<usize>` - The number of spins in each sub-system of Self.
    fn number_spins(&self) -> Vec<usize>;

    /// Returns the number of spins a physical operator acts upon in each spin sub-system of Self.
    ///
    /// This corresponds to returning the maximum index the spin operator acts on in each sub-system.
    ///
    /// # Returns
    ///
    /// * `Vec<usize>` - Maximum spin index currently used in each sub-system of Self.
    fn current_number_spins(&self) -> Vec<usize>;

    /// Returns the number of bosonic modes in each boson sub-system of Self.
    ///
    /// # Returns
    ///
    /// * `Vec<usize>` - The number of bosonic modes in each sub-system of Self.
    fn number_bosonic_modes(&self) -> Vec<usize>;

    /// Returns the number of bosonic modes a physical operator acts upon in each boson sub-system of Self.
    ///
    /// This corresponds to returning the maximum index the boson operator acts on in each sub-system.
    ///
    /// # Returns
    ///
    /// * `Vec<usize>` - Maximum boson index currently used in each sub-system of Self.
    fn current_number_bosonic_modes(&self) -> Vec<usize>;

    /// Returns the number of fermionic modes in each fermion sub-system of Self.
    ///
    /// # Returns
    ///
    /// * `Vec<usize>` - The number of fermionic modes in each sub-system of Self.
    fn number_fermionic_modes(&self) -> Vec<usize>;

    /// Returns the number of fermionic modes a physical operator acts upon in each fermion sub-system of Self.
    ///
    /// This corresponds to returning the maximum index the fermion operator acts on in each sub-system.
    ///
    /// # Returns
    ///
    /// * `Vec<usize>` - Maximum fermion index currently used in each sub-system of Self.
    fn current_number_fermionic_modes(&self) -> Vec<usize>;
}

pub trait HermitianOperateOnMixedSystems<'a>:
    OperateOnMixedSystems<'a>
    + OperateOnDensityMatrix<'a>
    + IntoIterator
    + FromIterator<(Self::Index, CalculatorComplex)>
    + Extend<(Self::Index, CalculatorComplex)>
    + PartialEq
    + Clone
where
    &'a Self: IntoIterator<Item = (&'a Self::Index, &'a Self::Value)>,
{
}
