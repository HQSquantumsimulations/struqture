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
use serde_test::{assert_tokens, Configure, Token};
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use struqture::bosons::*;
use struqture::fermions::*;
use struqture::mixed_systems::*;
use struqture::prelude::*;
use struqture::spins::{PauliProduct, PlusMinusProduct};
use test_case::test_case;

#[test_case(PlusMinusProduct::from_str("").unwrap(), &[], &[], &[], &[]; "empty")]
#[test_case(PlusMinusProduct::from_str("0+").unwrap(), &[0], &[1], &[0], &[1]; "0 - 1")]
#[test_case(PlusMinusProduct::from_str("0Z").unwrap(), &[1], &[], &[1], &[]; "1 - empty")]
#[test_case(PlusMinusProduct::from_str("4Z").unwrap(), &[], &[2000], &[], &[2000]; "empty - 2000")]
#[test_case(PlusMinusProduct::from_str("1-3+4Z").unwrap(), &[0,1,1], &[3,3,5], &[0,1,2], &[3,4,5]; "0,1,1 - 3,3,5")]
fn new_normal_ordered_passing(
    spins: PlusMinusProduct,
    boson_creators: &[usize],
    boson_annihilators: &[usize],
    fermion_creators: &[usize],
    fermion_annihilators: &[usize],
) {
    let bosons = BosonProduct::new(boson_creators.to_vec(), boson_annihilators.to_vec()).unwrap();
    let fermions =
        FermionProduct::new(fermion_creators.to_vec(), fermion_annihilators.to_vec()).unwrap();
    let test_new =
        MixedPlusMinusProduct::new([spins.clone()], [bosons.clone()], [fermions.clone()]);
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

#[test_case("", &[], &[], &[]; "empty")]
#[test_case(":S0+1+:", &[PlusMinusProduct::from_str("0+1+").unwrap(),], &[], &[]; "single spin systems")]
#[test_case(":S0+1+:S0Z:", &[PlusMinusProduct::from_str("0+1+").unwrap(), PlusMinusProduct::from_str("0Z").unwrap()], &[], &[]; "two spin systems")]
#[test_case(":Bc0a1:", &[], &[BosonProduct::from_str("c0a1").unwrap(),], &[]; "single boson systems")]
#[test_case(":Bc0a0:Bc0a1:", &[], &[BosonProduct::from_str("c0a0").unwrap(), BosonProduct::from_str("c0a1").unwrap(),], &[]; "two boson systems")]
#[test_case(":Fc0a1:", &[], &[], &[FermionProduct::from_str("c0a1").unwrap(),]; "single fermion systems")]
#[test_case(":Fc0a0:Fc0a1:", &[], &[], &[FermionProduct::from_str("c0a0").unwrap(), FermionProduct::from_str("c0a1").unwrap(),]; "two fermion systems")]
#[test_case(":S0+1+:Bc0a1:", &[PlusMinusProduct::from_str("0+1+").unwrap(),], &[BosonProduct::from_str("c0a1").unwrap(),], &[]; "spin-boson systems")]
#[test_case(":S0+1+:Fc0a1:", &[PlusMinusProduct::from_str("0+1+").unwrap(),], &[], &[FermionProduct::from_str("c0a1").unwrap(),]; "spin-fermion systems")]
#[test_case(":Bc0a1:Fc0a1:", &[], &[BosonProduct::from_str("c0a1").unwrap(),], &[FermionProduct::from_str("c0a1").unwrap(),]; "boson-fermion systems")]
#[test_case(":S0+1+:Bc0a1:Fc0a1:", &[PlusMinusProduct::from_str("0+1+").unwrap(),], &[BosonProduct::from_str("c0a1").unwrap(),], &[FermionProduct::from_str("c0a1").unwrap(),]; "spin-boson-fermion systems")]
#[test_case(":S0+1+:S0Z:Bc0a0:Bc0a1:Fc0a0:Fc0a1:", &[PlusMinusProduct::from_str("0+1+").unwrap(), PlusMinusProduct::from_str("0Z").unwrap(),], &[BosonProduct::from_str("c0a0").unwrap(), BosonProduct::from_str("c0a1").unwrap(),], &[FermionProduct::from_str("c0a0").unwrap(), FermionProduct::from_str("c0a1").unwrap(),]; "two spin-boson-fermion systems")]
fn from_string(
    stringformat: &str,
    spins: &[PlusMinusProduct],
    bosons: &[BosonProduct],
    fermions: &[FermionProduct],
) {
    let test_new = <MixedPlusMinusProduct as std::str::FromStr>::from_str(stringformat);
    assert!(test_new.is_ok());
    let res = test_new.unwrap();

    let empty_spins: Vec<PlusMinusProduct> = spins.to_vec();
    let res_spins: Vec<PlusMinusProduct> = res.spins().cloned().collect_vec();
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
#[test_case(":+c0a1c2a3:"; "other fail")]
#[test_case("c0a1c2a3"; "other fail 2")]
fn from_string_fail(stringformat: &str) {
    let test_new = <MixedPlusMinusProduct as std::str::FromStr>::from_str(stringformat);
    assert!(test_new.is_err());
}

#[test_case(PlusMinusProduct::from_str("").unwrap(), &[], &[], &[], &[]; "empty")]
#[test_case(PlusMinusProduct::from_str("0+").unwrap(), &[0], &[1], &[0], &[1]; "0 - 1")]
#[test_case(PlusMinusProduct::from_str("0-").unwrap(), &[1], &[], &[1], &[]; "1 - empty")]
#[test_case(PlusMinusProduct::from_str("0Z").unwrap(), &[], &[2000], &[], &[2000]; "empty - 2000")]
#[test_case(PlusMinusProduct::from_str("0+1-2Z").unwrap(), &[0,1,1], &[3,3,5], &[0,1,2], &[3,4,5]; "0,1,1 - 3,3,5")]
#[test_case(PlusMinusProduct::from_str("4+6Z").unwrap(), &[1,2], &[1,2], &[1,2], &[1,2]; "2,1 - 1,2")]
#[test_case(PlusMinusProduct::from_str("9-").unwrap(), &[0], &[0, 30], &[0], &[0, 30]; "0 - 0,30")]
fn from_string_import_export(
    spins: PlusMinusProduct,
    boson_creators: &[usize],
    boson_annihilators: &[usize],
    fermion_creators: &[usize],
    fermion_annihilators: &[usize],
) {
    let bosons = BosonProduct::new(boson_creators.to_vec(), boson_annihilators.to_vec()).unwrap();
    let fermions =
        FermionProduct::new(fermion_creators.to_vec(), fermion_annihilators.to_vec()).unwrap();
    let test_new =
        MixedPlusMinusProduct::new([spins.clone()], [bosons.clone()], [fermions.clone()]);
    let stringformat = format!("{}", test_new);
    let test_new = <MixedPlusMinusProduct as std::str::FromStr>::from_str(&stringformat).unwrap();
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

#[test_case(PlusMinusProduct::from_str("").unwrap(), &[], &[], &[], &[]; "empty")]
#[test_case(PlusMinusProduct::from_str("0+").unwrap(), &[0], &[1], &[0], &[1]; "0 - 1")]
#[test_case(PlusMinusProduct::from_str("0-").unwrap(), &[1], &[], &[1], &[]; "1 - empty")]
#[test_case(PlusMinusProduct::from_str("0Z").unwrap(), &[], &[2000], &[], &[2000]; "empty - 2000")]
#[test_case(PlusMinusProduct::from_str("0+1-2Z").unwrap(), &[0,1,1], &[3,3,5], &[0,1,2], &[3,4,5]; "0,1,1 - 3,3,5")]
#[test_case(PlusMinusProduct::from_str("4+6Z").unwrap(), &[1,2], &[1,2], &[1,2], &[1,2]; "2,1 - 1,2")]
#[test_case(PlusMinusProduct::from_str("9-").unwrap(), &[0], &[0, 30], &[0], &[0, 30]; "0 - 0,30")]
fn serialize_bincode(
    spins: PlusMinusProduct,
    boson_creators: &[usize],
    boson_annihilators: &[usize],
    fermion_creators: &[usize],
    fermion_annihilators: &[usize],
) {
    let bosons = BosonProduct::new(boson_creators.to_vec(), boson_annihilators.to_vec()).unwrap();
    let fermions =
        FermionProduct::new(fermion_creators.to_vec(), fermion_annihilators.to_vec()).unwrap();
    let test_new: MixedPlusMinusProduct = MixedPlusMinusProduct::new([spins], [bosons], [fermions]);

    let serialized = serialize(&test_new).unwrap();
    let deserialized: MixedPlusMinusProduct = deserialize(&serialized).unwrap();
    assert_eq!(test_new, deserialized);
}

#[test_case(PlusMinusProduct::from_str("").unwrap(), &[], &[], &[], &[]; "empty")]
#[test_case(PlusMinusProduct::from_str("0+").unwrap(), &[0], &[1], &[0], &[1]; "0 - 1")]
#[test_case(PlusMinusProduct::from_str("0-").unwrap(), &[1], &[], &[1], &[]; "1 - empty")]
#[test_case(PlusMinusProduct::from_str("0Z").unwrap(), &[], &[2000], &[], &[2000]; "empty - 2000")]
#[test_case(PlusMinusProduct::from_str("0+1-2Z").unwrap(), &[0,1,1], &[3,3,5], &[0,1,2], &[3,4,5]; "0,1,1 - 3,3,5")]
#[test_case(PlusMinusProduct::from_str("4+6Z").unwrap(), &[1,2], &[1,2], &[1,2], &[1,2]; "2,1 - 1,2")]
#[test_case(PlusMinusProduct::from_str("9-").unwrap(), &[0], &[0, 30], &[0], &[0, 30]; "0 - 0,30")]
fn serialize_json(
    spins: PlusMinusProduct,
    boson_creators: &[usize],
    boson_annihilators: &[usize],
    fermion_creators: &[usize],
    fermion_annihilators: &[usize],
) {
    let bosons = BosonProduct::new(boson_creators.to_vec(), boson_annihilators.to_vec()).unwrap();
    let fermions =
        FermionProduct::new(fermion_creators.to_vec(), fermion_annihilators.to_vec()).unwrap();
    let res = MixedPlusMinusProduct::new([spins], [bosons], [fermions]);
    let serialized = serde_json::to_string(&res).unwrap();
    let deserialized: MixedPlusMinusProduct = serde_json::from_str(&serialized).unwrap();
    assert_eq!(res, deserialized);
}

#[test_case(&[], &[], &[]; "empty")]
fn mixed_default(spins: &[PlusMinusProduct], bosons: &[BosonProduct], fermions: &[FermionProduct]) {
    let test_new = MixedPlusMinusProduct::default();

    let empty_spins: Vec<PlusMinusProduct> = spins.to_vec();
    let test_spins: Vec<PlusMinusProduct> = test_new.spins().cloned().collect_vec();
    assert_eq!(test_spins, empty_spins);

    let empty_bosons: Vec<BosonProduct> = bosons.to_vec();
    let test_bosons: Vec<BosonProduct> = test_new.bosons().cloned().collect_vec();
    assert_eq!(test_bosons, empty_bosons);

    let empty_fermions: Vec<FermionProduct> = fermions.to_vec();
    let test_fermions: Vec<FermionProduct> = test_new.fermions().cloned().collect_vec();
    assert_eq!(test_fermions, empty_fermions);
}

// Test the hermitian_conjugate and is_natural_hermitian functions of the MixedPlusMinusProduct
#[test]
fn hermitian_test() {
    let spins = PlusMinusProduct::from_str("0+").unwrap();
    let creators = &[0];
    let annihilators = &[3];
    let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let test_new = MixedPlusMinusProduct::new([spins], [bosons], [fermions]);

    assert!(!test_new.is_natural_hermitian());
    let spins_h = PlusMinusProduct::from_str("0-").unwrap();
    let creators_h = &[3];
    let annihilators_h = &[0];
    let bosons_h = BosonProduct::new(creators_h.to_vec(), annihilators_h.to_vec()).unwrap();
    let fermions_h = FermionProduct::new(creators_h.to_vec(), annihilators_h.to_vec()).unwrap();
    let hermitian_test = MixedPlusMinusProduct::new([spins_h], [bosons_h], [fermions_h]);
    assert_eq!(test_new.hermitian_conjugate(), (hermitian_test, 1.0));
}

// Test the hermitian_conjugate and is_natural_hermitian functions of the MixedPlusMinusProduct
#[test]
fn to_mixed_product_1() {
    let spins_1 = PlusMinusProduct::from_str("0+").unwrap();
    let creators = &[0];
    let annihilators = &[3];
    let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let test_new = MixedPlusMinusProduct::new([spins_1], [bosons.clone()], [fermions.clone()]);

    let spins_1_c = PauliProduct::from_str("0X").unwrap();
    let spins_2_c = PauliProduct::from_str("0Y").unwrap();
    let mixed_1 = MixedProduct::new([spins_1_c], [bosons.clone()], [fermions.clone()]).unwrap();
    let mixed_2 = MixedProduct::new([spins_2_c], [bosons], [fermions]).unwrap();

    let res: Vec<(MixedProduct, Complex64)> = test_new.try_into().unwrap();
    assert_eq!(res.len(), 2);
    for (left, right) in res.iter().zip(vec![
        (mixed_1, 0.5.into()),
        (mixed_2, Complex64::new(0.0, 0.5)),
    ]) {
        assert_eq!(left, &right);
    }
}

// Test the hermitian_conjugate and is_natural_hermitian functions of the MixedPlusMinusProduct
#[test]
fn to_mixed_product_2() {
    let spins_1 = PlusMinusProduct::from_str("0+").unwrap();
    let spins_2 = PlusMinusProduct::from_str("0-").unwrap();
    let creators = &[0];
    let annihilators = &[3];
    let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let test_new =
        MixedPlusMinusProduct::new([spins_1, spins_2], [bosons.clone()], [fermions.clone()]);

    let spins_1_c = PauliProduct::from_str("0X").unwrap();
    let spins_2_c = PauliProduct::from_str("0Y").unwrap();
    let mixed_1 = MixedProduct::new(
        [spins_1_c.clone(), spins_1_c.clone()],
        [bosons.clone()],
        [fermions.clone()],
    )
    .unwrap();
    let mixed_2 = MixedProduct::new(
        [spins_1_c.clone(), spins_2_c.clone()],
        [bosons.clone()],
        [fermions.clone()],
    )
    .unwrap();
    let mixed_3 = MixedProduct::new(
        [spins_2_c.clone(), spins_1_c],
        [bosons.clone()],
        [fermions.clone()],
    )
    .unwrap();
    let mixed_4 = MixedProduct::new([spins_2_c.clone(), spins_2_c], [bosons], [fermions]).unwrap();

    let res: Vec<(MixedProduct, Complex64)> = test_new.try_into().unwrap();
    assert_eq!(res.len(), 4);
    for (left, right) in res.iter().zip(vec![
        (mixed_1, 0.25.into()),
        (mixed_2, Complex64::new(0.0, -0.25)),
        (mixed_3, Complex64::new(0.0, 0.25)),
        (mixed_4, Complex64::new(0.25, 0.0)),
    ]) {
        assert_eq!(left, &right);
    }
}

// Test the hermitian_conjugate and is_natural_hermitian functions of the MixedPlusMinusProduct
#[test]
fn to_mixed_product_3() {
    let spins_1 = PlusMinusProduct::from_str("0+").unwrap();
    let spins_2 = PlusMinusProduct::from_str("0Z").unwrap();
    let spins_3 = PlusMinusProduct::from_str("0-").unwrap();
    let creators = &[0];
    let annihilators = &[3];
    let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let test_new = MixedPlusMinusProduct::new(
        [spins_1, spins_2, spins_3],
        [bosons.clone()],
        [fermions.clone()],
    );

    let spins_1_c = PauliProduct::from_str("0X").unwrap();
    let spins_2_c = PauliProduct::from_str("0Z").unwrap();
    let spins_3_c = PauliProduct::from_str("0Y").unwrap();
    let mixed_1 = MixedProduct::new(
        [spins_1_c.clone(), spins_2_c.clone(), spins_1_c.clone()],
        [bosons.clone()],
        [fermions.clone()],
    )
    .unwrap();
    let mixed_2 = MixedProduct::new(
        [spins_1_c.clone(), spins_2_c.clone(), spins_3_c.clone()],
        [bosons.clone()],
        [fermions.clone()],
    )
    .unwrap();
    let mixed_3 = MixedProduct::new(
        [spins_3_c.clone(), spins_2_c.clone(), spins_1_c],
        [bosons.clone()],
        [fermions.clone()],
    )
    .unwrap();
    let mixed_4 = MixedProduct::new(
        [spins_3_c.clone(), spins_2_c, spins_3_c],
        [bosons],
        [fermions],
    )
    .unwrap();

    let res: Vec<(MixedProduct, Complex64)> = test_new.try_into().unwrap();
    assert_eq!(res.len(), 4);
    for (left, right) in res.iter().zip(vec![
        (mixed_1, 0.25.into()),
        (mixed_2, Complex64::new(0.0, -0.25)),
        (mixed_3, Complex64::new(0.0, 0.25)),
        (mixed_4, Complex64::new(0.25, 0.0)),
    ]) {
        assert_eq!(left, &right);
    }
}

// Test the hermitian_conjugate and is_natural_hermitian functions of the MixedPlusMinusProduct
#[test]
fn to_mixed_product_longer() {
    let spins_1 = PlusMinusProduct::from_str("2Z3Z").unwrap();
    let spins_2 = PlusMinusProduct::from_str("1+2-3Z").unwrap();
    let creators = &[0];
    let annihilators = &[3];
    let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let test_new =
        MixedPlusMinusProduct::new([spins_1, spins_2], [bosons.clone()], [fermions.clone()]);

    let spins_1_c = PauliProduct::from_str("2Z3Z").unwrap();
    let mixed_1 = MixedProduct::new(
        [spins_1_c.clone(), PauliProduct::from_str("1X2X3Z").unwrap()],
        [bosons.clone()],
        [fermions.clone()],
    )
    .unwrap();
    let mixed_2 = MixedProduct::new(
        [spins_1_c.clone(), PauliProduct::from_str("1Y2X3Z").unwrap()],
        [bosons.clone()],
        [fermions.clone()],
    )
    .unwrap();
    let mixed_3 = MixedProduct::new(
        [spins_1_c.clone(), PauliProduct::from_str("1X2Y3Z").unwrap()],
        [bosons.clone()],
        [fermions.clone()],
    )
    .unwrap();
    let mixed_4 = MixedProduct::new(
        [spins_1_c, PauliProduct::from_str("1Y2Y3Z").unwrap()],
        [bosons],
        [fermions],
    )
    .unwrap();

    let res: Vec<(MixedProduct, Complex64)> = test_new.try_into().unwrap();
    assert_eq!(res.len(), 4);
    for (left, right) in res.iter().zip(vec![
        (mixed_1, 0.25.into()),
        (mixed_2, Complex64::new(0.0, 0.25)),
        (mixed_3, Complex64::new(0.0, -0.25)),
        (mixed_4, Complex64::new(0.25, 0.0)),
    ]) {
        assert_eq!(left, &right);
    }
}

// Test the hermitian_conjugate and is_natural_hermitian functions of the MixedPlusMinusProduct
#[test]
fn from_mixed_product_1() {
    let spins_1 = PauliProduct::from_str("0X").unwrap();
    let creators = &[0];
    let annihilators = &[3];
    let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let test_new = MixedProduct::new([spins_1], [bosons.clone()], [fermions.clone()]).unwrap();

    let spins_1_c = PlusMinusProduct::from_str("0+").unwrap();
    let spins_2_c = PlusMinusProduct::from_str("0-").unwrap();
    let mixed_1 = MixedPlusMinusProduct::new([spins_1_c], [bosons.clone()], [fermions.clone()]);
    let mixed_2 = MixedPlusMinusProduct::new([spins_2_c], [bosons], [fermions]);

    let res: Vec<(MixedPlusMinusProduct, Complex64)> = test_new.into();
    assert_eq!(res.len(), 2);
    for (left, right) in res.iter().zip(vec![
        (mixed_1, 1.0.into()),
        (mixed_2, Complex64::new(1.0, 0.0)),
    ]) {
        assert_eq!(left, &right);
    }
}

// Test the hermitian_conjugate and is_natural_hermitian functions of the MixedPlusMinusProduct
#[test]
fn from_mixed_product_2() {
    let spins_1 = PauliProduct::from_str("0X").unwrap();
    let spins_2 = PauliProduct::from_str("0Y").unwrap();
    let creators = &[0];
    let annihilators = &[3];
    let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let test_new =
        MixedProduct::new([spins_1, spins_2], [bosons.clone()], [fermions.clone()]).unwrap();

    let spins_1_c = PlusMinusProduct::from_str("0+").unwrap();
    let spins_2_c = PlusMinusProduct::from_str("0-").unwrap();
    let mixed_1 = MixedPlusMinusProduct::new(
        [spins_1_c.clone(), spins_1_c.clone()],
        [bosons.clone()],
        [fermions.clone()],
    );
    let mixed_2 = MixedPlusMinusProduct::new(
        [spins_1_c.clone(), spins_2_c.clone()],
        [bosons.clone()],
        [fermions.clone()],
    );
    let mixed_3 = MixedPlusMinusProduct::new(
        [spins_2_c.clone(), spins_1_c],
        [bosons.clone()],
        [fermions.clone()],
    );
    let mixed_4 = MixedPlusMinusProduct::new([spins_2_c.clone(), spins_2_c], [bosons], [fermions]);

    let res: Vec<(MixedPlusMinusProduct, Complex64)> = test_new.into();
    assert_eq!(res.len(), 4);
    for (left, right) in res.iter().zip(vec![
        (mixed_1, Complex64::new(0.0, -1.0)),
        (mixed_2, Complex64::new(0.0, 1.0)),
        (mixed_3, Complex64::new(0.0, -1.0)),
        (mixed_4, Complex64::new(0.0, 1.0)),
    ]) {
        assert_eq!(left, &right);
    }
}

// Test the hermitian_conjugate and is_natural_hermitian functions of the MixedPlusMinusProduct
#[test]
fn from_mixed_product_3() {
    let spins_1 = PauliProduct::from_str("0X").unwrap();
    let spins_2 = PauliProduct::from_str("0Z").unwrap();
    let spins_3 = PauliProduct::from_str("0Y").unwrap();
    let creators = &[0];
    let annihilators = &[3];
    let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let test_new = MixedProduct::new(
        [spins_1, spins_2, spins_3],
        [bosons.clone()],
        [fermions.clone()],
    )
    .unwrap();

    let spins_1_c = PlusMinusProduct::from_str("0+").unwrap();
    let spins_2_c = PlusMinusProduct::from_str("0Z").unwrap();
    let spins_3_c = PlusMinusProduct::from_str("0-").unwrap();
    let mixed_1 = MixedPlusMinusProduct::new(
        [spins_1_c.clone(), spins_2_c.clone(), spins_1_c.clone()],
        [bosons.clone()],
        [fermions.clone()],
    );
    let mixed_2 = MixedPlusMinusProduct::new(
        [spins_1_c.clone(), spins_2_c.clone(), spins_3_c.clone()],
        [bosons.clone()],
        [fermions.clone()],
    );
    let mixed_3 = MixedPlusMinusProduct::new(
        [spins_3_c.clone(), spins_2_c.clone(), spins_1_c],
        [bosons.clone()],
        [fermions.clone()],
    );
    let mixed_4 = MixedPlusMinusProduct::new(
        [spins_3_c.clone(), spins_2_c, spins_3_c],
        [bosons],
        [fermions],
    );

    let res: Vec<(MixedPlusMinusProduct, Complex64)> = test_new.into();
    assert_eq!(res.len(), 4);
    for (left, right) in res.iter().zip(vec![
        (mixed_1, Complex64::new(0.0, -1.0)),
        (mixed_2, Complex64::new(0.0, 1.0)),
        (mixed_3, Complex64::new(0.0, -1.0)),
        (mixed_4, Complex64::new(0.0, 1.0)),
    ]) {
        assert_eq!(left, &right);
    }
}

// Test the hermitian_conjugate and is_natural_hermitian functions of the MixedPlusMinusProduct
#[test]
fn from_mixed_product_longer() {
    let spins_1 = PauliProduct::from_str("2Z3Z").unwrap();
    let spins_2 = PauliProduct::from_str("1X2Y3Z").unwrap();
    let creators = &[0];
    let annihilators = &[3];
    let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let test_new =
        MixedProduct::new([spins_1, spins_2], [bosons.clone()], [fermions.clone()]).unwrap();

    let spins_1_c = PlusMinusProduct::from_str("2Z3Z").unwrap();
    let mixed_1 = MixedPlusMinusProduct::new(
        [
            spins_1_c.clone(),
            PlusMinusProduct::from_str("1+2+3Z").unwrap(),
        ],
        [bosons.clone()],
        [fermions.clone()],
    );
    let mixed_2 = MixedPlusMinusProduct::new(
        [
            spins_1_c.clone(),
            PlusMinusProduct::from_str("1-2+3Z").unwrap(),
        ],
        [bosons.clone()],
        [fermions.clone()],
    );
    let mixed_3 = MixedPlusMinusProduct::new(
        [
            spins_1_c.clone(),
            PlusMinusProduct::from_str("1+2-3Z").unwrap(),
        ],
        [bosons.clone()],
        [fermions.clone()],
    );
    let mixed_4 = MixedPlusMinusProduct::new(
        [spins_1_c, PlusMinusProduct::from_str("1-2-3Z").unwrap()],
        [bosons],
        [fermions],
    );

    let res: Vec<(MixedPlusMinusProduct, Complex64)> = test_new.into();
    assert_eq!(res.len(), 4);
    for (left, right) in res.iter().zip(vec![
        (mixed_1, Complex64::new(0.0, -1.0)),
        (mixed_2, Complex64::new(0.0, -1.0)),
        (mixed_3, Complex64::new(0.0, 1.0)),
        (mixed_4, Complex64::new(0.0, 1.0)),
    ]) {
        assert_eq!(left, &right);
    }
}

// Test the hermitian_conjugate and is_natural_hermitian functions of the MixedPlusMinusProduct
#[test]
fn number_particles() {
    let spins = PlusMinusProduct::from_str("0+").unwrap();
    let creators = &[0];
    let annihilators = &[3];
    let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let test_new = MixedPlusMinusProduct::new(
        [spins, PlusMinusProduct::new()],
        [BosonProduct::new([], []).unwrap(), bosons],
        [fermions],
    );

    assert_eq!(test_new.current_number_spins(), vec![1, 0]);
    assert_eq!(test_new.current_number_bosonic_modes(), vec![0, 4]);
    assert_eq!(test_new.current_number_fermionic_modes(), vec![4]);
}

// Test the Hash, Debug and Display traits of PlusMinusProduct
#[test]
fn debug() {
    let spins = PlusMinusProduct::from_str("0+1-2Z").unwrap();
    let b_creators = &[0, 1, 1];
    let b_annihilators = &[3, 3, 5];
    let f_creators = &[0, 1, 2];
    let f_annihilators = &[3, 4, 5];
    let bosons = BosonProduct::new(b_creators.to_vec(), b_annihilators.to_vec()).unwrap();
    let fermions = FermionProduct::new(f_creators.to_vec(), f_annihilators.to_vec()).unwrap();
    let test_new = MixedPlusMinusProduct::new([spins], [bosons], [fermions]);

    assert_eq!(
        format!("{:?}", test_new),
        "MixedPlusMinusProduct { spins: [PlusMinusProduct { items: [(0, Plus), (1, Minus), (2, Z)] }], bosons: [BosonProduct { creators: [0, 1, 1], annihilators: [3, 3, 5] }], fermions: [FermionProduct { creators: [0, 1, 2], annihilators: [3, 4, 5] }] }"
    );
    assert_eq!(
        format!("{}", test_new),
        "S0+1-2Z:Bc0c1c1a3a3a5:Fc0c1c2a3a4a5:"
    );
}

// Test the Hash, Debug and Display traits of PlusMinusProduct
#[test]
fn hash_debug() {
    let spins = PlusMinusProduct::from_str("0+1-2Z").unwrap();
    let b_creators = &[0, 1, 1];
    let b_annihilators = &[3, 3, 5];
    let f_creators = &[0, 1, 2];
    let f_annihilators = &[3, 4, 5];
    let bosons = BosonProduct::new(b_creators.to_vec(), b_annihilators.to_vec()).unwrap();
    let fermions = FermionProduct::new(f_creators.to_vec(), f_annihilators.to_vec()).unwrap();
    let test_new =
        MixedPlusMinusProduct::new([spins.clone()], [bosons.clone()], [fermions.clone()]);
    let test_new_1 = MixedPlusMinusProduct::new([spins], [bosons], [fermions]);

    let mut s_1 = DefaultHasher::new();
    test_new.hash(&mut s_1);
    let mut s_2 = DefaultHasher::new();
    test_new_1.hash(&mut s_2);
    assert_eq!(s_1.finish(), s_2.finish())
}

// Test the Clone, PartialEq, PartialOrd and Ord traits of PlusMinusProduct
#[test]
fn clone_partial_eq_partial_ord() {
    let spins = PlusMinusProduct::from_str("0+1-2Z").unwrap();
    let b_creators = &[0, 1, 1];
    let b_annihilators = &[3, 3, 5];
    let f_creators = &[0, 1, 2];
    let f_annihilators = &[3, 4, 5];
    let bosons = BosonProduct::new(b_creators.to_vec(), b_annihilators.to_vec()).unwrap();
    let fermions = FermionProduct::new(f_creators.to_vec(), f_annihilators.to_vec()).unwrap();
    let test_new =
        MixedPlusMinusProduct::new([spins.clone()], [bosons.clone()], [fermions.clone()]);

    // Test Clone trait
    assert_eq!(test_new.clone(), test_new);

    // Test PartialEq trait
    let test_0 = MixedPlusMinusProduct::new([spins.clone()], [bosons.clone()], [fermions.clone()]);
    let test_1 = MixedPlusMinusProduct::new(
        [PlusMinusProduct::from_str("").unwrap()],
        [bosons.clone()],
        [fermions.clone()],
    );
    assert!(test_0 == test_new);
    assert!(test_new == test_0);
    assert!(test_1 != test_new);
    assert!(test_new != test_1);

    // Test PartialOrd trait
    let test_0 = MixedPlusMinusProduct::new([spins], [bosons.clone()], [fermions.clone()]);
    let test_1 = MixedPlusMinusProduct::new(
        [PlusMinusProduct::from_str("").unwrap()],
        [bosons],
        [fermions],
    );

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
    let spins = PlusMinusProduct::from_str("0+").unwrap();
    let creators = &[0];
    let annihilators = &[3];
    let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let test_new = MixedPlusMinusProduct::new([spins], [bosons], [fermions]);

    let serialized = serde_json::to_string(&test_new).unwrap();
    let deserialized: MixedPlusMinusProduct = serde_json::from_str(&serialized).unwrap();

    assert_eq!(test_new, deserialized);
    assert_tokens(&test_new.readable(), &[Token::String("S0+:Bc0a3:Fc0a3:")]);
}

#[test]
fn serde_compact() {
    let spins = PlusMinusProduct::from_str("0+").unwrap();
    let creators = &[0];
    let annihilators = &[3];
    let bosons = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let fermions = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let test_new = MixedPlusMinusProduct::new([spins], [bosons], [fermions]);

    let serialized = serde_json::to_string(&test_new).unwrap();
    let deserialized: MixedPlusMinusProduct = serde_json::from_str(&serialized).unwrap();

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
                name: "SinglePlusMinusOperator",
                variant: "Plus",
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
fn test_mixed_plus_minus_product_schema() {
    let pp = MixedPlusMinusProduct::new(
        [PlusMinusProduct::new().plus(0), PlusMinusProduct::new()],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    );
    let schema = schemars::schema_for!(MixedPlusMinusProduct);
    let schema_checker = jsonschema::JSONSchema::compile(&serde_json::to_value(&schema).unwrap())
        .expect("schema is valid");
    let value = serde_json::to_value(pp).unwrap();
    let validation = schema_checker.validate(&value);
    assert!(validation.is_ok());
}

#[cfg(feature = "struqture_1_import")]
#[cfg(feature = "struqture_1_export")]
#[test]
fn test_from_to_struqture_1() {
    let pp_1 = struqture_1::mixed_systems::MixedPlusMinusProduct::new(
        [struqture_1::spins::PlusMinusProduct::from_str("0+").unwrap()],
        [struqture_1::bosons::BosonProduct::from_str("c0a1").unwrap()],
        [struqture_1::fermions::FermionProduct::from_str("c0a0").unwrap()],
    );
    let pp_2 = MixedPlusMinusProduct::new(
        [PlusMinusProduct::new().plus(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [0]).unwrap()],
    );
    assert!(MixedPlusMinusProduct::from_struqture_1(&pp_1).unwrap() == pp_2);
    assert!(pp_1 == pp_2.to_struqture_1().unwrap());
}
