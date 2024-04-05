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

//! Integration test for public API of PauliProduct

use ndarray::{array, Array2};
use num_complex::Complex64;
use qoqo_calculator::CalculatorComplex;
use serde_test::{assert_tokens, Configure, Token};
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};
use std::iter::{FromIterator, IntoIterator};
use std::str::FromStr;
use struqture::spins::{PauliProduct, SingleQubitOperator};
use struqture::{CorrespondsTo, GetValue, SpinIndex, StruqtureError, SymmetricIndex};
use test_case::test_case;

// Test the new function of the PauliProduct
#[test]
fn new() {
    let mut pp = PauliProduct::new();
    pp = pp.set_pauli(0, SingleQubitOperator::X);

    let mut pp_compare = PauliProduct::new();
    assert!(pp_compare.is_empty());
    assert!(pp_compare.is_natural_hermitian());
    assert_eq!(pp_compare.current_number_spins(), 0_usize);
    pp_compare = pp_compare.set_pauli(0, SingleQubitOperator::X);

    assert_eq!(pp, pp_compare);
    assert_eq!(pp_compare.corresponds_to(), pp_compare);
    assert_eq!(PauliProduct::new(), PauliProduct::default())
}

// Test the set_pauli and get functions of the PauliProduct
#[test]
fn internal_map_set_get() {
    let mut pp = PauliProduct::new();
    pp = pp.set_pauli(0, SingleQubitOperator::X);
    pp = pp.set_pauli(2, SingleQubitOperator::Y);
    pp = pp.set_pauli(3, SingleQubitOperator::Z);

    assert_eq!(pp.get(&0).unwrap(), &SingleQubitOperator::X);
    assert_eq!(pp.get(&2).unwrap(), &SingleQubitOperator::Y);
    assert_eq!(pp.get(&3).unwrap(), &SingleQubitOperator::Z);
    assert_eq!(pp.get(&1), None);

    let pp = pp.set_pauli(3, SingleQubitOperator::X);
    let pp = pp.set_pauli(1, SingleQubitOperator::Identity);
    assert_eq!(pp.get(&1), None);
    assert_eq!(pp.get(&3).unwrap(), &SingleQubitOperator::X);
    let pp = pp.set_pauli(3, SingleQubitOperator::Identity);
    assert_eq!(pp.get(&3), None);
    let pp = pp.set_pauli(3, SingleQubitOperator::X);

    assert_eq!(pp.current_number_spins(), 4_usize);
    assert_eq!(pp.len(), 3_usize);

    let mut internal: BTreeMap<usize, SingleQubitOperator> = BTreeMap::new();
    internal.insert(0, SingleQubitOperator::X);
    internal.insert(2, SingleQubitOperator::Y);
    internal.insert(3, SingleQubitOperator::X);
    assert!(pp.iter().map(|(k, _)| k).all(|k| internal.contains_key(k)));
}

// Test the set_pauli and get functions of the PauliProduct
#[test]
fn hermitian_conjugate() {
    let mut pp = PauliProduct::new();
    pp = pp.set_pauli(0, SingleQubitOperator::X);
    pp = pp.set_pauli(2, SingleQubitOperator::Y);
    pp = pp.set_pauli(3, SingleQubitOperator::Z);

    assert_eq!(pp.hermitian_conjugate(), (pp, 1.0));

    let mut pp = PauliProduct::new();
    pp = pp.set_pauli(0, SingleQubitOperator::X);
    pp = pp.set_pauli(3, SingleQubitOperator::Z);

    assert_eq!(pp.hermitian_conjugate(), (pp, 1.0));

    let mut pp = PauliProduct::new();
    pp = pp.set_pauli(0, SingleQubitOperator::X);
    pp = pp.set_pauli(3, SingleQubitOperator::Z);
    pp = pp.set_pauli(2, SingleQubitOperator::Y);
    pp = pp.set_pauli(4, SingleQubitOperator::Y);

    assert_eq!(pp.hermitian_conjugate(), (pp, 1.0));
}

// Test the x_y_z and get functions of the PauliProduct
#[test]
fn x_y_z() {
    let pp = PauliProduct::new();
    assert_eq!(
        pp.clone().x(0),
        pp.clone().set_pauli(0, SingleQubitOperator::X)
    );
    assert_eq!(
        pp.clone().y(2),
        pp.clone().set_pauli(2, SingleQubitOperator::Y)
    );
    assert_eq!(pp.clone().z(3), pp.set_pauli(3, SingleQubitOperator::Z));
}

// Test the concatenate function of the PauliProduct
#[test]
fn concatenate() {
    // Without error
    let mut pp_0 = PauliProduct::new();
    pp_0 = pp_0.set_pauli(0, SingleQubitOperator::X);
    let mut pp_1 = PauliProduct::new();
    pp_1 = pp_1.set_pauli(1, SingleQubitOperator::Z);

    let mut pp_both = PauliProduct::new();
    pp_both = pp_both.set_pauli(0, SingleQubitOperator::X);
    pp_both = pp_both.set_pauli(1, SingleQubitOperator::Z);

    assert_eq!(pp_0.concatenate(pp_1).unwrap(), pp_both);

    // With error
    let mut pp_0 = PauliProduct::new();
    pp_0 = pp_0.set_pauli(0, SingleQubitOperator::X);
    let mut pp_1 = PauliProduct::new();
    pp_1 = pp_1.set_pauli(0, SingleQubitOperator::Z);

    let error = pp_0.concatenate(pp_1);
    assert!(error.is_err());
    assert_eq!(
        error,
        Err(StruqtureError::ProductIndexAlreadyOccupied { index: 0 })
    );
}

// Test the remap_qubits function of the PauliProduct
#[test]
fn remap_qubits() {
    let mut pp = PauliProduct::new();
    pp = pp.set_pauli(0, SingleQubitOperator::X);
    pp = pp.set_pauli(1, SingleQubitOperator::Z);

    let mut pp_remapped = PauliProduct::new();
    pp_remapped = pp_remapped.set_pauli(1, SingleQubitOperator::X);
    pp_remapped = pp_remapped.set_pauli(0, SingleQubitOperator::Z);

    let mut mapping: HashMap<usize, usize> = HashMap::new();
    mapping.insert(0, 1);
    mapping.insert(1, 0);

    assert_eq!(pp.remap_qubits(&mapping), pp_remapped);
}

// Test the remap_qubits function of the PauliProduct
#[test]
fn remap_qubits_without_full() {
    let mut pp = PauliProduct::new();
    pp = pp.set_pauli(0, SingleQubitOperator::X);
    pp = pp.set_pauli(2, SingleQubitOperator::Z);

    let mut pp_remapped = PauliProduct::new();
    pp_remapped = pp_remapped.set_pauli(1, SingleQubitOperator::X);
    pp_remapped = pp_remapped.set_pauli(2, SingleQubitOperator::Z);

    let mut mapping: HashMap<usize, usize> = HashMap::new();
    mapping.insert(0, 1);

    assert_eq!(pp.remap_qubits(&mapping), pp_remapped);
}

// Test the from_str function of the PauliProduct
#[test]
fn from_str() {
    let mut pp = PauliProduct::new();

    // Empty should print as identity
    assert_eq!(PauliProduct::from_str("I").unwrap(), pp);

    pp = pp.set_pauli(0, SingleQubitOperator::X);
    pp = pp.set_pauli(2, SingleQubitOperator::Y);
    pp = pp.set_pauli(100, SingleQubitOperator::Z);
    let string = "0X2Y100Z";

    assert_eq!(PauliProduct::from_str(string).unwrap(), pp);

    let string_err = "0X100J";
    let error = PauliProduct::from_str(string_err);
    assert!(error.is_err());
    assert_eq!(
        error,
        Err(StruqtureError::IncorrectPauliEntry {
            pauli: "J".to_string()
        })
    );

    let string_err = " X";
    let error = PauliProduct::from_str(string_err);
    assert!(error.is_err());
    assert_eq!(
        error,
        Err(StruqtureError::FromStringFailed {
            msg: "Using   instead of unsigned integer as spin index".to_string()
        })
    );
}

// Test the Iter traits of PauliProduct: into_iter, from_iter and extend
#[test]
fn into_iter_from_iter_extend() {
    let mut pp_0: PauliProduct = PauliProduct::new().z(0);
    let pp_01: PauliProduct = PauliProduct::new().z(0).x(1);

    let pp_iter = pp_0.clone().into_iter();
    assert_eq!(PauliProduct::from_iter(pp_iter), pp_0);

    let mut mapping: BTreeMap<usize, SingleQubitOperator> = BTreeMap::new();
    mapping.insert(1, SingleQubitOperator::X);
    let mapping_iter = mapping.into_iter();
    pp_0.extend(mapping_iter);
    assert_eq!(pp_0, pp_01);
}

// Test the multiplication: SingleQubitOperator * SingleQubitOperator with all possible pauli matrices
#[test_case("", "0X", ("0X", Complex64::new(1.0, 0.0)); "i_x")]
#[test_case("0X", "", ("0X", Complex64::new(1.0, 0.0)); "x_i")]
#[test_case("0X", "0X", ("", Complex64::new(1.0, 0.0)); "x_x")]
#[test_case("0X", "0Y", ("0Z",Complex64::new(0.0, 1.0)); "x_y")]
#[test_case("0X", "0Z", ("0Y", Complex64::new(0.0, -1.0)); "x_z")]
#[test_case("0Y", "0X", ("0Z", Complex64::new(0.0, -1.0)); "y_x")]
#[test_case("0Y", "0Y", ("", Complex64::new(1.0, 0.0)); "y_y")]
#[test_case("0Y", "0Z", ("0X", Complex64::new(0.0, 1.0)); "y_z")]
#[test_case("0Z", "0X", ("0Y", Complex64::new(0.0, 1.0)); "z_x")]
#[test_case("0Z", "0Y", ("0X", Complex64::new(0.0, -1.0)); "z_y")]
#[test_case("0Z", "0Z", ("", Complex64::new(1.0, 0.0)); "z_z")]
#[test_case("0Z", "1Z", ("0Z1Z", Complex64::new(1.0, 0.0)); "different_indices")]
fn multiply(pp0: &str, pp1: &str, result: (&str, Complex64)) {
    let pp_0: PauliProduct = PauliProduct::from_str(pp0).unwrap();
    let pp_1: PauliProduct = PauliProduct::from_str(pp1).unwrap();

    let (mul_r, res) = PauliProduct::multiply(pp_0, pp_1);

    assert_eq!(mul_r, PauliProduct::from_str(result.0).unwrap());
    assert_eq!(res, result.1);
}

// Test the set_pauli and get functions of the PauliProduct
#[test]
fn get_value_get_transform() {
    let mut pp = PauliProduct::new();
    pp = pp.set_pauli(0, SingleQubitOperator::X);
    pp = pp.set_pauli(2, SingleQubitOperator::Z);

    assert_eq!(PauliProduct::get_key(&pp), pp);
    assert_eq!(
        PauliProduct::get_transform(&pp, CalculatorComplex::new(1.0, 2.0)),
        CalculatorComplex::new(1.0, 2.0)
    );
}

// Test the Hash, Debug and Display traits of PauliProduct
#[test]
fn hash_debug() {
    let mut pp = PauliProduct::new();

    // Empty should resolve as identity
    assert_eq!(format!("{}", pp), "I");

    pp = pp.set_pauli(0, SingleQubitOperator::X);
    pp = pp.set_pauli(2, SingleQubitOperator::Z);

    assert_eq!(
        format!("{:?}", pp),
        "PauliProduct { items: [(0, X), (2, Z)] }"
    );
    assert_eq!(format!("{}", pp), "0X2Z");

    let mut pp_1 = PauliProduct::new();
    pp_1 = pp_1.set_pauli(0, SingleQubitOperator::X);
    pp_1 = pp_1.set_pauli(2, SingleQubitOperator::Z);
    let mut pp_2 = PauliProduct::new();
    pp_2 = pp_2.set_pauli(2, SingleQubitOperator::Z);
    pp_2 = pp_2.set_pauli(0, SingleQubitOperator::X);

    let mut s_1 = DefaultHasher::new();
    pp_1.hash(&mut s_1);
    let mut s_2 = DefaultHasher::new();
    pp_2.hash(&mut s_2);
    assert_eq!(s_1.finish(), s_2.finish())
}

// Test the Clone, PartialEq, PartialOrd and Ord traits of PauliProduct
#[test]
fn clone_partial_eq_partial_ord() {
    let mut pp = PauliProduct::new();
    pp = pp.set_pauli(0, SingleQubitOperator::X);

    // Test Clone trait
    assert_eq!(pp.clone(), pp);

    // Test PartialEq trait
    let mut pp_0 = PauliProduct::new();
    pp_0 = pp_0.set_pauli(0, SingleQubitOperator::X);
    let mut pp_1 = PauliProduct::new();
    pp_1 = pp_1.set_pauli(0, SingleQubitOperator::Z);
    assert!(pp_0 == pp);
    assert!(pp == pp_0);
    assert!(pp_1 != pp);
    assert!(pp != pp_1);

    // Test PartialOrd trait
    let mut pp_0 = PauliProduct::new();
    pp_0 = pp_0.set_pauli(0, SingleQubitOperator::X);
    let mut pp_1 = PauliProduct::new();
    pp_1 = pp_1.set_pauli(0, SingleQubitOperator::Z);

    assert_eq!(pp_0.partial_cmp(&pp), Some(Ordering::Equal));
    assert_eq!(pp.partial_cmp(&pp_0), Some(Ordering::Equal));
    assert_eq!(pp_1.partial_cmp(&pp), Some(Ordering::Greater));
    assert_eq!(pp.partial_cmp(&pp_1), Some(Ordering::Less));

    // Test Ord trait
    assert_eq!(pp_0.cmp(&pp), Ordering::Equal);
    assert_eq!(pp.cmp(&pp_0), Ordering::Equal);
    assert_eq!(pp_1.cmp(&pp), Ordering::Greater);
    assert_eq!(pp.cmp(&pp_1), Ordering::Less);
}

#[test]
fn serde_json() {
    let mut pp = PauliProduct::new();
    pp = pp.set_pauli(0, SingleQubitOperator::X);

    let serialized = serde_json::to_string(&pp).unwrap();
    let deserialized: PauliProduct = serde_json::from_str(&serialized).unwrap();
    assert_eq!(pp, deserialized);
}

/// Test PauliProduct Serialization and Deserialization traits (readable)
#[test]
fn serde_readable() {
    let mut pp = PauliProduct::new();
    pp = pp.set_pauli(0, SingleQubitOperator::X);
    assert_tokens(&pp.readable(), &[Token::Str("0X")]);
}

#[test]
fn serde_readable_empty() {
    let pp = PauliProduct::new();
    assert_tokens(&pp.readable(), &[Token::Str("I")]);
}

#[test]
fn bincode() {
    let mut pp = PauliProduct::new();
    pp = pp.set_pauli(0, SingleQubitOperator::X);

    let encoded: Vec<u8> = bincode::serialize(&pp).unwrap();
    let decoded: PauliProduct = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(pp, decoded);

    let encoded: Vec<u8> = bincode::serialize(&pp.clone().compact()).unwrap();
    let decoded: PauliProduct = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(pp, decoded);
}

/// Test PauliProduct Serialization and Deserialization traits (compact)
#[test]
fn serde_compact() {
    let mut pp = PauliProduct::new();
    pp = pp.set_pauli(0, SingleQubitOperator::X);

    assert_tokens(
        &pp.compact(),
        &[
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::U64(0),
            Token::UnitVariant {
                name: "SingleQubitOperator",
                variant: "X",
            },
            Token::TupleEnd,
            Token::SeqEnd,
        ],
    );
}

/// Test PauliProduct Serialization and Deserialization traits (compact)
#[test]
fn bincode_single() {
    let spinop = SingleQubitOperator::X;

    let encoded: Vec<u8> = bincode::serialize(&spinop).unwrap();
    let decoded: SingleQubitOperator = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(spinop.clone(), decoded);

    let encoded: Vec<u8> = bincode::serialize(&spinop.compact()).unwrap();
    let decoded: SingleQubitOperator = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(spinop.clone(), decoded);

    let encoded: Vec<u8> = bincode::serialize(&spinop.readable()).unwrap();
    let decoded: SingleQubitOperator = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(spinop.clone(), decoded);
}

// Test the from_str function of the SingleQubitOperator
#[test]
fn single_from_str() {
    let id = SingleQubitOperator::Identity;
    let string_id = "I";
    assert_eq!(SingleQubitOperator::from_str(string_id).unwrap(), id);

    let x = SingleQubitOperator::X;
    let string_x = "X";
    assert_eq!(SingleQubitOperator::from_str(string_x).unwrap(), x);

    let y = SingleQubitOperator::Y;
    let string_y = "Y";
    assert_eq!(SingleQubitOperator::from_str(string_y).unwrap(), y);

    let z = SingleQubitOperator::Z;
    let string_z = "Z";
    assert_eq!(SingleQubitOperator::from_str(string_z).unwrap(), z);

    let string_err = "J";
    let error = SingleQubitOperator::from_str(string_err);
    assert!(error.is_err());
    assert_eq!(
        error,
        Err(StruqtureError::IncorrectPauliEntry {
            pauli: "J".to_string()
        })
    );
}

// Test the Debug and Display traits of SingleQubitOperator
#[test]
fn single_hash_debug() {
    assert_eq!(format!("{:?}", SingleQubitOperator::Identity), "Identity");
    assert_eq!(format!("{:?}", SingleQubitOperator::X), "X");
    assert_eq!(format!("{:?}", SingleQubitOperator::Y), "Y");
    assert_eq!(format!("{:?}", SingleQubitOperator::Z), "Z");

    assert_eq!(format!("{}", SingleQubitOperator::Identity), "I");
    assert_eq!(format!("{}", SingleQubitOperator::X), "X");
    assert_eq!(format!("{}", SingleQubitOperator::Y), "Y");
    assert_eq!(format!("{}", SingleQubitOperator::Z), "Z");

    let mut s_1 = DefaultHasher::new();
    SingleQubitOperator::X.hash(&mut s_1);
    let mut s_2 = DefaultHasher::new();
    SingleQubitOperator::X.hash(&mut s_2);
    assert_eq!(s_1.finish(), s_2.finish())
}

// Test the Clone, PartialEq, PartialOrd and Ord traits of SingleQubitOperator
#[test]
fn single_clone_partial_eq() {
    let x = SingleQubitOperator::X;

    // Test Clone trait
    assert_eq!(x.clone(), x);

    // Test PartialEq trait
    let x_0 = SingleQubitOperator::X;
    let y = SingleQubitOperator::Y;
    assert!(x_0 == x);
    assert!(x == x_0);
    assert!(y != x);
    assert!(x != y);

    // Test PartialOrd trait
    assert_eq!(x_0.partial_cmp(&x), Some(Ordering::Equal));
    assert_eq!(x.partial_cmp(&x_0), Some(Ordering::Equal));
    assert_eq!(y.partial_cmp(&x), Some(Ordering::Greater));
    assert_eq!(x.partial_cmp(&y), Some(Ordering::Less));

    // Test Ord trait
    assert_eq!(x_0.cmp(&x), Ordering::Equal);
    assert_eq!(x.cmp(&x_0), Ordering::Equal);
    assert_eq!(y.cmp(&x), Ordering::Greater);
    assert_eq!(x.cmp(&y), Ordering::Less);
}

#[test]
fn test_singlequbitoperator_product() {
    use itertools::Itertools;

    let help_vec = vec![
        (
            SingleQubitOperator::Identity,
            array![
                [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
                [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)]
            ],
        ),
        (
            SingleQubitOperator::X,
            array![
                [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
                [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)]
            ],
        ),
        (
            SingleQubitOperator::Y,
            array![
                [Complex64::new(0.0, 0.0), Complex64::new(0.0, -1.0)],
                [Complex64::new(0.0, 1.0), Complex64::new(0.0, 0.0)]
            ],
        ),
        (
            SingleQubitOperator::Z,
            array![
                [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
                [Complex64::new(0.0, 0.0), Complex64::new(-1.0, 0.0)]
            ],
        ),
    ];

    let mut lookup: HashMap<SingleQubitOperator, Array2<Complex64>> = HashMap::new();
    for (op, mat) in help_vec.clone().into_iter() {
        lookup.insert(op, mat);
    }

    for ((inner, inner_mat), (outer, outer_mat)) in
        help_vec.iter().cartesian_product(help_vec.iter())
    {
        let (op, prefactor) = SingleQubitOperator::multiply(*inner, *outer);
        let test_mat = lookup.get(&op).unwrap() * prefactor;
        let direct_matrix_multiplication = inner_mat.dot(outer_mat);
        assert_eq!(test_mat, direct_matrix_multiplication)
    }
}

#[cfg(feature = "json_schema")]
#[test]
fn test_pauli_product_schema() {
    let pp = PauliProduct::new();
    let schema = schemars::schema_for!(PauliProduct);
    let schema_checker = jsonschema::JSONSchema::compile(&serde_json::to_value(&schema).unwrap())
        .expect("schema is valid");
    let value = serde_json::to_value(pp).unwrap();
    let validation = schema_checker.validate(&value);
    assert!(validation.is_ok());
}

#[cfg(feature = "struqture_1_import")]
#[cfg(feature = "struqture_1_export")]
#[test]
fn test_from_to_struqture_1() {
    let pp = struqture_one::spins::PauliProduct::from_str("0X1Y25Z").unwrap();
    let pp_2 = PauliProduct::new().x(0).y(1).z(25);
    assert!(PauliProduct::from_struqture_1(&pp).unwrap() == pp_2);
    assert!(pp == pp_2.to_struqture_1().unwrap());
}
