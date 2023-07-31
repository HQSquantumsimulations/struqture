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
//! A boson system can contain any combination of none, one or several subsystems of spin, bosonic or fermionic types.
//! For example a mixed system with two spin-subsystems or a mixed system with a spin-subsystem and a bosonic-subsystem would both be valid.
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

mod boson_product;
pub use boson_product::BosonProductWrapper;

mod hermitian_boson_product;
pub use hermitian_boson_product::HermitianBosonProductWrapper;

mod bosonic_system;
pub use bosonic_system::BosonSystemWrapper;

mod bosonic_hamiltonian_system;
pub use bosonic_hamiltonian_system::BosonHamiltonianSystemWrapper;

mod bosonic_noise_system;
pub use bosonic_noise_system::BosonLindbladNoiseSystemWrapper;

mod bosonic_open_system;
pub use bosonic_open_system::BosonLindbladOpenSystemWrapper;

/// Bosons module of struqture Python interface
///
/// Module for representing bosonic indices (BosonProduct and HermitianBosonProduct), bosonic systems (BosonSystem and BosonHamiltonianSystem),
/// and Lindblad type bosonic open systems (BosonLindbladNoiseSystem, BosonLindbladOpenSystem).
///
/// .. autosummary::
///     :toctree: generated/
///
///     BosonProduct
///     HermitianBosonProduct
///     BosonSystem
///     BosonHamiltonianSystem
///     BosonLindbladNoiseSystem
///     BosonLindbladOpenSystem
///
#[pymodule]
pub fn bosons(_py: Python, m: &PyModule) -> PyResult<()> {
    // pyo3_log::init();
    m.add_class::<BosonProductWrapper>()?;
    m.add_class::<HermitianBosonProductWrapper>()?;
    m.add_class::<BosonSystemWrapper>()?;
    m.add_class::<BosonHamiltonianSystemWrapper>()?;
    m.add_class::<BosonLindbladNoiseSystemWrapper>()?;
    m.add_class::<BosonLindbladOpenSystemWrapper>()?;

    Ok(())
}
