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

//! Integration test for public API of mixed indices

use bincode::{deserialize, serialize};
use itertools::Itertools;
use num_complex::Complex64;
use qoqo_calculator::CalculatorComplex;
use serde_test::{assert_tokens, Configure, Token};
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use struqture::bosons::*;
use struqture::fermions::*;
use struqture::mixed_systems::*;
use struqture::prelude::*;
use struqture::spins::DecoherenceProduct;
use struqture::StruqtureError;
use test_case::test_case;

#[test_case(DecoherenceProduct::from_str("").unwrap(), &[], &[], &[], &[]; "empty")]
#[test_case(DecoherenceProduct::from_str("0X").unwrap(), &[0], &[1], &[0], &[1]; "0 - 1")]
#[test_case(DecoherenceProduct::from_str("0Z").unwrap(), &[1], &[], &[1], &[]; "1 - empty")]
#[test_case(DecoherenceProduct::from_str("4Z").unwrap(), &[], &[2000], &[], &[2000]; "empty - 2000")]
#[test_case(DecoherenceProduct::from_str("1iY3X4Z").unwrap(), &[0,1,1], &[3,3,5], &[0,1,2], &[3,4,5]; "0,1,1 - 3,3,5")]
fn new_normal_ordered_passing(
    spins: DecoherenceProduct,
    boson_creators: &[usize],
    boson_annihilators: &[usize],
    fermion_creators: &[usize],
    fermion_annihilators: &[usize],
) {
    let bosons = BosonProduct::new(boson_creators.to_vec(), boson_annihilators.to_vec()).unwrap();
    let fermions =
        FermionProduct::new(fermion_creators.to_vec(), fermion_annihilators.to_vec()).unwrap();
    let test_new =
        MixedDecoherenceProduct::new([spins.clone()], [bosons.clone()], [fermions.clone()]);
    assert!(test_new.is_ok());
    let res = test_new.unwrap();
    for (left, right) in res.spins().zip([spins.clone()].iter()) {
        assert_eq!(left, right);
    }
    for (left, right) in res.bosons().zip([bosons.clone()].iter()) {
        assert_eq!(left, right);
    }
    for (left, right) in res.fermions().zip([fermions.clone()].iter()) {
        assert_eq!(left, right);
    }

    let (valid_pair, coeff) = MixedDecoherenceProduct::create_valid_pair(
        [spins.clone()],
        [bosons.clone()],
        [fermions.clone()],
        1.0.into(),
    )
    .unwrap();
    assert_eq!(coeff, CalculatorComplex::from(1.0));
    for (left, right) in valid_pair.spins().zip([spins].iter()) {
        assert_eq!(left, right);
    }
    for (left, right) in valid_pair.bosons().zip([bosons].iter()) {
        assert_eq!(left, right);
    }
    for (left, right) in valid_pair.fermions().zip([fermions].iter()) {
        assert_eq!(left, right);
    }
}

#[test_case("", &[], &[], &[]; "empty")]
#[test_case(":S0X1X:", &[DecoherenceProduct::from_str("0X1X").unwrap(),], &[], &[]; "single spin systems")]
#[test_case(":S0X1X:S0Z:", &[DecoherenceProduct::from_str("0X1X").unwrap(), DecoherenceProduct::from_str("0Z").unwrap()], &[], &[]; "two spin systems")]
#[test_case(":Bc0a1:", &[], &[BosonProduct::from_str("c0a1").unwrap(),], &[]; "single boson systems")]
#[test_case(":Bc0a0:Bc0a1:", &[], &[BosonProduct::from_str("c0a0").unwrap(), BosonProduct::from_str("c0a1").unwrap(),], &[]; "two boson systems")]
#[test_case(":Fc0a1:", &[], &[], &[FermionProduct::from_str("c0a1").unwrap(),]; "single fermion systems")]
#[test_case(":Fc0a0:Fc0a1:", &[], &[], &[FermionProduct::from_str("c0a0").unwrap(), FermionProduct::from_str("c0a1").unwrap(),]; "two fermion systems")]
#[test_case(":S0X1X:Bc0a1:", &[DecoherenceProduct::from_str("0X1X").unwrap(),], &[BosonProduct::from_str("c0a1").unwrap(),], &[]; "spin-boson systems")]
#[test_case(":S0X1X:Fc0a1:", &[DecoherenceProduct::from_str("0X1X").unwrap(),], &[], &[FermionProduct::from_str("c0a1").unwrap(),]; "spin-fermion systems")]
#[test_case(":Bc0a1:Fc0a1:", &[], &[BosonProduct::from_str("c0a1").unwrap(),], &[FermionProduct::from_str("c0a1").unwrap(),]; "boson-fermion systems")]
#[test_case(":S0X1X:Bc0a1:Fc0a1:", &[DecoherenceProduct::from_str("0X1X").unwrap(),], &[BosonProduct::from_str("c0a1").unwrap(),], &[FermionProduct::from_str("c0a1").unwrap(),]; "spin-boson-fermion systems")]
#[test_case(":S0X1X:S0Z:Bc0a0:Bc0a1:Fc0a0:Fc0a1:", &[DecoherenceProduct::from_str("0X1X").unwrap(), DecoherenceProduct::from_str("0Z").unwrap(),], &[BosonProduct::from_str("c0a0").unwrap(), BosonProduct::from_str("c0a1").unwrap(),], &[FermionProduct::from_str("c0a0").unwrap(), FermionProduct::from_str("c0a1").unwrap(),]; "two spin-boson-fermion systems")]
fn from_string(
    stringformat: &str,
    spins: &[DecoherenceProduct],
    bosons: &[BosonProduct],
    fermions: &[FermionProduct],
) {
    let test_new = <MixedDecoherenceProduct as std::str::FromStr>::from_str(stringformat);
    assert!(test_new.is_ok());
    let res = test_new.unwrap();

    let empty_spins: Vec<DecoherenceProduct> = spins.to_vec();
    let res_spins: Vec<DecoherenceProduct> = res.spins().cloned().collect_vec();
    assert_eq!(res_spins, empty_spins);

    let empty_bosons: Vec<BosonProduct> = bosons.to_vec();
    let res_bosons: Vec<BosonProduct> = res.bosons().cloned().collect_vec();
    assert_eq!(res_bosons, empty_bosons);

    let empty_fermions: Vec<FermionProduct> = fermions.to_vec();
    let res_fermions: Vec<FermionProduct> = res.fermions().cloned().collect_vec();
    assert_eq!(res_fermions, empty_fermions);
}

#[test_case(":S0J:"; "spin fail")]
#[test_case(":Bc0a1c2a3:"; "boson fail")]
#[test_case(":Fc0a1c2a3:"; "fermion fail")]
#[test_case(":Xc0a1c2a3:"; "other fail")]
#[test_case("c0a1c2a3"; "other fail 2")]
fn from_string_fail(stringformat: &str) {
    let test_new = <MixedDecoherenceProduct as std::str::FromStr>::from_str(stringformat);
    assert!(test_new.is_err());
}

#[test_case(DecoherenceProduct::from_str("").unwrap(), &[], &[], &[], &[]; "empty")]
#[test_case(DecoherenceProduct::from_str("0X").unwrap(), &[0], &[1], &[0], &[1]; "0 - 1")]
#[test_case(DecoherenceProduct::from_str("0iY").unwrap(), &[1], &[], &[1], &[]; "1 - empty")]
#[test_case(DecoherenceProduct::from_str("0Z").unwrap(), &[], &[2000], &[], &[2000]; "empty - 2000")]
#[test_case(DecoherenceProduct::from_str("0X1iY2Z").unwrap(), &[0,1,1], &[3,3,5], &[0,1,2], &[3,4,5]; "0,1,1 - 3,3,5")]
#[test_case(DecoherenceProduct::from_str("4X6Z").unwrap(), &[1,2], &[1,2], &[1,2], &[1,2]; "2,1 - 1,2")]
#[test_case(DecoherenceProduct::from_str("9iY").unwrap(), &[0], &[0, 30], &[0], &[0, 30]; "0 - 0,30")]
fn from_string_import_export(
    spins: DecoherenceProduct,
    boson_creators: &[usize],
    boson_annihilators: &[usize],
    fermion_creators: &[usize],
    fermion_annihilators: &[usize],
) {
    let bosons = BosonProduct::new(boson_creators.to_vec(), boson_annihilators.to_vec()).unwrap();
    let fermions =
        FermionProduct::new(fermion_creators.to_vec(), fermion_annihilators.to_vec()).unwrap();
    let test_new =
        MixedDecoherenceProduct::new([spins.clone()], [bosons.clone()], [fermions.clone()]);
    assert!(test_new.is_ok());
    let res = test_new.unwrap();
    let stringformat = format!("{res}");
    let test_new = <MixedDecoherenceProduct as std::str::FromStr>::from_str(&stringformat).unwrap();
    for (left, right) in test_new.spins().zip([spins].iter()) {
        assert_eq!(left, right);
    }
    for (left, right) in test_new.bosons().zip([bosons].iter()) {
        assert_eq!(left, right);
    }
    for (left, right) in test_new.fermions().zip([fermions].iter()) {
        assert_eq!(left, right);
    }
}

#[test_case(DecoherenceProduct::from_str("").unwrap(), &[], &[], &[], &[]; "empty")]
#[test_case(DecoherenceProduct::from_str("0X").unwrap(), &[0], &[1], &[0], &[1]; "0 - 1")]
#[test_case(DecoherenceProduct::from_str("0iY").unwrap(), &[1], &[], &[1], &[]; "1 - empty")]
#[test_case(DecoherenceProduct::from_str("0Z").unwrap(), &[], &[2000], &[], &[2000]; "empty - 2000")]
#[test_case(DecoherenceProduct::from_str("0X1iY2Z").unwrap(), &[0,1,1], &[3,3,5], &[0,1,2], &[3,4,5]; "0,1,1 - 3,3,5")]
#[test_case(DecoherenceProduct::from_str("4X6Z").unwrap(), &[1,2], &[1,2], &[1,2], &[1,2]; "2,1 - 1,2")]
#[test_case(DecoherenceProduct::from_str("9iY").unwrap(), &[0], &[0, 30], &[0], &[0, 30]; "0 - 0,30")]
fn serialize_bincode(
    spins: DecoherenceProduct,
    boson_creators: &[usize],
    boson_annihilators: &[usize],
    fermion_creators: &[usize],
    fermion_annihilators: &[usize],
) {
    let bosons = BosonProduct::new(boson_creators.to_vec(), boson_annihilators.to_vec()).unwrap();
    let fermions =
        FermionProduct::new(fermion_creators.to_vec(), fermion_annihilators.to_vec()).unwrap();
    let test_new = MixedDecoherenceProduct::new([spins], [bosons], [fermions]);
    assert!(test_new.is_ok());
    let res = test_new.unwrap();
    let serialized = serialize(&res).unwrap();
    let deserialized: MixedDecoherenceProduct = deserialize(&serialized).unwrap();
    assert_eq!(res, deserialized);
}

#[test_case(DecoherenceProduct::from_str("").unwrap(), &[], &[], &[], &[]; "empty")]
#[test_case(DecoherenceProduct::from_str("0X").unwrap(), &[0], &[1], &[0], &[1]; "0 - 1")]
#[test_case(DecoherenceProduct::from_str("0iY").unwrap(), &[1], &[], &[1], &[]; "1 - empty")]
#[test_case(DecoherenceProduct::from_str("0Z").unwrap(), &[], &[2000], &[], &[2000]; "empty - 2000")]
#[test_case(DecoherenceProduct::from_str("0X1iY2Z").unwrap(), &[0,1,1], &[3,3,5], &[0,1,2], &[3,4,5]; "0,1,1 - 3,3,5")]
#[test_case(DecoherenceProduct::from_str("4X6Z").unwrap(), &[1,2], &[1,2], &[1,2], &[1,2]; "2,1 - 1,2")]
#[test_case(DecoherenceProduct::from_str("9iY").unwrap(), &[0], &[0, 30], &[0], &[0, 30]; "0 - 0,30")]
fn serialize_json(
    spins: DecoherenceProduct,
    boson_creators: &[usize],
    boson_annihilators: &[usize],
    fermion_creators: &[usize],
    fermion_annihilators: &[usize],
) {
    let bosons = BosonProduct::new(boson_creators.to_vec(), boson_annihilators.to_vec()).unwrap();
    let fermions =
        FermionProduct::new(fermion_creators.to_vec(), fermion_annihilators.to_vec()).unwrap();
    let test_new = MixedDecoherenceProduct::new([spins], [bosons], [fermions]);
    assert!(test_new.is_ok());
    let res = test_new.unwrap();
    let serialized = serde_json::to_string(&res).unwrap();
    let deserialized: MixedDecoherenceProduct = serde_json::from_str(&serialized).unwrap();
    assert_eq!(res, deserialized);
}

#[test]
fn multiply_error() {
    let spins_left = DecoherenceProduct::new().x(1);
    let spins_right = DecoherenceProduct::new().z(0);

    let left = MixedDecoherenceProduct::new([spins_left.clone(), spins_left], [], []).unwrap();
    let right = MixedDecoherenceProduct::new([spins_right], [], []).unwrap();

    let result = left * right;
    assert_eq!(
        result,
        Err(StruqtureError::MismatchedNumberSubsystems {
            target_number_spin_subsystems: 2,
            target_number_boson_subsystems: 0,
            target_number_fermion_subsystems: 0,
            actual_number_spin_subsystems: 1,
            actual_number_boson_subsystems: 0,
            actual_number_fermion_subsystems: 0,
        })
    );
}

#[test]
fn multiply_spins() {
    let spins_left = DecoherenceProduct::new().x(1);
    let spins_right = DecoherenceProduct::new().z(0);

    let left = MixedDecoherenceProduct::new([spins_left], [], []).unwrap();
    let right = MixedDecoherenceProduct::new([spins_right], [], []).unwrap();

    let result = (left * right).unwrap();
    let comparison: Vec<(MixedDecoherenceProduct, Complex64)> = vec![(
        MixedDecoherenceProduct::new([DecoherenceProduct::new().z(0).x(1)], vec![], vec![])
            .unwrap(),
        (1.0).into(),
    )];
    assert_eq!(result.len(), comparison.len());
    for res in comparison {
        assert!(result.contains(&res));
    }
}

#[test]
fn multiply_spins_bosons() {
    let creators_left: &[usize] = &[];
    let annihilators_left: &[usize] = &[0];
    let spins_left = DecoherenceProduct::new().x(1);
    let bosons_left =
        BosonProduct::new(creators_left.to_vec(), annihilators_left.to_vec()).unwrap();

    let creators_right: &[usize] = &[0];
    let annihilators_right: &[usize] = &[];
    let spins_right = DecoherenceProduct::new().z(0);
    let bosons_right =
        BosonProduct::new(creators_right.to_vec(), annihilators_right.to_vec()).unwrap();

    let left = MixedDecoherenceProduct::new(
        [spins_left],
        [bosons_left.clone(), bosons_right.clone()],
        [],
    )
    .unwrap();
    let right =
        MixedDecoherenceProduct::new([spins_right], [bosons_right, bosons_left], []).unwrap();

    let result = (left * right).unwrap();
    let comparison: Vec<(MixedDecoherenceProduct, Complex64)> = vec![
        (
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new().z(0).x(1)],
                vec![
                    BosonProduct::new([], []).unwrap(),
                    BosonProduct::new([0], [0]).unwrap(),
                ],
                vec![],
            )
            .unwrap(),
            (1.0).into(),
        ),
        (
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new().z(0).x(1)],
                vec![
                    BosonProduct::new([0], [0]).unwrap(),
                    BosonProduct::new([0], [0]).unwrap(),
                ],
                vec![],
            )
            .unwrap(),
            1.0.into(),
        ),
    ];
    assert_eq!(result.len(), comparison.len());
    for res in comparison {
        assert!(result.contains(&res));
    }
}

#[test]
fn multiply_spins_fermions() {
    let creators_left: &[usize] = &[];
    let annihilators_left: &[usize] = &[0];
    let spins_left = DecoherenceProduct::new().x(1);
    let fermions_left =
        FermionProduct::new(creators_left.to_vec(), annihilators_left.to_vec()).unwrap();

    let creators_right: &[usize] = &[0];
    let annihilators_right: &[usize] = &[];
    let spins_right = DecoherenceProduct::new().z(0);
    let fermions_right =
        FermionProduct::new(creators_right.to_vec(), annihilators_right.to_vec()).unwrap();

    let left = MixedDecoherenceProduct::new(
        [spins_left],
        [],
        [fermions_left.clone(), fermions_right.clone()],
    )
    .unwrap();
    let right =
        MixedDecoherenceProduct::new([spins_right], [], [fermions_right, fermions_left]).unwrap();

    let result = (left * right).unwrap();
    let comparison: Vec<(MixedDecoherenceProduct, Complex64)> = vec![
        (
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new().z(0).x(1)],
                vec![],
                vec![
                    FermionProduct::new([], []).unwrap(),
                    FermionProduct::new([0], [0]).unwrap(),
                ],
            )
            .unwrap(),
            1.0.into(),
        ),
        (
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new().z(0).x(1)],
                vec![],
                vec![
                    FermionProduct::new([0], [0]).unwrap(),
                    FermionProduct::new([0], [0]).unwrap(),
                ],
            )
            .unwrap(),
            (-1.0).into(),
        ),
    ];
    assert_eq!(result.len(), comparison.len());
    for res in comparison {
        assert!(result.contains(&res));
    }
}

#[test]
fn multiply_spins_bosons_fermions() {
    let creators_left: &[usize] = &[];
    let annihilators_left: &[usize] = &[0];
    let spins_left = DecoherenceProduct::new().x(1);
    let bosons_left =
        BosonProduct::new(creators_left.to_vec(), annihilators_left.to_vec()).unwrap();
    let fermions_left =
        FermionProduct::new(creators_left.to_vec(), annihilators_left.to_vec()).unwrap();

    let creators_right: &[usize] = &[0];
    let annihilators_right: &[usize] = &[];
    let spins_right = DecoherenceProduct::new().z(0);
    let bosons_right =
        BosonProduct::new(creators_right.to_vec(), annihilators_right.to_vec()).unwrap();
    let fermions_right =
        FermionProduct::new(creators_right.to_vec(), annihilators_right.to_vec()).unwrap();

    let left = MixedDecoherenceProduct::new(
        [spins_left],
        [bosons_left.clone(), bosons_right.clone()],
        [fermions_left.clone(), fermions_right.clone()],
    )
    .unwrap();
    let right = MixedDecoherenceProduct::new(
        [spins_right],
        [bosons_right, bosons_left],
        [fermions_right, fermions_left],
    )
    .unwrap();

    let result = (left * right).unwrap();
    let comparison: Vec<(MixedDecoherenceProduct, Complex64)> = vec![
        (
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new().z(0).x(1)],
                vec![
                    BosonProduct::new([], []).unwrap(),
                    BosonProduct::new([0], [0]).unwrap(),
                ],
                vec![
                    FermionProduct::new([], []).unwrap(),
                    FermionProduct::new([0], [0]).unwrap(),
                ],
            )
            .unwrap(),
            (1.0).into(),
        ),
        (
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new().z(0).x(1)],
                vec![
                    BosonProduct::new([], []).unwrap(),
                    BosonProduct::new([0], [0]).unwrap(),
                ],
                vec![
                    FermionProduct::new([0], [0]).unwrap(),
                    FermionProduct::new([0], [0]).unwrap(),
                ],
            )
            .unwrap(),
            (-1.0).into(),
        ),
        (
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new().z(0).x(1)],
                vec![
                    BosonProduct::new([0], [0]).unwrap(),
                    BosonProduct::new([0], [0]).unwrap(),
                ],
                vec![
                    FermionProduct::new([], []).unwrap(),
                    FermionProduct::new([0], [0]).unwrap(),
                ],
            )
            .unwrap(),
            1.0.into(),
        ),
        (
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new().z(0).x(1)],
                vec![
                    BosonProduct::new([0], [0]).unwrap(),
                    BosonProduct::new([0], [0]).unwrap(),
                ],
                vec![
                    FermionProduct::new([0], [0]).unwrap(),
                    FermionProduct::new([0], [0]).unwrap(),
                ],
            )
            .unwrap(),
            (-1.0).into(),
        ),
    ];
    assert_eq!(result.len(), comparison.len());
    for res in comparison {
        assert!(result.contains(&res));
    }
}

#[test_case(&[], &[], &[]; "empty")]
fn mixed_default(
    spins: &[DecoherenceProduct],
    bosons: &[BosonProduct],
    fermions: &[FermionProduct],
) {
    let test_new = MixedDecoherenceProduct::default();

    let empty_spins: Vec<DecoherenceProduct> = spins.to_vec();
    let test_spins: Vec<DecoherenceProduct> = test_new.spins().cloned().collect_vec();
    assert_eq!(test_spins, empty_spins);

    let empty_bosons: Vec<BosonProduct> = bosons.to_vec();
    let test_bosons: Vec<BosonProduct> = test_new.bosons().cloned().collect_vec();
    assert_eq!(test_bosons, empty_bosons);

    let empty_fermions: Vec<FermionProduct> = fermions.to_vec();
    let test_fermions: Vec<FermionProduct> = test_new.fermions().cloned().collect_vec();
    assert_eq!(test_fermions, empty_fermions);
}

// Test the hermitian_conjugate and is_natural_hermitian functions of the MixedDecoherenceProduct
#[test]
fn hermitian_test() {
    let spins = DecoherenceProduct::from_str("0X").unwrap();
    let creators = &[0];
    let annihilators = &[3];
    let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let test_new = MixedDecoherenceProduct::new([spins.clone()], [bosons], [fermions]).unwrap();

    assert!(!test_new.is_natural_hermitian());
    let creators_h = &[3];
    let annihilators_h = &[0];
    let bosons_h = BosonProduct::new(creators_h.to_vec(), annihilators_h.to_vec()).unwrap();
    let fermions_h = FermionProduct::new(creators_h.to_vec(), annihilators_h.to_vec()).unwrap();
    let hermitian_test = MixedDecoherenceProduct::new([spins], [bosons_h], [fermions_h]).unwrap();
    assert_eq!(test_new.hermitian_conjugate(), (hermitian_test, 1.0));
}

// Test the hermitian_conjugate and is_natural_hermitian functions of the MixedDecoherenceProduct
#[test]
fn get_value_mixed() {
    let spins = DecoherenceProduct::from_str("0X").unwrap();
    let creators = &[0];
    let annihilators = &[3];
    let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let test_new = MixedDecoherenceProduct::new([spins], [bosons], [fermions]).unwrap();

    assert_eq!(MixedDecoherenceProduct::get_key(&test_new), test_new);
    assert_eq!(
        MixedDecoherenceProduct::get_transform(&test_new, CalculatorComplex::new(1.0, 2.0)),
        CalculatorComplex::new(1.0, 2.0)
    );
}

// Test the Hash, Debug and Display traits of DecoherenceProduct
#[test]
fn debug() {
    let spins = DecoherenceProduct::from_str("0X1iY2Z").unwrap();
    let b_creators = &[0, 1, 1];
    let b_annihilators = &[3, 3, 5];
    let f_creators = &[0, 1, 2];
    let f_annihilators = &[3, 4, 5];
    let bosons = BosonProduct::new(b_creators.to_vec(), b_annihilators.to_vec()).unwrap();
    let fermions = FermionProduct::new(f_creators.to_vec(), f_annihilators.to_vec()).unwrap();
    let test_new = MixedDecoherenceProduct::new([spins], [bosons], [fermions]).unwrap();

    assert_eq!(
        format!("{test_new:?}"),
        "MixedDecoherenceProduct { spins: [DecoherenceProduct { items: [(0, X), (1, IY), (2, Z)] }], bosons: [BosonProduct { creators: [0, 1, 1], annihilators: [3, 3, 5] }], fermions: [FermionProduct { creators: [0, 1, 2], annihilators: [3, 4, 5] }] }"
    );
    assert_eq!(
        format!("{test_new}"),
        "S0X1iY2Z:Bc0c1c1a3a3a5:Fc0c1c2a3a4a5:"
    );
}

// Test the Hash, Debug and Display traits of DecoherenceProduct
#[test]
fn hash_debug() {
    let spins = DecoherenceProduct::from_str("0X1iY2Z").unwrap();
    let b_creators = &[0, 1, 1];
    let b_annihilators = &[3, 3, 5];
    let f_creators = &[0, 1, 2];
    let f_annihilators = &[3, 4, 5];
    let bosons = BosonProduct::new(b_creators.to_vec(), b_annihilators.to_vec()).unwrap();
    let fermions = FermionProduct::new(f_creators.to_vec(), f_annihilators.to_vec()).unwrap();
    let test_new =
        MixedDecoherenceProduct::new([spins.clone()], [bosons.clone()], [fermions.clone()])
            .unwrap();
    let test_new_1 = MixedDecoherenceProduct::new([spins], [bosons], [fermions]).unwrap();

    let mut s_1 = DefaultHasher::new();
    test_new.hash(&mut s_1);
    let mut s_2 = DefaultHasher::new();
    test_new_1.hash(&mut s_2);
    assert_eq!(s_1.finish(), s_2.finish())
}

// Test the Clone, PartialEq, PartialOrd and Ord traits of DecoherenceProduct
#[test]
fn clone_partial_eq_partial_ord() {
    let spins = DecoherenceProduct::from_str("0X1iY2Z").unwrap();
    let b_creators = &[0, 1, 1];
    let b_annihilators = &[3, 3, 5];
    let f_creators = &[0, 1, 2];
    let f_annihilators = &[3, 4, 5];
    let bosons = BosonProduct::new(b_creators.to_vec(), b_annihilators.to_vec()).unwrap();
    let fermions = FermionProduct::new(f_creators.to_vec(), f_annihilators.to_vec()).unwrap();
    let test_new =
        MixedDecoherenceProduct::new([spins.clone()], [bosons.clone()], [fermions.clone()])
            .unwrap();

    // Test Clone trait
    assert_eq!(test_new.clone(), test_new);

    // Test PartialEq trait
    let test_0 =
        MixedDecoherenceProduct::new([spins.clone()], [bosons.clone()], [fermions.clone()])
            .unwrap();
    let test_1 = MixedDecoherenceProduct::new(
        [DecoherenceProduct::from_str("").unwrap()],
        [bosons.clone()],
        [fermions.clone()],
    )
    .unwrap();
    assert!(test_0 == test_new);
    assert!(test_new == test_0);
    assert!(test_1 != test_new);
    assert!(test_new != test_1);

    // Test PartialOrd trait
    let test_0 =
        MixedDecoherenceProduct::new([spins], [bosons.clone()], [fermions.clone()]).unwrap();
    let test_1 = MixedDecoherenceProduct::new(
        [DecoherenceProduct::from_str("").unwrap()],
        [bosons],
        [fermions],
    )
    .unwrap();

    assert_eq!(test_0.partial_cmp(&test_new), Some(Ordering::Equal));
    assert_eq!(test_new.partial_cmp(&test_0), Some(Ordering::Equal));
    assert_eq!(test_1.partial_cmp(&test_new), Some(Ordering::Less));
    assert_eq!(test_new.partial_cmp(&test_1), Some(Ordering::Greater));

    // Test Ord trait
    assert_eq!(test_0.cmp(&test_new), Ordering::Equal);
    assert_eq!(test_new.cmp(&test_0), Ordering::Equal);
    assert_eq!(test_1.cmp(&test_new), Ordering::Less);
    assert_eq!(test_new.cmp(&test_1), Ordering::Greater);
}

#[test]
fn serde_readable() {
    let spins = DecoherenceProduct::from_str("0X").unwrap();
    let creators = &[0];
    let annihilators = &[3];
    let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let test_new = MixedDecoherenceProduct::new([spins], [bosons], [fermions]).unwrap();

    let serialized = serde_json::to_string(&test_new).unwrap();
    let deserialized: MixedDecoherenceProduct = serde_json::from_str(&serialized).unwrap();

    assert_eq!(test_new, deserialized);
    assert_tokens(&test_new.readable(), &[Token::String("S0X:Bc0a3:Fc0a3:")]);
}

#[test]
fn serde_compact() {
    let spins = DecoherenceProduct::from_str("0X").unwrap();
    let creators = &[0];
    let annihilators = &[3];
    let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let test_new = MixedDecoherenceProduct::new([spins], [bosons], [fermions]).unwrap();

    let serialized = serde_json::to_string(&test_new).unwrap();
    let deserialized: MixedDecoherenceProduct = serde_json::from_str(&serialized).unwrap();

    assert_eq!(test_new, deserialized);
    assert_tokens(
        &test_new.compact(),
        &[
            Token::Tuple { len: 3 },
            Token::Seq { len: Some(1) },
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::U64(0),
            Token::UnitVariant {
                name: "SingleDecoherenceOperator",
                variant: "X",
            },
            Token::TupleEnd,
            Token::SeqEnd,
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::Seq { len: Some(1) },
            Token::U64(0),
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::U64(3),
            Token::SeqEnd,
            Token::TupleEnd,
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::Seq { len: Some(1) },
            Token::U64(0),
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::U64(3),
            Token::SeqEnd,
            Token::TupleEnd,
            Token::SeqEnd,
            Token::TupleEnd,
        ],
    );
}

#[cfg(feature = "json_schema")]
#[test]
fn test_mixed_decoherence_product_schema() {
    let pp = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().x(0), DecoherenceProduct::new()],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let schema = schemars::schema_for!(MixedDecoherenceProduct);
    let schema_checker = jsonschema::validator_for(&serde_json::to_value(&schema).unwrap())
        .expect("schema is valid");
    let value = serde_json::to_value(pp).unwrap();
    let validation = schema_checker.validate(&value);
    assert!(validation.is_ok());
}

#[cfg(feature = "struqture_1_import")]
#[cfg(feature = "struqture_1_export")]
#[test]
fn test_from_to_struqture_1() {
    let pp_1: struqture_1::mixed_systems::MixedDecoherenceProduct =
        struqture_1::mixed_systems::MixedIndex::new(
            [struqture_1::spins::DecoherenceProduct::from_str("0X").unwrap()],
            [struqture_1::bosons::BosonProduct::from_str("c0a1").unwrap()],
            [struqture_1::fermions::FermionProduct::from_str("c0a0").unwrap()],
        )
        .unwrap();
    let pp_2 = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [0]).unwrap()],
    )
    .unwrap();
    assert!(MixedDecoherenceProduct::from_struqture_1(&pp_1).unwrap() == pp_2);
    assert!(pp_1 == pp_2.to_struqture_1().unwrap());
}
