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
//! In general the enduser should use the high-level modules [struqture::mixed_systems::MixedOperator] and [struqture::mixed_systems::MixedHamiltonian]
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

mod bosonic_operator;
pub use bosonic_operator::BosonOperatorWrapper;

mod bosonic_hamiltonian;
pub use bosonic_hamiltonian::BosonHamiltonianWrapper;

mod bosonic_noise_operator;
pub use bosonic_noise_operator::BosonLindbladNoiseOperatorWrapper;

mod bosonic_open_system;
pub use bosonic_open_system::BosonLindbladOpenSystemWrapper;

/// Bosons module of struqture Python interface
///
/// Module for representing bosonic indices (BosonProduct and HermitianBosonProduct), bosonic systems (BosonOperator and BosonHamiltonian),
/// and Lindblad type bosonic open systems (BosonLindbladNoiseOperator, BosonLindbladOpenSystem).
///
/// .. autosummary::
///     :toctree: generated/
///
///     BosonProduct
///     HermitianBosonProduct
///     BosonOperator
///     BosonHamiltonian
///     BosonLindbladNoiseOperator
///     BosonLindbladOpenSystem
///
#[pymodule]
pub fn bosons(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    // pyo3_log::init();
    m.add_class::<BosonProductWrapper>()?;
    m.add_class::<HermitianBosonProductWrapper>()?;
    m.add_class::<BosonOperatorWrapper>()?;
    m.add_class::<BosonHamiltonianWrapper>()?;
    m.add_class::<BosonLindbladNoiseOperatorWrapper>()?;
    m.add_class::<BosonLindbladOpenSystemWrapper>()?;

    Ok(())
}
