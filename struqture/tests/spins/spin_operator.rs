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

//! Integration test for public API of SpinOperator

use super::create_na_matrix_from_operator_list;
use nalgebra as na;
use num_complex::Complex64;
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde_test::{assert_tokens, Configure, Token};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::iter::{FromIterator, IntoIterator};
use std::ops::{Add, Sub};
use std::str::FromStr;
use struqture::prelude::*;
use struqture::spins::{
    OperateOnSpins, PauliProduct, SpinHamiltonian, SpinOperator, ToSparseMatrixOperator,
};
use struqture::{CooSparseMatrix, OperateOnDensityMatrix, SpinIndex};
use test_case::test_case;

// Test the new function of the SpinOperator
#[test]
fn new() {
    let so = SpinOperator::new();
    assert!(so.is_empty());
    assert_eq!(SpinOperator::new(), SpinOperator::default())
}

#[test]
fn empty_clone_options() {
    let pp_2: PauliProduct = PauliProduct::new().z(2);
    let mut system = SpinOperator::new();
    system.set(pp_2, CalculatorComplex::from(0.5)).unwrap();

    let empty: Option<usize> = None;
    let full: Option<usize> = Some(3);
    assert_eq!(system.empty_clone(empty), SpinOperator::new());
    assert_eq!(system.empty_clone(full), SpinOperator::with_capacity(1));
}

// Test the number_spins function of the SpinOperator
#[test]
fn internal_map_number_spins() {
    let pp_0: PauliProduct = PauliProduct::new().x(0);
    let pp_2: PauliProduct = PauliProduct::new().z(2);
    let mut so = SpinOperator::new();
    assert_eq!(so.number_spins(), 0_usize);
    so.set(pp_0, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.number_spins(), 1_usize);
    so.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.number_spins(), 3_usize);
}

// Test the len function of the SpinOperator
#[test]
fn internal_map_len() {
    let pp_2: PauliProduct = PauliProduct::new().z(2);
    let mut so = SpinOperator::new();
    so.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.len(), 1_usize);
}

// Test the set, set_pauli_product, get functions of the SpinSystem
#[test]
fn internal_map_set_get_dict() {
    let mut system = SpinOperator::new();
    assert_eq!(system.number_spins(), 0_usize);
    let pp_0: PauliProduct = PauliProduct::new().z(0);

    // 1) Test try_set_pauli_product and get functions
    // Vacant
    system
        .set(pp_0.clone(), CalculatorComplex::from(0.0))
        .unwrap();
    system
        .set(pp_0.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(system.number_spins(), 1_usize);
    assert_eq!(system.get(&pp_0), &CalculatorComplex::from(0.5));

    // 2) Test iter, keys, values functions
    let mut map: BTreeMap<PauliProduct, CalculatorComplex> = BTreeMap::new();
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

// Test the set, get and remove functions of the SpinOperator
#[test]
fn internal_map_set_get_remove() {
    let pp_2: PauliProduct = PauliProduct::new().z(2);
    let mut so = SpinOperator::new();

    // 1) Test try_set_pauli_product and get functions
    // Vacant
    so.set(pp_2.clone(), CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.get(&pp_2.clone()), &CalculatorComplex::from(0.5));

    // 2) Test remove function
    so.remove(&pp_2);
    assert_eq!(so, SpinOperator::new());
}

// Test the add_operator_product function of the SpinOperator
#[test]
fn internal_map_add_operator_product() {
    let pp_2: PauliProduct = PauliProduct::new().z(2);
    let mut so = SpinOperator::new();

    so.add_operator_product(pp_2.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(so.get(&pp_2), &CalculatorComplex::from(0.5));
    so.add_operator_product(pp_2.clone(), CalculatorComplex::from(-0.5))
        .unwrap();
    assert_eq!(so.get(&pp_2), &CalculatorComplex::from(0.0));
}

// Test the iter, keys and values functions of the SpinOperator
#[test]
fn internal_map_keys() {
    let pp_2: PauliProduct = PauliProduct::new().z(2);
    let mut so = SpinOperator::new();
    so.set(pp_2.clone(), CalculatorComplex::from(0.5)).unwrap();

    let mut map: BTreeMap<PauliProduct, CalculatorComplex> = BTreeMap::new();
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

// Test the Iter traits of SpinOperator: into_iter, from_iter and extend
#[test]
fn into_iter_from_iter_extend() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let pp_1: PauliProduct = PauliProduct::new().x(1);
    let mut system = SpinOperator::new();
    system
        .add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();

    let system_iter = system.clone().into_iter();
    assert_eq!(SpinOperator::from_iter(system_iter), system);
    let system_iter = (&system)
        .into_iter()
        .map(|(key, value)| (key.clone(), value.clone()));
    assert_eq!(SpinOperator::from_iter(system_iter), system);
    let mut hamiltonian = SpinOperator::new();
    hamiltonian
        .add_operator_product(pp_0.clone(), 1.0.into())
        .unwrap();
    for (first, second) in system.into_iter().zip(hamiltonian.iter()) {
        assert_eq!(first.0, *second.0);
        assert_eq!(first.1, *second.1);
    }

    let mut system = SpinOperator::new();
    system
        .add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut mapping: BTreeMap<PauliProduct, CalculatorComplex> = BTreeMap::new();
    mapping.insert(pp_1.clone(), CalculatorComplex::from(0.5));
    let mapping_iter = mapping.into_iter();
    system.extend(mapping_iter);

    let mut system_1 = SpinOperator::new();
    system_1
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    system_1
        .add_operator_product(pp_1, CalculatorComplex::from(0.5))
        .unwrap();

    assert_eq!(system, system_1);
}

#[test]
fn from_operator_pass() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let mut so_0 = SpinHamiltonian::new();
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorFloat::from(1.0));
    let mut so_0_1 = SpinOperator::new();
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(1.0));

    assert_eq!(SpinOperator::from(so_0), so_0_1);
}

// Test the separation of terms
#[test_case(1)]
#[test_case(2)]
#[test_case(3)]
fn separate_out_terms(number_spins: usize) {
    let pp_1_a: PauliProduct = PauliProduct::new().z(0);
    let pp_1_b: PauliProduct = PauliProduct::new().x(1);
    let pp_2_a: PauliProduct = PauliProduct::new().z(0).x(2);
    let pp_2_b: PauliProduct = PauliProduct::new().x(1).y(2);
    let pp_3_a: PauliProduct = PauliProduct::new().z(0).z(1).z(2);
    let pp_3_b: PauliProduct = PauliProduct::new().x(1).x(2).z(0);

    let mut allowed: Vec<(PauliProduct, f64)> = Vec::new();
    let mut not_allowed: Vec<(PauliProduct, f64)> = vec![
        (pp_1_a.clone(), 1.0),
        (pp_1_b.clone(), 1.1),
        (pp_2_a.clone(), 1.2),
        (pp_2_b.clone(), 1.3),
        (pp_3_a.clone(), 1.4),
        (pp_3_b.clone(), 1.5),
    ];

    match number_spins {
        1 => {
            allowed.push((pp_1_a.clone(), 1.0));
            allowed.push((pp_1_b.clone(), 1.1));
            not_allowed.remove(0);
            not_allowed.remove(0);
        }
        2 => {
            allowed.push((pp_2_a.clone(), 1.2));
            allowed.push((pp_2_b.clone(), 1.3));
            not_allowed.remove(2);
            not_allowed.remove(2);
        }
        3 => {
            allowed.push((pp_3_a.clone(), 1.4));
            allowed.push((pp_3_b.clone(), 1.5));
            not_allowed.remove(4);
            not_allowed.remove(4);
        }
        _ => panic!(),
    }

    let mut separated = SpinOperator::new();
    for (key, value) in allowed.iter() {
        separated
            .add_operator_product(key.clone(), value.into())
            .unwrap();
    }
    let mut remainder = SpinOperator::new();
    for (key, value) in not_allowed.iter() {
        remainder
            .add_operator_product(key.clone(), value.into())
            .unwrap();
    }

    let mut so = SpinOperator::new();
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

// Test the negative operation: -SpinOperator
#[test]
fn negative_so() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let mut so_0 = SpinOperator::new();
    so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut so_0_minus = SpinOperator::new();
    so_0_minus
        .add_operator_product(pp_0, CalculatorComplex::from(-1.0))
        .unwrap();

    assert_eq!(-so_0, so_0_minus);
}

// Test the addition: SpinOperator + SpinOperator
#[test]
fn add_so_so() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let pp_1: PauliProduct = PauliProduct::new().x(1);
    let mut so_0 = SpinOperator::new();
    so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut so_1 = SpinOperator::new();
    so_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    let mut so_0_1 = SpinOperator::new();
    so_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    so_0_1
        .add_operator_product(pp_1, CalculatorComplex::from(0.5))
        .unwrap();

    assert_eq!(so_0.clone() + so_1.clone(), so_0_1);
    assert_eq!(so_0.add(so_1), so_0_1);
}

// Test the subtraction: SpinOperator - SpinOperator
#[test]
fn sub_so_so() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let pp_1: PauliProduct = PauliProduct::new().x(1);
    let mut so_0 = SpinOperator::new();
    so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut so_1 = SpinOperator::new();
    so_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    let mut so_0_1 = SpinOperator::new();
    so_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    so_0_1
        .add_operator_product(pp_1, CalculatorComplex::from(-0.5))
        .unwrap();

    assert_eq!(so_0.clone() - so_1.clone(), so_0_1);
    assert_eq!(so_0.sub(so_1), so_0_1);
}

// Test the multiplication: SpinOperator * SpinOperator with all possible pauli matrices
#[test_case("0X", "0X", "0I", CalculatorComplex::from(1.0); "x_x_identity")]
#[test_case("0X1X", "0X", "0I1X", CalculatorComplex::new(1.0, 0.0); "x_x")]
#[test_case("0X1X", "0Y", "0Z1X", CalculatorComplex::new(0.0, 1.0); "x_y")]
#[test_case("0X1X", "0Z", "0Y1X", CalculatorComplex::new(0.0, -1.0); "x_z")]
#[test_case("0Y1X", "0X", "0Z1X", CalculatorComplex::new(0.0, -1.0); "y_x")]
#[test_case("0Y1X", "0Y", "0I1X", CalculatorComplex::new(1.0, 0.0); "y_y")]
#[test_case("0Y1X", "0Z", "0X1X", CalculatorComplex::new(0.0, 1.0); "y_z")]
#[test_case("0Z1X", "0X", "0Y1X", CalculatorComplex::new(0.0, 1.0); "z_x")]
#[test_case("0Z1X", "0Y", "0X1X", CalculatorComplex::new(0.0, -1.0); "z_y")]
#[test_case("0Z1X", "0Z", "0I1X", CalculatorComplex::new(1.0, 0.0); "z_z")]
fn mul_so_so_all_paulis(pp0: &str, pp1: &str, pp01: &str, coeff: CalculatorComplex) {
    let pp_0: PauliProduct = PauliProduct::from_str(pp0).unwrap();
    let mut so_0 = SpinOperator::new();
    so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0))
        .unwrap();
    let pp_1: PauliProduct = PauliProduct::from_str(pp1).unwrap();
    let mut so_1 = SpinOperator::new();
    so_1.add_operator_product(pp_1, CalculatorComplex::from(0.5))
        .unwrap();
    let mut so_0_1 = SpinOperator::new();
    let pp_0_1: PauliProduct = PauliProduct::from_str(pp01).unwrap();
    so_0_1.add_operator_product(pp_0_1, coeff).unwrap();

    assert_eq!(so_0 * so_1, so_0_1);
}

// Test the multiplication: SpinOperator * SpinOperator
#[test]
fn mul_so_so() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let pp_1: PauliProduct = PauliProduct::new().x(1);
    let pp_0_1: PauliProduct = PauliProduct::new().z(0).x(1);
    let mut so_0 = SpinOperator::new();
    so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0))
        .unwrap();
    let mut so_1 = SpinOperator::new();
    so_1.add_operator_product(pp_1, CalculatorComplex::from(0.5))
        .unwrap();
    let mut so_0_1 = SpinOperator::new();
    so_0_1
        .add_operator_product(pp_0_1, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(so_0 * so_1, so_0_1);
}

// Test the multiplication: SpinOperator * SpinOperator where they have a PauliProduct with the same index
#[test]
fn mul_so_so_same_index() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let pp_1: PauliProduct = PauliProduct::new().x(0);
    let pp_0_1: PauliProduct = PauliProduct::new().y(0);
    let mut so_0 = SpinOperator::new();
    so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0))
        .unwrap();
    let mut so_1 = SpinOperator::new();
    so_1.add_operator_product(pp_1, CalculatorComplex::from(0.5))
        .unwrap();
    let mut so_0_1 = SpinOperator::new();
    so_0_1
        .add_operator_product(pp_0_1, CalculatorComplex::new(0.0, 1.0))
        .unwrap();

    assert_eq!(so_0 * so_1, so_0_1);
}

// Test the multiplication: SpinOperator * Calculatorcomplex
#[test]
fn mul_so_cc() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let mut so_0 = SpinOperator::new();
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(2.0));
    let mut so_0_1 = SpinOperator::new();
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(6.0));

    assert_eq!(so_0 * CalculatorComplex::from(3.0), so_0_1);
}

// Test the Debug trait of SpinOperator
#[test]
fn debug() {
    let pp: PauliProduct = PauliProduct::new().z(0);
    let mut so = SpinOperator::new();
    let _ = so.set(pp, CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{:?}", so),
        "SpinOperator { internal_map: {PauliProduct { items: [(0, Z)] }: CalculatorComplex { re: Float(0.5), im: Float(0.0) }} }"
    );
}

// Test the Display trait of SpinOperator
#[test]
fn display() {
    let mut so = SpinOperator::new();
    let pp: PauliProduct = PauliProduct::new().z(0);
    let _ = so.set(pp, CalculatorComplex::from(0.5));

    assert_eq!(format!("{}", so), "SpinOperator{\n0Z: (5e-1 + i * 0e0),\n}");
}

// Test the hermitian_conjugate and is_natural_hermitian functions of the HermitianMixedProduct
#[test]
fn hermitian_test() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let mut system = SpinOperator::new();
    system
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(system.hermitian_conjugate(), system.clone());
}

#[test]
fn matrices() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let mut system = SpinOperator::new();
    system
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(
        system.sparse_lindblad_entries().unwrap()[0].0,
        (vec![], (vec![], vec![]))
    );
    assert_eq!(
        system.sparse_lindblad_entries().unwrap()[0].1,
        (vec![], (vec![], vec![]))
    );
    assert_eq!(
        system.sparse_lindblad_entries().unwrap()[0].2,
        Complex64::default()
    );

    let unitary_matrix: CooSparseMatrix =
        (vec![1.0.into(), (-1.0).into()], (vec![0, 1], vec![0, 1]));
    assert_eq!(system.unitary_sparse_matrix_coo().unwrap(), unitary_matrix);

    let mut superoperator_matrix: HashMap<usize, HashMap<usize, Complex64>> = HashMap::new();
    let mut row_0: HashMap<usize, Complex64> = HashMap::new();
    row_0.insert(0, 1.0.into());
    let mut row_1: HashMap<usize, Complex64> = HashMap::new();
    row_1.insert(1, (-1.0).into());
    let mut row_2: HashMap<usize, Complex64> = HashMap::new();
    row_2.insert(2, (-1.0).into());
    let mut row_3: HashMap<usize, Complex64> = HashMap::new();
    row_3.insert(3, 1.0.into());
    superoperator_matrix.insert(0, row_0);
    superoperator_matrix.insert(1, row_1);
    superoperator_matrix.insert(2, row_2);
    superoperator_matrix.insert(3, row_3);
}

// Test the Clone and PartialEq traits of SpinOperator
#[test]
fn clone_partial_eq() {
    let pp: PauliProduct = PauliProduct::new().z(0);
    let mut so = SpinOperator::new();
    so.set(pp, CalculatorComplex::from(0.5)).unwrap();

    // Test Clone trait
    assert_eq!(so.clone(), so);

    // Test PartialEq trait
    let pp_1: PauliProduct = PauliProduct::new().z(0);
    let mut so_1 = SpinOperator::new();
    so_1.set(pp_1, CalculatorComplex::from(0.5)).unwrap();
    let pp_2: PauliProduct = PauliProduct::new().z(2);
    let mut so_2 = SpinOperator::new();
    so_2.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert!(so_1 == so);
    assert!(so == so_1);
    assert!(so_2 != so);
    assert!(so != so_2);
}

/// Test SpinOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_json() {
    let pp = PauliProduct::new().x(0);
    let mut so = SpinOperator::new();
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let serialized = serde_json::to_string(&so).unwrap();
    let deserialized: SpinOperator = serde_json::from_str(&serialized).unwrap();
    assert_eq!(so, deserialized);
}

/// Test SpinOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_readable() {
    let pp = PauliProduct::new().x(0);
    let mut system = SpinOperator::new();
    system.set(pp, 0.5.into()).unwrap();
    assert_tokens(
        &system.readable(),
        &[
            Token::Struct {
                name: "SpinOperatorSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 3 },
            Token::Str("0X"),
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
            Token::Str("SpinOperator"),
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
    let pp = PauliProduct::new().x(0);
    let mut so = SpinOperator::new();
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let encoded: Vec<u8> = bincode::serialize(&so).unwrap();
    let decoded: SpinOperator = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(so, decoded);

    let encoded: Vec<u8> = bincode::serialize(&so.clone().compact()).unwrap();
    let decoded: SpinOperator = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(so, decoded);
}

#[test]
fn serde_compact() {
    let pp = PauliProduct::new().x(0);
    let mut system = SpinOperator::new();
    system.set(pp, 0.5.into()).unwrap();

    assert_tokens(
        &system.compact(),
        &[
            Token::Struct {
                name: "SpinOperatorSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 3 },
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::U64(0),
            Token::UnitVariant {
                name: "SingleSpinOperator",
                variant: "X",
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
            Token::Str("SpinOperator"),
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

#[test_case("0Z", &["Z"]; "0Z")]
#[test_case("1X", &["X", "I"]; "1X")]
#[test_case("1Y", &["Y", "I"]; "1Y")]
#[test_case("0Z1X", &["X", "Z"]; "0Z1X")]
#[test_case("0X1X", &["X", "X"]; "0X1X")]
#[test_case("0X1Y", &["Y", "X"]; "0X1Y")]
#[test_case("0X2Y", &["Y", "I","X"]; "0X2Y")]
fn test_superoperator(pauli_representation: &str, pauli_operators: &[&str]) {
    let mut system = SpinOperator::new();
    let pp: PauliProduct = PauliProduct::from_str(pauli_representation).unwrap();

    system.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let dimension = 4_usize.pow(pauli_operators.len() as u32);

    // Constructing matrix by hand:
    let cc0 = Complex64::new(0.0, 0.0);
    let cci = Complex64::new(0.0, 1.0);

    let identities: Vec<&str> = (0..pauli_operators.len()).map(|_| "I").collect();

    let i = create_na_matrix_from_operator_list(&identities);
    let h = create_na_matrix_from_operator_list(pauli_operators);

    let test_matrix = (h.kronecker(&i) - i.kronecker(&h.transpose())) * (-cci);

    let second_test_matrix = system.sparse_matrix_superoperator(None).unwrap();
    for row in 0..dimension {
        for column in 0..dimension {
            let key = (row, column);
            let val = test_matrix[(row, column)];
            let second_val = second_test_matrix.get(&key);

            match second_val {
                Some(x) => assert_eq!(&val, x),
                None => {
                    assert_eq!(val, cc0)
                }
            }
        }
    }

    let (test_vals, (test_rows, test_columns)) =
        system.sparse_matrix_superoperator_coo(None).unwrap();
    for (second_val, (row, column)) in test_vals
        .iter()
        .zip(test_rows.iter().zip(test_columns.iter()))
    {
        let val = test_matrix[(*row, *column)];
        assert_eq!(&val, second_val);
    }
}

#[test_case("0Z", &["Z"]; "0Z")]
#[test_case("1X", &["X", "I"]; "1X")]
#[test_case("1Y", &["Y", "I"]; "1Y")]
#[test_case("0Z1X", &["X", "Z"]; "0Z1X")]
#[test_case("0X1X", &["X", "X"]; "0X1X")]
#[test_case("0X1Y", &["Y", "X"]; "0X1Y")]
#[test_case("0X2Y", &["Y", "I","X"]; "0X2Y")]
fn test_operator(pauli_representation: &str, pauli_operators: &[&str]) {
    let mut system = SpinOperator::new();
    let pp: PauliProduct = PauliProduct::from_str(pauli_representation).unwrap();

    system.set(pp, CalculatorComplex::from(1.0)).unwrap();
    let dimension = 2_usize.pow(pauli_operators.len() as u32);

    // Constructing matrix by hand:
    let cc0 = Complex64::new(0.0, 0.0);

    let h = create_na_matrix_from_operator_list(pauli_operators);

    let test_matrix = h;

    let second_test_matrix = system.sparse_matrix(None).unwrap();

    #[allow(unused)]
    fn fast_convert(
        map: HashMap<(usize, usize), Complex64>,
        dimension: usize,
    ) -> na::DMatrix<Complex64> {
        let mut mat = na::DMatrix::<Complex64>::zeros(dimension, dimension);
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
                    assert_eq!(val, cc0)
                }
            }
        }
    }

    let coo_test_matrix = system.unitary_sparse_matrix_coo().unwrap();
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
            let second_val = coo_hashmap.get(&key);

            match second_val {
                Some(x) => assert_eq!(&val, x),
                None => {
                    assert_eq!(val, cc0)
                }
            }
        }
    }
}

#[test]
fn sparse_lindblad_entries() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let pp_1: PauliProduct = PauliProduct::new().x(1);
    let mut system = SpinOperator::new();
    system
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    system
        .add_operator_product(pp_1, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(
        system.sparse_lindblad_entries().unwrap()[0].0,
        (vec![], (vec![], vec![]))
    );
    assert_eq!(
        system.sparse_lindblad_entries().unwrap()[0].1,
        (vec![], (vec![], vec![]))
    );
    assert_eq!(
        system.sparse_lindblad_entries().unwrap()[0].2,
        Complex64::default()
    );
}

#[test]
fn test_spin_operator_pauli_multiplication() {
    // We use this a bunch
    let one = CalculatorComplex::from(1.0);
    // This is to test the algebra of
    // SpinOperator * PauliProduct and
    // PauliProduct * SpinOperator
    let pauli_1 = PauliProduct::from_str("0X").unwrap();
    let pauli_2 = PauliProduct::from_str("0Y").unwrap();
    let pauli_3 = PauliProduct::from_str("0Z").unwrap();
    let pauli_4 = PauliProduct::from_str("2Y").unwrap();
    let pauli_5 = PauliProduct::from_str("0X2Y").unwrap();

    let mut spin_op_1 = SpinOperator::new();
    let mut spin_op_2 = SpinOperator::new();
    let mut spin_op_3 = SpinOperator::new();
    let mut spin_op_4 = SpinOperator::new();
    let mut spin_op_5 = SpinOperator::new();

    spin_op_1.set(pauli_1.clone(), one.clone()).unwrap();
    spin_op_2.set(pauli_2.clone(), one.clone()).unwrap();
    spin_op_3.set(pauli_3.clone(), one.clone()).unwrap();
    spin_op_4.set(pauli_4, one.clone()).unwrap();
    spin_op_5.set(pauli_5, one).unwrap();

    // 0X2Y * 0X = 2Y,
    let prod_1a = spin_op_5.clone() * pauli_1.clone();
    //  0X * 0X2Y = 2Y
    let prod_1b = pauli_1 * spin_op_5.clone();

    // Products match 2Y = 2Y
    assert_eq!(prod_1a, prod_1b);
    // Product 2Y matches predefined SpinOp = 2Y
    assert_eq!(prod_1a, spin_op_4);

    // 0Z * 0Y = -i * 0X
    let prod_2a = spin_op_3.clone() * pauli_2.clone();
    // 0Y * 0Z = i * 0X
    let prod_2b = pauli_2.clone() * spin_op_3.clone();

    // 0Z * 0Y = -0Y * 0Z, -i * 0X = -(i * 0X)
    assert_eq!(prod_2a, -prod_2b);
    assert_eq!(prod_2a, spin_op_1 * CalculatorComplex::from((0.0, -1.0)));

    // 0X2Y * 0Y = i * 0Z2Y, i * 0Z2Y * 0Z = i * 2Y
    let prod_3 = (spin_op_5 * pauli_2) * pauli_3;
    assert_eq!(prod_3, spin_op_4 * CalculatorComplex::from((0.0, 1.0)));
}

#[cfg(feature = "json_schema")]
#[test]
fn test_spin_operator_schema() {
    let mut op = SpinOperator::new();
    op.set(PauliProduct::new().x(0), 1.0.into()).unwrap();
    op.set(PauliProduct::new().y(1).z(2), "val".into()).unwrap();
    let schema = schemars::schema_for!(SpinOperator);
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
    let pp_1 = struqture_one::spins::PauliProduct::from_str("0X1Y25Z").unwrap();
    let mut ss_1 = struqture_one::spins::SpinSystem::new(None);
    struqture_one::OperateOnDensityMatrix::set(&mut ss_1, pp_1.clone(), 1.0.into()).unwrap();

    let pp_2 = PauliProduct::new().x(0).y(1).z(25);
    let mut ss_2 = SpinOperator::new();
    ss_2.set(pp_2.clone(), 1.0.into()).unwrap();

    assert!(SpinOperator::from_struqture_1(&ss_1).unwrap() == ss_2);
    assert!(ss_1 == ss_2.to_struqture_1().unwrap());
}
