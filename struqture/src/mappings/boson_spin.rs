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

//! Mapping from bosonic operators to spin operators.
//!
//! This mapping was developped by Juha Leppäkangas at HQS Quantum Simulations. The paper detailing
//! the mapping, as well as its use in the context of open system dynamics, can be found at:
//!                         <https://arxiv.org/pdf/2210.12138>
//!
//! The mapping is given by:
//!
//! $ \hat{b}_i^{dagger} \hat{b}_i \rightarrow \sum_{j=1}^{N} \hat{\sigma}_+^{i,j} \hat{\sigma}_-^{i,j} $
//! $ \hat{b}_i^{dagger} + \hat{b}_i \rightarrow \frac{1}{\root{N}} \sum_{j=1}^{N} \hat{\sigma}_x^{i,j} $
//!
//! For a direct mapping, N is set to 1. For a Dicke mapping, N > 1.

use crate::StruqtureError;

pub trait BosonToSpin {
    /// The Output type for the BosonToSpin transformation
    ///
    /// For a HermitianBosonProduct it will be a PauliOperator.
    /// For a BosonHamiltonian it will be a PauliOperator.
    type Output;

    /// Transforms the given bosonic object into a spin object using the direct mapping.
    ///
    /// In this mapping, 1 bosonic mode is described by 1 spin.
    ///
    /// # Returns
    ///
    /// * `Ok(output)` - The result of the mapping to a spin object.
    /// * `Err(StruqtureError)` - The boson -> spin transformation is only available for
    ///   terms such as b†b or (b† + b).
    fn direct_boson_spin_mapping(&self) -> Result<Self::Output, StruqtureError> {
        self.dicke_boson_spin_mapping(1)
    }

    /// Transforms the given bosonic object into a spin object using the Dicke mapping.
    ///
    /// In this mapping, 1 bosonic mode is described by N>1 spins.
    /// We work in the subspace spanned by symmetric Dicke states of spins.
    ///
    /// # Arguments
    ///
    /// * `number_spins_per_bosonic_mode` - The number of spins to represent each bosonic mode.
    ///
    /// # Returns
    ///
    /// * `Ok(output)` - The result of the mapping to a spin object.
    /// * `Err(StruqtureError)` - The boson -> spin transformation is only available for
    ///   terms such as b†b or (b† + b).
    fn dicke_boson_spin_mapping(
        &self,
        number_spins_per_bosonic_mode: usize,
    ) -> Result<Self::Output, StruqtureError>;
}
