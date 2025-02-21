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

//! Integration test for public API of PlusMinusLindbladNoiseOperator

use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde_test::{assert_tokens, Configure, Token};
use std::collections::{BTreeMap, HashMap};
use std::iter::{FromIterator, IntoIterator};
use std::ops::{Add, Sub};
#[cfg(feature = "struqture_1_import")]
#[cfg(feature = "struqture_1_export")]
use std::str::FromStr;
use struqture::spins::{
    DecoherenceProduct, PlusMinusLindbladNoiseOperator, PlusMinusOperator, PlusMinusProduct,
    PauliLindbladNoiseOperator,
};
use struqture::{OperateOnDensityMatrix, SpinIndex, STRUQTURE_VERSION};

// Test the new function of the PlusMinusLindbladNoiseOperator
#[test]
fn new() {
    let slno = PlusMinusLindbladNoiseOperator::new();
    assert!(slno.is_empty());
    assert_eq!(
        PlusMinusLindbladNoiseOperator::new(),
        PlusMinusLindbladNoiseOperator::default()
    );
}

#[test]
fn empty_clone_options() {
    let dp_0: PlusMinusProduct = PlusMinusProduct::new().z(2);
    let mut slno = PlusMinusLindbladNoiseOperator::new();
    slno.set((dp_0.clone(), dp_0), CalculatorComplex::from(0.5))
        .unwrap();

    let empty: Option<usize> = None;
    let full: Option<usize> = Some(3);
    assert_eq!(
        slno.empty_clone(empty),
        PlusMinusLindbladNoiseOperator::new()
    );
    assert_eq!(
        slno.empty_clone(full),
        PlusMinusLindbladNoiseOperator::with_capacity(1)
    );
}

// // Test the current_number_spins function of the PlusMinusLindbladNoiseOperator
// #[test]
// fn internal_map_number_spins() {
//     let dp_0: PlusMinusProduct = PlusMinusProduct::new().plus(0);
//     let dp_2: PlusMinusProduct = PlusMinusProduct::new().z(2);
//     let mut slno = PlusMinusLindbladNoiseOperator::new();
//     assert_eq!(slno.current_number_spins(), 0_usize);
//     slno.set((dp_0.clone(), dp_0), CalculatorComplex::from(0.5))
//         .unwrap();
//     assert_eq!(slno.current_number_spins(), 1_usize);
//     slno.set((dp_2.clone(), dp_2), CalculatorComplex::from(0.5))
//         .unwrap();
//     assert_eq!(slno.current_number_spins(), 3_usize);
// }

// Test the len function of the PlusMinusLindbladNoiseOperator
#[test]
fn internal_map_len() {
    let dp_2: PlusMinusProduct = PlusMinusProduct::new().z(2);
    let mut slno = PlusMinusLindbladNoiseOperator::new();
    slno.set((dp_2.clone(), dp_2), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(slno.len(), 1_usize);
}

// Test the try_set_noise and get functions of the PlusMinusLindbladNoiseOperator
#[test]
fn internal_map_set_get() {
    let dp_2: PlusMinusProduct = PlusMinusProduct::new().z(2);
    let mut slno = PlusMinusLindbladNoiseOperator::new();
    // assert_eq!(slno.current_number_spins(), 0_usize);

    // Vacant
    slno.set((dp_2.clone(), dp_2.clone()), CalculatorComplex::from(0.0))
        .unwrap();
    slno.set((dp_2.clone(), dp_2.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(
        slno.get(&(dp_2.clone(), dp_2.clone())),
        &CalculatorComplex::from(0.5)
    );
    // assert_eq!(slno.current_number_spins(), 3_usize);

    // 2) Test iter, keys, values functions
    let mut map: BTreeMap<(PlusMinusProduct, PlusMinusProduct), CalculatorComplex> =
        BTreeMap::new();
    map.insert((dp_2.clone(), dp_2.clone()), CalculatorComplex::from(0.5));
    // iter
    let dict = slno.iter();
    for (item_d, item_m) in dict.zip(map.iter()) {
        assert_eq!(item_d, item_m);
    }
    // keys
    let keys = slno.keys();
    for (key_s, key_m) in keys.zip(map.keys()) {
        assert_eq!(key_s, key_m);
    }
    // values
    let values = slno.values();
    for (val_s, val_m) in values.zip(map.values()) {
        assert_eq!(val_s, val_m);
    }

    // 3) Test remove function
    slno.remove(&(dp_2.clone(), dp_2));
    assert_eq!(slno, PlusMinusLindbladNoiseOperator::new());
}

// Test the add_noise function of the PlusMinusLindbladNoiseOperator
#[test]
fn internal_map_add_noise() {
    let dp_2: PlusMinusProduct = PlusMinusProduct::new().z(2);
    let mut slno = PlusMinusLindbladNoiseOperator::new();

    let _ = slno.add_operator_product((dp_2.clone(), dp_2.clone()), CalculatorComplex::from(0.5));
    assert_eq!(
        slno.get(&(dp_2.clone(), dp_2.clone())),
        &CalculatorComplex::from(0.5)
    );
    let _ = slno.add_operator_product((dp_2.clone(), dp_2.clone()), CalculatorComplex::from(-0.5));
    assert_eq!(
        slno.get(&(dp_2.clone(), dp_2)),
        &CalculatorComplex::from(0.0)
    );
}

// Test the add_noise_from_full_operators function of the PlusMinusLindbladNoiseOperator
#[test]
fn internal_map_add_noise_from_full_operators() {
    let mut left: PlusMinusOperator = PlusMinusOperator::new();
    left.add_operator_product(PlusMinusProduct::new().plus(0), 0.5.into())
        .unwrap();
    left.add_operator_product(PlusMinusProduct::new().minus(0), 0.5.into())
        .unwrap();

    let mut right: PlusMinusOperator = PlusMinusOperator::new();
    right
        .add_operator_product(PlusMinusProduct::new().plus(0), 0.5.into())
        .unwrap();
    right
        .add_operator_product(PlusMinusProduct::new().minus(0), (-0.5).into())
        .unwrap();

    let mut slno = PlusMinusLindbladNoiseOperator::new();

    let _ = slno.add_noise_from_full_operators(&left, &right, CalculatorComplex::from(10.0));

    assert_eq!(
        slno.get(&(
            PlusMinusProduct::new().plus(0),
            PlusMinusProduct::new().plus(0)
        )),
        &CalculatorComplex::from(2.5)
    );
    assert_eq!(
        slno.get(&(
            PlusMinusProduct::new().plus(0),
            PlusMinusProduct::new().minus(0)
        )),
        &CalculatorComplex::from(-2.5)
    );
    assert_eq!(
        slno.get(&(
            PlusMinusProduct::new().minus(0),
            PlusMinusProduct::new().plus(0)
        )),
        &CalculatorComplex::from(2.5)
    );
    assert_eq!(
        slno.get(&(
            PlusMinusProduct::new().minus(0),
            PlusMinusProduct::new().minus(0)
        )),
        &CalculatorComplex::from(-2.5)
    );
    assert_eq!(
        slno.get(&(
            PlusMinusProduct::new().minus(0),
            PlusMinusProduct::new().z(0)
        )),
        &CalculatorComplex::from(0.0)
    );
    assert_eq!(
        slno.get(&(PlusMinusProduct::new().z(0), PlusMinusProduct::new().z(0))),
        &CalculatorComplex::from(0.0)
    );
}

// Test the add_noise_from_full_operators function of the PlusMinusLindbladNoiseOperator
#[test]
fn internal_map_add_noise_from_full_operators_complex() {
    let mut left: PlusMinusOperator = PlusMinusOperator::new();
    left.add_operator_product(
        PlusMinusProduct::new().minus(0),
        CalculatorComplex::new(0.0, 1.0),
    )
    .unwrap();

    let mut right: PlusMinusOperator = PlusMinusOperator::new();
    right
        .add_operator_product(
            PlusMinusProduct::new().minus(0),
            CalculatorComplex::new(0.0, 1.0),
        )
        .unwrap();

    let mut slno = PlusMinusLindbladNoiseOperator::new();

    let _ = slno.add_noise_from_full_operators(&left, &right, 10.into());

    assert_eq!(
        slno.get(&(
            PlusMinusProduct::new().minus(0),
            PlusMinusProduct::new().minus(0)
        )),
        &CalculatorComplex::from(10.0)
    );
}

// Test the Iter traits of PlusMinusLindbladNoiseOperator: into_iter, from_iter and extend
#[test]
fn into_iter_from_iter_extend() {
    let dp_0: PlusMinusProduct = PlusMinusProduct::new().z(0);
    let dp_1: PlusMinusProduct = PlusMinusProduct::new().plus(1);
    let mut slno_0 = PlusMinusLindbladNoiseOperator::new();
    let _ = slno_0.add_operator_product((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(1.0));

    let slno_iter = slno_0.clone().into_iter();
    assert_eq!(PlusMinusLindbladNoiseOperator::from_iter(slno_iter), slno_0);
    let slno_iter = (&slno_0)
        .into_iter()
        .map(|(key, value)| (key.clone(), value.clone()));
    assert_eq!(PlusMinusLindbladNoiseOperator::from_iter(slno_iter), slno_0);

    let mut mapping: BTreeMap<(PlusMinusProduct, PlusMinusProduct), CalculatorComplex> =
        BTreeMap::new();
    mapping.insert((dp_1.clone(), dp_1.clone()), CalculatorComplex::from(0.5));
    let mapping_iter = mapping.into_iter();
    slno_0.extend(mapping_iter);

    let mut slno_0_1 = PlusMinusLindbladNoiseOperator::new();
    let _ = slno_0_1.add_operator_product((dp_0.clone(), dp_0), CalculatorComplex::from(1.0));
    let _ = slno_0_1.add_operator_product((dp_1.clone(), dp_1), CalculatorComplex::from(0.5));

    assert_eq!(slno_0, slno_0_1);
}

// Test the remap_qubits function of the PauliProduct
#[test]
fn remap_qubits() {
    let dp_1: PlusMinusProduct = PlusMinusProduct::new().z(2).plus(0).z(1);
    let dp_2: PlusMinusProduct = PlusMinusProduct::new().minus(2).minus(1).minus(0);
    let mut slno = PlusMinusLindbladNoiseOperator::new();
    slno.set((dp_1.clone(), dp_1), CalculatorComplex::from(0.3))
        .unwrap();
    slno.set((dp_2.clone(), dp_2), CalculatorComplex::from(0.5))
        .unwrap();

    let dp_1_remapped: PlusMinusProduct = PlusMinusProduct::new().z(0).plus(1).z(2);
    let dp_2_remapped: PlusMinusProduct = PlusMinusProduct::new().minus(2).minus(1).minus(0);
    let mut slno_remapped = PlusMinusLindbladNoiseOperator::new();
    slno_remapped
        .set(
            (dp_1_remapped.clone(), dp_1_remapped),
            CalculatorComplex::from(0.3),
        )
        .unwrap();
    slno_remapped
        .set(
            (dp_2_remapped.clone(), dp_2_remapped),
            CalculatorComplex::from(0.5),
        )
        .unwrap();

    let mut mapping: HashMap<usize, usize> = HashMap::new();
    mapping.insert(0, 1);
    mapping.insert(1, 2);
    mapping.insert(2, 0);

    assert_eq!(slno.remap_qubits(&mapping), slno_remapped);
}

// Test the negative operation: -PlusMinusLindbladNoiseOperator
#[test]
fn negative_slno() {
    let dp_0: PlusMinusProduct = PlusMinusProduct::new().z(0);
    let mut slno_0 = PlusMinusLindbladNoiseOperator::new();
    let _ = slno_0.add_operator_product((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(1.0));
    let mut slno_0_minus = PlusMinusLindbladNoiseOperator::new();
    let _ = slno_0_minus.add_operator_product((dp_0.clone(), dp_0), CalculatorComplex::from(-1.0));

    assert_eq!(-slno_0, slno_0_minus);
}

// Test the addition: PlusMinusLindbladNoiseOperator + PlusMinusLindbladNoiseOperator
#[test]
fn add_slno_slno() {
    let dp_0: PlusMinusProduct = PlusMinusProduct::new().z(0);
    let dp_1: PlusMinusProduct = PlusMinusProduct::new().plus(1);
    let mut slno_0 = PlusMinusLindbladNoiseOperator::new();
    let _ = slno_0.add_operator_product((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(1.0));
    let mut slno_1 = PlusMinusLindbladNoiseOperator::new();
    let _ = slno_1.add_operator_product((dp_1.clone(), dp_1.clone()), CalculatorComplex::from(0.5));
    let mut slno_0_1 = PlusMinusLindbladNoiseOperator::new();
    let _ = slno_0_1.add_operator_product((dp_0.clone(), dp_0), CalculatorComplex::from(1.0));
    let _ = slno_0_1.add_operator_product((dp_1.clone(), dp_1), CalculatorComplex::from(0.5));

    assert_eq!(slno_0.clone() + slno_1.clone(), slno_0_1);
    assert_eq!(slno_0.add(slno_1), slno_0_1);
}

// Test the subtraction: PlusMinusLindbladNoiseOperator - PlusMinusLindbladNoiseOperator
#[test]
fn sub_slno_slno() {
    let dp_0: PlusMinusProduct = PlusMinusProduct::new().z(0);
    let dp_1: PlusMinusProduct = PlusMinusProduct::new().plus(1);
    let mut slno_0 = PlusMinusLindbladNoiseOperator::new();
    let _ = slno_0.add_operator_product((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(1.0));
    let mut slno_1 = PlusMinusLindbladNoiseOperator::new();
    let _ = slno_1.add_operator_product((dp_1.clone(), dp_1.clone()), CalculatorComplex::from(0.5));
    let mut slno_0_1 = PlusMinusLindbladNoiseOperator::new();
    let _ = slno_0_1.add_operator_product((dp_0.clone(), dp_0), CalculatorComplex::from(1.0));
    let _ = slno_0_1.add_operator_product((dp_1.clone(), dp_1), CalculatorComplex::from(-0.5));

    assert_eq!(slno_0.clone() - slno_1.clone(), slno_0_1);
    assert_eq!(slno_0.sub(slno_1), slno_0_1);
}

// Test the multiplication: PlusMinusLindbladNoiseOperator * Calculatorcomplex
#[test]
fn mul_so_cc() {
    let pp_0: PlusMinusProduct = PlusMinusProduct::new().z(0);
    let mut so_0 = PlusMinusLindbladNoiseOperator::new();
    let _ = so_0.set((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(2.0));
    let mut so_0_1 = PlusMinusLindbladNoiseOperator::new();
    let _ = so_0_1.set((pp_0.clone(), pp_0), CalculatorComplex::from(6.0));

    assert_eq!(so_0 * CalculatorComplex::from(3.0), so_0_1);
}

// Test the multiplication: PlusMinusLindbladNoiseOperator * Calculatorcomplex
#[test]
fn mul_so_cf() {
    let pp_0: PlusMinusProduct = PlusMinusProduct::new().z(0);
    let mut so_0 = PlusMinusLindbladNoiseOperator::new();
    let _ = so_0.add_operator_product((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(2.0));
    let mut so_0_1 = PlusMinusLindbladNoiseOperator::new();
    let _ = so_0_1.add_operator_product((pp_0.clone(), pp_0), CalculatorComplex::from(6.0));

    assert_eq!(so_0 * CalculatorFloat::from(3.0), so_0_1);
}

// Test the Debug trait of PlusMinusLindbladNoiseOperator
#[test]
fn debug() {
    let dp: PlusMinusProduct = PlusMinusProduct::new().z(0);
    let mut slno = PlusMinusLindbladNoiseOperator::new();
    let _ = slno.set((dp.clone(), dp), CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{:?}", slno),
        "PlusMinusLindbladNoiseOperator { internal_map: {(PlusMinusProduct { items: [(0, Z)] }, PlusMinusProduct { items: [(0, Z)] }): CalculatorComplex { re: Float(0.5), im: Float(0.0) }} }"
    );
}

// Test the Display trait of PlusMinusOperator
#[test]
fn display() {
    let mut so = PlusMinusLindbladNoiseOperator::new();
    let pp: PlusMinusProduct = PlusMinusProduct::new().z(0);
    let _ = so.set((pp.clone(), pp), CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{}", so),
        "PlusMinusLindbladNoiseOperator{\n(0Z, 0Z): (5e-1 + i * 0e0),\n}"
    );
}

// Test the Clone and PartialEq traits of PlusMinusLindbladNoiseOperator
#[test]
fn clone_partial_eq() {
    let dp: PlusMinusProduct = PlusMinusProduct::new().z(0);
    let mut slno = PlusMinusLindbladNoiseOperator::new();
    slno.set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    // Test Clone trait
    assert_eq!(slno.clone(), slno);

    // Test PartialEq trait
    let dp_1: PlusMinusProduct = PlusMinusProduct::new().z(0);
    let mut slno_1 = PlusMinusLindbladNoiseOperator::new();
    slno_1
        .set((dp_1.clone(), dp_1), CalculatorComplex::from(0.5))
        .unwrap();
    let dp_2: PlusMinusProduct = PlusMinusProduct::new().z(2);
    let mut slno_2 = PlusMinusLindbladNoiseOperator::new();
    slno_2
        .set((dp_2.clone(), dp_2), CalculatorComplex::from(0.5))
        .unwrap();
    assert!(slno_1 == slno);
    assert!(slno == slno_1);
    assert!(slno_2 != slno);
    assert!(slno != slno_2);
}

/// Test PlusMinusLindbladNoiseOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_json() {
    let dp = PlusMinusProduct::new().plus(0);
    let mut slno = PlusMinusLindbladNoiseOperator::new();
    slno.set((dp.clone(), dp), CalculatorComplex::from(1.0))
        .unwrap();

    let serialized = serde_json::to_string(&slno).unwrap();
    let deserialized: PlusMinusLindbladNoiseOperator = serde_json::from_str(&serialized).unwrap();
    assert_eq!(slno, deserialized);
}

/// Test PlusMinusLindbladNoiseOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_readable() {
    let dp = PlusMinusProduct::new().plus(0);
    let mut slno = PlusMinusLindbladNoiseOperator::new();
    slno.set((dp.clone(), dp), CalculatorComplex::from(1.0))
        .unwrap();

    assert_tokens(
        &slno.readable(),
        &[
            Token::Struct {
                name: "PlusMinusLindbladNoiseOperatorSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 4 },
            Token::Str("0+"),
            Token::Str("0+"),
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
            Token::Str("PlusMinusLindbladNoiseOperator"),
            Token::Str("min_version"),
            Token::Tuple { len: 3 },
            Token::U64(2),
            Token::U64(0),
            Token::U64(0),
            Token::TupleEnd,
            Token::Str("version"),
            Token::Str(STRUQTURE_VERSION),
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

#[test]
fn bincode() {
    let dp = PlusMinusProduct::new().plus(0);
    let mut slno = PlusMinusLindbladNoiseOperator::new();
    slno.set((dp.clone(), dp), CalculatorComplex::from(1.0))
        .unwrap();

    let encoded: Vec<u8> = bincode::serialize(&slno).unwrap();
    let decoded: PlusMinusLindbladNoiseOperator = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(slno, decoded);

    let encoded: Vec<u8> = bincode::serialize(&slno.clone().compact()).unwrap();
    let decoded: PlusMinusLindbladNoiseOperator = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(slno, decoded);
}

/// Test PlusMinusLindbladNoiseOperator Serialization and Deserialization traits (compact)
#[test]
fn serde_compact() {
    let dp = PlusMinusProduct::new().plus(0);
    let mut slno = PlusMinusLindbladNoiseOperator::new();
    slno.set((dp.clone(), dp), CalculatorComplex::from(1.0))
        .unwrap();

    assert_tokens(
        &slno.compact(),
        &[
            Token::Struct {
                name: "PlusMinusLindbladNoiseOperatorSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 4 },
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::U64(0),
            Token::UnitVariant {
                name: "SinglePlusMinusOperator",
                variant: "Plus",
            },
            Token::TupleEnd,
            Token::SeqEnd,
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
            Token::Str("PlusMinusLindbladNoiseOperator"),
            Token::Str("min_version"),
            Token::Tuple { len: 3 },
            Token::U64(2),
            Token::U64(0),
            Token::U64(0),
            Token::TupleEnd,
            Token::Str("version"),
            Token::Str(STRUQTURE_VERSION),
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

#[test]
fn so_from_pmo() {
    let pmp_vec: Vec<(PlusMinusProduct, CalculatorComplex)> = vec![
        (PlusMinusProduct::new().z(0), 3.0.into()),
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

    let mut qubit_op = PauliLindbladNoiseOperator::new();
    for (key_l, val_l) in dp_vec.iter() {
        for (key_r, val_r) in dp_vec.iter() {
            qubit_op
                .add_operator_product((key_l.clone(), key_r.clone()), val_l.clone() * val_r)
                .unwrap();
        }
    }

    let mut pm_op = PlusMinusLindbladNoiseOperator::new();
    for (key_l, val_l) in pmp_vec.iter() {
        for (key_r, val_r) in pmp_vec.iter() {
            pm_op
                .add_operator_product((key_l.clone(), key_r.clone()), val_l.clone() * val_r)
                .unwrap();
        }
    }

    assert_eq!(PauliLindbladNoiseOperator::from(pm_op.clone()), qubit_op);
}

#[test]
fn pmo_from_so() {
    let dp_vec: Vec<(DecoherenceProduct, CalculatorComplex)> = vec![
        (DecoherenceProduct::new().z(0), 1.0.into()),
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

    let mut qubit_op = PauliLindbladNoiseOperator::new();
    for (key_l, val_l) in dp_vec.iter() {
        for (key_r, val_r) in dp_vec.iter() {
            qubit_op
                .add_operator_product((key_l.clone(), key_r.clone()), val_l.clone() * val_r)
                .unwrap();
        }
    }

    let mut pm_op = PlusMinusLindbladNoiseOperator::new();
    for (key_l, val_l) in pmp_vec.iter() {
        for (key_r, val_r) in pmp_vec.iter() {
            pm_op
                .add_operator_product((key_l.clone(), key_r.clone()), val_l.clone() * val_r)
                .unwrap();
        }
    }

    assert_eq!(PlusMinusLindbladNoiseOperator::from(qubit_op), pm_op);
}

#[cfg(feature = "json_schema")]
#[test]
fn test_plus_minus_noise_operator_schema() {
    let mut op = PlusMinusLindbladNoiseOperator::new();
    op.set(
        (
            PlusMinusProduct::new().plus(0),
            PlusMinusProduct::new().plus(0),
        ),
        1.0.into(),
    )
    .unwrap();
    op.set(
        (
            PlusMinusProduct::new().plus(0),
            PlusMinusProduct::new().minus(1).z(2),
        ),
        "val".into(),
    )
    .unwrap();
    let schema = schemars::schema_for!(PlusMinusLindbladNoiseOperator);
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
    let pp_1 = struqture_1::spins::PlusMinusProduct::from_str("0+1-25Z").unwrap();
    let mut ss_1 = struqture_1::spins::PlusMinusLindbladNoiseOperator::new();
    struqture_1::OperateOnDensityMatrix::set(&mut ss_1, (pp_1.clone(), pp_1.clone()), 1.0.into())
        .unwrap();

    let pp_2 = PlusMinusProduct::new().plus(0).minus(1).z(25);
    let mut ss_2 = PlusMinusLindbladNoiseOperator::new();
    ss_2.set((pp_2.clone(), pp_2.clone()), 1.0.into()).unwrap();

    assert!(PlusMinusLindbladNoiseOperator::from_struqture_1(&ss_1).unwrap() == ss_2);
    assert!(ss_1 == ss_2.to_struqture_1().unwrap());
}
