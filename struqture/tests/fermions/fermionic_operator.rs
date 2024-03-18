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

//! Integration test for public API of FermionOperator

use bincode::{deserialize, serialize};
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde_test::{assert_tokens, Configure, Token};
use std::collections::BTreeMap;
use std::iter::{FromIterator, IntoIterator};
#[cfg(feature = "struqture_1_import")]
#[cfg(feature = "struqture_1_export")]
use std::str::FromStr;
use struqture::fermions::{
    FermionHamiltonian, FermionOperator, FermionProduct, HermitianFermionProduct,
};
use struqture::{ModeIndex, OperateOnDensityMatrix, OperateOnModes, OperateOnState};
use test_case::test_case;

// Test the new function of the FermionOperator
#[test]
fn new() {
    let so = FermionOperator::new();
    assert!(so.is_empty());
    assert_eq!(FermionOperator::new(), FermionOperator::default());
    assert_eq!(FermionOperator::with_capacity(2), FermionOperator::new());
}

#[test]
fn empty_clone_options() {
    let pp_2: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut system = FermionOperator::new();
    system.set(pp_2, CalculatorComplex::from(0.5)).unwrap();

    let empty: Option<usize> = None;
    let full: Option<usize> = Some(3);
    assert_eq!(system.empty_clone(empty), FermionOperator::new());
    assert_eq!(system.empty_clone(full), FermionOperator::with_capacity(1));
}

// Test the len function of the FermionOperator
#[test]
fn internal_map_len() {
    let pp_2: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut so = FermionOperator::new();
    so.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.len(), 1_usize);
}
// Test the set, set_pauli_product, get functions of the SpinSystem
#[test]
fn internal_map_set_get_dict() {
    let mut system = FermionOperator::new();
    assert_eq!(system.number_modes(), 0_usize);
    let pp_0: FermionProduct = FermionProduct::new([0], [0]).unwrap();

    // 1) Test try_set_pauli_product and get functions
    // Vacant
    system
        .set(pp_0.clone(), CalculatorComplex::from(0.0))
        .unwrap();
    system
        .set(pp_0.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(system.number_modes(), 1_usize);
    assert_eq!(system.get(&pp_0), &CalculatorComplex::from(0.5));

    // 2) Test iter, keys, values functions
    let mut map: BTreeMap<FermionProduct, CalculatorComplex> = BTreeMap::new();
    map.insert(pp_0.clone(), CalculatorComplex::from(0.5));
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

    // Test other arms of `set` function
    system
        .set(pp_0.clone(), CalculatorComplex::from(0.0))
        .unwrap();
    assert_eq!(system.get(&pp_0), &CalculatorComplex::from(0.0));
}

// Test the set, get and remove functions of the FermionOperator
#[test]
fn internal_map_set_get_remove() {
    let pp_2: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut so = FermionOperator::new();

    // 1) Test try_set_fermion_product and get functions
    // Vacant
    so.set(pp_2.clone(), CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.get(&pp_2.clone()), &CalculatorComplex::from(0.5));

    // 2) Test remove function
    so.remove(&pp_2);
    assert_eq!(so, FermionOperator::new());
}

// Test the add_operator_product function of the FermionOperator
#[test]
fn internal_map_add_operator_product() {
    let pp_2: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut so = FermionOperator::new();

    so.add_operator_product(pp_2.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(so.get(&pp_2), &CalculatorComplex::from(0.5));
    so.add_operator_product(pp_2.clone(), CalculatorComplex::from(-0.5))
        .unwrap();
    assert_eq!(so.get(&pp_2), &CalculatorComplex::from(0.0));
}

// Test the iter, keys and values functions of the FermionOperator
#[test]
fn internal_map_keys() {
    let pp_2: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut so = FermionOperator::new();
    so.set(pp_2.clone(), CalculatorComplex::from(0.5)).unwrap();

    let mut map: BTreeMap<FermionProduct, CalculatorComplex> = BTreeMap::new();
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
    let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let mut system = FermionOperator::new();
    let _ = system.add_operator_product(pp, CalculatorComplex::from(1.0));

    assert_eq!(system.hermitian_conjugate(), system.clone());
}

// Test the negative operation: -FermionOperator
#[test]
fn negative_so() {
    let pp_0: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let mut so_0 = FermionOperator::new();
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0));
    let mut so_0_minus = FermionOperator::new();
    let _ = so_0_minus.add_operator_product(pp_0, CalculatorComplex::from(-1.0));

    assert_eq!(-so_0, so_0_minus);
}

// Test the addition: FermionOperator + FermionOperator
#[test]
fn add_so_so() {
    let pp_0: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let pp_1: FermionProduct = FermionProduct::new([0], [1]).unwrap();
    let mut so_0 = FermionOperator::new();
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0));
    let mut so_1 = FermionOperator::new();
    let _ = so_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(0.5));
    let mut so_0_1 = FermionOperator::new();
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(1.0));
    let _ = so_0_1.add_operator_product(pp_1, CalculatorComplex::from(0.5));

    assert_eq!(so_0 + so_1, so_0_1);
}

// Test the subtraction: FermionOperator - FermionOperator
#[test]
fn sub_so_so() {
    let pp_0: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let pp_1: FermionProduct = FermionProduct::new([0], [1]).unwrap();
    let mut so_0 = FermionOperator::new();
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0));
    let mut so_1 = FermionOperator::new();
    let _ = so_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(0.5));
    let mut so_0_1 = FermionOperator::new();
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(1.0));
    let _ = so_0_1.add_operator_product(pp_1, CalculatorComplex::from(-0.5));

    assert_eq!(so_0 - so_1, so_0_1);
}

// Test the multiplication: FermionOperator * FermionOperator
#[test]
fn mul_so_so() {
    let pp_0: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let pp_1: FermionProduct = FermionProduct::new([1], [1]).unwrap();
    let pp_0_1: FermionProduct = FermionProduct::new([0, 1], [0, 1]).unwrap();
    let mut so_0 = FermionOperator::new();
    let _ = so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0));
    let mut so_1 = FermionOperator::new();
    let _ = so_1.add_operator_product(pp_1, CalculatorComplex::from(0.5));
    let mut so_0_1 = FermionOperator::new();
    let _ = so_0_1.add_operator_product(pp_0_1, CalculatorComplex::from(-1.0));
    assert_eq!((so_0 * so_1), so_0_1);
}

// Test the multiplication: FermionOperator * FermionOperator where they have a FermionProduct with the same index
#[test]
fn mul_so_so_same_index() {
    let pp_0: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let pp_1: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let pp_0_1: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let mut so_0 = FermionOperator::new();
    let _ = so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0));
    let mut so_1 = FermionOperator::new();
    let _ = so_1.add_operator_product(pp_1, CalculatorComplex::from(0.5));
    let mut so_0_1 = FermionOperator::new();
    let _ = so_0_1.add_operator_product(pp_0_1, CalculatorComplex::new(1.0, 0.0));

    assert_eq!(so_0 * so_1, so_0_1);
}

// Test the multiplication: FermionOperator * Calculatorcomplex
#[test]
fn mul_so_cc() {
    let pp_0: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut so_0 = FermionOperator::new();
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(2.0));
    let mut so_0_1 = FermionOperator::new();
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(6.0));

    assert_eq!(so_0 * CalculatorComplex::from(3.0), so_0_1);
}

// Test the multiplication: FermionOperator * CalculatorFloat
#[test]
fn mul_so_cf() {
    let pp_0: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut so_0 = FermionOperator::new();
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(2.0));
    let mut so_0_1 = FermionOperator::new();
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(6.0));

    assert_eq!(so_0 * CalculatorFloat::from(3.0), so_0_1);
}

#[test_case(&[], &[]; "empty")]
#[test_case(&[0], &[1]; "0 - 1")]
#[test_case(&[], &[2000]; "empty - 2000")]
#[test_case(&[0, 1, 2], &[3, 4, 5]; "0, 1, 2 - 3, 4, 5")]
fn from_operator_pass(creators: &[usize], annihilators: &[usize]) {
    let pp_0: HermitianFermionProduct =
        HermitianFermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let pp_1: FermionProduct =
        FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let pp_1_conj: FermionProduct =
        FermionProduct::new(annihilators.to_vec(), creators.to_vec()).unwrap();
    let mut so_0_1 = FermionOperator::new();
    let _ = so_0_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(2.0));
    if pp_1 != pp_1_conj {
        let _ = so_0_1.add_operator_product(pp_1_conj, CalculatorComplex::from(2.0));
    }
    let mut so_0 = FermionHamiltonian::new();
    let _ = so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0));

    assert_eq!(FermionOperator::from(so_0), so_0_1);
}

// Test the Iter traits of FermionOperator: into_iter, from_iter and extend
#[test]
fn into_iter_from_iter_extend() {
    let pp_0: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let pp_1: FermionProduct = FermionProduct::new([0], [1]).unwrap();
    let mut so_0 = FermionOperator::new();
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0));

    let so_iter = so_0.clone().into_iter();
    assert_eq!(FermionOperator::from_iter(so_iter), so_0);
    let so_iter = (&so_0)
        .into_iter()
        .map(|(key, value)| (key.clone(), value.clone()));
    assert_eq!(FermionOperator::from_iter(so_iter), so_0);
    let mut mapping: BTreeMap<FermionProduct, CalculatorComplex> = BTreeMap::new();
    mapping.insert(pp_1.clone(), CalculatorComplex::from(0.5));
    let mapping_iter = mapping.into_iter();
    so_0.extend(mapping_iter);

    let mut so_0_1 = FermionOperator::new();
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(1.0));
    let _ = so_0_1.add_operator_product(pp_1, CalculatorComplex::from(0.5));

    assert_eq!(so_0, so_0_1);
}

// Test the Debug trait of FermionOperator
#[test]
fn debug() {
    let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let mut so = FermionOperator::new();
    let _ = so.set(pp, CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{:?}", so),
        "FermionOperator { internal_map: {FermionProduct { creators: [0], annihilators: [0] }: CalculatorComplex { re: Float(0.5), im: Float(0.0) }} }"
    );
}

// Test the Display trait of FermionOperator
#[test]
fn display() {
    let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let mut so = FermionOperator::new();
    let _ = so.set(pp, CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{}", so),
        "FermionOperator{\nc0a0: (5e-1 + i * 0e0),\n}"
    );
}

// Test the Clone and PartialEq traits of FermionOperator
#[test]
fn clone_partial_eq() {
    let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let mut so = FermionOperator::new();
    so.set(pp, CalculatorComplex::from(0.5)).unwrap();

    // Test Clone trait
    assert_eq!(so.clone(), so);

    // Test PartialEq trait
    let pp_1: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let mut so_1 = FermionOperator::new();
    so_1.set(pp_1, CalculatorComplex::from(0.5)).unwrap();
    let pp_2: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut so_2 = FermionOperator::new();
    so_2.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert!(so_1 == so);
    assert!(so == so_1);
    assert!(so_2 != so);
    assert!(so != so_2);
}

/// Test SpinOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_json() {
    let pp = FermionProduct::new([0], [2]).unwrap();
    let mut so = FermionOperator::new();
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let serialized = serde_json::to_string(&so).unwrap();
    let deserialized: FermionOperator = serde_json::from_str(&serialized).unwrap();

    assert_eq!(so, deserialized);
}

/// Test SpinOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_readable() {
    let pp = FermionProduct::new([0], [0]).unwrap();
    let mut so = FermionOperator::new();
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();
    assert_tokens(
        &so.readable(),
        &[
            Token::Struct {
                name: "FermionOperatorSerialize",
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
            Token::Str("serialisation_meta"),
            Token::Struct {
                name: "StruqtureSerialisationMeta",
                len: 3,
            },
            Token::Str("type_name"),
            Token::Str("FermionOperator"),
            Token::Str("min_version"),
            Token::Tuple { len: 3 },
            Token::U64(2),
            Token::U64(0),
            Token::U64(0),
            Token::TupleEnd,
            Token::Str("version"),
            Token::Str("2.0.0"),
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

#[test]
fn bincode() {
    let pp = FermionProduct::new([0], [1]).unwrap();
    let mut so = FermionOperator::new();
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let serialized = serialize(&so).unwrap();
    let deserialized: FermionOperator = deserialize(&serialized).unwrap();
    assert_eq!(deserialized, so);

    let encoded: Vec<u8> = bincode::serialize(&so.clone().compact()).unwrap();
    let decoded: FermionOperator = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(so, decoded);
}

#[test]
fn serde_compact() {
    let pp = FermionProduct::new([0], [0]).unwrap();
    let mut so = FermionOperator::new();
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    assert_tokens(
        &so.compact(),
        &[
            Token::Struct {
                name: "FermionOperatorSerialize",
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
            Token::Str("serialisation_meta"),
            Token::Struct {
                name: "StruqtureSerialisationMeta",
                len: 3,
            },
            Token::Str("type_name"),
            Token::Str("FermionOperator"),
            Token::Str("min_version"),
            Token::Tuple { len: 3 },
            Token::U64(2),
            Token::U64(0),
            Token::U64(0),
            Token::TupleEnd,
            Token::Str("version"),
            Token::Str("2.0.0"),
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

#[cfg(feature = "json_schema")]
#[test]
fn test_fermion_operator_schema() {
    let mut op = FermionOperator::new();
    op.set(FermionProduct::new([0], [0]).unwrap(), 1.0.into())
        .unwrap();
    op.set(FermionProduct::new([1], [1]).unwrap(), "val".into())
        .unwrap();
    let schema = schemars::schema_for!(FermionOperator);
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
    let pp_1 = struqture_one::fermions::FermionProduct::from_str("c0a1").unwrap();
    let mut ss_1 = struqture_one::fermions::FermionSystem::new(None);
    struqture_one::OperateOnDensityMatrix::set(&mut ss_1, pp_1.clone(), 1.0.into()).unwrap();

    let pp_2 = FermionProduct::new([0], [1]).unwrap();
    let mut ss_2 = FermionOperator::new();
    ss_2.set(pp_2.clone(), 1.0.into()).unwrap();

    assert!(FermionOperator::from_struqture_1(&ss_1).unwrap() == ss_2);
    assert!(ss_1 == ss_2.to_struqture_1().unwrap());
}
