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

//! Integration test for public API of SpinLindbladNoiseSystem

use super::create_na_matrix_from_decoherence_list;
use na::DMatrix;
use nalgebra as na;
use num_complex::Complex64;
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde_test::{assert_tokens, Configure, Token};
use std::collections::{BTreeMap, HashMap};
use std::iter::{FromIterator, IntoIterator};
use std::ops::{Add, Sub};
use std::str::FromStr;
use struqture::prelude::*;
use struqture::spins::{DecoherenceProduct, SpinLindbladNoiseOperator, SpinLindbladNoiseSystem};
use struqture::{CooSparseMatrix, OperateOnDensityMatrix, SpinIndex, StruqtureError};
use test_case::test_case;

// Test the new function of the SpinLindbladNoiseSystem
#[test]
fn new() {
    let slno = SpinLindbladNoiseSystem::new(Some(1));
    assert!(slno.is_empty());
    assert_eq!(
        SpinLindbladNoiseSystem::new(None),
        SpinLindbladNoiseSystem::default()
    );
}

// Test the new function of the SpinLindbladNoiseSystem
#[test]
fn new_system() {
    let system = SpinLindbladNoiseSystem::new(Some(1));
    assert!(system.is_empty());
    assert_eq!(system.operator(), &SpinLindbladNoiseOperator::default());
    assert_eq!(system.number_spins(), 1_usize)
}

// Test the new function of the SpinLindbladNoiseSystem with no spins specified
#[test]
fn new_system_none() {
    let system = SpinLindbladNoiseSystem::new(None);
    assert!(system.operator().is_empty());
    assert_eq!(system.operator(), &SpinLindbladNoiseOperator::default());
    assert_eq!(system.number_spins(), 0_usize);
}

// Test the from_spin_operator and spin_operator functions of the SpinLindbladNoiseSystem with number_spins = None
#[test]
fn from_noise_operator_none() {
    let mut slno: SpinLindbladNoiseOperator = SpinLindbladNoiseOperator::new();
    let mut slns = SpinLindbladNoiseSystem::new(None);
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().z(2);
    slno.set((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    slns.set((dp_0.clone(), dp_0), CalculatorComplex::from(0.5))
        .unwrap();

    assert_eq!(
        slns,
        SpinLindbladNoiseSystem::from_operator(slno.clone(), None).unwrap()
    );
    assert_eq!(
        slns.operator(),
        SpinLindbladNoiseSystem::from_operator(slno.clone(), None)
            .unwrap()
            .operator()
    );
    assert_eq!(
        &slno,
        SpinLindbladNoiseSystem::from_operator(slno.clone(), None)
            .unwrap()
            .operator()
    );
}

// Test the from_spin_operator and spin_operator functions of the SpinLindbladNoiseSystem with number_spins = Some(2)
#[test]
fn from_noise_operator_some() {
    let mut slno: SpinLindbladNoiseOperator = SpinLindbladNoiseOperator::new();
    let mut system = SpinLindbladNoiseSystem::new(Some(2));
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().z(1);
    slno.add_operator_product((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(1.0))
        .unwrap();
    system
        .add_operator_product((dp_0.clone(), dp_0), CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(
        system,
        SpinLindbladNoiseSystem::from_operator(slno.clone(), Some(2)).unwrap()
    );
    assert_eq!(
        system.operator(),
        SpinLindbladNoiseSystem::from_operator(slno.clone(), Some(2))
            .unwrap()
            .operator()
    );
    assert_eq!(
        &slno,
        SpinLindbladNoiseSystem::from_operator(slno.clone(), Some(2))
            .unwrap()
            .operator()
    );
    assert_eq!(
        SpinLindbladNoiseSystem::from_operator(slno.clone(), Some(0)),
        Err(StruqtureError::NumberSpinsExceeded {})
    );
}

#[test]
fn empty_clone_options() {
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slno = SpinLindbladNoiseSystem::new(Some(3));
    slno.set((dp_0.clone(), dp_0), CalculatorComplex::from(0.5))
        .unwrap();

    let empty: Option<usize> = None;
    let full: Option<usize> = Some(3);
    assert_eq!(slno.empty_clone(empty), SpinLindbladNoiseSystem::new(full));
    assert_eq!(
        slno.empty_clone(full),
        SpinLindbladNoiseSystem::with_capacity(full, 1)
    );
}

// Test the current_number_spins function of the SpinLindbladNoiseSystem
#[test]
fn internal_map_current_number_spins() {
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().x(0);
    let dp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slno = SpinLindbladNoiseSystem::new(Some(3));
    assert_eq!(slno.current_number_spins(), 0_usize);
    slno.set((dp_0.clone(), dp_0), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(slno.current_number_spins(), 1_usize);
    slno.set((dp_2.clone(), dp_2), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(slno.current_number_spins(), 3_usize);
}

// Test the len function of the SpinLindbladNoiseSystem
#[test]
fn internal_map_len() {
    let dp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slno = SpinLindbladNoiseSystem::new(Some(3));
    slno.set((dp_2.clone(), dp_2), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(slno.len(), 1_usize);
}

// Test the try_set_noise and get functions of the SpinLindbladNoiseSystem
#[test]
fn internal_map_set_get() {
    let dp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slno = SpinLindbladNoiseSystem::new(Some(3));
    assert_eq!(slno.number_spins(), 3_usize);

    // Vacant
    slno.set((dp_2.clone(), dp_2.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(
        slno.get(&(dp_2.clone(), dp_2.clone())),
        &CalculatorComplex::from(0.5)
    );
    assert_eq!(slno.number_spins(), 3_usize);

    // 2) Test iter, keys, values functions
    let mut map: BTreeMap<(DecoherenceProduct, DecoherenceProduct), CalculatorComplex> =
        BTreeMap::new();
    map.insert((dp_2.clone(), dp_2.clone()), CalculatorComplex::from(0.5));
    // iter
    let dict = slno.iter();
    for (item_d, item_m) in dict.zip(map.iter()) {
        assert_eq!(item_d, item_m);
    }
    // keys
    let keys = slno.keys();
    for (key_s, key_m) in keys.zip(map.keys()) {
        assert_eq!(key_s, key_m);
    }
    // values
    let values = slno.values();
    for (val_s, val_m) in values.zip(map.values()) {
        assert_eq!(val_s, val_m);
    }

    // 3) Test remove function
    slno.remove(&(dp_2.clone(), dp_2));
    assert_eq!(slno, SpinLindbladNoiseSystem::new(Some(3)));
}

// Test the add_noise function of the SpinLindbladNoiseSystem
#[test]
fn internal_map_add_noise() {
    let dp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slno = SpinLindbladNoiseSystem::new(Some(3));

    let _ = slno.add_operator_product((dp_2.clone(), dp_2.clone()), CalculatorComplex::from(0.5));
    assert_eq!(
        slno.get(&(dp_2.clone(), dp_2.clone())),
        &CalculatorComplex::from(0.5)
    );
    let _ = slno.add_operator_product((dp_2.clone(), dp_2.clone()), CalculatorComplex::from(-0.5));
    assert_eq!(
        slno.get(&(dp_2.clone(), dp_2)),
        &CalculatorComplex::from(0.0)
    );
}

// Test the Iter traits of SpinLindbladNoiseSystem: into_iter, from_iter and extend
#[test]
fn into_iter_from_iter_extend() {
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let dp_1: DecoherenceProduct = DecoherenceProduct::new().x(1);
    let mut slno_0 = SpinLindbladNoiseSystem::new(None);
    let _ = slno_0.add_operator_product((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(1.0));

    let slno_iter = slno_0.clone().into_iter();
    assert_eq!(SpinLindbladNoiseSystem::from_iter(slno_iter), slno_0);
    let slno_iter = (&slno_0)
        .into_iter()
        .map(|(key, value)| (key.clone(), value.clone()));
    assert_eq!(SpinLindbladNoiseSystem::from_iter(slno_iter), slno_0);

    let mut mapping: BTreeMap<(DecoherenceProduct, DecoherenceProduct), CalculatorComplex> =
        BTreeMap::new();
    mapping.insert((dp_1.clone(), dp_1.clone()), CalculatorComplex::from(0.5));
    let mapping_iter = mapping.into_iter();
    slno_0.extend(mapping_iter);

    let mut slno_0_1 = SpinLindbladNoiseSystem::new(None);
    let _ = slno_0_1.add_operator_product((dp_0.clone(), dp_0), CalculatorComplex::from(1.0));
    let _ = slno_0_1.add_operator_product((dp_1.clone(), dp_1), CalculatorComplex::from(0.5));

    assert_eq!(slno_0, slno_0_1);
}

// Test the negative operation: -SpinLindbladNoiseSystem
#[test]
fn negative_slno() {
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slno_0 = SpinLindbladNoiseSystem::new(Some(1));
    let _ = slno_0.add_operator_product((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(1.0));
    let mut slno_0_minus = SpinLindbladNoiseSystem::new(Some(1));
    let _ = slno_0_minus.add_operator_product((dp_0.clone(), dp_0), CalculatorComplex::from(-1.0));

    assert_eq!(-slno_0, slno_0_minus);
}

// Test the addition: SpinLindbladNoiseSystem + SpinLindbladNoiseSystem
#[test]
fn add_slno_slno() {
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let dp_1: DecoherenceProduct = DecoherenceProduct::new().x(1);
    let mut slno_0 = SpinLindbladNoiseSystem::new(Some(2));
    let _ = slno_0.add_operator_product((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(1.0));
    let mut slno_1 = SpinLindbladNoiseSystem::new(Some(2));
    let _ = slno_1.add_operator_product((dp_1.clone(), dp_1.clone()), CalculatorComplex::from(0.5));
    let mut slno_0_1 = SpinLindbladNoiseSystem::new(Some(2));
    let _ = slno_0_1.add_operator_product((dp_0.clone(), dp_0), CalculatorComplex::from(1.0));
    let _ = slno_0_1.add_operator_product((dp_1.clone(), dp_1), CalculatorComplex::from(0.5));

    assert_eq!(slno_0.clone() + slno_1.clone(), Ok(slno_0_1.clone()));
    assert_eq!(slno_0.add(slno_1), Ok(slno_0_1));
}

// Test the subtraction: SpinLindbladNoiseSystem - SpinLindbladNoiseSystem
#[test]
fn sub_slno_slno() {
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let dp_1: DecoherenceProduct = DecoherenceProduct::new().x(1);
    let mut slno_0 = SpinLindbladNoiseSystem::new(Some(2));
    let _ = slno_0.add_operator_product((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(1.0));
    let mut slno_1 = SpinLindbladNoiseSystem::new(Some(2));
    let _ = slno_1.add_operator_product((dp_1.clone(), dp_1.clone()), CalculatorComplex::from(0.5));
    let mut slno_0_1 = SpinLindbladNoiseSystem::new(Some(2));
    let _ = slno_0_1.add_operator_product((dp_0.clone(), dp_0), CalculatorComplex::from(1.0));
    let _ = slno_0_1.add_operator_product((dp_1.clone(), dp_1), CalculatorComplex::from(-0.5));

    assert_eq!(slno_0.clone() - slno_1.clone(), Ok(slno_0_1.clone()));
    assert_eq!(slno_0.sub(slno_1), Ok(slno_0_1));
}

// Test the multiplication: SpinLindbladNoiseSystem * Calculatorcomplex
#[test]
fn mul_so_cc() {
    let pp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut so_0 = SpinLindbladNoiseSystem::new(Some(1));
    let _ = so_0.set((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(2.0));
    let mut so_0_1 = SpinLindbladNoiseSystem::new(Some(1));
    let _ = so_0_1.set((pp_0.clone(), pp_0), CalculatorComplex::from(6.0));

    assert_eq!(so_0 * CalculatorComplex::from(3.0), so_0_1);
}

// Test the multiplication: SpinLindbladNoiseSystem * Calculatorcomplex
#[test]
fn mul_so_cf() {
    let pp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut so_0 = SpinLindbladNoiseSystem::new(Some(1));
    let _ = so_0.add_operator_product((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(2.0));
    let mut so_0_1 = SpinLindbladNoiseSystem::new(Some(1));
    let _ = so_0_1.add_operator_product((pp_0.clone(), pp_0), CalculatorComplex::from(6.0));

    assert_eq!(so_0 * CalculatorFloat::from(3.0), so_0_1);
}

// Test the Debug trait of SpinLindbladNoiseSystem
#[test]
fn debug() {
    let dp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slno = SpinLindbladNoiseSystem::new(Some(1));
    let _ = slno.set((dp.clone(), dp), CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{:?}", slno),
        "SpinLindbladNoiseSystem { number_spins: Some(1), operator: SpinLindbladNoiseOperator { internal_map: {(DecoherenceProduct { items: [(0, Z)] }, DecoherenceProduct { items: [(0, Z)] }): CalculatorComplex { re: Float(0.5), im: Float(0.0) }} } }"
    );
}

// Test the Display trait of DecoherenceOperator
#[test]
fn display() {
    let mut so = SpinLindbladNoiseSystem::new(Some(1));
    let pp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let _ = so.set((pp.clone(), pp), CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{}", so),
        "SpinLindbladNoiseSystem(1){\n(0Z, 0Z): (5e-1 + i * 0e0),\n}"
    );
}

// Test the Clone and PartialEq traits of SpinLindbladNoiseSystem
#[test]
fn clone_partial_eq() {
    let dp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slno = SpinLindbladNoiseSystem::new(Some(1));
    slno.set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    // Test Clone trait
    assert_eq!(slno.clone(), slno);

    // Test PartialEq trait
    let dp_1: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slno_1 = SpinLindbladNoiseSystem::new(Some(1));
    slno_1
        .set((dp_1.clone(), dp_1), CalculatorComplex::from(0.5))
        .unwrap();
    let dp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slno_2 = SpinLindbladNoiseSystem::new(Some(3));
    slno_2
        .set((dp_2.clone(), dp_2), CalculatorComplex::from(0.5))
        .unwrap();
    assert!(slno_1 == slno);
    assert!(slno == slno_1);
    assert!(slno_2 != slno);
    assert!(slno != slno_2);
}

#[test]
fn serde_json() {
    let dp = DecoherenceProduct::new().x(0);
    let mut slno = SpinLindbladNoiseSystem::new(Some(1));
    slno.set((dp.clone(), dp), CalculatorComplex::from(1.0))
        .unwrap();

    let serialized = serde_json::to_string(&slno).unwrap();
    let deserialized: SpinLindbladNoiseSystem = serde_json::from_str(&serialized).unwrap();
    assert_eq!(slno, deserialized);
}

/// Test SpinLindbladNoiseSystem Serialization and Deserialization traits (readable)
#[test]
fn serde_readable() {
    use struqture::MINIMUM_STRUQTURE_VERSION;
    let major_version = MINIMUM_STRUQTURE_VERSION.0;
    let minor_version = MINIMUM_STRUQTURE_VERSION.1;

    let dp = DecoherenceProduct::new().x(0);
    let mut slno = SpinLindbladNoiseSystem::new(Some(1));
    slno.set((dp.clone(), dp), CalculatorComplex::from(1.0))
        .unwrap();

    assert_tokens(
        &slno.readable(),
        &[
            Token::Struct {
                name: "SpinLindbladNoiseSystem",
                len: 2,
            },
            Token::Str("number_spins"),
            Token::Some,
            Token::U64(1),
            Token::Str("operator"),
            Token::Struct {
                name: "SpinLindbladNoiseOperatorSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 4 },
            Token::Str("0X"),
            Token::Str("0X"),
            Token::F64(1.0),
            Token::F64(0.0),
            Token::TupleEnd,
            Token::SeqEnd,
            Token::Str("_struqture_version"),
            Token::Struct {
                name: "StruqtureVersionSerializable",
                len: 2,
            },
            Token::Str("major_version"),
            Token::U32(major_version),
            Token::Str("minor_version"),
            Token::U32(minor_version),
            Token::StructEnd,
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

#[test]
fn bincode() {
    let dp = DecoherenceProduct::new().x(0);
    let mut slno = SpinLindbladNoiseSystem::new(Some(1));
    slno.set((dp.clone(), dp), CalculatorComplex::from(1.0))
        .unwrap();

    let encoded: Vec<u8> = bincode::serialize(&slno).unwrap();
    let decoded: SpinLindbladNoiseSystem = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(slno, decoded);

    let encoded: Vec<u8> = bincode::serialize(&slno.clone().compact()).unwrap();
    let decoded: SpinLindbladNoiseSystem = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(slno, decoded);
}

/// Test SpinLindbladNoiseSystem Serialization and Deserialization traits (compact)
#[test]
fn serde_compact() {
    use struqture::MINIMUM_STRUQTURE_VERSION;
    let major_version = MINIMUM_STRUQTURE_VERSION.0;
    let minor_version = MINIMUM_STRUQTURE_VERSION.1;

    let dp = DecoherenceProduct::new().x(0);
    let mut slno = SpinLindbladNoiseSystem::new(Some(1));
    slno.set((dp.clone(), dp), CalculatorComplex::from(1.0))
        .unwrap();

    assert_tokens(
        &slno.compact(),
        &[
            Token::Struct {
                name: "SpinLindbladNoiseSystem",
                len: 2,
            },
            Token::Str("number_spins"),
            Token::Some,
            Token::U64(1),
            Token::Str("operator"),
            Token::Struct {
                name: "SpinLindbladNoiseOperatorSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 4 },
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::U64(0),
            Token::UnitVariant {
                name: "SingleDecoherenceOperator",
                variant: "X",
            },
            Token::TupleEnd,
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::U64(0),
            Token::UnitVariant {
                name: "SingleDecoherenceOperator",
                variant: "X",
            },
            Token::TupleEnd,
            Token::SeqEnd,
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(1.0),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(0.0),
            Token::TupleEnd,
            Token::SeqEnd,
            Token::Str("_struqture_version"),
            Token::Struct {
                name: "StruqtureVersionSerializable",
                len: 2,
            },
            Token::Str("major_version"),
            Token::U32(major_version),
            Token::Str("minor_version"),
            Token::U32(minor_version),
            Token::StructEnd,
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

#[test_case("0Z", "0Z", &["Z"], &["Z"]; "0Z")]
#[test_case("1X", "1X", &["X", "I"], &["X", "I"]; "1X1X")]
#[test_case("0iY", "0iY", &["iY"], &["iY"]; "0m0m")]
#[test_case("0X", "0X", &[ "X"], &["X"]; "0x0x")]
#[test_case("0X1X", "0X1X", &["X", "X"], &["X", "X"]; "0x1x0x1x")]
#[test_case("0X", "0iY", &["X"], &["iY"]; "0p0m")]
#[test_case("1X", "1iY", &["X", "I"], &["iY", "I"]; "1p1m")]
#[test_case("1Z", "1iY", &["Z", "I"], &["iY", "I"]; "getting_x")]
fn test_superoperator(
    left_representation: &str,
    right_representation: &str,
    left_operators: &[&str],
    right_operators: &[&str],
) {
    let mut system = SpinLindbladNoiseSystem::new(Some(2));
    let left: DecoherenceProduct = DecoherenceProduct::from_str(left_representation).unwrap();
    let right: DecoherenceProduct = DecoherenceProduct::from_str(right_representation).unwrap();

    let _ = system.set((left, right), 1.0.into());

    let dimension = 4_usize.pow(left_operators.len() as u32);

    // Constructing matrix by hand:

    let identities: Vec<&str> = (0..left_operators.len()).map(|_| "I").collect();

    let i = create_na_matrix_from_decoherence_list(&identities);
    let l_left = create_na_matrix_from_decoherence_list(left_operators);
    let l_right = create_na_matrix_from_decoherence_list(right_operators).transpose();

    let product = l_right.clone() * l_left.clone();

    let test_matrix = l_left.kronecker(&l_right.transpose())
        - (product.kronecker(&i) + i.kronecker(&product.transpose())) * Complex64::from(0.5);

    let second_test_matrix = system
        .sparse_matrix_superoperator(Some(left_operators.len()))
        .unwrap();
    let (test_vals, (test_rows, test_columns)) = system
        .sparse_matrix_superoperator_coo(Some(left_operators.len()))
        .unwrap();
    for (second_val, (row, column)) in test_vals
        .iter()
        .zip(test_rows.iter().zip(test_columns.iter()))
    {
        let val = test_matrix[(*row, *column)];
        assert_eq!(&val, second_val);
    }

    #[allow(unused)]
    fn fast_convert(map: HashMap<(usize, usize), f64>, dimension: usize) -> na::DMatrix<f64> {
        let mut mat = na::DMatrix::<f64>::zeros(dimension, dimension);
        for ((row, column), val) in map.iter() {
            mat[(*row, *column)] = *val;
        }
        mat
    }

    for row in 0..dimension {
        for column in 0..dimension {
            let key = (row, column);
            let val = test_matrix[(row, column)];
            let second_val = second_test_matrix.get(&key);

            match second_val {
                Some(x) => assert_eq!(&val, x),
                None => {
                    assert_eq!(val, Complex64::from(0.0))
                }
            }
        }
    }

    let full_result = system.sparse_lindblad_entries().unwrap();
    let coo_test_matrix = full_result[0].clone().0;
    let mut coo_hashmap: HashMap<(usize, usize), Complex64> = HashMap::new();
    for i in 0..coo_test_matrix.0.len() {
        coo_hashmap.insert(
            (coo_test_matrix.1 .0[i], coo_test_matrix.1 .1[i]),
            coo_test_matrix.0[i],
        );
    }
    for row in 0..dimension {
        for column in 0..dimension {
            let key = (row, column);
            let val = test_matrix[(row, column)];
            let second_val = second_test_matrix.get(&key);

            match second_val {
                Some(x) => assert_eq!(&val, x),
                None => {
                    assert_eq!(val, 0.0.into())
                }
            }
        }
    }
}

#[test]
fn test_superoperator_multiple_entries() {
    let mut system = SpinLindbladNoiseSystem::new(Some(2));

    let _ = system.set(
        (
            DecoherenceProduct::from_str("1X").unwrap(),
            DecoherenceProduct::from_str("1X").unwrap(),
        ),
        1e-1.into(),
    );
    let _ = system.set(
        (
            DecoherenceProduct::from_str("1X").unwrap(),
            DecoherenceProduct::from_str("1Z").unwrap(),
        ),
        1e-0.into(),
    );
    let _ = system.set(
        (
            DecoherenceProduct::from_str("1Z").unwrap(),
            DecoherenceProduct::from_str("1X").unwrap(),
        ),
        1e-0.into(),
    );
    let _ = system.set(
        (
            DecoherenceProduct::from_str("1iY").unwrap(),
            DecoherenceProduct::from_str("1iY").unwrap(),
        ),
        1e-0.into(),
    );
    let _ = system.set(
        (
            DecoherenceProduct::from_str("1Z").unwrap(),
            DecoherenceProduct::from_str("1Z").unwrap(),
        ),
        2.0.into(),
    );

    let dimension = 4_usize.pow(2_u32);

    // Constructing matrix by hand:

    let identities: Vec<&str> = (0..2).map(|_| "I").collect();

    let i = create_na_matrix_from_decoherence_list(&identities);
    let mut test_matrix = DMatrix::<Complex64>::zeros(16, 16);
    for (left_operators, right_operators, prefactor) in [
        (&["X", "I"], &["X", "I"], 1e-1),
        (&["X", "I"], &["Z", "I"], 1e-0),
        (&["Z", "I"], &["X", "I"], 1e-0),
        (&["iY", "I"], &["iY", "I"], 1e-0),
        (&["Z", "I"], &["Z", "I"], 2.0),
    ] {
        let l_left = create_na_matrix_from_decoherence_list(left_operators);
        let l_right = create_na_matrix_from_decoherence_list(right_operators).transpose();

        let product = l_right.clone() * l_left.clone();

        test_matrix += (l_left.kronecker(&l_right.transpose())
            - (product.kronecker(&i) + i.kronecker(&product.transpose())) * Complex64::from(0.5))
            * Complex64::from(prefactor);
    }
    let second_test_matrix = system.sparse_matrix_superoperator(Some(2)).unwrap();

    #[allow(unused)]
    fn fast_convert(map: HashMap<(usize, usize), f64>, dimension: usize) -> na::DMatrix<f64> {
        let mut mat = na::DMatrix::<f64>::zeros(dimension, dimension);
        for ((row, column), val) in map.iter() {
            mat[(*row, *column)] = *val;
        }
        mat
    }

    for row in 0..dimension {
        for column in 0..dimension {
            let key = (row, column);
            let val: Complex64 = test_matrix[(row, column)];
            let second_val = second_test_matrix.get(&key);
            match second_val {
                Some(x) => assert_eq!(&val, x),
                None => {
                    assert_eq!(val, Complex64::from(0.0))
                }
            }
        }
    }

    let (test_vals, (test_rows, test_columns)) =
        system.sparse_matrix_superoperator_coo(Some(2)).unwrap();
    for (second_val, (row, column)) in test_vals
        .iter()
        .zip(test_rows.iter().zip(test_columns.iter()))
    {
        let val = test_matrix[(*row, *column)];
        assert_eq!(&val, second_val);
    }
}

#[test]
fn unitary_matrix() {
    let mut system = SpinLindbladNoiseSystem::new(Some(2));
    let pp0: DecoherenceProduct = DecoherenceProduct::new().z(0).x(1);
    let pp1: DecoherenceProduct = DecoherenceProduct::new().x(0);
    let _ = system.set((pp0, pp1), CalculatorComplex::from(1.0));

    let unitary_matrix: CooSparseMatrix = (vec![], (vec![], vec![]));
    assert_eq!(system.unitary_sparse_matrix_coo().unwrap(), unitary_matrix);
}
