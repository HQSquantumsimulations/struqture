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

//! Integration test for public API of bosonic indices

use crate::BosonTinyVec;
use qoqo_calculator::CalculatorComplex;
use serde_test::{assert_tokens, Configure, Token};
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use struqture::bosons::*;
use struqture::prelude::*;
use struqture::{CorrespondsTo, GetValue, StruqtureError};
use test_case::test_case;
use tinyvec::tiny_vec;
use tinyvec::TinyVec;

#[test]
fn default() {
    assert_eq!(
        BosonProduct::default(),
        BosonProduct::new(vec![], vec![]).unwrap()
    );
}

#[test]
fn test_remap_modes_passing() {
    let bp = BosonProduct::new([0, 1], []).unwrap();
    let reordering_dictionary = HashMap::from([(0, 1), (1, 0)]);
    let (remapped_bp, coeff) = bp.remap_modes(&reordering_dictionary).unwrap();

    assert_eq!(remapped_bp, bp);
    assert_eq!(coeff, 1.0.into());

    let bp = BosonProduct::new([0, 2], [1]).unwrap();
    let reordering_dictionary = HashMap::from([(0, 2), (1, 0), (2, 1)]);
    let (remapped_bp, coeff) = bp.remap_modes(&reordering_dictionary).unwrap();
    let expected_bp = BosonProduct::new([1, 2], [0]).unwrap();

    assert_eq!(remapped_bp, expected_bp);
    assert_eq!(coeff, 1.0.into());

    let bp = BosonProduct::new([0, 2], [1]).unwrap();
    let reordering_dictionary = HashMap::from([(0, 2), (2, 0)]);
    let (remapped_bp, coeff) = bp.remap_modes(&reordering_dictionary).unwrap();
    let expected_bp = BosonProduct::new([0, 2], [1]).unwrap();

    assert_eq!(remapped_bp, expected_bp);
    assert_eq!(coeff, 1.0.into());
}

#[test_case(&[(0, 1), (1, 3), (2, 1)])]
#[test_case(&[(0, 1), (2, 3)])]
fn test_remap_modes_error(remap_dict: &[(usize, usize)]) {
    let bp = BosonProduct::new([0, 2], [1, 3]).unwrap();
    let reordering_dictionary: HashMap<usize, usize> = remap_dict.iter().cloned().collect();
    let err = bp.remap_modes(&reordering_dictionary);

    assert!(err.is_err())
}

#[test_case(&[], &[], 0, 0, 0; "empty")]
#[test_case(&[0], &[1], 1, 1, 2; "0 - 1")]
#[test_case(&[1], &[], 1, 0, 2; "1 - empty")]
#[test_case(&[], &[2000], 0, 1, 2001; "empty - 2000")]
#[test_case(&[0,1,1], &[3,3,5], 3, 3, 6; "0,1,1 - 3,3,5")]
fn new_normal_ordered_passing(
    creators: &[usize],
    annihilators: &[usize],
    n_creators: usize,
    n_annihilators: usize,
    n_modes: usize,
) {
    let creators = creators.to_vec();
    let annihilators = annihilators.to_vec();
    let test_new = BosonProduct::new(creators.clone(), annihilators.clone());
    assert!(test_new.is_ok());
    let res = test_new.unwrap();
    if creators.is_empty() && annihilators.is_empty() {
        assert!(res.is_natural_hermitian());
    } else {
        assert!(!res.is_natural_hermitian());
    }
    assert_eq!(res, res.corresponds_to());
    assert_eq!(res.number_creators(), n_creators);
    assert_eq!(res.number_annihilators(), n_annihilators);
    assert_eq!(res.current_number_modes(), n_modes);
    let cvec: Vec<usize> = res.creators().copied().collect();
    let avec: Vec<usize> = res.annihilators().copied().collect();
    assert_eq!(cvec, creators);
    assert_eq!(avec, annihilators);

    let (res, prefac) =
        BosonProduct::create_valid_pair(creators.clone(), annihilators.clone(), 1.0.into())
            .unwrap();
    assert_eq!(prefac, CalculatorComplex::from(1.0));
    let cvec: Vec<usize> = res.creators().copied().collect();
    let avec: Vec<usize> = res.annihilators().copied().collect();
    assert_eq!(cvec, creators);
    assert_eq!(avec, annihilators);
}

#[test_case(&[2,1], &[1,2], &[1,2], &[1,2]; "2,1 - 1,2")]
#[test_case(&[0], &[30, 0], &[0], &[0, 30]; "0 - 1")]
fn new_normal_ordered_resorting(
    creators: &[usize],
    annihilators: &[usize],
    creators_sorted: &[usize],
    annihilators_sorted: &[usize],
) {
    let creators = creators.to_vec();
    let annihilators = annihilators.to_vec();
    let test_new = BosonProduct::new(creators, annihilators);
    assert!(test_new.is_ok());
    let res = test_new.unwrap();
    let cvec: Vec<usize> = res.creators().copied().collect();
    let avec: Vec<usize> = res.annihilators().copied().collect();
    assert_eq!(cvec, creators_sorted);
    assert_eq!(avec, annihilators_sorted);
}

#[test_case("", &[], &[]; "empty")]
#[test_case("c0a1",&[0], &[1]; "0 - 1")]
#[test_case("c1",&[1], &[]; "1 - empty")]
#[test_case("a2000", &[], &[2000]; "empty - 2000")]
#[test_case("c0c1c1a3a3a5",&[0,1,1], &[3,3,5]; "0,1,1 - 3,3,5")]
#[test_case("c2c1a1a2", &[1,2], &[1,2]; "2,1 - 1,2")]
#[test_case("c0a30a0", &[0], &[0, 30]; "0 - 0,30")]
fn from_string(stringformat: &str, creators_sorted: &[usize], annihilators_sorted: &[usize]) {
    let test_new = <BosonProduct as std::str::FromStr>::from_str(stringformat);
    assert!(test_new.is_ok());
    let res = test_new.unwrap();
    let cvec: Vec<usize> = res.creators().copied().collect();
    let avec: Vec<usize> = res.annihilators().copied().collect();
    assert_eq!(cvec, creators_sorted);
    assert_eq!(avec, annihilators_sorted);
}

#[test]
fn from_string_fail() {
    let test_new = <BosonProduct as std::str::FromStr>::from_str("c0a1c2a3");
    assert!(test_new.is_err());
    assert_eq!(
        test_new,
        Err(StruqtureError::IndicesNotNormalOrdered {
            index_i: 2,
            index_j: 3
        })
    );

    let test_new = <BosonProduct as std::str::FromStr>::from_str("c0a1b2");
    assert!(test_new.is_err());
    assert_eq!(
        test_new,
        Err(StruqtureError::FromStringFailed {
            msg: "Used operator b that is neither 'c' nor 'a' in BosonProduct::from_str".into()
        })
    );

    let test_new = <BosonProduct as std::str::FromStr>::from_str("c0a#");
    assert!(test_new.is_err());
    assert_eq!(
        test_new,
        Err(StruqtureError::FromStringFailed {
            msg: "Index in given creators or annihilators is not an integer: #".into()
        })
    );
}

#[test_case( &[], &[]; "empty")]
#[test_case(&[0], &[1]; "0 - 1")]
#[test_case(&[1], &[]; "1 - empty")]
#[test_case( &[], &[2000]; "empty - 2000")]
#[test_case(&[0,1,1], &[3,3,5]; "0,1,1 - 3,3,5")]
#[test_case( &[1,2], &[1,2]; "2,1 - 1,2")]
#[test_case( &[0], &[0, 30]; "0 - 0,30")]
fn from_string_import_export(creators: &[usize], annihilators: &[usize]) {
    let creators = creators.to_vec();
    let annihilators = annihilators.to_vec();
    let test = BosonProduct::new(creators, annihilators).unwrap();
    let stringformat = format!("{}", test);
    let test_new = <BosonProduct as std::str::FromStr>::from_str(&stringformat);
    assert!(test_new.is_ok());
    assert_eq!(test, test_new.unwrap());
}

#[test_case( &[], &[]; "empty")]
#[test_case(&[0], &[1]; "0 - 1")]
#[test_case(&[1], &[]; "1 - empty")]
#[test_case( &[], &[2000]; "empty - 2000")]
#[test_case(&[0,1,1], &[3,3,5]; "0,1,1 - 3,3,5")]
#[test_case( &[1,2], &[1,2]; "2,1 - 1,2")]
#[test_case( &[0], &[0, 30]; "0 - 0,30")]
fn test_serde_json(creators: &[usize], annihilators: &[usize]) {
    let creators = creators.to_vec();
    let annihilators = annihilators.to_vec();
    let test = BosonProduct::new(creators, annihilators).unwrap();
    let serialized = serde_json::to_string(&test).unwrap();
    let deserialized: BosonProduct = serde_json::from_str(&serialized).unwrap();
    assert_eq!(test, deserialized);
}

#[test_case( &[], &[], Token::Str("I"); "empty")]
#[test_case(&[0], &[1], Token::Str("c0a1"); "0 - 1")]
#[test_case(&[1], &[], Token::Str("c1"); "1 - empty")]
#[test_case( &[], &[2000], Token::Str("a2000"); "empty - 2000")]
#[test_case(&[0,1,1], &[3,3,5], Token::Str("c0c1c1a3a3a5"); "0,1,1 - 3,3,5")]
#[test_case( &[1,2], &[1,2], Token::Str("c1c2a1a2"); "2,1 - 1,2")]
#[test_case( &[0], &[0, 30], Token::Str("c0a0a30"); "0 - 0,30")]
fn serde_readable(creators: &[usize], annihilators: &[usize], result: Token) {
    let creators = creators.to_vec();
    let annihilators = annihilators.to_vec();
    let test = BosonProduct::new(creators, annihilators).unwrap();

    assert_tokens(&test.readable(), &[result]);
}

#[test_case( &[], &[]; "empty")]
#[test_case(&[0], &[1]; "0 - 1")]
#[test_case(&[1], &[]; "1 - empty")]
#[test_case( &[], &[2000]; "empty - 2000")]
#[test_case(&[0,1,1], &[3,3,5]; "0,1,1 - 3,3,5")]
#[test_case( &[1,2], &[1,2]; "2,1 - 1,2")]
#[test_case( &[0], &[0, 30]; "0 - 0,30")]
fn test_bincode(creators: &[usize], annihilators: &[usize]) {
    let creators = creators.to_vec();
    let annihilators = annihilators.to_vec();
    let test = BosonProduct::new(creators, annihilators).unwrap();

    let serialized = bincode::serialize(&test).unwrap();
    let deserialized: BosonProduct = bincode::deserialize(&serialized).unwrap();
    assert_eq!(test, deserialized);

    let serialized = bincode::serialize(&test.clone().compact()).unwrap();
    let deserialized: BosonProduct = bincode::deserialize(&serialized).unwrap();
    assert_eq!(test, deserialized);
}

#[test_case(&[1], &[]; "1 - empty")]
fn serde_compact_creators(creators: &[usize], annihilators: &[usize]) {
    let creators = creators.to_vec();
    let annihilators = annihilators.to_vec();
    let test = BosonProduct::new(creators, annihilators).unwrap();

    assert_tokens(
        &test.compact(),
        &[
            Token::Tuple { len: 2 },
            Token::Seq { len: Some(1) },
            Token::U64(1),
            Token::SeqEnd,
            Token::Seq { len: Some(0) },
            Token::SeqEnd,
            Token::TupleEnd,
        ],
    );
}

#[test_case( &[], &[2000]; "empty - 2000")]
fn serde_compact_annihilators(creators: &[usize], annihilators: &[usize]) {
    let creators = creators.to_vec();
    let annihilators = annihilators.to_vec();
    let test = BosonProduct::new(creators, annihilators).unwrap();

    assert_tokens(
        &test.compact(),
        &[
            Token::Tuple { len: 2 },
            Token::Seq { len: Some(0) },
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::U64(2000),
            Token::SeqEnd,
            Token::TupleEnd,
        ],
    );
}

#[test_case(&[0,1,1], &[3,3,5]; "0,1,1 - 3,3,5")]
fn serde_compact_both(creators: &[usize], annihilators: &[usize]) {
    let creators = creators.to_vec();
    let annihilators = annihilators.to_vec();
    let test = BosonProduct::new(creators, annihilators).unwrap();

    assert_tokens(
        &test.compact(),
        &[
            Token::Tuple { len: 2 },
            Token::Seq { len: Some(3) },
            Token::U64(0),
            Token::U64(1),
            Token::U64(1),
            Token::SeqEnd,
            Token::Seq { len: Some(3) },
            Token::U64(3),
            Token::U64(3),
            Token::U64(5),
            Token::SeqEnd,
            Token::TupleEnd,
        ],
    );
}

// Test the set_pauli and get functions of the PauliProduct
#[test]
fn hermitian_conjugate() {
    let creators = &[0];
    let annihilators = &[0];
    let bp = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();

    assert_eq!(bp.hermitian_conjugate(), (bp, 1.0));

    let creators = &[0];
    let annihilators = &[];
    let bp = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let creators_h = &[];
    let annihilators_h = &[0];
    let bp_h = BosonProduct::new(creators_h.to_vec(), annihilators_h.to_vec()).unwrap();

    assert_eq!(bp.hermitian_conjugate(), (bp_h, 1.0));

    let creators = &[];
    let annihilators = &[0];
    let bp = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let creators_h = &[0];
    let annihilators_h = &[];
    let bp_h = BosonProduct::new(creators_h.to_vec(), annihilators_h.to_vec()).unwrap();

    assert_eq!(bp.hermitian_conjugate(), (bp_h, 1.0));

    let creators = &[0, 1];
    let annihilators = &[0, 3];
    let bp = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let creators_h = &[0, 3];
    let annihilators_h = &[0, 1];
    let bp_h = BosonProduct::new(creators_h.to_vec(), annihilators_h.to_vec()).unwrap();

    assert_eq!(bp.hermitian_conjugate(), (bp_h, 1.0));
}

// Test the set_pauli and get functions of the PauliProduct
#[test]
fn get_value_get_transform_bp() {
    let creators = &[0];
    let annihilators = &[0];
    let bp = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();

    assert_eq!(BosonProduct::get_key(&bp), bp);
    assert_eq!(
        BosonProduct::get_transform(&bp, CalculatorComplex::new(1.0, 2.0)),
        CalculatorComplex::new(1.0, 2.0)
    );
}

// Test the set_pauli and get functions of the PauliProduct
#[test]
fn get_value_get_transform_hbp() {
    let creators = &[0];
    let annihilators = &[0];
    let bp = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let hbp = HermitianBosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();

    assert_eq!(BosonProduct::get_key(&hbp), bp);
    assert_eq!(
        BosonProduct::get_transform(&hbp, CalculatorComplex::new(1.0, 2.0)),
        CalculatorComplex::new(1.0, 2.0)
    );
}

#[test_case(tiny_vec!([usize; 2] => 0, 2, 4), tiny_vec!([usize; 2] => 1, 3, 5),
     vec![(tiny_vec!([usize; 2] => 1, 3, 5), tiny_vec!([usize; 2] => 0, 2, 4))]; "0,2,4 - 1,3,5")]
#[test_case(tiny_vec!([usize; 2] => 0), tiny_vec!([usize; 2] => 0),
     vec![(tiny_vec!([usize; 2] => 0), tiny_vec!([usize; 2] => 0)), (tiny_vec!([usize; 2]), tiny_vec!([usize; 2]))]; "0, - 0")]
#[test_case(tiny_vec!([usize; 2] => 1,20), tiny_vec!([usize; 2] => 1,30),
     vec![(tiny_vec!([usize; 2] => 1,30), tiny_vec!([usize; 2] => 1,20)), (tiny_vec!([usize; 2] => 30), tiny_vec!([usize; 2] => 20))]; "1,20 - 1,30")]
#[test_case(tiny_vec!([usize; 2] => 10,10,20), tiny_vec!([usize; 2] => 10,30),
     vec![(tiny_vec!([usize; 2] => 10,30), tiny_vec!([usize; 2] => 10, 10,20)), (tiny_vec!([usize; 2] => 30), tiny_vec!([usize; 2] => 10, 20)), (tiny_vec!([usize; 2] => 30), tiny_vec!([usize; 2] => 10, 20))]; "10, 10, 20 - 10,30")]
#[test_case(tiny_vec!([usize; 2] => 10,20,30), tiny_vec!([usize; 2] => 10,30),
     vec![(tiny_vec!([usize; 2]), tiny_vec!([usize; 2] => 20)), (tiny_vec!([usize; 2] => 30), tiny_vec!([usize; 2] => 20, 30)), (tiny_vec!([usize; 2] => 10), tiny_vec!([usize; 2] => 10, 20)), (tiny_vec!([usize; 2] => 10, 30), tiny_vec!([usize; 2] => 10, 20, 30))]; "10, 20, 30 - 10,30")]
#[test_case(tiny_vec!([usize; 2] => 10,20), tiny_vec!([usize; 2] => 10),
     vec![(tiny_vec!([usize; 2]), tiny_vec!([usize; 2] => 20)), (tiny_vec!([usize; 2] => 10), tiny_vec!([usize; 2] => 10, 20))]; "100, 20, 30 - 10,30")]
fn multiply(
    annihilators_left: TinyVec<[usize; 2]>,
    creators_right: TinyVec<[usize; 2]>,
    expected: Vec<(BosonTinyVec, BosonTinyVec)>,
) {
    let annihilators_right: TinyVec<[usize; 2]> = tiny_vec![78, 43];
    let creators_left: TinyVec<[usize; 2]> = tiny_vec![84, 95];
    let left = BosonProduct::new(creators_left.clone(), annihilators_left).unwrap();
    let right = BosonProduct::new(creators_right, annihilators_right.clone()).unwrap();
    let mut result_list: Vec<BosonProduct> = Vec::new();
    for mut pair in expected {
        pair.0.extend(creators_left.iter().copied());
        pair.1.extend(annihilators_right.iter().copied());
        result_list.push(BosonProduct::new(pair.0, pair.1).unwrap());
    }
    let result = left * right;

    assert_eq!(result.len(), result_list.len());
    for res in result_list {
        assert!(result.contains(&res));
    }
}

#[test]
fn multiply_list_right() {
    let annihilators_right: TinyVec<[usize; 2]> = tiny_vec![78, 43];
    let creators_left: TinyVec<[usize; 2]> = tiny_vec![84, 95];
    let left = BosonProduct::new(creators_left.clone(), tiny_vec!([usize; 2] => 1,20)).unwrap();
    let mut right: Vec<BosonProduct> = Vec::new();
    right.push(
        BosonProduct::new(tiny_vec!([usize; 2] => 1,30), annihilators_right.clone()).unwrap(),
    );
    right.push(BosonProduct::new(tiny_vec!([usize; 2] => 0), annihilators_right.clone()).unwrap());
    let mut result_list: Vec<BosonProduct> = Vec::new();
    let expected = vec![
        (tiny_vec!([usize; 2] => 1,30), tiny_vec!([usize; 2] => 1,20)),
        (tiny_vec!([usize; 2] => 30), tiny_vec!([usize; 2] => 20)),
        (tiny_vec!([usize; 2] => 0), tiny_vec!([usize; 2] => 1,20)),
    ];
    for mut pair in expected {
        pair.0.extend(creators_left.iter().copied());
        pair.1.extend(annihilators_right.iter().copied());
        result_list.push(BosonProduct::new(pair.0, pair.1).unwrap());
    }
    let result = left * right;

    assert_eq!(result.len(), result_list.len());
    for res in result_list {
        assert!(result.contains(&res));
    }
}

#[test]
fn multiply_list_left() {
    let annihilators_right: TinyVec<[usize; 2]> = tiny_vec![78, 43];
    let creators_left: TinyVec<[usize; 2]> = tiny_vec![84, 95];
    let right =
        BosonProduct::new(tiny_vec!([usize; 2] => 1,20), annihilators_right.clone()).unwrap();
    let mut left: Vec<BosonProduct> = Vec::new();
    left.push(BosonProduct::new(creators_left.clone(), tiny_vec!([usize; 2] => 1,30)).unwrap());
    left.push(BosonProduct::new(creators_left.clone(), tiny_vec!([usize; 2] => 0)).unwrap());
    let mut result_list: Vec<BosonProduct> = Vec::new();
    let expected = vec![
        (tiny_vec!([usize; 2] => 1,20), tiny_vec!([usize; 2] => 1,30)),
        (tiny_vec!([usize; 2] => 20), tiny_vec!([usize; 2] => 30)),
        (tiny_vec!([usize; 2] => 1,20), tiny_vec!([usize; 2] => 0)),
    ];
    for mut pair in expected {
        pair.0.extend(creators_left.iter().copied());
        pair.1.extend(annihilators_right.iter().copied());
        result_list.push(BosonProduct::new(pair.0, pair.1).unwrap());
    }
    let result = left * right;

    assert_eq!(result.len(), result_list.len());
    for res in result_list {
        assert!(result.contains(&res));
    }
}
// Test the Hash, Debug and Display traits of PauliProduct
#[test]
fn hash_debug() {
    let creators = &[0];
    let annihilators = &[3];
    let bp = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();

    assert_eq!(
        format!("{:?}", bp),
        "BosonProduct { creators: [0], annihilators: [3] }"
    );
    assert_eq!(format!("{}", bp), "c0a3");

    let creators = &[0];
    let annihilators = &[3];
    let bp_1 = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let creators = &[0];
    let annihilators = &[3];
    let bp_2 = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();

    let mut s_1 = DefaultHasher::new();
    bp_1.hash(&mut s_1);
    let mut s_2 = DefaultHasher::new();
    bp_2.hash(&mut s_2);
    assert_eq!(s_1.finish(), s_2.finish())
}

// Test the Clone, PartialEq, PartialOrd and Ord traits of PauliProduct
#[test]
fn clone_partial_eq_partial_ord() {
    let creators = &[0];
    let annihilators = &[3];
    let bp = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();

    // Test Clone trait
    assert_eq!(bp.clone(), bp);

    // Test PartialEq trait
    let creators = &[0];
    let annihilators = &[3];
    let bp_0 = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let creators = &[1];
    let annihilators = &[3];
    let bp_1 = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    assert!(bp_0 == bp);
    assert!(bp == bp_0);
    assert!(bp_1 != bp);
    assert!(bp != bp_1);

    // Test PartialOrd trait
    let creators = &[0];
    let annihilators = &[3];
    let bp_0 = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let creators = &[1];
    let annihilators = &[3];
    let bp_1 = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();

    assert_eq!(bp_0.partial_cmp(&bp), Some(Ordering::Equal));
    assert_eq!(bp.partial_cmp(&bp_0), Some(Ordering::Equal));
    assert_eq!(bp_1.partial_cmp(&bp), Some(Ordering::Greater));
    assert_eq!(bp.partial_cmp(&bp_1), Some(Ordering::Less));

    // Test Ord trait
    assert_eq!(bp_0.cmp(&bp), Ordering::Equal);
    assert_eq!(bp.cmp(&bp_0), Ordering::Equal);
    assert_eq!(bp_1.cmp(&bp), Ordering::Greater);
    assert_eq!(bp.cmp(&bp_1), Ordering::Less);
}

#[cfg(feature = "json_schema")]
#[test]
fn test_boson_product_schema() {
    let pp = BosonProduct::new([0], [0]).unwrap();
    let schema = schemars::schema_for!(BosonProduct);
    let schema_checker = jsonschema::JSONSchema::compile(&serde_json::to_value(&schema).unwrap())
        .expect("schema is valid");
    let value = serde_json::to_value(pp).unwrap();
    let validation = schema_checker.validate(&value);
    assert!(validation.is_ok());
}
