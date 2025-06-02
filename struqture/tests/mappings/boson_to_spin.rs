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

use qoqo_calculator::CalculatorComplex;
use struqture::bosons::{
    BosonHamiltonian, BosonLindbladNoiseOperator, BosonProduct, HermitianBosonProduct,
};
use struqture::mappings::BosonToSpin;
use struqture::prelude::*;
use struqture::spins::{
    DecoherenceProduct, PauliLindbladNoiseOperator, PauliOperator, PauliProduct,
};

#[test]
fn test_hermitian_boson_product_to_spin_annihilator_only_simple() {
    let bp = HermitianBosonProduct::new([], [0]).unwrap();
    let pp_1 = PauliProduct::new().x(0);
    let mut so = PauliOperator::new();
    so.add_operator_product(pp_1.clone(), CalculatorComplex::new(1.0, 0.0))
        .unwrap();

    assert_eq!(bp.dicke_boson_spin_mapping(1).unwrap(), so);
    assert_eq!(bp.direct_boson_spin_mapping().unwrap(), so);
}

#[test]
fn test_hermitian_boson_product_to_spin_annihilator_only() {
    let bp = HermitianBosonProduct::new([], [1]).unwrap();
    let pp_1 = PauliProduct::new().x(4);
    let pp_2 = PauliProduct::new().x(5);
    let pp_3 = PauliProduct::new().x(6);
    let pp_4 = PauliProduct::new().x(7);
    let mut so = PauliOperator::new();
    so.add_operator_product(pp_1.clone(), CalculatorComplex::new(0.5, 0.0))
        .unwrap();
    so.add_operator_product(pp_2.clone(), CalculatorComplex::new(0.5, 0.0))
        .unwrap();
    so.add_operator_product(pp_3.clone(), CalculatorComplex::new(0.5, 0.0))
        .unwrap();
    so.add_operator_product(pp_4.clone(), CalculatorComplex::new(0.5, 0.0))
        .unwrap();

    assert_eq!(bp.dicke_boson_spin_mapping(4).unwrap(), so);
}

#[test]
fn test_hermitian_boson_product_to_spin_simple() {
    let bp = HermitianBosonProduct::new([0], [0]).unwrap();
    let pp_1 = PauliProduct::new().z(0);
    let mut so = PauliOperator::new();
    so.add_operator_product(pp_1.clone(), CalculatorComplex::new(0.5, 0.0))
        .unwrap();

    assert_eq!(bp.dicke_boson_spin_mapping(1).unwrap(), so);
    assert_eq!(bp.direct_boson_spin_mapping().unwrap(), so);
}

#[test]
fn test_hermitian_boson_product_to_spin() {
    let bp = HermitianBosonProduct::new([1], [1]).unwrap();
    let pp_1 = PauliProduct::new().z(3);
    let pp_2 = PauliProduct::new().z(4);
    let pp_3 = PauliProduct::new().z(5);
    let mut so = PauliOperator::new();
    so.add_operator_product(pp_1.clone(), CalculatorComplex::new(0.5, 0.0))
        .unwrap();
    so.add_operator_product(pp_2.clone(), CalculatorComplex::new(0.5, 0.0))
        .unwrap();
    so.add_operator_product(pp_3.clone(), CalculatorComplex::new(0.5, 0.0))
        .unwrap();

    assert_eq!(bp.dicke_boson_spin_mapping(3).unwrap(), so);
}

#[test]
fn test_hermitian_boson_product_to_spin_error() {
    let bp = HermitianBosonProduct::new([0], [1]).unwrap();
    assert_eq!(bp.dicke_boson_spin_mapping(1), Err(struqture::StruqtureError::GenericError{ msg: "The boson -> spin transformation is only available for terms such as b†b or (b† + b), but the term here is: c0a1".into() }));
    assert_eq!(bp.direct_boson_spin_mapping(), Err(struqture::StruqtureError::GenericError{ msg: "The boson -> spin transformation is only available for terms such as b†b or (b† + b), but the term here is: c0a1".into() }));
    let bp = HermitianBosonProduct::new([0, 1], [0, 1]).unwrap();
    assert_eq!(bp.dicke_boson_spin_mapping(1), Err(struqture::StruqtureError::GenericError{ msg: "The boson -> spin transformation is only available for terms such as b†b or (b† + b), but the term here is: c0c1a0a1".into() }));
    assert_eq!(bp.direct_boson_spin_mapping(), Err(struqture::StruqtureError::GenericError{ msg: "The boson -> spin transformation is only available for terms such as b†b or (b† + b), but the term here is: c0c1a0a1".into() }));
}

#[test]
fn test_boson_product_to_spin_annihilator_only_simple() {
    let bp = BosonProduct::new([], [0]).unwrap();
    let pp_1 = PauliProduct::new().x(0);
    let mut so = PauliOperator::new();
    so.add_operator_product(pp_1.clone(), CalculatorComplex::new(1.0, 0.0))
        .unwrap();

    assert_eq!(bp.dicke_boson_spin_mapping(1).unwrap(), so);
    assert_eq!(bp.direct_boson_spin_mapping().unwrap(), so);
}

#[test]
fn test_boson_product_to_spin_annihilator_only() {
    let bp = BosonProduct::new([], [1]).unwrap();
    let pp_1 = PauliProduct::new().x(4);
    let pp_2 = PauliProduct::new().x(5);
    let pp_3 = PauliProduct::new().x(6);
    let pp_4 = PauliProduct::new().x(7);
    let mut so = PauliOperator::new();
    so.add_operator_product(pp_1.clone(), CalculatorComplex::new(0.5, 0.0))
        .unwrap();
    so.add_operator_product(pp_2.clone(), CalculatorComplex::new(0.5, 0.0))
        .unwrap();
    so.add_operator_product(pp_3.clone(), CalculatorComplex::new(0.5, 0.0))
        .unwrap();
    so.add_operator_product(pp_4.clone(), CalculatorComplex::new(0.5, 0.0))
        .unwrap();

    assert_eq!(bp.dicke_boson_spin_mapping(4).unwrap(), so);
}

#[test]
fn test_boson_product_to_spin_simple() {
    let bp = BosonProduct::new([0], [0]).unwrap();
    let pp_1 = PauliProduct::new().z(0);
    let mut so = PauliOperator::new();
    so.add_operator_product(pp_1.clone(), CalculatorComplex::new(0.5, 0.0))
        .unwrap();

    assert_eq!(bp.dicke_boson_spin_mapping(1).unwrap(), so);
    assert_eq!(bp.direct_boson_spin_mapping().unwrap(), so);
}

#[test]
fn test_boson_product_to_spin() {
    let bp = BosonProduct::new([1], [1]).unwrap();
    let pp_1 = PauliProduct::new().z(3);
    let pp_2 = PauliProduct::new().z(4);
    let pp_3 = PauliProduct::new().z(5);
    let mut so = PauliOperator::new();
    so.add_operator_product(pp_1.clone(), CalculatorComplex::new(0.5, 0.0))
        .unwrap();
    so.add_operator_product(pp_2.clone(), CalculatorComplex::new(0.5, 0.0))
        .unwrap();
    so.add_operator_product(pp_3.clone(), CalculatorComplex::new(0.5, 0.0))
        .unwrap();

    assert_eq!(bp.dicke_boson_spin_mapping(3).unwrap(), so);
}

#[test]
fn test_boson_product_to_spin_error() {
    let bp = BosonProduct::new([0], [1]).unwrap();
    assert_eq!(bp.dicke_boson_spin_mapping(1), Err(struqture::StruqtureError::GenericError{ msg: "The boson -> spin transformation is only available for terms such as b†b or (b† + b), but the term here is: c0a1".into() }));
    assert_eq!(bp.direct_boson_spin_mapping(), Err(struqture::StruqtureError::GenericError{ msg: "The boson -> spin transformation is only available for terms such as b†b or (b† + b), but the term here is: c0a1".into() }));
    let bp = BosonProduct::new([0, 1], [0, 1]).unwrap();
    assert_eq!(bp.dicke_boson_spin_mapping(1), Err(struqture::StruqtureError::GenericError{ msg: "The boson -> spin transformation is only available for terms such as b†b or (b† + b), but the term here is: c0c1a0a1".into() }));
    assert_eq!(bp.direct_boson_spin_mapping(), Err(struqture::StruqtureError::GenericError{ msg: "The boson -> spin transformation is only available for terms such as b†b or (b† + b), but the term here is: c0c1a0a1".into() }));
}

#[test]
fn test_boson_hamiltonian_to_spin_annihilator_only_simple() {
    let bp = HermitianBosonProduct::new([], [0]).unwrap();
    let mut bo = BosonHamiltonian::new();
    bo.add_operator_product(bp, 0.3.into()).unwrap();
    let pp_1 = PauliProduct::new().x(0);
    let mut so = PauliOperator::new();
    so.add_operator_product(pp_1.clone(), CalculatorComplex::new(0.3, 0.0))
        .unwrap();

    assert_eq!(bo.dicke_boson_spin_mapping(1).unwrap(), so);
    assert_eq!(bo.direct_boson_spin_mapping().unwrap(), so);
}

#[test]
fn test_boson_hamiltonian_to_spin_annihilator_only() {
    let bp_1 = HermitianBosonProduct::new([], [1]).unwrap();
    let bp_2 = HermitianBosonProduct::new([], [3]).unwrap();
    let bp_3 = HermitianBosonProduct::new([], [4]).unwrap();
    let mut bo = BosonHamiltonian::new();
    bo.add_operator_product(bp_1, 2.0.into()).unwrap();
    bo.add_operator_product(bp_2, 0.5.into()).unwrap();
    bo.add_operator_product(bp_3, 0.1.into()).unwrap();

    let pp_1 = PauliProduct::new().x(2);
    let pp_2 = PauliProduct::new().x(3);
    let pp_3 = PauliProduct::new().x(6);
    let pp_4 = PauliProduct::new().x(7);
    let pp_5 = PauliProduct::new().x(8);
    let pp_6 = PauliProduct::new().x(9);
    let mut so = PauliOperator::new();
    so.add_operator_product(pp_1.clone(), CalculatorComplex::new(2.0, 0.0))
        .unwrap();
    so.add_operator_product(pp_2.clone(), CalculatorComplex::new(2.0, 0.0))
        .unwrap();
    so.add_operator_product(pp_3.clone(), CalculatorComplex::new(0.5, 0.0))
        .unwrap();
    so.add_operator_product(pp_4.clone(), CalculatorComplex::new(0.5, 0.0))
        .unwrap();
    so.add_operator_product(pp_5.clone(), CalculatorComplex::new(0.1, 0.0))
        .unwrap();
    so.add_operator_product(pp_6.clone(), CalculatorComplex::new(0.1, 0.0))
        .unwrap();
    so = so * (1.0 / 2.0_f64.sqrt());

    assert_eq!(bo.dicke_boson_spin_mapping(2).unwrap(), so);
}

#[test]
fn test_boson_hamiltonian_to_spin_simple() {
    let bp = HermitianBosonProduct::new([0], [0]).unwrap();
    let mut bo = BosonHamiltonian::new();
    bo.add_operator_product(bp, 0.3.into()).unwrap();

    let pp_1 = PauliProduct::new().z(0);
    let mut so = PauliOperator::new();
    so.add_operator_product(pp_1.clone(), CalculatorComplex::new(0.15, 0.0))
        .unwrap();

    assert_eq!(bo.dicke_boson_spin_mapping(1).unwrap(), so);
    assert_eq!(bo.direct_boson_spin_mapping().unwrap(), so);
}

#[test]
fn test_boson_hamiltonian_to_spin() {
    let bp_1 = HermitianBosonProduct::new([1], [1]).unwrap();
    let bp_2 = HermitianBosonProduct::new([2], [2]).unwrap();
    let mut bo = BosonHamiltonian::new();
    bo.add_operator_product(bp_1, 3.0.into()).unwrap();
    bo.add_operator_product(bp_2, 0.1.into()).unwrap();

    let pp_1 = PauliProduct::new().z(3);
    let pp_2 = PauliProduct::new().z(4);
    let pp_3 = PauliProduct::new().z(5);
    let pp_4 = PauliProduct::new().z(6);
    let pp_5 = PauliProduct::new().z(7);
    let pp_6 = PauliProduct::new().z(8);
    let mut so = PauliOperator::new();
    so.add_operator_product(pp_1.clone(), CalculatorComplex::new(1.5, 0.0))
        .unwrap();
    so.add_operator_product(pp_2.clone(), CalculatorComplex::new(1.5, 0.0))
        .unwrap();
    so.add_operator_product(pp_3.clone(), CalculatorComplex::new(1.5, 0.0))
        .unwrap();
    so.add_operator_product(pp_4.clone(), CalculatorComplex::new(0.05, 0.0))
        .unwrap();
    so.add_operator_product(pp_5.clone(), CalculatorComplex::new(0.05, 0.0))
        .unwrap();
    so.add_operator_product(pp_6.clone(), CalculatorComplex::new(0.05, 0.0))
        .unwrap();

    assert_eq!(bo.dicke_boson_spin_mapping(3).unwrap(), so);
}

#[test]
fn test_boson_hamiltonian_to_spin_all_terms() {
    let bp_1 = HermitianBosonProduct::new([1], [1]).unwrap();
    let bp_2 = HermitianBosonProduct::new([2], [2]).unwrap();
    let bp_3 = HermitianBosonProduct::new([], [0]).unwrap();
    let bp_4 = HermitianBosonProduct::new([], [1]).unwrap();
    let mut bo = BosonHamiltonian::new();
    bo.add_operator_product(bp_1, 3.0.into()).unwrap();
    bo.add_operator_product(bp_2, 0.1.into()).unwrap();
    bo.add_operator_product(bp_3, 1.0.into()).unwrap();
    bo.add_operator_product(bp_4, 0.5.into()).unwrap();

    let mut so = PauliOperator::new();
    let pp_1 = PauliProduct::new().x(0);
    let pp_2 = PauliProduct::new().x(1);
    let pp_3 = PauliProduct::new().x(2);
    let pp_4 = PauliProduct::new().x(3);
    let pp_5 = PauliProduct::new().x(4);
    let pp_6 = PauliProduct::new().x(5);
    so.add_operator_product(pp_1.clone(), CalculatorComplex::new(1.0, 0.0))
        .unwrap();
    so.add_operator_product(pp_2.clone(), CalculatorComplex::new(1.0, 0.0))
        .unwrap();
    so.add_operator_product(pp_3.clone(), CalculatorComplex::new(1.0, 0.0))
        .unwrap();
    so.add_operator_product(pp_4.clone(), CalculatorComplex::new(0.5, 0.0))
        .unwrap();
    so.add_operator_product(pp_5.clone(), CalculatorComplex::new(0.5, 0.0))
        .unwrap();
    so.add_operator_product(pp_6.clone(), CalculatorComplex::new(0.5, 0.0))
        .unwrap();
    so = so * (1.0 / 3.0_f64.sqrt());
    let pp_1 = PauliProduct::new().z(3);
    let pp_2 = PauliProduct::new().z(4);
    let pp_3 = PauliProduct::new().z(5);
    let pp_4 = PauliProduct::new().z(6);
    let pp_5 = PauliProduct::new().z(7);
    let pp_6 = PauliProduct::new().z(8);
    so.add_operator_product(pp_1.clone(), CalculatorComplex::new(1.5, 0.0))
        .unwrap();
    so.add_operator_product(pp_2.clone(), CalculatorComplex::new(1.5, 0.0))
        .unwrap();
    so.add_operator_product(pp_3.clone(), CalculatorComplex::new(1.5, 0.0))
        .unwrap();
    so.add_operator_product(pp_4.clone(), CalculatorComplex::new(0.05, 0.0))
        .unwrap();
    so.add_operator_product(pp_5.clone(), CalculatorComplex::new(0.05, 0.0))
        .unwrap();
    so.add_operator_product(pp_6.clone(), CalculatorComplex::new(0.05, 0.0))
        .unwrap();

    assert_eq!(bo.dicke_boson_spin_mapping(3).unwrap(), so);
}

#[test]
fn test_boson_hamiltonian_to_spin_error() {
    let bp_1 = HermitianBosonProduct::new([0], [1]).unwrap();
    let bp_2 = HermitianBosonProduct::new([], [0]).unwrap();
    let bp_3 = HermitianBosonProduct::new([0], [0]).unwrap();
    let mut bo = BosonHamiltonian::new();
    bo.add_operator_product(bp_1, 1.0.into()).unwrap();
    bo.add_operator_product(bp_2.clone(), 1.0.into()).unwrap();
    bo.add_operator_product(bp_3.clone(), 1.0.into()).unwrap();

    assert_eq!(bo.dicke_boson_spin_mapping(1), Err(struqture::StruqtureError::GenericError{ msg: "The boson -> spin transformation is only available for terms such as b†b or (b† + b), but the term here is: c0a1".into() }));
    assert_eq!(bo.direct_boson_spin_mapping(), Err(struqture::StruqtureError::GenericError{ msg: "The boson -> spin transformation is only available for terms such as b†b or (b† + b), but the term here is: c0a1".into() }));

    let bp_1 = HermitianBosonProduct::new([0, 1], [0, 1]).unwrap();
    let mut bo = BosonHamiltonian::new();
    bo.add_operator_product(bp_1, 1.0.into()).unwrap();
    bo.add_operator_product(bp_2, 1.0.into()).unwrap();
    bo.add_operator_product(bp_3, 1.0.into()).unwrap();
    assert_eq!(bo.dicke_boson_spin_mapping(1), Err(struqture::StruqtureError::GenericError{ msg: "The boson -> spin transformation is only available for terms such as b†b or (b† + b), but the term here is: c0c1a0a1".into() }));
    assert_eq!(bo.direct_boson_spin_mapping(), Err(struqture::StruqtureError::GenericError{ msg: "The boson -> spin transformation is only available for terms such as b†b or (b† + b), but the term here is: c0c1a0a1".into() }));
}

#[test]
fn test_boson_noise_operator_to_spin_annihilator_only_simple() {
    let bp = BosonProduct::new([], [0]).unwrap();
    let mut bo = BosonLindbladNoiseOperator::new();
    bo.add_operator_product((bp.clone(), bp), 0.3.into())
        .unwrap();
    let pp_1 = DecoherenceProduct::new().x(0);
    let mut so = PauliLindbladNoiseOperator::new();
    so.add_operator_product((pp_1.clone(), pp_1), CalculatorComplex::new(0.3, 0.0))
        .unwrap();

    assert_eq!(bo.dicke_boson_spin_mapping(1).unwrap(), so);
    assert_eq!(bo.direct_boson_spin_mapping().unwrap(), so);
}

#[test]
fn test_boson_noise_operator_to_spin_annihilator_only() {
    let bp_1 = BosonProduct::new([], [1]).unwrap();
    let bp_2 = BosonProduct::new([], [3]).unwrap();
    let bp_3 = BosonProduct::new([], [4]).unwrap();
    let mut bo = BosonLindbladNoiseOperator::new();
    bo.add_operator_product((bp_1.clone(), bp_1), 2.0.into())
        .unwrap();
    bo.add_operator_product((bp_2.clone(), bp_2), 0.5.into())
        .unwrap();
    bo.add_operator_product((bp_3.clone(), bp_3), 0.1.into())
        .unwrap();

    let pp_1 = DecoherenceProduct::new().x(2);
    let pp_2 = DecoherenceProduct::new().x(3);
    let pp_3 = DecoherenceProduct::new().x(6);
    let pp_4 = DecoherenceProduct::new().x(7);
    let pp_5 = DecoherenceProduct::new().x(8);
    let pp_6 = DecoherenceProduct::new().x(9);
    let mut so = PauliLindbladNoiseOperator::new();
    so.add_operator_product(
        (pp_1.clone(), pp_1.clone()),
        CalculatorComplex::new(2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_1.clone(), pp_2.clone()),
        CalculatorComplex::new(2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_2.clone(), pp_1.clone()),
        CalculatorComplex::new(2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_2.clone(), pp_2.clone()),
        CalculatorComplex::new(2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_3.clone(), pp_3.clone()),
        CalculatorComplex::new(0.5, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_3.clone(), pp_4.clone()),
        CalculatorComplex::new(0.5, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_4.clone(), pp_3.clone()),
        CalculatorComplex::new(0.5, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_4.clone(), pp_4.clone()),
        CalculatorComplex::new(0.5, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_5.clone(), pp_5.clone()),
        CalculatorComplex::new(0.1, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_5.clone(), pp_6.clone()),
        CalculatorComplex::new(0.1, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_6.clone(), pp_5.clone()),
        CalculatorComplex::new(0.1, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_6.clone(), pp_6.clone()),
        CalculatorComplex::new(0.1, 0.0),
    )
    .unwrap();
    so = so * (1.0 / 2.0_f64.sqrt()) * (1.0 / 2.0_f64.sqrt());

    let res = bo.dicke_boson_spin_mapping(2).unwrap();
    let keys_res: Vec<&(DecoherenceProduct, DecoherenceProduct)> = res.keys().collect();
    let keys_so: Vec<&(DecoherenceProduct, DecoherenceProduct)> = so.keys().collect();
    assert_eq!(keys_res, keys_so);
    for key in res.keys() {
        assert!((res.get(key).clone() - so.get(key)).abs().float().unwrap() < &0.0000001_f64);
    }
}

#[test]
fn test_boson_noise_operator_to_spin_simple() {
    let bp = BosonProduct::new([0], [0]).unwrap();
    let mut bo = BosonLindbladNoiseOperator::new();
    bo.add_operator_product((bp.clone(), bp), 0.3.into())
        .unwrap();

    let pp_1 = DecoherenceProduct::new().z(0);
    let mut so = PauliLindbladNoiseOperator::new();
    so.add_operator_product(
        (pp_1.clone(), pp_1),
        CalculatorComplex::new(0.15 / 2.0, 0.0),
    )
    .unwrap();

    assert_eq!(bo.dicke_boson_spin_mapping(1).unwrap(), so);
    assert_eq!(bo.direct_boson_spin_mapping().unwrap(), so);
}

#[test]
fn test_boson_noise_operator_to_spin() {
    let bp_1 = BosonProduct::new([0], [0]).unwrap();
    let bp_2 = BosonProduct::new([2], [2]).unwrap();
    let mut bo = BosonLindbladNoiseOperator::new();
    bo.add_operator_product((bp_1.clone(), bp_1), 3.0.into())
        .unwrap();
    bo.add_operator_product((bp_2.clone(), bp_2), 1.0.into())
        .unwrap();

    let pp_1 = DecoherenceProduct::new().z(0);
    let pp_2 = DecoherenceProduct::new().z(1);
    let pp_3 = DecoherenceProduct::new().z(2);
    let pp_4 = DecoherenceProduct::new().z(6);
    let pp_5 = DecoherenceProduct::new().z(7);
    let pp_6 = DecoherenceProduct::new().z(8);
    let mut so = PauliLindbladNoiseOperator::new();
    so.add_operator_product(
        (pp_1.clone(), pp_1.clone()),
        CalculatorComplex::new(1.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_1.clone(), pp_2.clone()),
        CalculatorComplex::new(1.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_1.clone(), pp_3.clone()),
        CalculatorComplex::new(1.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_2.clone(), pp_1.clone()),
        CalculatorComplex::new(1.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_2.clone(), pp_2.clone()),
        CalculatorComplex::new(1.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_2.clone(), pp_3.clone()),
        CalculatorComplex::new(1.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_3.clone(), pp_1.clone()),
        CalculatorComplex::new(1.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_3.clone(), pp_2.clone()),
        CalculatorComplex::new(1.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_3.clone(), pp_3.clone()),
        CalculatorComplex::new(1.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_4.clone(), pp_4.clone()),
        CalculatorComplex::new(0.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_4.clone(), pp_5.clone()),
        CalculatorComplex::new(0.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_4.clone(), pp_6.clone()),
        CalculatorComplex::new(0.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_5.clone(), pp_4.clone()),
        CalculatorComplex::new(0.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_5.clone(), pp_5.clone()),
        CalculatorComplex::new(0.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_5.clone(), pp_6.clone()),
        CalculatorComplex::new(0.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_6.clone(), pp_4.clone()),
        CalculatorComplex::new(0.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_6.clone(), pp_5.clone()),
        CalculatorComplex::new(0.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_6.clone(), pp_6.clone()),
        CalculatorComplex::new(0.5 / 2.0, 0.0),
    )
    .unwrap();

    assert_eq!(bo.dicke_boson_spin_mapping(3).unwrap(), so);
}

#[test]
fn test_boson_noise_operator_to_spin_all_terms() {
    let bp_1 = BosonProduct::new([0], [0]).unwrap();
    let bp_2 = BosonProduct::new([2], [2]).unwrap();
    let bp_3 = BosonProduct::new([], [1]).unwrap();
    let bp_4 = BosonProduct::new([], [3]).unwrap();
    let mut bo = BosonLindbladNoiseOperator::new();
    bo.add_operator_product((bp_1.clone(), bp_1), 3.0.into())
        .unwrap();
    bo.add_operator_product((bp_2.clone(), bp_2), 1.0.into())
        .unwrap();
    bo.add_operator_product((bp_3.clone(), bp_3), 2.0.into())
        .unwrap();
    bo.add_operator_product((bp_4.clone(), bp_4), 0.5.into())
        .unwrap();

    let mut so = PauliLindbladNoiseOperator::new();
    let pp_1 = DecoherenceProduct::new().x(3);
    let pp_2 = DecoherenceProduct::new().x(4);
    let pp_3 = DecoherenceProduct::new().x(5);
    let pp_4 = DecoherenceProduct::new().x(9);
    let pp_5 = DecoherenceProduct::new().x(10);
    let pp_6 = DecoherenceProduct::new().x(11);
    so.add_operator_product(
        (pp_1.clone(), pp_1.clone()),
        CalculatorComplex::new(2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_1.clone(), pp_2.clone()),
        CalculatorComplex::new(2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_1.clone(), pp_3.clone()),
        CalculatorComplex::new(2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_2.clone(), pp_1.clone()),
        CalculatorComplex::new(2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_2.clone(), pp_2.clone()),
        CalculatorComplex::new(2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_2.clone(), pp_3.clone()),
        CalculatorComplex::new(2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_3.clone(), pp_1.clone()),
        CalculatorComplex::new(2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_3.clone(), pp_2.clone()),
        CalculatorComplex::new(2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_3.clone(), pp_3.clone()),
        CalculatorComplex::new(2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_4.clone(), pp_4.clone()),
        CalculatorComplex::new(0.5, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_4.clone(), pp_5.clone()),
        CalculatorComplex::new(0.5, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_4.clone(), pp_6.clone()),
        CalculatorComplex::new(0.5, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_5.clone(), pp_4.clone()),
        CalculatorComplex::new(0.5, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_5.clone(), pp_5.clone()),
        CalculatorComplex::new(0.5, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_5.clone(), pp_6.clone()),
        CalculatorComplex::new(0.5, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_6.clone(), pp_4.clone()),
        CalculatorComplex::new(0.5, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_6.clone(), pp_5.clone()),
        CalculatorComplex::new(0.5, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_6.clone(), pp_6.clone()),
        CalculatorComplex::new(0.5, 0.0),
    )
    .unwrap();
    so = so * (1.0 / 3.0_f64.sqrt()) * (1.0 / 3.0_f64.sqrt());
    let pp_1 = DecoherenceProduct::new().z(0);
    let pp_2 = DecoherenceProduct::new().z(1);
    let pp_3 = DecoherenceProduct::new().z(2);
    let pp_4 = DecoherenceProduct::new().z(6);
    let pp_5 = DecoherenceProduct::new().z(7);
    let pp_6 = DecoherenceProduct::new().z(8);
    so.add_operator_product(
        (pp_1.clone(), pp_1.clone()),
        CalculatorComplex::new(1.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_1.clone(), pp_2.clone()),
        CalculatorComplex::new(1.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_1.clone(), pp_3.clone()),
        CalculatorComplex::new(1.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_2.clone(), pp_1.clone()),
        CalculatorComplex::new(1.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_2.clone(), pp_2.clone()),
        CalculatorComplex::new(1.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_2.clone(), pp_3.clone()),
        CalculatorComplex::new(1.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_3.clone(), pp_1.clone()),
        CalculatorComplex::new(1.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_3.clone(), pp_2.clone()),
        CalculatorComplex::new(1.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_3.clone(), pp_3.clone()),
        CalculatorComplex::new(1.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_4.clone(), pp_4.clone()),
        CalculatorComplex::new(0.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_4.clone(), pp_5.clone()),
        CalculatorComplex::new(0.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_4.clone(), pp_6.clone()),
        CalculatorComplex::new(0.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_5.clone(), pp_4.clone()),
        CalculatorComplex::new(0.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_5.clone(), pp_5.clone()),
        CalculatorComplex::new(0.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_5.clone(), pp_6.clone()),
        CalculatorComplex::new(0.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_6.clone(), pp_4.clone()),
        CalculatorComplex::new(0.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_6.clone(), pp_5.clone()),
        CalculatorComplex::new(0.5 / 2.0, 0.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_6.clone(), pp_6.clone()),
        CalculatorComplex::new(0.5 / 2.0, 0.0),
    )
    .unwrap();

    let res = bo.dicke_boson_spin_mapping(3).unwrap();
    let keys_res: Vec<&(DecoherenceProduct, DecoherenceProduct)> = res.keys().collect();
    let keys_so: Vec<&(DecoherenceProduct, DecoherenceProduct)> = so.keys().collect();
    for key in keys_so.clone() {
        assert!(keys_res.contains(&key));
    }
    for key in keys_res {
        assert!(keys_so.contains(&key));
    }
    for key in res.keys() {
        assert!((res.get(key).clone() - so.get(key)).abs().float().unwrap() < &0.0000001_f64);
    }
}

#[test]
fn test_boson_noise_operator_to_spin_error() {
    let bp_1 = BosonProduct::new([0], [1]).unwrap();
    let bp_2 = BosonProduct::new([], [0]).unwrap();
    let bp_3 = BosonProduct::new([0], [0]).unwrap();
    let mut bo = BosonLindbladNoiseOperator::new();
    bo.add_operator_product((bp_1.clone(), bp_1), 1.0.into())
        .unwrap();
    bo.add_operator_product((bp_2.clone(), bp_2.clone()), 1.0.into())
        .unwrap();
    bo.add_operator_product((bp_3.clone(), bp_3.clone()), 1.0.into())
        .unwrap();

    assert_eq!(bo.dicke_boson_spin_mapping(1), Err(struqture::StruqtureError::GenericError{ msg: "The boson -> spin transformation is only available for terms such as b†b or (b† + b), but the term here is: c0a1".into() }));
    assert_eq!(bo.direct_boson_spin_mapping(), Err(struqture::StruqtureError::GenericError{ msg: "The boson -> spin transformation is only available for terms such as b†b or (b† + b), but the term here is: c0a1".into() }));

    let bp_1 = BosonProduct::new([0, 1], [0, 1]).unwrap();
    let mut bo = BosonLindbladNoiseOperator::new();
    bo.add_operator_product((bp_1.clone(), bp_1), 1.0.into())
        .unwrap();
    bo.add_operator_product((bp_2.clone(), bp_2), 1.0.into())
        .unwrap();
    bo.add_operator_product((bp_3.clone(), bp_3), 1.0.into())
        .unwrap();
    assert_eq!(bo.dicke_boson_spin_mapping(1), Err(struqture::StruqtureError::GenericError{ msg: "The boson -> spin transformation is only available for terms such as b†b or (b† + b), but the term here is: c0c1a0a1".into() }));
    assert_eq!(bo.direct_boson_spin_mapping(), Err(struqture::StruqtureError::GenericError{ msg: "The boson -> spin transformation is only available for terms such as b†b or (b† + b), but the term here is: c0c1a0a1".into() }));
}
