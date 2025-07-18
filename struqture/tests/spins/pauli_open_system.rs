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

//! Integration test for public API of PauliLindbladOpenSystem

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
    DecoherenceProduct, PauliHamiltonian, PauliLindbladNoiseOperator, PauliLindbladOpenSystem,
    PauliProduct,
};
use struqture::SpinIndex;
use struqture::STRUQTURE_VERSION;
use test_case::test_case;

// Test the new function of the PauliLindbladOpenSystem
#[test]
fn new_system() {
    let system = PauliLindbladOpenSystem::new();
    assert_eq!(system.system(), &PauliHamiltonian::new());
    assert_eq!(system.noise(), &PauliLindbladNoiseOperator::new());
    assert_eq!(system.current_number_spins(), 0_usize)
}

// Test the new function of the PauliLindbladOpenSystem with no spins specified
#[test]
fn new_system_none() {
    let system = PauliLindbladOpenSystem::new();
    assert!(system.system().is_empty());
    assert_eq!(system.system(), &PauliHamiltonian::default());
    assert!(system.noise().is_empty());
    assert_eq!(system.noise(), &PauliLindbladNoiseOperator::default());
    assert_eq!(system.current_number_spins(), 0_usize);
}

// Test the group function of the PauliLindbladOpenSystem
#[test]
fn group() {
    let slos =
        PauliLindbladOpenSystem::group(PauliHamiltonian::new(), PauliLindbladNoiseOperator::new());
    assert!(slos.is_ok());
    let slos = slos.unwrap();
    assert!(slos.system().is_empty() && slos.noise().is_empty());
    assert_eq!(slos, PauliLindbladOpenSystem::default())
}

#[test]
fn empty_clone_options() {
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slos = PauliLindbladOpenSystem::new();
    slos.noise_mut()
        .set((dp_0.clone(), dp_0), CalculatorComplex::from(0.5))
        .unwrap();
}

// Test the try_set_noise and get functions of the PauliLindbladOpenSystem
#[test]
fn internal_map_set_get_system_noise() {
    let pp_0: PauliProduct = PauliProduct::new().x(0);
    let dp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slos = PauliLindbladOpenSystem::default();

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

// Test the add_noise function of the PauliLindbladOpenSystem
#[test]
fn internal_map_add_system_noise() {
    let pp_0: PauliProduct = PauliProduct::new().x(0);
    let dp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slos = PauliLindbladOpenSystem::default();

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

// Test the iter, keys and values functions of the PauliLindbladOpenSystem
#[test]
fn internal_map_keys() {
    let pp_0: PauliProduct = PauliProduct::new().x(0);
    let dp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slos = PauliLindbladOpenSystem::default();

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

// Test the noise and system functions of the PauliLindbladOpenSystem
#[test]
fn noise_system() {
    let pp_0: PauliProduct = PauliProduct::new().x(0);
    let dp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
    let mut slos = PauliLindbladOpenSystem::default();

    slos.system_mut()
        .set(pp_0.clone(), CalculatorFloat::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp_2.clone(), dp_2.clone()), CalculatorComplex::from(0.5))
        .unwrap();

    let mut system = PauliHamiltonian::new();
    system.set(pp_0, CalculatorFloat::from(0.4)).unwrap();
    let mut noise = PauliLindbladNoiseOperator::new();
    noise
        .set((dp_2.clone(), dp_2), CalculatorComplex::from(0.5))
        .unwrap();

    assert_eq!(slos.system(), &system);
    assert_eq!(slos.noise(), &noise);
}

// Test the negative operation: -PauliLindbladOpenSystem
#[test]
fn negative_slos() {
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let pp_0: PauliProduct = PauliProduct::new().x(0);
    let mut slos_0 = PauliLindbladOpenSystem::new();
    slos_0
        .system_mut()
        .set(pp_0.clone(), CalculatorFloat::from(0.4))
        .unwrap();
    slos_0
        .noise_mut()
        .set((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_0_minus = PauliLindbladOpenSystem::new();
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

// Test the addition: PauliLindbladOpenSystem + PauliLindbladOpenSystem
#[test]
fn add_slos_slos() {
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let pp_0: PauliProduct = PauliProduct::new().x(0);
    let dp_1: DecoherenceProduct = DecoherenceProduct::new().x(1);
    let pp_1: PauliProduct = PauliProduct::new().z(1);
    let mut slos_0 = PauliLindbladOpenSystem::new();
    slos_0
        .system_mut()
        .set(pp_0.clone(), CalculatorFloat::from(0.4))
        .unwrap();
    slos_0
        .noise_mut()
        .set((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_1 = PauliLindbladOpenSystem::new();
    slos_1
        .system_mut()
        .set(pp_1.clone(), CalculatorFloat::from(0.4))
        .unwrap();
    slos_1
        .noise_mut()
        .set((dp_1.clone(), dp_1.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_0_1 = PauliLindbladOpenSystem::new();
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

// Test the subtraction: PauliLindbladOpenSystem - PauliLindbladOpenSystem
#[test]
fn sub_slos_slos() {
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let pp_0: PauliProduct = PauliProduct::new().x(0);
    let dp_1: DecoherenceProduct = DecoherenceProduct::new().x(1);
    let pp_1: PauliProduct = PauliProduct::new().z(1);
    let mut slos_0 = PauliLindbladOpenSystem::new();
    slos_0
        .system_mut()
        .set(pp_0.clone(), CalculatorFloat::from(0.4))
        .unwrap();
    slos_0
        .noise_mut()
        .set((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_1 = PauliLindbladOpenSystem::new();
    slos_1
        .system_mut()
        .set(pp_1.clone(), CalculatorFloat::from(0.4))
        .unwrap();
    slos_1
        .noise_mut()
        .set((dp_1.clone(), dp_1.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_0_1 = PauliLindbladOpenSystem::new();
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

// Test the multiplication: PauliLindbladOpenSystem * Calculatorcomplex
#[test]
fn mul_so_cf() {
    let dp_0: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let pp_0: PauliProduct = PauliProduct::new().x(0);
    let mut slos_0 = PauliLindbladOpenSystem::new();
    slos_0
        .system_mut()
        .set(pp_0.clone(), CalculatorFloat::from(1.0))
        .unwrap();
    slos_0
        .noise_mut()
        .set((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_0_1 = PauliLindbladOpenSystem::new();
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

// Test the Debug trait of PauliLindbladOpenSystem
#[test]
fn debug() {
    let pp: PauliProduct = PauliProduct::new().x(1);
    let dp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slos = PauliLindbladOpenSystem::new();
    slos.system_mut()
        .set(pp, CalculatorFloat::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(
        format!("{slos:?}"),
        "PauliLindbladOpenSystem { system: PauliHamiltonian { internal_map: {PauliProduct { items: [(1, X)] }: Float(0.4)} }, noise: PauliLindbladNoiseOperator { internal_map: {(DecoherenceProduct { items: [(0, Z)] }, DecoherenceProduct { items: [(0, Z)] }): CalculatorComplex { re: Float(0.5), im: Float(0.0) }} } }"
    );
}

// Test the Display trait of PauliLindbladOpenSystem
#[test]
fn display() {
    let pp: PauliProduct = PauliProduct::new().x(1);
    let dp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slos = PauliLindbladOpenSystem::new();
    slos.system_mut()
        .set(pp, CalculatorFloat::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    assert_eq!(
        format!("{slos}"),
        "PauliLindbladOpenSystem{\nSystem: {\n1X: 4e-1,\n}\nNoise: {\n(0Z, 0Z): (5e-1 + i * 0e0),\n}\n}"
    );
}

// Test the Clone and PartialEq traits of PauliLindbladOpenSystem
#[test]
fn clone_partial_eq() {
    let pp: PauliProduct = PauliProduct::new().x(1);
    let dp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slos = PauliLindbladOpenSystem::default();
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
    let mut slos_1 = PauliLindbladOpenSystem::default();
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
    let mut slos_2 = PauliLindbladOpenSystem::default();
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

/// Test PauliLindbladOpenSystem Serialization and Deserialization traits (readable)
#[test]
fn serde_json() {
    let pp: PauliProduct = PauliProduct::new().x(1);
    let dp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slos = PauliLindbladOpenSystem::default();
    slos.system_mut()
        .set(pp, CalculatorFloat::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    let serialized = serde_json::to_string(&slos).unwrap();
    let deserialized: PauliLindbladOpenSystem = serde_json::from_str(&serialized).unwrap();
    assert_eq!(slos, deserialized);
}

#[test]
fn serde_readable() {
    let pp = PauliProduct::new().x(0);
    let dp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slos = PauliLindbladOpenSystem::new();
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
                name: "PauliLindbladOpenSystem",
                len: 2,
            },
            Token::Str("system"),
            Token::Struct {
                name: "PauliHamiltonianSerialize",
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
            Token::Str("PauliHamiltonian"),
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
            Token::Str("noise"),
            Token::Struct {
                name: "PauliLindbladNoiseOperatorSerialize",
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
            Token::Str("PauliLindbladNoiseOperator"),
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
            Token::StructEnd,
        ],
    );
}

#[test]
fn bincode() {
    let pp: PauliProduct = PauliProduct::new().x(1);
    let dp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slos = PauliLindbladOpenSystem::default();
    slos.system_mut()
        .set(pp, CalculatorFloat::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    let config = bincode::config::legacy();
    let serialized = bincode::serde::encode_to_vec(&slos, config).unwrap();
    let (deserialized, _len): (PauliLindbladOpenSystem, usize) =
        bincode::serde::decode_from_slice(&serialized, config).unwrap();
    assert_eq!(deserialized, slos);

    let encoded: Vec<u8> = bincode::serde::encode_to_vec(&slos.clone().compact(), config).unwrap();
    let (decoded, _len): (PauliLindbladOpenSystem, usize) =
        bincode::serde::decode_from_slice(&encoded, config).unwrap();
    assert_eq!(slos, decoded);
}

/// Test PauliLindbladOpenSystem Serialization and Deserialization traits (compact)
#[test]
fn serde_compact() {
    let pp = PauliProduct::new().x(0);
    let dp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    let mut slos = PauliLindbladOpenSystem::new();
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
                name: "PauliLindbladOpenSystem",
                len: 2,
            },
            Token::Str("system"),
            Token::Struct {
                name: "PauliHamiltonianSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::U64(0),
            Token::UnitVariant {
                name: "SinglePauliOperator",
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
            Token::Str("PauliHamiltonian"),
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
            Token::Str("noise"),
            Token::Struct {
                name: "PauliLindbladNoiseOperatorSerialize",
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
            Token::Str("PauliLindbladNoiseOperator"),
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
    let mut system = PauliLindbladOpenSystem::new();
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

    let second_test_matrix = system
        .sparse_matrix_superoperator(system.current_number_spins())
        .unwrap();
    let (test_vals, (test_rows, test_columns)) = system
        .sparse_matrix_superoperator_coo(system.current_number_spins())
        .unwrap();
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
    let mut system = PauliLindbladOpenSystem::new();
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

    let second_test_matrix = system
        .sparse_matrix_superoperator(system.current_number_spins())
        .unwrap();
    let (test_vals, (test_rows, test_columns)) = system
        .sparse_matrix_superoperator_coo(system.current_number_spins())
        .unwrap();
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
    let mut system = PauliLindbladOpenSystem::new();
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

    let second_test_matrix = system
        .sparse_matrix_superoperator(system.current_number_spins())
        .unwrap();
    let (test_vals, (test_rows, test_columns)) = system
        .sparse_matrix_superoperator_coo(system.current_number_spins())
        .unwrap();
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
fn test_truncate() {
    let mut system = PauliLindbladOpenSystem::new();
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

    let mut test_system1 = PauliLindbladOpenSystem::new();
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

    let mut test_system2 = PauliLindbladOpenSystem::new();
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
fn test_noise_system_schema() {
    let mut op = PauliLindbladOpenSystem::new();
    let pp: PauliProduct = PauliProduct::new().x(1);
    let dp: DecoherenceProduct = DecoherenceProduct::new().z(0);
    op.system_mut().set(pp, CalculatorFloat::from(0.4)).unwrap();
    op.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();
    let schema = schemars::schema_for!(PauliLindbladOpenSystem);
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
    let pp_1 = struqture_1::spins::PauliProduct::from_str("0X1Y3Z").unwrap();
    let dp_1 = struqture_1::spins::DecoherenceProduct::from_str("0X1iY25Z").unwrap();
    let mut ss_1 = struqture_1::spins::SpinLindbladOpenSystem::new(None);
    let system_mut_1 = struqture_1::OpenSystem::system_mut(&mut ss_1);
    struqture_1::OperateOnDensityMatrix::set(system_mut_1, pp_1.clone(), 2.0.into()).unwrap();
    let noise_mut_1 = struqture_1::OpenSystem::noise_mut(&mut ss_1);
    struqture_1::OperateOnDensityMatrix::set(noise_mut_1, (dp_1.clone(), dp_1.clone()), 1.0.into())
        .unwrap();

    let pp_2 = PauliProduct::new().x(0).y(1).z(3);
    let dp_2 = DecoherenceProduct::new().x(0).iy(1).z(25);
    let mut ss_2 = PauliLindbladOpenSystem::new();
    ss_2.system_mut().set(pp_2.clone(), 2.0.into()).unwrap();
    ss_2.noise_mut()
        .set((dp_2.clone(), dp_2.clone()), 1.0.into())
        .unwrap();

    assert!(PauliLindbladOpenSystem::from_struqture_1(&ss_1).unwrap() == ss_2);
    assert!(ss_1 == ss_2.to_struqture_1().unwrap());
}
