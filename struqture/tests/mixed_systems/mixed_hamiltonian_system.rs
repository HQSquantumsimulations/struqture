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

//! Integration test for public API of MixedHamiltonianSystem

// use num_complex::Complex64;
use bincode::{deserialize, serialize};
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde_test::{assert_tokens, Configure, Token};
use std::collections::BTreeMap;
use std::collections::HashMap;
use struqture::bosons::BosonProduct;
use struqture::fermions::FermionProduct;
use struqture::mixed_systems::{
    HermitianMixedProduct, MixedHamiltonian, MixedHamiltonianSystem, MixedSystem,
};
use struqture::prelude::*;
use struqture::spins::PauliProduct;
use struqture::OperateOnDensityMatrix;
use struqture::SpinIndex;
use struqture::StruqtureError;
use test_case::test_case;

#[test]
fn new_empty() {
    let mo = MixedHamiltonianSystem::new([], [], []);
    assert_eq!(mo.hamiltonian(), &MixedHamiltonian::default());
}

// Test the new function of the MixedHamiltonianSystem
#[test_case(0_usize, 0_usize, 0_usize, vec![0], vec![0], vec![0]; "0, 0, 0")]
#[test_case(1_usize, 2_usize, 1_usize, vec![0], vec![0], vec![0]; "1, 2, 1")]
#[test_case(2_usize, 1_usize, 2_usize, vec![0], vec![0], vec![0]; "2, 1, 2")]
#[test_case(10_usize, 10_usize, 10_usize, vec![0], vec![0], vec![0]; "10, 10, 10")]
fn new_current(
    n_pauli: usize,
    n_bosons: usize,
    n_fermions: usize,
    number_spins: Vec<usize>,
    number_bosonic_modes: Vec<usize>,
    number_fermionic_modes: Vec<usize>,
) {
    let mo = MixedHamiltonianSystem::new([Some(n_pauli)], [Some(n_bosons)], [Some(n_fermions)]);
    assert!(mo.is_empty());
    assert_eq!(number_spins, mo.current_number_spins());
    assert_eq!(number_bosonic_modes, mo.current_number_bosonic_modes());
    assert_eq!(number_fermionic_modes, mo.current_number_fermionic_modes());
}

#[test_case(0_usize, 0_usize, 0_usize, vec![0], vec![0], vec![0]; "0, 0, 0")]
#[test_case(1_usize, 2_usize, 1_usize, vec![1], vec![2], vec![1]; "1, 2, 1")]
#[test_case(2_usize, 1_usize, 2_usize, vec![2], vec![1], vec![2]; "2, 1, 2")]
#[test_case(10_usize, 10_usize, 10_usize, vec![10], vec![10], vec![10]; "10, 10, 10")]
fn new_number(
    n_pauli: usize,
    n_bosons: usize,
    n_fermions: usize,
    number_spins: Vec<usize>,
    number_bosonic_modes: Vec<usize>,
    number_fermionic_modes: Vec<usize>,
) {
    let mo = MixedHamiltonianSystem::new([Some(n_pauli)], [Some(n_bosons)], [Some(n_fermions)]);
    assert!(mo.is_empty());
    assert_eq!(number_spins, mo.number_spins());
    assert_eq!(number_bosonic_modes, mo.number_bosonic_modes());
    assert_eq!(number_fermionic_modes, mo.number_fermionic_modes());
}

// Test the new function of the MixedHamiltonianSystem
#[test_case(0_usize, 0_usize, 0_usize, vec![0], vec![0], vec![0]; "0, 0, 0")]
#[test_case(1_usize, 2_usize, 1_usize, vec![0], vec![0], vec![0]; "1, 2, 1")]
#[test_case(2_usize, 1_usize, 2_usize, vec![0], vec![0], vec![0]; "2, 1, 2")]
#[test_case(10_usize, 10_usize, 10_usize, vec![0], vec![0], vec![0]; "10, 10, 10")]
fn new_with_capacity_current(
    n_pauli: usize,
    n_bosons: usize,
    n_fermions: usize,
    number_spins: Vec<usize>,
    number_bosonic_modes: Vec<usize>,
    number_fermionic_modes: Vec<usize>,
) {
    let mo = MixedHamiltonianSystem::new([Some(n_pauli)], [Some(n_bosons)], [Some(n_fermions)]);
    assert!(mo.is_empty());
    assert_eq!(number_spins, mo.current_number_spins());
    assert_eq!(number_bosonic_modes, mo.current_number_bosonic_modes());
    assert_eq!(number_fermionic_modes, mo.current_number_fermionic_modes());
}

#[test_case(0_usize, 0_usize, 0_usize, vec![0], vec![0], vec![0]; "0, 0, 0")]
#[test_case(1_usize, 2_usize, 1_usize, vec![1], vec![2], vec![1]; "1, 2, 1")]
#[test_case(2_usize, 1_usize, 2_usize, vec![2], vec![1], vec![2]; "2, 1, 2")]
#[test_case(10_usize, 10_usize, 10_usize, vec![10], vec![10], vec![10]; "10, 10, 10")]
fn new_with_capacity_number(
    n_pauli: usize,
    n_bosons: usize,
    n_fermions: usize,
    number_spins: Vec<usize>,
    number_bosonic_modes: Vec<usize>,
    number_fermionic_modes: Vec<usize>,
) {
    let mo = MixedHamiltonianSystem::new([Some(n_pauli)], [Some(n_bosons)], [Some(n_fermions)]);
    assert!(mo.is_empty());
    assert_eq!(number_spins, mo.number_spins());
    assert_eq!(number_bosonic_modes, mo.number_bosonic_modes());
    assert_eq!(number_fermionic_modes, mo.number_fermionic_modes());
}

#[test]
fn empty_clone_options() {
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedHamiltonianSystem::new([Some(1)], [Some(2)], [Some(3)]);
    mo.set(pp_0, CalculatorComplex::from(0.5)).unwrap();

    let empty: Option<usize> = None;
    let full: Option<usize> = Some(2);
    assert_eq!(
        mo.empty_clone(empty),
        MixedHamiltonianSystem::new([Some(1)], [Some(2)], [Some(3)])
    );
    assert_eq!(
        mo.empty_clone(full),
        MixedHamiltonianSystem::with_capacity([Some(1)], [Some(2)], [Some(3)], 2)
    );
}

// Test the from_spin_operator and spin_operator functions of the SpinSystem with number_spins = None
#[test]
fn from_spin_operator_none() {
    let mut mo: MixedHamiltonian = MixedHamiltonian::new(1, 1, 1);
    let mut system = MixedHamiltonianSystem::new([None], [None], [None]);
    let pp: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    mo.add_operator_product(pp.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    system
        .add_operator_product(pp, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(
        system,
        MixedHamiltonianSystem::from_hamiltonian(mo.clone(), [None], [None], [None]).unwrap()
    );
    assert_eq!(
        system.hamiltonian(),
        MixedHamiltonianSystem::from_hamiltonian(mo.clone(), [None], [None], [None])
            .unwrap()
            .hamiltonian()
    );
    assert_eq!(
        &mo,
        MixedHamiltonianSystem::from_hamiltonian(mo.clone(), [None], [None], [None])
            .unwrap()
            .hamiltonian()
    );
}

// Test the from_spin_operator and spin_operator functions of the SpinSystem with number_spins = Some(2)
#[test]
fn from_spin_operator_some() {
    let mut mo: MixedHamiltonian = MixedHamiltonian::new(1, 1, 1);
    let mut system = MixedHamiltonianSystem::new([Some(2)], [Some(2)], [Some(2)]);
    let pp: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [1]).unwrap()],
    )
    .unwrap();
    mo.add_operator_product(pp.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    system
        .add_operator_product(pp, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(
        system,
        MixedHamiltonianSystem::from_hamiltonian(mo.clone(), [Some(2)], [Some(2)], [Some(2)])
            .unwrap()
    );
    assert_eq!(
        system.hamiltonian(),
        MixedHamiltonianSystem::from_hamiltonian(mo.clone(), [Some(2)], [Some(2)], [Some(2)])
            .unwrap()
            .hamiltonian()
    );
    assert_eq!(
        &mo,
        MixedHamiltonianSystem::from_hamiltonian(mo.clone(), [Some(2)], [Some(2)], [Some(2)])
            .unwrap()
            .hamiltonian()
    );
    assert_eq!(
        MixedHamiltonianSystem::from_hamiltonian(mo.clone(), [Some(2)], [Some(2)], [Some(1)]),
        Err(StruqtureError::NumberSpinsExceeded {})
    );
}

// Test the current_number_spins function of the MixedHamiltonianSystem
#[test]
fn internal_map_current_number_spins_and_modes() {
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let pp_2: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(4)]);
    assert_eq!(mo.current_number_spins(), vec![0_usize]);
    assert_eq!(mo.current_number_bosonic_modes(), vec![0_usize]);
    assert_eq!(mo.current_number_fermionic_modes(), vec![0_usize]);

    mo.set(pp_0, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(mo.current_number_spins(), vec![1_usize]);
    assert_eq!(mo.current_number_bosonic_modes(), vec![2_usize]);
    assert_eq!(mo.current_number_fermionic_modes(), vec![3_usize]);

    mo.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(mo.current_number_spins(), vec![3_usize]);
    assert_eq!(mo.current_number_bosonic_modes(), vec![4_usize]);
    assert_eq!(mo.current_number_fermionic_modes(), vec![4_usize]);
}

// Test the len function of the SpinOperator
#[test]
fn internal_map_len() {
    let pp_2: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(3)]);
    mo.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(mo.len(), 1_usize);
}

// Test the iter, keys and values functions of the MixedHamiltonianSystem
#[test]
fn internal_map_keys() {
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(1)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedHamiltonianSystem::new([Some(2)], [Some(4)], [Some(3)]);
    let pp_0_vec: Vec<(HermitianMixedProduct, CalculatorComplex)> =
        vec![(pp_0.clone(), 0.3.into())];
    mo.extend(pp_0_vec.iter().cloned());

    let mut map: BTreeMap<HermitianMixedProduct, CalculatorComplex> = BTreeMap::new();
    map.insert(pp_0, CalculatorComplex::from(0.3));

    // iter
    let dict = mo.iter();
    for (item_d, item_m) in dict.zip(map.iter()) {
        assert_eq!(item_d, item_m);
    }
    // into_iter
    for (item_d, item_m) in mo.clone().into_iter().zip(map.iter()) {
        assert_eq!(item_d.0, *item_m.0);
        assert_eq!(item_d.1, *item_m.1);
    }
    // keys
    let keys = mo.keys();
    for (key_s, key_m) in keys.zip(map.keys()) {
        assert_eq!(key_s, key_m);
    }
    // values
    let values = mo.values();
    for (val_s, val_m) in values.zip(map.values()) {
        assert_eq!(val_s, val_m);
    }
}

// Test the set, get and remove functions of the SpinOperator
#[test]
fn internal_map_set_get_remove() {
    let pp_2: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(3)]);

    // 1) Test try_set_pauli_product and get functions
    // Vacant
    assert_eq!(mo.set(pp_2.clone(), 0.0.into()), Ok(None));
    mo.set(pp_2.clone(), CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(mo.get(&pp_2.clone()), &CalculatorComplex::from(0.5));
    assert_eq!(
        mo.set(pp_2.clone(), 0.0.into()),
        Ok(Some(CalculatorComplex::new(0.5, 0.0)))
    );
    // 2) Test remove function
    mo.remove(&pp_2);
    assert_eq!(
        mo,
        MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(3)])
    );
}

#[test]
fn set_fail_number_subsystems() {
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let pp_2: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedHamiltonianSystem::new([], [Some(2)], [Some(3)]);
    assert_eq!(mo.current_number_spins(), Vec::<usize>::new());
    assert_eq!(mo.current_number_bosonic_modes(), vec![0_usize]);
    assert_eq!(mo.current_number_fermionic_modes(), vec![0_usize]);

    let err = mo.set(pp_0, CalculatorComplex::from(0.5));
    assert_eq!(
        err,
        Err(StruqtureError::MissmatchedNumberSubsystems {
            target_number_spin_subsystems: 0,
            target_number_boson_subsystems: 1,
            target_number_fermion_subsystems: 1,
            actual_number_spin_subsystems: 1,
            actual_number_boson_subsystems: 1,
            actual_number_fermion_subsystems: 1,
        })
    );

    let mut mo = MixedHamiltonianSystem::new([Some(3)], [], [Some(3)]);
    assert_eq!(mo.current_number_spins(), vec![0_usize]);
    assert_eq!(mo.current_number_bosonic_modes(), Vec::<usize>::new());
    assert_eq!(mo.current_number_fermionic_modes(), vec![0_usize]);

    let err = mo.set(pp_2.clone(), CalculatorComplex::from(0.5));
    assert_eq!(
        err,
        Err(StruqtureError::MissmatchedNumberSubsystems {
            target_number_spin_subsystems: 1,
            target_number_boson_subsystems: 0,
            target_number_fermion_subsystems: 1,
            actual_number_spin_subsystems: 1,
            actual_number_boson_subsystems: 1,
            actual_number_fermion_subsystems: 1,
        })
    );

    let mut mo = MixedHamiltonianSystem::new([Some(3)], [Some(4)], []);
    assert_eq!(mo.current_number_spins(), vec![0_usize]);
    assert_eq!(mo.current_number_bosonic_modes(), vec![0_usize]);
    assert_eq!(mo.current_number_fermionic_modes(), Vec::<usize>::new());

    let err = mo.set(pp_2, CalculatorComplex::from(0.5));
    assert_eq!(
        err,
        Err(StruqtureError::MissmatchedNumberSubsystems {
            target_number_spin_subsystems: 1,
            target_number_boson_subsystems: 1,
            target_number_fermion_subsystems: 0,
            actual_number_spin_subsystems: 1,
            actual_number_boson_subsystems: 1,
            actual_number_fermion_subsystems: 1,
        })
    );
}

#[test]
fn set_fail_number_particles() {
    let mut mo = MixedHamiltonianSystem::new([Some(1)], [Some(1)], [Some(1)]);

    let pp: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(1)],
        [BosonProduct::new([], []).unwrap()],
        [FermionProduct::new([], []).unwrap()],
    )
    .unwrap();
    let err = mo.set(pp, CalculatorComplex::from(0.5));
    assert_eq!(err, Err(StruqtureError::MissmatchedNumberSpins));

    let pp: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new()],
        [BosonProduct::new([1], [1]).unwrap()],
        [FermionProduct::new([], []).unwrap()],
    )
    .unwrap();
    let err = mo.set(pp, CalculatorComplex::from(0.5));
    assert_eq!(err, Err(StruqtureError::MissmatchedNumberModes));

    let pp: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new()],
        [BosonProduct::new([], []).unwrap()],
        [FermionProduct::new([1], [1]).unwrap()],
    )
    .unwrap();
    let err = mo.set(pp, CalculatorComplex::from(0.5));
    assert_eq!(err, Err(StruqtureError::MissmatchedNumberModes));
}

// Test the add_operator_product function of the MixedHamiltonianSystem
#[test]
fn internal_map_add_operator_product() {
    let pp_2: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(3)]);

    mo.add_operator_product(pp_2.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(mo.get(&pp_2), &CalculatorComplex::from(0.5));
    mo.add_operator_product(pp_2.clone(), CalculatorComplex::from(-0.5))
        .unwrap();
    assert_eq!(mo.get(&pp_2), &CalculatorComplex::from(0.0));
}

#[test]
fn fail_add_operator_product_number_subsystems() {
    let pp_2: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();

    let mut mo = MixedHamiltonianSystem::new([], [Some(4)], [Some(3)]);
    let err = mo.add_operator_product(pp_2.clone(), CalculatorComplex::from(0.5));
    assert_eq!(
        err,
        Err(StruqtureError::MissmatchedNumberSubsystems {
            target_number_spin_subsystems: 0,
            target_number_boson_subsystems: 1,
            target_number_fermion_subsystems: 1,
            actual_number_spin_subsystems: 1,
            actual_number_boson_subsystems: 1,
            actual_number_fermion_subsystems: 1,
        })
    );

    let mut mo = MixedHamiltonianSystem::new([Some(3)], [], [Some(3)]);
    let err = mo.add_operator_product(pp_2.clone(), CalculatorComplex::from(0.5));
    assert_eq!(
        err,
        Err(StruqtureError::MissmatchedNumberSubsystems {
            target_number_spin_subsystems: 1,
            target_number_boson_subsystems: 0,
            target_number_fermion_subsystems: 1,
            actual_number_spin_subsystems: 1,
            actual_number_boson_subsystems: 1,
            actual_number_fermion_subsystems: 1,
        })
    );

    let mut mo = MixedHamiltonianSystem::new([Some(3)], [Some(4)], []);
    let err = mo.add_operator_product(pp_2, CalculatorComplex::from(0.5));
    assert_eq!(
        err,
        Err(StruqtureError::MissmatchedNumberSubsystems {
            target_number_spin_subsystems: 1,
            target_number_boson_subsystems: 1,
            target_number_fermion_subsystems: 0,
            actual_number_spin_subsystems: 1,
            actual_number_boson_subsystems: 1,
            actual_number_fermion_subsystems: 1,
        })
    );
}

#[test]
fn fail_add_operator_product_number_particles() {
    let pp_2: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();

    let mut mo = MixedHamiltonianSystem::new([Some(1)], [Some(4)], [Some(3)]);
    let err = mo.add_operator_product(pp_2.clone(), CalculatorComplex::from(0.5));
    assert_eq!(err, Err(StruqtureError::MissmatchedNumberSpins));

    let mut mo = MixedHamiltonianSystem::new([Some(3)], [Some(1)], [Some(3)]);
    let err = mo.add_operator_product(pp_2.clone(), CalculatorComplex::from(0.5));
    assert_eq!(err, Err(StruqtureError::MissmatchedNumberModes));

    let mut mo = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(1)]);
    let err = mo.add_operator_product(pp_2, CalculatorComplex::from(0.5));
    assert_eq!(err, Err(StruqtureError::MissmatchedNumberModes));
}

// Test the hermitian_conjugate and is_natural_hermitian functions of the HermitianMixedProduct
#[test]
fn hermitian_test() {
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut test_new = MixedHamiltonianSystem::new([Some(1)], [Some(2)], [Some(3)]);
    test_new.set(pp_0, CalculatorComplex::from(0.5)).unwrap();

    assert_eq!(test_new.hermitian_conjugate(), test_new.clone());
}

// Test the negative operation: -MixedHamiltonianSystem
#[test]
fn negative_mo() {
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo_0 = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(3)]);
    mo_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut mo_0_minus = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(3)]);
    mo_0_minus
        .add_operator_product(pp_0, CalculatorComplex::from(-1.0))
        .unwrap();

    assert_eq!(-mo_0, mo_0_minus);
}

// Test the addition: SpinOperator + SpinOperator
#[test]
fn add_so_so() {
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let pp_1: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(1)],
        [BosonProduct::new([1], [2]).unwrap()],
        [FermionProduct::new([1], [3]).unwrap()],
    )
    .unwrap();
    let mut mo_0 = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(4)]);
    mo_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut mo_1 = MixedHamiltonianSystem::new([Some(2)], [Some(3)], [Some(4)]);
    mo_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(-1.0))
        .unwrap();
    let mut mo_0_1 = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(4)]);
    mo_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    mo_0_1
        .add_operator_product(pp_1, CalculatorComplex::from(-1.0))
        .unwrap();

    assert_eq!(mo_0 + mo_1, Ok(mo_0_1));
}

// Test the addition: SpinOperator + SpinOperator
#[test]
fn sub_so_so() {
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let pp_1: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(1)],
        [BosonProduct::new([1], [2]).unwrap()],
        [FermionProduct::new([1], [3]).unwrap()],
    )
    .unwrap();
    let mut mo_0 = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(4)]);
    mo_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut mo_1 = MixedHamiltonianSystem::new([Some(2)], [Some(3)], [Some(4)]);
    mo_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(-1.0))
        .unwrap();
    let mut mo_0_1 = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(4)]);
    mo_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    mo_0_1
        .add_operator_product(pp_1, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(mo_0 - mo_1, Ok(mo_0_1));
}

#[test]
fn mul_so_so() {
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let pp_1: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(1)],
        [BosonProduct::new([1], [2]).unwrap()],
        [FermionProduct::new([1], [3]).unwrap()],
    )
    .unwrap();
    let mut mo_0 = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(4)]);
    mo_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut mo_1 = MixedHamiltonianSystem::new([Some(2)], [Some(3)], [Some(4)]);
    mo_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(2.0))
        .unwrap();

    let pp_0_1 = (pp_0 * pp_1).unwrap();
    let mut mo_0_1 = MixedSystem::new([Some(3)], [Some(4)], [Some(4)]);
    for pp in pp_0_1 {
        mo_0_1
            .add_operator_product(pp.0, CalculatorComplex::from(2.0) * pp.1)
            .unwrap();
    }

    assert_eq!(mo_0 * mo_1, Ok(mo_0_1));
}

// Test the multiplication: FermionOperator * FermionOperator
#[test]
fn mul_so_so_error() {
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let pp_1: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(1)],
        [],
        [FermionProduct::new([1], [3]).unwrap()],
    )
    .unwrap();
    let mut mo_0 = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(4)]);
    mo_0.add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    let mut mo_1 = MixedHamiltonianSystem::new([Some(2)], [], [Some(4)]);
    mo_1.add_operator_product(pp_1, CalculatorComplex::from(2.0))
        .unwrap();

    assert_eq!(
        mo_0 * mo_1,
        Err(StruqtureError::MissmatchedNumberSubsystems {
            target_number_spin_subsystems: 1,
            target_number_boson_subsystems: 1,
            target_number_fermion_subsystems: 1,
            actual_number_spin_subsystems: 1,
            actual_number_boson_subsystems: 0,
            actual_number_fermion_subsystems: 1,
        })
    );
}

// Test the multiplication: SpinOperator * Calculatorcomplex
#[test]
fn mul_so_cf() {
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo_0 = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(3)]);
    mo_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(2.0))
        .unwrap();
    let mut mo_0_1 = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(3)]);
    mo_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(6.0))
        .unwrap();

    assert_eq!(mo_0 * CalculatorFloat::from(3.0), mo_0_1);
}

// Test the multiplication: SpinOperator * Calculatorcomplex
#[test]
fn mul_so_cc() {
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo_0 = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(3)]);
    mo_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(2.0))
        .unwrap();
    let mut mo_0_1 = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(3)]);
    mo_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(6.0))
        .unwrap();

    assert_eq!(mo_0 * CalculatorComplex::from(3.0), mo_0_1);
}

// Test the Iter traits of FermionOperator: into_iter, from_iter and extend
#[test]
fn into_iter_from_iter_extend() {
    assert_eq!(
        MixedHamiltonianSystem::from_iter(MixedHamiltonianSystem::default().into_iter()),
        MixedHamiltonianSystem::new([], [], [])
    );
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let pp_1: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(0)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let pp_2: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(1)],
        [BosonProduct::new([1], [2]).unwrap()],
        [FermionProduct::new([1], [3]).unwrap()],
    )
    .unwrap();
    let mut mo_0 = MixedHamiltonianSystem::new([None], [None], [None]);
    mo_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(2.0))
        .unwrap();
    mo_0.add_operator_product(pp_1.clone(), CalculatorComplex::from(1.0))
        .unwrap();

    let mo_iter = mo_0.clone().into_iter();
    assert_eq!(MixedHamiltonianSystem::from_iter(mo_iter), mo_0);
    let mo_iter = (&mo_0)
        .into_iter()
        .map(|(key, value)| (key.clone(), value.clone()));
    assert_eq!(MixedHamiltonianSystem::from_iter(mo_iter), mo_0);
    let mut mapping: BTreeMap<HermitianMixedProduct, CalculatorComplex> = BTreeMap::new();
    mapping.insert(pp_2.clone(), CalculatorComplex::from(0.5));
    let mapping_iter = mapping.into_iter();
    mo_0.extend(mapping_iter);

    let mut mo_0_1 = MixedHamiltonianSystem::new([None], [None], [None]);
    let _ = mo_0_1.add_operator_product(pp_0, CalculatorComplex::from(2.0));
    let _ = mo_0_1.add_operator_product(pp_1, CalculatorComplex::from(1.0));
    let _ = mo_0_1.add_operator_product(pp_2, CalculatorComplex::from(0.5));

    assert_eq!(mo_0, mo_0_1);
}

// Test the from_iter function of the MixedHamiltonian
#[test]
fn from_iterator() {
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let pp_1: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().x(1)],
        [BosonProduct::new([1], [2]).unwrap()],
        [FermionProduct::new([1], [3]).unwrap()],
    )
    .unwrap();

    // iterator with two items
    let mut mo_0 = MixedHamiltonianSystem::new([None], [None], [None]);
    mo_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    mo_0.add_operator_product(pp_1.clone(), CalculatorComplex::from(2.0))
        .unwrap();
    let mut iterator: HashMap<HermitianMixedProduct, CalculatorComplex> = HashMap::new();
    iterator.insert(pp_0, 1.0.into());
    iterator.insert(pp_1, 2.0.into());
    assert_eq!(
        MixedHamiltonianSystem::from_iter(iterator.iter().map(|(x, y)| (x.clone(), y.clone()))),
        mo_0
    );

    // iterator with no items
    let mo_0 = MixedHamiltonianSystem::new([], [], []);
    let iterator: HashMap<HermitianMixedProduct, CalculatorComplex> = HashMap::new();
    assert_eq!(
        MixedHamiltonianSystem::from_iter(iterator.iter().map(|(x, y)| (x.clone(), y.clone()))),
        mo_0
    );
}

#[test]
fn default() {
    assert_eq!(
        MixedHamiltonianSystem::default(),
        MixedHamiltonianSystem::new([], [], [])
    );
}

// Test the Hash, Debug and Display traits of PauliProduct
#[test]
fn debug() {
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(4)]);
    mo.add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    assert_eq!(
        format!("{:?}", mo),
        "MixedHamiltonianSystem { number_spins: [Some(3)], number_bosons: [Some(4)], number_fermions: [Some(4)], hamiltonian: MixedHamiltonian { internal_map: {HermitianMixedProduct { spins: [PauliProduct { items: [(2, Z)] }], bosons: [BosonProduct { creators: [0], annihilators: [3] }], fermions: [FermionProduct { creators: [0], annihilators: [3] }] }: CalculatorComplex { re: Float(1.0), im: Float(0.0) }}, n_spins: 1, n_bosons: 1, n_fermions: 1 } }"
    );
}

// Test the Hash, Debug and Display traits of PauliProduct
#[test]
fn display() {
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let pp_1: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new()],
        [BosonProduct::new([0], [0]).unwrap()],
        [FermionProduct::new([], []).unwrap()],
    )
    .unwrap();
    let mut mo = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(4)]);
    mo.add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    mo.add_operator_product(pp_1, CalculatorComplex::from(2.0))
        .unwrap();
    assert_eq!(
        format!("{}", mo),
        format!(
            "MixedHamiltonianSystem(\nnumber_spins: 3, \nnumber_bosons: 4, \nnumber_fermions: 4, )\n{{SI:Bc0a0:FI:: {},\nS2Z:Bc0a3:Fc0a3:: {},\n}}",
            CalculatorComplex::from(2.0), CalculatorComplex::from(1.0)
        )
    );
}

// Test the Clone, PartialEq, PartialOrd and Ord traits of PauliProduct
#[test]
fn clone_partial_eq_partial_ord() {
    let pp_0: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(4)]);
    mo.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();

    // Test Clone trait
    assert_eq!(mo.clone(), mo);

    // Test PartialEq trait
    let mut mo_0 = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(4)]);
    mo_0.set(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut mo_1 = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(4)]);
    mo_1.set(pp_0, CalculatorComplex::from(2.0)).unwrap();
    assert!(mo_0 == mo);
    assert!(mo == mo_0);
    assert!(mo_1 != mo);
    assert!(mo != mo_1);
}

#[test]
fn serde_json() {
    let pp: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(4)]);
    mo.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let serialized = serde_json::to_string(&mo).unwrap();
    let deserialized: MixedHamiltonianSystem = serde_json::from_str(&serialized).unwrap();

    assert_eq!(mo, deserialized);
}

/// Test SpinOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_readable() {
    use struqture::MINIMUM_STRUQTURE_VERSION;
    let major_version = MINIMUM_STRUQTURE_VERSION.0;
    let minor_version = MINIMUM_STRUQTURE_VERSION.1;

    let pp: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(3)]);
    mo.set(pp, CalculatorComplex::from(0.5)).unwrap();
    assert_tokens(
        &mo.readable(),
        &[
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
            Token::U64(3),
            Token::SeqEnd,
            Token::Str("hamiltonian"),
            Token::Struct {
                name: "MixedHamiltonianSerialize",
                len: 5,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 3 },
            Token::Str("S2Z:Bc0a3:Fc0a2:"),
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
        ],
    );
}

#[test]
fn bincode() {
    let pp: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(3)]);
    mo.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let serialized = serialize(&mo).unwrap();
    let deserialized: MixedHamiltonianSystem = deserialize(&serialized).unwrap();
    assert_eq!(deserialized, mo);

    let encoded: Vec<u8> = bincode::serialize(&mo.clone().compact()).unwrap();
    let decoded: MixedHamiltonianSystem = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(mo, decoded);
}

#[test]
fn serde_compact() {
    use struqture::MINIMUM_STRUQTURE_VERSION;
    let major_version = MINIMUM_STRUQTURE_VERSION.0;
    let minor_version = MINIMUM_STRUQTURE_VERSION.1;

    let pp: HermitianMixedProduct = HermitianMixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedHamiltonianSystem::new([Some(3)], [Some(4)], [Some(3)]);
    mo.set(pp, CalculatorComplex::from(0.5)).unwrap();
    assert_tokens(
        &mo.compact(),
        &[
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
            Token::U64(3),
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
            Token::U64(2),
            Token::UnitVariant {
                name: "SingleSpinOperator",
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
            Token::U64(2),
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
        ],
    );
}

#[cfg(feature = "json_schema")]
#[test_case(None)]
#[test_case(Some(4))]
fn test_mixed_hamiltonian_system_schema(number_particles: Option<usize>) {
    let mut op = MixedHamiltonianSystem::new(
        [number_particles, number_particles],
        [number_particles],
        [number_particles],
    );
    let pp = HermitianMixedProduct::new(
        [PauliProduct::new().x(0), PauliProduct::new()],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    op.set(pp, 1.0.into()).unwrap();
    let pp = HermitianMixedProduct::new(
        [PauliProduct::new().x(1), PauliProduct::new()],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    op.set(pp, "val".into()).unwrap();
    let schema = schemars::schema_for!(MixedHamiltonianSystem);
    let schema_checker = jsonschema::validator_for(&serde_json::to_value(&schema).unwrap())
        .expect("schema is valid");
    let value = serde_json::to_value(&op).unwrap();
    let validation = schema_checker.validate(&value);

    assert!(validation.is_ok());
}
