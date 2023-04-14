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
    FermionSystem,
};
use struqture::mappings::JordanWignerSpinToFermion;
use struqture::prelude::*;
use struqture::spins::{
    DecoherenceProduct, PauliProduct, PlusMinusLindbladNoiseOperator, PlusMinusOperator,
    PlusMinusProduct, SpinHamiltonian, SpinHamiltonianSystem, SpinLindbladNoiseOperator,
    SpinLindbladNoiseSystem, SpinLindbladOpenSystem, SpinOperator, SpinSystem,
};

#[test]
fn test_jw_plusminus_product() {
    let mut pmp = PlusMinusProduct::new();
    let mut fo = FermionOperator::new();
    fo.add_operator_product(
        FermionProduct::new([], [])
            .expect("Internal bug in add_operator_product for FermionOperator."),
        1.0.into(),
    )
    .expect("Internal bug in FermionProduct::new");

    assert_eq!(pmp.jordan_wigner(), fo);

    fo = FermionOperator::new();
    pmp = pmp.plus(0).minus(1).z(2);
    let fp1 = FermionProduct::new([1], [0]).unwrap();
    let fp2 = FermionProduct::new([1, 2], [0, 2]).unwrap();
    fo.add_operator_product(fp1, 1.0.into()).unwrap();
    fo.add_operator_product(fp2, 2.0.into()).unwrap();

    assert_eq!(pmp.jordan_wigner(), fo);
}

#[test]
fn test_jw_plusminus_operator() {
    let mut pmo = PlusMinusOperator::new();
    let fo = FermionOperator::new();

    assert_eq!(pmo.jordan_wigner(), fo);

    let pmp1 = PlusMinusProduct::new().plus(1).minus(2).plus(3).z(4);
    let pmp2 = PlusMinusProduct::new().plus(0).plus(1).plus(2).z(3);
    let cc1 = CalculatorComplex::new(1.0, 2.0);
    let cc2 = CalculatorComplex::new(2.0, 1.0);
    pmo.add_operator_product(pmp1.clone(), cc1.clone()).unwrap();
    pmo.add_operator_product(pmp2.clone(), cc2.clone()).unwrap();
    let jw_pair1 = pmp1.jordan_wigner() * cc1;
    let jw_pair2 = pmp2.jordan_wigner() * cc2;

    assert_eq!(pmo.jordan_wigner(), jw_pair1 + jw_pair2);
}

#[test]
fn test_jw_plusminus_noise_operator() {
    let mut pmno = PlusMinusLindbladNoiseOperator::new();
    let mut fno = FermionLindbladNoiseOperator::new();

    assert_eq!(pmno.jordan_wigner(), fno);

    let pmp = PlusMinusProduct::new().plus(0);
    pmno.add_operator_product((pmp.clone(), pmp.clone()), CalculatorComplex::new(1.0, 0.0))
        .unwrap();
    let fp = FermionProduct::new([], [0]).unwrap();
    fno.add_operator_product((fp.clone(), fp.clone()), CalculatorComplex::new(1.0, 0.0))
        .unwrap();

    assert_eq!(pmno.jordan_wigner(), fno);
}

#[test]
fn test_jw_decoherence_product() {
    // TODO
}

#[test]
fn test_jw_pauli_product() {
    let mut pp = PauliProduct::new();
    let mut fo = FermionOperator::new();
    fo.add_operator_product(
        FermionProduct::new([], [])
            .expect("Internal bug in add_operator_product for FermionOperator."),
        1.0.into(),
    )
    .expect("Internal bug in FermionProduct::new");

    assert_eq!(pp.jordan_wigner(), fo);

    fo = FermionOperator::new();
    pp = pp.x(0).y(1).z(2);

    fo.add_operator_product(
        FermionProduct::new([], [0, 1]).unwrap(),
        CalculatorComplex::new(0.0, 1.0),
    )
    .unwrap();
    fo.add_operator_product(
        FermionProduct::new([1], [0]).unwrap(),
        CalculatorComplex::new(0.0, 1.0),
    )
    .unwrap();
    fo.add_operator_product(
        FermionProduct::new([0], [1]).unwrap(),
        CalculatorComplex::new(0.0, -1.0),
    )
    .unwrap();
    fo.add_operator_product(
        FermionProduct::new([0, 1], []).unwrap(),
        CalculatorComplex::new(0.0, 1.0),
    )
    .unwrap();
    fo.add_operator_product(
        FermionProduct::new([2], [0, 1, 2]).unwrap(),
        CalculatorComplex::new(0.0, -2.0),
    )
    .unwrap();
    fo.add_operator_product(
        FermionProduct::new([1, 2], [0, 2]).unwrap(),
        CalculatorComplex::new(0.0, 2.0),
    )
    .unwrap();
    fo.add_operator_product(
        FermionProduct::new([0, 2], [1, 2]).unwrap(),
        CalculatorComplex::new(0.0, -2.0),
    )
    .unwrap();
    fo.add_operator_product(
        FermionProduct::new([0, 1, 2], [2]).unwrap(),
        CalculatorComplex::new(0.0, -2.0),
    )
    .unwrap();

    assert_eq!(pp.jordan_wigner(), fo);
}

#[test]
fn test_jw_spin_operator() {
    let mut so = SpinOperator::new();
    let pp1 = PauliProduct::new().z(0).y(1).x(2);
    let pp2 = PauliProduct::new().y(1).x(2).z(3);
    let cc1 = CalculatorComplex::new(1.0, 2.0);
    let cc2 = CalculatorComplex::new(2.0, 1.0);
    so.add_operator_product(pp1.clone(), cc1.clone()).unwrap();
    so.add_operator_product(pp2.clone(), cc2.clone()).unwrap();
    let jw_pair1 = pp1.jordan_wigner() * cc1;
    let jw_pair2 = pp2.jordan_wigner() * cc2;

    assert_eq!(so.jordan_wigner(), jw_pair1 + jw_pair2);
}

#[test]
fn test_jw_spin_hamiltonian() {
    let mut sh = SpinHamiltonian::new();

    let pp1 = PauliProduct::new().z(0).y(1).x(2);
    let pp2 = PauliProduct::new().y(1).x(2).z(3);
    sh.add_operator_product(pp1.clone(), 1.0.into()).unwrap();
    sh.add_operator_product(pp2.clone(), 2.0.into()).unwrap();

    let jw_pp1 = pp1.jordan_wigner();
    let jw_pp2 = pp2.jordan_wigner();

    let filtered_jw_pp1 = FermionOperator::from_iter(jw_pp1.into_iter().filter(|x| {
        (*x).0.is_natural_hermitian() || (*x).0.creators().min() < (*x).0.annihilators().min()
    }));
    let filtered_jw_pp2 = FermionOperator::from_iter(jw_pp2.into_iter().filter(|x| {
        (*x).0.is_natural_hermitian() || (*x).0.creators().min() < (*x).0.annihilators().min()
    }));
    let jw_pp1_hamiltonian = FermionHamiltonian::try_from(filtered_jw_pp1).unwrap();
    let jw_pp2_hamiltonian = FermionHamiltonian::try_from(filtered_jw_pp2).unwrap();
    let res = (jw_pp1_hamiltonian * CalculatorFloat::from(1.0)
        + jw_pp2_hamiltonian * CalculatorFloat::from(2.0))
    .unwrap();

    assert_eq!(sh.jordan_wigner(), res);
}

#[test]
fn test_jw_spin_noise_operator() {
    let mut fno = FermionLindbladNoiseOperator::new();
    let mut sno = SpinLindbladNoiseOperator::new();

    assert_eq!(sno.jordan_wigner(), fno);

    let fp = FermionProduct::new([0], [0]).unwrap();
    fno.add_operator_product((fp.clone(), fp.clone()), CalculatorComplex::new(1.0, 0.0))
        .unwrap();
    let dp = DecoherenceProduct::new().z(0);
    sno.add_operator_product((dp.clone(), dp.clone()), CalculatorComplex::new(0.25, 0.0))
        .unwrap();

    assert_eq!(sno.jordan_wigner(), fno);
}

#[test]
fn test_jw_spin_systems() {
    // Test SpinSystem
    let mut so = SpinOperator::new();
    so.add_operator_product(PauliProduct::new().x(1), CalculatorComplex::new(1.0, 1.0))
        .unwrap();
    let ss = SpinSystem::from_operator(so.clone(), Some(5)).unwrap();
    let fo = so.jordan_wigner();
    let fs = FermionSystem::from_operator(fo, Some(5)).unwrap();

    assert_eq!(ss.jordan_wigner(), fs);

    // Test SpinHamiltonianSystem
    let mut sh = SpinHamiltonian::new();
    sh.add_operator_product(PauliProduct::new().x(1), 1.0.into())
        .unwrap();
    let shs = SpinHamiltonianSystem::from_hamiltonian(sh.clone(), Some(5)).unwrap();
    let fh = sh.jordan_wigner();
    let fhs = FermionHamiltonianSystem::from_hamiltonian(fh, Some(5)).unwrap();

    assert_eq!(shs.jordan_wigner(), fhs);

    // Test SpinLindbladNoiseSystem
    let mut sno = SpinLindbladNoiseOperator::new();
    let pp1 = DecoherenceProduct::new().x(1);
    let pp2 = DecoherenceProduct::new().iy(2);
    sno.add_operator_product((pp1, pp2), CalculatorComplex::new(1.0, 2.0))
        .unwrap();
    let sns = SpinLindbladNoiseSystem::from_operator(sno.clone(), Some(5)).unwrap();
    let fno = sno.jordan_wigner();
    let fns = FermionLindbladNoiseSystem::from_operator(fno, Some(5)).unwrap();

    assert_eq!(sns.jordan_wigner(), fns);

    // Test SpinLindbladOpenSystem

    let sos = SpinLindbladOpenSystem::group(shs, sns).unwrap();
    let fos = FermionLindbladOpenSystem::group(fhs, fns).unwrap();

    assert_eq!(sos.jordan_wigner(), fos);
}
