// Copyright © 2020-2022 HQS Quantum Simulations GmbH. All Rights Reserved.
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
//! $$ JW(a_p^{\dagger}) = \left[ \prod_{i = 1}^{p - 1} Z_i \right] \left( \frac{X_p - i Y_p}{2}
//! \right) $$
//! 
//! $$ JW(a_p) = \left[ \prod_{i = 1}^{p - 1} Z_i \right] \left( \frac{X_p + i Y_p}{2} \right) $$


use crate::OperateOnDensityMatrix;
use crate::fermions::{
    FermionProduct,
    HermitianFermionProduct,
    FermionOperator,
    FermionHamiltonian,
    FermionLindbladOpenSystem,
};
use crate::spins::{
    PauliProduct,
    SpinOperator,
    SpinHamiltonian,
    SpinLindbladOpenSystem,
    SingleSpinOperator,
};
use crate::prelude::*;
use qoqo_calculator::CalculatorComplex;

pub trait JordanWignerFermionToSpin{

    /// The Output type for the JordanWigner transformation
    ///
    /// For a FermionProduct, HermitianFermionProduct or FermionOperator it will be a SpinOperator
    /// For a FermionHamiltonian it will be a SpinHamiltonian
    /// For a FermionLindbladOpenSystem it will be a SpinLindbladOpenSystem etc.
    type Output;

    /// Transform the given fermionic object into a spin object using
    /// the Jordan Wigner mapping.
    fn jordan_wigner(&self) -> Self::Output;

}

pub trait JordanWignerSpinToFermion{

    /// The Output type for the JordanWigner transformation
    ///
    /// TODO missing PauliOperator
    /// For a SpinOperator it will be a FermionOperator
    /// For a SpinHamiltonian it will be a FermionHamiltonian
    /// For a SpinLindbladOpenSystem it will be a FermionLindbladOpenSystem
    /// etc.
    type Output;

    /// Transform the given fermionic object into a spin object using
    /// the Jordan Wigner mapping.
    fn jordan_wigner(self) -> Self::Output;

}

