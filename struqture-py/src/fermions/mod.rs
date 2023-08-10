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
//! A fermion system can contain any combination of none, one or several subsystems of spin, fermionic or fermionic types.
//! For example a mixed system with two spin-subsystems or a mixed system with a spin-subsystem and a fermionic-subsystem would both be valid.
//!
//! This module can be used to represent mixed quantum operators, mixed quantum Hamiltonians and mixed open quantum systems.
//!
//! In general the enduser should use the high-level modules [struqture::mixed_systems::MixedSystem] and [struqture::mixed_systems::MixedHamiltonianSystem]
//! to represent mixed quantum Operators and mixed Hamiltonians respectively.
//!
//! Open Quantum Systems should be represented using [struqture::mixed_systems::MixedLindbladOpenSystem].
//!
//!

use pyo3::prelude::*;

mod fermion_product;
pub use fermion_product::FermionProductWrapper;

mod hermitian_fermion_product;
pub use hermitian_fermion_product::HermitianFermionProductWrapper;

mod fermionic_system;
pub use fermionic_system::FermionSystemWrapper;

mod fermionic_hamiltonian_system;
pub use fermionic_hamiltonian_system::FermionHamiltonianSystemWrapper;

mod fermionic_noise_system;
pub use fermionic_noise_system::FermionLindbladNoiseSystemWrapper;

mod fermionic_open_system;
pub use fermionic_open_system::FermionLindbladOpenSystemWrapper;

/// Fermions module of struqture Python interface
///
/// Module for representing fermionic indices (FermionProduct and HermitianFermionProduct), fermionic systems (FermionSystem and FermionHamiltonianSystem),
/// and Lindblad type fermionic open systems (FermionLindbladNoiseSystem, FermionLindbladOpenSystem).
///
/// .. autosummary::
///     :toctree: generated/
///
///     FermionProduct
///     HermitianFermionProduct
///     FermionSystem
///     FermionHamiltonianSystem
///     FermionLindbladNoiseSystem
///     FermionLindbladOpenSystem
///
#[pymodule]
pub fn fermions(_py: Python, m: &PyModule) -> PyResult<()> {
    // pyo3_log::init();
    m.add_class::<FermionProductWrapper>()?;
    m.add_class::<HermitianFermionProductWrapper>()?;
    m.add_class::<FermionSystemWrapper>()?;
    m.add_class::<FermionHamiltonianSystemWrapper>()?;
    m.add_class::<FermionLindbladNoiseSystemWrapper>()?;
    m.add_class::<FermionLindbladOpenSystemWrapper>()?;

    Ok(())
}
