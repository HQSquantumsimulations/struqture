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
//! In general the enduser should use the high-level modules [struqture::mixed_systems::MixedOperator] and [struqture::mixed_systems::MixedHamiltonian]
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

mod fermionic_operator;
pub use fermionic_operator::FermionOperatorWrapper;

mod fermionic_hamiltonian;
pub use fermionic_hamiltonian::FermionHamiltonianWrapper;

mod fermionic_noise_operator;
pub use fermionic_noise_operator::FermionLindbladNoiseOperatorWrapper;

mod fermionic_open_system;
pub use fermionic_open_system::FermionLindbladOpenSystemWrapper;

/// Fermions module of struqture Python interface
///
/// Module for representing fermionic indices (FermionProduct and HermitianFermionProduct), fermionic systems (FermionOperator and FermionHamiltonian),
/// and Lindblad type fermionic open systems (FermionLindbladNoiseOperator, FermionLindbladOpenSystem).
///
/// .. autosummary::
///     :toctree: generated/
///
///     FermionProduct
///     HermitianFermionProduct
///     FermionOperator
///     FermionHamiltonian
///     FermionLindbladNoiseOperator
///     FermionLindbladOpenSystem
///
#[pymodule]
pub fn fermions(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    // pyo3_log::init();
    m.add_class::<FermionProductWrapper>()?;
    m.add_class::<HermitianFermionProductWrapper>()?;
    m.add_class::<FermionOperatorWrapper>()?;
    m.add_class::<FermionHamiltonianWrapper>()?;
    m.add_class::<FermionLindbladNoiseOperatorWrapper>()?;
    m.add_class::<FermionLindbladOpenSystemWrapper>()?;

    Ok(())
}
