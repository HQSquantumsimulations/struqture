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

//! Integration test for public API of FermionSystem

use bincode::{deserialize, serialize};
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde_test::{assert_tokens, Configure, Token};
use std::collections::BTreeMap;
use std::iter::{FromIterator, IntoIterator};
use std::str::FromStr;
use struqture::fermions::{FermionOperator, FermionProduct, FermionSystem};
use struqture::{
    ModeIndex, OperateOnDensityMatrix, OperateOnModes, OperateOnState, StruqtureError,
};

// Test the new function of the SpinSystem
#[test]
fn new_system() {
    let system = FermionSystem::new(Some(1));
    assert!(system.is_empty());
    assert_eq!(system.operator(), &FermionOperator::default());
    assert_eq!(system.number_modes(), 1_usize)
}

// Test the new function of the SpinSystem with no spins specified
#[test]
fn new_system_none() {
    let system = FermionSystem::new(None);
    assert!(system.operator().is_empty());
    assert_eq!(system.operator(), &FermionOperator::default());
    assert_eq!(system.number_modes(), 0_usize);
    assert_eq!(FermionSystem::new(None), FermionSystem::default());
}

#[test]
fn empty_clone_options() {
    let pp_2: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut system = FermionSystem::new(Some(3));
    system.set(pp_2, CalculatorComplex::from(0.5)).unwrap();

    let empty: Option<usize> = None;
    let full: Option<usize> = Some(3);
    assert_eq!(system.empty_clone(empty), FermionSystem::new(Some(3)));
    assert_eq!(
        system.empty_clone(full),
        FermionSystem::with_capacity(Some(3), 1)
    );
}

// Test the from_spin_operator and spin_operator functions of the FermionSystem with number_spins = None
#[test]
fn from_fermion_operator_none() {
    let mut so: FermionOperator = FermionOperator::new();
    let mut system = FermionSystem::new(None);
    let pp: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    so.add_operator_product(pp.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    system
        .add_operator_product(pp, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(
        system,
        FermionSystem::from_operator(so.clone(), None).unwrap()
    );
    assert_eq!(
        system.operator(),
        FermionSystem::from_operator(so.clone(), None)
            .unwrap()
            .operator()
    );
    assert_eq!(
        &so,
        FermionSystem::from_operator(so.clone(), None)
            .unwrap()
            .operator()
    );
}

// Test the from_spin_operator and spin_operator functions of the FermionSystem with number_spins = Some(2)
#[test]
fn from_fermion_operator_some() {
    let mut so: FermionOperator = FermionOperator::new();
    let mut system = FermionSystem::new(Some(2));
    let pp: FermionProduct = FermionProduct::new([0], [1]).unwrap();
    so.add_operator_product(pp.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    system
        .add_operator_product(pp, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(
        system,
        FermionSystem::from_operator(so.clone(), Some(2)).unwrap()
    );
    assert_eq!(
        system.operator(),
        FermionSystem::from_operator(so.clone(), Some(2))
            .unwrap()
            .operator()
    );
    assert_eq!(
        &so,
        FermionSystem::from_operator(so.clone(), Some(2))
            .unwrap()
            .operator()
    );
    assert_eq!(
        FermionSystem::from_operator(so.clone(), Some(0)),
        Err(StruqtureError::NumberModesExceeded {})
    );
}

// Test the current_number_modes function of the FermionSystem
#[test]
fn internal_map_current_number_modes() {
    let pp_0: FermionProduct = FermionProduct::new([0], [1]).unwrap();
    let pp_2: FermionProduct = FermionProduct::new([2], [3]).unwrap();
    let mut so = FermionSystem::new(Some(4));
    assert_eq!(so.current_number_modes(), 0_usize);
    so.set(pp_0, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.current_number_modes(), 2_usize);
    so.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.current_number_modes(), 4_usize);
}

// Test the len function of the FermionSystem
#[test]
fn internal_map_len() {
    let pp_2: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut so = FermionSystem::new(Some(3));
    so.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.len(), 1_usize);
}
// Test the set, set_pauli_product, get functions of the SpinSystem
#[test]
fn internal_map_set_get_dict() {
    let mut system = FermionSystem::new(Some(1));
    assert_eq!(system.current_number_modes(), 0_usize);
    assert_eq!(system.number_modes(), 1_usize);
    let pp_0: FermionProduct = FermionProduct::new([0], [0]).unwrap();

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

    let pp_2: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let error = system.set(pp_2, CalculatorComplex::from(0.5));
    assert_eq!(error, Err(StruqtureError::NumberModesExceeded));

    // 2) Test iter, keys, values functions
    let mut map: BTreeMap<FermionProduct, CalculatorComplex> = BTreeMap::new();
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

// Test the set, get and remove functions of the FermionSystem
#[test]
fn internal_map_set_get_remove() {
    let pp_2: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut so = FermionSystem::new(Some(3));

    // 1) Test try_set_fermion_product and get functions
    // Vacant
    so.set(pp_2.clone(), CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.get(&pp_2.clone()), &CalculatorComplex::from(0.5));

    // 2) Test remove function
    so.remove(&pp_2);
    assert_eq!(so, FermionSystem::new(Some(3)));
}

// Test the add_operator_product function of the FermionSystem
#[test]
fn internal_map_add_operator_product() {
    let pp_2: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut so = FermionSystem::new(Some(3));

    so.add_operator_product(pp_2.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(so.get(&pp_2), &CalculatorComplex::from(0.5));
    so.add_operator_product(pp_2.clone(), CalculatorComplex::from(-0.5))
        .unwrap();
    assert_eq!(so.get(&pp_2), &CalculatorComplex::from(0.0));

    let pp_3: FermionProduct = FermionProduct::new([0], [3]).unwrap();
    let error = so.add_operator_product(pp_3, CalculatorComplex::from(0.5));
    assert_eq!(error, Err(StruqtureError::NumberModesExceeded {}));
}

// Test the iter, keys and values functions of the FermionSystem
#[test]
fn internal_map_keys() {
    let pp_2: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut so = FermionSystem::new(Some(3));
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

// Test the Iter traits of FermionSystem: into_iter, from_iter and extend
#[test]
fn into_iter_from_iter_extend() {
    let pp_0: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let pp_1: FermionProduct = FermionProduct::new([0], [1]).unwrap();
    let mut so_0 = FermionSystem::new(None);
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0));

    let so_iter = so_0.clone().into_iter();
    assert_eq!(FermionSystem::from_iter(so_iter), so_0);
    let so_iter = (&so_0)
        .into_iter()
        .map(|(key, value)| (key.clone(), value.clone()));
    assert_eq!(FermionSystem::from_iter(so_iter), so_0);
    let mut mapping: BTreeMap<FermionProduct, CalculatorComplex> = BTreeMap::new();
    mapping.insert(pp_1.clone(), CalculatorComplex::from(0.5));
    let mapping_iter = mapping.into_iter();
    so_0.extend(mapping_iter);

    let mut so_0_1 = FermionSystem::new(None);
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(1.0));
    let _ = so_0_1.add_operator_product(pp_1, CalculatorComplex::from(0.5));

    assert_eq!(so_0, so_0_1);
}

// Test the Iter traits of SpinSystem: extend with a panic
#[test]
#[should_panic]
fn iter_extend_panic() {
    let pp_0: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let pp_1: FermionProduct = FermionProduct::new([0], [1]).unwrap();
    let mut system = FermionSystem::new(None);
    let _ = system.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0));

    let mut mapping: BTreeMap<FermionProduct, CalculatorComplex> = BTreeMap::new();
    mapping.insert(pp_1.clone(), CalculatorComplex::from(0.5));
    let mapping_iter = mapping.into_iter();
    system.extend(mapping_iter);

    let mut system_0 = FermionSystem::new(Some(2));
    system_0.add_operator_product(pp_0, 1.0.into()).unwrap();
    system_0.add_operator_product(pp_1, 0.5.into()).unwrap();
    assert_eq!(system, system_0);
}

// Test the hermitian_conjugate and is_natural_hermitian functions of the HermitianMixedProduct
#[test]
fn hermitian_test() {
    let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let mut system = FermionSystem::new(Some(1));
    let _ = system.add_operator_product(pp, CalculatorComplex::from(1.0));

    assert_eq!(system.hermitian_conjugate(), system.clone());
}

// Test the negative operation: -FermionSystem
#[test]
fn negative_so() {
    let pp_0: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let mut so_0 = FermionSystem::new(Some(1));
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0));
    let mut so_0_minus = FermionSystem::new(Some(1));
    let _ = so_0_minus.add_operator_product(pp_0, CalculatorComplex::from(-1.0));

    assert_eq!(-so_0, so_0_minus);
}

// Test the addition: FermionSystem + FermionSystem
#[test]
fn add_so_so() {
    let pp_0: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let pp_1: FermionProduct = FermionProduct::new([0], [1]).unwrap();
    let mut so_0 = FermionSystem::new(Some(2));
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0));
    let mut so_1 = FermionSystem::new(Some(2));
    let _ = so_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(0.5));
    let mut so_0_1 = FermionSystem::new(Some(2));
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(1.0));
    let _ = so_0_1.add_operator_product(pp_1, CalculatorComplex::from(0.5));

    assert_eq!(so_0 + so_1, Ok(so_0_1));
}

// Test the subtraction: FermionSystem - FermionSystem
#[test]
fn sub_so_so() {
    let pp_0: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let pp_1: FermionProduct = FermionProduct::new([0], [1]).unwrap();
    let mut so_0 = FermionSystem::new(Some(2));
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0));
    let mut so_1 = FermionSystem::new(Some(2));
    let _ = so_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(0.5));
    let mut so_0_1 = FermionSystem::new(Some(2));
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(1.0));
    let _ = so_0_1.add_operator_product(pp_1, CalculatorComplex::from(-0.5));

    assert_eq!(so_0 - so_1, Ok(so_0_1));
}

// Test the multiplication: FermionSystem * FermionSystem
#[test]
fn mul_so_so() {
    let pp_0: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let pp_1: FermionProduct = FermionProduct::new([1], [1]).unwrap();
    let pp_0_1: FermionProduct = FermionProduct::new([0, 1], [0, 1]).unwrap();
    let mut so_0 = FermionSystem::new(Some(2));
    let _ = so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0));
    let mut so_1 = FermionSystem::new(Some(2));
    let _ = so_1.add_operator_product(pp_1, CalculatorComplex::from(0.5));
    let mut so_0_1 = FermionSystem::new(Some(2));
    let _ = so_0_1.add_operator_product(pp_0_1, CalculatorComplex::from(-1.0));
    assert_eq!((so_0 * so_1), so_0_1);
}

// Test the multiplication: FermionSystem * FermionSystem where they have a FermionProduct with the same index
#[test]
fn mul_so_so_same_index() {
    let pp_0: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let pp_1: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let pp_0_1: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let mut so_0 = FermionSystem::new(Some(1));
    let _ = so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0));
    let mut so_1 = FermionSystem::new(Some(1));
    let _ = so_1.add_operator_product(pp_1, CalculatorComplex::from(0.5));
    let mut so_0_1 = FermionSystem::new(Some(1));
    let _ = so_0_1.add_operator_product(pp_0_1, CalculatorComplex::new(1.0, 0.0));

    assert_eq!(so_0 * so_1, so_0_1);
}

// Test the multiplication: FermionSystem * Calculatorcomplex
#[test]
fn mul_so_cc() {
    let pp_0: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut so_0 = FermionSystem::new(Some(3));
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(2.0));
    let mut so_0_1 = FermionSystem::new(Some(3));
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(6.0));

    assert_eq!(so_0 * CalculatorComplex::from(3.0), so_0_1);
}

// Test the multiplication: FermionSystem * CalculatorFloat
#[test]
fn mul_so_cf() {
    let pp_0: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut so_0 = FermionSystem::new(Some(3));
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(2.0));
    let mut so_0_1 = FermionSystem::new(Some(3));
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(6.0));

    assert_eq!(so_0 * CalculatorFloat::from(3.0), so_0_1);
}

// Test the Debug trait of FermionSystem
#[test]
fn debug() {
    let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let mut so = FermionSystem::new(Some(1));
    let _ = so.set(pp, CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{:?}", so),
        "FermionSystem { number_modes: Some(1), operator: FermionOperator { internal_map: {FermionProduct { creators: [0], annihilators: [0] }: CalculatorComplex { re: Float(0.5), im: Float(0.0) }} } }"
    );
}

// Test the Display trait of FermionSystem
#[test]
fn display() {
    let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let mut so = FermionSystem::new(Some(1));
    let _ = so.set(pp, CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{}", so),
        "FermionSystem(1){\nc0a0: (5e-1 + i * 0e0),\n}"
    );
}

// Test the Clone and PartialEq traits of FermionSystem
#[test]
fn clone_partial_eq() {
    let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let mut so = FermionSystem::new(Some(1));
    so.set(pp, CalculatorComplex::from(0.5)).unwrap();

    // Test Clone trait
    assert_eq!(so.clone(), so);

    // Test PartialEq trait
    let pp_1: FermionProduct = FermionProduct::new([0], [0]).unwrap();
    let mut so_1 = FermionSystem::new(Some(1));
    so_1.set(pp_1, CalculatorComplex::from(0.5)).unwrap();
    let pp_2: FermionProduct = FermionProduct::new([0], [2]).unwrap();
    let mut so_2 = FermionSystem::new(Some(3));
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
    let mut so = FermionSystem::new(Some(3));
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let serialized = serde_json::to_string(&so).unwrap();
    let deserialized: FermionSystem = serde_json::from_str(&serialized).unwrap();

    assert_eq!(so, deserialized);
}

/// Test SpinOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_readable() {
    use struqture::STRUQTURE_VERSION;
    let mut rsplit = STRUQTURE_VERSION.split('.').take(2);
    let major_version = u32::from_str(
        rsplit
            .next()
            .expect("Internal error: Version not conforming to semver"),
    )
    .expect("Internal error: Major version is not unsigned integer.");
    let minor_version = u32::from_str(
        rsplit
            .next()
            .expect("Internal error: Version not conforming to semver"),
    )
    .expect("Internal error: Minor version is not unsigned integer.");

    let pp = FermionProduct::new([0], [0]).unwrap();
    let mut so = FermionSystem::new(Some(1));
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();
    assert_tokens(
        &so.readable(),
        &[
            Token::Struct {
                name: "FermionSystem",
                len: 2,
            },
            Token::Str("number_modes"),
            Token::Some,
            Token::U64(1),
            Token::Str("operator"),
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
    let pp = FermionProduct::new([0], [1]).unwrap();
    let mut so = FermionSystem::new(Some(2));
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let serialized = serialize(&so).unwrap();
    let deserialized: FermionSystem = deserialize(&serialized).unwrap();
    assert_eq!(deserialized, so);

    let encoded: Vec<u8> = bincode::serialize(&so.clone().compact()).unwrap();
    let decoded: FermionSystem = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(so, decoded);
}

#[test]
fn serde_compact() {
    use struqture::STRUQTURE_VERSION;
    let mut rsplit = STRUQTURE_VERSION.split('.').take(2);
    let major_version = u32::from_str(
        rsplit
            .next()
            .expect("Internal error: Version not conforming to semver"),
    )
    .expect("Internal error: Major version is not unsigned integer.");
    let minor_version = u32::from_str(
        rsplit
            .next()
            .expect("Internal error: Version not conforming to semver"),
    )
    .expect("Internal error: Minor version is not unsigned integer.");

    let pp = FermionProduct::new([0], [0]).unwrap();
    let mut so = FermionSystem::new(Some(1));
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    assert_tokens(
        &so.compact(),
        &[
            Token::Struct {
                name: "FermionSystem",
                len: 2,
            },
            Token::Str("number_modes"),
            Token::Some,
            Token::U64(1),
            Token::Str("operator"),
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
