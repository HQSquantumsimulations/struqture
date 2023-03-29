// Copyright © 2021-2022 HQS Quantum Simulations GmbH. All Rights Reserved.
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

//! Integration test for public API of SpinSystem

use super::create_na_matrix_from_operator_list;
use num_complex::Complex64;
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde_test::{assert_tokens, Configure, Token};
use std::collections::{BTreeMap, HashMap};
use std::iter::{FromIterator, IntoIterator};
use std::ops::{Add, Sub};
use std::str::FromStr;
use struqture::prelude::*;
use struqture::spins::{
    OperateOnSpins, PauliProduct, SpinOperator, SpinSystem, ToSparseMatrixOperator,
};
use struqture::{CooSparseMatrix, OperateOnDensityMatrix, SpinIndex, StruqtureError};
use test_case::test_case;

// Test the new function of the SpinSystem
#[test]
fn new_system() {
    let system = SpinSystem::new(Some(1));
    assert!(system.is_empty());
    assert_eq!(system.operator(), &SpinOperator::default());
    assert_eq!(system.number_spins(), 1_usize)
}

// Test the new function of the SpinSystem with no spins specified
#[test]
fn new_system_none() {
    let system = SpinSystem::new(None);
    assert!(system.operator().is_empty());
    assert_eq!(system.operator(), &SpinOperator::default());
    assert_eq!(system.number_spins(), 0_usize);
    assert_eq!(system, SpinSystem::default());
}

#[test]
fn empty_clone_options() {
    let pp_2: PauliProduct = PauliProduct::new().z(2);
    let mut system = SpinSystem::new(Some(3));
    system.set(pp_2, CalculatorComplex::from(0.5)).unwrap();

    let empty: Option<usize> = None;
    let full: Option<usize> = Some(3);
    assert_eq!(system.empty_clone(empty), SpinSystem::new(Some(3)));
    assert_eq!(system.empty_clone(full), SpinSystem::with_capacity(full, 1));
}

// Test the len function of the SpinSystem
#[test]
fn internal_map_len() {
    let pp_2: PauliProduct = PauliProduct::new().z(2);
    let mut system = SpinSystem::new(None);
    system.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(system.len(), 1_usize);
}

// Test the set, set_pauli_product, get functions of the SpinSystem
#[test]
fn internal_map_set_get_dict() {
    let mut system = SpinSystem::new(Some(1));
    assert_eq!(system.number_spins(), 1_usize);
    let pp_0: PauliProduct = PauliProduct::new().z(0);

    // 1) Test try_set_pauli_product and get functions
    // Vacant
    system
        .set(pp_0.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(system.get(&pp_0), &CalculatorComplex::from(0.5));

    // Occupied + Error 2
    let pp_2 = PauliProduct::new().x(2);
    let error = system.set(pp_2, CalculatorComplex::from(0.1));
    assert!(error.is_err());
    assert_eq!(error, Err(StruqtureError::NumberSpinsExceeded));

    // 2) Test iter, keys, values functions
    let mut map: BTreeMap<PauliProduct, CalculatorComplex> = BTreeMap::new();
    map.insert(pp_0.clone(), CalculatorComplex::from(0.5));
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

    // 3) Test set_pauli_product and get functions
    // Vacant
    let mut system = SpinSystem::new(Some(1));
    system
        .set(pp_0.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(system.get(&pp_0), &CalculatorComplex::from(0.5));
    // Occupied + Error 2
    let pp_2 = PauliProduct::new().x(2);
    let error = system.set(pp_2, CalculatorComplex::from(0.1));
    assert!(error.is_err());
    assert_eq!(error, Err(StruqtureError::NumberSpinsExceeded));
}

// Test the get and remove functions of the spinOperator
#[test]
fn internal_map_set_get_remove() {
    let pp: PauliProduct = PauliProduct::new().z(0);
    let mut system = SpinSystem::new(Some(1));

    // 1) Test try_set_spin_product and get functions
    // Vacant
    system.set(pp.clone(), 0.5.into()).unwrap();
    assert_eq!(system.get(&pp.clone()), &0.5.into());

    // 2) Test remove function
    system.remove(&pp);
    assert_eq!(system, SpinSystem::new(Some(1)));
}

// Test the add_pauli_product function of the SpinSystem
#[test]
fn internal_map_add_operator_product() {
    let mut system = SpinSystem::new(Some(2));
    let pp: PauliProduct = PauliProduct::new().z(0);
    system
        .add_operator_product(pp.clone(), CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(system.get(&pp), &CalculatorComplex::from(1.0));

    let pp_2 = PauliProduct::new().x(2);
    let error = system.add_operator_product(pp_2, CalculatorComplex::from(0.1));
    assert!(error.is_err());
    assert_eq!(error, Err(StruqtureError::NumberSpinsExceeded));
}

// Test the from_spin_operator and spin_operator functions of the SpinSystem with number_spins = None
#[test]
fn from_spin_operator_none() {
    let mut so: SpinOperator = SpinOperator::new();
    let mut system = SpinSystem::new(None);
    let pp: PauliProduct = PauliProduct::new().z(0);
    so.add_operator_product(pp.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    system
        .add_operator_product(pp, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(system, SpinSystem::from_operator(so.clone(), None).unwrap());
    assert_eq!(
        system.operator(),
        SpinSystem::from_operator(so.clone(), None)
            .unwrap()
            .operator()
    );
    assert_eq!(
        &so,
        SpinSystem::from_operator(so.clone(), None)
            .unwrap()
            .operator()
    );
}

// Test the from_spin_operator and spin_operator functions of the SpinSystem with number_spins = Some(2)
#[test]
fn from_spin_operator_some() {
    let mut so: SpinOperator = SpinOperator::new();
    let mut system = SpinSystem::new(Some(2));
    let pp: PauliProduct = PauliProduct::new().z(0);
    so.add_operator_product(pp.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    system
        .add_operator_product(pp, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(
        system,
        SpinSystem::from_operator(so.clone(), Some(2)).unwrap()
    );
    assert_eq!(
        system.operator(),
        SpinSystem::from_operator(so.clone(), Some(2))
            .unwrap()
            .operator()
    );
    assert_eq!(
        &so,
        SpinSystem::from_operator(so.clone(), Some(2))
            .unwrap()
            .operator()
    );
    assert_eq!(
        SpinSystem::from_operator(so.clone(), Some(0)),
        Err(StruqtureError::NumberSpinsExceeded {})
    );
}

// Test the Iter traits of SpinSystem: into_iter, from_iter and extend
#[test]
fn into_iter_from_iter_extend() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let pp_1: PauliProduct = PauliProduct::new().x(1);
    let mut system = SpinSystem::new(None);
    system
        .add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();

    let system_iter = system.clone().into_iter();
    assert_eq!(SpinSystem::from_iter(system_iter), system);
    let system_iter = (&system)
        .into_iter()
        .map(|(key, value)| (key.clone(), value.clone()));
    assert_eq!(SpinSystem::from_iter(system_iter), system);

    let mut hamiltonian = SpinOperator::new();
    hamiltonian
        .add_operator_product(pp_0.clone(), 1.0.into())
        .unwrap();
    for (first, second) in system.into_iter().zip(hamiltonian.iter()) {
        assert_eq!(first.0, *second.0);
        assert_eq!(first.1, *second.1);
    }

    let mut system = SpinSystem::new(Some(2));
    system
        .add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut mapping: BTreeMap<PauliProduct, CalculatorComplex> = BTreeMap::new();
    mapping.insert(pp_1.clone(), CalculatorComplex::from(0.5));
    let mapping_iter = mapping.into_iter();
    system.extend(mapping_iter);

    let mut system_1 = SpinSystem::new(Some(2));
    system_1
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    system_1
        .add_operator_product(pp_1, CalculatorComplex::from(0.5))
        .unwrap();

    assert_eq!(system, system_1);
}

// Test the Iter traits of SpinSystem: extend with a panic
#[test]
#[should_panic]
fn iter_extend_panic() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let pp_1: PauliProduct = PauliProduct::new().x(1);
    let mut system = SpinSystem::new(Some(1));
    system
        .add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();

    let system_iter = system.clone().into_iter();
    assert_eq!(SpinSystem::from_iter(system_iter), system);

    let mut mapping: BTreeMap<PauliProduct, CalculatorComplex> = BTreeMap::new();
    mapping.insert(pp_1.clone(), CalculatorComplex::from(0.5));
    let mapping_iter = mapping.into_iter();
    system.extend(mapping_iter);

    let mut system_0 = SpinSystem::new(Some(2));
    system_0.add_operator_product(pp_0, 1.0.into()).unwrap();
    system_0.add_operator_product(pp_1, 0.5.into()).unwrap();
    assert_eq!(system, system_0);
}

// Test the hermitian_conjugate and is_natural_hermitian functions of the HermitianMixedProduct
#[test]
fn hermitian_test() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let mut system = SpinSystem::new(Some(1));
    system
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(system.hermitian_conjugate(), system.clone());
}

// Test the Debug trait of SpinSystem
#[test]
fn debug_system() {
    let system = SpinSystem::new(Some(2));

    assert_eq!(
        format!("{:?}", system),
        "SpinSystem { number_spins: Some(2), operator: SpinOperator { internal_map: {} } }"
    );
}

// Test the Display trait of SpinSystem
#[test]
fn display_system() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let mut system = SpinSystem::new(Some(1));
    system
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(
        format!("{}", system),
        "SpinSystem(1){\n0Z: (1e0 + i * 0e0),\n}"
    );
}

#[test]
fn matrices() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let mut system = SpinSystem::new(Some(1));
    system
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(
        system.sparse_lindblad_entries().unwrap()[0].0,
        (vec![], (vec![], vec![]))
    );
    assert_eq!(
        system.sparse_lindblad_entries().unwrap()[0].1,
        (vec![], (vec![], vec![]))
    );
    assert_eq!(
        system.sparse_lindblad_entries().unwrap()[0].2,
        Complex64::default()
    );

    let unitary_matrix: CooSparseMatrix =
        (vec![1.0.into(), (-1.0).into()], (vec![0, 1], vec![0, 1]));
    assert_eq!(system.unitary_sparse_matrix_coo().unwrap(), unitary_matrix);

    let mut superoperator_matrix: HashMap<usize, HashMap<usize, Complex64>> = HashMap::new();
    let mut row_0: HashMap<usize, Complex64> = HashMap::new();
    row_0.insert(0, 1.0.into());
    let mut row_1: HashMap<usize, Complex64> = HashMap::new();
    row_1.insert(1, (-1.0).into());
    let mut row_2: HashMap<usize, Complex64> = HashMap::new();
    row_2.insert(2, (-1.0).into());
    let mut row_3: HashMap<usize, Complex64> = HashMap::new();
    row_3.insert(3, 1.0.into());
    superoperator_matrix.insert(0, row_0);
    superoperator_matrix.insert(1, row_1);
    superoperator_matrix.insert(2, row_2);
    superoperator_matrix.insert(3, row_3);
}

// Test the negative operation: -SpinSystem
#[test]
fn negative_system() {
    let pp_0: PauliProduct = PauliProduct::from_str("0Z").unwrap();
    let mut system_0 = SpinSystem::new(Some(1));
    system_0
        .add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut system_0_minus = SpinSystem::new(Some(1));
    system_0_minus
        .add_operator_product(pp_0, CalculatorComplex::from(-1.0))
        .unwrap();

    assert_eq!(-system_0, system_0_minus);
}

// Test the addition: SpinSystem + SpinSystem
#[test]
fn add_system_system() {
    let pp_0: PauliProduct = PauliProduct::from_str("0Z").unwrap();
    let pp_1: PauliProduct = PauliProduct::from_str("1X").unwrap();
    let mut system_0 = SpinSystem::new(Some(2));
    system_0
        .add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut system_1 = SpinSystem::new(Some(2));
    system_1
        .add_operator_product(pp_1.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    let mut system_0_1 = SpinSystem::new(Some(2));
    system_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    system_0_1
        .add_operator_product(pp_1, CalculatorComplex::from(0.5))
        .unwrap();

    assert_eq!(system_0.clone() + system_1.clone(), Ok(system_0_1.clone()));
    assert_eq!(system_0.add(system_1), Ok(system_0_1));
}

// Test the addition: SpinSystem + SpinSystem
#[test]
fn add_system_iter() {
    let pp_0: PauliProduct = PauliProduct::from_str("0Z").unwrap();
    let pp_1: PauliProduct = PauliProduct::from_str("1X").unwrap();
    let mut system_0 = SpinSystem::new(Some(2));
    system_0
        .add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();

    let mut mapping: BTreeMap<PauliProduct, CalculatorComplex> = BTreeMap::new();
    mapping.insert(pp_1.clone(), CalculatorComplex::from(0.5));
    let mapping_iter = mapping.into_iter();

    let mut system_0_1 = SpinSystem::new(Some(2));
    system_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    system_0_1
        .add_operator_product(pp_1, CalculatorComplex::from(0.5))
        .unwrap();

    assert_eq!(system_0 + mapping_iter, Ok(system_0_1));
}

// Test the subtraction: SpinSystem - SpinSystem
#[test]
fn sub_system_system() {
    let pp_0: PauliProduct = PauliProduct::from_str("0Z").unwrap();
    let pp_1: PauliProduct = PauliProduct::from_str("1X").unwrap();
    let mut system_0 = SpinSystem::new(Some(2));
    system_0
        .add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut system_1 = SpinSystem::new(Some(2));
    system_1
        .add_operator_product(pp_1.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    let mut system_0_1 = SpinSystem::new(Some(2));
    system_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    system_0_1
        .add_operator_product(pp_1, CalculatorComplex::from(-0.5))
        .unwrap();

    assert_eq!(system_0.clone() - system_1.clone(), Ok(system_0_1.clone()));
    assert_eq!(system_0.sub(system_1), Ok(system_0_1));
}

// Test the subtraction: SpinSystem - SpinSystem
#[test]
fn sub_system_system_iter() {
    let pp_0: PauliProduct = PauliProduct::from_str("0Z").unwrap();
    let pp_1: PauliProduct = PauliProduct::from_str("1X").unwrap();
    let mut system_0 = SpinSystem::new(Some(2));
    system_0
        .add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();

    let mut mapping: BTreeMap<PauliProduct, CalculatorComplex> = BTreeMap::new();
    mapping.insert(pp_1.clone(), CalculatorComplex::from(0.5));
    let mapping_iter = mapping.into_iter();

    let mut system_0_1 = SpinSystem::new(Some(2));
    system_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    system_0_1
        .add_operator_product(pp_1, CalculatorComplex::from(-0.5))
        .unwrap();

    assert_eq!(system_0 - mapping_iter, Ok(system_0_1));
}

// Test the multiplication: SpinSystem * SpinSystem with all systemssible pauli matrices
#[test_case("0X", "0X", "", CalculatorComplex::from(0.0); "x_x_empty")]
#[test_case("0X1X", "0X", "1X", CalculatorComplex::new(1.0, 0.0); "x_x")]
#[test_case("0X1X", "0Y", "0Z1X", CalculatorComplex::new(0.0, 1.0); "x_y")]
#[test_case("0X1X", "0Z", "0Y1X", CalculatorComplex::new(0.0, -1.0); "x_z")]
#[test_case("0Y1X", "0X", "0Z1X", CalculatorComplex::new(0.0, -1.0); "y_x")]
#[test_case("0Y1X", "0Y", "1X", CalculatorComplex::new(1.0, 0.0); "y_y")]
#[test_case("0Y1X", "0Z", "0X1X", CalculatorComplex::new(0.0, 1.0); "y_z")]
#[test_case("0Z1X", "0X", "0Y1X", CalculatorComplex::new(0.0, 1.0); "z_x")]
#[test_case("0Z1X", "0Y", "0X1X", CalculatorComplex::new(0.0, -1.0); "z_y")]
#[test_case("0Z1X", "0Z", "1X", CalculatorComplex::new(1.0, 0.0); "z_z")]
fn mul_system_system_all_paulis(pp0: &str, pp1: &str, pp01: &str, coeff: CalculatorComplex) {
    let pp_0: PauliProduct = PauliProduct::from_str(pp0).unwrap();
    let mut system_0 = SpinSystem::new(Some(2));
    system_0
        .add_operator_product(pp_0, CalculatorComplex::from(2.0))
        .unwrap();
    let pp_1: PauliProduct = PauliProduct::from_str(pp1).unwrap();
    let mut system_1 = SpinSystem::new(Some(1));
    system_1
        .add_operator_product(pp_1, CalculatorComplex::from(0.5))
        .unwrap();
    let mut system_0_1 = SpinSystem::new(Some(2));
    if pp01.is_empty() {
        assert!((system_0 * system_1).keys().next().unwrap().is_empty());
    } else {
        let pp_0_1: PauliProduct = PauliProduct::from_str(pp01).unwrap();
        system_0_1.add_operator_product(pp_0_1, coeff).unwrap();
        assert_eq!(system_0 * system_1, system_0_1);
    }
}

// Test the multiplication: SpinSystem * SpinSystem
#[test]
fn mul_system_system() {
    let pp_0: PauliProduct = PauliProduct::from_str("0Z").unwrap();
    let pp_1: PauliProduct = PauliProduct::from_str("1X").unwrap();
    let pp_0_1: PauliProduct = PauliProduct::from_str("0Z1X").unwrap();
    let mut system_0 = SpinSystem::new(Some(2));
    system_0
        .add_operator_product(pp_0, CalculatorComplex::from(2.0))
        .unwrap();
    let mut system_1 = SpinSystem::new(Some(2));
    system_1
        .add_operator_product(pp_1, CalculatorComplex::from(0.5))
        .unwrap();
    let mut system_0_1 = SpinSystem::new(Some(2));
    system_0_1
        .add_operator_product(pp_0_1, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(system_0 * system_1, system_0_1);
}

// Test the multiplication: SpinSystem * SpinSystem where they have a PauliProduct with the same index
#[test]
fn mul_system_system_same_index() {
    let pp_0: PauliProduct = PauliProduct::from_str("0Z").unwrap();
    let pp_1: PauliProduct = PauliProduct::from_str("0X").unwrap();
    let pp_0_1: PauliProduct = PauliProduct::from_str("0Y").unwrap();
    let mut system_0 = SpinSystem::new(Some(2));
    system_0
        .add_operator_product(pp_0, CalculatorComplex::from(2.0))
        .unwrap();
    let mut system_1 = SpinSystem::new(Some(2));
    system_1
        .add_operator_product(pp_1, CalculatorComplex::from(0.5))
        .unwrap();
    let mut system_0_1 = SpinSystem::new(Some(2));
    system_0_1
        .add_operator_product(pp_0_1, CalculatorComplex::new(0.0, 1.0))
        .unwrap();

    assert_eq!(system_0 * system_1, system_0_1);
}

// Test the multiplication: SpinSystem * Calculatorcomplex
#[test]
fn mul_system_cc() {
    let pp_0: PauliProduct = PauliProduct::from_str("0Z").unwrap();
    let mut system_0 = SpinSystem::new(Some(1));
    system_0
        .add_operator_product(pp_0.clone(), CalculatorComplex::from(2.0))
        .unwrap();
    let mut system_0_1 = SpinSystem::new(Some(1));
    system_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(6.0))
        .unwrap();

    assert_eq!(system_0 * CalculatorComplex::from(3.0), system_0_1);
}

// Test the multiplication: SpinSystem * CalculatorFloat
#[test]
fn mul_system_cf() {
    let pp_0: PauliProduct = PauliProduct::from_str("0Z").unwrap();
    let mut system_0 = SpinSystem::new(Some(1));
    system_0
        .add_operator_product(pp_0.clone(), CalculatorComplex::from(2.0))
        .unwrap();
    let mut system_0_1 = SpinSystem::new(Some(1));
    system_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(6.0))
        .unwrap();

    assert_eq!(system_0 * CalculatorFloat::from(3.0), system_0_1);
}

// Test the Clone and PartialEq traits of SpinSystem
#[test]
fn clone_partial_eq_system() {
    let mut system = SpinSystem::new(Some(2));
    let pp: PauliProduct = PauliProduct::new().z(0);

    let mut system_0 = SpinSystem::new(Some(2));
    let pp_0: PauliProduct = PauliProduct::new().z(0);

    let mut system_1 = SpinSystem::new(Some(3));
    let pp_1: PauliProduct = PauliProduct::new().x(1);

    // Test PartialEq trait
    assert!(system_0 == system);
    assert!(system == system_0);
    assert!(system_1 != system);
    assert!(system != system_1);

    system
        .add_operator_product(pp, CalculatorComplex::from(1.0))
        .unwrap();
    system_0
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    system_1
        .add_operator_product(pp_1, CalculatorComplex::from(2.0))
        .unwrap();

    assert!(system_0 == system);
    assert!(system == system_0);
    assert!(system_1 != system);
    assert!(system != system_1);

    // Test Clone trait
    assert_eq!(system.clone(), system);
}

#[test]
fn serde_json() {
    let pp: PauliProduct = PauliProduct::new().z(0);
    let mut ss = SpinSystem::new(Some(1));
    ss.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let serialized = serde_json::to_string(&ss).unwrap();
    let deserialized: SpinSystem = serde_json::from_str(&serialized).unwrap();
    assert_eq!(ss, deserialized);
}

/// Test SpinSystem Serialization and Deserialization traits (readable)
#[test]
fn serde_readable() {
    use struqture::MINIMUM_STRUQTURE_VERSION;
    let major_version = MINIMUM_STRUQTURE_VERSION.0;
    let minor_version = MINIMUM_STRUQTURE_VERSION.1;

    let pp = PauliProduct::new().x(0);
    let mut system = SpinSystem::new(Some(2));
    system.set(pp, 0.5.into()).unwrap();

    assert_tokens(
        &system.readable(),
        &[
            Token::Struct {
                name: "SpinSystem",
                len: 2,
            },
            Token::Str("number_spins"),
            Token::Some,
            Token::U64(2),
            Token::Str("operator"),
            Token::Struct {
                name: "SpinOperatorSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 3 },
            Token::Str("0X"),
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
        ],
    );
}

#[test]
fn bincode() {
    let pp: PauliProduct = PauliProduct::new().z(0);
    let mut ss = SpinSystem::new(Some(1));
    ss.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let encoded: Vec<u8> = bincode::serialize(&ss).unwrap();
    let decoded: SpinSystem = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(ss, decoded);

    let encoded: Vec<u8> = bincode::serialize(&ss.clone().compact()).unwrap();
    let decoded: SpinSystem = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(ss, decoded);
}

/// Test SpinSystem Serialization and Deserialization traits (compact)
#[test]
fn serde_compact() {
    use struqture::MINIMUM_STRUQTURE_VERSION;
    let major_version = MINIMUM_STRUQTURE_VERSION.0;
    let minor_version = MINIMUM_STRUQTURE_VERSION.1;

    let pp = PauliProduct::new().x(0);
    let mut system = SpinSystem::new(Some(2));
    system.set(pp, 0.5.into()).unwrap();

    assert_tokens(
        &system.compact(),
        &[
            Token::Struct {
                name: "SpinSystem",
                len: 2,
            },
            Token::Str("number_spins"),
            Token::Some,
            Token::U64(2),
            Token::Str("operator"),
            Token::Struct {
                name: "SpinOperatorSerialize",
                len: 2,
            },
            Token::Str("items"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 3 },
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::U64(0),
            Token::UnitVariant {
                name: "SingleSpinOperator",
                variant: "X",
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

#[test_case("0Z", &["Z"]; "0Z")]
#[test_case("0Z", &["I","Z"]; "0Z1I")]
#[test_case("0X", &["I","X"]; "0X1I")]
#[test_case("0I", &["I"]; "0I")]
#[test_case("1X", &["X", "I"]; "1X")]
#[test_case("1Y", &["Y", "I"]; "1Y")]
#[test_case("0Z1X", &["X", "Z"]; "0Z1X")]
#[test_case("0X1X", &["X", "X"]; "0X1X")]
#[test_case("0X1Y", &["Y", "X"]; "0X1Y")]
#[test_case("0X2Y", &["Y", "I","X"]; "0X2Y")]
fn test_superoperator(pauli_representation: &str, pauli_operators: &[&str]) {
    let mut system = SpinSystem::new(Some(pauli_operators.len()));
    let pp: PauliProduct = PauliProduct::from_str(pauli_representation).unwrap();

    system.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let dimension = 4_usize.pow(pauli_operators.len() as u32);

    // Constructing matrix by hand:
    let cc0 = Complex64::new(0.0, 0.0);
    let cci = Complex64::new(0.0, 1.0);

    let identities: Vec<&str> = (0..pauli_operators.len()).map(|_| "I").collect();

    let i = create_na_matrix_from_operator_list(&identities);
    let h = create_na_matrix_from_operator_list(pauli_operators);

    let test_matrix = (h.kronecker(&i) - i.kronecker(&h.transpose())) * (-cci);

    let second_test_matrix = system.sparse_matrix_superoperator(None).unwrap();
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
    let (test_vals, (test_rows, test_columns)) =
        system.sparse_matrix_superoperator_coo(None).unwrap();
    for (second_val, (row, column)) in test_vals
        .iter()
        .zip(test_rows.iter().zip(test_columns.iter()))
    {
        let val = test_matrix[(*row, *column)];
        assert_eq!(&val, second_val);
    }
}

#[test_case("0Z", &["Z"]; "0Z")]
#[test_case("0Z", &["I","Z"]; "0Z1I")]
#[test_case("0X", &["I","X"]; "0X1I")]
#[test_case("0I", &["I"]; "0I")]
#[test_case("1X", &["X", "I"]; "1X")]
#[test_case("1Y", &["Y", "I"]; "1Y")]
#[test_case("0Z1X", &["X", "Z"]; "0Z1X")]
#[test_case("0X1X", &["X", "X"]; "0X1X")]
#[test_case("0X1Y", &["Y", "X"]; "0X1Y")]
#[test_case("0X2Y", &["Y", "I","X"]; "0X2Y")]
fn test_operator(pauli_representation: &str, pauli_operators: &[&str]) {
    let mut system = SpinSystem::new(Some(pauli_operators.len()));
    let pp: PauliProduct = PauliProduct::from_str(pauli_representation).unwrap();

    system.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let dimension = 2_usize.pow(pauli_operators.len() as u32);

    // Constructing matrix by hand:
    let cc0 = Complex64::new(0.0, 0.0);

    let h = create_na_matrix_from_operator_list(pauli_operators);

    let test_matrix = h;

    let second_test_matrix = system.sparse_matrix(None).unwrap();
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
fn sparse_lindblad_entries() {
    let pp_0: PauliProduct = PauliProduct::new().z(0);
    let pp_1: PauliProduct = PauliProduct::new().x(1);
    let mut system = SpinSystem::new(Some(2));
    system
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    system
        .add_operator_product(pp_1, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(
        system.sparse_lindblad_entries().unwrap()[0].0,
        (vec![], (vec![], vec![]))
    );
    assert_eq!(
        system.sparse_lindblad_entries().unwrap()[0].1,
        (vec![], (vec![], vec![]))
    );
    assert_eq!(
        system.sparse_lindblad_entries().unwrap()[0].2,
        Complex64::default()
    );
}

#[test_case(1.0,0.0;"real")]
#[test_case(0.0,1.0;"imag")]
#[test_case(0.7,0.7;"mixed")]
fn test_truncate(re: f64, im: f64) {
    let mut system = SpinSystem::new(None);
    system
        .set(
            PauliProduct::from_str("0X").unwrap(),
            CalculatorComplex::new(100.0 * re, 100.0 * im),
        )
        .unwrap();
    system
        .set(
            PauliProduct::from_str("1Y").unwrap(),
            CalculatorComplex::new(10.0 * re, 10.0 * im),
        )
        .unwrap();
    system
        .set(
            PauliProduct::from_str("2Z").unwrap(),
            CalculatorComplex::new(re, im),
        )
        .unwrap();
    system
        .set(PauliProduct::from_str("0X1Z").unwrap(), "test".into())
        .unwrap();

    let mut test_system1 = SpinSystem::new(None);
    test_system1
        .set(
            PauliProduct::from_str("0X").unwrap(),
            CalculatorComplex::new(100.0 * re, 100.0 * im),
        )
        .unwrap();
    test_system1
        .set(
            PauliProduct::from_str("1Y").unwrap(),
            CalculatorComplex::new(10.0 * re, 10.0 * im),
        )
        .unwrap();
    test_system1
        .set(PauliProduct::from_str("0X1Z").unwrap(), "test".into())
        .unwrap();

    let mut test_system2 = SpinSystem::new(None);
    test_system2
        .set(
            PauliProduct::from_str("0X").unwrap(),
            CalculatorComplex::new(100.0 * re, 100.0 * im),
        )
        .unwrap();
    test_system2
        .set(PauliProduct::from_str("0X1Z").unwrap(), "test".into())
        .unwrap();

    let comparison_system1 = system.truncate(5.0);
    assert_eq!(test_system1, comparison_system1);
    let comparison_system2 = system.truncate(50.0);
    assert_eq!(test_system2, comparison_system2);
}
