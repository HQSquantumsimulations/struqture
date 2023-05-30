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

use pyo3::prelude::*;

mod mixed_product;
pub use mixed_product::MixedProductWrapper;

mod mixed_plus_minus_product;
pub use mixed_plus_minus_product::MixedPlusMinusProductWrapper;

mod mixed_hermitian_product;
pub use mixed_hermitian_product::HermitianMixedProductWrapper;

mod mixed_decoherence_product;
pub use mixed_decoherence_product::MixedDecoherenceProductWrapper;

mod mixed_system;
pub use mixed_system::MixedSystemWrapper;

mod mixed_plus_minus_operator;
pub use mixed_plus_minus_operator::MixedPlusMinusOperatorWrapper;

mod mixed_hamiltonian_system;
pub use mixed_hamiltonian_system::MixedHamiltonianSystemWrapper;

mod mixed_noise_system;
pub use mixed_noise_system::MixedLindbladNoiseSystemWrapper;

mod mixed_open_system;
pub use mixed_open_system::MixedLindbladOpenSystemWrapper;

/// Module for representing mixed physical systems.
///
/// A mixed physical system can contain any combination of none, one, or several subsystems
/// of spin, bosonic, or fermionic types.
/// For example a mixed system with two spin-subsystems or a mixed system with a bosonic-subsystem and a bosonic-subsystem would both be valid.
///
/// This module, here the python inferface for struqture, can be used to represent
/// mixed quantum indices (MixedProduct, HermitianMixedProduct and MixedDecoherenceProduct),
/// mixed systems (MixedSystem and MixedHamiltonianSystem) and Lindblad type mixed open systems
/// (MixedLindbladNoiseSystem and MixedLindbladOpenSystem).
///
/// .. autosummary::
///     :toctree: generated/
///
///     MixedProduct
///     HermitianMixedProduct
///     MixedDecoherenceProduct
///     MixedSystem
///     MixedHamiltonianSystem
///     MixedLindbladNoiseSystem
///     MixedLindbladOpenSystem
///
#[pymodule]
pub fn mixed_systems(_py: Python, m: &PyModule) -> PyResult<()> {
    // pyo3_log::init();
    m.add_class::<MixedProductWrapper>()?;
    m.add_class::<HermitianMixedProductWrapper>()?;
    m.add_class::<MixedDecoherenceProductWrapper>()?;
    m.add_class::<MixedSystemWrapper>()?;
    m.add_class::<MixedHamiltonianSystemWrapper>()?;
    m.add_class::<MixedLindbladNoiseSystemWrapper>()?;
    m.add_class::<MixedLindbladOpenSystemWrapper>()?;

    Ok(())
}
