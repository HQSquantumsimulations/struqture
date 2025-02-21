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

use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use struqture::fermions::{
    FermionHamiltonian, FermionLindbladNoiseOperator, FermionLindbladOpenSystem, FermionOperator,
    FermionProduct, HermitianFermionProduct,
};
use struqture::mappings::JordanWignerFermionToSpin;
use struqture::prelude::*;
use struqture::spins::{
    DecoherenceProduct, PauliProduct, PauliHamiltonian, PauliLindbladNoiseOperator,
    PauliLindbladOpenSystem, PauliOperator, SinglePauliOperator,
};

#[test]
fn test_jw_fermion_product() {
    let fp = FermionProduct::new([1], [2]).unwrap();
    let pp_1 = PauliProduct::new().y(1).x(2);
    let pp_2 = PauliProduct::new().x(1).y(2);
    let pp_3 = PauliProduct::new().y(1).y(2);
    let pp_4 = PauliProduct::new().x(1).x(2);
    let mut so = PauliOperator::new();
    so.add_operator_product(pp_1, CalculatorComplex::new(0.0, -0.25))
        .unwrap();
    so.add_operator_product(pp_2, CalculatorComplex::new(0.0, 0.25))
        .unwrap();
    so.add_operator_product(pp_3, CalculatorComplex::new(0.25, 0.0))
        .unwrap();
    so.add_operator_product(pp_4, CalculatorComplex::new(0.25, 0.0))
        .unwrap();

    assert_eq!(fp.jordan_wigner(), so);

    let fp = FermionProduct::new([], []).unwrap();
    let mut so = PauliOperator::new();
    let mut id = PauliProduct::new();
    id = id.set_pauli(0, SinglePauliOperator::Identity);
    so.add_operator_product(id, CalculatorComplex::new(1.0, 0.0))
        .unwrap();

    assert_eq!(fp.jordan_wigner(), so)
}

#[test]
fn test_jw_hermitian_fermion_product() {
    let hfp = HermitianFermionProduct::new([1], [2]).unwrap();
    let pp_1 = PauliProduct::new().y(1).y(2);
    let pp_2 = PauliProduct::new().x(1).x(2);
    let mut so = PauliHamiltonian::new();
    so.add_operator_product(pp_1, CalculatorFloat::from(0.5))
        .unwrap();
    so.add_operator_product(pp_2, CalculatorFloat::from(0.5))
        .unwrap();

    assert_eq!(hfp.jordan_wigner(), so);

    let hfp = HermitianFermionProduct::new([], []).unwrap();
    let mut so = PauliHamiltonian::new();
    let mut id = PauliProduct::new();
    id = id.set_pauli(0, SinglePauliOperator::Identity);
    so.add_operator_product(id, 1.0.into()).unwrap();

    assert_eq!(hfp.jordan_wigner(), so);
}

#[test]
fn test_jw_fermion_operator() {
    let mut fo = FermionOperator::new();
    let so = PauliOperator::new();

    assert_eq!(fo.jordan_wigner(), so);

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
fn test_jw_fermion_hamiltonian() {
    let mut fh = FermionHamiltonian::new();
    let hfp1 = HermitianFermionProduct::new([1, 2], [2, 4]).unwrap();
    let hfp2 = HermitianFermionProduct::new([1, 2], [1, 3]).unwrap();
    fh.add_operator_product(hfp1.clone(), CalculatorComplex::new(1.0, 0.0))
        .unwrap();
    fh.add_operator_product(hfp2.clone(), CalculatorComplex::new(2.0, 0.0))
        .unwrap();
    let jw_hfp1 = hfp1.jordan_wigner();
    let jw_hfp2 = hfp2.jordan_wigner();

    assert_eq!(
        fh.jordan_wigner(),
        jw_hfp1 * CalculatorFloat::from(1.0) + jw_hfp2 * CalculatorFloat::from(2.0)
    );

    let mut fh = FermionHamiltonian::new();
    let hfp = HermitianFermionProduct::new([], [1, 2]).unwrap();
    let coeff = CalculatorComplex::new(1.0, 2.0);
    fh.add_operator_product(hfp.clone(), coeff.clone()).unwrap();

    let mut so = PauliOperator::new();
    let xx = PauliProduct::new().x(1).x(2);
    let xy = PauliProduct::new().x(1).y(2);
    let yx = PauliProduct::new().y(1).x(2);
    let yy = PauliProduct::new().y(1).y(2);
    so.add_operator_product(xx, (coeff.re.clone() * (-1.0)).into())
        .unwrap();
    so.add_operator_product(xy, coeff.im.clone().into())
        .unwrap();
    so.add_operator_product(yx, coeff.im.into()).unwrap();
    so.add_operator_product(yy, coeff.re.into()).unwrap();

    let sh = PauliHamiltonian::try_from(so).unwrap() * CalculatorFloat::from(0.5);

    assert_eq!(sh, fh.jordan_wigner());

    let mut fh = FermionHamiltonian::new();
    let hfp = HermitianFermionProduct::new([2], [2]).unwrap();
    fh.add_operator_product(hfp.clone(), 3.0.into()).unwrap();

    let mut sh = PauliHamiltonian::new();
    let mut id = PauliProduct::new();
    id = id.set_pauli(2, SinglePauliOperator::Identity);
    sh.set(id, 1.5.into()).unwrap();
    sh.set(PauliProduct::new().z(2), (-1.5).into()).unwrap();

    assert_eq!(sh, fh.jordan_wigner());
}

#[test]
fn test_jw_fermion_noise_operator() {
    let mut fno = FermionLindbladNoiseOperator::new();
    let mut sno = PauliLindbladNoiseOperator::new();

    assert_eq!(fno.jordan_wigner(), sno);

    let fp = FermionProduct::new([0], [0]).unwrap();
    fno.add_operator_product((fp.clone(), fp), CalculatorComplex::new(1.0, 0.0))
        .unwrap();
    let dp = DecoherenceProduct::new().z(0);
    sno.add_operator_product((dp.clone(), dp), CalculatorComplex::new(0.25, 0.0))
        .unwrap();

    assert_eq!(fno.jordan_wigner(), sno);
}

#[test]
fn test_jw_fermion_systems_to_spin() {
    let mut fh = FermionHamiltonian::new();
    fh.add_operator_product(
        HermitianFermionProduct::new([1], [2]).unwrap(),
        CalculatorComplex::new(1.0, 2.0),
    )
    .unwrap();
    let sh = fh.jordan_wigner();

    let mut fno = FermionLindbladNoiseOperator::new();
    let fp1 = FermionProduct::new([1], [2]).unwrap();
    let fp2 = FermionProduct::new([2], [3]).unwrap();
    fno.add_operator_product((fp1, fp2), CalculatorComplex::new(1.0, 2.0))
        .unwrap();
    let sno = fno.jordan_wigner();

    // Test FermionLindbladOpenSystem
    let sos = PauliLindbladOpenSystem::group(sh, sno).unwrap();
    let fos = FermionLindbladOpenSystem::group(fh, fno).unwrap();

    assert_eq!(fos.jordan_wigner(), sos);
}
