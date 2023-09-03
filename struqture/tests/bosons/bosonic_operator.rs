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

//! Integration test for public API of BosonOperator

use bincode::{deserialize, serialize};
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde_test::{assert_tokens, Configure, Token};
use std::collections::BTreeMap;
use std::iter::{FromIterator, IntoIterator};
use struqture::bosons::{BosonHamiltonian, BosonOperator, BosonProduct, HermitianBosonProduct};
use struqture::{ModeIndex, OperateOnDensityMatrix, OperateOnModes, OperateOnState};
use test_case::test_case;

// Test the new function of the BosonOperator
#[test]
fn new() {
    let so = BosonOperator::new();
    assert!(so.is_empty());
    assert_eq!(BosonOperator::new(), BosonOperator::default());
    assert_eq!(BosonOperator::with_capacity(2), BosonOperator::new());
}

#[test]
fn empty_clone_options() {
    let pp_2: BosonProduct = BosonProduct::new([0], [2]).unwrap();
    let mut system = BosonOperator::new();
    system.set(pp_2, CalculatorComplex::from(0.5)).unwrap();

    let empty: Option<usize> = None;
    let full: Option<usize> = Some(3);
    assert_eq!(system.empty_clone(empty), BosonOperator::new());
    assert_eq!(system.empty_clone(full), BosonOperator::with_capacity(1));
}

// Test the current_number_modes function of the BosonOperator
#[test]
fn internal_map_current_number_modes() {
    let pp_0: BosonProduct = BosonProduct::new([0], [1]).unwrap();
    let pp_2: BosonProduct = BosonProduct::new([2], [3]).unwrap();
    let mut so = BosonOperator::new();
    assert_eq!(so.current_number_modes(), 0_usize);
    so.set(pp_0, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.current_number_modes(), 2_usize);
    so.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.current_number_modes(), 4_usize);
}

// Test the len function of the BosonOperator
#[test]
fn internal_map_len() {
    let pp_2: BosonProduct = BosonProduct::new([0], [2]).unwrap();
    let mut so = BosonOperator::new();
    so.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.len(), 1_usize);
}
// Test the set, set_pauli_product, get functions of the SpinSystem
#[test]
fn internal_map_set_get_dict() {
    let mut system = BosonOperator::new();
    assert_eq!(system.current_number_modes(), 0_usize);
    assert_eq!(system.number_modes(), 0_usize);
    let pp_0: BosonProduct = BosonProduct::new([0], [0]).unwrap();

    // 1) Test try_set_pauli_product and get functions
    // Vacant
    system
        .set(pp_0.clone(), CalculatorComplex::from(0.0))
        .unwrap();
    system
        .set(pp_0.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(system.current_number_modes(), 1_usize);
    assert_eq!(system.number_modes(), 1_usize);
    assert_eq!(system.get(&pp_0), &CalculatorComplex::from(0.5));

    // 2) Test iter, keys, values functions
    let mut map: BTreeMap<BosonProduct, CalculatorComplex> = BTreeMap::new();
    map.insert(pp_0, CalculatorComplex::from(0.5));
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

// Test the set, get and remove functions of the BosonOperator
#[test]
fn internal_map_set_get_remove() {
    let pp_2: BosonProduct = BosonProduct::new([0], [2]).unwrap();
    let mut so = BosonOperator::new();

    // 1) Test try_set_boson_product and get functions
    // Vacant
    so.set(pp_2.clone(), CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.get(&pp_2.clone()), &CalculatorComplex::from(0.5));

    // 2) Test remove function
    so.remove(&pp_2);
    assert_eq!(so, BosonOperator::new());
}

// Test the add_operator_product function of the BosonOperator
#[test]
fn internal_map_add_operator_product() {
    let pp_2: BosonProduct = BosonProduct::new([0], [2]).unwrap();
    let mut so = BosonOperator::new();

    so.add_operator_product(pp_2.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(so.get(&pp_2), &CalculatorComplex::from(0.5));
    so.add_operator_product(pp_2.clone(), CalculatorComplex::from(-0.5))
        .unwrap();
    assert_eq!(so.get(&pp_2), &CalculatorComplex::from(0.0));
}

// Test the iter, keys and values functions of the BosonOperator
#[test]
fn internal_map_keys() {
    let pp_2: BosonProduct = BosonProduct::new([0], [2]).unwrap();
    let mut so = BosonOperator::new();
    so.set(pp_2.clone(), CalculatorComplex::from(0.5)).unwrap();

    let mut map: BTreeMap<BosonProduct, CalculatorComplex> = BTreeMap::new();
    map.insert(pp_2, CalculatorComplex::from(0.5));

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

// Test the hermitian_conjugate and is_natural_hermitian functions of the HermitianMixedProduct
#[test]
fn hermitian_test() {
    let pp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut system = BosonOperator::new();
    let _ = system.add_operator_product(pp, CalculatorComplex::from(1.0));

    assert_eq!(system.hermitian_conjugate(), system.clone());
}

// Test the negative operation: -BosonOperator
#[test]
fn negative_so() {
    let pp_0: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut so_0 = BosonOperator::new();
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0));
    let mut so_0_minus = BosonOperator::new();
    let _ = so_0_minus.add_operator_product(pp_0, CalculatorComplex::from(-1.0));

    assert_eq!(-so_0, so_0_minus);
}

// Test the addition: BosonOperator + BosonOperator
#[test]
fn add_so_so() {
    let pp_0: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let pp_1: BosonProduct = BosonProduct::new([0], [1]).unwrap();
    let mut so_0 = BosonOperator::new();
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0));
    let mut so_1 = BosonOperator::new();
    let _ = so_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(0.5));
    let mut so_0_1 = BosonOperator::new();
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(1.0));
    let _ = so_0_1.add_operator_product(pp_1, CalculatorComplex::from(0.5));

    assert_eq!(so_0 + so_1, so_0_1);
}

// Test the subtraction: BosonOperator - BosonOperator
#[test]
fn sub_so_so() {
    let pp_0: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let pp_1: BosonProduct = BosonProduct::new([0], [1]).unwrap();
    let mut so_0 = BosonOperator::new();
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0));
    let mut so_1 = BosonOperator::new();
    let _ = so_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(0.5));
    let mut so_0_1 = BosonOperator::new();
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(1.0));
    let _ = so_0_1.add_operator_product(pp_1, CalculatorComplex::from(-0.5));

    assert_eq!(so_0 - so_1, so_0_1);
}

// Test the multiplication: BosonOperator * BosonOperator
#[test]
fn mul_so_so() {
    let pp_0: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let pp_1: BosonProduct = BosonProduct::new([1], [1]).unwrap();
    let pp_0_1: BosonProduct = BosonProduct::new([0, 1], [0, 1]).unwrap();
    let mut so_0 = BosonOperator::new();
    let _ = so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0));
    let mut so_1 = BosonOperator::new();
    let _ = so_1.add_operator_product(pp_1, CalculatorComplex::from(0.5));
    let mut so_0_1 = BosonOperator::new();
    let _ = so_0_1.add_operator_product(pp_0_1, CalculatorComplex::from(1.0));
    assert_eq!((so_0 * so_1), so_0_1);
}

// Test the multiplication: BosonOperator * BosonOperator where they have a BosonProduct with the same index
#[test]
fn mul_so_so_same_index() {
    let pp_0: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let pp_1: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let pp_0_1: BosonProduct = BosonProduct::new([0, 0], [0, 0]).unwrap();
    let pp_0_1_comm: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut so_0 = BosonOperator::new();
    let _ = so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0));
    let mut so_1 = BosonOperator::new();
    let _ = so_1.add_operator_product(pp_1, CalculatorComplex::from(0.5));
    let mut so_0_1 = BosonOperator::new();
    let _ = so_0_1.add_operator_product(pp_0_1, CalculatorComplex::new(1.0, 0.0));
    let _ = so_0_1.add_operator_product(pp_0_1_comm, CalculatorComplex::new(1.0, 0.0));

    assert_eq!(so_0 * so_1, so_0_1);
}

// Test the multiplication: BosonOperator * Calculatorcomplex
#[test]
fn mul_so_cc() {
    let pp_0: BosonProduct = BosonProduct::new([0], [2]).unwrap();
    let mut so_0 = BosonOperator::new();
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(2.0));
    let mut so_0_1 = BosonOperator::new();
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(6.0));

    assert_eq!(so_0 * CalculatorComplex::from(3.0), so_0_1);
}

// Test the multiplication: BosonOperator * CalculatorFloat
#[test]
fn mul_so_cf() {
    let pp_0: BosonProduct = BosonProduct::new([0], [2]).unwrap();
    let mut so_0 = BosonOperator::new();
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(2.0));
    let mut so_0_1 = BosonOperator::new();
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(6.0));

    assert_eq!(so_0 * CalculatorFloat::from(3.0), so_0_1);
}

#[test_case(&[], &[]; "empty")]
#[test_case(&[0], &[1]; "0 - 1")]
#[test_case(&[], &[2000]; "empty - 2000")]
#[test_case(&[0,1,1], &[3,3,5]; "0,1,1 - 3,3,5")]
fn from_operator_pass(creators: &[usize], annihilators: &[usize]) {
    let pp_0: HermitianBosonProduct =
        HermitianBosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let pp_1: BosonProduct = BosonProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let pp_1_conj: BosonProduct =
        BosonProduct::new(annihilators.to_vec(), creators.to_vec()).unwrap();
    let mut so_0_1 = BosonOperator::new();
    let _ = so_0_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(2.0));
    if pp_1 != pp_1_conj {
        let _ = so_0_1.add_operator_product(pp_1_conj, CalculatorComplex::from(2.0));
    }
    let mut so_0 = BosonHamiltonian::new();
    let _ = so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0));

    assert_eq!(BosonOperator::from(so_0), so_0_1);
}

// Test the separation of terms
#[test_case((1, 1))]
#[test_case((1, 2))]
#[test_case((2, 1))]
#[test_case((2, 2))]
fn separate_out_terms(number_spins: (usize, usize)) {
    let pp_1_a: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let pp_1_b: BosonProduct = BosonProduct::new([1], [1]).unwrap();
    let pp_2_a: BosonProduct = BosonProduct::new([0, 1], [1]).unwrap();
    let pp_2_b: BosonProduct = BosonProduct::new([0], [0, 1]).unwrap();
    let pp_3_a: BosonProduct = BosonProduct::new([0, 1], [0, 1]).unwrap();
    let pp_3_b: BosonProduct = BosonProduct::new([0, 2], [0, 2]).unwrap();

    let mut allowed: Vec<(BosonProduct, f64)> = Vec::new();
    let mut not_allowed: Vec<(BosonProduct, f64)> = vec![
        (pp_1_a.clone(), 1.0),
        (pp_1_b.clone(), 1.1),
        (pp_2_a.clone(), 1.2),
        (pp_2_b.clone(), 1.3),
        (pp_3_a.clone(), 1.4),
        (pp_3_b.clone(), 1.5),
    ];

    match number_spins {
        (1, 1) => {
            allowed.push((pp_1_a.clone(), 1.0));
            allowed.push((pp_1_b.clone(), 1.1));
            not_allowed.remove(0);
            not_allowed.remove(0);
        }
        (2, 1) => {
            allowed.push((pp_2_a.clone(), 1.2));
            not_allowed.remove(2);
        }
        (1, 2) => {
            allowed.push((pp_2_b.clone(), 1.3));
            not_allowed.remove(3);
        }
        (2, 2) => {
            allowed.push((pp_3_a.clone(), 1.4));
            allowed.push((pp_3_b.clone(), 1.5));
            not_allowed.remove(4);
            not_allowed.remove(4);
        }
        _ => panic!(),
    }

    let mut separated = BosonOperator::new();
    for (key, value) in allowed.iter() {
        separated
            .add_operator_product(key.clone(), value.into())
            .unwrap();
    }
    let mut remainder = BosonOperator::new();
    for (key, value) in not_allowed.iter() {
        remainder
            .add_operator_product(key.clone(), value.into())
            .unwrap();
    }

    let mut so = BosonOperator::new();
    so.add_operator_product(pp_1_a, CalculatorComplex::from(1.0))
        .unwrap();
    so.add_operator_product(pp_1_b, CalculatorComplex::from(1.1))
        .unwrap();
    so.add_operator_product(pp_2_a, CalculatorComplex::from(1.2))
        .unwrap();
    so.add_operator_product(pp_2_b, CalculatorComplex::from(1.3))
        .unwrap();
    so.add_operator_product(pp_3_a, CalculatorComplex::from(1.4))
        .unwrap();
    so.add_operator_product(pp_3_b, CalculatorComplex::from(1.5))
        .unwrap();

    let result = so.separate_into_n_terms(number_spins).unwrap();
    assert_eq!(result.0, separated);
    assert_eq!(result.1, remainder);
}

// Test the Iter traits of BosonOperator: into_iter, from_iter and extend
#[test]
fn into_iter_from_iter_extend() {
    let pp_0: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let pp_1: BosonProduct = BosonProduct::new([0], [1]).unwrap();
    let mut so_0 = BosonOperator::new();
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0));

    let so_iter = so_0.clone().into_iter();
    assert_eq!(BosonOperator::from_iter(so_iter), so_0);
    let so_iter = (&so_0)
        .into_iter()
        .map(|(key, value)| (key.clone(), value.clone()));
    assert_eq!(BosonOperator::from_iter(so_iter), so_0);
    let mut mapping: BTreeMap<BosonProduct, CalculatorComplex> = BTreeMap::new();
    mapping.insert(pp_1.clone(), CalculatorComplex::from(0.5));
    let mapping_iter = mapping.into_iter();
    so_0.extend(mapping_iter);

    let mut so_0_1 = BosonOperator::new();
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(1.0));
    let _ = so_0_1.add_operator_product(pp_1, CalculatorComplex::from(0.5));

    assert_eq!(so_0, so_0_1);
}

// Test the Debug trait of BosonOperator
#[test]
fn debug() {
    let pp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut so = BosonOperator::new();
    let _ = so.set(pp, CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{:?}", so),
        "BosonOperator { internal_map: {BosonProduct { creators: [0], annihilators: [0] }: CalculatorComplex { re: Float(0.5), im: Float(0.0) }} }"
    );
}

// Test the Display trait of BosonOperator
#[test]
fn display() {
    let pp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut so = BosonOperator::new();
    let _ = so.set(pp, CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{}", so),
        "BosonOperator{\nc0a0: (5e-1 + i * 0e0),\n}"
    );
}

// Test the Clone and PartialEq traits of BosonOperator
#[test]
fn clone_partial_eq() {
    let pp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut so = BosonOperator::new();
    so.set(pp, CalculatorComplex::from(0.5)).unwrap();

    // Test Clone trait
    assert_eq!(so.clone(), so);

    // Test PartialEq trait
    let pp_1: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut so_1 = BosonOperator::new();
    so_1.set(pp_1, CalculatorComplex::from(0.5)).unwrap();
    let pp_2: BosonProduct = BosonProduct::new([0], [2]).unwrap();
    let mut so_2 = BosonOperator::new();
    so_2.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert!(so_1 == so);
    assert!(so == so_1);
    assert!(so_2 != so);
    assert!(so != so_2);
}

/// Test SpinOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_json() {
    let pp = BosonProduct::new([0], [2]).unwrap();
    let mut so = BosonOperator::new();
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let serialized = serde_json::to_string(&so).unwrap();
    let deserialized: BosonOperator = serde_json::from_str(&serialized).unwrap();

    assert_eq!(so, deserialized);
}

/// Test SpinOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_readable() {
    use struqture::MINIMUM_STRUQTURE_VERSION;
    let major_version = MINIMUM_STRUQTURE_VERSION.0;
    let minor_version = MINIMUM_STRUQTURE_VERSION.1;

    let pp = BosonProduct::new([0], [0]).unwrap();
    let mut so = BosonOperator::new();
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();
    assert_tokens(
        &so.readable(),
        &[
            Token::Struct {
                name: "BosonOperatorSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 3 },
            Token::Str("c0a0"),
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
        ],
    );
}

#[test]
fn bincode() {
    let pp = BosonProduct::new([0], [1]).unwrap();
    let mut so = BosonOperator::new();
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let serialized = serialize(&so).unwrap();
    let deserialized: BosonOperator = deserialize(&serialized).unwrap();
    assert_eq!(deserialized, so);

    let encoded: Vec<u8> = bincode::serialize(&so.clone().compact()).unwrap();
    let decoded: BosonOperator = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(so, decoded);
}

#[test]
fn serde_compact() {
    use struqture::MINIMUM_STRUQTURE_VERSION;
    let major_version = MINIMUM_STRUQTURE_VERSION.0;
    let minor_version = MINIMUM_STRUQTURE_VERSION.1;

    let pp = BosonProduct::new([0], [0]).unwrap();
    let mut so = BosonOperator::new();
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    assert_tokens(
        &so.compact(),
        &[
            Token::Struct {
                name: "BosonOperatorSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 3 },
            Token::Tuple { len: 2 },
            Token::Seq { len: Some(1) },
            Token::U64(0),
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::U64(0),
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
        ],
    );
}

#[cfg(feature = "json_schema")]
#[test]
fn test_boson_operator_schema() {
    let mut op = BosonOperator::new();
    op.set(BosonProduct::new([0], [1]).unwrap(), 1.0.into())
        .unwrap();
    op.set(BosonProduct::new([1], [1]).unwrap(), "val".into())
        .unwrap();
    let schema = schemars::schema_for!(BosonOperator);
    let schema_checker = jsonschema::JSONSchema::compile(&serde_json::to_value(&schema).unwrap())
        .expect("schema is valid");
    let value = serde_json::to_value(&op).unwrap();
    let validation = schema_checker.validate(&value);

    assert!(validation.is_ok());
}
