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

//! Integration test for public API of QubitOperator

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
    OperateOnSpins, PauliProduct, QubitHamiltonian, QubitOperator, ToSparseMatrixOperator,
};
use struqture::{CooSparseMatrix, OperateOnDensityMatrix, SpinIndex};
use test_case::test_case;

// Test the new function of the QubitOperator
#[test]
fn new() {
    let so = QubitOperator::new();
    assert!(so.is_empty());
    assert_eq!(QubitOperator::new(), QubitOperator::default())
}

#[test]
fn empty_clone_options() {
    let pp_2: PauliProduct = PauliProduct::new().z(2);
    let mut system = QubitOperator::new();
    system.set(pp_2, CalculatorComplex::from(0.5)).unwrap();

    let empty: Option<usize> = None;
    let full: Option<usize> = Some(3);
    assert_eq!(system.empty_clone(empty), QubitOperator::new());
    assert_eq!(system.empty_clone(full), QubitOperator::with_capacity(1));
}

// Test the number_spins function of the QubitOperator
#[test]
fn internal_map_number_spins() {
    let pp_0: PauliProduct = PauliProduct::new().x(0);
    let pp_2: PauliProduct = PauliProduct::new().z(2);
    let mut so = QubitOperator::new();
    assert_eq!(so.number_spins(), 0_usize);
    so.set(pp_0, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.number_spins(), 1_usize);
    so.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.number_spins(), 3_usize);
}

// Test the len function of the QubitOperator
#[test]
fn internal_map_len() {
    let pp_2: PauliProduct = PauliProduct::new().z(2);
    let mut so = QubitOperator::new();
    so.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.len(), 1_usize);
}

// Test the set, set_pauli_product, get functions of the SpinSystem
#[test]
fn internal_map_set_get_dict() {
    let mut system = QubitOperator::new();
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

// Test the set, get and remove functions of the QubitOperator
#[test]
fn internal_map_set_get_remove() {
    let pp_2: PauliProduct = PauliProduct::new().z(2);
    let mut so = QubitOperator::new();

    // 1) Test try_set_pauli_product and get functions
    // Vacant
    so.set(pp_2.clone(), CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.get(&pp_2.clone()), &CalculatorComplex::from(0.5));

    // 2) Test remove function
    so.remove(&pp_2);
    assert_eq!(so, QubitOperator::new());
}

// Test the add_operator_product function of the QubitOperator
#[test]
fn internal_map_add_operator_product() {
    let pp_2: PauliProduct = PauliProduct::new().z(2);
    let mut so = QubitOperator::new();

    so.add_operator_product(pp_2.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(so.get(&pp_2), &CalculatorComplex::from(0.5));
    so.add_operator_product(pp_2.clone(), CalculatorComplex::from(-0.5))
        .unwrap();
    assert_eq!(so.get(&pp_2), &CalculatorComplex::from(0.0));
}

// Test the iter, keys and values functions of the QubitOperator
#[test]
fn internal_map_keys() {
    let pp_2: PauliProduct = PauliProduct::new().z(2);
    let mut so = QubitOperator::new();
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

// Test the Iter traits of QubitOperator: into_iter, from_iter and extend
#[test]
fn into_iter_from_iter_extend() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let pp_1: PauliProduct = PauliProduct::new().x(1);
    let mut system = QubitOperator::new();
    system
        .add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();

    let system_iter = system.clone().into_iter();
    assert_eq!(QubitOperator::from_iter(system_iter), system);
    let system_iter = (&system)
        .into_iter()
        .map(|(key, value)| (key.clone(), value.clone()));
    assert_eq!(QubitOperator::from_iter(system_iter), system);
    let mut hamiltonian = QubitOperator::new();
    hamiltonian
        .add_operator_product(pp_0.clone(), 1.0.into())
        .unwrap();
    for (first, second) in system.into_iter().zip(hamiltonian.iter()) {
        assert_eq!(first.0, *second.0);
        assert_eq!(first.1, *second.1);
    }

    let mut system = QubitOperator::new();
    system
        .add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut mapping: BTreeMap<PauliProduct, CalculatorComplex> = BTreeMap::new();
    mapping.insert(pp_1.clone(), CalculatorComplex::from(0.5));
    let mapping_iter = mapping.into_iter();
    system.extend(mapping_iter);

    let mut system_1 = QubitOperator::new();
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
    let mut so_0 = QubitHamiltonian::new();
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorFloat::from(1.0));
    let mut so_0_1 = QubitOperator::new();
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(1.0));

    assert_eq!(QubitOperator::from(so_0), so_0_1);
}

// Test the negative operation: -QubitOperator
#[test]
fn negative_so() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let mut so_0 = QubitOperator::new();
    so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut so_0_minus = QubitOperator::new();
    so_0_minus
        .add_operator_product(pp_0, CalculatorComplex::from(-1.0))
        .unwrap();

    assert_eq!(-so_0, so_0_minus);
}

// Test the addition: QubitOperator + QubitOperator
#[test]
fn add_so_so() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let pp_1: PauliProduct = PauliProduct::new().x(1);
    let mut so_0 = QubitOperator::new();
    so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut so_1 = QubitOperator::new();
    so_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    let mut so_0_1 = QubitOperator::new();
    so_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    so_0_1
        .add_operator_product(pp_1, CalculatorComplex::from(0.5))
        .unwrap();

    assert_eq!(so_0.clone() + so_1.clone(), so_0_1);
    assert_eq!(so_0.add(so_1), so_0_1);
}

// Test the subtraction: QubitOperator - QubitOperator
#[test]
fn sub_so_so() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let pp_1: PauliProduct = PauliProduct::new().x(1);
    let mut so_0 = QubitOperator::new();
    so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut so_1 = QubitOperator::new();
    so_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    let mut so_0_1 = QubitOperator::new();
    so_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    so_0_1
        .add_operator_product(pp_1, CalculatorComplex::from(-0.5))
        .unwrap();

    assert_eq!(so_0.clone() - so_1.clone(), so_0_1);
    assert_eq!(so_0.sub(so_1), so_0_1);
}

// Test the multiplication: QubitOperator * QubitOperator with all possible pauli matrices
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
    let mut so_0 = QubitOperator::new();
    so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0))
        .unwrap();
    let pp_1: PauliProduct = PauliProduct::from_str(pp1).unwrap();
    let mut so_1 = QubitOperator::new();
    so_1.add_operator_product(pp_1, CalculatorComplex::from(0.5))
        .unwrap();
    let mut so_0_1 = QubitOperator::new();
    let pp_0_1: PauliProduct = PauliProduct::from_str(pp01).unwrap();
    so_0_1.add_operator_product(pp_0_1, coeff).unwrap();

    assert_eq!(so_0 * so_1, so_0_1);
}

// Test the multiplication: QubitOperator * QubitOperator
#[test]
fn mul_so_so() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let pp_1: PauliProduct = PauliProduct::new().x(1);
    let pp_0_1: PauliProduct = PauliProduct::new().z(0).x(1);
    let mut so_0 = QubitOperator::new();
    so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0))
        .unwrap();
    let mut so_1 = QubitOperator::new();
    so_1.add_operator_product(pp_1, CalculatorComplex::from(0.5))
        .unwrap();
    let mut so_0_1 = QubitOperator::new();
    so_0_1
        .add_operator_product(pp_0_1, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(so_0 * so_1, so_0_1);
}

// Test the multiplication: QubitOperator * QubitOperator where they have a PauliProduct with the same index
#[test]
fn mul_so_so_same_index() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let pp_1: PauliProduct = PauliProduct::new().x(0);
    let pp_0_1: PauliProduct = PauliProduct::new().y(0);
    let mut so_0 = QubitOperator::new();
    so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0))
        .unwrap();
    let mut so_1 = QubitOperator::new();
    so_1.add_operator_product(pp_1, CalculatorComplex::from(0.5))
        .unwrap();
    let mut so_0_1 = QubitOperator::new();
    so_0_1
        .add_operator_product(pp_0_1, CalculatorComplex::new(0.0, 1.0))
        .unwrap();

    assert_eq!(so_0 * so_1, so_0_1);
}

// Test the multiplication: QubitOperator * Calculatorcomplex
#[test]
fn mul_so_cc() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let mut so_0 = QubitOperator::new();
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(2.0));
    let mut so_0_1 = QubitOperator::new();
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(6.0));

    assert_eq!(so_0 * CalculatorComplex::from(3.0), so_0_1);
}

// Test the Debug trait of QubitOperator
#[test]
fn debug() {
    let pp: PauliProduct = PauliProduct::new().z(0);
    let mut so = QubitOperator::new();
    let _ = so.set(pp, CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{:?}", so),
        "QubitOperator { internal_map: {PauliProduct { items: [(0, Z)] }: CalculatorComplex { re: Float(0.5), im: Float(0.0) }} }"
    );
}

// Test the Display trait of QubitOperator
#[test]
fn display() {
    let mut so = QubitOperator::new();
    let pp: PauliProduct = PauliProduct::new().z(0);
    let _ = so.set(pp, CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{}", so),
        "QubitOperator{\n0Z: (5e-1 + i * 0e0),\n}"
    );
}

// Test the hermitian_conjugate and is_natural_hermitian functions of the HermitianMixedProduct
#[test]
fn hermitian_test() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let mut system = QubitOperator::new();
    system
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(system.hermitian_conjugate(), system.clone());
}

#[test]
fn matrices() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let mut system = QubitOperator::new();
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

// Test the Clone and PartialEq traits of QubitOperator
#[test]
fn clone_partial_eq() {
    let pp: PauliProduct = PauliProduct::new().z(0);
    let mut so = QubitOperator::new();
    so.set(pp, CalculatorComplex::from(0.5)).unwrap();

    // Test Clone trait
    assert_eq!(so.clone(), so);

    // Test PartialEq trait
    let pp_1: PauliProduct = PauliProduct::new().z(0);
    let mut so_1 = QubitOperator::new();
    so_1.set(pp_1, CalculatorComplex::from(0.5)).unwrap();
    let pp_2: PauliProduct = PauliProduct::new().z(2);
    let mut so_2 = QubitOperator::new();
    so_2.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert!(so_1 == so);
    assert!(so == so_1);
    assert!(so_2 != so);
    assert!(so != so_2);
}

/// Test QubitOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_json() {
    let pp = PauliProduct::new().x(0);
    let mut so = QubitOperator::new();
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let serialized = serde_json::to_string(&so).unwrap();
    let deserialized: QubitOperator = serde_json::from_str(&serialized).unwrap();
    assert_eq!(so, deserialized);
}

/// Test QubitOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_readable() {
    let pp = PauliProduct::new().x(0);
    let mut system = QubitOperator::new();
    system.set(pp, 0.5.into()).unwrap();
    assert_tokens(
        &system.readable(),
        &[
            Token::Struct {
                name: "QubitOperatorSerialize",
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
            Token::Str("QubitOperator"),
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
    let mut so = QubitOperator::new();
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let encoded: Vec<u8> = bincode::serialize(&so).unwrap();
    let decoded: QubitOperator = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(so, decoded);

    let encoded: Vec<u8> = bincode::serialize(&so.clone().compact()).unwrap();
    let decoded: QubitOperator = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(so, decoded);
}

#[test]
fn serde_compact() {
    let pp = PauliProduct::new().x(0);
    let mut system = QubitOperator::new();
    system.set(pp, 0.5.into()).unwrap();

    assert_tokens(
        &system.compact(),
        &[
            Token::Struct {
                name: "QubitOperatorSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 3 },
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::U64(0),
            Token::UnitVariant {
                name: "SingleQubitOperator",
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
            Token::Str("QubitOperator"),
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
    let mut system = QubitOperator::new();
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
    let mut system = QubitOperator::new();
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
    let mut system = QubitOperator::new();
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
    // QubitOperator * PauliProduct and
    // PauliProduct * QubitOperator
    let pauli_1 = PauliProduct::from_str("0X").unwrap();
    let pauli_2 = PauliProduct::from_str("0Y").unwrap();
    let pauli_3 = PauliProduct::from_str("0Z").unwrap();
    let pauli_4 = PauliProduct::from_str("2Y").unwrap();
    let pauli_5 = PauliProduct::from_str("0X2Y").unwrap();

    let mut spin_op_1 = QubitOperator::new();
    let mut spin_op_2 = QubitOperator::new();
    let mut spin_op_3 = QubitOperator::new();
    let mut spin_op_4 = QubitOperator::new();
    let mut spin_op_5 = QubitOperator::new();

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
    let mut op = QubitOperator::new();
    op.set(PauliProduct::new().x(0), 1.0.into()).unwrap();
    op.set(PauliProduct::new().y(1).z(2), "val".into()).unwrap();
    let schema = schemars::schema_for!(QubitOperator);
    let schema_checker = jsonschema::JSONSchema::compile(&serde_json::to_value(&schema).unwrap())
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
    let mut ss_2 = QubitOperator::new();
    ss_2.set(pp_2.clone(), 1.0.into()).unwrap();

    assert!(QubitOperator::from_struqture_1(&ss_1).unwrap() == ss_2);
    assert!(ss_1 == ss_2.to_struqture_1().unwrap());
}
