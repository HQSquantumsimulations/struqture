// Copyright © 2021-2022 HQS Quantum Simulations GmbH. All Rights Reserved.
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

//! Integration test for public API of fermionic indices

// use crate::FermionTinyVec;
use qoqo_calculator::CalculatorComplex;
use serde_test::{assert_tokens, Configure, Token};
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use struqture::fermions::*;
use struqture::prelude::*;
use struqture::{CorrespondsTo, GetValue, StruqtureError};
use test_case::test_case;
use tinyvec::tiny_vec;
use tinyvec::TinyVec;

#[test]
fn default() {
    assert_eq!(
        HermitianFermionProduct::default(),
        HermitianFermionProduct::new(vec![], vec![]).unwrap()
    );
}

#[test]
fn new_error() {
    let error = HermitianFermionProduct::new(vec![43], vec![9]);
    assert!(error.is_err());
    assert_eq!(
        error,
        Err(StruqtureError::CreatorsAnnihilatorsMinimumIndex {
            creators_min: Some(43),
            annihilators_min: Some(9)
        })
    );
}

#[test_case(&[], &[], 0, 0, 0; "empty")]
#[test_case(&[0], &[1], 1, 1, 2; "0 - 1")]
#[test_case(&[0,1], &[0,2], 2, 2, 3; "0,1 - 0,2")]
#[test_case(&[], &[2000], 0, 1, 2001; "empty - 2000")]
fn new_normal_ordered_passing(
    creators: &[usize],
    annihilators: &[usize],
    n_creators: usize,
    n_annihilators: usize,
    n_modes: usize,
) {
    let creators = creators.to_vec();
    let annihilators = annihilators.to_vec();
    let test_new = HermitianFermionProduct::new(creators.clone(), annihilators.clone());
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

    let (res, prefac) = HermitianFermionProduct::create_valid_pair(
        creators.clone(),
        annihilators.clone(),
        CalculatorComplex::new(1.0, 2.0),
    )
    .unwrap();
    assert_eq!(prefac, CalculatorComplex::new(1.0, 2.0));
    let cvec: Vec<usize> = res.creators().copied().collect();
    let avec: Vec<usize> = res.annihilators().copied().collect();
    assert_eq!(cvec, creators);
    assert_eq!(avec, annihilators);

    let (res, prefac) = HermitianFermionProduct::create_valid_pair(
        annihilators.clone(),
        creators.clone(),
        CalculatorComplex::new(1.0, 2.0),
    )
    .unwrap();
    if creators.is_empty() && annihilators.is_empty() {
        assert_eq!(prefac, CalculatorComplex::new(1.0, 2.0));
    } else {
        assert_eq!(prefac, CalculatorComplex::new(1.0, -2.0));
    }
    let cvec: Vec<usize> = res.creators().copied().collect();
    let avec: Vec<usize> = res.annihilators().copied().collect();
    assert_eq!(cvec, creators);
    assert_eq!(avec, annihilators);
}

#[test_case(&[2,1], &[1,2]; "2,1 - 1,2")]
#[test_case(&[0], &[30, 0]; "0 - 1")]
fn new_normal_ordered_normal_order_error(creators: &[usize], annihilators: &[usize]) {
    let creators = creators.to_vec();
    let annihilators = annihilators.to_vec();
    let test_new = HermitianFermionProduct::new(creators, annihilators);
    assert!(test_new.is_err());
    assert_eq!(test_new, Err(StruqtureError::IncorrectlyOrderedIndices));
}

#[test_case(&[1], &[0], 1.0,3.0, true; "conjugate")]
#[test_case(&[0], &[1], 1.0,3.0, false; "not conjugate")]
#[test_case(&[0,2], &[0,1], 1.0,3.0, true; " conjugate second")]
#[test_case(&[2000], &[], 1.0,3.0, true; "empty 2000")]
#[test_case(&[0,2000], &[0], 1.0,3.0, true; "0 2000")]
#[test_case(&[0,2000], &[0,3000], 1.0,3.0, false; "0 3000")]

fn test_conjugation_in_valid_product(
    creators: &[usize],
    annihilators: &[usize],
    real: f64,
    imag: f64,
    conjugated: bool,
) {
    let creators = creators.to_vec();
    let annihilators = annihilators.to_vec();
    let (res, prefac) = HermitianFermionProduct::create_valid_pair(
        creators.clone(),
        annihilators.clone(),
        CalculatorComplex::new(real, imag),
    )
    .unwrap();
    if conjugated {
        assert_eq!(prefac, CalculatorComplex::new(real, -imag));
        assert_eq!(
            res,
            HermitianFermionProduct::new(annihilators, creators).unwrap()
        );
    } else {
        assert_eq!(prefac, CalculatorComplex::new(real, imag));
        assert_eq!(
            res,
            HermitianFermionProduct::new(creators, annihilators).unwrap()
        );
    }
}

#[test_case(&[1], &[]; "1-empty")]
#[test_case(&[2], &[1]; "2 - 1")]
#[test_case(&[0,2], &[0,1]; "0,2 - 0,1")]
#[test_case(&[0,1], &[0]; "0,1 - 0,empty")]

fn hermitian_error(creators: &[usize], annihilators: &[usize]) {
    let creators = creators.to_vec();
    let annihilators = annihilators.to_vec();
    let test_new = HermitianFermionProduct::new(creators, annihilators);
    assert!(test_new.is_err());
}

#[test_case(&[1, 2, 2], &[1, 2]; "creators")]
#[test_case(&[0, 1], &[0, 1, 1]; "annihilators")]
fn new_normal_ordered_doubles_error(creators: &[usize], annihilators: &[usize]) {
    let creators = creators.to_vec();
    let annihilators = annihilators.to_vec();
    let test_new = HermitianFermionProduct::new(creators, annihilators);
    assert!(test_new.is_err());
    assert_eq!(test_new, Err(StruqtureError::IncorrectlyOrderedIndices {}));
}

#[test_case("", &[], &[]; "empty")]
#[test_case("c0a1",&[0], &[1]; "0 - 1")]
#[test_case("a2000", &[], &[2000]; "empty - 2000")]
#[test_case("c0c1c2a3a4a5",&[0,1,2], &[3,4,5]; "0,1,2 - 3,4,5")]
#[test_case("c1c2a1a2", &[1,2], &[1,2]; "2,1 - 1,2")]
#[test_case("c0a0a30", &[0], &[0, 30]; "0 - 0,30")]
fn from_string(stringformat: &str, creators_sorted: &[usize], annihilators_sorted: &[usize]) {
    let test_new = <HermitianFermionProduct as std::str::FromStr>::from_str(stringformat);
    assert!(test_new.is_ok());
    let res = test_new.unwrap();
    let cvec: Vec<usize> = res.creators().copied().collect();
    let avec: Vec<usize> = res.annihilators().copied().collect();
    assert_eq!(cvec, creators_sorted);
    assert_eq!(avec, annihilators_sorted);
}

#[test]
fn from_string_fail() {
    let test_new = <HermitianFermionProduct as std::str::FromStr>::from_str("c0a1c2a3");
    assert!(test_new.is_err());
    assert_eq!(
        test_new,
        Err(StruqtureError::IndicesNotNormalOrdered {
            index_i: 2,
            index_j: 3
        })
    );

    let test_new = <HermitianFermionProduct as std::str::FromStr>::from_str("c0a30a0");
    assert!(test_new.is_err());
    assert_eq!(test_new, Err(StruqtureError::IncorrectlyOrderedIndices));

    let test_new = <HermitianFermionProduct as std::str::FromStr>::from_str("c2c1a1a2");
    assert!(test_new.is_err());
    assert_eq!(test_new, Err(StruqtureError::IncorrectlyOrderedIndices));

    let test_new = <HermitianFermionProduct as std::str::FromStr>::from_str("c0a1b2");
    assert!(test_new.is_err());
    assert_eq!(
        test_new,
        Err(StruqtureError::FromStringFailed {
            msg: "Used operator b that is neither 'c' nor 'a' in HermitianFermionProduct::from_str"
                .into()
        })
    );

    let test_new = <HermitianFermionProduct as std::str::FromStr>::from_str("c0a#");
    assert!(test_new.is_err());
    assert_eq!(
        test_new,
        Err(StruqtureError::FromStringFailed {
            msg: "Index of Fermion operator # is not a HermitianFermionProduct::from_str".into()
        })
    );
}

#[test_case( &[], &[]; "empty")]
#[test_case(&[0], &[1]; "0 - 1")]
#[test_case( &[], &[2000]; "empty - 2000")]
#[test_case(&[0,1,2], &[3,4,5]; "0,1,2 - 3,4,5")]
#[test_case( &[1,2], &[1,2]; "2,1 - 1,2")]
#[test_case( &[0], &[0, 30]; "0 - 0,30")]
fn from_string_import_export(creators: &[usize], annihilators: &[usize]) {
    let creators = creators.to_vec();
    let annihilators = annihilators.to_vec();
    let test = HermitianFermionProduct::new(creators, annihilators).unwrap();
    let stringformat = format!("{}", test);
    let test_new = <HermitianFermionProduct as std::str::FromStr>::from_str(&stringformat);
    assert!(test_new.is_ok());
    assert_eq!(test, test_new.unwrap());
}

#[test]
fn corresponds_to_fermion() {
    let creators = &[0];
    let annihilators = &[1];
    let hbp = HermitianFermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();

    let bp = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let result: HermitianFermionProduct = bp.corresponds_to();
    assert_eq!(result, hbp);

    let bp = FermionProduct::new(annihilators.to_vec(), creators.to_vec()).unwrap();
    let result: HermitianFermionProduct = bp.corresponds_to();
    assert_eq!(result, hbp);

    let bp = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let result: FermionProduct = hbp.corresponds_to();
    assert_eq!(result, bp);
}

#[test_case( &[], &[]; "empty")]
#[test_case(&[0], &[1]; "0 - 1")]
#[test_case( &[], &[2000]; "empty - 2000")]
#[test_case(&[0,1,2], &[3,4,5]; "0,1,2 - 3,4,5")]
#[test_case( &[1,2], &[1,2]; "2,1 - 1,2")]
#[test_case( &[0], &[0, 30]; "0 - 0,30")]
fn test_serde_json(creators: &[usize], annihilators: &[usize]) {
    let creators = creators.to_vec();
    let annihilators = annihilators.to_vec();
    let test = HermitianFermionProduct::new(creators, annihilators).unwrap();
    let serialized = serde_json::to_string(&test).unwrap();
    let deserialized: HermitianFermionProduct = serde_json::from_str(&serialized).unwrap();
    assert_eq!(test, deserialized);
}

#[test_case( &[], &[], Token::Str("I"); "empty")]
#[test_case(&[0], &[1], Token::Str("c0a1"); "0 - 1")]
#[test_case( &[], &[2000], Token::Str("a2000"); "empty - 2000")]
#[test_case(&[0,1,2], &[3,4,5], Token::Str("c0c1c2a3a4a5"); "0,1,2 - 3,4,5")]
#[test_case( &[1,2], &[1,2], Token::Str("c1c2a1a2"); "2,1 - 1,2")]
#[test_case( &[0], &[0, 30], Token::Str("c0a0a30"); "0 - 0,30")]
fn serde_readable(creators: &[usize], annihilators: &[usize], result: Token) {
    let creators = creators.to_vec();
    let annihilators = annihilators.to_vec();
    let test = HermitianFermionProduct::new(creators, annihilators).unwrap();

    assert_tokens(&test.readable(), &[result]);
}

#[test_case( &[], &[]; "empty")]
#[test_case(&[0], &[1]; "0 - 1")]
#[test_case( &[], &[2000]; "empty - 2000")]
#[test_case(&[0,1,2], &[3,4,5]; "0,1,2 - 3,4,5")]
#[test_case( &[1,2], &[1,2]; "2,1 - 1,2")]
#[test_case( &[0], &[0, 30]; "0 - 0,30")]
fn test_bincode(creators: &[usize], annihilators: &[usize]) {
    let creators = creators.to_vec();
    let annihilators = annihilators.to_vec();
    let test = HermitianFermionProduct::new(creators, annihilators).unwrap();

    let serialized = bincode::serialize(&test).unwrap();
    let deserialized: HermitianFermionProduct = bincode::deserialize(&serialized).unwrap();
    assert_eq!(test, deserialized);

    let serialized = bincode::serialize(&test.clone().compact()).unwrap();
    let deserialized: HermitianFermionProduct = bincode::deserialize(&serialized).unwrap();
    assert_eq!(test, deserialized);
}

#[test_case( &[], &[2000]; "empty - 2000")]
fn serde_compact_annihilators(creators: &[usize], annihilators: &[usize]) {
    let creators = creators.to_vec();
    let annihilators = annihilators.to_vec();
    let test = HermitianFermionProduct::new(creators, annihilators).unwrap();

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

#[test_case(&[0,1,2], &[3,4,5]; "0,1,2 - 3,4,5")]
fn serde_compact_both(creators: &[usize], annihilators: &[usize]) {
    let creators = creators.to_vec();
    let annihilators = annihilators.to_vec();
    let test = HermitianFermionProduct::new(creators, annihilators).unwrap();

    assert_tokens(
        &test.compact(),
        &[
            Token::Tuple { len: 2 },
            Token::Seq { len: Some(3) },
            Token::U64(0),
            Token::U64(1),
            Token::U64(2),
            Token::SeqEnd,
            Token::Seq { len: Some(3) },
            Token::U64(3),
            Token::U64(4),
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
    let hbp = HermitianFermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();

    assert_eq!(hbp.hermitian_conjugate(), (hbp, 1.0));

    let creators = &[];
    let annihilators = &[0];
    let hbp = HermitianFermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();

    assert_eq!(hbp.hermitian_conjugate(), (hbp, 1.0));

    let creators = &[0, 1];
    let annihilators = &[0, 3];
    let hbp = HermitianFermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();

    assert_eq!(hbp.hermitian_conjugate(), (hbp, 1.0));
}

// Test the set_pauli and get functions of the PauliProduct
#[test]
fn get_value_get_transform_hermitian() {
    let creators = &[0];
    let annihilators = &[0];
    let hbp = HermitianFermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();

    assert_eq!(HermitianFermionProduct::get_key(&hbp), hbp);
    assert_eq!(
        HermitianFermionProduct::get_transform(&hbp, CalculatorComplex::new(1.0, 2.0)),
        CalculatorComplex::new(1.0, 2.0)
    );
}

// Test the set_pauli and get functions of the PauliProduct
#[test]
fn get_value_get_transform_fermion() {
    let creators = &[0];
    let annihilators = &[1];
    let hbp = HermitianFermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();

    let bp = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    assert_eq!(HermitianFermionProduct::get_key(&bp), hbp);
    assert_eq!(
        HermitianFermionProduct::get_transform(&bp, CalculatorComplex::new(1.0, 2.0)),
        CalculatorComplex::new(1.0, 2.0)
    );

    let bp = FermionProduct::new(annihilators.to_vec(), creators.to_vec()).unwrap();
    assert_eq!(HermitianFermionProduct::get_key(&bp), hbp);
    assert_eq!(
        HermitianFermionProduct::get_transform(&bp, CalculatorComplex::new(1.0, 2.0)),
        CalculatorComplex::new(1.0, -2.0)
    );
}

type MulVec = Vec<(TinyVec<[usize; 2]>, TinyVec<[usize; 2]>, f64)>;
#[test_case(tiny_vec!([usize; 2] => 0, 2, 4), tiny_vec!([usize; 2] => 3, 5, 7),
     vec![(tiny_vec!([usize; 2] => 0, 1, 3, 5, 7), tiny_vec!([usize; 2] => 0, 2, 4, 84, 95), -1.0), (tiny_vec!([usize; 2] => 0, 1, 84, 95), tiny_vec!([usize; 2] => 0, 2, 3, 4, 5, 7), -1.0),
          (tiny_vec!([usize; 2] => 0, 2, 3, 4, 5, 7), tiny_vec!([usize; 2] => 0, 1, 84, 95), -1.0), (tiny_vec!([usize; 2] => 0, 2, 4, 84, 95), tiny_vec!([usize; 2] => 0, 1, 3, 5, 7), 1.0)]; "0,2,4 - 3,5,7")]
#[test_case(tiny_vec!([usize; 2] => 2), tiny_vec!([usize; 2] => 2),
     vec![(tiny_vec!([usize; 2] => 0, 1), tiny_vec!([usize; 2] => 84, 95), 1.0), (tiny_vec!([usize; 2] => 0, 1, 2), tiny_vec!([usize; 2] => 2, 84, 95), -1.0), (tiny_vec!([usize; 2] => 2, 84, 95), tiny_vec!([usize; 2] => 0, 1, 2), 1.0)]; "2, - 2")]
#[test_case(tiny_vec!([usize; 2] => 2,20), tiny_vec!([usize; 2] => 2,30),
     vec![(tiny_vec!([usize; 2] => 0, 1, 30), tiny_vec!([usize; 2] => 20, 84, 95), 1.0), (tiny_vec!([usize; 2] => 0, 1, 2, 30), tiny_vec!([usize; 2] => 2, 20, 84, 95), 1.0), (tiny_vec!([usize; 2] => 2, 20, 84, 95), tiny_vec!([usize; 2] => 0, 1, 2, 30), 1.0)]; "2,20 - 2,30")]
#[test_case(tiny_vec!([usize; 2] => 10,20,30), tiny_vec!([usize; 2] => 10,30),
     vec![(tiny_vec!([usize; 2] => 10, 20, 30, 84, 95), tiny_vec!([usize; 2] => 0, 1, 10, 30), 1.0), (tiny_vec!([usize; 2] => 0, 1), tiny_vec!([usize; 2] => 20, 84, 95), 1.0), (tiny_vec!([usize; 2] => 0, 1, 10), tiny_vec!([usize; 2] => 10, 20, 84, 95), -1.0),
          (tiny_vec!([usize; 2] => 0, 1, 30), tiny_vec!([usize; 2] => 20, 30, 84, 95), 1.0), (tiny_vec!([usize; 2] => 0, 1, 10, 30), tiny_vec!([usize; 2] => 10, 20, 30, 84, 95), 1.0)]; "10, 20, 30 - 10,30")]
fn multiply_hermitian_hermitian(
    mut annihilators_left: TinyVec<[usize; 2]>,
    mut creators_right: TinyVec<[usize; 2]>,
    expected: MulVec,
) {
    let creators_left: TinyVec<[usize; 2]> = tiny_vec![0, 1];
    let annihilators_right: TinyVec<[usize; 2]> = tiny_vec![84, 95];
    annihilators_left.sort_unstable();
    creators_right.sort_unstable();
    let left = HermitianFermionProduct::new(creators_left, annihilators_left).unwrap();
    let right = HermitianFermionProduct::new(creators_right, annihilators_right).unwrap();
    let mut result_list: Vec<(FermionProduct, f64)> = Vec::new();
    for pair in expected {
        result_list.push((FermionProduct::new(pair.0, pair.1).unwrap(), pair.2));
    }
    let result = left * right;
    assert_eq!(result.len(), result_list.len());
    for res in result_list {
        assert!(result.contains(&res));
    }
}

#[test_case(tiny_vec!([usize; 2] => 0, 2, 4), tiny_vec!([usize; 2] => 1, 3, 5),
     vec![(tiny_vec!([usize; 2] => 1, 3, 5, 43, 78), tiny_vec!([usize; 2] => 0, 2, 4, 84, 95), -1.0),
          (tiny_vec!([usize; 2] => 43, 78, 84, 95), tiny_vec!([usize; 2] => 0, 1, 2, 3, 4, 5), -1.0)]; "0,2,4 - 1,3,5")]
#[test_case(tiny_vec!([usize; 2] => 0), tiny_vec!([usize; 2] => 1),
     vec![(tiny_vec!([usize; 2] => 1, 43, 78), tiny_vec!([usize; 2] => 0, 84, 95), -1.0),
          (tiny_vec!([usize; 2] => 43, 78, 84, 95), tiny_vec!([usize; 2] => 0, 1), -1.0)]; "0, - 1")]
#[test_case(tiny_vec!([usize; 2] => 2,84), tiny_vec!([usize; 2] => 2,30),
    vec![(tiny_vec!([usize; 2]), tiny_vec!([usize; 2]), 1.0)]; "2,84 - 2,30")]
#[test_case(tiny_vec!([usize; 2] => 10,20,30), tiny_vec!([usize; 2] => 10,30),
    vec![(tiny_vec!([usize; 2] => 43, 78), tiny_vec!([usize; 2] => 20, 84, 95), 1.0), (tiny_vec!([usize; 2] => 30, 43, 78), tiny_vec!([usize; 2] => 20, 30, 84, 95), 1.0), (tiny_vec!([usize; 2] => 10, 43, 78), tiny_vec!([usize; 2] => 10, 20, 84, 95), -1.0), (tiny_vec!([usize; 2] => 10, 30, 43, 78), tiny_vec!([usize; 2] => 10, 20, 30, 84, 95), 1.0)]; "10, 20, 30 - 10,30")]
#[test_case(tiny_vec!([usize; 2] => 10,20,43), tiny_vec!([usize; 2] => 10,43),
vec![(tiny_vec!([usize; 2] => 43, 78), tiny_vec!([usize; 2] => 20, 84, 95), 1.0), (tiny_vec!([usize; 2] => 10, 43, 78), tiny_vec!([usize; 2] => 10, 20, 84, 95), -1.0)]; "100, 20, 30 - 10,30")]
fn multiply_fermion_hermitian(
    annihilators_left: TinyVec<[usize; 2]>,
    creators_right: TinyVec<[usize; 2]>,
    expected: MulVec,
) {
    let creators_left: TinyVec<[usize; 2]> = tiny_vec![43, 78];
    let annihilators_right: TinyVec<[usize; 2]> = tiny_vec![84, 95];
    let left = FermionProduct::new(creators_left, annihilators_left).unwrap();
    let right = HermitianFermionProduct::new(creators_right, annihilators_right).unwrap();
    let mut result_list: Vec<(FermionProduct, f64)> = Vec::new();
    for pair in expected {
        if pair.0.is_empty() && pair.1.is_empty() {
            continue;
        } else {
            result_list.push((FermionProduct::new(pair.0, pair.1).unwrap(), pair.2));
        }
    }
    let result = left * right;

    assert_eq!(result.len(), result_list.len());
    for res in result_list {
        assert!(result.contains(&res));
    }
}

#[test_case(tiny_vec!([usize; 2] => 0, 2, 4), tiny_vec!([usize; 2] => 3, 5, 7),
     vec![(tiny_vec!([usize; 2] => 0, 1, 3, 5, 7), tiny_vec!([usize; 2] => 0, 2, 4, 84, 95), -1.0), (tiny_vec!([usize; 2] => 0, 2, 3, 4, 5, 7), tiny_vec!([usize; 2] => 0, 1, 84, 95), -1.0)]; "0,2,4 - 3,5,7")]
#[test_case(tiny_vec!([usize; 2] => 2), tiny_vec!([usize; 2] => 2),
     vec![(tiny_vec!([usize; 2] => 0, 1), tiny_vec!([usize; 2] => 84, 95), 1.0), (tiny_vec!([usize; 2] => 0, 1, 2), tiny_vec!([usize; 2] => 2, 84, 95), -1.0)]; "2, - 2")]
#[test_case(tiny_vec!([usize; 2] => 2,20), tiny_vec!([usize; 2] => 2,30),
     vec![(tiny_vec!([usize; 2] => 0, 1, 30), tiny_vec!([usize; 2] => 20, 84, 95), 1.0), (tiny_vec!([usize; 2] => 0, 1, 2, 30), tiny_vec!([usize; 2] => 2, 20,  84, 95), 1.0)]; "2,20 - 2,30")]
#[test_case(tiny_vec!([usize; 2] => 10,20,30), tiny_vec!([usize; 2] => 10,30),
     vec![(tiny_vec!([usize; 2] => 0, 1), tiny_vec!([usize; 2] => 20, 84, 95), 1.0), (tiny_vec!([usize; 2] => 0, 1, 30), tiny_vec!([usize; 2] => 20, 30, 84, 95), 1.0),
          (tiny_vec!([usize; 2] => 0, 1, 10), tiny_vec!([usize; 2] => 10, 20, 84, 95), -1.0), (tiny_vec!([usize; 2] => 0, 1, 10, 30), tiny_vec!([usize; 2] => 10, 20, 30, 84, 95), 1.0)]; "10, 20, 30 - 10,30")]
fn multiply_hermitian_fermion(
    annihilators_left: TinyVec<[usize; 2]>,
    creators_right: TinyVec<[usize; 2]>,
    expected: MulVec,
) {
    let creators_left: TinyVec<[usize; 2]> = tiny_vec![0, 1];
    let annihilators_right: TinyVec<[usize; 2]> = tiny_vec![84, 95];
    let left = HermitianFermionProduct::new(creators_left, annihilators_left).unwrap();
    let right = FermionProduct::new(creators_right, annihilators_right).unwrap();
    let mut result_list: Vec<(FermionProduct, f64)> = Vec::new();
    for pair in expected {
        result_list.push((FermionProduct::new(pair.0, pair.1).unwrap(), pair.2));
    }
    let result = left * &right;

    assert_eq!(result.len(), result_list.len());
    for res in result_list {
        assert!(result.contains(&res));
    }
}

#[test]
fn multiply_list_right() {
    let annihilators_right: TinyVec<[usize; 2]> = tiny_vec![43, 78];
    let creators_left: TinyVec<[usize; 2]> = tiny_vec![0, 95];
    let left = HermitianFermionProduct::new(creators_left, tiny_vec!([usize; 2] => 1,20)).unwrap();
    let mut right: Vec<FermionProduct> = Vec::new();
    right.push(
        FermionProduct::new(tiny_vec!([usize; 2] => 1,30), annihilators_right.clone()).unwrap(),
    );
    right.push(FermionProduct::new(tiny_vec!([usize; 2] => 0), annihilators_right).unwrap());
    let mut result_list: Vec<(FermionProduct, f64)> = Vec::new();
    let expected = vec![
        (
            tiny_vec!([usize; 2] => 0, 30, 95),
            tiny_vec!([usize; 2] => 20, 43, 78),
            -1.0,
        ),
        (
            tiny_vec!([usize; 2] => 0, 1, 30, 95),
            tiny_vec!([usize; 2] => 1, 20, 43, 78),
            1.0,
        ),
        (
            tiny_vec!([usize; 2] => 1, 20),
            tiny_vec!([usize; 2] => 43, 78, 95),
            -1.0,
        ),
        (
            tiny_vec!([usize; 2] => 0, 1, 20),
            tiny_vec!([usize; 2] => 0, 43, 78, 95),
            1.0,
        ),
    ];
    for pair in expected {
        result_list.push((FermionProduct::new(pair.0, pair.1).unwrap(), pair.2));
    }
    let result = left * right;

    assert_eq!(result.len(), result_list.len());
    for res in result_list {
        assert!(result.contains(&res));
    }
}

#[test]
fn multiply_list_left() {
    let annihilators_right: TinyVec<[usize; 2]> = tiny_vec![43, 78];
    let creators_left: TinyVec<[usize; 2]> = tiny_vec![84, 95];
    let right =
        HermitianFermionProduct::new(tiny_vec!([usize; 2] => 1,20), annihilators_right).unwrap();
    let mut left: Vec<FermionProduct> = Vec::new();
    left.push(FermionProduct::new(creators_left.clone(), tiny_vec!([usize; 2] => 1,30)).unwrap());
    left.push(FermionProduct::new(creators_left, tiny_vec!([usize; 2] => 0)).unwrap());
    let mut result_list: Vec<(FermionProduct, f64)> = Vec::new();
    let expected = vec![
        (
            tiny_vec!([usize; 2] => 20, 84, 95),
            tiny_vec!([usize; 2] => 30, 43, 78),
            1.0,
        ),
        (
            tiny_vec!([usize; 2] => 1, 20, 84, 95),
            tiny_vec!([usize; 2] => 1, 30, 43, 78),
            1.0,
        ),
        (
            tiny_vec!([usize; 2] => 1, 20, 84, 95),
            tiny_vec!([usize; 2] => 0, 43, 78),
            1.0,
        ),
        (
            tiny_vec!([usize; 2] => 43, 78, 84, 95),
            tiny_vec!([usize; 2] => 0, 1, 20),
            1.0,
        ),
    ];
    for pair in expected {
        result_list.push((FermionProduct::new(pair.0, pair.1).unwrap(), pair.2));
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
    let hbp = HermitianFermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();

    assert_eq!(
        format!("{:?}", hbp),
        "HermitianFermionProduct { creators: [0], annihilators: [3] }"
    );
    assert_eq!(format!("{}", hbp), "c0a3");

    let creators = &[0];
    let annihilators = &[3];
    let hbp_1 = HermitianFermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let creators = &[0];
    let annihilators = &[3];
    let hbp_2 = HermitianFermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();

    let mut s_1 = DefaultHasher::new();
    hbp_1.hash(&mut s_1);
    let mut s_2 = DefaultHasher::new();
    hbp_2.hash(&mut s_2);
    assert_eq!(s_1.finish(), s_2.finish())
}

// Test the Clone, PartialEq, PartialOrd and Ord traits of PauliProduct
#[test]
fn clone_partial_eq_partial_ord() {
    let creators = &[0];
    let annihilators = &[3];
    let hbp = HermitianFermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();

    // Test Clone trait
    assert_eq!(hbp.clone(), hbp);

    // Test PartialEq trait
    let creators = &[0];
    let annihilators = &[3];
    let hbp_0 = HermitianFermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let creators = &[1];
    let annihilators = &[3];
    let hbp_1 = HermitianFermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    assert!(hbp_0 == hbp);
    assert!(hbp == hbp_0);
    assert!(hbp_1 != hbp);
    assert!(hbp != hbp_1);

    // Test PartialOrd trait
    let creators = &[0];
    let annihilators = &[3];
    let hbp_0 = HermitianFermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let creators = &[1];
    let annihilators = &[3];
    let hbp_1 = HermitianFermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();

    assert_eq!(hbp_0.partial_cmp(&hbp), Some(Ordering::Equal));
    assert_eq!(hbp.partial_cmp(&hbp_0), Some(Ordering::Equal));
    assert_eq!(hbp_1.partial_cmp(&hbp), Some(Ordering::Greater));
    assert_eq!(hbp.partial_cmp(&hbp_1), Some(Ordering::Less));

    // Test Ord trait
    assert_eq!(hbp_0.cmp(&hbp), Ordering::Equal);
    assert_eq!(hbp.cmp(&hbp_0), Ordering::Equal);
    assert_eq!(hbp_1.cmp(&hbp), Ordering::Greater);
    assert_eq!(hbp.cmp(&hbp_1), Ordering::Less);
}

#[cfg(feature = "json_schema")]
#[test]
fn test_hermitian_fermion_product_schema() {
    let pp = HermitianFermionProduct::new([0], [0]).unwrap();
    let schema = schemars::schema_for!(HermitianFermionProduct);
    let schema_checker = jsonschema::JSONSchema::compile(&serde_json::to_value(&schema).unwrap())
        .expect("schema is valid");
    let value = serde_json::to_value(pp).unwrap();
    let validation = schema_checker.validate(&value);
    assert!(validation.is_ok());
}
