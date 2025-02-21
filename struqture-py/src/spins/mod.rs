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
use pyo3::prelude::*;

mod pauli_product;
pub use pauli_product::PauliProductWrapper;

mod decoherence_product;
pub use decoherence_product::DecoherenceProductWrapper;

mod plus_minus_product;
pub use plus_minus_product::PlusMinusProductWrapper;

mod plus_minus_operator;
pub use plus_minus_operator::PlusMinusOperatorWrapper;

mod plus_minus_noise_operator;
pub use plus_minus_noise_operator::PlusMinusLindbladNoiseOperatorWrapper;

mod pauli_operator;
pub use pauli_operator::PauliOperatorWrapper;

mod pauli_hamiltonian;
pub use pauli_hamiltonian::PauliHamiltonianWrapper;

mod pauli_noise_operator;
pub use pauli_noise_operator::PauliLindbladNoiseOperatorWrapper;

mod pauli_open_system;
pub use pauli_open_system::PauliLindbladOpenSystemWrapper;

/// Spin module of struqture Python interface
///
/// Module for representing spin indices (PauliProduct and DecoherenceProduct), spin systems (PauliOperator and PauliHamiltonian)
/// and Lindblad type spin open systems (PauliLindbladNoiseOperator and PauliLindbladOpenSystem).
///
/// .. autosummary::
///     :toctree: generated/
///
///     PauliProduct
///     DecoherenceProduct
///     PauliOperator
///     PauliHamiltonian
///     PauliLindbladNoiseOperator
///     PauliLindbladOpenSystem
///
#[pymodule]
pub fn spins(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    // pyo3_log::init();
    m.add_class::<PauliProductWrapper>()?;
    m.add_class::<DecoherenceProductWrapper>()?;
    m.add_class::<PauliOperatorWrapper>()?;
    m.add_class::<PauliHamiltonianWrapper>()?;
    m.add_class::<PauliLindbladNoiseOperatorWrapper>()?;
    m.add_class::<PauliLindbladOpenSystemWrapper>()?;
    m.add_class::<PlusMinusProductWrapper>()?;
    m.add_class::<PlusMinusOperatorWrapper>()?;
    m.add_class::<PlusMinusLindbladNoiseOperatorWrapper>()?;
    Ok(())
}
