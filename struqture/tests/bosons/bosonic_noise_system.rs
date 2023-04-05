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

//! Integration test for public API of BosonLindbladNoiseSystem

use bincode::{deserialize, serialize};
use qoqo_calculator::CalculatorComplex;
use serde_test::{assert_tokens, Configure, Token};
use std::collections::BTreeMap;
use std::iter::{FromIterator, IntoIterator};
use struqture::bosons::{BosonLindbladNoiseOperator, BosonLindbladNoiseSystem, BosonProduct};
use struqture::{ModeIndex, OperateOnDensityMatrix, OperateOnModes, StruqtureError};
use test_case::test_case;

// Test the new function of the SpinLindbladNoiseSystem
#[test]
fn new_system() {
    let system = BosonLindbladNoiseSystem::new(Some(1));
    assert!(system.is_empty());
    assert_eq!(system.operator(), &BosonLindbladNoiseOperator::default());
    assert_eq!(system.current_number_modes(), 1_usize);
    assert_eq!(system.number_modes(), 1_usize);
}

// Test the new function of the SpinLindbladNoiseSystem with no spins specified
#[test]
fn new_system_none() {
    let system = BosonLindbladNoiseSystem::new(None);
    assert!(system.operator().is_empty());
    assert_eq!(system.operator(), &BosonLindbladNoiseOperator::default());
    assert_eq!(system.current_number_modes(), 0_usize);
    assert_eq!(system.number_modes(), 0_usize);
}

// Test the from_spin_operator and spin_operator functions of the SpinLindbladNoiseSystem with number_spins = None
#[test]
fn from_noise_operator_none() {
    let mut slno: BosonLindbladNoiseOperator = BosonLindbladNoiseOperator::new();
    let mut slns = BosonLindbladNoiseSystem::new(None);
    let dp_0: BosonProduct = BosonProduct::new([0], [2]).unwrap();
    slno.set((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    slns.set((dp_0.clone(), dp_0), CalculatorComplex::from(0.5))
        .unwrap();

    assert_eq!(
        slns,
        BosonLindbladNoiseSystem::from_operator(slno.clone(), None).unwrap()
    );
    assert_eq!(
        slns.operator(),
        BosonLindbladNoiseSystem::from_operator(slno.clone(), None)
            .unwrap()
            .operator()
    );
    assert_eq!(
        &slno,
        BosonLindbladNoiseSystem::from_operator(slno.clone(), None)
            .unwrap()
            .operator()
    );
}

// Test the from_spin_operator and spin_operator functions of the SpinLindbladNoiseSystem with number_spins = Some(2)
#[test]
fn from_noise_operator_some() {
    let mut slno: BosonLindbladNoiseOperator = BosonLindbladNoiseOperator::new();
    let mut system = BosonLindbladNoiseSystem::new(Some(3));
    let dp_0: BosonProduct = BosonProduct::new([0], [2]).unwrap();
    slno.add_operator_product((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(1.0))
        .unwrap();
    system
        .add_operator_product((dp_0.clone(), dp_0), CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(
        system,
        BosonLindbladNoiseSystem::from_operator(slno.clone(), Some(3)).unwrap()
    );
    assert_eq!(
        system.operator(),
        BosonLindbladNoiseSystem::from_operator(slno.clone(), Some(3))
            .unwrap()
            .operator()
    );
    assert_eq!(
        &slno,
        BosonLindbladNoiseSystem::from_operator(slno.clone(), Some(3))
            .unwrap()
            .operator()
    );
    assert_eq!(
        BosonLindbladNoiseSystem::from_operator(slno.clone(), Some(0)),
        Err(StruqtureError::NumberModesExceeded {})
    );
}

#[test]
fn empty_clone_options() {
    let pp_2: BosonProduct = BosonProduct::new([0], [2]).unwrap();
    let mut system = BosonLindbladNoiseSystem::new(Some(3));
    system
        .set((pp_2.clone(), pp_2), CalculatorComplex::from(0.5))
        .unwrap();

    let empty: Option<usize> = None;
    let full: Option<usize> = Some(3);
    assert_eq!(
        system.empty_clone(empty),
        BosonLindbladNoiseSystem::new(full)
    );
    assert_eq!(
        system.empty_clone(full),
        BosonLindbladNoiseSystem::with_capacity(full, 1)
    );
}

// Test the current_number_modes function of the BosonLindbladNoiseSystem
#[test]
fn internal_map_current_number_modes() {
    let pp_0: BosonProduct = BosonProduct::new([0], [1]).unwrap();
    let pp_2: BosonProduct = BosonProduct::new([2], [3]).unwrap();
    let mut so = BosonLindbladNoiseSystem::new(Some(4));
    assert_eq!(so.current_number_modes(), 4_usize);
    so.set((pp_0.clone(), pp_0), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(so.current_number_modes(), 4_usize);
    so.set((pp_2.clone(), pp_2), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(so.current_number_modes(), 4_usize);
}

// Test the len function of the BosonLindbladNoiseSystem
#[test]
fn internal_map_len() {
    let pp_2: BosonProduct = BosonProduct::new([0], [2]).unwrap();
    let mut so = BosonLindbladNoiseSystem::new(Some(3));
    so.set((pp_2.clone(), pp_2), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(so.len(), 1_usize);
}
// Test the set, set_pauli_product, get functions of the SpinSystem
#[test]
fn internal_map_set_get_dict() {
    let mut system = BosonLindbladNoiseSystem::new(Some(1));
    assert_eq!(system.current_number_modes(), 1_usize);
    assert_eq!(system.number_modes(), 1_usize);
    let pp_0: BosonProduct = BosonProduct::new([0], [0]).unwrap();

    // 1) Test try_set_pauli_product and get functions
    // Vacant
    system
        .set((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(0.0))
        .unwrap();
    system
        .set((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(system.current_number_modes(), 1_usize);
    assert_eq!(system.number_modes(), 1_usize);
    assert_eq!(
        system.get(&(pp_0.clone(), pp_0.clone())),
        &CalculatorComplex::from(0.5)
    );

    // 2) Test iter, keys, values functions
    let mut map: BTreeMap<(BosonProduct, BosonProduct), CalculatorComplex> = BTreeMap::new();
    map.insert((pp_0.clone(), pp_0), CalculatorComplex::from(0.5));
    // iter
    let dict = system.iter();
    for (item_d, item_m) in dict.zip(map.iter()) {
        assert_eq!(item_d, item_m);
    }
    // keys
    let keys = system.keys();
    for (key_s, key_m) in keys.zip(map.keys()) {
        assert_eq!(key_s, key_m);
    }
    // values
    let values = system.values();
    for (val_s, val_m) in values.zip(map.values()) {
        assert_eq!(val_s, val_m);
    }
}

// Test the set, get and remove functions of the BosonLindbladNoiseSystem
#[test]
fn internal_map_set_get_remove() {
    let pp_2: BosonProduct = BosonProduct::new([0], [2]).unwrap();
    let mut so = BosonLindbladNoiseSystem::new(Some(3));

    // 1) Test try_set_boson_product and get functions
    // Vacant
    so.set((pp_2.clone(), pp_2.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(
        so.get(&(pp_2.clone(), pp_2.clone())),
        &CalculatorComplex::from(0.5)
    );

    // 2) Test remove function
    so.remove(&(pp_2.clone(), pp_2));
    assert_eq!(so, BosonLindbladNoiseSystem::new(Some(3)));
}

// Test the add_operator_product function of the BosonLindbladNoiseSystem
#[test]
fn internal_map_add_operator_product() {
    let pp_2: BosonProduct = BosonProduct::new([0], [2]).unwrap();
    let mut so = BosonLindbladNoiseSystem::new(Some(3));

    so.add_operator_product((pp_2.clone(), pp_2.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(
        so.get(&(pp_2.clone(), pp_2.clone())),
        &CalculatorComplex::from(0.5)
    );
    so.add_operator_product((pp_2.clone(), pp_2.clone()), CalculatorComplex::from(-0.5))
        .unwrap();
    assert_eq!(so.get(&(pp_2.clone(), pp_2)), &CalculatorComplex::from(0.0));
}

// Test the iter, keys and values functions of the BosonLindbladNoiseSystem
#[test]
fn internal_map_keys() {
    let pp_2: BosonProduct = BosonProduct::new([0], [2]).unwrap();
    let mut so = BosonLindbladNoiseSystem::new(Some(3));
    so.set((pp_2.clone(), pp_2.clone()), CalculatorComplex::from(0.5))
        .unwrap();

    let mut map: BTreeMap<(BosonProduct, BosonProduct), CalculatorComplex> = BTreeMap::new();
    map.insert((pp_2.clone(), pp_2), CalculatorComplex::from(0.5));

    // iter
    let dict = so.iter();
    for (item_d, item_m) in dict.zip(map.iter()) {
        assert_eq!(item_d, item_m);
    }
    // keys
    let keys = so.keys();
    for (key_s, key_m) in keys.zip(map.keys()) {
        assert_eq!(key_s, key_m);
    }
    // values
    let values = so.values();
    for (val_s, val_m) in values.zip(map.values()) {
        assert_eq!(val_s, val_m);
    }
}

// Test the separation of terms
#[test_case((1, 1), (1, 1))]
#[test_case((1, 1), (2, 2))]
#[test_case((2, 2), (1, 2))]
#[test_case((1, 2), (2, 1))]
fn separate_out_terms(number_spins_left: (usize, usize), number_spins_right: (usize, usize)) {
    let pp_1_a: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let pp_2_a: BosonProduct = BosonProduct::new([0, 1], [1]).unwrap();
    let pp_2_b: BosonProduct = BosonProduct::new([0], [0, 1]).unwrap();
    let pp_3_a: BosonProduct = BosonProduct::new([0, 1], [0, 1]).unwrap();

    let mut allowed: Vec<(BosonProduct, BosonProduct, f64)> = Vec::new();
    let mut not_allowed: Vec<(BosonProduct, BosonProduct, f64)> = vec![
        (pp_1_a.clone(), pp_1_a.clone(), 1.0),
        (pp_1_a.clone(), pp_3_a.clone(), 1.0),
        (pp_3_a.clone(), pp_2_b.clone(), 1.0),
        (pp_2_b.clone(), pp_2_a.clone(), 1.0),
    ];

    match (number_spins_left, number_spins_right) {
        ((1, 1), (1, 1)) => {
            allowed.push(not_allowed[0].clone());
            not_allowed.remove(0);
        }
        ((1, 1), (2, 2)) => {
            allowed.push(not_allowed[1].clone());
            not_allowed.remove(1);
        }
        ((2, 2), (1, 2)) => {
            allowed.push(not_allowed[2].clone());
            not_allowed.remove(2);
        }
        ((1, 2), (2, 1)) => {
            allowed.push(not_allowed[3].clone());
            not_allowed.remove(3);
        }
        _ => panic!(),
    }

    let mut separated = BosonLindbladNoiseSystem::default();
    for (key_l, key_r, value) in allowed.iter() {
        separated
            .add_operator_product((key_l.clone(), key_r.clone()), value.into())
            .unwrap();
    }
    let mut remainder = BosonLindbladNoiseSystem::default();
    for (key_l, key_r, value) in not_allowed.iter() {
        remainder
            .add_operator_product((key_l.clone(), key_r.clone()), value.into())
            .unwrap();
    }

    let mut so = BosonLindbladNoiseSystem::default();
    so.add_operator_product(
        (pp_1_a.clone(), pp_1_a.clone()),
        CalculatorComplex::from(1.0),
    )
    .unwrap();
    so.add_operator_product((pp_1_a, pp_3_a.clone()), CalculatorComplex::from(1.0))
        .unwrap();
    so.add_operator_product((pp_3_a, pp_2_b.clone()), CalculatorComplex::from(1.0))
        .unwrap();
    so.add_operator_product((pp_2_b, pp_2_a), CalculatorComplex::from(1.0))
        .unwrap();

    let result = so
        .separate_into_n_terms(number_spins_left, number_spins_right)
        .unwrap();
    assert_eq!(result.0, separated);
    assert_eq!(result.1, remainder);
}

// Test the negative operation: -BosonLindbladNoiseSystem
#[test]
fn negative_so() {
    let pp_0: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut so_0 = BosonLindbladNoiseSystem::new(Some(1));
    let _ = so_0.add_operator_product((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(1.0));
    let mut so_0_minus = BosonLindbladNoiseSystem::new(Some(1));
    let _ = so_0_minus.add_operator_product((pp_0.clone(), pp_0), CalculatorComplex::from(-1.0));

    assert_eq!(-so_0, so_0_minus);
}

// Test the addition: BosonLindbladNoiseSystem + BosonLindbladNoiseSystem
#[test]
fn add_so_so() {
    let pp_0: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let pp_1: BosonProduct = BosonProduct::new([0], [1]).unwrap();
    let mut so_0 = BosonLindbladNoiseSystem::new(Some(2));
    let _ = so_0.add_operator_product((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(1.0));
    let mut so_1 = BosonLindbladNoiseSystem::new(Some(2));
    let _ = so_1.add_operator_product((pp_1.clone(), pp_1.clone()), CalculatorComplex::from(0.5));
    let mut so_0_1 = BosonLindbladNoiseSystem::new(Some(2));
    let _ = so_0_1.add_operator_product((pp_0.clone(), pp_0), CalculatorComplex::from(1.0));
    let _ = so_0_1.add_operator_product((pp_1.clone(), pp_1), CalculatorComplex::from(0.5));

    assert_eq!(so_0 + so_1, Ok(so_0_1));
}

// Test the subtraction: BosonLindbladNoiseSystem - BosonLindbladNoiseSystem
#[test]
fn sub_so_so() {
    let pp_0: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let pp_1: BosonProduct = BosonProduct::new([0], [1]).unwrap();
    let mut so_0 = BosonLindbladNoiseSystem::new(Some(2));
    let _ = so_0.add_operator_product((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(1.0));
    let mut so_1 = BosonLindbladNoiseSystem::new(Some(2));
    let _ = so_1.add_operator_product((pp_1.clone(), pp_1.clone()), CalculatorComplex::from(0.5));
    let mut so_0_1 = BosonLindbladNoiseSystem::new(Some(2));
    let _ = so_0_1.add_operator_product((pp_0.clone(), pp_0), CalculatorComplex::from(1.0));
    let _ = so_0_1.add_operator_product((pp_1.clone(), pp_1), CalculatorComplex::from(-0.5));

    assert_eq!(so_0 - so_1, Ok(so_0_1));
}

// Test the multiplication: BosonLindbladNoiseSystem * Calculatorcomplex
#[test]
fn mul_so_cc() {
    let pp_0: BosonProduct = BosonProduct::new([0], [2]).unwrap();
    let mut so_0 = BosonLindbladNoiseSystem::new(Some(3));
    let _ = so_0.add_operator_product((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(2.0));
    let mut so_0_1 = BosonLindbladNoiseSystem::new(Some(3));
    let _ = so_0_1.add_operator_product((pp_0.clone(), pp_0), CalculatorComplex::from(6.0));

    assert_eq!(so_0 * CalculatorComplex::from(3.0), so_0_1);
}

// Test the Iter traits of BosonLindbladNoiseSystem: into_iter, from_iter and extend
#[test]
fn into_iter_from_iter_extend() {
    let pp_0: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let pp_1: BosonProduct = BosonProduct::new([0], [1]).unwrap();
    let mut so_0 = BosonLindbladNoiseSystem::new(None);
    let _ = so_0.add_operator_product((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(1.0));

    let so_iter = so_0.clone().into_iter();
    assert_eq!(BosonLindbladNoiseSystem::from_iter(so_iter), so_0);
    let system_iter = (&so_0)
        .into_iter()
        .map(|(key, value)| (key.clone(), value.clone()));
    assert_eq!(BosonLindbladNoiseSystem::from_iter(system_iter), so_0);
    let mut mapping: BTreeMap<(BosonProduct, BosonProduct), CalculatorComplex> = BTreeMap::new();
    mapping.insert((pp_1.clone(), pp_1.clone()), CalculatorComplex::from(0.5));
    let mapping_iter = mapping.into_iter();
    so_0.extend(mapping_iter);

    let mut so_0_1 = BosonLindbladNoiseSystem::new(None);
    let _ = so_0_1.add_operator_product((pp_0.clone(), pp_0), CalculatorComplex::from(1.0));
    let _ = so_0_1.add_operator_product((pp_1.clone(), pp_1), CalculatorComplex::from(0.5));

    assert_eq!(so_0, so_0_1);
}

// Test the Debug trait of BosonLindbladNoiseSystem
#[test]
fn debug() {
    let pp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut so = BosonLindbladNoiseSystem::new(Some(1));
    let _ = so.set((pp.clone(), pp), CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{:?}", so),
        "BosonLindbladNoiseSystem { number_modes: Some(1), operator: BosonLindbladNoiseOperator { internal_map: {(BosonProduct { creators: [0], annihilators: [0] }, BosonProduct { creators: [0], annihilators: [0] }): CalculatorComplex { re: Float(0.5), im: Float(0.0) }} } }"
    );
}

// Test the Display trait of BosonLindbladNoiseSystem
#[test]
fn display() {
    let pp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut so = BosonLindbladNoiseSystem::new(Some(1));
    let _ = so.set((pp.clone(), pp), CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{}", so),
        "BosonLindbladNoiseSystem(1){\n(c0a0, c0a0): (5e-1 + i * 0e0),\n}"
    );
}

// Test the Clone and PartialEq traits of BosonLindbladNoiseSystem
#[test]
fn clone_partial_eq() {
    let pp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut so = BosonLindbladNoiseSystem::new(Some(1));
    so.set((pp.clone(), pp), CalculatorComplex::from(0.5))
        .unwrap();

    // Test Clone trait
    assert_eq!(so.clone(), so);

    // Test PartialEq trait
    let pp_1: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut so_1 = BosonLindbladNoiseSystem::new(Some(1));
    so_1.set((pp_1.clone(), pp_1), CalculatorComplex::from(0.5))
        .unwrap();
    let pp_2: BosonProduct = BosonProduct::new([0], [2]).unwrap();
    let mut so_2 = BosonLindbladNoiseSystem::new(Some(3));
    so_2.set((pp_2.clone(), pp_2), CalculatorComplex::from(0.5))
        .unwrap();
    assert!(so_1 == so);
    assert!(so == so_1);
    assert!(so_2 != so);
    assert!(so != so_2);
}

/// Test SpinOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_json() {
    let pp = BosonProduct::new([0], [1]).unwrap();
    let mut so = BosonLindbladNoiseSystem::new(Some(2));
    so.set((pp.clone(), pp), CalculatorComplex::from(1.0))
        .unwrap();

    let serialized = serde_json::to_string(&so).unwrap();
    let deserialized: BosonLindbladNoiseSystem = serde_json::from_str(&serialized).unwrap();

    assert_eq!(so, deserialized);
}

/// Test SpinOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_readable() {
    use struqture::MINIMUM_STRUQTURE_VERSION;
    let major_version = MINIMUM_STRUQTURE_VERSION.0;
    let minor_version = MINIMUM_STRUQTURE_VERSION.1;

    let pp = BosonProduct::new([0], [1]).unwrap();
    let mut so = BosonLindbladNoiseSystem::new(Some(2));
    so.set((pp.clone(), pp), CalculatorComplex::from(1.0))
        .unwrap();

    assert_tokens(
        &so.readable(),
        &[
            Token::Struct {
                name: "BosonLindbladNoiseSystem",
                len: 2,
            },
            Token::Str("number_modes"),
            Token::Some,
            Token::U64(2),
            Token::Str("operator"),
            Token::Struct {
                name: "BosonLindbladNoiseOperatorSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 4 },
            Token::Str("c0a1"),
            Token::Str("c0a1"),
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
    let pp = BosonProduct::new([0], [1]).unwrap();
    let mut so = BosonLindbladNoiseSystem::new(Some(2));
    so.set((pp.clone(), pp), CalculatorComplex::from(1.0))
        .unwrap();

    let serialized = serialize(&so).unwrap();
    let deserialized: BosonLindbladNoiseSystem = deserialize(&serialized).unwrap();
    assert_eq!(deserialized, so);

    let encoded: Vec<u8> = bincode::serialize(&so.clone().compact()).unwrap();
    let decoded: BosonLindbladNoiseSystem = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(so, decoded);
}

#[test]
fn serde_compact() {
    use struqture::MINIMUM_STRUQTURE_VERSION;
    let major_version = MINIMUM_STRUQTURE_VERSION.0;
    let minor_version = MINIMUM_STRUQTURE_VERSION.1;

    let pp = BosonProduct::new([0], [1]).unwrap();
    let mut so = BosonLindbladNoiseSystem::new(Some(2));
    so.set((pp.clone(), pp), CalculatorComplex::from(1.0))
        .unwrap();

    assert_tokens(
        &so.compact(),
        &[
            Token::Struct {
                name: "BosonLindbladNoiseSystem",
                len: 2,
            },
            Token::Str("number_modes"),
            Token::Some,
            Token::U64(2),
            Token::Str("operator"),
            Token::Struct {
                name: "BosonLindbladNoiseOperatorSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 4 },
            Token::Tuple { len: 2 },
            Token::Seq { len: Some(1) },
            Token::U64(0),
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::U64(1),
            Token::SeqEnd,
            Token::TupleEnd,
            Token::Tuple { len: 2 },
            Token::Seq { len: Some(1) },
            Token::U64(0),
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::U64(1),
            Token::SeqEnd,
            Token::TupleEnd,
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
