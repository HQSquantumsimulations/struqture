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

//! Integration test for public API of DecoherenceOperator

use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
// use num_complex::Complex64;
use serde_test::{assert_tokens, Configure, Token};
use std::collections::BTreeMap;
use std::iter::{FromIterator, IntoIterator};
use std::ops::{Add, Sub};
use std::str::FromStr;
use struqture::prelude::*;
use struqture::spins::{DecoherenceOperator, DecoherenceProduct, PauliProduct, QubitOperator};
use struqture::SpinIndex;
use test_case::test_case;

// Test the new function of the DecoherenceOperator
#[test]
fn new() {
    let so = DecoherenceOperator::new();
    assert!(so.is_empty());
    assert_eq!(DecoherenceOperator::new(), DecoherenceOperator::default())
}

#[test]
fn empty_clone_options() {
    let pp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut system = DecoherenceOperator::new();
    system.set(pp_2, CalculatorComplex::from(0.5)).unwrap();

    let empty: Option<usize> = None;
    let full: Option<usize> = Some(3);
    assert_eq!(system.empty_clone(empty), DecoherenceOperator::new());
    assert_eq!(
        system.empty_clone(full),
        DecoherenceOperator::with_capacity(1)
    );
}

// Test the current_number_spins function of the DecoherenceOperator
#[test]
fn internal_map_number_spins() {
    let pp_0: DecoherenceProduct = DecoherenceProduct::new().x(0);
    let pp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut so = DecoherenceOperator::new();
    assert_eq!(so.current_number_spins(), 0_usize);
    so.set(pp_0, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.current_number_spins(), 1_usize);
    so.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.current_number_spins(), 3_usize);
}

// Test the len function of the DecoherenceOperator
#[test]
fn internal_map_len() {
    let pp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut so = DecoherenceOperator::new();
    so.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.len(), 1_usize);
}

// Test the set, get and keys/values/iter functions of the SpinSystem
#[test]
fn internal_map_set_get_dict() {
    let mut system = DecoherenceOperator::new();
    assert_eq!(system.current_number_spins(), 0_usize);
    let pp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);

    // 1) Test set and get functions
    // Vacant
    system
        .set(pp_0.clone(), CalculatorComplex::from(0.0))
        .unwrap();
    system
        .set(pp_0.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(system.current_number_spins(), 1_usize);
    assert_eq!(system.get(&pp_0), &CalculatorComplex::from(0.5));

    // 2) Test iter, keys, values functions
    let mut map: BTreeMap<DecoherenceProduct, CalculatorComplex> = BTreeMap::new();
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

// Test the set, get and remove functions of the DecoherenceOperator
#[test]
fn internal_map_set_get_remove() {
    let pp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut so = DecoherenceOperator::new();

    // 1) Test try_set_pauli_product and get functions
    // Vacant
    so.set(pp_2.clone(), CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.get(&pp_2.clone()), &CalculatorComplex::from(0.5));

    // 2) Test remove function
    so.remove(&pp_2);
    assert_eq!(so, DecoherenceOperator::new());
}

// Test the add_operator_product function of the DecoherenceOperator
#[test]
fn internal_map_add_operator_product() {
    let pp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut so = DecoherenceOperator::new();

    so.add_operator_product(pp_2.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(so.get(&pp_2), &CalculatorComplex::from(0.5));
    so.add_operator_product(pp_2.clone(), CalculatorComplex::from(-0.5))
        .unwrap();
    assert_eq!(so.get(&pp_2), &CalculatorComplex::from(0.0));
}

// Test the iter, keys and values functions of the DecoherenceOperator
#[test]
fn internal_map_keys() {
    let pp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut so = DecoherenceOperator::new();
    let _ = so.set(pp_2.clone(), CalculatorComplex::from(0.5)).unwrap();

    let mut map: BTreeMap<DecoherenceProduct, CalculatorComplex> = BTreeMap::new();
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

// Test the Iter traits of DecoherenceOperator: into_iter, from_iter and extend
#[test]
fn into_iter_from_iter_extend() {
    let pp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let pp_1: DecoherenceProduct = DecoherenceProduct::new().x(1);
    let mut system = DecoherenceOperator::new();
    system
        .add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();

    let system_iter = system.clone().into_iter();
    assert_eq!(DecoherenceOperator::from_iter(system_iter), system);
    let system_iter = (&system)
        .into_iter()
        .map(|(key, value)| (key.clone(), value.clone()));
    assert_eq!(DecoherenceOperator::from_iter(system_iter), system);

    let mut hamiltonian = DecoherenceOperator::new();
    hamiltonian
        .add_operator_product(pp_0.clone(), 1.0.into())
        .unwrap();
    for (first, second) in system.into_iter().zip(hamiltonian.iter()) {
        assert_eq!(first.0, *second.0);
        assert_eq!(first.1, *second.1);
    }

    let mut system = DecoherenceOperator::new();
    system
        .add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut mapping: BTreeMap<DecoherenceProduct, CalculatorComplex> = BTreeMap::new();
    mapping.insert(pp_1.clone(), CalculatorComplex::from(0.5));
    let mapping_iter = mapping.into_iter();
    system.extend(mapping_iter);

    let mut system_1 = DecoherenceOperator::new();
    system_1
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    system_1
        .add_operator_product(pp_1, CalculatorComplex::from(0.5))
        .unwrap();

    assert_eq!(system, system_1);
}

// Test the negative operation: -DecoherenceOperator
#[test]
fn negative_so() {
    let pp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut so_0 = DecoherenceOperator::new();
    so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut so_0_minus = DecoherenceOperator::new();
    so_0_minus
        .add_operator_product(pp_0, CalculatorComplex::from(-1.0))
        .unwrap();

    assert_eq!(-so_0, so_0_minus);
}

// Test the addition: DecoherenceOperator + DecoherenceOperator
#[test]
fn add_so_so() {
    let pp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let pp_1: DecoherenceProduct = DecoherenceProduct::new().x(1);
    let mut so_0 = DecoherenceOperator::new();
    so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut so_1 = DecoherenceOperator::new();
    so_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    let mut so_0_1 = DecoherenceOperator::new();
    so_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    so_0_1
        .add_operator_product(pp_1, CalculatorComplex::from(0.5))
        .unwrap();

    assert_eq!(so_0.clone() + so_1.clone(), so_0_1);
    assert_eq!(so_0.add(so_1), so_0_1);
}

// Test the subtraction: DecoherenceOperator - DecoherenceOperator
#[test]
fn sub_so_so() {
    let pp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let pp_1: DecoherenceProduct = DecoherenceProduct::new().x(1);
    let mut so_0 = DecoherenceOperator::new();
    so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut so_1 = DecoherenceOperator::new();
    so_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    let mut so_0_1 = DecoherenceOperator::new();
    so_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    so_0_1
        .add_operator_product(pp_1, CalculatorComplex::from(-0.5))
        .unwrap();

    assert_eq!(so_0.clone() - so_1.clone(), so_0_1);
    assert_eq!(so_0.sub(so_1), so_0_1);
}

// Test the multiplication: DecoherenceOperator * DecoherenceOperator with all possible pauli matrices
#[test_case("0X", "0X", 
            ("0I", CalculatorComplex::from(1.0)); "plus_plus_identity")]
#[test_case("0X1X", "0X",
            ("0I1X", CalculatorComplex::new(1.0, 0.0)); "plus_plus")]
#[test_case("0X1X", "0iY",
            ("0Z1X", CalculatorComplex::new(-1.0, 0.0)); "plus_minus")]
#[test_case("0X1X", "0Z",
            ("0iY1X", CalculatorComplex::new(-1.0, 0.0)); "plus_z")]
#[test_case("0iY1X", "0X",
            ("0Z1X", CalculatorComplex::new(1.0, 0.0))
                ; "minus_plus")]
#[test_case("0iY1X", "0iY", 
            ("0I1X", CalculatorComplex::new(-1.0, 0.0)); "minus_minus")]
#[test_case("0iY1X", "0Z",
            ("0X1X", CalculatorComplex::new(-1.0, 0.0)); "minus_z")]
#[test_case("0Z1X", "0X",
            ("0iY1X", CalculatorComplex::new(1.0, 0.0)); "z_plus")]
#[test_case("0Z1X", "0iY",
            ("0X1X", CalculatorComplex::new(1.0, 0.0)); "z_minus")]
#[test_case("0Z1X", "0Z",
            ("0I1X", CalculatorComplex::new(1.0, 0.0)); "z_z")]
#[test_case("0Z", "0Z",
            ("0I", CalculatorComplex::new(1.0, 0.0)); "z_z_identity")]
fn mul_so_so_all_paulis(pp0: &str, pp1: &str, pp01: (&str, CalculatorComplex)) {
    let pp_0: DecoherenceProduct = DecoherenceProduct::from_str(pp0).unwrap();
    let mut so_0 = DecoherenceOperator::new();
    so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0))
        .unwrap();
    let pp_1: DecoherenceProduct = DecoherenceProduct::from_str(pp1).unwrap();
    let mut so_1 = DecoherenceOperator::new();
    so_1.add_operator_product(pp_1, CalculatorComplex::from(0.5))
        .unwrap();
    let mut so_0_1 = DecoherenceOperator::new();
    let (pp, coeff) = pp01;
    let pp_0_1: DecoherenceProduct = DecoherenceProduct::from_str(pp).unwrap();
    so_0_1.add_operator_product(pp_0_1, coeff).unwrap();

    assert_eq!(so_0 * so_1, so_0_1);
}

// Test the multiplication: DecoherenceOperator * DecoherenceOperator
#[test]
fn mul_so_so() {
    let pp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let pp_1: DecoherenceProduct = DecoherenceProduct::new().x(1);
    let pp_0_1: DecoherenceProduct = DecoherenceProduct::new().z(0).x(1);
    let mut so_0 = DecoherenceOperator::new();
    so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0))
        .unwrap();
    let mut so_1 = DecoherenceOperator::new();
    so_1.add_operator_product(pp_1, CalculatorComplex::from(0.5))
        .unwrap();
    let mut so_0_1 = DecoherenceOperator::new();
    so_0_1
        .add_operator_product(pp_0_1, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(so_0 * so_1, so_0_1);
}

#[test]
fn test_conjugate_rotz_0_x1_x() {
    let mut rot_z = DecoherenceOperator::new();
    rot_z
        .add_operator_product(
            DecoherenceProduct::new().z(0),
            CalculatorComplex::new(0.0, 0.2),
        )
        .unwrap();
    rot_z
        .add_operator_product(DecoherenceProduct::new(), CalculatorComplex::from(0.98))
        .unwrap();
    let mut inner = DecoherenceOperator::new();
    inner
        .add_operator_product(
            DecoherenceProduct::new().x(0).x(1),
            CalculatorComplex::from(1.0),
        )
        .unwrap();
    let _ = rot_z.clone() * inner * rot_z.hermitian_conjugate();
}

// Test the multiplication: DecoherenceOperator * DecoherenceOperator where they have a DecoherenceProduct with the same index
#[test]
fn mul_so_so_same_index() {
    let pp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let pp_1: DecoherenceProduct = DecoherenceProduct::new().x(0);
    let pp_0_1: DecoherenceProduct = DecoherenceProduct::new().iy(0);
    let mut so_0 = DecoherenceOperator::new();
    so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0))
        .unwrap();
    let mut so_1 = DecoherenceOperator::new();
    so_1.add_operator_product(pp_1, CalculatorComplex::from(0.5))
        .unwrap();
    let mut so_0_1 = DecoherenceOperator::new();
    so_0_1
        .add_operator_product(pp_0_1, CalculatorComplex::new(1.0, 0.0))
        .unwrap();

    assert_eq!(so_0 * so_1, so_0_1);
}

// Test the multiplication: DecoherenceOperator * Calculatorcomplex
#[test]
fn mul_so_cc() {
    let pp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut so_0 = DecoherenceOperator::new();
    so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(2.0))
        .unwrap();
    let mut so_0_1 = DecoherenceOperator::new();
    so_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(6.0))
        .unwrap();

    assert_eq!(so_0 * CalculatorComplex::from(3.0), so_0_1);
}

// Test the multiplication: DecoherenceOperator * Calculatorcomplex
#[test]
fn mul_so_cf() {
    let pp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut so_0 = DecoherenceOperator::new();
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(2.0));
    let mut so_0_1 = DecoherenceOperator::new();
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(6.0));

    assert_eq!(so_0 * CalculatorFloat::from(3.0), so_0_1);
}

// Test the From<QubitOperator> trait
#[test]
fn test_from_qubit_operator() {
    let mut so = QubitOperator::new();
    let pp_0 = PauliProduct::new().x(0).y(1).z(2);
    let c0 = CalculatorComplex::new(1.0, 2.0);
    let pp_1 = PauliProduct::new().x(0).y(1).y(2);
    let c1 = CalculatorComplex::new(2.0, 1.0);
    let pp_2 = PauliProduct::new().y(0).y(1).y(2);
    let c2 = CalculatorComplex::new(2.0, 3.0);
    so.add_operator_product(pp_0, c0).unwrap();
    so.add_operator_product(pp_1, c1).unwrap();
    so.add_operator_product(pp_2, c2).unwrap();

    let mut dec_op = DecoherenceOperator::new();
    let dp_0 = DecoherenceProduct::new().x(0).iy(1).z(2);
    let dp_1 = DecoherenceProduct::new().x(0).iy(1).iy(2);
    let dp_2 = DecoherenceProduct::new().iy(0).iy(1).iy(2);
    let d0 = CalculatorComplex::new(2.0, -1.0);
    let d1 = CalculatorComplex::new(-2.0, -1.0);
    let d2 = CalculatorComplex::new(-3.0, 2.0);
    dec_op.add_operator_product(dp_0, d0).unwrap();
    dec_op.add_operator_product(dp_1, d1).unwrap();
    dec_op.add_operator_product(dp_2, d2).unwrap();

    assert_eq!(DecoherenceOperator::from(so), dec_op);
}

// Test the Debug trait of DecoherenceOperator
#[test]
fn debug() {
    let pp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut so = DecoherenceOperator::new();
    let _ = so.set(pp, CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{:?}", so),
        "DecoherenceOperator { internal_map: {DecoherenceProduct { items: [(0, Z)] }: CalculatorComplex { re: Float(0.5), im: Float(0.0) }} }"
    );
}

// Test the Display trait of DecoherenceOperator
#[test]
fn display() {
    let mut so = DecoherenceOperator::new();
    let pp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let _ = so.set(pp, CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{}", so),
        "DecoherenceOperator{\n0Z: (5e-1 + i * 0e0),\n}"
    );
}

// Test the hermitian_conjugate and is_natural_hermitian functions of the HermitianMixedProduct
#[test]
fn hermitian_test() {
    let pp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut system = DecoherenceOperator::new();
    system
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(system.hermitian_conjugate(), system.clone());
}

// Test the Clone and PartialEq traits of DecoherenceOperator
#[test]
fn clone_partial_eq() {
    let pp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut so = DecoherenceOperator::new();
    so.set(pp, CalculatorComplex::from(0.5)).unwrap();

    // Test Clone trait
    assert_eq!(so.clone(), so);

    // Test PartialEq trait
    let pp_1: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut so_1 = DecoherenceOperator::new();
    so_1.set(pp_1, CalculatorComplex::from(0.5)).unwrap();
    let pp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut so_2 = DecoherenceOperator::new();
    so_2.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert!(so_1 == so);
    assert!(so == so_1);
    assert!(so_2 != so);
    assert!(so != so_2);
}

#[test]
fn serde_json() {
    let pp = DecoherenceProduct::new().x(0);
    let mut so = DecoherenceOperator::new();
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let serialized = serde_json::to_string(&so).unwrap();
    let deserialized: DecoherenceOperator = serde_json::from_str(&serialized).unwrap();
    assert_eq!(so, deserialized);
}

/// Test DecoherenceOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_readable() {
    let pp = DecoherenceProduct::new().x(0);
    let mut so = DecoherenceOperator::new();
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    assert_tokens(
        &so.readable(),
        &[
            Token::Struct {
                name: "DecoherenceOperatorSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 3 },
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
            Token::Str("DecoherenceOperator"),
            Token::Str("min_version"),
            Token::Tuple { len: 3 },
            Token::U64(2),
            Token::U64(0),
            Token::U64(0),
            Token::TupleEnd,
            Token::Str("version"),
            Token::Str("2.0.0-alpha.0"),
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

#[test]
fn bincode() {
    let pp = DecoherenceProduct::new().x(0);
    let mut so = DecoherenceOperator::new();
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let encoded: Vec<u8> = bincode::serialize(&so).unwrap();
    let decoded: DecoherenceOperator = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(so, decoded);

    let encoded: Vec<u8> = bincode::serialize(&so.clone().compact()).unwrap();
    let decoded: DecoherenceOperator = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(so, decoded);
}

/// Test DecoherenceOperator Serialization and Deserialization traits (compact)
#[test]
fn serde_compact() {
    let pp = DecoherenceProduct::new().x(0);
    let mut so = DecoherenceOperator::new();
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    assert_tokens(
        &so.compact(),
        &[
            Token::Struct {
                name: "DecoherenceOperatorSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 3 },
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
            Token::Str("DecoherenceOperator"),
            Token::Str("min_version"),
            Token::Tuple { len: 3 },
            Token::U64(2),
            Token::U64(0),
            Token::U64(0),
            Token::TupleEnd,
            Token::Str("version"),
            Token::Str("2.0.0-alpha.0"),
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

#[cfg(feature = "json_schema")]
#[test]
fn test_decoherence_operator_schema() {
    let mut op = DecoherenceOperator::new();
    op.set(DecoherenceProduct::new().x(0), 1.0.into()).unwrap();
    op.set(DecoherenceProduct::new().iy(1).z(2), "val".into())
        .unwrap();
    let schema = schemars::schema_for!(DecoherenceOperator);
    let schema_checker = jsonschema::validator_for(&serde_json::to_value(&schema).unwrap())
        .expect("schema is valid");
    let value = serde_json::to_value(&op).unwrap();
    let validation = schema_checker.validate(&value);

    assert!(validation.is_ok());
}

#[cfg(feature = "struqture_1_import")]
#[cfg(feature = "struqture_1_export")]
#[test]
fn test_from_to_struqture_1() {
    let pp_1 = struqture_one::spins::DecoherenceProduct::from_str("0X1iY25Z").unwrap();
    let mut ss_1 = struqture_one::spins::DecoherenceOperator::new();
    struqture_one::OperateOnDensityMatrix::set(&mut ss_1, pp_1.clone(), 1.0.into()).unwrap();

    let pp_2 = DecoherenceProduct::new().x(0).iy(1).z(25);
    let mut ss_2 = DecoherenceOperator::new();
    ss_2.set(pp_2.clone(), 1.0.into()).unwrap();

    assert!(DecoherenceOperator::from_struqture_1(&ss_1).unwrap() == ss_2);
    assert!(ss_1 == ss_2.to_struqture_1().unwrap());
}
