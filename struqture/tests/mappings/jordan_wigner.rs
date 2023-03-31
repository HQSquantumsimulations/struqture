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
    DecoherenceProduct, PauliProduct, SingleDecoherenceOperator, SingleSpinOperator,
    SpinHamiltonian, SpinHamiltonianSystem, SpinLindbladNoiseOperator, SpinLindbladNoiseSystem,
    SpinLindbladOpenSystem, SpinOperator,
};

#[test]
fn test_jw_fermion_product_to_spin() {
    let fp = FermionProduct::new([1], [2]).unwrap();
    let pp_1 = PauliProduct::new().y(1).x(2);
    let pp_2 = PauliProduct::new().x(1).y(2);
    let pp_3 = PauliProduct::new().y(1).y(2);
    let pp_4 = PauliProduct::new().x(1).x(2);
    let mut so = SpinOperator::new();
    so.add_operator_product(pp_1.clone(), CalculatorComplex::new(0.0, -0.25))
        .unwrap();
    so.add_operator_product(pp_2.clone(), CalculatorComplex::new(0.0, 0.25))
        .unwrap();
    so.add_operator_product(pp_3.clone(), CalculatorComplex::new(0.25, 0.0))
        .unwrap();
    so.add_operator_product(pp_4.clone(), CalculatorComplex::new(0.25, 0.0))
        .unwrap();

    assert_eq!(fp.jordan_wigner(), so);

    let fp = FermionProduct::new([], []).unwrap();
    let mut so = SpinOperator::new();
    let mut id = PauliProduct::new();
    id = id.set_pauli(0, SingleSpinOperator::Identity);
    so.add_operator_product(id.clone(), CalculatorComplex::new(1.0, 0.0))
        .unwrap();

    assert_eq!(fp.jordan_wigner(), so)
}

#[test]
fn test_jw_hermitian_fermion_product_to_spin() {
    let hfp = HermitianFermionProduct::new([1], [2]).unwrap();
    let pp_1 = PauliProduct::new().y(1).y(2);
    let pp_2 = PauliProduct::new().x(1).x(2);
    let mut so = SpinHamiltonian::new();
    so.add_operator_product(pp_1.clone(), CalculatorFloat::from(0.5))
        .unwrap();
    so.add_operator_product(pp_2.clone(), CalculatorFloat::from(0.5))
        .unwrap();

    assert_eq!(hfp.jordan_wigner(), so)
}

#[test]
fn test_jw_fermion_operator_to_spin() {
    let mut fo = FermionOperator::new();
    let fp1 = FermionProduct::new([1, 2], [2, 3]).unwrap();
    let fp2 = FermionProduct::new([3, 4], [2, 5]).unwrap();
    fo.add_operator_product(fp1.clone(), CalculatorComplex::new(1.0, 2.0))
        .unwrap();
    fo.add_operator_product(fp2.clone(), CalculatorComplex::new(2.0, 1.0))
        .unwrap();
    let jw_pair1 = fp1.jordan_wigner() * CalculatorComplex::new(1.0, 2.0);
    let jw_pair2 = fp2.jordan_wigner() * CalculatorComplex::new(2.0, 1.0);

    assert_eq!(fo.jordan_wigner(), jw_pair1 + jw_pair2);
}

#[test]
fn test_jw_fermion_hamiltonian_to_spin() {
    let mut fh = FermionHamiltonian::new();
    let hfp1 = HermitianFermionProduct::new([1, 2], [2, 4]).unwrap();
    let hfp2 = HermitianFermionProduct::new([1, 3], [1, 2]).unwrap();
    fh.add_operator_product(hfp1.clone(), CalculatorComplex::new(1.0, 2.0))
        .unwrap();
    fh.add_operator_product(hfp2.clone(), CalculatorComplex::new(2.0, 1.0))
        .unwrap();
    let jw_pair1 = hfp1.jordan_wigner();
    let jw_pair2 = hfp2.jordan_wigner();
    let jw_pair1_hamiltonian = SpinHamiltonian::try_from(jw_pair1).unwrap();
    let jw_pair2_hamiltonian = SpinHamiltonian::try_from(jw_pair2).unwrap();

    assert_eq!(
        fh.jordan_wigner(),
        jw_pair1_hamiltonian * CalculatorFloat::from(1.0)
            + jw_pair2_hamiltonian * CalculatorFloat::from(2.0)
    );
}

#[test]
fn test_jw_fermion_noise_operator_to_spin() {
    let mut fno = FermionLindbladNoiseOperator::new();
    let fp = FermionProduct::new([0], [0]).unwrap();
    fno.add_operator_product((fp.clone(), fp.clone()), CalculatorComplex::new(1.0, 0.0))
        .unwrap();
    let mut sno = SpinLindbladNoiseOperator::new();
    let dp = DecoherenceProduct::new().z(0);
    sno.add_operator_product((dp.clone(), dp.clone()), CalculatorComplex::new(0.25, 0.0))
        .unwrap();

    // TEMP adding extra terms that would cancel, because the implementation of
    // SpinLindbladNoiseOperator still saves them
    let id = DecoherenceProduct::new().set_pauli(0, SingleDecoherenceOperator::Identity);
    sno.add_operator_product((id.clone(), id.clone()), CalculatorComplex::new(0.25, 0.0))
        .unwrap();
    sno.add_operator_product((id.clone(), dp.clone()), CalculatorComplex::new(-0.25, 0.0))
        .unwrap();
    sno.add_operator_product((dp.clone(), id.clone()), CalculatorComplex::new(-0.25, 0.0))
        .unwrap();

    assert_eq!(fno.jordan_wigner(), sno)
}

#[test]
fn test_jw_fermion_systems_to_spin() {
    // Test FermionHamiltonianSystem
    let mut fh = FermionHamiltonian::new();
    fh.add_operator_product(
        HermitianFermionProduct::new([1], [2]).unwrap(),
        CalculatorComplex::new(1.0, 2.0),
    )
    .unwrap();
    let fhs = FermionHamiltonianSystem::from_hamiltonian(fh.clone(), Some(5)).unwrap();
    let sh = fh.jordan_wigner();
    let shs = SpinHamiltonianSystem::from_hamiltonian(sh, Some(5)).unwrap();

    assert_eq!(fhs.jordan_wigner(), shs);

    // Test FermionLindbladNoiseSystem
    let mut fno = FermionLindbladNoiseOperator::new();
    let fp1 = FermionProduct::new([1], [2]).unwrap();
    let fp2 = FermionProduct::new([2], [3]).unwrap();
    fno.add_operator_product((fp1, fp2), CalculatorComplex::new(1.0, 2.0))
        .unwrap();
    let fns = FermionLindbladNoiseSystem::from_operator(fno.clone(), Some(5)).unwrap();
    let sno = fno.jordan_wigner();
    let sns = SpinLindbladNoiseSystem::from_operator(sno, Some(5)).unwrap();

    assert_eq!(fns.jordan_wigner(), sns);

    // Test FermionLindbladOpenSystem
    let sos = SpinLindbladOpenSystem::group(shs, sns).unwrap();
    let fos = FermionLindbladOpenSystem::group(fhs, fns).unwrap();

    assert_eq!(fos.jordan_wigner(), sos);
}
