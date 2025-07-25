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

//! Integration test for public API of PauliLindbladNoiseOperator

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
use struqture::spins::{DecoherenceOperator, DecoherenceProduct, PauliLindbladNoiseOperator};
use struqture::{prelude::*, STRUQTURE_VERSION};
use struqture::{OperateOnDensityMatrix, SpinIndex};
use test_case::test_case;

// Test the new function of the PauliLindbladNoiseOperator
#[test]
fn new() {
    let slno = PauliLindbladNoiseOperator::new();
    assert!(slno.is_empty());
    assert_eq!(
        PauliLindbladNoiseOperator::new(),
        PauliLindbladNoiseOperator::default()
    );
}

#[test]
fn empty_clone_options() {
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slno = PauliLindbladNoiseOperator::new();
    slno.set((dp_0.clone(), dp_0), CalculatorComplex::from(0.5))
        .unwrap();

    let empty: Option<usize> = None;
    let full: Option<usize> = Some(3);
    assert_eq!(slno.empty_clone(empty), PauliLindbladNoiseOperator::new());
    assert_eq!(
        slno.empty_clone(full),
        PauliLindbladNoiseOperator::with_capacity(1)
    );
}

// Test the current_number_spins function of the PauliLindbladNoiseOperator
#[test]
fn internal_map_number_spins() {
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().x(0);
    let dp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slno = PauliLindbladNoiseOperator::new();
    assert_eq!(slno.current_number_spins(), 0_usize);
    slno.set((dp_0.clone(), dp_0), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(slno.current_number_spins(), 1_usize);
    slno.set((dp_2.clone(), dp_2), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(slno.current_number_spins(), 3_usize);
}

// Test the len function of the PauliLindbladNoiseOperator
#[test]
fn internal_map_len() {
    let dp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slno = PauliLindbladNoiseOperator::new();
    slno.set((dp_2.clone(), dp_2), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(slno.len(), 1_usize);
}

// Test the try_set_noise and get functions of the PauliLindbladNoiseOperator
#[test]
fn internal_map_set_get() {
    let dp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slno = PauliLindbladNoiseOperator::new();
    assert_eq!(slno.current_number_spins(), 0_usize);

    // Vacant
    slno.set((dp_2.clone(), dp_2.clone()), CalculatorComplex::from(0.0))
        .unwrap();
    slno.set((dp_2.clone(), dp_2.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(
        slno.get(&(dp_2.clone(), dp_2.clone())),
        &CalculatorComplex::from(0.5)
    );
    assert_eq!(slno.current_number_spins(), 3_usize);

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
    assert_eq!(slno, PauliLindbladNoiseOperator::new());
}

// Test the add_noise function of the PauliLindbladNoiseOperator
#[test]
fn internal_map_add_noise() {
    let dp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slno = PauliLindbladNoiseOperator::new();

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

// Test the add_noise_from_full_operators function of the PauliLindbladNoiseOperator
#[test]
fn internal_map_add_noise_from_full_operators() {
    let mut left: DecoherenceOperator = DecoherenceOperator::new();
    left.add_operator_product(DecoherenceProduct::new().x(0), 0.5.into())
        .unwrap();
    left.add_operator_product(DecoherenceProduct::new().iy(0), 0.5.into())
        .unwrap();

    let mut right: DecoherenceOperator = DecoherenceOperator::new();
    right
        .add_operator_product(DecoherenceProduct::new().x(0), 0.5.into())
        .unwrap();
    right
        .add_operator_product(DecoherenceProduct::new().iy(0), (-0.5).into())
        .unwrap();

    let mut slno = PauliLindbladNoiseOperator::new();

    let _ = slno.add_noise_from_full_operators(&left, &right, CalculatorComplex::from(10.0));

    assert_eq!(
        slno.get(&(
            DecoherenceProduct::new().x(0),
            DecoherenceProduct::new().x(0)
        )),
        &CalculatorComplex::from(2.5)
    );
    assert_eq!(
        slno.get(&(
            DecoherenceProduct::new().x(0),
            DecoherenceProduct::new().iy(0)
        )),
        &CalculatorComplex::from(-2.5)
    );
    assert_eq!(
        slno.get(&(
            DecoherenceProduct::new().iy(0),
            DecoherenceProduct::new().x(0)
        )),
        &CalculatorComplex::from(2.5)
    );
    assert_eq!(
        slno.get(&(
            DecoherenceProduct::new().iy(0),
            DecoherenceProduct::new().iy(0)
        )),
        &CalculatorComplex::from(-2.5)
    );
    assert_eq!(
        slno.get(&(
            DecoherenceProduct::new().iy(0),
            DecoherenceProduct::new().z(0)
        )),
        &CalculatorComplex::from(0.0)
    );
    assert_eq!(
        slno.get(&(
            DecoherenceProduct::new().z(0),
            DecoherenceProduct::new().z(0)
        )),
        &CalculatorComplex::from(0.0)
    );
}

// Test the add_noise_from_full_operators function of the PauliLindbladNoiseOperator
#[test]
fn internal_map_add_noise_from_full_operators_complex() {
    let mut left: DecoherenceOperator = DecoherenceOperator::new();
    left.add_operator_product(
        DecoherenceProduct::new().iy(0),
        CalculatorComplex::new(0.0, 1.0),
    )
    .unwrap();

    let mut right: DecoherenceOperator = DecoherenceOperator::new();
    right
        .add_operator_product(
            DecoherenceProduct::new().iy(0),
            CalculatorComplex::new(0.0, 1.0),
        )
        .unwrap();

    let mut slno = PauliLindbladNoiseOperator::new();

    let _ = slno.add_noise_from_full_operators(&left, &right, 10.into());

    assert_eq!(
        slno.get(&(
            DecoherenceProduct::new().iy(0),
            DecoherenceProduct::new().iy(0)
        )),
        &CalculatorComplex::from(10.0)
    );
}

// Test the Iter traits of PauliLindbladNoiseOperator: into_iter, from_iter and extend
#[test]
fn into_iter_from_iter_extend() {
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let dp_1: DecoherenceProduct = DecoherenceProduct::new().x(1);
    let mut slno_0 = PauliLindbladNoiseOperator::new();
    let _ = slno_0.add_operator_product((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(1.0));

    let slno_iter = slno_0.clone().into_iter();
    assert_eq!(PauliLindbladNoiseOperator::from_iter(slno_iter), slno_0);
    let slno_iter = (&slno_0)
        .into_iter()
        .map(|(key, value)| (key.clone(), value.clone()));
    assert_eq!(PauliLindbladNoiseOperator::from_iter(slno_iter), slno_0);

    let mut mapping: BTreeMap<(DecoherenceProduct, DecoherenceProduct), CalculatorComplex> =
        BTreeMap::new();
    mapping.insert((dp_1.clone(), dp_1.clone()), CalculatorComplex::from(0.5));
    let mapping_iter = mapping.into_iter();
    slno_0.extend(mapping_iter);

    let mut slno_0_1 = PauliLindbladNoiseOperator::new();
    let _ = slno_0_1.add_operator_product((dp_0.clone(), dp_0), CalculatorComplex::from(1.0));
    let _ = slno_0_1.add_operator_product((dp_1.clone(), dp_1), CalculatorComplex::from(0.5));

    assert_eq!(slno_0, slno_0_1);
}

// Test the remap_qubits function of the PauliProduct
#[test]
fn remap_qubits() {
    let dp_1: DecoherenceProduct = DecoherenceProduct::new().z(2).x(0).z(1);
    let dp_2: DecoherenceProduct = DecoherenceProduct::new().iy(2).iy(1).iy(0);
    let mut slno = PauliLindbladNoiseOperator::new();
    slno.set((dp_1.clone(), dp_1), CalculatorComplex::from(0.3))
        .unwrap();
    slno.set((dp_2.clone(), dp_2), CalculatorComplex::from(0.5))
        .unwrap();

    let dp_1_remapped: DecoherenceProduct = DecoherenceProduct::new().z(0).x(1).z(2);
    let dp_2_remapped: DecoherenceProduct = DecoherenceProduct::new().iy(2).iy(1).iy(0);
    let mut slno_remapped = PauliLindbladNoiseOperator::new();
    slno_remapped
        .set(
            (dp_1_remapped.clone(), dp_1_remapped),
            CalculatorComplex::from(0.3),
        )
        .unwrap();
    slno_remapped
        .set(
            (dp_2_remapped.clone(), dp_2_remapped),
            CalculatorComplex::from(0.5),
        )
        .unwrap();

    let mut mapping: HashMap<usize, usize> = HashMap::new();
    mapping.insert(0, 1);
    mapping.insert(1, 2);
    mapping.insert(2, 0);

    assert_eq!(slno.remap_qubits(&mapping), slno_remapped);
}

// Test the negative operation: -PauliLindbladNoiseOperator
#[test]
fn negative_slno() {
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slno_0 = PauliLindbladNoiseOperator::new();
    let _ = slno_0.add_operator_product((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(1.0));
    let mut slno_0_minus = PauliLindbladNoiseOperator::new();
    let _ = slno_0_minus.add_operator_product((dp_0.clone(), dp_0), CalculatorComplex::from(-1.0));

    assert_eq!(-slno_0, slno_0_minus);
}

// Test the addition: PauliLindbladNoiseOperator + PauliLindbladNoiseOperator
#[test]
fn add_slno_slno() {
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let dp_1: DecoherenceProduct = DecoherenceProduct::new().x(1);
    let mut slno_0 = PauliLindbladNoiseOperator::new();
    let _ = slno_0.add_operator_product((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(1.0));
    let mut slno_1 = PauliLindbladNoiseOperator::new();
    let _ = slno_1.add_operator_product((dp_1.clone(), dp_1.clone()), CalculatorComplex::from(0.5));
    let mut slno_0_1 = PauliLindbladNoiseOperator::new();
    let _ = slno_0_1.add_operator_product((dp_0.clone(), dp_0), CalculatorComplex::from(1.0));
    let _ = slno_0_1.add_operator_product((dp_1.clone(), dp_1), CalculatorComplex::from(0.5));

    assert_eq!(slno_0.clone() + slno_1.clone(), slno_0_1);
    assert_eq!(slno_0.add(slno_1), slno_0_1);
}

// Test the subtraction: PauliLindbladNoiseOperator - PauliLindbladNoiseOperator
#[test]
fn sub_slno_slno() {
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let dp_1: DecoherenceProduct = DecoherenceProduct::new().x(1);
    let mut slno_0 = PauliLindbladNoiseOperator::new();
    let _ = slno_0.add_operator_product((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(1.0));
    let mut slno_1 = PauliLindbladNoiseOperator::new();
    let _ = slno_1.add_operator_product((dp_1.clone(), dp_1.clone()), CalculatorComplex::from(0.5));
    let mut slno_0_1 = PauliLindbladNoiseOperator::new();
    let _ = slno_0_1.add_operator_product((dp_0.clone(), dp_0), CalculatorComplex::from(1.0));
    let _ = slno_0_1.add_operator_product((dp_1.clone(), dp_1), CalculatorComplex::from(-0.5));

    assert_eq!(slno_0.clone() - slno_1.clone(), slno_0_1);
    assert_eq!(slno_0.sub(slno_1), slno_0_1);
}

// Test the multiplication: PauliLindbladNoiseOperator * Calculatorcomplex
#[test]
fn mul_so_cc() {
    let pp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut so_0 = PauliLindbladNoiseOperator::new();
    let _ = so_0.set((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(2.0));
    let mut so_0_1 = PauliLindbladNoiseOperator::new();
    let _ = so_0_1.set((pp_0.clone(), pp_0), CalculatorComplex::from(6.0));

    assert_eq!(so_0 * CalculatorComplex::from(3.0), so_0_1);
}

// Test the multiplication: PauliLindbladNoiseOperator * Calculatorcomplex
#[test]
fn mul_so_cf() {
    let pp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut so_0 = PauliLindbladNoiseOperator::new();
    let _ = so_0.add_operator_product((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(2.0));
    let mut so_0_1 = PauliLindbladNoiseOperator::new();
    let _ = so_0_1.add_operator_product((pp_0.clone(), pp_0), CalculatorComplex::from(6.0));

    assert_eq!(so_0 * CalculatorFloat::from(3.0), so_0_1);
}

// Test the Debug trait of PauliLindbladNoiseOperator
#[test]
fn debug() {
    let dp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slno = PauliLindbladNoiseOperator::new();
    let _ = slno.set((dp.clone(), dp), CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{slno:?}"),
        "PauliLindbladNoiseOperator { internal_map: {(DecoherenceProduct { items: [(0, Z)] }, DecoherenceProduct { items: [(0, Z)] }): CalculatorComplex { re: Float(0.5), im: Float(0.0) }} }"
    );
}

// Test the Display trait of DecoherenceOperator
#[test]
fn display() {
    let mut so = PauliLindbladNoiseOperator::new();
    let pp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let _ = so.set((pp.clone(), pp), CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{so}"),
        "PauliLindbladNoiseOperator{\n(0Z, 0Z): (5e-1 + i * 0e0),\n}"
    );
}

// Test the Clone and PartialEq traits of PauliLindbladNoiseOperator
#[test]
fn clone_partial_eq() {
    let dp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slno = PauliLindbladNoiseOperator::new();
    slno.set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    // Test Clone trait
    assert_eq!(slno.clone(), slno);

    // Test PartialEq trait
    let dp_1: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slno_1 = PauliLindbladNoiseOperator::new();
    slno_1
        .set((dp_1.clone(), dp_1), CalculatorComplex::from(0.5))
        .unwrap();
    let dp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slno_2 = PauliLindbladNoiseOperator::new();
    slno_2
        .set((dp_2.clone(), dp_2), CalculatorComplex::from(0.5))
        .unwrap();
    assert!(slno_1 == slno);
    assert!(slno == slno_1);
    assert!(slno_2 != slno);
    assert!(slno != slno_2);
}

// Test the separation of terms
#[test_case(1, 1)]
#[test_case(1, 2)]
#[test_case(1, 3)]
#[test_case(2, 1)]
#[test_case(2, 2)]
#[test_case(2, 3)]
#[test_case(3, 1)]
#[test_case(3, 2)]
#[test_case(3, 3)]
fn separate_out_terms(number_spins_left: usize, number_spins_right: usize) {
    let pp_1_a: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let pp_2_a: DecoherenceProduct = DecoherenceProduct::new().z(0).x(2);
    let pp_3_a: DecoherenceProduct = DecoherenceProduct::new().z(0).z(1).z(2);

    let mut allowed: Vec<(DecoherenceProduct, DecoherenceProduct, f64)> = Vec::new();
    let mut not_allowed: Vec<(DecoherenceProduct, DecoherenceProduct, f64)> = vec![
        (pp_1_a.clone(), pp_1_a.clone(), 1.0),
        (pp_1_a.clone(), pp_2_a.clone(), 1.0),
        (pp_1_a.clone(), pp_3_a.clone(), 1.0),
        (pp_2_a.clone(), pp_1_a.clone(), 1.0),
        (pp_2_a.clone(), pp_2_a.clone(), 1.0),
        (pp_2_a.clone(), pp_3_a.clone(), 1.0),
        (pp_3_a.clone(), pp_1_a.clone(), 1.0),
        (pp_3_a.clone(), pp_2_a.clone(), 1.0),
        (pp_3_a.clone(), pp_3_a.clone(), 1.0),
    ];

    match (number_spins_left, number_spins_right) {
        (1, 1) => {
            allowed.push(not_allowed[0].clone());
            not_allowed.remove(0);
        }
        (1, 2) => {
            allowed.push(not_allowed[1].clone());
            not_allowed.remove(1);
        }
        (1, 3) => {
            allowed.push(not_allowed[2].clone());
            not_allowed.remove(2);
        }
        (2, 1) => {
            allowed.push(not_allowed[3].clone());
            not_allowed.remove(3);
        }
        (2, 2) => {
            allowed.push(not_allowed[4].clone());
            not_allowed.remove(4);
        }
        (2, 3) => {
            allowed.push(not_allowed[5].clone());
            not_allowed.remove(5);
        }
        (3, 1) => {
            allowed.push(not_allowed[6].clone());
            not_allowed.remove(6);
        }
        (3, 2) => {
            allowed.push(not_allowed[7].clone());
            not_allowed.remove(7);
        }
        (3, 3) => {
            allowed.push(not_allowed[8].clone());
            not_allowed.remove(8);
        }
        _ => panic!(),
    }

    let mut separated = PauliLindbladNoiseOperator::new();
    for (key_l, key_r, value) in allowed.iter() {
        separated
            .add_operator_product((key_l.clone(), key_r.clone()), value.into())
            .unwrap();
    }
    let mut remainder = PauliLindbladNoiseOperator::new();
    for (key_l, key_r, value) in not_allowed.iter() {
        remainder
            .add_operator_product((key_l.clone(), key_r.clone()), value.into())
            .unwrap();
    }

    let mut so = PauliLindbladNoiseOperator::new();
    so.add_operator_product(
        (pp_1_a.clone(), pp_1_a.clone()),
        CalculatorComplex::from(1.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_1_a.clone(), pp_2_a.clone()),
        CalculatorComplex::from(1.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_1_a.clone(), pp_3_a.clone()),
        CalculatorComplex::from(1.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_2_a.clone(), pp_1_a.clone()),
        CalculatorComplex::from(1.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_2_a.clone(), pp_2_a.clone()),
        CalculatorComplex::from(1.0),
    )
    .unwrap();
    so.add_operator_product(
        (pp_2_a.clone(), pp_3_a.clone()),
        CalculatorComplex::from(1.0),
    )
    .unwrap();
    so.add_operator_product((pp_3_a.clone(), pp_1_a), CalculatorComplex::from(1.0))
        .unwrap();
    so.add_operator_product((pp_3_a.clone(), pp_2_a), CalculatorComplex::from(1.0))
        .unwrap();
    so.add_operator_product((pp_3_a.clone(), pp_3_a), CalculatorComplex::from(1.0))
        .unwrap();

    let result = so
        .separate_into_n_terms(number_spins_left, number_spins_right)
        .unwrap();
    assert_eq!(result.0, separated);
    assert_eq!(result.1, remainder);
}

/// Test PauliLindbladNoiseOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_json() {
    let dp = DecoherenceProduct::new().x(0);
    let mut slno = PauliLindbladNoiseOperator::new();
    slno.set((dp.clone(), dp), CalculatorComplex::from(1.0))
        .unwrap();

    let serialized = serde_json::to_string(&slno).unwrap();
    let deserialized: PauliLindbladNoiseOperator = serde_json::from_str(&serialized).unwrap();
    assert_eq!(slno, deserialized);
}

/// Test PauliLindbladNoiseOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_readable() {
    let dp = DecoherenceProduct::new().x(0);
    let mut slno = PauliLindbladNoiseOperator::new();
    slno.set((dp.clone(), dp), CalculatorComplex::from(1.0))
        .unwrap();

    assert_tokens(
        &slno.readable(),
        &[
            Token::Struct {
                name: "PauliLindbladNoiseOperatorSerialize",
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
            Token::Str("serialisation_meta"),
            Token::Struct {
                name: "StruqtureSerialisationMeta",
                len: 3,
            },
            Token::Str("type_name"),
            Token::Str("PauliLindbladNoiseOperator"),
            Token::Str("min_version"),
            Token::Tuple { len: 3 },
            Token::U64(2),
            Token::U64(0),
            Token::U64(0),
            Token::TupleEnd,
            Token::Str("version"),
            Token::Str(STRUQTURE_VERSION),
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

#[test]
fn bincode() {
    let dp = DecoherenceProduct::new().x(0);
    let mut slno = PauliLindbladNoiseOperator::new();
    slno.set((dp.clone(), dp), CalculatorComplex::from(1.0))
        .unwrap();

    let config = bincode::config::legacy();

    let serialized = bincode::serde::encode_to_vec(&slno, config).unwrap();
    let (deserialized, _len): (PauliLindbladNoiseOperator, usize) =
        bincode::serde::decode_from_slice(&serialized, config).unwrap();
    assert_eq!(deserialized, slno);

    let encoded: Vec<u8> = bincode::serde::encode_to_vec(slno.clone().compact(), config).unwrap();
    let (decoded, _len): (PauliLindbladNoiseOperator, usize) =
        bincode::serde::decode_from_slice(&encoded, config).unwrap();
    assert_eq!(slno, decoded);
}

/// Test PauliLindbladNoiseOperator Serialization and Deserialization traits (compact)
#[test]
fn serde_compact() {
    let dp = DecoherenceProduct::new().x(0);
    let mut slno = PauliLindbladNoiseOperator::new();
    slno.set((dp.clone(), dp), CalculatorComplex::from(1.0))
        .unwrap();

    assert_tokens(
        &slno.compact(),
        &[
            Token::Struct {
                name: "PauliLindbladNoiseOperatorSerialize",
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
            Token::Str("serialisation_meta"),
            Token::Struct {
                name: "StruqtureSerialisationMeta",
                len: 3,
            },
            Token::Str("type_name"),
            Token::Str("PauliLindbladNoiseOperator"),
            Token::Str("min_version"),
            Token::Tuple { len: 3 },
            Token::U64(2),
            Token::U64(0),
            Token::U64(0),
            Token::TupleEnd,
            Token::Str("version"),
            Token::Str(STRUQTURE_VERSION),
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
fn test_superoperator(
    left_representation: &str,
    right_representation: &str,
    left_operators: &[&str],
    right_operators: &[&str],
) {
    let mut system = PauliLindbladNoiseOperator::new();
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
        .sparse_matrix_superoperator(left_operators.len())
        .unwrap();
    let (test_vals, (test_rows, test_columns)) = system
        .sparse_matrix_superoperator_coo(left_operators.len())
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
}

#[test]
fn test_superoperator_multiple_entries() {
    let mut system = PauliLindbladNoiseOperator::new();

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
    let second_test_matrix = system.sparse_matrix_superoperator(2).unwrap();

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

    let (test_vals, (test_rows, test_columns)) = system.sparse_matrix_superoperator_coo(2).unwrap();
    for (second_val, (row, column)) in test_vals
        .iter()
        .zip(test_rows.iter().zip(test_columns.iter()))
    {
        let val = test_matrix[(*row, *column)];
        assert_eq!(&val, second_val);
    }
}

// Test the failure of creating the PauliLindbladNoiseOperator with identity terms
#[test]
fn illegal_identity_operators() {
    let empty_fp = DecoherenceProduct::new();
    let fp = DecoherenceProduct::new().x(0);
    let mut fno_left_identity = PauliLindbladNoiseOperator::new();
    let cc = CalculatorComplex::new(1.0, 1.0);
    let ok = fno_left_identity
        .add_operator_product((empty_fp.clone(), fp.clone()), cc.clone())
        .is_err();
    assert!(ok);
    let mut fno_right_identity = PauliLindbladNoiseOperator::new();
    let ok = fno_right_identity
        .add_operator_product((fp, empty_fp.clone()), cc.clone())
        .is_err();
    assert!(ok);
    let mut fno_both_identity = PauliLindbladNoiseOperator::new();
    let ok = fno_both_identity
        .add_operator_product((empty_fp.clone(), empty_fp), cc)
        .is_err();
    assert!(ok);
}

#[cfg(feature = "json_schema")]
#[test]
fn test_noise_operator_schema() {
    let mut op = PauliLindbladNoiseOperator::new();
    op.set(
        (
            DecoherenceProduct::new().x(0),
            DecoherenceProduct::new().x(0),
        ),
        1.0.into(),
    )
    .unwrap();
    op.set(
        (
            DecoherenceProduct::new().x(0),
            DecoherenceProduct::new().iy(1).z(2),
        ),
        "val".into(),
    )
    .unwrap();
    let schema = schemars::schema_for!(PauliLindbladNoiseOperator);
    let schema_checker = jsonschema::validator_for(&serde_json::to_value(&schema).unwrap())
        .expect("schema is valid");
    let value = serde_json::to_value(&op).unwrap();
    let val = match value {
        serde_json::Value::Object(ob) => ob,
        _ => panic!(),
    };
    let value: serde_json::Value = serde_json::to_value(val).unwrap();
    let validation = schema_checker.validate(&value);
    assert!(validation.is_ok());
}

#[cfg(feature = "struqture_1_import")]
#[cfg(feature = "struqture_1_export")]
#[test]
fn test_from_to_struqture_1() {
    let pp_1 = struqture_1::spins::DecoherenceProduct::from_str("0X1iY25Z").unwrap();
    let mut ss_1 = struqture_1::spins::SpinLindbladNoiseSystem::new(None);
    struqture_1::OperateOnDensityMatrix::set(&mut ss_1, (pp_1.clone(), pp_1.clone()), 1.0.into())
        .unwrap();

    let pp_2 = DecoherenceProduct::new().x(0).iy(1).z(25);
    let mut ss_2 = PauliLindbladNoiseOperator::new();
    ss_2.set((pp_2.clone(), pp_2.clone()), 1.0.into()).unwrap();

    assert!(PauliLindbladNoiseOperator::from_struqture_1(&ss_1).unwrap() == ss_2);
    assert!(ss_1 == ss_2.to_struqture_1().unwrap());
}
