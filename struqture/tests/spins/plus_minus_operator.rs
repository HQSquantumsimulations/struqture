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

//! Integration test for public API of PlusMinusOperator

use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde_test::{assert_tokens, Configure, Token};
use std::collections::BTreeMap;
use std::iter::{FromIterator, IntoIterator};
use std::ops::{Add, Sub};
#[cfg(feature = "struqture_1_import")]
#[cfg(feature = "struqture_1_export")]
use std::str::FromStr;
use struqture::prelude::*;
use struqture::spins::{
    DecoherenceOperator, DecoherenceProduct, PauliProduct, PlusMinusOperator, PlusMinusProduct,
    QubitHamiltonian, QubitOperator,
};
use struqture::OperateOnDensityMatrix;

// Test the new function of the PlusMinusOperator
#[test]
fn new() {
    let so = PlusMinusOperator::new();
    assert!(so.is_empty());
    assert_eq!(PlusMinusOperator::new(), PlusMinusOperator::default())
}

#[test]
fn empty_clone_options() {
    let pp_2: PlusMinusProduct = PlusMinusProduct::new().z(2);
    let mut system = PlusMinusOperator::new();
    system.set(pp_2, CalculatorComplex::from(0.5)).unwrap();

    let empty: Option<usize> = None;
    let full: Option<usize> = Some(3);
    assert_eq!(system.empty_clone(empty), PlusMinusOperator::new());
    assert_eq!(
        system.empty_clone(full),
        PlusMinusOperator::with_capacity(1)
    );
}

// // Test the current_number_spins function of the PlusMinusOperator
// #[test]
// fn internal_map_number_spins() {
//     let pp_0: PlusMinusProduct = PlusMinusProduct::new().plus(0);
//     let pp_2: PlusMinusProduct = PlusMinusProduct::new().z(2);
//     let mut so = PlusMinusOperator::new();
//     assert_eq!(so.current_number_spins(), 0_usize);
//     so.set(pp_0, CalculatorComplex::from(0.5)).unwrap();
//     assert_eq!(so.current_number_spins(), 1_usize);
//     so.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
//     assert_eq!(so.current_number_spins(), 3_usize);
// }

// Test the len function of the PlusMinusOperator
#[test]
fn internal_map_len() {
    let pp_2: PlusMinusProduct = PlusMinusProduct::new().z(2);
    let mut so = PlusMinusOperator::new();
    so.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.len(), 1_usize);
}

// Test the set, set_pauli_product, get functions of the SpinSystem
#[test]
fn internal_map_set_get_dict() {
    let mut system = PlusMinusOperator::new();
    // assert_eq!(system.current_number_spins(), 0_usize);
    let pp_0: PlusMinusProduct = PlusMinusProduct::new().z(0);

    // 1) Test try_set_pauli_product and get functions
    // Vacant
    system
        .set(pp_0.clone(), CalculatorComplex::from(0.0))
        .unwrap();
    system
        .set(pp_0.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    // assert_eq!(system.current_number_spins(), 1_usize);
    assert_eq!(system.get(&pp_0), &CalculatorComplex::from(0.5));

    // 2) Test iter, keys, values functions
    let mut map: BTreeMap<PlusMinusProduct, CalculatorComplex> = BTreeMap::new();
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

// Test the set, get and remove functions of the PlusMinusOperator
#[test]
fn internal_map_set_get_remove() {
    let pp_2: PlusMinusProduct = PlusMinusProduct::new().z(2);
    let mut so = PlusMinusOperator::new();

    // 1) Test try_set_pauli_product and get functions
    // Vacant
    so.set(pp_2.clone(), CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.get(&pp_2.clone()), &CalculatorComplex::from(0.5));

    // 2) Test remove function
    so.remove(&pp_2);
    assert_eq!(so, PlusMinusOperator::new());
}

// Test the add_operator_product function of the PlusMinusOperator
#[test]
fn internal_map_add_operator_product() {
    let pp_2: PlusMinusProduct = PlusMinusProduct::new().z(2);
    let mut so = PlusMinusOperator::new();

    so.add_operator_product(pp_2.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(so.get(&pp_2), &CalculatorComplex::from(0.5));
    so.add_operator_product(pp_2.clone(), CalculatorComplex::from(-0.5))
        .unwrap();
    assert_eq!(so.get(&pp_2), &CalculatorComplex::from(0.0));
}

// Test the iter, keys and values functions of the PlusMinusOperator
#[test]
fn internal_map_keys() {
    let pp_2: PlusMinusProduct = PlusMinusProduct::new().z(2);
    let mut so = PlusMinusOperator::new();
    so.set(pp_2.clone(), CalculatorComplex::from(0.5)).unwrap();

    let mut map: BTreeMap<PlusMinusProduct, CalculatorComplex> = BTreeMap::new();
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

// Test the Iter traits of PlusMinusOperator: into_iter, from_iter and extend
#[test]
fn into_iter_from_iter_extend() {
    let pp_0: PlusMinusProduct = PlusMinusProduct::new().z(0);
    let pp_1: PlusMinusProduct = PlusMinusProduct::new().plus(1);
    let mut system = PlusMinusOperator::new();
    system
        .add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();

    let system_iter = system.clone().into_iter();
    assert_eq!(PlusMinusOperator::from_iter(system_iter), system);
    let system_iter = (&system)
        .into_iter()
        .map(|(key, value)| (key.clone(), value.clone()));
    assert_eq!(PlusMinusOperator::from_iter(system_iter), system);
    let mut hamiltonian = PlusMinusOperator::new();
    hamiltonian
        .add_operator_product(pp_0.clone(), 1.0.into())
        .unwrap();
    for (first, second) in system.into_iter().zip(hamiltonian.iter()) {
        assert_eq!(first.0, *second.0);
        assert_eq!(first.1, *second.1);
    }

    let mut system = PlusMinusOperator::new();
    system
        .add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut mapping: BTreeMap<PlusMinusProduct, CalculatorComplex> = BTreeMap::new();
    mapping.insert(pp_1.clone(), CalculatorComplex::from(0.5));
    let mapping_iter = mapping.into_iter();
    system.extend(mapping_iter);

    let mut system_1 = PlusMinusOperator::new();
    system_1
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    system_1
        .add_operator_product(pp_1, CalculatorComplex::from(0.5))
        .unwrap();

    assert_eq!(system, system_1);
}

// Test the negative operation: -PlusMinusOperator
#[test]
fn negative_so() {
    let pp_0: PlusMinusProduct = PlusMinusProduct::new().z(0);
    let mut so_0 = PlusMinusOperator::new();
    so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut so_0_minus = PlusMinusOperator::new();
    so_0_minus
        .add_operator_product(pp_0, CalculatorComplex::from(-1.0))
        .unwrap();

    assert_eq!(-so_0, so_0_minus);
}

// Test the addition: PlusMinusOperator + PlusMinusOperator
#[test]
fn add_so_so() {
    let pp_0: PlusMinusProduct = PlusMinusProduct::new().z(0);
    let pp_1: PlusMinusProduct = PlusMinusProduct::new().plus(1);
    let mut so_0 = PlusMinusOperator::new();
    so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut so_1 = PlusMinusOperator::new();
    so_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    let mut so_0_1 = PlusMinusOperator::new();
    so_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    so_0_1
        .add_operator_product(pp_1, CalculatorComplex::from(0.5))
        .unwrap();

    assert_eq!(so_0.clone() + so_1.clone(), so_0_1);
    assert_eq!(so_0.add(so_1), so_0_1);
}

// Test the subtraction: PlusMinusOperator - PlusMinusOperator
#[test]
fn sub_so_so() {
    let pp_0: PlusMinusProduct = PlusMinusProduct::new().z(0);
    let pp_1: PlusMinusProduct = PlusMinusProduct::new().plus(1);
    let mut so_0 = PlusMinusOperator::new();
    so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut so_1 = PlusMinusOperator::new();
    so_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    let mut so_0_1 = PlusMinusOperator::new();
    so_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    so_0_1
        .add_operator_product(pp_1, CalculatorComplex::from(-0.5))
        .unwrap();

    assert_eq!(so_0.clone() - so_1.clone(), so_0_1);
    assert_eq!(so_0.sub(so_1), so_0_1);
}

// Test the multiplication: PlusMinusOperator * Calculatorcomplex
#[test]
fn mul_so_cc() {
    let pp_0: PlusMinusProduct = PlusMinusProduct::new().z(0);
    let mut so_0 = PlusMinusOperator::new();
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(2.0));
    let mut so_0_1 = PlusMinusOperator::new();
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(6.0));

    assert_eq!(so_0 * CalculatorComplex::from(3.0), so_0_1);
}

// Test the Debug trait of PlusMinusOperator
#[test]
fn debug() {
    let pp: PlusMinusProduct = PlusMinusProduct::new().z(0);
    let mut so = PlusMinusOperator::new();
    let _ = so.set(pp, CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{:?}", so),
        "PlusMinusOperator { internal_map: {PlusMinusProduct { items: [(0, Z)] }: CalculatorComplex { re: Float(0.5), im: Float(0.0) }} }"
    );
}

// Test the Display trait of PlusMinusOperator
#[test]
fn display() {
    let mut so = PlusMinusOperator::new();
    let pp: PlusMinusProduct = PlusMinusProduct::new().z(0);
    let _ = so.set(pp, CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{}", so),
        "PlusMinusOperator{\n0Z: (5e-1 + i * 0e0),\n}"
    );
}

// Test the hermitian_conjugate and is_natural_hermitian functions of the HermitianMixedProduct
#[test]
fn hermitian_test() {
    let pp_0: PlusMinusProduct = PlusMinusProduct::new().z(0);
    let mut system = PlusMinusOperator::new();
    system
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(system.hermitian_conjugate(), system.clone());
}

// Test the Clone and PartialEq traits of PlusMinusOperator
#[test]
fn clone_partial_eq() {
    let pp: PlusMinusProduct = PlusMinusProduct::new().z(0);
    let mut so = PlusMinusOperator::new();
    so.set(pp, CalculatorComplex::from(0.5)).unwrap();

    // Test Clone trait
    assert_eq!(so.clone(), so);

    // Test PartialEq trait
    let pp_1: PlusMinusProduct = PlusMinusProduct::new().z(0);
    let mut so_1 = PlusMinusOperator::new();
    so_1.set(pp_1, CalculatorComplex::from(0.5)).unwrap();
    let pp_2: PlusMinusProduct = PlusMinusProduct::new().z(2);
    let mut so_2 = PlusMinusOperator::new();
    so_2.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert!(so_1 == so);
    assert!(so == so_1);
    assert!(so_2 != so);
    assert!(so != so_2);
}

/// Test PlusMinusOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_json() {
    let pp = PlusMinusProduct::new().plus(0);
    let mut so = PlusMinusOperator::new();
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let serialized = serde_json::to_string(&so).unwrap();
    let deserialized: PlusMinusOperator = serde_json::from_str(&serialized).unwrap();
    assert_eq!(so, deserialized);
}

/// Test PlusMinusOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_readable() {
    let pp = PlusMinusProduct::new().plus(0);
    let mut system = PlusMinusOperator::new();
    system.set(pp, 0.5.into()).unwrap();
    assert_tokens(
        &system.readable(),
        &[
            Token::Struct {
                name: "PlusMinusOperatorSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 3 },
            Token::Str("0+"),
            Token::F64(0.5),
            Token::F64(0.0),
            Token::TupleEnd,
            Token::SeqEnd,
            Token::Str("serialisation_meta"),
            Token::Struct {
                name: "StruqtureSerialisationMeta",
                len: 3,
            },
            Token::Str("type_name"),
            Token::Str("PlusMinusOperator"),
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
    let pp = PlusMinusProduct::new().plus(0);
    let mut so = PlusMinusOperator::new();
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let encoded: Vec<u8> = bincode::serialize(&so).unwrap();
    let decoded: PlusMinusOperator = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(so, decoded);

    let encoded: Vec<u8> = bincode::serialize(&so.clone().compact()).unwrap();
    let decoded: PlusMinusOperator = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(so, decoded);
}

#[test]
fn serde_compact() {
    let pp = PlusMinusProduct::new().plus(0);
    let mut system = PlusMinusOperator::new();
    system.set(pp, 0.5.into()).unwrap();

    assert_tokens(
        &system.compact(),
        &[
            Token::Struct {
                name: "PlusMinusOperatorSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 3 },
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::U64(0),
            Token::UnitVariant {
                name: "SinglePlusMinusOperator",
                variant: "Plus",
            },
            Token::TupleEnd,
            Token::SeqEnd,
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(0.5),
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
            Token::Str("PlusMinusOperator"),
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
fn do_from_pmo() {
    let pmp_vec: Vec<(PlusMinusProduct, CalculatorComplex)> = vec![
        (PlusMinusProduct::new().z(0), 3.0.into()),
        (PlusMinusProduct::new(), 0.5.into()),
        (
            PlusMinusProduct::new().plus(1),
            CalculatorComplex::new(0.0, 1.0),
        ),
        (PlusMinusProduct::new().minus(2), 2.0.into()),
        (
            PlusMinusProduct::new().plus(0).minus(1).z(2),
            CalculatorComplex::new(1.0, 1.0),
        ),
    ];
    let dp_vec: Vec<(DecoherenceProduct, CalculatorComplex)> = vec![
        (DecoherenceProduct::new().z(0), 3.0.into()),
        (DecoherenceProduct::new(), 0.5.into()),
        (
            DecoherenceProduct::new().x(1),
            CalculatorComplex::new(0.0, 0.5),
        ),
        (
            DecoherenceProduct::new().iy(1),
            CalculatorComplex::new(0.0, 0.5),
        ),
        (
            DecoherenceProduct::new().x(2),
            CalculatorComplex::new(1.0, 0.0),
        ),
        (
            DecoherenceProduct::new().iy(2),
            CalculatorComplex::new(-1.0, 0.0),
        ),
        (
            DecoherenceProduct::new().x(0).x(1).z(2),
            CalculatorComplex::new(0.25, 0.25),
        ),
        (
            DecoherenceProduct::new().iy(0).x(1).z(2),
            CalculatorComplex::new(0.25, 0.25),
        ),
        (
            DecoherenceProduct::new().x(0).iy(1).z(2),
            CalculatorComplex::new(-0.25, -0.25),
        ),
        (
            DecoherenceProduct::new().iy(0).iy(1).z(2),
            CalculatorComplex::new(-0.25, -0.25),
        ),
    ];

    let mut decoh_op = DecoherenceOperator::new();
    for (key, val) in dp_vec.iter() {
        decoh_op
            .add_operator_product(key.clone(), val.clone())
            .unwrap();
    }

    let mut pm_op = PlusMinusOperator::new();
    for (key, val) in pmp_vec.iter() {
        pm_op
            .add_operator_product(key.clone(), val.clone())
            .unwrap();
    }

    assert_eq!(DecoherenceOperator::from(pm_op), decoh_op);
}

#[test]
fn pmo_from_do() {
    let dp_vec: Vec<(DecoherenceProduct, CalculatorComplex)> = vec![
        (DecoherenceProduct::new().z(0), 1.0.into()),
        (DecoherenceProduct::new(), 0.5.into()),
        (
            DecoherenceProduct::new().x(0),
            CalculatorComplex::new(0.0, 1.0),
        ),
        (DecoherenceProduct::new().iy(0), 2.0.into()),
        (
            DecoherenceProduct::new().x(0).iy(1).z(2),
            CalculatorComplex::new(1.0, 1.0),
        ),
    ];
    let pmp_vec: Vec<(PlusMinusProduct, CalculatorComplex)> = vec![
        (
            PlusMinusProduct::new().z(0),
            CalculatorComplex::new(1.0, 0.0),
        ),
        (PlusMinusProduct::new(), CalculatorComplex::new(0.5, 0.0)),
        (
            PlusMinusProduct::new().plus(0),
            CalculatorComplex::new(0.0, 1.0),
        ),
        (
            PlusMinusProduct::new().minus(0),
            CalculatorComplex::new(0.0, 1.0),
        ),
        (
            PlusMinusProduct::new().plus(0),
            CalculatorComplex::new(2.0, 0.0),
        ),
        (
            PlusMinusProduct::new().minus(0),
            CalculatorComplex::new(-2.0, 0.0),
        ),
        (
            PlusMinusProduct::new().plus(0).plus(1).z(2),
            CalculatorComplex::new(1.0, 1.0),
        ),
        (
            PlusMinusProduct::new().minus(0).plus(1).z(2),
            CalculatorComplex::new(1.0, 01.0),
        ),
        (
            PlusMinusProduct::new().plus(0).minus(1).z(2),
            CalculatorComplex::new(-1.0, -1.0),
        ),
        (
            PlusMinusProduct::new().minus(0).minus(1).z(2),
            CalculatorComplex::new(-1.0, -1.0),
        ),
    ];

    let mut decoh_op = DecoherenceOperator::new();
    for (key, val) in dp_vec.iter() {
        decoh_op
            .add_operator_product(key.clone(), val.clone())
            .unwrap();
    }

    let mut pm_op = PlusMinusOperator::new();
    for (key, val) in pmp_vec.iter() {
        pm_op
            .add_operator_product(key.clone(), val.clone())
            .unwrap();
    }

    assert_eq!(PlusMinusOperator::from(decoh_op), pm_op);
}

#[test]
fn sh_from_pmo() {
    let pmp_vec: Vec<(PlusMinusProduct, CalculatorComplex)> = vec![
        (PlusMinusProduct::new().z(0), 1.0.into()),
        (PlusMinusProduct::new(), 0.5.into()),
        (
            PlusMinusProduct::new().z(0).z(1).z(2),
            CalculatorComplex::new(1.5, 0.0),
        ),
    ];
    let pp_vec: Vec<(PauliProduct, CalculatorFloat)> = vec![
        (PauliProduct::new().z(0), 1.0.into()),
        (PauliProduct::new(), 0.5.into()),
        (PauliProduct::new().z(0).z(1).z(2), 1.5.into()),
    ];

    let mut qubit_ham = QubitHamiltonian::new();
    for (key, val) in pp_vec.iter() {
        qubit_ham
            .add_operator_product(key.clone(), val.clone())
            .unwrap();
    }

    let mut pm_op = PlusMinusOperator::new();
    for (key, val) in pmp_vec.iter() {
        pm_op
            .add_operator_product(key.clone(), val.clone())
            .unwrap();
    }

    assert_eq!(QubitHamiltonian::try_from(pm_op).unwrap(), qubit_ham);
}

#[test]
fn so_from_pmo() {
    let pmp_vec: Vec<(PlusMinusProduct, CalculatorComplex)> = vec![
        (PlusMinusProduct::new().z(0), 1.0.into()),
        (PlusMinusProduct::new(), 0.5.into()),
        (
            PlusMinusProduct::new().plus(0),
            CalculatorComplex::new(0.0, 1.0),
        ),
        (PlusMinusProduct::new().minus(0), 2.0.into()),
        (
            PlusMinusProduct::new().plus(0).minus(1).z(2),
            CalculatorComplex::new(1.0, 1.0),
        ),
    ];
    let pp_vec: Vec<(PauliProduct, CalculatorComplex)> = vec![
        (PauliProduct::new().z(0), CalculatorComplex::new(1.0, 0.0)),
        (PauliProduct::new(), CalculatorComplex::new(0.5, 0.0)),
        (PauliProduct::new().x(0), CalculatorComplex::new(0.0, 0.5)),
        (PauliProduct::new().y(0), CalculatorComplex::new(-0.5, 0.0)),
        (PauliProduct::new().x(0), CalculatorComplex::new(1.0, 0.0)),
        (PauliProduct::new().y(0), CalculatorComplex::new(0.0, -1.0)),
        (
            PauliProduct::new().x(0).x(1).z(2),
            CalculatorComplex::new(0.25, 0.25),
        ),
        (
            PauliProduct::new().y(0).x(1).z(2),
            CalculatorComplex::new(-0.25, 0.25),
        ),
        (
            PauliProduct::new().x(0).y(1).z(2),
            CalculatorComplex::new(0.25, -0.25),
        ),
        (
            PauliProduct::new().y(0).y(1).z(2),
            CalculatorComplex::new(0.25, 0.25),
        ),
    ];

    let mut qubit_op = QubitOperator::new();
    for (key, val) in pp_vec.iter() {
        qubit_op
            .add_operator_product(key.clone(), val.clone())
            .unwrap();
    }

    let mut pm_op = PlusMinusOperator::new();
    for (key, val) in pmp_vec.iter() {
        pm_op
            .add_operator_product(key.clone(), val.clone())
            .unwrap();
    }

    assert_eq!(QubitOperator::from(pm_op.clone()), qubit_op);
    assert!(QubitHamiltonian::try_from(pm_op).is_err());
}

#[test]
fn pmo_from_sh() {
    let pp_vec: Vec<(PauliProduct, CalculatorFloat)> = vec![
        (PauliProduct::new().z(0), 1.0.into()),
        (PauliProduct::new(), 0.5.into()),
        (PauliProduct::new().x(0), 1.0.into()),
        (PauliProduct::new().y(0), 2.0.into()),
        (PauliProduct::new().x(0).y(1).z(2), 1.0.into()),
    ];
    let pmp_vec: Vec<(PlusMinusProduct, CalculatorComplex)> = vec![
        (
            PlusMinusProduct::new().z(0),
            CalculatorComplex::new(1.0, 0.0),
        ),
        (PlusMinusProduct::new(), CalculatorComplex::new(0.5, 0.0)),
        (
            PlusMinusProduct::new().plus(0),
            CalculatorComplex::new(1.0, 0.0),
        ),
        (
            PlusMinusProduct::new().minus(0),
            CalculatorComplex::new(1.0, 0.0),
        ),
        (
            PlusMinusProduct::new().plus(0),
            CalculatorComplex::new(0.0, -2.0),
        ),
        (
            PlusMinusProduct::new().minus(0),
            CalculatorComplex::new(0.0, 2.0),
        ),
        (
            PlusMinusProduct::new().plus(0).plus(1).z(2),
            CalculatorComplex::new(0.0, -1.0),
        ),
        (
            PlusMinusProduct::new().minus(0).plus(1).z(2),
            CalculatorComplex::new(0.0, -1.0),
        ),
        (
            PlusMinusProduct::new().plus(0).minus(1).z(2),
            CalculatorComplex::new(0.0, 1.0),
        ),
        (
            PlusMinusProduct::new().minus(0).minus(1).z(2),
            CalculatorComplex::new(0.0, 1.0),
        ),
    ];

    let mut qubit_op = QubitHamiltonian::new();
    for (key, val) in pp_vec.iter() {
        qubit_op
            .add_operator_product(key.clone(), val.clone())
            .unwrap();
    }

    let mut pm_op = PlusMinusOperator::new();
    for (key, val) in pmp_vec.iter() {
        pm_op
            .add_operator_product(key.clone(), val.clone())
            .unwrap();
    }

    assert_eq!(PlusMinusOperator::from(qubit_op), pm_op);
}

#[test]
fn pmo_from_so() {
    let pp_vec: Vec<(PauliProduct, CalculatorComplex)> = vec![
        (PauliProduct::new().z(0), 1.0.into()),
        (PauliProduct::new(), 0.5.into()),
        (PauliProduct::new().x(0), CalculatorComplex::new(0.0, 1.0)),
        (PauliProduct::new().y(0), 2.0.into()),
        (
            PauliProduct::new().x(0).y(1).z(2),
            CalculatorComplex::new(1.0, 1.0),
        ),
    ];
    let pmp_vec: Vec<(PlusMinusProduct, CalculatorComplex)> = vec![
        (
            PlusMinusProduct::new().z(0),
            CalculatorComplex::new(1.0, 0.0),
        ),
        (PlusMinusProduct::new(), CalculatorComplex::new(0.5, 0.0)),
        (
            PlusMinusProduct::new().plus(0),
            CalculatorComplex::new(0.0, 1.0),
        ),
        (
            PlusMinusProduct::new().minus(0),
            CalculatorComplex::new(0.0, 1.0),
        ),
        (
            PlusMinusProduct::new().plus(0),
            CalculatorComplex::new(0.0, -2.0),
        ),
        (
            PlusMinusProduct::new().minus(0),
            CalculatorComplex::new(0.0, 2.0),
        ),
        (
            PlusMinusProduct::new().plus(0).plus(1).z(2),
            CalculatorComplex::new(1.0, -1.0),
        ),
        (
            PlusMinusProduct::new().minus(0).plus(1).z(2),
            CalculatorComplex::new(1.0, -1.0),
        ),
        (
            PlusMinusProduct::new().plus(0).minus(1).z(2),
            CalculatorComplex::new(-1.0, 1.0),
        ),
        (
            PlusMinusProduct::new().minus(0).minus(1).z(2),
            CalculatorComplex::new(-1.0, 1.0),
        ),
    ];

    let mut qubit_op = QubitOperator::new();
    for (key, val) in pp_vec.iter() {
        qubit_op
            .add_operator_product(key.clone(), val.clone())
            .unwrap();
    }

    let mut pm_op = PlusMinusOperator::new();
    for (key, val) in pmp_vec.iter() {
        pm_op
            .add_operator_product(key.clone(), val.clone())
            .unwrap();
    }

    assert_eq!(PlusMinusOperator::from(qubit_op), pm_op);
}

#[cfg(feature = "json_schema")]
#[test]
fn test_plus_minus_operator_schema() {
    let mut op = PlusMinusOperator::new();
    op.set(PlusMinusProduct::new().plus(0), 1.0.into()).unwrap();
    op.set(PlusMinusProduct::new().minus(1).z(2), "val".into())
        .unwrap();
    let schema = schemars::schema_for!(PlusMinusOperator);
    let schema_checker = jsonschema::JSONSchema::compile(&serde_json::to_value(&schema).unwrap())
        .expect("schema is valid");
    let value = serde_json::to_value(&op).unwrap();
    let validation = schema_checker.validate(&value);

    assert!(validation.is_ok());
}

#[cfg(feature = "struqture_1_import")]
#[cfg(feature = "struqture_1_export")]
#[test]
fn test_from_to_struqture_1() {
    let pp_1 = struqture_one::spins::PlusMinusProduct::from_str("0+1-25Z").unwrap();
    let mut ss_1 = struqture_one::spins::PlusMinusOperator::new();
    struqture_one::OperateOnDensityMatrix::set(&mut ss_1, pp_1.clone(), 1.0.into()).unwrap();

    let pp_2 = PlusMinusProduct::new().plus(0).minus(1).z(25);
    let mut ss_2 = PlusMinusOperator::new();
    ss_2.set(pp_2.clone(), 1.0.into()).unwrap();

    assert!(PlusMinusOperator::from_struqture_1(&ss_1).unwrap() == ss_2);
    assert!(ss_1 == ss_2.to_struqture_1().unwrap());
}
