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

//! Integration test for public API of MixedLindbladOpenSystem

use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde_test::{assert_tokens, Configure, Token};
use std::collections::BTreeMap;
use struqture::bosons::BosonProduct;
use struqture::fermions::FermionProduct;
use struqture::mixed_systems::{
    HermitianMixedProduct, MixedDecoherenceProduct, MixedHamiltonianSystem,
    MixedLindbladNoiseSystem, MixedLindbladOpenSystem,
};
use struqture::prelude::*;
use struqture::spins::{DecoherenceProduct, PauliProduct};
#[cfg(feature = "json_schema")]
use test_case::test_case;

// Test the new function of the MixedLindbladOpenSystem
#[test]
fn new_system() {
    let system = MixedLindbladOpenSystem::new([Some(1)], [Some(1)], [Some(1)]);
    assert_eq!(
        system.system(),
        &MixedHamiltonianSystem::new([Some(1)], [Some(1)], [Some(1)])
    );
    assert_eq!(
        system.noise(),
        &MixedLindbladNoiseSystem::new([Some(1)], [Some(1)], [Some(1)])
    );
    assert_eq!(vec![0], system.current_number_spins());
    assert_eq!(vec![0], system.current_number_bosonic_modes());
    assert_eq!(vec![0], system.current_number_fermionic_modes());
    assert_eq!(vec![1], system.number_spins());
    assert_eq!(vec![1], system.number_bosonic_modes());
    assert_eq!(vec![1], system.number_fermionic_modes());

    assert_eq!(
        MixedLindbladOpenSystem::new([], [], []),
        MixedLindbladOpenSystem::default()
    );
}

// Test the new function of the MixedLindbladOpenSystem with no modes specified
#[test]
fn new_system_none() {
    let system = MixedLindbladOpenSystem::new([None], [None], [None]);
    assert!(system.system().is_empty());
    assert_eq!(
        system.system(),
        &MixedHamiltonianSystem::new([None], [None], [None])
    );
    assert!(system.noise().is_empty());
    assert_eq!(
        system.noise(),
        &MixedLindbladNoiseSystem::new([None], [None], [None])
    );
    assert_eq!(vec![0], system.current_number_spins());
    assert_eq!(vec![0], system.current_number_bosonic_modes());
    assert_eq!(vec![0], system.current_number_fermionic_modes());
    assert_eq!(vec![0], system.number_spins());
    assert_eq!(vec![0], system.number_bosonic_modes());
    assert_eq!(vec![0], system.number_fermionic_modes());
}

// Test the group function of the MixedLindbladOpenSystem
#[test]
fn group() {
    let slos = MixedLindbladOpenSystem::group(
        MixedHamiltonianSystem::new([], [], []),
        MixedLindbladNoiseSystem::new([], [], []),
    );
    assert!(slos.is_ok());
    let slos = slos.unwrap();
    assert!(slos.system().is_empty() && slos.noise().is_empty());
    assert_eq!(slos, MixedLindbladOpenSystem::default())
}

// Test the group function of the MixedLindbladOpenSystem
#[test]
fn group_with_none() {
    let mlos = MixedLindbladOpenSystem::group(
        MixedHamiltonianSystem::new([None], [None], [None]),
        MixedLindbladNoiseSystem::new([Some(2)], [Some(2)], [Some(2)]),
    );
    assert!(mlos.is_ok());
    let os = mlos.unwrap();
    let (system, noise) = os.ungroup();
    assert_eq!(noise.number_bosonic_modes(), vec![2]);
    assert_eq!(noise.number_fermionic_modes(), vec![2]);
    assert_eq!(noise.number_spins(), vec![2]);
    assert_eq!(system.number_bosonic_modes(), vec![2]);
    assert_eq!(system.number_fermionic_modes(), vec![2]);
    assert_eq!(system.number_spins(), vec![2]);

    let mlos = MixedLindbladOpenSystem::group(
        MixedHamiltonianSystem::new([Some(2)], [Some(2)], [Some(2)]),
        MixedLindbladNoiseSystem::new([None], [None], [None]),
    );
    assert!(mlos.is_ok());
    let os = mlos.unwrap();
    let (system, noise) = os.ungroup();
    assert_eq!(noise.number_bosonic_modes(), vec![2]);
    assert_eq!(noise.number_fermionic_modes(), vec![2]);
    assert_eq!(noise.number_spins(), vec![2]);
    assert_eq!(system.number_bosonic_modes(), vec![2]);
    assert_eq!(system.number_fermionic_modes(), vec![2]);
    assert_eq!(system.number_spins(), vec![2]);
}

// Test the group function of the MixedLindbladOpenSystem
#[test]
fn group_failing() {
    let mlos = MixedLindbladOpenSystem::group(
        MixedHamiltonianSystem::new([None], [None], [Some(3)]),
        MixedLindbladNoiseSystem::new([Some(2)], [Some(2)], [Some(2)]),
    );
    assert!(mlos.is_err());

    let mlos = MixedLindbladOpenSystem::group(
        MixedHamiltonianSystem::new([None], [Some(3)], [None]),
        MixedLindbladNoiseSystem::new([Some(2)], [Some(2)], [Some(2)]),
    );
    assert!(mlos.is_err());
    let mlos = MixedLindbladOpenSystem::group(
        MixedHamiltonianSystem::new([Some(3)], [None], [None]),
        MixedLindbladNoiseSystem::new([Some(2)], [Some(2)], [Some(2)]),
    );
    assert!(mlos.is_err());
}

#[test]
fn empty_clone_options() {
    let dp_0: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let mut slos = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);
    slos.noise_mut()
        .set((dp_0.clone(), dp_0), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(
        slos.empty_clone(),
        MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)])
    );
}

// Test the try_set_noise and get functions of the MixedLindbladOpenSystem
#[test]
fn internal_map_set_get_system_noise() {
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [1]).unwrap()],
    )
    .unwrap();
    let dp_2: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let mut slos = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);

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

// Test the add_noise function of the MixedLindbladOpenSystem
#[test]
fn internal_map_add_system_noise() {
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [1]).unwrap()],
    )
    .unwrap();
    let dp_2: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let mut slos = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);

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

// Test the iter, keys and values functions of the MixedLindbladOpenSystem
#[test]
fn internal_map_keys() {
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [1]).unwrap()],
    )
    .unwrap();
    let dp_2: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let mut slos = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);

    slos.system_mut()
        .set(pp_0.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    slos.noise_mut()
        .set((dp_2.clone(), dp_2.clone()), CalculatorComplex::from(0.5))
        .unwrap();

    let mut map_system: BTreeMap<HermitianMixedProduct, CalculatorComplex> = BTreeMap::new();
    map_system.insert(pp_0, CalculatorComplex::from(0.5));
    let mut map_noise: BTreeMap<
        (MixedDecoherenceProduct, MixedDecoherenceProduct),
        CalculatorComplex,
    > = BTreeMap::new();
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

// Test the noise and system functions of the MixedLindbladOpenSystem
#[test]
fn noise_system() {
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [1]).unwrap()],
    )
    .unwrap();
    let dp_2: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let mut slos = MixedLindbladOpenSystem::new([None], [None], [None]);

    slos.system_mut()
        .set(pp_0.clone(), CalculatorComplex::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp_2.clone(), dp_2.clone()), CalculatorComplex::from(0.5))
        .unwrap();

    let mut system = MixedHamiltonianSystem::new([None], [None], [None]);
    system.set(pp_0, CalculatorComplex::from(0.4)).unwrap();
    let mut noise = MixedLindbladNoiseSystem::new([None], [None], [None]);
    noise
        .set((dp_2.clone(), dp_2), CalculatorComplex::from(0.5))
        .unwrap();

    assert_eq!(slos.system(), &system);
    assert_eq!(slos.noise(), &noise);
}

// Test the negative operation: -MixedLindbladOpenSystem
#[test]
fn negative_slos() {
    let dp_0: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [1]).unwrap()],
    )
    .unwrap();
    let mut slos_0 = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);
    slos_0
        .system_mut()
        .set(pp_0.clone(), CalculatorComplex::from(0.4))
        .unwrap();
    slos_0
        .noise_mut()
        .set((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_0_minus = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);
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

// Test the addition: MixedLindbladOpenSystem + MixedLindbladOpenSystem
#[test]
fn add_slos_slos() {
    let dp_0: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [1]).unwrap()],
    )
    .unwrap();
    let dp_1: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().x(1)],
        [BosonProduct::new([0], [2]).unwrap()],
        [FermionProduct::new([0], [1]).unwrap()],
    )
    .unwrap();
    let pp_1: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [0]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut slos_0 = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);
    slos_0
        .system_mut()
        .set(pp_0.clone(), CalculatorComplex::from(0.4))
        .unwrap();
    slos_0
        .noise_mut()
        .set((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_1 = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);
    slos_1
        .system_mut()
        .set(pp_1.clone(), CalculatorComplex::from(0.4))
        .unwrap();
    slos_1
        .noise_mut()
        .set((dp_1.clone(), dp_1.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_0_1 = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);
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

// Test the subtraction: MixedLindbladOpenSystem - MixedLindbladOpenSystem
#[test]
fn sub_slos_slos() {
    let dp_0: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [1]).unwrap()],
    )
    .unwrap();
    let dp_1: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().x(1)],
        [BosonProduct::new([0], [2]).unwrap()],
        [FermionProduct::new([0], [1]).unwrap()],
    )
    .unwrap();
    let pp_1: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [0]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut slos_0 = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);
    slos_0
        .system_mut()
        .set(pp_0.clone(), CalculatorComplex::from(0.4))
        .unwrap();
    slos_0
        .noise_mut()
        .set((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_1 = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);
    slos_1
        .system_mut()
        .set(pp_1.clone(), CalculatorComplex::from(0.4))
        .unwrap();
    slos_1
        .noise_mut()
        .set((dp_1.clone(), dp_1.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_0_1 = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);
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

// Test the multiplication: MixedLindbladOpenSystem * Calculatorcomplex
#[test]
fn mul_so_cf() {
    let dp_0: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [1]).unwrap()],
    )
    .unwrap();
    let mut slos_0 = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);
    slos_0
        .system_mut()
        .set(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    slos_0
        .noise_mut()
        .set((dp_0.clone(), dp_0.clone()), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_0_1 = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);
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

// Test the Debug trait of MixedLindbladOpenSystem
#[test]
fn debug() {
    let pp: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [1]).unwrap()],
    )
    .unwrap();
    let dp: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let mut slos = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);
    slos.system_mut()
        .set(pp, CalculatorComplex::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(
        format!("{:?}", slos),
        "MixedLindbladOpenSystem { system: MixedHamiltonianSystem { number_spins: [Some(3)], number_bosons: [Some(4)], number_fermions: [Some(4)], hamiltonian: MixedHamiltonian { internal_map: {HermitianMixedProduct { spins: [PauliProduct { items: [(0, X)] }], bosons: [BosonProduct { creators: [0], annihilators: [1] }], fermions: [FermionProduct { creators: [0], annihilators: [1] }] }: CalculatorComplex { re: Float(0.4), im: Float(0.0) }}, n_spins: 1, n_bosons: 1, n_fermions: 1 } }, noise: MixedLindbladNoiseSystem { number_spins: [Some(3)], number_bosons: [Some(4)], number_fermions: [Some(4)], operator: MixedLindbladNoiseOperator { internal_map: {(MixedDecoherenceProduct { spins: [DecoherenceProduct { items: [(2, Z)] }], bosons: [BosonProduct { creators: [0], annihilators: [3] }], fermions: [FermionProduct { creators: [0], annihilators: [3] }] }, MixedDecoherenceProduct { spins: [DecoherenceProduct { items: [(2, Z)] }], bosons: [BosonProduct { creators: [0], annihilators: [3] }], fermions: [FermionProduct { creators: [0], annihilators: [3] }] }): CalculatorComplex { re: Float(0.5), im: Float(0.0) }}, n_spins: 1, n_bosons: 1, n_fermions: 1 } } }"
    );
}

// Test the Display trait of MixedLindbladOpenSystem
#[test]
fn display() {
    let pp: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [1]).unwrap()],
    )
    .unwrap();
    let dp: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let mut slos = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);
    slos.system_mut()
        .set(pp, CalculatorComplex::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    assert_eq!(
        format!("{}", slos),
        "MixedLindbladOpenSystem{\nSystem: {\nMixedHamiltonianSystem(\nnumber_spins: 3, \nnumber_bosons: 4, \nnumber_fermions: 4, )\n{S0X:Bc0a1:Fc0a1:: (4e-1 + i * 0e0),\n}}\nNoise: {\nMixedLindbladNoiseSystem(\nnumber_spins: 3, \nnumber_bosons: 4, \nnumber_fermions: 4, )\n{(S2Z:Bc0a3:Fc0a3:, S2Z:Bc0a3:Fc0a3:): (5e-1 + i * 0e0),\n}}\n}"
    );
}

// Test the Clone and PartialEq traits of MixedLindbladOpenSystem
#[test]
fn clone_partial_eq() {
    let pp: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [1]).unwrap()],
    )
    .unwrap();
    let dp: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let mut slos = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);
    slos.system_mut()
        .set(pp, CalculatorComplex::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    // Test Clone trait
    assert_eq!(slos.clone(), slos);

    // Test PartialEq trait
    let pp_1: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [1]).unwrap()],
    )
    .unwrap();
    let dp_1: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let pp_2: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [0]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let dp_2: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().x(0)],
        [BosonProduct::new([0], [2]).unwrap()],
        [FermionProduct::new([0], [1]).unwrap()],
    )
    .unwrap();
    let mut slos_1 = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);
    slos_1
        .system_mut()
        .set(pp_1, CalculatorComplex::from(0.4))
        .unwrap();
    slos_1
        .noise_mut()
        .set((dp_1.clone(), dp_1), CalculatorComplex::from(0.5))
        .unwrap();
    let mut slos_2 = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);
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

#[test]
fn serde_json() {
    let pp: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [1]).unwrap()],
    )
    .unwrap();
    let dp: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let mut slos = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);
    slos.system_mut()
        .set(pp, CalculatorComplex::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    let serialized = serde_json::to_string(&slos).unwrap();
    let deserialized: MixedLindbladOpenSystem = serde_json::from_str(&serialized).unwrap();
    assert_eq!(slos, deserialized);
}

#[test]
fn serde_readable() {
    use struqture::MINIMUM_STRUQTURE_VERSION;
    let major_version = MINIMUM_STRUQTURE_VERSION.0;
    let minor_version = MINIMUM_STRUQTURE_VERSION.1;

    let pp: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [1]).unwrap()],
    )
    .unwrap();
    let dp: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let mut slos = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);
    slos.system_mut()
        .set(pp, CalculatorComplex::from(0.5))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    assert_tokens(
        &slos.readable(),
        &[
            Token::Struct {
                name: "MixedLindbladOpenSystem",
                len: 2,
            },
            Token::Str("system"),
            Token::Struct {
                name: "MixedHamiltonianSystem",
                len: 4,
            },
            Token::Str("number_spins"),
            Token::Seq { len: Some(1) },
            Token::Some,
            Token::U64(3),
            Token::SeqEnd,
            Token::Str("number_bosons"),
            Token::Seq { len: Some(1) },
            Token::Some,
            Token::U64(4),
            Token::SeqEnd,
            Token::Str("number_fermions"),
            Token::Seq { len: Some(1) },
            Token::Some,
            Token::U64(4),
            Token::SeqEnd,
            Token::Str("hamiltonian"),
            Token::Struct {
                name: "MixedHamiltonianSerialize",
                len: 5,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 3 },
            Token::Str("S0X:Bc0a1:Fc0a1:"),
            Token::F64(0.5),
            Token::F64(0.0),
            Token::TupleEnd,
            Token::SeqEnd,
            Token::Str("n_spins"),
            Token::U64(1),
            Token::Str("n_bosons"),
            Token::U64(1),
            Token::Str("n_fermions"),
            Token::U64(1),
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
                name: "MixedLindbladNoiseSystem",
                len: 4,
            },
            Token::Str("number_spins"),
            Token::Seq { len: Some(1) },
            Token::Some,
            Token::U64(3),
            Token::SeqEnd,
            Token::Str("number_bosons"),
            Token::Seq { len: Some(1) },
            Token::Some,
            Token::U64(4),
            Token::SeqEnd,
            Token::Str("number_fermions"),
            Token::Seq { len: Some(1) },
            Token::Some,
            Token::U64(4),
            Token::SeqEnd,
            Token::Str("operator"),
            Token::Struct {
                name: "MixedLindbladNoiseOperatorSerialize",
                len: 5,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 4 },
            Token::Str("S2Z:Bc0a3:Fc0a3:"),
            Token::Str("S2Z:Bc0a3:Fc0a3:"),
            Token::F64(0.5),
            Token::F64(0.0),
            Token::TupleEnd,
            Token::SeqEnd,
            Token::Str("n_spins"),
            Token::U64(1),
            Token::Str("n_bosons"),
            Token::U64(1),
            Token::Str("n_fermions"),
            Token::U64(1),
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
    let pp: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [1]).unwrap()],
    )
    .unwrap();
    let dp: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let mut slos = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);
    slos.system_mut()
        .set(pp, CalculatorComplex::from(0.4))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    let config = bincode::config::legacy();
    let serialized: Vec<u8> = bincode::serde::encode_to_vec(&slos, config).unwrap();
    let (deserialized, _len): (MixedLindbladOpenSystem, usize) =
        bincode::serde::decode_from_slice(&serialized[..], config).unwrap();
    assert_eq!(slos, deserialized);

    let serialized: Vec<u8> =
        bincode::serde::encode_to_vec(slos.clone().compact(), config).unwrap();
    let (deserialized, _len): (MixedLindbladOpenSystem, usize) =
        bincode::serde::decode_from_slice(&serialized[..], config).unwrap();
    assert_eq!(slos, deserialized);
}

/// Test MixedLindbladOpenSystem Serialization and Deserialization traits (compact)
#[test]
fn serde_compact() {
    use struqture::MINIMUM_STRUQTURE_VERSION;
    let major_version = MINIMUM_STRUQTURE_VERSION.0;
    let minor_version = MINIMUM_STRUQTURE_VERSION.1;

    let pp: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [1]).unwrap()],
    )
    .unwrap();
    let dp: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let mut slos = MixedLindbladOpenSystem::new([Some(3)], [Some(4)], [Some(4)]);
    slos.system_mut()
        .set(pp, CalculatorComplex::from(0.5))
        .unwrap();
    slos.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();

    assert_tokens(
        &slos.compact(),
        &[
            Token::Struct {
                name: "MixedLindbladOpenSystem",
                len: 2,
            },
            Token::Str("system"),
            Token::Struct {
                name: "MixedHamiltonianSystem",
                len: 4,
            },
            Token::Str("number_spins"),
            Token::Seq { len: Some(1) },
            Token::Some,
            Token::U64(3),
            Token::SeqEnd,
            Token::Str("number_bosons"),
            Token::Seq { len: Some(1) },
            Token::Some,
            Token::U64(4),
            Token::SeqEnd,
            Token::Str("number_fermions"),
            Token::Seq { len: Some(1) },
            Token::Some,
            Token::U64(4),
            Token::SeqEnd,
            Token::Str("hamiltonian"),
            Token::Struct {
                name: "MixedHamiltonianSerialize",
                len: 5,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 3 },
            Token::Tuple { len: 3 },
            Token::Seq { len: Some(1) },
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::U64(0),
            Token::UnitVariant {
                name: "SingleSpinOperator",
                variant: "X",
            },
            Token::TupleEnd,
            Token::SeqEnd,
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::Seq { len: Some(1) },
            Token::U64(0),
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::U64(1),
            Token::SeqEnd,
            Token::TupleEnd,
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::Seq { len: Some(1) },
            Token::U64(0),
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::U64(1),
            Token::SeqEnd,
            Token::TupleEnd,
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
            Token::Str("n_spins"),
            Token::U64(1),
            Token::Str("n_bosons"),
            Token::U64(1),
            Token::Str("n_fermions"),
            Token::U64(1),
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
                name: "MixedLindbladNoiseSystem",
                len: 4,
            },
            Token::Str("number_spins"),
            Token::Seq { len: Some(1) },
            Token::Some,
            Token::U64(3),
            Token::SeqEnd,
            Token::Str("number_bosons"),
            Token::Seq { len: Some(1) },
            Token::Some,
            Token::U64(4),
            Token::SeqEnd,
            Token::Str("number_fermions"),
            Token::Seq { len: Some(1) },
            Token::Some,
            Token::U64(4),
            Token::SeqEnd,
            Token::Str("operator"),
            Token::Struct {
                name: "MixedLindbladNoiseOperatorSerialize",
                len: 5,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 4 },
            Token::Tuple { len: 3 },
            Token::Seq { len: Some(1) },
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::U64(2),
            Token::UnitVariant {
                name: "SingleDecoherenceOperator",
                variant: "Z",
            },
            Token::TupleEnd,
            Token::SeqEnd,
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::Seq { len: Some(1) },
            Token::U64(0),
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::U64(3),
            Token::SeqEnd,
            Token::TupleEnd,
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::Seq { len: Some(1) },
            Token::U64(0),
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::U64(3),
            Token::SeqEnd,
            Token::TupleEnd,
            Token::SeqEnd,
            Token::TupleEnd,
            Token::Tuple { len: 3 },
            Token::Seq { len: Some(1) },
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::U64(2),
            Token::UnitVariant {
                name: "SingleDecoherenceOperator",
                variant: "Z",
            },
            Token::TupleEnd,
            Token::SeqEnd,
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::Seq { len: Some(1) },
            Token::U64(0),
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::U64(3),
            Token::SeqEnd,
            Token::TupleEnd,
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::Seq { len: Some(1) },
            Token::U64(0),
            Token::SeqEnd,
            Token::Seq { len: Some(1) },
            Token::U64(3),
            Token::SeqEnd,
            Token::TupleEnd,
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
            Token::Str("n_spins"),
            Token::U64(1),
            Token::Str("n_bosons"),
            Token::U64(1),
            Token::Str("n_fermions"),
            Token::U64(1),
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
    let mut system = MixedLindbladOpenSystem::new([None], [None], [None]);
    system
        .system_mut()
        .set(
            HermitianMixedProduct::new(
                [PauliProduct::new().z(0)],
                [BosonProduct::new([], []).unwrap()],
                [FermionProduct::new([], []).unwrap()],
            )
            .unwrap(),
            1.0.into(),
        )
        .unwrap();
    system
        .system_mut()
        .set(
            HermitianMixedProduct::new(
                [PauliProduct::new().z(1).x(0)],
                [BosonProduct::new([], []).unwrap()],
                [FermionProduct::new([], []).unwrap()],
            )
            .unwrap(),
            0.1.into(),
        )
        .unwrap();
    system
        .system_mut()
        .set(
            HermitianMixedProduct::new(
                [PauliProduct::new().z(1).x(0)],
                [BosonProduct::new([], []).unwrap()],
                [FermionProduct::new([0], [0]).unwrap()],
            )
            .unwrap(),
            0.01.into(),
        )
        .unwrap();
    system
        .system_mut()
        .set(
            HermitianMixedProduct::new(
                [PauliProduct::new().z(1).x(0)],
                [BosonProduct::new([], []).unwrap()],
                [FermionProduct::new([1], [1]).unwrap()],
            )
            .unwrap(),
            "test".into(),
        )
        .unwrap();

    let _ = system.noise_mut().set(
        (
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new()],
                [BosonProduct::new([], []).unwrap()],
                [FermionProduct::new([1], [1]).unwrap()],
            )
            .unwrap(),
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new()],
                [BosonProduct::new([], []).unwrap()],
                [FermionProduct::new([1], [1]).unwrap()],
            )
            .unwrap(),
        ),
        "test".into(),
    );
    let _ = system.noise_mut().set(
        (
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new()],
                [BosonProduct::new([0], []).unwrap()],
                [FermionProduct::new([1], [1]).unwrap()],
            )
            .unwrap(),
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new()],
                [BosonProduct::new([0], []).unwrap()],
                [FermionProduct::new([1], [1]).unwrap()],
            )
            .unwrap(),
        ),
        1.0.into(),
    );
    let _ = system.noise_mut().set(
        (
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new()],
                [BosonProduct::new([0], [1]).unwrap()],
                [FermionProduct::new([1], [1]).unwrap()],
            )
            .unwrap(),
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new()],
                [BosonProduct::new([0], [1]).unwrap()],
                [FermionProduct::new([1], [1]).unwrap()],
            )
            .unwrap(),
        ),
        0.1.into(),
    );
    let _ = system.noise_mut().set(
        (
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new()],
                [BosonProduct::new([0, 1, 2], [1]).unwrap()],
                [FermionProduct::new([1], [1]).unwrap()],
            )
            .unwrap(),
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new()],
                [BosonProduct::new([0, 1, 2], [1]).unwrap()],
                [FermionProduct::new([1], [1]).unwrap()],
            )
            .unwrap(),
        ),
        0.01.into(),
    );

    let mut test_system1 = MixedLindbladOpenSystem::new([None], [None], [None]);
    test_system1
        .system_mut()
        .set(
            HermitianMixedProduct::new(
                [PauliProduct::new().z(0)],
                [BosonProduct::new([], []).unwrap()],
                [FermionProduct::new([], []).unwrap()],
            )
            .unwrap(),
            1.0.into(),
        )
        .unwrap();
    test_system1
        .system_mut()
        .set(
            HermitianMixedProduct::new(
                [PauliProduct::new().z(1).x(0)],
                [BosonProduct::new([], []).unwrap()],
                [FermionProduct::new([], []).unwrap()],
            )
            .unwrap(),
            0.1.into(),
        )
        .unwrap();
    test_system1
        .system_mut()
        .set(
            HermitianMixedProduct::new(
                [PauliProduct::new().z(1).x(0)],
                [BosonProduct::new([], []).unwrap()],
                [FermionProduct::new([1], [1]).unwrap()],
            )
            .unwrap(),
            "test".into(),
        )
        .unwrap();
    let _ = test_system1.noise_mut().set(
        (
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new()],
                [BosonProduct::new([], []).unwrap()],
                [FermionProduct::new([1], [1]).unwrap()],
            )
            .unwrap(),
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new()],
                [BosonProduct::new([], []).unwrap()],
                [FermionProduct::new([1], [1]).unwrap()],
            )
            .unwrap(),
        ),
        "test".into(),
    );
    let _ = test_system1.noise_mut().set(
        (
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new()],
                [BosonProduct::new([0], []).unwrap()],
                [FermionProduct::new([1], [1]).unwrap()],
            )
            .unwrap(),
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new()],
                [BosonProduct::new([0], []).unwrap()],
                [FermionProduct::new([1], [1]).unwrap()],
            )
            .unwrap(),
        ),
        1.0.into(),
    );
    let _ = test_system1.noise_mut().set(
        (
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new()],
                [BosonProduct::new([0], [1]).unwrap()],
                [FermionProduct::new([1], [1]).unwrap()],
            )
            .unwrap(),
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new()],
                [BosonProduct::new([0], [1]).unwrap()],
                [FermionProduct::new([1], [1]).unwrap()],
            )
            .unwrap(),
        ),
        0.1.into(),
    );

    let mut test_system2 = MixedLindbladOpenSystem::new([None], [None], [None]);
    test_system2
        .system_mut()
        .set(
            HermitianMixedProduct::new(
                [PauliProduct::new().z(0)],
                [BosonProduct::new([], []).unwrap()],
                [FermionProduct::new([], []).unwrap()],
            )
            .unwrap(),
            1.0.into(),
        )
        .unwrap();
    test_system2
        .system_mut()
        .set(
            HermitianMixedProduct::new(
                [PauliProduct::new().z(1).x(0)],
                [BosonProduct::new([], []).unwrap()],
                [FermionProduct::new([1], [1]).unwrap()],
            )
            .unwrap(),
            "test".into(),
        )
        .unwrap();
    let _ = test_system2.noise_mut().set(
        (
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new()],
                [BosonProduct::new([], []).unwrap()],
                [FermionProduct::new([1], [1]).unwrap()],
            )
            .unwrap(),
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new()],
                [BosonProduct::new([], []).unwrap()],
                [FermionProduct::new([1], [1]).unwrap()],
            )
            .unwrap(),
        ),
        "test".into(),
    );
    let _ = test_system2.noise_mut().set(
        (
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new()],
                [BosonProduct::new([0], []).unwrap()],
                [FermionProduct::new([1], [1]).unwrap()],
            )
            .unwrap(),
            MixedDecoherenceProduct::new(
                [DecoherenceProduct::new()],
                [BosonProduct::new([0], []).unwrap()],
                [FermionProduct::new([1], [1]).unwrap()],
            )
            .unwrap(),
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
#[test_case(Some(4))]
fn test_mixed_open_system_schema(number_particles: Option<usize>) {
    let mut op = MixedLindbladOpenSystem::new(
        [number_particles, number_particles],
        [number_particles],
        [number_particles],
    );
    let pp: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0), PauliProduct::new()],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [1]).unwrap()],
    )
    .unwrap();
    let dp: MixedDecoherenceProduct = MixedDecoherenceProduct::new(
        [DecoherenceProduct::new().z(2), DecoherenceProduct::new()],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    op.system_mut()
        .set(pp, CalculatorComplex::from(0.4))
        .unwrap();
    op.noise_mut()
        .set((dp.clone(), dp), CalculatorComplex::from(0.5))
        .unwrap();
    let schema = schemars::schema_for!(MixedLindbladOpenSystem);
    let schema_checker = jsonschema::validator_for(&serde_json::to_value(&schema).unwrap())
        .expect("schema is valid");
    let value = serde_json::to_value(&op).unwrap();
    let validation = schema_checker.validate(&value);

    assert!(validation.is_ok());
}
