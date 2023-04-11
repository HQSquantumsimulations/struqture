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

use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use struqture::fermions::{
    FermionHamiltonian, FermionHamiltonianSystem, FermionLindbladNoiseOperator,
    FermionLindbladNoiseSystem, FermionLindbladOpenSystem, FermionOperator, FermionProduct,
    HermitianFermionProduct,
};
use struqture::mappings::JordanWignerFermionToSpin;
use struqture::prelude::*;
use struqture::spins::{
    DecoherenceProduct, PauliProduct, SingleSpinOperator, SpinHamiltonian, SpinHamiltonianSystem,
    SpinLindbladNoiseOperator, SpinLindbladNoiseSystem, SpinLindbladOpenSystem, SpinOperator,
    PlusMinusProduct, PlusMinusOperator, PlusMinusNoiseOperator,
};

#[test]
fn test_jw_plusminus_product() {

    let mut pmp = PlusMinusProduct::new();
    let mut fo = FermionOperator::new();

    assert_eq!(pmp.jordan_wigner(), fo);

    pmp = pmp.plus(0).minus(1).z(2);
    let fp1 = FermionProduct::new([1], [0]).unwrap();
    let fp2 = FermionProduct::new([1, 2], [0, 2]).unwrap();
    fo.add_operator_product(fp1, 1.0.into());
    fo.add_operator_product(fp2, 2.0.into());

    assert_eq!(pmp.jordan_wigner(), fo);

}

#[test]
fn test_jw_plusminus_operator() {

    let mut pmo = PlusMinusOperator::new();
    let mut fo = FermionOperator::new();

    assert_eq!(pmo.jordan_wigner, fo);

    let pmp1 = PlusMinusProduct::new().plus(1).minus(2).plus(3).z(4);
    let pmp2 = PlusMinusProduct::new().plus(0).plus(1).plus(2).z(3);
    let cc1 = CalculatorComplex::new(1.0, 2.0);
    let cc2 = CalculatorComplex::new(2.0, 1.0);
    pmo.add_operator_product(pmp1, cc1.clone());
    pmo.add_operator_product(pmp2, cc2.clone());
    let jw_pair1 = pmp1.jordan_wigner() * cc1;
    let jw_pair2 = pmp2.jordan_wigner() * cc2;

    assert_eq!(pmo.jordan_wigner, jw_pair1 + jw_pair2);
}

#[test]
fn test_jw_plusminus_noise_operator() {
}

#[test]
fn test_jw_pauli_product() {
    let mut pp = PauliProduct();
    let mut fo = FermionOperator::new();

    assert_eq!(pp.jordan_wigner, fo);

    pp = pp.x(1).y(2).z(3);
    
    fo.add_operator_product(
        FermionOperator::new([], [0, 1]).unwrap(),
        CalculatorComplex::new(0.0, 1.0),
    );
    fo.add_operator_product(
        FermionOperator::new([1], [0]).unwrap(),
        CalculatorComplex::new(0.0, 1.0),
    );
    fo.add_operator_product(
        FermionOperator::new([0], [1]).unwrap(),
        CalculatorComplex::new(0.0, -1.0),
    );
    fo.add_operator_product(
        FermionOperator::new([0, 1], []).unwrap(),
        CalculatorComplex::new(0.0, 1.0),
    );
    fo.add_operator_product(
        FermionOperator::new([2], [0, 1, 2]).unwrap(),
        CalculatorComplex::new(0.0, -2.0),
    );
    fo.add_operator_product(
        FermionOperator::new([1, 2], [0, 2]).unwrap(),
        CalculatorComplex::new(0.0, 2.0),
    );
    fo.add_operator_product(
        FermionOperator::new([0, 2], [1, 2]).unwrap(),
        CalculatorComplex::new(0.0, -2.0),
    );
    fo.add_operator_product(
        FermionOperator::new([0, 1, 2], [2]).unwrap(),
        CalculatorComplex::new(0.0, -2.0),
    );

    assert_eq!(pp.jordan_wigner(), fo);
}

#[test]
fn test_jw_spin_operator() {
}

#[test]
fn test_jw_spin_hamiltonian() {
}

#[test]
fn test_jw_spin_noise_operator() {
}

#[test]
fn test_jw_spin_open_system() {
}

#[test]
fn test_jw_spin_systems() {
}
