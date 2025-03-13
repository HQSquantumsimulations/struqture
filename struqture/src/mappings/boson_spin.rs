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

//! Jordan-Wigner mapping between fermionic operators and spin operators.
//!
//! The convention used is to treat the qubit state $|0 \rangle$ as empty, and the state $|1\rangle$
//! as occupied by a fermion. The corresponding mapping is given by
//!
//! JW(a_p^{dagger}) = ( \prod_{i = 1}^{p - 1} Z_i )(X_p - i Y_p)*1/2
//! JW(a_p) = ( \prod_{i = 1}^{p - 1} Z_i )(X_p + i Y_p)*1/2

use crate::StruqtureError;

pub trait BosonToSpin {
    /// The Output type for the BosonToSpin transformation
    ///
    /// For a HermitianBosonProduct it will be a PauliOperator.
    /// For a BosonHamiltonian it will be a PauliOperator.
    type Output;

    /// Transform the given bosonic object into a spin object using
    /// the mapping.
    fn boson_spin_mapping(
        &self,
        number_spins_per_bosonic_mode: usize,
    ) -> Result<Self::Output, StruqtureError>;
}
