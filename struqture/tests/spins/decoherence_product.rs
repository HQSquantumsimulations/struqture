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

//! Integration test for public API of DecoherenceProduct

use super::create_na_matrix_from_decoherence_list;
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
use struqture::spins::{
    DecoherenceProduct, PauliProduct, SingleDecoherenceOperator, SingleQubitOperator,
};
use struqture::{CorrespondsTo, GetValue, SpinIndex, StruqtureError, SymmetricIndex};
use test_case::test_case;

// Test the new function of the DecoherenceProduct
#[test]
fn new() {
    let mut dp = DecoherenceProduct::new();
    dp = dp.set_pauli(0, SingleDecoherenceOperator::X);

    let mut dp_compare = DecoherenceProduct::new();
    assert!(dp_compare.is_empty());
    assert!(dp_compare.is_natural_hermitian());
    assert_eq!(dp_compare.current_number_spins(), 0_usize);
    dp_compare = dp_compare.set_pauli(0, SingleDecoherenceOperator::X);

    assert_eq!(dp, dp_compare);
    assert_eq!(dp_compare.corresponds_to(), dp_compare);
    assert_eq!(DecoherenceProduct::new(), DecoherenceProduct::default())
}

// Test the set_pauli and get functions of the DecoherenceProduct
#[test]
fn hermitian_conjugate() {
    let mut dp = DecoherenceProduct::new();
    dp = dp.set_pauli(0, SingleDecoherenceOperator::X);
    dp = dp.set_pauli(2, SingleDecoherenceOperator::IY);
    dp = dp.set_pauli(3, SingleDecoherenceOperator::Z);

    assert_eq!(dp.hermitian_conjugate(), (dp, -1.0));

    let mut dp = DecoherenceProduct::new();
    dp = dp.set_pauli(0, SingleDecoherenceOperator::X);
    dp = dp.set_pauli(3, SingleDecoherenceOperator::Z);

    assert_eq!(dp.hermitian_conjugate(), (dp, 1.0));

    let mut dp = DecoherenceProduct::new();
    dp = dp.set_pauli(0, SingleDecoherenceOperator::X);
    dp = dp.set_pauli(3, SingleDecoherenceOperator::Z);
    dp = dp.set_pauli(2, SingleDecoherenceOperator::IY);
    dp = dp.set_pauli(4, SingleDecoherenceOperator::IY);

    assert_eq!(dp.hermitian_conjugate(), (dp, 1.0));
}

#[test]
fn test_singledecoherenceoperator_product() {
    use itertools::Itertools;

    let help_vec = vec![
        (
            SingleDecoherenceOperator::Identity,
            array![[1.0, 0.0], [0.0, 1.0]],
        ),
        (SingleDecoherenceOperator::X, array![[0.0, 1.0], [1.0, 0.0]]),
        (
            SingleDecoherenceOperator::IY,
            array![[0.0, 1.0], [-1.0, 0.0]],
        ),
        (
            SingleDecoherenceOperator::Z,
            array![[1.0, 0.0], [0.0, -1.0]],
        ),
    ];

    let mut lookup: HashMap<SingleDecoherenceOperator, Array2<f64>> = HashMap::new();
    for (op, mat) in help_vec.clone().into_iter() {
        lookup.insert(op, mat);
    }

    for ((inner, inner_mat), (outer, outer_mat)) in
        help_vec.iter().cartesian_product(help_vec.iter())
    {
        //let mut test_mat = array![[0.0, 0.0], [0.0, 0.0]];
        let (op, prefactor) = SingleDecoherenceOperator::multiply(*inner, *outer);
        let test_mat = lookup.get(&op).unwrap() * prefactor;
        let direct_matrix_multiplication = inner_mat.dot(outer_mat);
        assert_eq!(test_mat, direct_matrix_multiplication)
    }
}

// Test the x_iy_z and get functions of the DecoherenceProduct
#[test]
fn x_iy_z() {
    let dp = DecoherenceProduct::new();
    assert_eq!(
        dp.clone().x(0),
        dp.clone().set_pauli(0, SingleDecoherenceOperator::X)
    );
    assert_eq!(
        dp.clone().iy(2),
        dp.clone().set_pauli(2, SingleDecoherenceOperator::IY)
    );
    assert_eq!(
        dp.clone().z(3),
        dp.set_pauli(3, SingleDecoherenceOperator::Z)
    );
}

// Test the concatenate function of the DecoherenceProduct
#[test]
fn concatenate() {
    // Without error
    let mut dp_0 = DecoherenceProduct::new();
    dp_0 = dp_0.set_pauli(0, SingleDecoherenceOperator::X);
    let mut dp_1 = DecoherenceProduct::new();
    dp_1 = dp_1.set_pauli(1, SingleDecoherenceOperator::Z);

    let mut dp_both = DecoherenceProduct::new();
    dp_both = dp_both.set_pauli(0, SingleDecoherenceOperator::X);
    dp_both = dp_both.set_pauli(1, SingleDecoherenceOperator::Z);

    assert_eq!(dp_0.concatenate(dp_1).unwrap(), dp_both);

    // With error
    let mut dp_0 = DecoherenceProduct::new();
    dp_0 = dp_0.set_pauli(0, SingleDecoherenceOperator::X);
    let mut dp_1 = DecoherenceProduct::new();
    dp_1 = dp_1.set_pauli(0, SingleDecoherenceOperator::Z);

    let error = dp_0.concatenate(dp_1);
    assert!(error.is_err());
    assert_eq!(
        error,
        Err(StruqtureError::ProductIndexAlreadyOccupied { index: 0 })
    );
}

// Test the remap_qubits function of the DecoherenceProduct
#[test]
fn remap_qubits() {
    let mut dp = DecoherenceProduct::new();
    dp = dp.set_pauli(0, SingleDecoherenceOperator::X);
    dp = dp.set_pauli(1, SingleDecoherenceOperator::Z);

    let mut dp_remapped = DecoherenceProduct::new();
    dp_remapped = dp_remapped.set_pauli(1, SingleDecoherenceOperator::X);
    dp_remapped = dp_remapped.set_pauli(0, SingleDecoherenceOperator::Z);

    let mut mapping: HashMap<usize, usize> = HashMap::new();
    mapping.insert(0, 1);
    mapping.insert(1, 0);

    assert_eq!(dp.remap_qubits(&mapping), dp_remapped);
}

// Test the remap_qubits function of the DecoherenceProduct
#[test]
fn remap_qubits_without_full() {
    let mut dp = DecoherenceProduct::new();
    dp = dp.set_pauli(0, SingleDecoherenceOperator::X);
    dp = dp.set_pauli(2, SingleDecoherenceOperator::Z);

    let mut dp_remapped = DecoherenceProduct::new();
    dp_remapped = dp_remapped.set_pauli(1, SingleDecoherenceOperator::X);
    dp_remapped = dp_remapped.set_pauli(2, SingleDecoherenceOperator::Z);

    let mut mapping: HashMap<usize, usize> = HashMap::new();
    mapping.insert(0, 1);

    assert_eq!(dp.remap_qubits(&mapping), dp_remapped);
}

// Test the from_str function of the DecoherenceProduct
#[test]
fn from_str() {
    let mut dp = DecoherenceProduct::new();

    // Identity should be the same as empty
    assert_eq!(DecoherenceProduct::from_str("I").unwrap(), dp);

    dp = dp.set_pauli(0, SingleDecoherenceOperator::X);
    dp = dp.set_pauli(1, SingleDecoherenceOperator::Z);
    dp = dp.set_pauli(200, SingleDecoherenceOperator::IY);
    let string = "0X1Z200iY";

    assert_eq!(DecoherenceProduct::from_str(string).unwrap(), dp);

    let string_err = "0X100J";
    let error = DecoherenceProduct::from_str(string_err);
    assert!(error.is_err());
    assert_eq!(
        error,
        Err(StruqtureError::IncorrectPauliEntry {
            pauli: "J".to_string()
        })
    );

    let string_err = " X";
    let error = DecoherenceProduct::from_str(string_err);
    assert!(error.is_err());
    assert_eq!(
        error,
        Err(StruqtureError::FromStringFailed {
            msg: "Using   instead of unsigned integer as spin index".to_string()
        })
    );
}

// Test the Iter traits of DecoherenceProduct: into_iter, from_iter and extend
#[test]
fn into_iter_from_iter_extend() {
    let mut dp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let dp_01: DecoherenceProduct = DecoherenceProduct::new().z(0).x(1);

    let dp_iter = dp_0.clone().into_iter();
    assert_eq!(DecoherenceProduct::from_iter(dp_iter), dp_0);

    let mut mapping: BTreeMap<usize, SingleDecoherenceOperator> = BTreeMap::new();
    mapping.insert(1, SingleDecoherenceOperator::X);
    let mapping_iter = mapping.into_iter();
    dp_0.extend(mapping_iter);
    assert_eq!(dp_0, dp_01);
}

// Test the multiplication: SingleQubitOperator * SingleQubitOperator with all possible pauli matrices
#[test_case("", "0X", ("0X", Complex64::new(1.0, 0.0)); "i_x")]
#[test_case("0X", "", ("0X", Complex64::new(1.0, 0.0)); "x_i")]
#[test_case("0X", "0X", ("", Complex64::new(1.0, 0.0)); "x_x")]
#[test_case("0X", "0iY", ("0Z",Complex64::new(-1.0, 0.0)); "x_y")]
#[test_case("0X", "0Z", ("0iY", Complex64::new(-1.0, 0.0)); "x_z")]
#[test_case("0iY", "0X", ("0Z", Complex64::new(1.0, 0.0)); "y_x")]
#[test_case("0iY", "0iY", ("", Complex64::new(-1.0, 0.0)); "y_y")]
#[test_case("0iY", "0Z", ("0X", Complex64::new(-1.0, 0.0)); "y_z")]
#[test_case("0Z", "0X", ("0iY", Complex64::new(1.0, 0.0)); "z_x")]
#[test_case("0Z", "0iY", ("0X", Complex64::new(1.0, 0.0)); "z_y")]
#[test_case("0Z", "0Z", ("", Complex64::new(1.0, 0.0)); "z_z")]
#[test_case("0Z", "1Z", ("0Z1Z", Complex64::new(1.0, 0.0)); "different_indices")]
fn multiply(pp0: &str, pp1: &str, result: (&str, Complex64)) {
    let pp_0: DecoherenceProduct = DecoherenceProduct::from_str(pp0).unwrap();
    let pp_1: DecoherenceProduct = DecoherenceProduct::from_str(pp1).unwrap();

    let (mul_r, res) = DecoherenceProduct::multiply(pp_0, pp_1);

    assert_eq!(mul_r, DecoherenceProduct::from_str(result.0).unwrap());
    assert_eq!(res, result.1);
}

// Test the to_coo function
#[test]
fn to_coo_simple() {
    let mut dp = DecoherenceProduct::new();
    dp = dp.set_pauli(0, SingleDecoherenceOperator::Z);
    dp = dp.set_pauli(1, SingleDecoherenceOperator::X);

    let result = dp.to_coo(2).unwrap();
    let one = Complex64::new(1.0, 0.0);
    assert_eq!(result.0, vec![one, -one, one, -one]);
    assert_eq!(result.1 .0, vec![0, 1, 2, 3]);
    assert_eq!(result.1 .1, vec![2, 3, 0, 1]);
}

#[test_case("0Z", &["Z"]; "0Z")]
#[test_case("1X", &["X", "I"]; "1X")]
#[test_case("0iY", &["iY"]; "0iY")]
#[test_case("0X", &["X"]; "0X")]
#[test_case("0X1X", &["X", "X"]; "0x1x")]
#[test_case("0X1iY", &["iY", "X"]; "0X1iY")]
#[test_case("1Z2iY", &["iY", "Z", "I"]; "1z2iy")]
fn test_superoperator(representation: &str, operators: &[&str]) {
    let pp: DecoherenceProduct = DecoherenceProduct::from_str(representation).unwrap();
    let dimension = 2_usize.pow(operators.len() as u32);

    // Constructing matrix by hand:
    let test_matrix = create_na_matrix_from_decoherence_list(operators);

    let coo_test_matrix = pp.to_coo(3).unwrap();
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
                    assert_eq!(val, 0.0.into())
                }
            }
        }
    }
}

// Test the set_pauli and get functions of the PauliProduct
#[test]
fn get_value_get_transform() {
    let mut dp = DecoherenceProduct::new();
    dp = dp.set_pauli(0, SingleDecoherenceOperator::X);
    dp = dp.set_pauli(2, SingleDecoherenceOperator::Z);

    assert_eq!(DecoherenceProduct::get_key(&dp), dp);
    assert_eq!(
        DecoherenceProduct::get_transform(&dp, CalculatorComplex::new(1.0, 2.0)),
        CalculatorComplex::new(1.0, 2.0)
    );
}

// Test the set_pauli and get functions of the PauliProduct
#[test]
fn get_value_get_transform_tuple() {
    let mut dp = DecoherenceProduct::new();
    dp = dp.set_pauli(0, SingleDecoherenceOperator::X);
    dp = dp.set_pauli(2, SingleDecoherenceOperator::Z);

    assert_eq!(
        <(DecoherenceProduct, DecoherenceProduct)>::get_key(&(dp.clone(), dp.clone())),
        (dp.clone(), dp.clone())
    );
    assert_eq!(
        <(DecoherenceProduct, DecoherenceProduct)>::get_transform(
            &(dp.clone(), dp),
            CalculatorComplex::new(1.0, 2.0)
        ),
        CalculatorComplex::new(1.0, 2.0)
    );
}

// Test the Hash, Debug and Display traits of DecoherenceProduct
#[test]
fn hash_debug() {
    let mut dp = DecoherenceProduct::new();

    // Empty should resolve as identity:
    assert_eq!(format!("{}", dp), "I");

    dp = dp.set_pauli(0, SingleDecoherenceOperator::X);
    dp = dp.set_pauli(2, SingleDecoherenceOperator::Z);

    assert_eq!(
        format!("{:?}", dp),
        "DecoherenceProduct { items: [(0, X), (2, Z)] }"
    );
    assert_eq!(format!("{}", dp), "0X2Z");

    let mut dp_1 = DecoherenceProduct::new();
    dp_1 = dp_1.set_pauli(0, SingleDecoherenceOperator::X);
    dp_1 = dp_1.set_pauli(2, SingleDecoherenceOperator::Z);
    let mut dp_2 = DecoherenceProduct::new();
    dp_2 = dp_2.set_pauli(2, SingleDecoherenceOperator::Z);
    dp_2 = dp_2.set_pauli(0, SingleDecoherenceOperator::X);

    let mut s_1 = DefaultHasher::new();
    dp_1.hash(&mut s_1);
    let mut s_2 = DefaultHasher::new();
    dp_2.hash(&mut s_2);
    assert_eq!(s_1.finish(), s_2.finish())
}

// Test the Clone, PartialEq, PartialOrd and Ord traits of DecoherenceProduct
#[test]
fn clone_partial_eq_partial_ord() {
    let mut dp = DecoherenceProduct::new();
    dp = dp.set_pauli(0, SingleDecoherenceOperator::X);

    // Test Clone trait
    assert_eq!(dp.clone(), dp);

    // Test PartialEq trait
    let mut dp_0 = DecoherenceProduct::new();
    dp_0 = dp_0.set_pauli(0, SingleDecoherenceOperator::X);
    let mut dp_1 = DecoherenceProduct::new();
    dp_1 = dp_1.set_pauli(0, SingleDecoherenceOperator::Z);
    assert!(dp_0 == dp);
    assert!(dp == dp_0);
    assert!(dp_1 != dp);
    assert!(dp != dp_1);

    // Test PartialOrd trait
    let mut dp_0 = DecoherenceProduct::new();
    dp_0 = dp_0.set_pauli(0, SingleDecoherenceOperator::X);
    let mut dp_1 = DecoherenceProduct::new();
    dp_1 = dp_1.set_pauli(0, SingleDecoherenceOperator::Z);

    assert_eq!(dp_0.partial_cmp(&dp), Some(Ordering::Equal));
    assert_eq!(dp.partial_cmp(&dp_0), Some(Ordering::Equal));
    assert_eq!(dp_1.partial_cmp(&dp), Some(Ordering::Greater));
    assert_eq!(dp.partial_cmp(&dp_1), Some(Ordering::Less));

    // Test Ord trait
    assert_eq!(dp_0.cmp(&dp), Ordering::Equal);
    assert_eq!(dp.cmp(&dp_0), Ordering::Equal);
    assert_eq!(dp_1.cmp(&dp), Ordering::Greater);
    assert_eq!(dp.cmp(&dp_1), Ordering::Less);
}

/// Test DecoherenceProduct Serialization and Deserialization traits (readable)
#[test]
fn serde_json() {
    let mut dp = DecoherenceProduct::new();
    dp = dp.set_pauli(0, SingleDecoherenceOperator::X);

    let serialized = serde_json::to_string(&dp).unwrap();
    let deserialized: DecoherenceProduct = serde_json::from_str(&serialized).unwrap();
    assert_eq!(dp, deserialized);
}

/// Test DecoherenceProduct Serialization and Deserialization traits (readable)
#[test]
fn serde_readable() {
    let mut dp = DecoherenceProduct::new();
    dp = dp.set_pauli(0, SingleDecoherenceOperator::X);

    assert_tokens(&dp.readable(), &[Token::Str("0X")]);
}

#[test]
fn serde_readable_empty() {
    let dp = DecoherenceProduct::new();
    assert_tokens(&dp.readable(), &[Token::Str("I")]);
}

/// Test DecoherenceProduct Serialization and Deserialization traits (compact)
#[test]
fn bincode() {
    let mut dp = DecoherenceProduct::new();
    dp = dp.set_pauli(0, SingleDecoherenceOperator::X);

    let encoded: Vec<u8> = bincode::serialize(&dp).unwrap();
    let decoded: DecoherenceProduct = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(dp, decoded);

    let encoded: Vec<u8> = bincode::serialize(&dp.clone().compact()).unwrap();
    let decoded: DecoherenceProduct = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(dp, decoded);
}

/// Test DecoherenceProduct Serialization and Deserialization traits (compact)
#[test]
fn serde_compact() {
    let mut dp = DecoherenceProduct::new();
    dp = dp.set_pauli(0, SingleDecoherenceOperator::X);

    assert_tokens(
        &dp.compact(),
        &[
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::U64(0),
            Token::UnitVariant {
                name: "SingleDecoherenceOperator",
                variant: "X",
            },
            Token::TupleEnd,
            Token::SeqEnd,
        ],
    );
}

// Test the conversion from SingleDecoherenceOperator to SingleQubitOperator
#[test_case("I", ("I", Complex64::new(1.0, 0.0)); "identity")]
#[test_case("X", ("X", Complex64::new(1.0, 0.0)); "x")]
#[test_case("iY", ("Y", Complex64::new(0.0, 1.0)); "iy")]
#[test_case("Z", ("Z", Complex64::new(1.0, 0.0)); "z")]
fn decoh_to_spin(dp_str: &str, result: (&str, Complex64)) {
    let dp: SingleDecoherenceOperator = SingleDecoherenceOperator::from_str(dp_str).unwrap();

    let conv_res = SingleDecoherenceOperator::decoherence_to_spin(dp);
    assert_eq!(conv_res.0, SingleQubitOperator::from_str(result.0).unwrap());
    assert_eq!(conv_res.1, result.1);
}

// Test the conversion from SingleQubitOperator to SingleDecoherenceOperator
#[test_case("I", ("I", Complex64::new(1.0, 0.0)); "identity")]
#[test_case("X", ("X", Complex64::new(1.0, 0.0)); "x")]
#[test_case("Y", ("iY", Complex64::new(0.0, -1.0)); "y")]
#[test_case("Z", ("Z", Complex64::new(1.0, 0.0)); "z")]
fn spin_to_decoh(pp_str: &str, result: (&str, Complex64)) {
    let pp: SingleQubitOperator = SingleQubitOperator::from_str(pp_str).unwrap();

    let conv_res = SingleDecoherenceOperator::spin_to_decoherence(pp);
    assert_eq!(
        conv_res.0,
        SingleDecoherenceOperator::from_str(result.0).unwrap()
    );
    assert_eq!(conv_res.1, result.1);
}

// Test the from_str function of the SingleDecoherenceOperator
#[test]
fn single_from_str() {
    let id = SingleDecoherenceOperator::Identity;
    let string_id = "I";
    assert_eq!(SingleDecoherenceOperator::from_str(string_id).unwrap(), id);

    let x = SingleDecoherenceOperator::X;
    let string_x = "X";
    assert_eq!(SingleDecoherenceOperator::from_str(string_x).unwrap(), x);

    let y = SingleDecoherenceOperator::IY;
    let string_y = "iY";
    assert_eq!(SingleDecoherenceOperator::from_str(string_y).unwrap(), y);

    let z = SingleDecoherenceOperator::Z;
    let string_z = "Z";
    assert_eq!(SingleDecoherenceOperator::from_str(string_z).unwrap(), z);

    let string_err = "J";
    let error = SingleDecoherenceOperator::from_str(string_err);
    assert!(error.is_err());
    assert_eq!(
        error,
        Err(StruqtureError::IncorrectPauliEntry {
            pauli: "J".to_string()
        })
    );
}

// Test the Debug and Display traits of SingleDecoherenceOperator
#[test]
fn single_hash_debug() {
    assert_eq!(
        format!("{:?}", SingleDecoherenceOperator::Identity),
        "Identity"
    );
    assert_eq!(format!("{:?}", SingleDecoherenceOperator::X), "X");
    assert_eq!(format!("{:?}", SingleDecoherenceOperator::IY), "IY");
    assert_eq!(format!("{:?}", SingleDecoherenceOperator::Z), "Z");

    assert_eq!(format!("{}", SingleDecoherenceOperator::Identity), "I");
    assert_eq!(format!("{}", SingleDecoherenceOperator::X), "X");
    assert_eq!(format!("{}", SingleDecoherenceOperator::IY), "iY");
    assert_eq!(format!("{}", SingleDecoherenceOperator::Z), "Z");

    let mut s_1 = DefaultHasher::new();
    SingleDecoherenceOperator::X.hash(&mut s_1);
    let mut s_2 = DefaultHasher::new();
    SingleDecoherenceOperator::X.hash(&mut s_2);
    assert_eq!(s_1.finish(), s_2.finish())
}

#[test_case(SingleDecoherenceOperator::X, SingleDecoherenceOperator::X, 1.0; "x")]
#[test_case(SingleDecoherenceOperator::IY, SingleDecoherenceOperator::IY, -1.0; "iy")]
#[test_case(SingleDecoherenceOperator::Z, SingleDecoherenceOperator::Z, 1.0; "z")]
#[test_case(SingleDecoherenceOperator::Identity, SingleDecoherenceOperator::Identity, 1.0; "identity")]
fn hermitian_conjugate_single(
    input: SingleDecoherenceOperator,
    output: SingleDecoherenceOperator,
    factor: f64,
) {
    assert_eq!(input.hermitian_conjugate().0, output);
    assert_eq!(input.hermitian_conjugate().1, factor);
}

// Test the Clone, PartialEq, PartialOrd and Ord traits of SingleDecoherenceOperator
#[test]
fn single_clone_partial_eq() {
    let x = SingleDecoherenceOperator::X;

    // Test Clone trait
    assert_eq!(x.clone(), x);

    // Test PartialEq trait
    let x_0 = SingleDecoherenceOperator::X;
    let y = SingleDecoherenceOperator::IY;
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

#[test_case("1X2Y3Z4X", "1X2iY3Z4X", Complex64::new(0.0, -1.0); "1X2Y3Z4X")]
#[test_case("1Y2Y3Z4X", "1iY2iY3Z4X", Complex64::new(-1.0, 0.0); "1Y2Y3Z4X")]
#[test_case("1Y2Y3Y4X", "1iY2iY3iY4X", Complex64::new(0.0, 1.0); "1Y2Y3Y4X")]
#[test_case("1Y2Y3Y4Y", "1iY2iY3iY4iY", Complex64::new(1.0, 0.0); "1Y2Y3Y4Y")]
fn test_to_decoherence_and_back(pp: &str, dp: &str, factor: Complex64) {
    let original_pp = PauliProduct::from_str(pp).unwrap();
    let original_dp = DecoherenceProduct::from_str(dp).unwrap();

    let (converted_dp, dp_factor) = DecoherenceProduct::spin_to_decoherence(original_pp.clone());
    let (converted_pp, pp_factor) = DecoherenceProduct::decoherence_to_spin(original_dp.clone());

    assert_eq!(original_pp, converted_pp);
    assert_eq!(original_dp, converted_dp);

    assert_eq!(factor, dp_factor);
    assert_eq!(factor, pp_factor.conj());
}

#[cfg(feature = "json_schema")]
#[test]
fn test_decoherence_product_schema() {
    let pp = DecoherenceProduct::new();
    let schema = schemars::schema_for!(DecoherenceProduct);
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
    let pp = struqture_1::spins::DecoherenceProduct::from_str("0X1iY25Z").unwrap();
    let pp_2 = DecoherenceProduct::new().x(0).iy(1).z(25);
    assert!(DecoherenceProduct::from_struqture_1(&pp).unwrap() == pp_2);
    assert!(pp == pp_2.to_struqture_1().unwrap());
}
