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

//! Integration test for public API of PlusMinusProduct

use ndarray::{array, Array2};
use num_complex::Complex64;
use serde_test::{assert_tokens, Configure, Token};
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};
use std::iter::{FromIterator, IntoIterator};
use std::str::FromStr;
use struqture::spins::{
    DecoherenceProduct, PauliProduct, PlusMinusProduct, SingleDecoherenceOperator,
    SinglePlusMinusOperator, SingleSpinOperator,
};
use struqture::{SpinIndex, StruqtureError, SymmetricIndex};

// Test the new function of the PlusMinusProduct
#[test]
fn new() {
    let mut pmp = PlusMinusProduct::new();
    assert!(pmp.is_natural_hermitian());
    pmp = pmp.set_pauli(0, SinglePlusMinusOperator::Z);
    assert!(pmp.is_natural_hermitian());
    pmp = pmp.set_pauli(1, SinglePlusMinusOperator::Minus);
    assert!(!pmp.is_natural_hermitian());
    let mut pmp_h = PlusMinusProduct::new();
    pmp_h = pmp_h.set_pauli(2, SinglePlusMinusOperator::Plus);
    assert!(!pmp_h.is_natural_hermitian());

    let mut pp_compare = PlusMinusProduct::new();
    // assert!(pp_compare.is_empty());
    // assert_eq!(pp_compare.current_number_spins(), 0_usize);
    pp_compare = pp_compare.set_pauli(0, SinglePlusMinusOperator::Z);
    pp_compare = pp_compare.set_pauli(1, SinglePlusMinusOperator::Minus);

    assert_eq!(pmp, pp_compare);
    // assert_eq!(pp_compare.corresponds_to(), pp_compare);
    assert_eq!(PlusMinusProduct::new(), PlusMinusProduct::default())
}

// Test the set_pauli and get functions of the PlusMinusProduct
#[test]
fn internal_map_set_get() {
    let mut pmp = PlusMinusProduct::new();
    pmp = pmp.set_pauli(0, SinglePlusMinusOperator::Plus);
    pmp = pmp.set_pauli(2, SinglePlusMinusOperator::Minus);
    pmp = pmp.set_pauli(3, SinglePlusMinusOperator::Z);

    assert_eq!(pmp.get(&0).unwrap(), &SinglePlusMinusOperator::Plus);
    assert_eq!(pmp.get(&2).unwrap(), &SinglePlusMinusOperator::Minus);
    assert_eq!(pmp.get(&3).unwrap(), &SinglePlusMinusOperator::Z);
    assert_eq!(pmp.get(&1), None);

    let pmp = pmp.set_pauli(3, SinglePlusMinusOperator::Plus);
    let pmp = pmp.set_pauli(1, SinglePlusMinusOperator::Identity);
    assert_eq!(pmp.get(&1), None);
    assert_eq!(pmp.get(&3).unwrap(), &SinglePlusMinusOperator::Plus);
    let pmp = pmp.set_pauli(3, SinglePlusMinusOperator::Identity);
    assert_eq!(pmp.get(&3), None);
    let pmp = pmp.set_pauli(3, SinglePlusMinusOperator::Plus);

    // assert_eq!(pmp.current_number_spins(), 4_usize);
    // assert_eq!(pmp.len(), 3_usize);

    let mut internal: BTreeMap<usize, SinglePlusMinusOperator> = BTreeMap::new();
    internal.insert(0, SinglePlusMinusOperator::Plus);
    internal.insert(2, SinglePlusMinusOperator::Minus);
    internal.insert(3, SinglePlusMinusOperator::Plus);
    assert!(pmp.iter().map(|(k, _)| k).all(|k| internal.contains_key(k)));
}

// Test the set_pauli and get functions of the PlusMinusProduct
#[test]
fn hermitian_conjugate() {
    let mut pmp = PlusMinusProduct::new();
    pmp = pmp.set_pauli(0, SinglePlusMinusOperator::Plus);
    pmp = pmp.set_pauli(2, SinglePlusMinusOperator::Minus);
    pmp = pmp.set_pauli(3, SinglePlusMinusOperator::Z);

    let mut pmp_conj = PlusMinusProduct::new();
    pmp_conj = pmp_conj.set_pauli(0, SinglePlusMinusOperator::Minus);
    pmp_conj = pmp_conj.set_pauli(2, SinglePlusMinusOperator::Plus);
    pmp_conj = pmp_conj.set_pauli(3, SinglePlusMinusOperator::Z);
    assert_eq!(pmp.hermitian_conjugate(), (pmp_conj, 1.0));

    let mut pmp = PlusMinusProduct::new();
    pmp = pmp.set_pauli(0, SinglePlusMinusOperator::Plus);
    pmp = pmp.set_pauli(3, SinglePlusMinusOperator::Z);

    let mut pmp_conj = PlusMinusProduct::new();
    pmp_conj = pmp_conj.set_pauli(0, SinglePlusMinusOperator::Minus);
    pmp_conj = pmp_conj.set_pauli(3, SinglePlusMinusOperator::Z);
    assert_eq!(pmp.hermitian_conjugate(), (pmp_conj, 1.0));

    let mut pmp = PlusMinusProduct::new();
    pmp = pmp.set_pauli(0, SinglePlusMinusOperator::Plus);
    pmp = pmp.set_pauli(3, SinglePlusMinusOperator::Z);
    pmp = pmp.set_pauli(2, SinglePlusMinusOperator::Minus);
    pmp = pmp.set_pauli(4, SinglePlusMinusOperator::Minus);

    let mut pmp_conj = PlusMinusProduct::new();
    pmp_conj = pmp_conj.set_pauli(0, SinglePlusMinusOperator::Minus);
    pmp_conj = pmp_conj.set_pauli(3, SinglePlusMinusOperator::Z);
    pmp_conj = pmp_conj.set_pauli(2, SinglePlusMinusOperator::Plus);
    pmp_conj = pmp_conj.set_pauli(4, SinglePlusMinusOperator::Plus);
    assert_eq!(pmp.hermitian_conjugate(), (pmp_conj, 1.0));
}

// Test the x_y_z and get functions of the PlusMinusProduct
#[test]
fn x_y_z() {
    let pmp = PlusMinusProduct::new();
    assert_eq!(
        pmp.clone().plus(0),
        pmp.clone().set_pauli(0, SinglePlusMinusOperator::Plus)
    );
    assert_eq!(
        pmp.clone().minus(2),
        pmp.clone().set_pauli(2, SinglePlusMinusOperator::Minus)
    );
    assert_eq!(
        pmp.clone().z(3),
        pmp.set_pauli(3, SinglePlusMinusOperator::Z)
    );
}

// Test the concatenate function of the PlusMinusProduct
#[test]
fn concatenate() {
    // Without error
    let mut pp_0 = PlusMinusProduct::new();
    pp_0 = pp_0.set_pauli(0, SinglePlusMinusOperator::Plus);
    let mut pp_1 = PlusMinusProduct::new();
    pp_1 = pp_1.set_pauli(1, SinglePlusMinusOperator::Z);

    let mut pp_both = PlusMinusProduct::new();
    pp_both = pp_both.set_pauli(0, SinglePlusMinusOperator::Plus);
    pp_both = pp_both.set_pauli(1, SinglePlusMinusOperator::Z);

    assert_eq!(pp_0.concatenate(pp_1).unwrap(), pp_both);

    // With error
    let mut pp_0 = PlusMinusProduct::new();
    pp_0 = pp_0.set_pauli(0, SinglePlusMinusOperator::Plus);
    let mut pp_1 = PlusMinusProduct::new();
    pp_1 = pp_1.set_pauli(0, SinglePlusMinusOperator::Z);

    let error = pp_0.concatenate(pp_1);
    assert!(error.is_err());
    assert_eq!(
        error,
        Err(StruqtureError::ProductIndexAlreadyOccupied { index: 0 })
    );
}

// Test the remap_qubits function of the PlusMinusProduct
#[test]
fn remap_qubits() {
    let mut pmp = PlusMinusProduct::new();
    pmp = pmp.set_pauli(0, SinglePlusMinusOperator::Plus);
    pmp = pmp.set_pauli(1, SinglePlusMinusOperator::Z);

    let mut pp_remapped = PlusMinusProduct::new();
    pp_remapped = pp_remapped.set_pauli(1, SinglePlusMinusOperator::Plus);
    pp_remapped = pp_remapped.set_pauli(0, SinglePlusMinusOperator::Z);

    let mut mapping: HashMap<usize, usize> = HashMap::new();
    mapping.insert(0, 1);
    mapping.insert(1, 0);

    assert_eq!(pmp.remap_qubits(&mapping), pp_remapped);
}

// Test the remap_qubits function of the PlusMinusProduct
#[test]
fn remap_qubits_without_full() {
    let mut pmp = PlusMinusProduct::new();
    pmp = pmp.set_pauli(0, SinglePlusMinusOperator::Plus);
    pmp = pmp.set_pauli(2, SinglePlusMinusOperator::Z);

    let mut pp_remapped = PlusMinusProduct::new();
    pp_remapped = pp_remapped.set_pauli(1, SinglePlusMinusOperator::Plus);
    pp_remapped = pp_remapped.set_pauli(2, SinglePlusMinusOperator::Z);

    let mut mapping: HashMap<usize, usize> = HashMap::new();
    mapping.insert(0, 1);

    assert_eq!(pmp.remap_qubits(&mapping), pp_remapped);
}

// Test the from_str function of the PlusMinusProduct
#[test]
fn from_str() {
    let mut pmp = PlusMinusProduct::new();

    // Empty should print as identity
    assert_eq!(PlusMinusProduct::from_str("I").unwrap(), pmp);

    pmp = pmp.set_pauli(0, SinglePlusMinusOperator::Plus);
    pmp = pmp.set_pauli(2, SinglePlusMinusOperator::Minus);
    pmp = pmp.set_pauli(100, SinglePlusMinusOperator::Z);
    let string = "0+2-100Z";

    assert_eq!(PlusMinusProduct::from_str(string).unwrap(), pmp);

    let string_err = "0Z100J";
    let error = PlusMinusProduct::from_str(string_err);
    assert!(error.is_err());
    assert_eq!(
        error,
        Err(StruqtureError::IncorrectPauliEntry {
            pauli: "J".to_string()
        })
    );

    let string_err = "3 +";
    let error = PlusMinusProduct::from_str(string_err);
    assert!(error.is_err());
    assert_eq!(
        error,
        Err(StruqtureError::FromStringFailed {
            msg: "Using 3  instead of unsigned integer as spin index".to_string()
        })
    );

    let string_err = "0Z0+";
    let error = PlusMinusProduct::from_str(string_err);
    assert!(error.is_err());
    assert_eq!(
        error,
        Err(StruqtureError::FromStringFailed {
            msg: "At least one spin index is used more than once.".to_string()
        })
    );

    let string_err = "X";
    let error = PlusMinusProduct::from_str(string_err);
    assert!(error.is_err());
    assert_eq!(
        error,
        Err(StruqtureError::FromStringFailed {
            msg: "Missing spin index in the following PlusMinusProduct: X".to_string()
        })
    );
}

// Test the Iter traits of PlusMinusProduct: into_iter, from_iter and extend
#[test]
fn into_iter_from_iter_extend() {
    let mut pp_0: PlusMinusProduct = PlusMinusProduct::new().z(0);
    let pp_01: PlusMinusProduct = PlusMinusProduct::new().z(0).plus(1);

    let pp_iter = pp_0.clone().into_iter();
    assert_eq!(PlusMinusProduct::from_iter(pp_iter), pp_0);

    let mut mapping: BTreeMap<usize, SinglePlusMinusOperator> = BTreeMap::new();
    mapping.insert(1, SinglePlusMinusOperator::Plus);
    let mapping_iter = mapping.into_iter();
    pp_0.extend(mapping_iter);
    assert_eq!(pp_0, pp_01);
}

// Test the Hash, Debug and Display traits of PlusMinusProduct
#[test]
fn hash_debug() {
    let mut pmp = PlusMinusProduct::new();

    // Empty should resolve as identity
    assert_eq!(format!("{}", pmp), "I");

    pmp = pmp.set_pauli(0, SinglePlusMinusOperator::Plus);
    pmp = pmp.set_pauli(1, SinglePlusMinusOperator::Minus);
    pmp = pmp.set_pauli(2, SinglePlusMinusOperator::Z);
    pmp = pmp.set_pauli(3, SinglePlusMinusOperator::Identity);

    assert_eq!(
        format!("{:?}", pmp),
        "PlusMinusProduct { items: [(0, Plus), (1, Minus), (2, Z)] }"
    );
    assert_eq!(format!("{}", pmp), "0+1-2Z");

    let mut pp_1 = PlusMinusProduct::new();
    pp_1 = pp_1.set_pauli(0, SinglePlusMinusOperator::Plus);
    pp_1 = pp_1.set_pauli(1, SinglePlusMinusOperator::Minus);
    pp_1 = pp_1.set_pauli(2, SinglePlusMinusOperator::Z);
    let mut pp_2 = PlusMinusProduct::new();
    pp_2 = pp_2.set_pauli(2, SinglePlusMinusOperator::Z);
    pp_2 = pp_2.set_pauli(1, SinglePlusMinusOperator::Minus);
    pp_2 = pp_2.set_pauli(0, SinglePlusMinusOperator::Plus);

    let mut s_1 = DefaultHasher::new();
    pp_1.hash(&mut s_1);
    let mut s_2 = DefaultHasher::new();
    pp_2.hash(&mut s_2);
    assert_eq!(s_1.finish(), s_2.finish())
}

// Test the Clone, PartialEq, PartialOrd and Ord traits of PlusMinusProduct
#[test]
fn clone_partial_eq_partial_ord() {
    let mut pmp = PlusMinusProduct::new();
    pmp = pmp.set_pauli(0, SinglePlusMinusOperator::Plus);

    // Test Clone trait
    assert_eq!(pmp.clone(), pmp);

    // Test PartialEq trait
    let mut pp_0 = PlusMinusProduct::new();
    pp_0 = pp_0.set_pauli(0, SinglePlusMinusOperator::Plus);
    let mut pp_1 = PlusMinusProduct::new();
    pp_1 = pp_1.set_pauli(0, SinglePlusMinusOperator::Z);
    pp_1 = pp_1.set_pauli(1, SinglePlusMinusOperator::Minus);
    assert!(pp_0 == pmp);
    assert!(pmp == pp_0);
    assert!(pp_1 != pmp);
    assert!(pmp != pp_1);

    // Test PartialOrd trait
    let mut pp_0 = PlusMinusProduct::new();
    pp_0 = pp_0.set_pauli(0, SinglePlusMinusOperator::Plus);
    let mut pp_1 = PlusMinusProduct::new();
    pp_1 = pp_1.set_pauli(0, SinglePlusMinusOperator::Z);

    assert_eq!(pp_0.partial_cmp(&pmp), Some(Ordering::Equal));
    assert_eq!(pmp.partial_cmp(&pp_0), Some(Ordering::Equal));
    assert_eq!(pp_1.partial_cmp(&pmp), Some(Ordering::Greater));
    assert_eq!(pmp.partial_cmp(&pp_1), Some(Ordering::Less));

    // Test Ord trait
    assert_eq!(pp_0.cmp(&pmp), Ordering::Equal);
    assert_eq!(pmp.cmp(&pp_0), Ordering::Equal);
    assert_eq!(pp_1.cmp(&pmp), Ordering::Greater);
    assert_eq!(pmp.cmp(&pp_1), Ordering::Less);
}

#[test]
fn serde_json() {
    let mut pmp = PlusMinusProduct::new();
    pmp = pmp.set_pauli(0, SinglePlusMinusOperator::Plus);

    let serialized = serde_json::to_string(&pmp).unwrap();
    let deserialized: PlusMinusProduct = serde_json::from_str(&serialized).unwrap();
    assert_eq!(pmp, deserialized);
}

/// Test PlusMinusProduct Serialization and Deserialization traits (readable)
#[test]
fn serde_readable() {
    let mut pmp = PlusMinusProduct::new();
    pmp = pmp.set_pauli(0, SinglePlusMinusOperator::Plus);
    assert_tokens(&pmp.readable(), &[Token::Str("0+")]);
}

#[test]
fn serde_readable_empty() {
    let pmp = PlusMinusProduct::new();
    assert_tokens(&pmp.readable(), &[Token::Str("I")]);
}

#[test]
fn bincode() {
    let mut pmp = PlusMinusProduct::new();
    pmp = pmp.set_pauli(0, SinglePlusMinusOperator::Plus);

    let config = bincode::config::legacy();
    let serialized: Vec<u8> = bincode::serde::encode_to_vec(&pmp, config).unwrap();
    let (deserialized, _len): (PlusMinusProduct, usize) =
        bincode::serde::decode_from_slice(&serialized[..], config).unwrap();
    assert_eq!(pmp, deserialized);

    let serialized: Vec<u8> = bincode::serde::encode_to_vec(pmp.clone().compact(), config).unwrap();
    let (deserialized, _len): (PlusMinusProduct, usize) =
        bincode::serde::decode_from_slice(&serialized[..], config).unwrap();
    assert_eq!(pmp, deserialized);
}

/// Test PlusMinusProduct Serialization and Deserialization traits (compact)
#[test]
fn serde_compact() {
    let mut pmp = PlusMinusProduct::new();
    pmp = pmp.set_pauli(0, SinglePlusMinusOperator::Plus);

    assert_tokens(
        &pmp.compact(),
        &[
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::U64(0),
            Token::UnitVariant {
                name: "SinglePlusMinusOperator",
                variant: "Plus",
            },
            Token::TupleEnd,
            Token::SeqEnd,
        ],
    );
}

/// Test PlusMinusProduct Serialization and Deserialization traits (compact)
#[test]
fn bincode_single() {
    let spinop = SinglePlusMinusOperator::Plus;

    let config = bincode::config::legacy();
    let encoded: Vec<u8> = bincode::serde::encode_to_vec(&spinop, config).unwrap();
    let (decoded, _): (SinglePlusMinusOperator, usize) =
        bincode::serde::decode_from_slice(&encoded[..], config).unwrap();
    assert_eq!(spinop.clone(), decoded);

    let encoded: Vec<u8> = bincode::serde::encode_to_vec(&spinop.compact(), config).unwrap();
    let (decoded, _): (SinglePlusMinusOperator, usize) =
        bincode::serde::decode_from_slice(&encoded[..], config).unwrap();
    assert_eq!(spinop.clone(), decoded);

    let encoded: Vec<u8> = bincode::serde::encode_to_vec(&spinop.readable(), config).unwrap();
    let (decoded, _): (SinglePlusMinusOperator, usize) =
        bincode::serde::decode_from_slice(&encoded[..], config).unwrap();
    assert_eq!(spinop.clone(), decoded);
}

// Test the from_str function of the SinglePlusMinusOperator
#[test]
fn single_from_str() {
    let id = SinglePlusMinusOperator::Identity;
    let string_id = "I";
    assert_eq!(SinglePlusMinusOperator::from_str(string_id).unwrap(), id);

    let x = SinglePlusMinusOperator::Plus;
    let string_x = "+";
    assert_eq!(SinglePlusMinusOperator::from_str(string_x).unwrap(), x);

    let y = SinglePlusMinusOperator::Minus;
    let string_y = "-";
    assert_eq!(SinglePlusMinusOperator::from_str(string_y).unwrap(), y);

    let z = SinglePlusMinusOperator::Z;
    let string_z = "Z";
    assert_eq!(SinglePlusMinusOperator::from_str(string_z).unwrap(), z);

    let string_err = "J";
    let error = SinglePlusMinusOperator::from_str(string_err);
    assert!(error.is_err());
    assert_eq!(
        error,
        Err(StruqtureError::IncorrectPauliEntry {
            pauli: "J".to_string()
        })
    );
}

// Test the Debug and Display traits of SinglePlusMinusOperator
#[test]
fn single_hash_debug() {
    assert_eq!(
        format!("{:?}", SinglePlusMinusOperator::Identity),
        "Identity"
    );
    assert_eq!(format!("{:?}", SinglePlusMinusOperator::Plus), "Plus");
    assert_eq!(format!("{:?}", SinglePlusMinusOperator::Minus), "Minus");
    assert_eq!(format!("{:?}", SinglePlusMinusOperator::Z), "Z");

    assert_eq!(format!("{}", SinglePlusMinusOperator::Identity), "I");
    assert_eq!(format!("{}", SinglePlusMinusOperator::Plus), "+");
    assert_eq!(format!("{}", SinglePlusMinusOperator::Minus), "-");
    assert_eq!(format!("{}", SinglePlusMinusOperator::Z), "Z");

    let mut s_1 = DefaultHasher::new();
    SinglePlusMinusOperator::Plus.hash(&mut s_1);
    let mut s_2 = DefaultHasher::new();
    SinglePlusMinusOperator::Plus.hash(&mut s_2);
    assert_eq!(s_1.finish(), s_2.finish())
}

// Test the Clone, PartialEq, PartialOrd and Ord traits of SinglePlusMinusOperator
#[test]
fn single_clone_partial_eq() {
    let x = SinglePlusMinusOperator::Plus;

    // Test Clone trait
    assert_eq!(x.clone(), x);

    // Test PartialEq trait
    let x_0 = SinglePlusMinusOperator::Plus;
    let y = SinglePlusMinusOperator::Minus;
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
fn test_single_plus_minus_operator_product() {
    use itertools::Itertools;

    let help_vec = vec![
        (
            SinglePlusMinusOperator::Identity,
            array![
                [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
                [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)]
            ],
        ),
        (
            SinglePlusMinusOperator::Plus,
            array![
                [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
                [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)]
            ],
        ),
        (
            SinglePlusMinusOperator::Minus,
            array![
                [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
                [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)]
            ],
        ),
        (
            SinglePlusMinusOperator::Z,
            array![
                [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
                [Complex64::new(0.0, 0.0), Complex64::new(-1.0, 0.0)]
            ],
        ),
    ];

    let mut lookup: HashMap<SinglePlusMinusOperator, Array2<Complex64>> = HashMap::new();
    for (op, mat) in help_vec.clone().into_iter() {
        lookup.insert(op, mat);
    }

    for ((inner, inner_mat), (outer, outer_mat)) in
        help_vec.iter().cartesian_product(help_vec.iter())
    {
        let result_vector = SinglePlusMinusOperator::multiply(*inner, *outer);
        let mut test_mat = array![
            [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)]
        ];
        for (op, prefactor) in result_vector.iter() {
            test_mat = test_mat + lookup.get(op).unwrap() * *prefactor;
        }
        let direct_matrix_multiplication = inner_mat.dot(outer_mat);
        assert_eq!(test_mat, direct_matrix_multiplication)
    }
}

#[test]
fn single_so_from_single_pm() {
    let result: Vec<(SingleSpinOperator, Complex64)> =
        Vec::<(SingleSpinOperator, Complex64)>::from(SinglePlusMinusOperator::Z);
    assert_eq!(
        result,
        vec![(SingleSpinOperator::Z, Complex64::new(1.0, 0.0))]
    );

    let result: Vec<(SingleSpinOperator, Complex64)> =
        Vec::<(SingleSpinOperator, Complex64)>::from(SinglePlusMinusOperator::Identity);
    assert_eq!(
        result,
        vec![(SingleSpinOperator::Identity, Complex64::new(1.0, 0.0))]
    );

    let result: Vec<(SingleSpinOperator, Complex64)> =
        Vec::<(SingleSpinOperator, Complex64)>::from(SinglePlusMinusOperator::Plus);
    assert_eq!(
        result,
        vec![
            (SingleSpinOperator::X, Complex64::new(0.5, 0.0)),
            (SingleSpinOperator::Y, Complex64::new(0.0, 0.5))
        ]
    );

    let result: Vec<(SingleSpinOperator, Complex64)> =
        Vec::<(SingleSpinOperator, Complex64)>::from(SinglePlusMinusOperator::Minus);
    assert_eq!(
        result,
        vec![
            (SingleSpinOperator::X, Complex64::new(0.5, 0.0)),
            (SingleSpinOperator::Y, Complex64::new(0.0, -0.5))
        ]
    );
}

#[test]
fn single_do_from_single_pm() {
    let result: Vec<(SingleDecoherenceOperator, Complex64)> =
        Vec::<(SingleDecoherenceOperator, Complex64)>::from(SinglePlusMinusOperator::Z);
    assert_eq!(
        result,
        vec![(SingleDecoherenceOperator::Z, Complex64::new(1.0, 0.0))]
    );

    let result: Vec<(SingleDecoherenceOperator, Complex64)> =
        Vec::<(SingleDecoherenceOperator, Complex64)>::from(SinglePlusMinusOperator::Identity);
    assert_eq!(
        result,
        vec![(
            SingleDecoherenceOperator::Identity,
            Complex64::new(1.0, 0.0)
        )]
    );

    let result: Vec<(SingleDecoherenceOperator, Complex64)> =
        Vec::<(SingleDecoherenceOperator, Complex64)>::from(SinglePlusMinusOperator::Plus);
    assert_eq!(
        result,
        vec![
            (SingleDecoherenceOperator::X, Complex64::new(0.5, 0.0)),
            (SingleDecoherenceOperator::IY, Complex64::new(0.5, 0.0))
        ]
    );

    let result: Vec<(SingleDecoherenceOperator, Complex64)> =
        Vec::<(SingleDecoherenceOperator, Complex64)>::from(SinglePlusMinusOperator::Minus);
    assert_eq!(
        result,
        vec![
            (SingleDecoherenceOperator::X, Complex64::new(0.5, 0.0)),
            (SingleDecoherenceOperator::IY, Complex64::new(-0.5, 0.0))
        ]
    );
}

#[test]
fn single_pm_from_single_so() {
    let result: Vec<(SinglePlusMinusOperator, Complex64)> =
        Vec::<(SinglePlusMinusOperator, Complex64)>::from(SingleSpinOperator::Z);
    assert_eq!(
        result,
        vec![(SinglePlusMinusOperator::Z, Complex64::new(1.0, 0.0))]
    );

    let result: Vec<(SinglePlusMinusOperator, Complex64)> =
        Vec::<(SinglePlusMinusOperator, Complex64)>::from(SingleSpinOperator::Identity);
    assert_eq!(
        result,
        vec![(SinglePlusMinusOperator::Identity, Complex64::new(1.0, 0.0))]
    );

    let result: Vec<(SinglePlusMinusOperator, Complex64)> =
        Vec::<(SinglePlusMinusOperator, Complex64)>::from(SingleSpinOperator::X);
    assert_eq!(
        result,
        vec![
            (SinglePlusMinusOperator::Plus, Complex64::new(1.0, 0.0)),
            (SinglePlusMinusOperator::Minus, Complex64::new(1.0, 0.0))
        ]
    );

    let result: Vec<(SinglePlusMinusOperator, Complex64)> =
        Vec::<(SinglePlusMinusOperator, Complex64)>::from(SingleSpinOperator::Y);
    assert_eq!(
        result,
        vec![
            (SinglePlusMinusOperator::Plus, Complex64::new(0.0, -1.0)),
            (SinglePlusMinusOperator::Minus, Complex64::new(0.0, 1.0))
        ]
    );
}

#[test]
fn single_pm_from_single_do() {
    let result: Vec<(SinglePlusMinusOperator, Complex64)> =
        Vec::<(SinglePlusMinusOperator, Complex64)>::from(SingleDecoherenceOperator::Z);
    assert_eq!(
        result,
        vec![(SinglePlusMinusOperator::Z, Complex64::new(1.0, 0.0))]
    );

    let result: Vec<(SinglePlusMinusOperator, Complex64)> =
        Vec::<(SinglePlusMinusOperator, Complex64)>::from(SingleDecoherenceOperator::Identity);
    assert_eq!(
        result,
        vec![(SinglePlusMinusOperator::Identity, Complex64::new(1.0, 0.0))]
    );

    let result: Vec<(SinglePlusMinusOperator, Complex64)> =
        Vec::<(SinglePlusMinusOperator, Complex64)>::from(SingleDecoherenceOperator::X);
    assert_eq!(
        result,
        vec![
            (SinglePlusMinusOperator::Plus, Complex64::new(1.0, 0.0)),
            (SinglePlusMinusOperator::Minus, Complex64::new(1.0, 0.0))
        ]
    );

    let result: Vec<(SinglePlusMinusOperator, Complex64)> =
        Vec::<(SinglePlusMinusOperator, Complex64)>::from(SingleDecoherenceOperator::IY);
    assert_eq!(
        result,
        vec![
            (SinglePlusMinusOperator::Plus, Complex64::new(1.0, 0.0)),
            (SinglePlusMinusOperator::Minus, Complex64::new(-1.0, 0.0))
        ]
    );
}

#[test]
fn pm_from_pp() {
    let result: Vec<(PlusMinusProduct, Complex64)> =
        Vec::<(PlusMinusProduct, Complex64)>::from(PauliProduct::new().z(0));
    assert_eq!(
        result,
        vec![(PlusMinusProduct::new().z(0), Complex64::new(1.0, 0.0))]
    );

    let result: Vec<(PlusMinusProduct, Complex64)> =
        Vec::<(PlusMinusProduct, Complex64)>::from(PauliProduct::new());
    assert_eq!(
        result,
        vec![(PlusMinusProduct::new(), Complex64::new(1.0, 0.0))]
    );

    let result: Vec<(PlusMinusProduct, Complex64)> =
        Vec::<(PlusMinusProduct, Complex64)>::from(PauliProduct::new().x(0));
    assert_eq!(
        result,
        vec![
            (PlusMinusProduct::new().plus(0), Complex64::new(1.0, 0.0)),
            (PlusMinusProduct::new().minus(0), Complex64::new(1.0, 0.0))
        ]
    );

    let result: Vec<(PlusMinusProduct, Complex64)> =
        Vec::<(PlusMinusProduct, Complex64)>::from(PauliProduct::new().y(0));
    assert_eq!(
        result,
        vec![
            (PlusMinusProduct::new().plus(0), Complex64::new(0.0, -1.0)),
            (PlusMinusProduct::new().minus(0), Complex64::new(0.0, 1.0))
        ]
    );

    let result: Vec<(PlusMinusProduct, Complex64)> =
        Vec::<(PlusMinusProduct, Complex64)>::from(PauliProduct::new().x(0).y(1).z(2));
    assert_eq!(
        result,
        vec![
            (
                PlusMinusProduct::new().plus(0).plus(1).z(2),
                Complex64::new(0.0, -1.0)
            ),
            (
                PlusMinusProduct::new().minus(0).plus(1).z(2),
                Complex64::new(0.0, -1.0)
            ),
            (
                PlusMinusProduct::new().plus(0).minus(1).z(2),
                Complex64::new(0.0, 1.0)
            ),
            (
                PlusMinusProduct::new().minus(0).minus(1).z(2),
                Complex64::new(0.0, 1.0)
            ),
        ]
    );
}

#[test]
fn pp_from_pm() {
    let result: Vec<(PauliProduct, Complex64)> =
        Vec::<(PauliProduct, Complex64)>::from(PlusMinusProduct::new().z(0));
    assert_eq!(
        result,
        vec![(PauliProduct::new().z(0), Complex64::new(1.0, 0.0))]
    );

    let result: Vec<(PauliProduct, Complex64)> =
        Vec::<(PauliProduct, Complex64)>::from(PlusMinusProduct::new());
    assert_eq!(
        result,
        vec![(PauliProduct::new(), Complex64::new(1.0, 0.0))]
    );

    let result: Vec<(PauliProduct, Complex64)> =
        Vec::<(PauliProduct, Complex64)>::from(PlusMinusProduct::new().plus(0));
    assert_eq!(
        result,
        vec![
            (PauliProduct::new().x(0), Complex64::new(0.5, 0.0)),
            (PauliProduct::new().y(0), Complex64::new(0.0, 0.5))
        ]
    );

    let result: Vec<(PauliProduct, Complex64)> =
        Vec::<(PauliProduct, Complex64)>::from(PlusMinusProduct::new().minus(0));
    assert_eq!(
        result,
        vec![
            (PauliProduct::new().x(0), Complex64::new(0.5, 0.0)),
            (PauliProduct::new().y(0), Complex64::new(0.0, -0.5))
        ]
    );

    let result: Vec<(PauliProduct, Complex64)> =
        Vec::<(PauliProduct, Complex64)>::from(PlusMinusProduct::new().plus(0).minus(1).z(2));
    assert_eq!(
        result,
        vec![
            (
                PauliProduct::new().x(0).x(1).z(2),
                Complex64::new(0.25, 0.0)
            ),
            (
                PauliProduct::new().y(0).x(1).z(2),
                Complex64::new(0.0, 0.25)
            ),
            (
                PauliProduct::new().x(0).y(1).z(2),
                Complex64::new(0.0, -0.25)
            ),
            (
                PauliProduct::new().y(0).y(1).z(2),
                Complex64::new(0.25, 0.0)
            ),
        ]
    );
}

#[test]
fn pm_from_dp() {
    let result: Vec<(PlusMinusProduct, Complex64)> =
        Vec::<(PlusMinusProduct, Complex64)>::from(DecoherenceProduct::new().z(0));
    assert_eq!(
        result,
        vec![(PlusMinusProduct::new().z(0), Complex64::new(1.0, 0.0))]
    );

    let result: Vec<(PlusMinusProduct, Complex64)> =
        Vec::<(PlusMinusProduct, Complex64)>::from(DecoherenceProduct::new());
    assert_eq!(
        result,
        vec![(PlusMinusProduct::new(), Complex64::new(1.0, 0.0))]
    );

    let result: Vec<(PlusMinusProduct, Complex64)> =
        Vec::<(PlusMinusProduct, Complex64)>::from(DecoherenceProduct::new().x(0));
    assert_eq!(
        result,
        vec![
            (PlusMinusProduct::new().plus(0), Complex64::new(1.0, 0.0)),
            (PlusMinusProduct::new().minus(0), Complex64::new(1.0, 0.0))
        ]
    );

    let result: Vec<(PlusMinusProduct, Complex64)> =
        Vec::<(PlusMinusProduct, Complex64)>::from(DecoherenceProduct::new().iy(0));
    assert_eq!(
        result,
        vec![
            (PlusMinusProduct::new().plus(0), Complex64::new(1.0, 0.0)),
            (PlusMinusProduct::new().minus(0), Complex64::new(-1.0, 0.0))
        ]
    );

    let result: Vec<(PlusMinusProduct, Complex64)> =
        Vec::<(PlusMinusProduct, Complex64)>::from(DecoherenceProduct::new().x(0).iy(1).z(2));
    assert_eq!(
        result,
        vec![
            (
                PlusMinusProduct::new().plus(0).plus(1).z(2),
                Complex64::new(1.0, 0.0)
            ),
            (
                PlusMinusProduct::new().minus(0).plus(1).z(2),
                Complex64::new(1.0, 0.0)
            ),
            (
                PlusMinusProduct::new().plus(0).minus(1).z(2),
                Complex64::new(-1.0, 0.0)
            ),
            (
                PlusMinusProduct::new().minus(0).minus(1).z(2),
                Complex64::new(-1.0, 0.0)
            ),
        ]
    );
}

#[test]
fn dp_from_pm() {
    let result: Vec<(DecoherenceProduct, Complex64)> =
        Vec::<(DecoherenceProduct, Complex64)>::from(PlusMinusProduct::new().z(0));
    assert_eq!(
        result,
        vec![(DecoherenceProduct::new().z(0), Complex64::new(1.0, 0.0))]
    );

    let result: Vec<(DecoherenceProduct, Complex64)> =
        Vec::<(DecoherenceProduct, Complex64)>::from(PlusMinusProduct::new());
    assert_eq!(
        result,
        vec![(DecoherenceProduct::new(), Complex64::new(1.0, 0.0))]
    );

    let result: Vec<(DecoherenceProduct, Complex64)> =
        Vec::<(DecoherenceProduct, Complex64)>::from(PlusMinusProduct::new().plus(0));
    assert_eq!(
        result,
        vec![
            (DecoherenceProduct::new().x(0), Complex64::new(0.5, 0.0)),
            (DecoherenceProduct::new().iy(0), Complex64::new(0.5, 0.0))
        ]
    );

    let result: Vec<(DecoherenceProduct, Complex64)> =
        Vec::<(DecoherenceProduct, Complex64)>::from(PlusMinusProduct::new().minus(0));
    assert_eq!(
        result,
        vec![
            (DecoherenceProduct::new().x(0), Complex64::new(0.5, 0.0)),
            (DecoherenceProduct::new().iy(0), Complex64::new(-0.5, 0.0))
        ]
    );

    let result: Vec<(DecoherenceProduct, Complex64)> =
        Vec::<(DecoherenceProduct, Complex64)>::from(PlusMinusProduct::new().plus(0).minus(1).z(2));
    assert_eq!(
        result,
        vec![
            (
                DecoherenceProduct::new().x(0).x(1).z(2),
                Complex64::new(0.25, 0.0)
            ),
            (
                DecoherenceProduct::new().iy(0).x(1).z(2),
                Complex64::new(0.25, 0.0)
            ),
            (
                DecoherenceProduct::new().x(0).iy(1).z(2),
                Complex64::new(-0.25, 0.0)
            ),
            (
                DecoherenceProduct::new().iy(0).iy(1).z(2),
                Complex64::new(-0.25, 0.0)
            ),
        ]
    );
}

#[cfg(feature = "json_schema")]
#[test]
fn test_plus_minus_product_schema() {
    let pp = PlusMinusProduct::new();
    let schema = schemars::schema_for!(PlusMinusProduct);
    let schema_checker = jsonschema::validator_for(&serde_json::to_value(&schema).unwrap())
        .expect("schema is valid");
    let value = serde_json::to_value(pp).unwrap();
    let validation = schema_checker.validate(&value);
    assert!(validation.is_ok());
}
