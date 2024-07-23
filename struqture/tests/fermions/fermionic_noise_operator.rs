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

//! Integration test for public API of FermionLindbladNoiseOperator

use bincode::{deserialize, serialize};
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde_test::{assert_tokens, Configure, Token};
use std::collections::BTreeMap;
use std::iter::{FromIterator, IntoIterator};
#[cfg(feature = "struqture_1_import")]
#[cfg(feature = "struqture_1_export")]
use std::str::FromStr;
use struqture::fermions::{FermionLindbladNoiseOperator, FermionOperator, FermionProduct};
use struqture::{ModeIndex, OperateOnDensityMatrix, OperateOnModes};

// Test the new function of the FermionLindbladNoiseOperator
#[test]
fn new() {
    let so = FermionLindbladNoiseOperator::new();
    assert!(so.is_empty());
    assert_eq!(
        FermionLindbladNoiseOperator::new(),
        FermionLindbladNoiseOperator::default()
    );
    assert_eq!(
        FermionLindbladNoiseOperator::with_capacity(2),
        FermionLindbladNoiseOperator::new()
    );
}

#[test]
fn empty_clone_options() {
    let pp_2: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut system = FermionLindbladNoiseOperator::new();
    system
        .set((pp_2.clone(), pp_2), CalculatorComplex::from(0.5))
        .unwrap();

    let empty: Option<usize> = None;
    let full: Option<usize> = Some(3);
    assert_eq!(
        system.empty_clone(empty),
        FermionLindbladNoiseOperator::new()
    );
    assert_eq!(
        system.empty_clone(full),
        FermionLindbladNoiseOperator::with_capacity(1)
    );
}

// Test the len function of the FermionLindbladNoiseOperator
#[test]
fn internal_map_len() {
    let pp_2: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut so = FermionLindbladNoiseOperator::new();
    so.set((pp_2.clone(), pp_2), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(so.len(), 1_usize);
}
// Test the set, set_pauli_product, get functions of the SpinSystem
#[test]
fn internal_map_set_get_dict() {
    let mut system = FermionLindbladNoiseOperator::new();
    assert_eq!(system.current_number_modes(), 0_usize);
    let pp_0: FermionProduct = FermionProduct::new([0], [0]).unwrap();

    // 1) Test try_set_pauli_product and get functions
    // Vacant
    system
        .set((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(0.0))
        .unwrap();
    system
        .set((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(system.current_number_modes(), 1_usize);
    assert_eq!(
        system.get(&(pp_0.clone(), pp_0.clone())),
        &CalculatorComplex::from(0.5)
    );

    // 2) Test iter, keys, values functions
    let mut map: BTreeMap<(FermionProduct, FermionProduct), CalculatorComplex> = BTreeMap::new();
    map.insert((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(0.5));
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

    system
        .set((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(0.0))
        .unwrap();
    assert_eq!(
        system.get(&(pp_0.clone(), pp_0)),
        &CalculatorComplex::from(0.0)
    );
}

// Test the set, get and remove functions of the FermionLindbladNoiseOperator
#[test]
fn internal_map_set_get_remove() {
    let pp_2: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut so = FermionLindbladNoiseOperator::new();

    // 1) Test try_set_fermion_product and get functions
    // Vacant
    so.set((pp_2.clone(), pp_2.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(
        so.get(&(pp_2.clone(), pp_2.clone())),
        &CalculatorComplex::from(0.5)
    );

    // 2) Test remove function
    so.remove(&(pp_2.clone(), pp_2));
    assert_eq!(so, FermionLindbladNoiseOperator::new());
}

// Test the add_operator_product function of the FermionLindbladNoiseOperator
#[test]
fn internal_map_add_operator_product() {
    let pp_2: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut so = FermionLindbladNoiseOperator::new();

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

// Test the iter, keys and values functions of the FermionLindbladNoiseOperator
#[test]
fn internal_map_keys() {
    let pp_2: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut so = FermionLindbladNoiseOperator::new();
    so.set((pp_2.clone(), pp_2.clone()), CalculatorComplex::from(0.5))
        .unwrap();

    let mut map: BTreeMap<(FermionProduct, FermionProduct), CalculatorComplex> = BTreeMap::new();
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

// Test the negative operation: -FermionLindbladNoiseOperator
#[test]
fn negative_so() {
    let pp_0: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let mut so_0 = FermionLindbladNoiseOperator::new();
    let _ = so_0.add_operator_product((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(1.0));
    let mut so_0_minus = FermionLindbladNoiseOperator::new();
    let _ = so_0_minus.add_operator_product((pp_0.clone(), pp_0), CalculatorComplex::from(-1.0));

    assert_eq!(-so_0, so_0_minus);
}

// Test the addition: FermionLindbladNoiseOperator + FermionLindbladNoiseOperator
#[test]
fn add_so_so() {
    let pp_0: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let pp_1: FermionProduct = FermionProduct::new([0], [1]).unwrap();
    let mut so_0 = FermionLindbladNoiseOperator::new();
    let _ = so_0.add_operator_product((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(1.0));
    let mut so_1 = FermionLindbladNoiseOperator::new();
    let _ = so_1.add_operator_product((pp_1.clone(), pp_1.clone()), CalculatorComplex::from(0.5));
    let mut so_0_1 = FermionLindbladNoiseOperator::new();
    let _ = so_0_1.add_operator_product((pp_0.clone(), pp_0), CalculatorComplex::from(1.0));
    let _ = so_0_1.add_operator_product((pp_1.clone(), pp_1), CalculatorComplex::from(0.5));

    assert_eq!(so_0 + so_1, so_0_1);
}

// Test the subtraction: FermionLindbladNoiseOperator - FermionLindbladNoiseOperator
#[test]
fn sub_so_so() {
    let pp_0: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let pp_1: FermionProduct = FermionProduct::new([0], [1]).unwrap();
    let mut so_0 = FermionLindbladNoiseOperator::new();
    let _ = so_0.add_operator_product((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(1.0));
    let mut so_1 = FermionLindbladNoiseOperator::new();
    let _ = so_1.add_operator_product((pp_1.clone(), pp_1.clone()), CalculatorComplex::from(0.5));
    let mut so_0_1 = FermionLindbladNoiseOperator::new();
    let _ = so_0_1.add_operator_product((pp_0.clone(), pp_0), CalculatorComplex::from(1.0));
    let _ = so_0_1.add_operator_product((pp_1.clone(), pp_1), CalculatorComplex::from(-0.5));

    assert_eq!(so_0 - so_1, so_0_1);
}

// Test the multiplication: FermionLindbladNoiseOperator * Calculatorcomplex
#[test]
fn mul_so_cc() {
    let pp_0: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut so_0 = FermionLindbladNoiseOperator::new();
    let _ = so_0.add_operator_product((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(2.0));
    let mut so_0_1 = FermionLindbladNoiseOperator::new();
    let _ = so_0_1.add_operator_product((pp_0.clone(), pp_0), CalculatorComplex::from(6.0));

    assert_eq!(so_0 * CalculatorComplex::from(3.0), so_0_1);
}

// Test the multiplication: FermionLindbladNoiseOperator * CalculatorFloat
#[test]
fn mul_so_cf() {
    let pp_0: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut so_0 = FermionLindbladNoiseOperator::new();
    let _ = so_0.add_operator_product((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(2.0));
    let mut so_0_1 = FermionLindbladNoiseOperator::new();
    let _ = so_0_1.add_operator_product((pp_0.clone(), pp_0), CalculatorComplex::from(6.0));

    assert_eq!(so_0 * CalculatorFloat::from(3.0), so_0_1);
}

// Test the add_noise_from_full_operators function of the FermionLindbladNoiseOperator
#[test]
fn internal_map_add_noise_from_full_operators() {
    let mut left = FermionOperator::new();
    left.add_operator_product(FermionProduct::new([0], [1]).unwrap(), 2.0.into())
        .unwrap();
    left.add_operator_product(FermionProduct::new([1], [2]).unwrap(), 3.0.into())
        .unwrap();

    let mut right = FermionOperator::new();
    right
        .add_operator_product(
            FermionProduct::new([1], [2]).unwrap(),
            CalculatorComplex::new(1.0, 1.0),
        )
        .unwrap();
    right
        .add_operator_product(
            FermionProduct::new([2], [3]).unwrap(),
            CalculatorComplex::new(3.0, 1.0),
        )
        .unwrap();

    let mut flno = FermionLindbladNoiseOperator::new();
    flno.add_noise_from_full_operators(&left, &right, 1.0.into())
        .unwrap();

    assert_eq!(
        flno.get(&(
            FermionProduct::new([1], [2]).unwrap(),
            FermionProduct::new([1], [2]).unwrap(),
        )),
        &CalculatorComplex::new(3.0, -3.0)
    );
    assert_eq!(
        flno.get(&(
            FermionProduct::new([0], [1]).unwrap(),
            FermionProduct::new([2], [3]).unwrap(),
        )),
        &CalculatorComplex::new(6.0, -2.0)
    );
    assert_eq!(
        flno.get(&(
            FermionProduct::new([0], [1]).unwrap(),
            FermionProduct::new([1], [2]).unwrap(),
        )),
        &CalculatorComplex::new(2.0, -2.0)
    );
    assert_eq!(
        flno.get(&(
            FermionProduct::new([1], [2]).unwrap(),
            FermionProduct::new([2], [3]).unwrap(),
        )),
        &CalculatorComplex::new(9.0, -3.0)
    );
}

// Test the Iter traits of FermionLindbladNoiseOperator: into_iter, from_iter and extend
#[test]
fn into_iter_from_iter_extend() {
    let pp_0: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let pp_1: FermionProduct = FermionProduct::new([0], [1]).unwrap();
    let mut so_0 = FermionLindbladNoiseOperator::new();
    let _ = so_0.add_operator_product((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(1.0));

    let so_iter = so_0.clone().into_iter();
    assert_eq!(FermionLindbladNoiseOperator::from_iter(so_iter), so_0);
    let system_iter = (&so_0)
        .into_iter()
        .map(|(key, value)| (key.clone(), value.clone()));
    assert_eq!(FermionLindbladNoiseOperator::from_iter(system_iter), so_0);
    let mut mapping: BTreeMap<(FermionProduct, FermionProduct), CalculatorComplex> =
        BTreeMap::new();
    mapping.insert((pp_1.clone(), pp_1.clone()), CalculatorComplex::from(0.5));
    let mapping_iter = mapping.into_iter();
    so_0.extend(mapping_iter);

    let mut so_0_1 = FermionLindbladNoiseOperator::new();
    let _ = so_0_1.add_operator_product((pp_0.clone(), pp_0), CalculatorComplex::from(1.0));
    let _ = so_0_1.add_operator_product((pp_1.clone(), pp_1), CalculatorComplex::from(0.5));

    assert_eq!(so_0, so_0_1);
}

// Test the failure of creating the FermionLindbladNoiseOperator with identity terms
#[test]
fn illegal_identity_operators() {
    let empty_fp = FermionProduct::new([], []).unwrap();
    let fp = FermionProduct::new([0], [1]).unwrap();
    let mut fno_left_identity = FermionLindbladNoiseOperator::new();
    let cc = CalculatorComplex::new(1.0, 1.0);
    let ok = fno_left_identity
        .add_operator_product((empty_fp.clone(), fp.clone()), cc.clone())
        .is_err();
    assert!(ok);
    let mut fno_right_identity = FermionLindbladNoiseOperator::new();
    let ok = fno_right_identity
        .add_operator_product((fp, empty_fp.clone()), cc.clone())
        .is_err();
    assert!(ok);
    let mut fno_both_identity = FermionLindbladNoiseOperator::new();
    let ok = fno_both_identity
        .add_operator_product((empty_fp.clone(), empty_fp), cc)
        .is_err();
    assert!(ok);
}

// Test the Debug trait of FermionLindbladNoiseOperator
#[test]
fn debug() {
    let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let mut so = FermionLindbladNoiseOperator::new();
    let _ = so.set((pp.clone(), pp), CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{:?}", so),
        "FermionLindbladNoiseOperator { internal_map: {(FermionProduct { creators: [0], annihilators: [0] }, FermionProduct { creators: [0], annihilators: [0] }): CalculatorComplex { re: Float(0.5), im: Float(0.0) }} }"
    );
}

// Test the Display trait of FermionLindbladNoiseOperator
#[test]
fn display() {
    let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let mut so = FermionLindbladNoiseOperator::new();
    let _ = so.set((pp.clone(), pp), CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{}", so),
        "FermionLindbladNoiseOperator{\n(c0a0, c0a0): (5e-1 + i * 0e0),\n}"
    );
}

// Test the Clone and PartialEq traits of FermionLindbladNoiseOperator
#[test]
fn clone_partial_eq() {
    let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let mut so = FermionLindbladNoiseOperator::new();
    so.set((pp.clone(), pp), CalculatorComplex::from(0.5))
        .unwrap();

    // Test Clone trait
    assert_eq!(so.clone(), so);

    // Test PartialEq trait
    let pp_1: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let mut so_1 = FermionLindbladNoiseOperator::new();
    so_1.set((pp_1.clone(), pp_1), CalculatorComplex::from(0.5))
        .unwrap();
    let pp_2: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut so_2 = FermionLindbladNoiseOperator::new();
    so_2.set((pp_2.clone(), pp_2), CalculatorComplex::from(0.5))
        .unwrap();
    assert!(so_1 == so);
    assert!(so == so_1);
    assert!(so_2 != so);
    assert!(so != so_2);
}

/// Test QubitOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_json() {
    let pp = FermionProduct::new([0], [1]).unwrap();
    let mut so = FermionLindbladNoiseOperator::new();
    so.set((pp.clone(), pp), CalculatorComplex::from(1.0))
        .unwrap();

    let serialized = serde_json::to_string(&so).unwrap();
    let deserialized: FermionLindbladNoiseOperator = serde_json::from_str(&serialized).unwrap();

    assert_eq!(so, deserialized);
}

/// Test QubitOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_readable() {
    let pp = FermionProduct::new([0], [1]).unwrap();
    let mut so = FermionLindbladNoiseOperator::new();
    so.set((pp.clone(), pp), CalculatorComplex::from(1.0))
        .unwrap();

    assert_tokens(
        &so.readable(),
        &[
            Token::Struct {
                name: "FermionLindbladNoiseOperatorSerialize",
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
            Token::Str("serialisation_meta"),
            Token::Struct {
                name: "StruqtureSerialisationMeta",
                len: 3,
            },
            Token::Str("type_name"),
            Token::Str("FermionLindbladNoiseOperator"),
            Token::Str("min_version"),
            Token::Tuple { len: 3 },
            Token::U64(2),
            Token::U64(0),
            Token::U64(0),
            Token::TupleEnd,
            Token::Str("version"),
            Token::Str("2.0.0-alpha.3"),
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

#[test]
fn bincode() {
    let pp = FermionProduct::new([0], [1]).unwrap();
    let mut so = FermionLindbladNoiseOperator::new();
    so.set((pp.clone(), pp), CalculatorComplex::from(1.0))
        .unwrap();

    let serialized = serialize(&so).unwrap();
    let deserialized: FermionLindbladNoiseOperator = deserialize(&serialized).unwrap();
    assert_eq!(deserialized, so);

    let encoded: Vec<u8> = bincode::serialize(&so.clone().compact()).unwrap();
    let decoded: FermionLindbladNoiseOperator = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(so, decoded);
}

#[test]
fn serde_compact() {
    let pp = FermionProduct::new([0], [1]).unwrap();
    let mut so = FermionLindbladNoiseOperator::new();
    so.set((pp.clone(), pp), CalculatorComplex::from(1.0))
        .unwrap();

    assert_tokens(
        &so.compact(),
        &[
            Token::Struct {
                name: "FermionLindbladNoiseOperatorSerialize",
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
            Token::Str("serialisation_meta"),
            Token::Struct {
                name: "StruqtureSerialisationMeta",
                len: 3,
            },
            Token::Str("type_name"),
            Token::Str("FermionLindbladNoiseOperator"),
            Token::Str("min_version"),
            Token::Tuple { len: 3 },
            Token::U64(2),
            Token::U64(0),
            Token::U64(0),
            Token::TupleEnd,
            Token::Str("version"),
            Token::Str("2.0.0-alpha.3"),
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

#[cfg(feature = "json_schema")]
#[test]
fn test_fermion_noise_operator_schema() {
    let mut op = FermionLindbladNoiseOperator::new();
    op.set(
        (
            FermionProduct::new([0], [1]).unwrap(),
            FermionProduct::new([0], [0]).unwrap(),
        ),
        1.0.into(),
    )
    .unwrap();
    op.set(
        (
            FermionProduct::new([1], [1]).unwrap(),
            FermionProduct::new([1], [1]).unwrap(),
        ),
        "val".into(),
    )
    .unwrap();
    let schema = schemars::schema_for!(FermionLindbladNoiseOperator);
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
    let pp_1 = struqture_1::fermions::FermionProduct::from_str("c0a1").unwrap();
    let mut ss_1 = struqture_1::fermions::FermionLindbladNoiseSystem::new(None);
    struqture_1::OperateOnDensityMatrix::set(&mut ss_1, (pp_1.clone(), pp_1.clone()), 1.0.into())
        .unwrap();

    let pp_2 = FermionProduct::new([0], [1]).unwrap();
    let mut ss_2 = FermionLindbladNoiseOperator::new();
    ss_2.set((pp_2.clone(), pp_2.clone()), 1.0.into()).unwrap();

    assert!(FermionLindbladNoiseOperator::from_struqture_1(&ss_1).unwrap() == ss_2);
    assert!(ss_1 == ss_2.to_struqture_1().unwrap());
}
