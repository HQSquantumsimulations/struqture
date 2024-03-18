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

//! Integration test for public API of QubitLindbladOpenSystem

use super::create_na_matrix_from_decoherence_list;
use super::create_na_matrix_from_operator_list;
use nalgebra as na;
use num_complex::Complex64;
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde_test::{assert_tokens, Configure, Token};
use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;
use struqture::prelude::*;
use struqture::spins::{
    DecoherenceProduct, PauliProduct, QubitHamiltonian, QubitLindbladNoiseOperator,
    QubitLindbladOpenSystem,
};
use struqture::SpinIndex;
use test_case::test_case;

// Test the new function of the QubitLindbladOpenSystem
#[test]
fn new_system() {
    let system = QubitLindbladOpenSystem::new();
    assert_eq!(system.system(), &QubitHamiltonian::new());
    assert_eq!(system.noise(), &QubitLindbladNoiseOperator::new());
    assert_eq!(system.number_spins(), 0_usize)
}

// Test the new function of the QubitLindbladOpenSystem with no spins specified
#[test]
fn new_system_none() {
    let system = QubitLindbladOpenSystem::new();
    assert!(system.system().is_empty());
    assert_eq!(system.system(), &QubitHamiltonian::default());
    assert!(system.noise().is_empty());
    assert_eq!(system.noise(), &QubitLindbladNoiseOperator::default());
    assert_eq!(system.number_spins(), 0_usize);
}

// Test the group function of the QubitLindbladOpenSystem
#[test]
fn group() {
    let slos =
        QubitLindbladOpenSystem::group(QubitHamiltonian::new(), QubitLindbladNoiseOperator::new());
    assert!(slos.is_ok());
    let slos = slos.unwrap();
    assert!(slos.system().is_empty() && slos.noise().is_empty());
    assert_eq!(slos, QubitLindbladOpenSystem::default())
}

#[test]
fn empty_clone_options() {
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slos = QubitLindbladOpenSystem::new();
    slos.noise_mut()
        .set((dp_0.clone(), dp_0), CalculatorComplex::from(0.5))
        .unwrap();
}

// Test the try_set_noise and get functions of the QubitLindbladOpenSystem
#[test]
fn internal_map_set_get_system_noise() {
    let pp_0: PauliProduct = PauliProduct::new().x(0);
    let dp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slos = QubitLindbladOpenSystem::default();

    // 1) System
    slos.system_mut()
        .set(pp_0.clone(), CalculatorFloat::from(0.4))
        .unwrap();
    assert_eq!(slos.system().get(&pp_0), &CalculatorFloat::from(0.4));

    // 2) Noise
    // Vacant
    slos.noise_mut()
        .set((dp_2.clone(), dp_2.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(
        slos.noise().get(&(dp_2.clone(), dp_2)),
        &CalculatorComplex::from(0.5)
    );
    // Occupied
}

// Test the add_noise function of the QubitLindbladOpenSystem
#[test]
fn internal_map_add_system_noise() {
    let pp_0: PauliProduct = PauliProduct::new().x(0);
    let dp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slos = QubitLindbladOpenSystem::default();

    // System
    slos.system_mut()
        .add_operator_product(pp_0.clone(), CalculatorFloat::from(0.4))
        .unwrap();
    assert_eq!(slos.system().get(&pp_0), &CalculatorFloat::from(0.4));
    slos.system_mut()
        .add_operator_product(pp_0.clone(), CalculatorFloat::from(-0.4))
        .unwrap();
    assert_eq!(slos.system().get(&pp_0), &CalculatorFloat::from(0.0));

    // Noise
    slos.noise_mut()
        .add_operator_product((dp_2.clone(), dp_2.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(
        slos.noise().get(&(dp_2.clone(), dp_2.clone())),
        &CalculatorComplex::from(0.5)
    );
    slos.noise_mut()
        .add_operator_product((dp_2.clone(), dp_2.clone()), CalculatorComplex::from(-0.5))
        .unwrap();
    assert_eq!(
        slos.noise().get(&(dp_2.clone(), dp_2)),
        &CalculatorComplex::from(0.0)
    );
}

// Test the iter, keys and values functions of the QubitLindbladOpenSystem
#[test]
fn internal_map_keys() {
    let pp_0: PauliProduct = PauliProduct::new().x(0);
    let dp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slos = QubitLindbladOpenSystem::default();

    slos.system_mut()
        .set(pp_0.clone(), CalculatorFloat::from(0.5))
        .unwrap();
    slos.noise_mut()
        .set((dp_2.clone(), dp_2.clone()), CalculatorComplex::from(0.5))
        .unwrap();

    let mut map_system: BTreeMap<PauliProduct, CalculatorFloat> = BTreeMap::new();
    map_system.insert(pp_0, CalculatorFloat::from(0.5));
    let mut map_noise: BTreeMap<(DecoherenceProduct, DecoherenceProduct), CalculatorComplex> =
        BTreeMap::new();
    map_noise.insert((dp_2.clone(), dp_2), CalculatorComplex::from(0.5));

    // iter
    let dict_system = slos.system().iter();
    for (item_d, item_m) in dict_system.zip(map_system.iter()) {
        assert_eq!(item_d, item_m);
    }
    let dict_noise = slos.noise().iter();
    for (item_d, item_m) in dict_noise.zip(map_noise.iter()) {
        assert_eq!(item_d, item_m);
    }
    // keys
    let keys_system = slos.system().keys();
    for (key_s, key_m) in keys_system.zip(map_system.keys()) {
        assert_eq!(key_s, key_m);
    }
    let keys_noise = slos.noise().keys();
    for (key_s, key_m) in keys_noise.zip(map_noise.keys()) {
        assert_eq!(key_s, key_m);
    }
    // values
    let values_system = slos.system().values();
    for (val_s, val_m) in values_system.zip(map_system.values()) {
        assert_eq!(val_s, val_m);
    }
    let values_noise = slos.noise().values();
    for (val_s, val_m) in values_noise.zip(map_noise.values()) {
        assert_eq!(val_s, val_m);
    }
}

// Test the noise and system functions of the QubitLindbladOpenSystem
#[test]
fn noise_system() {
    let pp_0: PauliProduct = PauliProduct::new().x(0);
    let dp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slos = QubitLindbladOpenSystem::default();

    slos.system_mut()
        .set(pp_0.clone(), CalculatorFloat::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp_2.clone(), dp_2.clone()), CalculatorComplex::from(0.5))
        .unwrap();

    let mut system = QubitHamiltonian::new();
    system.set(pp_0, CalculatorFloat::from(0.4)).unwrap();
    let mut noise = QubitLindbladNoiseOperator::new();
    noise
        .set((dp_2.clone(), dp_2), CalculatorComplex::from(0.5))
        .unwrap();

    assert_eq!(slos.system(), &system);
    assert_eq!(slos.noise(), &noise);
}

// Test the negative operation: -QubitLindbladOpenSystem
#[test]
fn negative_slos() {
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let pp_0: PauliProduct = PauliProduct::new().x(0);
    let mut slos_0 = QubitLindbladOpenSystem::new();
    slos_0
        .system_mut()
        .set(pp_0.clone(), CalculatorFloat::from(0.4))
        .unwrap();
    slos_0
        .noise_mut()
        .set((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_0_minus = QubitLindbladOpenSystem::new();
    slos_0_minus
        .system_mut()
        .set(pp_0, CalculatorFloat::from(-0.4))
        .unwrap();
    slos_0_minus
        .noise_mut()
        .set((dp_0.clone(), dp_0), CalculatorComplex::from(-0.5))
        .unwrap();

    assert_eq!(-slos_0, slos_0_minus);
}

// Test the addition: QubitLindbladOpenSystem + QubitLindbladOpenSystem
#[test]
fn add_slos_slos() {
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let pp_0: PauliProduct = PauliProduct::new().x(0);
    let dp_1: DecoherenceProduct = DecoherenceProduct::new().x(1);
    let pp_1: PauliProduct = PauliProduct::new().z(1);
    let mut slos_0 = QubitLindbladOpenSystem::new();
    slos_0
        .system_mut()
        .set(pp_0.clone(), CalculatorFloat::from(0.4))
        .unwrap();
    slos_0
        .noise_mut()
        .set((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_1 = QubitLindbladOpenSystem::new();
    slos_1
        .system_mut()
        .set(pp_1.clone(), CalculatorFloat::from(0.4))
        .unwrap();
    slos_1
        .noise_mut()
        .set((dp_1.clone(), dp_1.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_0_1 = QubitLindbladOpenSystem::new();
    slos_0_1
        .system_mut()
        .set(pp_0, CalculatorFloat::from(0.4))
        .unwrap();
    slos_0_1
        .noise_mut()
        .set((dp_0.clone(), dp_0), CalculatorComplex::from(0.5))
        .unwrap();
    slos_0_1
        .system_mut()
        .set(pp_1, CalculatorFloat::from(0.4))
        .unwrap();
    slos_0_1
        .noise_mut()
        .set((dp_1.clone(), dp_1), CalculatorComplex::from(0.5))
        .unwrap();

    assert_eq!(slos_0 + slos_1, Ok(slos_0_1));
}

// Test the subtraction: QubitLindbladOpenSystem - QubitLindbladOpenSystem
#[test]
fn sub_slos_slos() {
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let pp_0: PauliProduct = PauliProduct::new().x(0);
    let dp_1: DecoherenceProduct = DecoherenceProduct::new().x(1);
    let pp_1: PauliProduct = PauliProduct::new().z(1);
    let mut slos_0 = QubitLindbladOpenSystem::new();
    slos_0
        .system_mut()
        .set(pp_0.clone(), CalculatorFloat::from(0.4))
        .unwrap();
    slos_0
        .noise_mut()
        .set((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_1 = QubitLindbladOpenSystem::new();
    slos_1
        .system_mut()
        .set(pp_1.clone(), CalculatorFloat::from(0.4))
        .unwrap();
    slos_1
        .noise_mut()
        .set((dp_1.clone(), dp_1.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_0_1 = QubitLindbladOpenSystem::new();
    slos_0_1
        .system_mut()
        .set(pp_0, CalculatorFloat::from(0.4))
        .unwrap();
    slos_0_1
        .noise_mut()
        .set((dp_0.clone(), dp_0), CalculatorComplex::from(0.5))
        .unwrap();
    slos_0_1
        .system_mut()
        .set(pp_1, CalculatorFloat::from(-0.4))
        .unwrap();
    slos_0_1
        .noise_mut()
        .set((dp_1.clone(), dp_1), CalculatorComplex::from(-0.5))
        .unwrap();

    assert_eq!(slos_0 - slos_1, Ok(slos_0_1));
}

// Test the multiplication: QubitLindbladOpenSystem * Calculatorcomplex
#[test]
fn mul_so_cf() {
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let pp_0: PauliProduct = PauliProduct::new().x(0);
    let mut slos_0 = QubitLindbladOpenSystem::new();
    slos_0
        .system_mut()
        .set(pp_0.clone(), CalculatorFloat::from(1.0))
        .unwrap();
    slos_0
        .noise_mut()
        .set((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_0_1 = QubitLindbladOpenSystem::new();
    slos_0_1
        .system_mut()
        .set(pp_0, CalculatorFloat::from(3.0))
        .unwrap();
    slos_0_1
        .noise_mut()
        .set((dp_0.clone(), dp_0), CalculatorComplex::from(1.5))
        .unwrap();

    assert_eq!(slos_0 * CalculatorFloat::from(3.0), slos_0_1);
}

// Test the Debug trait of QubitLindbladOpenSystem
#[test]
fn debug() {
    let pp: PauliProduct = PauliProduct::new().x(1);
    let dp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slos = QubitLindbladOpenSystem::new();
    slos.system_mut()
        .set(pp, CalculatorFloat::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(
        format!("{:?}", slos),
        "QubitLindbladOpenSystem { system: QubitHamiltonian { internal_map: {PauliProduct { items: [(1, X)] }: Float(0.4)} }, noise: QubitLindbladNoiseOperator { internal_map: {(DecoherenceProduct { items: [(0, Z)] }, DecoherenceProduct { items: [(0, Z)] }): CalculatorComplex { re: Float(0.5), im: Float(0.0) }} } }"
    );
}

// Test the Display trait of QubitLindbladOpenSystem
#[test]
fn display() {
    let pp: PauliProduct = PauliProduct::new().x(1);
    let dp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slos = QubitLindbladOpenSystem::new();
    slos.system_mut()
        .set(pp, CalculatorFloat::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    assert_eq!(
        format!("{}", slos),
        "QubitLindbladOpenSystem{\nSystem: {\n1X: 4e-1,\n}\nNoise: {\n(0Z, 0Z): (5e-1 + i * 0e0),\n}\n}"
    );
}

// Test the Clone and PartialEq traits of QubitLindbladOpenSystem
#[test]
fn clone_partial_eq() {
    let pp: PauliProduct = PauliProduct::new().x(1);
    let dp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slos = QubitLindbladOpenSystem::default();
    slos.system_mut()
        .set(pp, CalculatorFloat::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    // Test Clone trait
    assert_eq!(slos.clone(), slos);

    // Test PartialEq trait
    let pp_1: PauliProduct = PauliProduct::new().x(1);
    let dp_1: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slos_1 = QubitLindbladOpenSystem::default();
    slos_1
        .system_mut()
        .set(pp_1, CalculatorFloat::from(0.4))
        .unwrap();
    slos_1
        .noise_mut()
        .set((dp_1.clone(), dp_1), CalculatorComplex::from(0.5))
        .unwrap();
    let pp_2: PauliProduct = PauliProduct::new().x(0);
    let dp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slos_2 = QubitLindbladOpenSystem::default();
    assert!(slos_1 == slos);
    assert!(slos == slos_1);
    assert!(slos_2 != slos);
    assert!(slos != slos_2);
    slos_2
        .system_mut()
        .set(pp_2, CalculatorFloat::from(0.4))
        .unwrap();
    assert!(slos_2 != slos);
    assert!(slos != slos_2);
    slos_2
        .noise_mut()
        .set((dp_2.clone(), dp_2), CalculatorComplex::from(0.5))
        .unwrap();
    assert!(slos_2 != slos);
    assert!(slos != slos_2);
}

/// Test QubitLindbladOpenSystem Serialization and Deserialization traits (readable)
#[test]
fn serde_json() {
    let pp: PauliProduct = PauliProduct::new().x(1);
    let dp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slos = QubitLindbladOpenSystem::default();
    slos.system_mut()
        .set(pp, CalculatorFloat::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    let serialized = serde_json::to_string(&slos).unwrap();
    let deserialized: QubitLindbladOpenSystem = serde_json::from_str(&serialized).unwrap();
    assert_eq!(slos, deserialized);
}

#[test]
fn serde_readable() {
    let pp = PauliProduct::new().x(0);
    let dp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slos = QubitLindbladOpenSystem::new();
    slos.system_mut()
        .set(pp, CalculatorFloat::from(1.0))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    assert_tokens(
        &slos.readable(),
        &[
            Token::Struct {
                name: "QubitLindbladOpenSystem",
                len: 2,
            },
            Token::Str("system"),
            Token::Struct {
                name: "QubitHamiltonianSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::Str("0X"),
            Token::F64(1.0),
            Token::TupleEnd,
            Token::SeqEnd,
            Token::Str("serialisation_meta"),
            Token::Struct {
                name: "StruqtureSerialisationMeta",
                len: 3,
            },
            Token::Str("type_name"),
            Token::Str("QubitHamiltonian"),
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
            Token::Str("noise"),
            Token::Struct {
                name: "QubitLindbladNoiseOperatorSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 4 },
            Token::Str("0Z"),
            Token::Str("0Z"),
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
            Token::Str("QubitLindbladNoiseOperator"),
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
            Token::StructEnd,
        ],
    );
}

#[test]
fn bincode() {
    let pp: PauliProduct = PauliProduct::new().x(1);
    let dp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slos = QubitLindbladOpenSystem::default();
    slos.system_mut()
        .set(pp, CalculatorFloat::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    let encoded: Vec<u8> = bincode::serialize(&slos).unwrap();
    let decoded: QubitLindbladOpenSystem = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(slos, decoded);

    let encoded: Vec<u8> = bincode::serialize(&slos.clone().compact()).unwrap();
    let decoded: QubitLindbladOpenSystem = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(slos, decoded);
}

/// Test QubitLindbladOpenSystem Serialization and Deserialization traits (compact)
#[test]
fn serde_compact() {
    let pp = PauliProduct::new().x(0);
    let dp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slos = QubitLindbladOpenSystem::new();
    slos.system_mut()
        .set(pp, CalculatorFloat::from(1.0))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    assert_tokens(
        &slos.compact(),
        &[
            Token::Struct {
                name: "QubitLindbladOpenSystem",
                len: 2,
            },
            Token::Str("system"),
            Token::Struct {
                name: "QubitHamiltonianSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
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
            Token::F64(1.0),
            Token::TupleEnd,
            Token::SeqEnd,
            Token::Str("serialisation_meta"),
            Token::Struct {
                name: "StruqtureSerialisationMeta",
                len: 3,
            },
            Token::Str("type_name"),
            Token::Str("QubitHamiltonian"),
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
            Token::Str("noise"),
            Token::Struct {
                name: "QubitLindbladNoiseOperatorSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 4 },
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::U64(0),
            Token::UnitVariant {
                name: "SingleDecoherenceOperator",
                variant: "Z",
            },
            Token::TupleEnd,
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::U64(0),
            Token::UnitVariant {
                name: "SingleDecoherenceOperator",
                variant: "Z",
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
            Token::Str("QubitLindbladNoiseOperator"),
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
            Token::StructEnd,
        ],
    );
}

#[test_case("0Z", "0Z", &["Z"], &["Z"]; "0Z")]
#[test_case("1X", "1X", &["X", "I"], &["X", "I"]; "1X1X")]
#[test_case("0iY", "0iY", &["iY"], &["iY"]; "0m0m")]
#[test_case("0X", "0iY", &["X"], &["iY"]; "0p0m")]
#[test_case("1X", "1iY", &["X", "I"], &["iY", "I"]; "1p1m")]
fn test_superoperator_noisy(
    left_representation: &str,
    right_representation: &str,
    left_operators: &[&str],
    right_operators: &[&str],
) {
    let mut system = QubitLindbladOpenSystem::new();
    let left: DecoherenceProduct = DecoherenceProduct::from_str(left_representation).unwrap();
    let right: DecoherenceProduct = DecoherenceProduct::from_str(right_representation).unwrap();

    let _ = system.noise_mut().set((left, right), 1.0.into());

    let dimension = 4_usize.pow(left_operators.len() as u32);

    // Constructing matrix by hand:

    let identities: Vec<&str> = (0..left_operators.len()).map(|_| "I").collect();

    let i = create_na_matrix_from_decoherence_list(&identities);
    let l_left = create_na_matrix_from_decoherence_list(left_operators);
    let l_right = create_na_matrix_from_decoherence_list(right_operators).transpose();

    let product = l_right.clone() * l_left.clone();

    let test_matrix = l_left.kronecker(&l_right.transpose())
        - (product.kronecker(&i) + i.kronecker(&product.transpose())) * Complex64::new(0.5, 0.0);

    let second_test_matrix = system.sparse_matrix_superoperator(None).unwrap();
    let (test_vals, (test_rows, test_columns)) =
        system.sparse_matrix_superoperator_coo(None).unwrap();
    for (second_val, (row, column)) in test_vals
        .iter()
        .zip(test_rows.iter().zip(test_columns.iter()))
    {
        let val = test_matrix[(*row, *column)];
        assert_eq!(&val, second_val);
    }
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
                    assert_eq!(val, 0.0.into())
                }
            }
        }
    }

    let full_result = system.sparse_lindblad_entries().unwrap();
    let coo_test_matrix = full_result[0].clone().0;
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
            let second_val = second_test_matrix.get(&key);

            match second_val {
                Some(x) => assert_eq!(&val, x),
                None => {
                    assert_eq!(val, 0.0.into())
                }
            }
        }
    }
}

#[test_case("0Z", &["Z"]; "0Z")]
#[test_case("0I", &["I"]; "0I")]
#[test_case("1X", &["X", "I"]; "1X")]
#[test_case("1Y", &["Y", "I"]; "1Y")]
#[test_case("0Z1X", &["X", "Z"]; "0Z1X")]
#[test_case("0X1X", &["X", "X"]; "0X1X")]
#[test_case("0X1Y", &["Y", "X"]; "0X1Y")]
#[test_case("0X2Y", &["Y", "I","X"]; "0X2Y")]
fn test_superoperator_hamiltonian(pauli_representation: &str, pauli_operators: &[&str]) {
    let mut system = QubitLindbladOpenSystem::new();
    let pp: PauliProduct = PauliProduct::from_str(pauli_representation).unwrap();

    system.system_mut().set(pp, 1.0.into()).unwrap();

    let dimension = 4_usize.pow(pauli_operators.len() as u32);

    // Constructing matrix by hand:
    let cc0 = Complex64::new(0.0, 0.0);
    let cci = Complex64::new(0.0, 1.0);

    let identities: Vec<&str> = (0..pauli_operators.len()).map(|_| "I").collect();

    let i = create_na_matrix_from_operator_list(&identities);
    let h = create_na_matrix_from_operator_list(pauli_operators);

    let test_matrix = (h.kronecker(&i) - i.kronecker(&h.transpose())) * (-cci);

    let second_test_matrix = system.sparse_matrix_superoperator(None).unwrap();
    let (test_vals, (test_rows, test_columns)) =
        system.sparse_matrix_superoperator_coo(None).unwrap();
    for (second_val, (row, column)) in test_vals
        .iter()
        .zip(test_rows.iter().zip(test_columns.iter()))
    {
        let val = test_matrix[(*row, *column)];
        assert_eq!(&val, second_val);
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
}

#[test]
fn test_superoperator_hamiltonian_and_noise() {
    let mut system = QubitLindbladOpenSystem::new();
    let pp: PauliProduct = PauliProduct::from_str("0Z").unwrap();

    system.system_mut().set(pp, 1.0.into()).unwrap();

    let left: DecoherenceProduct = DecoherenceProduct::from_str("0Z").unwrap();
    let right: DecoherenceProduct = DecoherenceProduct::from_str("0Z").unwrap();

    let _ = system.noise_mut().set((left, right), 1.0.into());

    let dimension = 4_usize.pow(1_u32);

    // Constructing matrix by hand:
    let cc0 = Complex64::new(0.0, 0.0);
    let cci = Complex64::new(0.0, 1.0);

    let identities: Vec<&str> = (0..1).map(|_| "I").collect();

    let i = create_na_matrix_from_operator_list(&identities);
    let h = create_na_matrix_from_operator_list(&["Z"]);

    let test_matrix = (h.kronecker(&i) - i.kronecker(&h.transpose())) * (-cci);

    let identities: Vec<&str> = (0..1).map(|_| "I").collect();

    let i = create_na_matrix_from_decoherence_list(&identities);
    let l_left = create_na_matrix_from_decoherence_list(&["Z"]);
    let l_right = create_na_matrix_from_decoherence_list(&["Z"]).transpose();

    let product = l_right.clone() * l_left.clone();

    let test_matrix2 = l_left.kronecker(&l_right.transpose())
        - (product.kronecker(&i) + i.kronecker(&product.transpose())) * Complex64::new(0.5, 0.0);

    let test_matrix = test_matrix + test_matrix2;

    let second_test_matrix = system.sparse_matrix_superoperator(None).unwrap();
    let (test_vals, (test_rows, test_columns)) =
        system.sparse_matrix_superoperator_coo(None).unwrap();
    for (second_val, (row, column)) in test_vals
        .iter()
        .zip(test_rows.iter().zip(test_columns.iter()))
    {
        let val = test_matrix[(*row, *column)];
        assert_eq!(&val, second_val);
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
}

#[test_case("0Z", &["Z"]; "0Z")]
#[test_case("1X", &["X", "I"]; "1X")]
#[test_case("1Y", &["Y", "I"]; "1Y")]
#[test_case("0Z1X", &["X", "Z"]; "0Z1X")]
#[test_case("0X1X", &["X", "X"]; "0X1X")]
#[test_case("0X1Y", &["Y", "X"]; "0X1Y")]
#[test_case("0X2Y", &["Y", "I","X"]; "0X2Y")]
fn test_operator(pauli_representation: &str, pauli_operators: &[&str]) {
    let mut system = QubitLindbladOpenSystem::new();
    let pp: PauliProduct = PauliProduct::from_str(pauli_representation).unwrap();

    system.system_mut().set(pp, 1.0.into()).unwrap();

    let dimension = 2_usize.pow(pauli_operators.len() as u32);

    // Constructing matrix by hand:
    let cc0 = Complex64::new(0.0, 0.0);

    let h = create_na_matrix_from_operator_list(pauli_operators);

    let test_matrix = h;

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
fn test_truncate() {
    let mut system = QubitLindbladOpenSystem::new();
    system
        .system_mut()
        .set(PauliProduct::from_str("0X").unwrap(), 1.0.into())
        .unwrap();
    system
        .system_mut()
        .set(PauliProduct::from_str("1Y").unwrap(), 0.1.into())
        .unwrap();
    system
        .system_mut()
        .set(PauliProduct::from_str("2Z").unwrap(), 0.01.into())
        .unwrap();
    system
        .system_mut()
        .set(PauliProduct::from_str("0X1Z").unwrap(), "test".into())
        .unwrap();

    let _ = system.noise_mut().set(
        (
            DecoherenceProduct::from_str("0X").unwrap(),
            DecoherenceProduct::from_str("0X").unwrap(),
        ),
        "test".into(),
    );
    let _ = system.noise_mut().set(
        (
            DecoherenceProduct::from_str("1X").unwrap(),
            DecoherenceProduct::from_str("1X").unwrap(),
        ),
        1.0.into(),
    );
    let _ = system.noise_mut().set(
        (
            DecoherenceProduct::from_str("2X").unwrap(),
            DecoherenceProduct::from_str("2X").unwrap(),
        ),
        0.1.into(),
    );
    let _ = system.noise_mut().set(
        (
            DecoherenceProduct::from_str("3X").unwrap(),
            DecoherenceProduct::from_str("3X").unwrap(),
        ),
        0.01.into(),
    );

    let mut test_system1 = QubitLindbladOpenSystem::new();
    test_system1
        .system_mut()
        .set(PauliProduct::from_str("0X").unwrap(), 1.0.into())
        .unwrap();
    test_system1
        .system_mut()
        .set(PauliProduct::from_str("1Y").unwrap(), 0.1.into())
        .unwrap();
    test_system1
        .system_mut()
        .set(PauliProduct::from_str("0X1Z").unwrap(), "test".into())
        .unwrap();
    let _ = test_system1.noise_mut().set(
        (
            DecoherenceProduct::from_str("0X").unwrap(),
            DecoherenceProduct::from_str("0X").unwrap(),
        ),
        "test".into(),
    );
    let _ = test_system1.noise_mut().set(
        (
            DecoherenceProduct::from_str("1X").unwrap(),
            DecoherenceProduct::from_str("1X").unwrap(),
        ),
        1.0.into(),
    );
    let _ = test_system1.noise_mut().set(
        (
            DecoherenceProduct::from_str("2X").unwrap(),
            DecoherenceProduct::from_str("2X").unwrap(),
        ),
        0.1.into(),
    );

    let mut test_system2 = QubitLindbladOpenSystem::new();
    test_system2
        .system_mut()
        .set(PauliProduct::from_str("0X").unwrap(), 1.0.into())
        .unwrap();
    test_system2
        .system_mut()
        .set(PauliProduct::from_str("0X1Z").unwrap(), "test".into())
        .unwrap();
    let _ = test_system2.noise_mut().set(
        (
            DecoherenceProduct::from_str("0X").unwrap(),
            DecoherenceProduct::from_str("0X").unwrap(),
        ),
        "test".into(),
    );
    let _ = test_system2.noise_mut().set(
        (
            DecoherenceProduct::from_str("1X").unwrap(),
            DecoherenceProduct::from_str("1X").unwrap(),
        ),
        1.0.into(),
    );

    let comparison_system1 = system.truncate(0.05);
    assert_eq!(test_system1, comparison_system1);
    let comparison_system2 = system.truncate(0.5);
    assert_eq!(test_system2, comparison_system2);
}

#[cfg(feature = "json_schema")]
#[test]
fn test_spin_noise_system_schema() {
    let mut op = QubitLindbladOpenSystem::new();
    let pp: PauliProduct = PauliProduct::new().x(1);
    let dp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    op.system_mut().set(pp, CalculatorFloat::from(0.4)).unwrap();
    op.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();
    let schema = schemars::schema_for!(QubitLindbladOpenSystem);
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
    let pp_1 = struqture_one::spins::PauliProduct::from_str("0X1Y3Z").unwrap();
    let dp_1 = struqture_one::spins::DecoherenceProduct::from_str("0X1iY25Z").unwrap();
    let mut ss_1 = struqture_one::spins::SpinLindbladOpenSystem::new(None);
    let system_mut_1 = struqture_one::OpenSystem::system_mut(&mut ss_1);
    struqture_one::OperateOnDensityMatrix::set(system_mut_1, pp_1.clone(), 2.0.into()).unwrap();
    let noise_mut_1 = struqture_one::OpenSystem::noise_mut(&mut ss_1);
    struqture_one::OperateOnDensityMatrix::set(
        noise_mut_1,
        (dp_1.clone(), dp_1.clone()),
        1.0.into(),
    )
    .unwrap();

    let pp_2 = PauliProduct::new().x(0).y(1).z(3);
    let dp_2 = DecoherenceProduct::new().x(0).iy(1).z(25);
    let mut ss_2 = QubitLindbladOpenSystem::new();
    ss_2.system_mut().set(pp_2.clone(), 2.0.into()).unwrap();
    ss_2.noise_mut()
        .set((dp_2.clone(), dp_2.clone()), 1.0.into())
        .unwrap();

    assert!(QubitLindbladOpenSystem::from_struqture_1(&ss_1).unwrap() == ss_2);
    assert!(ss_1 == ss_2.to_struqture_1().unwrap());
}
