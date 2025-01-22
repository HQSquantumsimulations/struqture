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

//! Integration test for public API of BosonHamiltonianSystem

use bincode::{deserialize, serialize};
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde_test::{assert_tokens, Configure, Token};
use std::collections::BTreeMap;
use std::iter::{FromIterator, IntoIterator};
use struqture::bosons::{
    BosonHamiltonian, BosonHamiltonianSystem, BosonProduct, BosonSystem, HermitianBosonProduct,
};
use struqture::{
    ModeIndex, OperateOnDensityMatrix, OperateOnModes, OperateOnState, StruqtureError,
};
use test_case::test_case;

// Test the new function of the SpinSystem
#[test]
fn new_system() {
    let system = BosonHamiltonianSystem::new(Some(1));
    assert!(system.is_empty());
    assert_eq!(system.hamiltonian(), &BosonHamiltonian::default());
    assert_eq!(system.number_modes(), 1_usize)
}

// Test the new function of the SpinSystem with no spins specified
#[test]
fn new_system_none() {
    let system = BosonHamiltonianSystem::new(None);
    assert!(system.hamiltonian().is_empty());
    assert_eq!(system.hamiltonian(), &BosonHamiltonian::default());
    assert_eq!(system.number_modes(), 0_usize);
    assert_eq!(
        BosonHamiltonianSystem::new(None),
        BosonHamiltonianSystem::default()
    );
}

#[test]
fn empty_clone_options() {
    let pp_2: HermitianBosonProduct = HermitianBosonProduct::new([0], [2]).unwrap();
    let mut system = BosonHamiltonianSystem::new(Some(3));
    system.set(pp_2, CalculatorComplex::from(0.5)).unwrap();

    let empty: Option<usize> = None;
    let full: Option<usize> = Some(3);
    assert_eq!(
        system.empty_clone(empty),
        BosonHamiltonianSystem::new(Some(3))
    );
    assert_eq!(
        system.empty_clone(full),
        BosonHamiltonianSystem::with_capacity(Some(3), 1)
    );
}

// Test the from_spin_operator and spin_operator functions of the BosonSystem with number_spins = None
#[test]
fn from_boson_operator_none() {
    let mut so: BosonHamiltonian = BosonHamiltonian::new();
    let mut system = BosonHamiltonianSystem::new(None);
    let pp: HermitianBosonProduct = HermitianBosonProduct::new([0], [2]).unwrap();
    so.add_operator_product(pp.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    system
        .add_operator_product(pp, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(
        system,
        BosonHamiltonianSystem::from_hamiltonian(so.clone(), None).unwrap()
    );
    assert_eq!(
        system.hamiltonian(),
        BosonHamiltonianSystem::from_hamiltonian(so.clone(), None)
            .unwrap()
            .hamiltonian()
    );
    assert_eq!(
        &so,
        BosonHamiltonianSystem::from_hamiltonian(so.clone(), None)
            .unwrap()
            .hamiltonian()
    );
}

// Test the from_spin_operator and spin_operator functions of the BosonSystem with number_spins = Some(2)
#[test]
fn from_boson_operator_some() {
    let mut so: BosonHamiltonian = BosonHamiltonian::new();
    let mut system = BosonHamiltonianSystem::new(Some(2));
    let pp: HermitianBosonProduct = HermitianBosonProduct::new([0], [1]).unwrap();
    so.add_operator_product(pp.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    system
        .add_operator_product(pp, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(
        system,
        BosonHamiltonianSystem::from_hamiltonian(so.clone(), Some(2)).unwrap()
    );
    assert_eq!(
        system.hamiltonian(),
        BosonHamiltonianSystem::from_hamiltonian(so.clone(), Some(2))
            .unwrap()
            .hamiltonian()
    );
    assert_eq!(
        &so,
        BosonHamiltonianSystem::from_hamiltonian(so.clone(), Some(2))
            .unwrap()
            .hamiltonian()
    );
    assert_eq!(
        BosonHamiltonianSystem::from_hamiltonian(so.clone(), Some(0)),
        Err(StruqtureError::NumberModesExceeded {})
    );
}

// Test the current_number_modes function of the BosonHamiltonianSystem
#[test]
fn internal_map_current_number_modes() {
    let pp_0: HermitianBosonProduct = HermitianBosonProduct::new([0], [1]).unwrap();
    let pp_2: HermitianBosonProduct = HermitianBosonProduct::new([2], [3]).unwrap();
    let mut so = BosonHamiltonianSystem::new(Some(4));
    assert_eq!(so.current_number_modes(), 0_usize);
    so.set(pp_0, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.current_number_modes(), 2_usize);
    so.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.current_number_modes(), 4_usize);
}

// Test the len function of the BosonHamiltonianSystem
#[test]
fn internal_map_len() {
    let pp_2: HermitianBosonProduct = HermitianBosonProduct::new([0], [2]).unwrap();
    let mut so = BosonHamiltonianSystem::new(Some(3));
    so.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.len(), 1_usize);
}
// Test the set, set_pauli_product, get functions of the SpinSystem
#[test]
fn internal_map_set_get_dict() {
    let mut system = BosonHamiltonianSystem::new(Some(1));
    assert_eq!(system.current_number_modes(), 0_usize);
    assert_eq!(system.number_modes(), 1_usize);
    let pp_0: HermitianBosonProduct = HermitianBosonProduct::new([0], [0]).unwrap();

    // 1) Test set and get functions
    // Vacant
    system
        .set(pp_0.clone(), CalculatorComplex::from(0.0))
        .unwrap();
    system
        .set(pp_0.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(system.current_number_modes(), 1_usize);
    assert_eq!(system.number_modes(), 1_usize);
    assert_eq!(system.get(&pp_0), &CalculatorComplex::from(0.5));

    // 2) Test iter, keys, values functions
    let mut map: BTreeMap<HermitianBosonProduct, CalculatorComplex> = BTreeMap::new();
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

// Test the set, get and remove functions of the BosonHamiltonianSystem
#[test]
fn internal_map_set_get_remove() {
    let pp_2: HermitianBosonProduct = HermitianBosonProduct::new([0], [2]).unwrap();
    let mut so = BosonHamiltonianSystem::new(Some(3));

    // 1) Test try_set_boson_product and get functions
    // Vacant
    so.set(pp_2.clone(), CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(so.get(&pp_2.clone()), &CalculatorComplex::from(0.5));

    // 2) Test remove function
    so.remove(&pp_2);
    assert_eq!(so, BosonHamiltonianSystem::new(Some(3)));
}

// Test the add_operator_product function of the BosonHamiltonianSystem
#[test]
fn internal_map_add_operator_product() {
    let pp_2: HermitianBosonProduct = HermitianBosonProduct::new([0], [2]).unwrap();
    let mut so = BosonHamiltonianSystem::new(Some(3));

    so.add_operator_product(pp_2.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(so.get(&pp_2), &CalculatorComplex::from(0.5));
    so.add_operator_product(pp_2.clone(), CalculatorComplex::from(-0.5))
        .unwrap();
    assert_eq!(so.get(&pp_2), &CalculatorComplex::from(0.0));
}

// Test the iter, keys and values functions of the BosonHamiltonianSystem
#[test]
fn internal_map_keys() {
    let pp_2: HermitianBosonProduct = HermitianBosonProduct::new([0], [2]).unwrap();
    let mut so = BosonHamiltonianSystem::new(Some(3));
    so.set(pp_2.clone(), CalculatorComplex::from(0.5)).unwrap();

    let mut map: BTreeMap<HermitianBosonProduct, CalculatorComplex> = BTreeMap::new();
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

// Test the hermitian_conjugate and is_natural_hermitian functions of the HermitianMixedProduct
#[test]
fn hermitian_test() {
    let pp: HermitianBosonProduct = HermitianBosonProduct::new([0], [0]).unwrap();
    let mut system = BosonHamiltonianSystem::new(Some(1));
    let _ = system.add_operator_product(pp, CalculatorComplex::from(1.0));

    assert_eq!(system.hermitian_conjugate(), system.clone());
}

// Test the separation of terms
#[test_case((1, 1))]
#[test_case((1, 2))]
#[test_case((2, 1))]
#[test_case((2, 2))]
fn separate_out_terms(number_spins: (usize, usize)) {
    let pp_1_a: HermitianBosonProduct = HermitianBosonProduct::new([0], [0]).unwrap();
    let pp_1_b: HermitianBosonProduct = HermitianBosonProduct::new([1], [1]).unwrap();
    let pp_2_a: HermitianBosonProduct = HermitianBosonProduct::new([0, 1], [1]).unwrap();
    let pp_2_b: HermitianBosonProduct = HermitianBosonProduct::new([0], [0, 1]).unwrap();
    let pp_3_a: HermitianBosonProduct = HermitianBosonProduct::new([0, 1], [0, 1]).unwrap();
    let pp_3_b: HermitianBosonProduct = HermitianBosonProduct::new([0, 2], [0, 2]).unwrap();

    let mut allowed: Vec<(HermitianBosonProduct, f64)> = Vec::new();
    let mut not_allowed: Vec<(HermitianBosonProduct, f64)> = vec![
        (pp_1_a.clone(), 1.0),
        (pp_1_b.clone(), 1.1),
        (pp_2_a.clone(), 1.2),
        (pp_2_b.clone(), 1.3),
        (pp_3_a.clone(), 1.4),
        (pp_3_b.clone(), 1.5),
    ];

    match number_spins {
        (1, 1) => {
            allowed.push((pp_1_a.clone(), 1.0));
            allowed.push((pp_1_b.clone(), 1.1));
            not_allowed.remove(0);
            not_allowed.remove(0);
        }
        (2, 1) => {
            allowed.push((pp_2_a.clone(), 1.2));
            not_allowed.remove(2);
        }
        (1, 2) => {
            allowed.push((pp_2_b.clone(), 1.3));
            not_allowed.remove(3);
        }
        (2, 2) => {
            allowed.push((pp_3_a.clone(), 1.4));
            allowed.push((pp_3_b.clone(), 1.5));
            not_allowed.remove(4);
            not_allowed.remove(4);
        }
        _ => panic!(),
    }

    let mut separated = BosonHamiltonianSystem::default();
    for (key, value) in allowed.iter() {
        separated
            .add_operator_product(key.clone(), value.into())
            .unwrap();
    }
    let mut remainder = BosonHamiltonianSystem::default();
    for (key, value) in not_allowed.iter() {
        remainder
            .add_operator_product(key.clone(), value.into())
            .unwrap();
    }

    let mut so = BosonHamiltonianSystem::default();
    so.add_operator_product(pp_1_a, CalculatorComplex::from(1.0))
        .unwrap();
    so.add_operator_product(pp_1_b, CalculatorComplex::from(1.1))
        .unwrap();
    so.add_operator_product(pp_2_a, CalculatorComplex::from(1.2))
        .unwrap();
    so.add_operator_product(pp_2_b, CalculatorComplex::from(1.3))
        .unwrap();
    so.add_operator_product(pp_3_a, CalculatorComplex::from(1.4))
        .unwrap();
    so.add_operator_product(pp_3_b, CalculatorComplex::from(1.5))
        .unwrap();

    let result = so.separate_into_n_terms(number_spins).unwrap();
    assert_eq!(result.0, separated);
    assert_eq!(result.1, remainder);
}

// Test the negative operation: -BosonHamiltonianSystem
#[test]
fn negative_so() {
    let pp_0: HermitianBosonProduct = HermitianBosonProduct::new([0], [0]).unwrap();
    let mut so_0 = BosonHamiltonianSystem::new(Some(1));
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0));
    let mut so_0_minus = BosonHamiltonianSystem::new(Some(1));
    let _ = so_0_minus.add_operator_product(pp_0, CalculatorComplex::from(-1.0));

    assert_eq!(-so_0, so_0_minus);
}

// Test the addition: BosonHamiltonianSystem + BosonHamiltonianSystem
#[test]
fn add_so_so() {
    let pp_0: HermitianBosonProduct = HermitianBosonProduct::new([0], [0]).unwrap();
    let pp_1: HermitianBosonProduct = HermitianBosonProduct::new([0], [1]).unwrap();
    let mut so_0 = BosonHamiltonianSystem::new(Some(2));
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0));
    let mut so_1 = BosonHamiltonianSystem::new(Some(2));
    let _ = so_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(0.5));
    let mut so_0_1 = BosonHamiltonianSystem::new(Some(2));
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(1.0));
    let _ = so_0_1.add_operator_product(pp_1, CalculatorComplex::from(0.5));

    assert_eq!(so_0 + so_1, Ok(so_0_1));
}

// Test the subtraction: BosonHamiltonianSystem - BosonHamiltonianSystem
#[test]
fn sub_so_so() {
    let pp_0: HermitianBosonProduct = HermitianBosonProduct::new([0], [0]).unwrap();
    let pp_1: HermitianBosonProduct = HermitianBosonProduct::new([0], [1]).unwrap();
    let mut so_0 = BosonHamiltonianSystem::new(Some(2));
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0));
    let mut so_1 = BosonHamiltonianSystem::new(Some(2));
    let _ = so_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(0.5));
    let mut so_0_1 = BosonHamiltonianSystem::new(Some(2));
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(1.0));
    let _ = so_0_1.add_operator_product(pp_1, CalculatorComplex::from(-0.5));

    assert_eq!(so_0 - so_1, Ok(so_0_1));
}

// Test the multiplication: BosonHamiltonianSystem * BosonHamiltonianSystem
#[test]
fn mul_so_so() {
    let pp_0: HermitianBosonProduct = HermitianBosonProduct::new([0], [0]).unwrap();
    let pp_1: HermitianBosonProduct = HermitianBosonProduct::new([1], [1]).unwrap();
    let pp_0_1: BosonProduct = BosonProduct::new([0, 1], [0, 1]).unwrap();
    let mut so_0 = BosonHamiltonianSystem::new(Some(2));
    let _ = so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0));
    let mut so_1 = BosonHamiltonianSystem::new(Some(2));
    let _ = so_1.add_operator_product(pp_1, CalculatorComplex::from(0.5));
    let mut so_0_1 = BosonSystem::new(Some(2));
    let _ = so_0_1.add_operator_product(pp_0_1, CalculatorComplex::from(1.0));
    assert_eq!((so_0 * so_1), Ok(so_0_1));
}

// Test the multiplication: BosonHamiltonianSystem * BosonHamiltonianSystem
#[test]
fn mul_so_so_help() {
    let pp_0: HermitianBosonProduct = HermitianBosonProduct::new([0], [1]).unwrap();
    let pp_1: HermitianBosonProduct = HermitianBosonProduct::new([2], [3]).unwrap();
    let mut so_0 = BosonHamiltonianSystem::new(Some(4));
    let _ = so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0));
    let mut so_1 = BosonHamiltonianSystem::new(Some(4));
    let _ = so_1.add_operator_product(pp_1, CalculatorComplex::from(0.5));
    let mut so_0_1 = BosonSystem::new(Some(4));
    let _ = so_0_1.add_operator_product(
        BosonProduct::new([0, 2], [1, 3]).unwrap(),
        CalculatorComplex::from(1.0),
    );
    let _ = so_0_1.add_operator_product(
        BosonProduct::new([0, 3], [1, 2]).unwrap(),
        CalculatorComplex::from(1.0),
    );
    let _ = so_0_1.add_operator_product(
        BosonProduct::new([1, 2], [0, 3]).unwrap(),
        CalculatorComplex::from(1.0),
    );
    let _ = so_0_1.add_operator_product(
        BosonProduct::new([1, 3], [0, 2]).unwrap(),
        CalculatorComplex::from(1.0),
    );
    assert_eq!((so_0 * so_1), Ok(so_0_1));
}

// Test the multiplication: BosonHamiltonianSystem * BosonHamiltonianSystem where they have a HermitianBosonProduct with the same index
#[test]
fn mul_so_so_same_index() {
    let pp_0: HermitianBosonProduct = HermitianBosonProduct::new([0], [0]).unwrap();
    let pp_1: HermitianBosonProduct = HermitianBosonProduct::new([0], [0]).unwrap();
    let pp_0_1: BosonProduct = BosonProduct::new([0, 0], [0, 0]).unwrap();
    let pp_0_1_comm: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut so_0 = BosonHamiltonianSystem::new(Some(1));
    let _ = so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0));
    let mut so_1 = BosonHamiltonianSystem::new(Some(1));
    let _ = so_1.add_operator_product(pp_1, CalculatorComplex::from(0.5));
    let mut so_0_1 = BosonSystem::new(Some(1));
    let _ = so_0_1.add_operator_product(pp_0_1, CalculatorComplex::new(1.0, 0.0));
    let _ = so_0_1.add_operator_product(pp_0_1_comm, CalculatorComplex::new(1.0, 0.0));

    assert_eq!(so_0 * so_1, Ok(so_0_1));
}

// Test the multiplication: BosonHamiltonianSystem * Calculatorcomplex
#[test]
fn mul_so_cf() {
    let pp_0: HermitianBosonProduct = HermitianBosonProduct::new([0], [2]).unwrap();
    let pp_1: HermitianBosonProduct = HermitianBosonProduct::new([0], [2]).unwrap();
    let mut so_0 = BosonHamiltonianSystem::new(Some(3));
    let _ = so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0));
    let mut so_0_1 = BosonHamiltonianSystem::new(Some(3));
    let _ = so_0_1.add_operator_product(pp_1, CalculatorComplex::from(6.0));

    assert_eq!(so_0 * CalculatorFloat::from(3.0), so_0_1);
}

// Test the multiplication: BosonHamiltonianSystem * Calculatorcomplex
#[test]
fn mul_so_cc_natural_hermitian() {
    let pp_0: HermitianBosonProduct = HermitianBosonProduct::new([0], [0]).unwrap();
    let pp_1: BosonProduct = BosonProduct::new([0], [0]).unwrap();
    let mut so_0 = BosonHamiltonianSystem::new(Some(1));
    let _ = so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0));
    let mut so_0_1 = BosonSystem::new(Some(1));
    let _ = so_0_1.add_operator_product(pp_1, CalculatorComplex::from(6.0));

    assert_eq!(so_0 * CalculatorComplex::from(3.0), so_0_1);
}

// Test the multiplication: BosonHamiltonianSystem * Calculatorcomplex
#[test]
fn mul_so_cc() {
    let pp_0: HermitianBosonProduct = HermitianBosonProduct::new([0], [2]).unwrap();
    let pp_1: BosonProduct = BosonProduct::new([0], [2]).unwrap();
    let pp_2: BosonProduct = BosonProduct::new([2], [0]).unwrap();
    let mut so_0 = BosonHamiltonianSystem::new(Some(3));
    let _ = so_0.add_operator_product(pp_0, CalculatorComplex::from(2.0));
    let mut so_0_1 = BosonSystem::new(Some(3));
    let _ = so_0_1.add_operator_product(pp_1, CalculatorComplex::new(2.0, 6.0));
    let _ = so_0_1.add_operator_product(pp_2, CalculatorComplex::new(2.0, 6.0));

    assert_eq!(so_0 * CalculatorComplex::new(1.0, 3.0), so_0_1);
}

// Test the Iter traits of BosonHamiltonianSystem: into_iter, from_iter and extend
#[test]
fn into_iter_from_iter_extend() {
    let pp_0: HermitianBosonProduct = HermitianBosonProduct::new([0], [0]).unwrap();
    let pp_1: HermitianBosonProduct = HermitianBosonProduct::new([0], [1]).unwrap();
    let mut so_0 = BosonHamiltonianSystem::new(None);
    let _ = so_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0));

    let so_iter = so_0.clone().into_iter();
    assert_eq!(BosonHamiltonianSystem::from_iter(so_iter), so_0);
    let system_iter = (&so_0)
        .into_iter()
        .map(|(key, value)| (key.clone(), value.clone()));
    assert_eq!(BosonHamiltonianSystem::from_iter(system_iter), so_0);

    let mut mapping: BTreeMap<HermitianBosonProduct, CalculatorComplex> = BTreeMap::new();
    mapping.insert(pp_1.clone(), CalculatorComplex::from(0.5));
    let mapping_iter = mapping.into_iter();
    so_0.extend(mapping_iter);

    let mut so_0_1 = BosonHamiltonianSystem::new(None);
    let _ = so_0_1.add_operator_product(pp_0, CalculatorComplex::from(1.0));
    let _ = so_0_1.add_operator_product(pp_1, CalculatorComplex::from(0.5));

    assert_eq!(so_0, so_0_1);
}

// Test the Debug trait of BosonHamiltonianSystem
#[test]
fn debug() {
    let pp: HermitianBosonProduct = HermitianBosonProduct::new([0], [0]).unwrap();
    let mut so = BosonHamiltonianSystem::new(Some(1));
    let _ = so.set(pp, CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{:?}", so),
        "BosonHamiltonianSystem { number_modes: Some(1), hamiltonian: BosonHamiltonian { internal_map: {HermitianBosonProduct { creators: [0], annihilators: [0] }: CalculatorComplex { re: Float(0.5), im: Float(0.0) }} } }"
    );
}

// Test the Display trait of BosonHamiltonianSystem
#[test]
fn display() {
    let pp: HermitianBosonProduct = HermitianBosonProduct::new([0], [0]).unwrap();
    let mut so = BosonHamiltonianSystem::new(Some(1));
    let _ = so.set(pp, CalculatorComplex::from(0.5));

    assert_eq!(
        format!("{}", so),
        "BosonHamiltonianSystem(1){\nc0a0: (5e-1 + i * 0e0),\n}"
    );
}

// Test the Clone and PartialEq traits of BosonHamiltonianSystem
#[test]
fn clone_partial_eq() {
    let pp: HermitianBosonProduct = HermitianBosonProduct::new([0], [0]).unwrap();
    let mut so = BosonHamiltonianSystem::new(Some(1));
    so.set(pp, CalculatorComplex::from(0.5)).unwrap();

    // Test Clone trait
    assert_eq!(so.clone(), so);

    // Test PartialEq trait
    let pp_1: HermitianBosonProduct = HermitianBosonProduct::new([0], [0]).unwrap();
    let mut so_1 = BosonHamiltonianSystem::new(Some(1));
    so_1.set(pp_1, CalculatorComplex::from(0.5)).unwrap();
    let pp_2: HermitianBosonProduct = HermitianBosonProduct::new([0], [2]).unwrap();
    let mut so_2 = BosonHamiltonianSystem::new(Some(3));
    so_2.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert!(so_1 == so);
    assert!(so == so_1);
    assert!(so_2 != so);
    assert!(so != so_2);
}

/// Test BosonHamiltonianSystem Serialization and Deserialization traits (readable)

#[test]
fn serde_json() {
    let pp = HermitianBosonProduct::new([0], [2]).unwrap();
    let mut so = BosonHamiltonianSystem::new(Some(3));
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let serialized = serde_json::to_string(&so).unwrap();
    let deserialized: BosonHamiltonianSystem = serde_json::from_str(&serialized).unwrap();
    assert_eq!(so, deserialized);
}

/// Test SpinOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_readable() {
    use struqture::MINIMUM_STRUQTURE_VERSION;
    let major_version = MINIMUM_STRUQTURE_VERSION.0;
    let minor_version = MINIMUM_STRUQTURE_VERSION.1;

    let pp = HermitianBosonProduct::new([0], [2]).unwrap();
    let mut so = BosonHamiltonianSystem::new(Some(3));
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    assert_tokens(
        &so.readable(),
        &[
            Token::Struct {
                name: "BosonHamiltonianSystem",
                len: 2,
            },
            Token::Str("number_modes"),
            Token::Some,
            Token::U64(3),
            Token::Str("hamiltonian"),
            Token::Struct {
                name: "BosonHamiltonianSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 3 },
            Token::Str("c0a2"),
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
        ],
    );
}

/// Test BosonHamiltonianSystem Serialization and Deserialization traits (compact)
#[test]
fn bincode() {
    let pp = HermitianBosonProduct::new([0], [1]).unwrap();
    let mut so = BosonHamiltonianSystem::new(Some(2));
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let serialized = serialize(&so).unwrap();
    let deserialized: BosonHamiltonianSystem = deserialize(&serialized).unwrap();
    assert_eq!(deserialized, so);

    let serialized = serialize(&so.clone().compact()).unwrap();
    let deserialized: BosonHamiltonianSystem = deserialize(&serialized).unwrap();
    assert_eq!(deserialized, so);
}

#[test]
fn serde_compact() {
    use struqture::MINIMUM_STRUQTURE_VERSION;
    let major_version = MINIMUM_STRUQTURE_VERSION.0;
    let minor_version = MINIMUM_STRUQTURE_VERSION.1;

    let pp = HermitianBosonProduct::new([0], [2]).unwrap();
    let mut so = BosonHamiltonianSystem::new(Some(3));
    so.set(pp, CalculatorComplex::from(1.0)).unwrap();

    assert_tokens(
        &so.compact(),
        &[
            Token::Struct {
                name: "BosonHamiltonianSystem",
                len: 2,
            },
            Token::Str("number_modes"),
            Token::Some,
            Token::U64(3),
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
            Token::U64(2),
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
        ],
    );
}

#[cfg(feature = "json_schema")]
#[test_case(None)]
#[test_case(Some(3))]
fn test_boson_hamiltonian_system_schema(number_bosons: Option<usize>) {
    let mut op = BosonHamiltonianSystem::new(number_bosons);
    op.set(HermitianBosonProduct::new([0], [0]).unwrap(), 1.0.into())
        .unwrap();
    op.set(HermitianBosonProduct::new([1], [1]).unwrap(), "val".into())
        .unwrap();
    let schema = schemars::schema_for!(BosonHamiltonianSystem);
    let schema_checker = jsonschema::validator_for(&serde_json::to_value(&schema).unwrap())
        .expect("schema is valid");
    let value = serde_json::to_value(&op).unwrap();
    let validation = schema_checker.validate(&value);

    assert!(validation.is_ok());
}
