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

//! Integration test for public API of BosonLindbladOpenSystem

use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde_test::{assert_tokens, Configure, Token};
use std::collections::BTreeMap;
use struqture::bosons::{
    BosonHamiltonianSystem, BosonLindbladNoiseSystem, BosonLindbladOpenSystem, BosonProduct,
    HermitianBosonProduct,
};
use struqture::prelude::*;
use struqture::ModeIndex;
#[cfg(feature = "json_schema")]
use test_case::test_case;

// Test the new function of the BosonLindbladOpenSystem
#[test]
fn new_system() {
    let system = BosonLindbladOpenSystem::new(Some(1));
    assert_eq!(system.system(), &BosonHamiltonianSystem::new(Some(1)));
    assert_eq!(system.noise(), &BosonLindbladNoiseSystem::new(Some(1)));
    assert_eq!(system.number_modes(), 1_usize);
    assert_eq!(system.current_number_modes(), 1_usize);
}

// Test the new function of the BosonLindbladOpenSystem with no modes specified
#[test]
fn new_system_none() {
    let system = BosonLindbladOpenSystem::new(None);
    assert!(system.system().is_empty());
    assert_eq!(system.system(), &BosonHamiltonianSystem::default());
    assert!(system.noise().is_empty());
    assert_eq!(system.noise(), &BosonLindbladNoiseSystem::default());
    assert_eq!(system.number_modes(), 0_usize);
    assert_eq!(system.current_number_modes(), 0_usize);
}

// Test the group function of the BosonLindbladOpenSystem
#[test]
fn group() {
    let slos = BosonLindbladOpenSystem::group(
        BosonHamiltonianSystem::new(None),
        BosonLindbladNoiseSystem::new(None),
    );
    assert!(slos.is_ok());
    let slos = slos.unwrap();
    assert!(slos.system().is_empty() && slos.noise().is_empty());
    assert_eq!(slos, BosonLindbladOpenSystem::default())
}

#[test]
fn group_with_none() {
    let blos = BosonLindbladOpenSystem::group(
        BosonHamiltonianSystem::new(None),
        BosonLindbladNoiseSystem::new(Some(2)),
    );

    assert!(blos.is_ok());
    let os = blos.unwrap();
    let (system, noise) = os.ungroup();

    assert_eq!(noise.number_modes(), 2);
    assert_eq!(system.number_modes(), 2);

    let blos = BosonLindbladOpenSystem::group(
        BosonHamiltonianSystem::new(Some(2)),
        BosonLindbladNoiseSystem::new(None),
    );

    assert!(blos.is_ok());
    let os = blos.unwrap();
    let (system, noise) = os.ungroup();

    assert_eq!(noise.number_modes(), 2);
    assert_eq!(system.number_modes(), 2);
}

// Test the group function of the BosonLindbladOpenSystem
#[test]
fn group_failing() {
    let slos = BosonLindbladOpenSystem::group(
        BosonHamiltonianSystem::new(Some(3)),
        BosonLindbladNoiseSystem::new(Some(2)),
    );
    assert!(slos.is_err());
}

#[test]
fn empty_clone_options() {
    let dp_0: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut slos = BosonLindbladOpenSystem::new(Some(3));
    slos.noise_mut()
        .set((dp_0.clone(), dp_0), CalculatorComplex::from(0.5))
        .unwrap();

    let full: Option<usize> = Some(3);
    assert_eq!(slos.empty_clone(), BosonLindbladOpenSystem::new(full));
}

// Test the try_set_noise and get functions of the BosonLindbladOpenSystem
#[test]
fn internal_map_set_get_system_noise() {
    let pp_0: HermitianBosonProduct = HermitianBosonProduct::new([], [0]).unwrap();
    let dp_2: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut slos = BosonLindbladOpenSystem::default();

    // 1) System
    slos.system_mut()
        .set(pp_0.clone(), CalculatorComplex::from(0.4))
        .unwrap();
    assert_eq!(slos.system().get(&pp_0), &CalculatorComplex::from(0.4));

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

// Test the add_noise function of the BosonLindbladOpenSystem
#[test]
fn internal_map_add_system_noise() {
    let pp_0: HermitianBosonProduct = HermitianBosonProduct::new([], [0]).unwrap();
    let dp_2: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut slos = BosonLindbladOpenSystem::default();

    // System
    slos.system_mut()
        .add_operator_product(pp_0.clone(), CalculatorComplex::from(0.4))
        .unwrap();
    assert_eq!(slos.system().get(&pp_0), &CalculatorComplex::from(0.4));
    slos.system_mut()
        .add_operator_product(pp_0.clone(), CalculatorComplex::from(-0.4))
        .unwrap();
    assert_eq!(slos.system().get(&pp_0), &CalculatorComplex::from(0.0));

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

// Test the iter, keys and values functions of the BosonLindbladOpenSystem
#[test]
fn internal_map_keys() {
    let pp_0: HermitianBosonProduct = HermitianBosonProduct::new([], [0]).unwrap();
    let dp_2: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut slos = BosonLindbladOpenSystem::default();

    slos.system_mut()
        .set(pp_0.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    slos.noise_mut()
        .set((dp_2.clone(), dp_2.clone()), CalculatorComplex::from(0.5))
        .unwrap();

    let mut map_system: BTreeMap<HermitianBosonProduct, CalculatorComplex> = BTreeMap::new();
    map_system.insert(pp_0, CalculatorComplex::from(0.5));
    let mut map_noise: BTreeMap<(BosonProduct, BosonProduct), CalculatorComplex> = BTreeMap::new();
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

// Test the noise and system functions of the BosonLindbladOpenSystem
#[test]
fn noise_system() {
    let pp_0: HermitianBosonProduct = HermitianBosonProduct::new([], [0]).unwrap();
    let dp_2: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut slos = BosonLindbladOpenSystem::default();

    slos.system_mut()
        .set(pp_0.clone(), CalculatorComplex::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp_2.clone(), dp_2.clone()), CalculatorComplex::from(0.5))
        .unwrap();

    let mut system = BosonHamiltonianSystem::new(None);
    system.set(pp_0, CalculatorComplex::from(0.4)).unwrap();
    let mut noise = BosonLindbladNoiseSystem::new(None);
    noise
        .set((dp_2.clone(), dp_2), CalculatorComplex::from(0.5))
        .unwrap();

    assert_eq!(slos.system(), &system);
    assert_eq!(slos.noise(), &noise);
}

// Test the negative operation: -BosonLindbladOpenSystem
#[test]
fn negative_slos() {
    let dp_0: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let pp_0: HermitianBosonProduct = HermitianBosonProduct::new([], [0]).unwrap();
    let mut slos_0 = BosonLindbladOpenSystem::new(Some(1));
    slos_0
        .system_mut()
        .set(pp_0.clone(), CalculatorComplex::from(0.4))
        .unwrap();
    slos_0
        .noise_mut()
        .set((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_0_minus = BosonLindbladOpenSystem::new(Some(1));
    slos_0_minus
        .system_mut()
        .set(pp_0, CalculatorComplex::from(-0.4))
        .unwrap();
    slos_0_minus
        .noise_mut()
        .set((dp_0.clone(), dp_0), CalculatorComplex::from(-0.5))
        .unwrap();

    assert_eq!(-slos_0, slos_0_minus);
}

// Test the addition: BosonLindbladOpenSystem + BosonLindbladOpenSystem
#[test]
fn add_slos_slos() {
    let dp_0: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let pp_0: HermitianBosonProduct = HermitianBosonProduct::new([], [0]).unwrap();
    let dp_1: BosonProduct = BosonProduct::new([0], [1]).unwrap();
    let pp_1: HermitianBosonProduct = HermitianBosonProduct::new([0], [1]).unwrap();
    let mut slos_0 = BosonLindbladOpenSystem::new(Some(2));
    slos_0
        .system_mut()
        .set(pp_0.clone(), CalculatorComplex::from(0.4))
        .unwrap();
    slos_0
        .noise_mut()
        .set((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_1 = BosonLindbladOpenSystem::new(Some(2));
    slos_1
        .system_mut()
        .set(pp_1.clone(), CalculatorComplex::from(0.4))
        .unwrap();
    slos_1
        .noise_mut()
        .set((dp_1.clone(), dp_1.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_0_1 = BosonLindbladOpenSystem::new(Some(2));
    slos_0_1
        .system_mut()
        .set(pp_0, CalculatorComplex::from(0.4))
        .unwrap();
    slos_0_1
        .noise_mut()
        .set((dp_0.clone(), dp_0), CalculatorComplex::from(0.5))
        .unwrap();
    slos_0_1
        .system_mut()
        .set(pp_1, CalculatorComplex::from(0.4))
        .unwrap();
    slos_0_1
        .noise_mut()
        .set((dp_1.clone(), dp_1), CalculatorComplex::from(0.5))
        .unwrap();

    assert_eq!(slos_0 + slos_1, Ok(slos_0_1));
}

// Test the subtraction: BosonLindbladOpenSystem - BosonLindbladOpenSystem
#[test]
fn sub_slos_slos() {
    let dp_0: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let pp_0: HermitianBosonProduct = HermitianBosonProduct::new([], [0]).unwrap();
    let dp_1: BosonProduct = BosonProduct::new([0], [1]).unwrap();
    let pp_1: HermitianBosonProduct = HermitianBosonProduct::new([0], [1]).unwrap();
    let mut slos_0 = BosonLindbladOpenSystem::new(Some(2));
    slos_0
        .system_mut()
        .set(pp_0.clone(), CalculatorComplex::from(0.4))
        .unwrap();
    slos_0
        .noise_mut()
        .set((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_1 = BosonLindbladOpenSystem::new(Some(2));
    slos_1
        .system_mut()
        .set(pp_1.clone(), CalculatorComplex::from(0.4))
        .unwrap();
    slos_1
        .noise_mut()
        .set((dp_1.clone(), dp_1.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_0_1 = BosonLindbladOpenSystem::new(Some(2));
    slos_0_1
        .system_mut()
        .set(pp_0, CalculatorComplex::from(0.4))
        .unwrap();
    slos_0_1
        .noise_mut()
        .set((dp_0.clone(), dp_0), CalculatorComplex::from(0.5))
        .unwrap();
    slos_0_1
        .system_mut()
        .set(pp_1, CalculatorComplex::from(-0.4))
        .unwrap();
    slos_0_1
        .noise_mut()
        .set((dp_1.clone(), dp_1), CalculatorComplex::from(-0.5))
        .unwrap();

    assert_eq!(slos_0 - slos_1, Ok(slos_0_1));
}

// Test the multiplication: BosonLindbladOpenSystem * Calculatorcomplex
#[test]
fn mul_so_cf() {
    let dp_0: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let pp_0: HermitianBosonProduct = HermitianBosonProduct::new([], [0]).unwrap();
    let mut slos_0 = BosonLindbladOpenSystem::new(Some(2));
    slos_0
        .system_mut()
        .set(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    slos_0
        .noise_mut()
        .set((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_0_1 = BosonLindbladOpenSystem::new(Some(2));
    slos_0_1
        .system_mut()
        .set(pp_0, CalculatorComplex::from(3.0))
        .unwrap();
    slos_0_1
        .noise_mut()
        .set((dp_0.clone(), dp_0), CalculatorComplex::from(1.5))
        .unwrap();

    assert_eq!(slos_0 * CalculatorFloat::from(3.0), slos_0_1);
}

// Test the Debug trait of BosonLindbladOpenSystem
#[test]
fn debug() {
    let pp: HermitianBosonProduct = HermitianBosonProduct::new([0], [1]).unwrap();
    let dp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut slos = BosonLindbladOpenSystem::new(Some(2));
    slos.system_mut()
        .set(pp, CalculatorComplex::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(
        format!("{:?}", slos),
        "BosonLindbladOpenSystem { system: BosonHamiltonianSystem { number_modes: Some(2), hamiltonian: BosonHamiltonian { internal_map: {HermitianBosonProduct { creators: [0], annihilators: [1] }: CalculatorComplex { re: Float(0.4), im: Float(0.0) }} } }, noise: BosonLindbladNoiseSystem { number_modes: Some(2), operator: BosonLindbladNoiseOperator { internal_map: {(BosonProduct { creators: [0], annihilators: [0] }, BosonProduct { creators: [0], annihilators: [0] }): CalculatorComplex { re: Float(0.5), im: Float(0.0) }} } } }"
    );
}

// Test the Display trait of BosonLindbladOpenSystem
#[test]
fn display() {
    let pp: HermitianBosonProduct = HermitianBosonProduct::new([0], [1]).unwrap();
    let dp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut slos = BosonLindbladOpenSystem::new(Some(2));
    slos.system_mut()
        .set(pp, CalculatorComplex::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    assert_eq!(
        format!("{}", slos),
        "BosonLindbladOpenSystem(2){\nSystem: {\nc0a1: (4e-1 + i * 0e0),\n}\nNoise: {\n(c0a0, c0a0): (5e-1 + i * 0e0),\n}\n}"
    );
}

// Test the Clone and PartialEq traits of BosonLindbladOpenSystem
#[test]
fn clone_partial_eq() {
    let pp: HermitianBosonProduct = HermitianBosonProduct::new([0], [1]).unwrap();
    let dp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut slos = BosonLindbladOpenSystem::default();
    slos.system_mut()
        .set(pp, CalculatorComplex::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    // Test Clone trait
    assert_eq!(slos.clone(), slos);

    // Test PartialEq trait
    let pp_1: HermitianBosonProduct = HermitianBosonProduct::new([0], [1]).unwrap();
    let dp_1: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut slos_1 = BosonLindbladOpenSystem::default();
    slos_1
        .system_mut()
        .set(pp_1, CalculatorComplex::from(0.4))
        .unwrap();
    slos_1
        .noise_mut()
        .set((dp_1.clone(), dp_1), CalculatorComplex::from(0.5))
        .unwrap();
    let pp_2: HermitianBosonProduct = HermitianBosonProduct::new([], [0]).unwrap();
    let dp_2: BosonProduct = BosonProduct::new([0], [1]).unwrap();
    let mut slos_2 = BosonLindbladOpenSystem::default();
    assert!(slos_1 == slos);
    assert!(slos == slos_1);
    assert!(slos_2 != slos);
    assert!(slos != slos_2);
    slos_2
        .system_mut()
        .set(pp_2, CalculatorComplex::from(0.4))
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

/// Test BosonLindbladOpenSystem Serialization and Deserialization traits (readable)
#[test]
fn serde_json() {
    let pp: HermitianBosonProduct = HermitianBosonProduct::new([0], [1]).unwrap();
    let dp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut slos = BosonLindbladOpenSystem::default();
    slos.system_mut()
        .set(pp, CalculatorComplex::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    let serialized = serde_json::to_string(&slos).unwrap();
    let deserialized: BosonLindbladOpenSystem = serde_json::from_str(&serialized).unwrap();
    assert_eq!(slos, deserialized);
}

#[test]
fn serde_readable() {
    use struqture::MINIMUM_STRUQTURE_VERSION;
    let major_version = MINIMUM_STRUQTURE_VERSION.0;
    let minor_version = MINIMUM_STRUQTURE_VERSION.1;

    let pp: HermitianBosonProduct = HermitianBosonProduct::new([0], [1]).unwrap();
    let dp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut slos = BosonLindbladOpenSystem::new(Some(2));
    slos.system_mut()
        .set(pp, CalculatorComplex::from(1.0))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    assert_tokens(
        &slos.readable(),
        &[
            Token::Struct {
                name: "BosonLindbladOpenSystem",
                len: 2,
            },
            Token::Str("system"),
            Token::Struct {
                name: "BosonHamiltonianSystem",
                len: 2,
            },
            Token::Str("number_modes"),
            Token::Some,
            Token::U64(2),
            Token::Str("hamiltonian"),
            Token::Struct {
                name: "BosonHamiltonianSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 3 },
            Token::Str("c0a1"),
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
            Token::Str("noise"),
            Token::Struct {
                name: "BosonLindbladNoiseSystem",
                len: 2,
            },
            Token::Str("number_modes"),
            Token::Some,
            Token::U64(2),
            Token::Str("operator"),
            Token::Struct {
                name: "BosonLindbladNoiseOperatorSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 4 },
            Token::Str("c0a0"),
            Token::Str("c0a0"),
            Token::F64(0.5),
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
            Token::StructEnd,
        ],
    );
}

#[test]
fn bincode() {
    let pp: HermitianBosonProduct = HermitianBosonProduct::new([0], [1]).unwrap();
    let dp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut slos = BosonLindbladOpenSystem::default();
    slos.system_mut()
        .set(pp, CalculatorComplex::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    let config = bincode::config::legacy();
    let serialized: Vec<u8> = bincode::serde::encode_to_vec(&slos, config).unwrap();
    let (deserialized, _len): (BosonLindbladOpenSystem, usize) =
        bincode::serde::decode_from_slice(&serialized[..], config).unwrap();
    assert_eq!(slos, deserialized);

    let serialized: Vec<u8> =
        bincode::serde::encode_to_vec(slos.clone().compact(), config).unwrap();
    let (deserialized, _len): (BosonLindbladOpenSystem, usize) =
        bincode::serde::decode_from_slice(&serialized[..], config).unwrap();
    assert_eq!(slos, deserialized);
}

/// Test BosonLindbladOpenSystem Serialization and Deserialization traits (compact)
#[test]
fn serde_compact() {
    use struqture::MINIMUM_STRUQTURE_VERSION;
    let major_version = MINIMUM_STRUQTURE_VERSION.0;
    let minor_version = MINIMUM_STRUQTURE_VERSION.1;

    let pp: HermitianBosonProduct = HermitianBosonProduct::new([0], [1]).unwrap();
    let dp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut slos = BosonLindbladOpenSystem::new(Some(2));
    slos.system_mut()
        .set(pp, CalculatorComplex::from(1.0))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    assert_tokens(
        &slos.compact(),
        &[
            Token::Struct {
                name: "BosonLindbladOpenSystem",
                len: 2,
            },
            Token::Str("system"),
            Token::Struct {
                name: "BosonHamiltonianSystem",
                len: 2,
            },
            Token::Str("number_modes"),
            Token::Some,
            Token::U64(2),
            Token::Str("hamiltonian"),
            Token::Struct {
                name: "BosonHamiltonianSerialize",
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
            Token::U64(1),
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
            Token::Str("noise"),
            Token::Struct {
                name: "BosonLindbladNoiseSystem",
                len: 2,
            },
            Token::Str("number_modes"),
            Token::Some,
            Token::U64(2),
            Token::Str("operator"),
            Token::Struct {
                name: "BosonLindbladNoiseOperatorSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 4 },
            Token::Tuple { len: 2 },
            Token::Seq { len: Some(1) },
            Token::U64(0),
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::U64(0),
            Token::SeqEnd,
            Token::TupleEnd,
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
            Token::F64(0.5),
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
            Token::StructEnd,
        ],
    );
}
#[test]
fn test_truncate() {
    let mut system = BosonLindbladOpenSystem::new(None);
    system
        .system_mut()
        .set(HermitianBosonProduct::new([0], [1]).unwrap(), 1.0.into())
        .unwrap();
    system
        .system_mut()
        .set(HermitianBosonProduct::new([0], [2]).unwrap(), 0.1.into())
        .unwrap();
    system
        .system_mut()
        .set(HermitianBosonProduct::new([3], [4]).unwrap(), 0.01.into())
        .unwrap();
    system
        .system_mut()
        .set(
            HermitianBosonProduct::new([0, 3], [1, 4]).unwrap(),
            "test".into(),
        )
        .unwrap();

    let _ = system.noise_mut().set(
        (
            BosonProduct::new([0, 1], [0, 1]).unwrap(),
            BosonProduct::new([0, 1], [0, 1]).unwrap(),
        ),
        "test".into(),
    );
    let _ = system.noise_mut().set(
        (
            BosonProduct::new([0, 2], [0, 2]).unwrap(),
            BosonProduct::new([0, 2], [0, 2]).unwrap(),
        ),
        1.0.into(),
    );
    let _ = system.noise_mut().set(
        (
            BosonProduct::new([1, 2], [1, 2]).unwrap(),
            BosonProduct::new([1, 2], [1, 2]).unwrap(),
        ),
        0.1.into(),
    );
    let _ = system.noise_mut().set(
        (
            BosonProduct::new([1], [1]).unwrap(),
            BosonProduct::new([1], [1]).unwrap(),
        ),
        0.01.into(),
    );

    let mut test_system1 = BosonLindbladOpenSystem::new(None);
    test_system1
        .system_mut()
        .set(HermitianBosonProduct::new([0], [1]).unwrap(), 1.0.into())
        .unwrap();
    test_system1
        .system_mut()
        .set(HermitianBosonProduct::new([0], [2]).unwrap(), 0.1.into())
        .unwrap();
    test_system1
        .system_mut()
        .set(
            HermitianBosonProduct::new([0, 3], [1, 4]).unwrap(),
            "test".into(),
        )
        .unwrap();
    let _ = test_system1.noise_mut().set(
        (
            BosonProduct::new([0, 1], [0, 1]).unwrap(),
            BosonProduct::new([0, 1], [0, 1]).unwrap(),
        ),
        "test".into(),
    );
    let _ = test_system1.noise_mut().set(
        (
            BosonProduct::new([0, 2], [0, 2]).unwrap(),
            BosonProduct::new([0, 2], [0, 2]).unwrap(),
        ),
        1.0.into(),
    );
    let _ = test_system1.noise_mut().set(
        (
            BosonProduct::new([1, 2], [1, 2]).unwrap(),
            BosonProduct::new([1, 2], [1, 2]).unwrap(),
        ),
        0.1.into(),
    );

    let mut test_system2 = BosonLindbladOpenSystem::new(None);
    test_system2
        .system_mut()
        .set(HermitianBosonProduct::new([0], [1]).unwrap(), 1.0.into())
        .unwrap();
    test_system2
        .system_mut()
        .set(
            HermitianBosonProduct::new([0, 3], [1, 4]).unwrap(),
            "test".into(),
        )
        .unwrap();
    let _ = test_system2.noise_mut().set(
        (
            BosonProduct::new([0, 1], [0, 1]).unwrap(),
            BosonProduct::new([0, 1], [0, 1]).unwrap(),
        ),
        "test".into(),
    );
    let _ = test_system2.noise_mut().set(
        (
            BosonProduct::new([0, 2], [0, 2]).unwrap(),
            BosonProduct::new([0, 2], [0, 2]).unwrap(),
        ),
        1.0.into(),
    );

    let comparison_system1 = system.truncate(0.05);
    assert_eq!(test_system1, comparison_system1);
    let comparison_system2 = system.truncate(0.5);
    assert_eq!(test_system2, comparison_system2);
}

#[cfg(feature = "json_schema")]
#[test_case(None)]
#[test_case(Some(3))]
fn test_boson_open_system_schema(number_bosons: Option<usize>) {
    let mut op = BosonLindbladOpenSystem::new(number_bosons);
    let pp: HermitianBosonProduct = HermitianBosonProduct::new([0], [1]).unwrap();
    let dp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    op.system_mut()
        .set(pp, CalculatorComplex::from(0.4))
        .unwrap();
    op.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from("val"))
        .unwrap();
    let schema = schemars::schema_for!(BosonLindbladOpenSystem);
    let schema_checker = jsonschema::validator_for(&serde_json::to_value(&schema).unwrap())
        .expect("schema is valid");
    let value = serde_json::to_value(&op).unwrap();
    let validation = schema_checker.validate(&value);

    assert!(validation.is_ok());
}
