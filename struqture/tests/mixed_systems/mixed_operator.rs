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

//! Integration test for public API of MixedOperator

// use num_complex::Complex64;
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde_test::{assert_tokens, Configure, Token};
use std::collections::BTreeMap;
use std::collections::HashMap;
use struqture::bosons::BosonProduct;
use struqture::fermions::FermionProduct;
use struqture::prelude::*;
use struqture::spins::PauliProduct;
use struqture::StruqtureError;
use struqture::STRUQTURE_VERSION;

#[cfg(feature = "struqture_1_import")]
#[cfg(feature = "struqture_1_export")]
use std::str::FromStr;
use struqture::mixed_systems::{MixedOperator, MixedProduct};
use struqture::OperateOnDensityMatrix;
use struqture::SpinIndex;
use test_case::test_case;

// Test the new function of the MixedOperator
#[test_case(0_usize, 0_usize, 0_usize, vec![], vec![], vec![]; "0, 0, 0")]
#[test_case(1_usize, 2_usize, 1_usize, vec![0], vec![0, 0], vec![0]; "1, 2, 1")]
#[test_case(2_usize, 1_usize, 2_usize, vec![0, 0], vec![0], vec![0, 0]; "2, 1, 2")]
#[test_case(10_usize, 10_usize, 10_usize, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; "10, 10, 10")]
fn new(
    n_pauli: usize,
    n_bosons: usize,
    n_fermions: usize,
    current_number_spins: Vec<usize>,
    number_bosonic_modes: Vec<usize>,
    number_fermionic_modes: Vec<usize>,
) {
    let mo = MixedOperator::new(n_pauli, n_bosons, n_fermions);
    assert!(mo.is_empty());
    assert_eq!(current_number_spins, mo.current_number_spins());
    assert_eq!(number_bosonic_modes, mo.current_number_bosonic_modes());
    assert_eq!(number_fermionic_modes, mo.current_number_fermionic_modes());
}

// Test the new function of the MixedOperator
#[test_case(0_usize, 0_usize, 0_usize, vec![], vec![], vec![]; "0, 0, 0")]
#[test_case(1_usize, 2_usize, 1_usize, vec![0], vec![0, 0], vec![0]; "1, 2, 1")]
#[test_case(2_usize, 1_usize, 2_usize, vec![0, 0], vec![0], vec![0, 0]; "2, 1, 2")]
#[test_case(10_usize, 10_usize, 10_usize, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; "10, 10, 10")]
fn new_with_capacity(
    n_pauli: usize,
    n_bosons: usize,
    n_fermions: usize,
    current_number_spins: Vec<usize>,
    number_bosonic_modes: Vec<usize>,
    number_fermionic_modes: Vec<usize>,
) {
    let mo = MixedOperator::new(n_pauli, n_bosons, n_fermions);
    assert!(mo.is_empty());
    assert_eq!(current_number_spins, mo.current_number_spins());
    assert_eq!(number_bosonic_modes, mo.current_number_bosonic_modes());
    assert_eq!(number_fermionic_modes, mo.current_number_fermionic_modes());
}

#[test]
fn empty_clone_options() {
    let pp_0: MixedProduct = MixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedOperator::new(1, 1, 1);
    mo.set(pp_0, CalculatorComplex::from(0.5)).unwrap();

    let empty: Option<usize> = None;
    let full: Option<usize> = Some(2);
    assert_eq!(mo.empty_clone(empty), MixedOperator::new(1, 1, 1));
    assert_eq!(
        mo.empty_clone(full),
        MixedOperator::with_capacity(1, 1, 1, 2)
    );
}

// Test the len function of the PauliOperator
#[test]
fn internal_map_len() {
    let pp_2: MixedProduct = MixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedOperator::new(1, 1, 1);
    mo.set(pp_2, CalculatorComplex::from(0.5)).unwrap();
    assert_eq!(mo.len(), 1_usize);
}

// Test the iter, keys and values functions of the MixedOperator
#[test]
fn internal_map_keys() {
    let pp_0: MixedProduct = MixedProduct::new(
        [PauliProduct::new().z(1)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedOperator::new(1, 1, 1);
    let pp_0_vec: Vec<(MixedProduct, CalculatorComplex)> = vec![(pp_0.clone(), 0.3.into())];
    mo.extend(pp_0_vec.iter().cloned());

    let mut map: BTreeMap<MixedProduct, CalculatorComplex> = BTreeMap::new();
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

// Test the set, get and remove functions of the PauliOperator
#[test]
fn internal_map_set_get_remove() {
    let pp_2: MixedProduct = MixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedOperator::new(1, 1, 1);

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
    assert_eq!(mo, MixedOperator::new(1, 1, 1));
}

#[test]
fn set_fail() {
    let pp_0: MixedProduct = MixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let pp_2: MixedProduct = MixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedOperator::new(0, 1, 1);
    assert_eq!(mo.current_number_spins(), Vec::<usize>::new());
    assert_eq!(mo.current_number_bosonic_modes(), vec![0_usize]);
    assert_eq!(mo.current_number_fermionic_modes(), vec![0_usize]);

    let err = mo.set(pp_0, CalculatorComplex::from(0.5));
    assert_eq!(
        err,
        Err(StruqtureError::MismatchedNumberSubsystems {
            target_number_spin_subsystems: 0,
            target_number_boson_subsystems: 1,
            target_number_fermion_subsystems: 1,
            actual_number_spin_subsystems: 1,
            actual_number_boson_subsystems: 1,
            actual_number_fermion_subsystems: 1,
        })
    );

    let mut mo = MixedOperator::new(1, 0, 1);
    assert_eq!(mo.current_number_spins(), vec![0_usize]);
    assert_eq!(mo.current_number_bosonic_modes(), Vec::<usize>::new());
    assert_eq!(mo.current_number_fermionic_modes(), vec![0_usize]);

    let err = mo.set(pp_2.clone(), CalculatorComplex::from(0.5));
    assert_eq!(
        err,
        Err(StruqtureError::MismatchedNumberSubsystems {
            target_number_spin_subsystems: 1,
            target_number_boson_subsystems: 0,
            target_number_fermion_subsystems: 1,
            actual_number_spin_subsystems: 1,
            actual_number_boson_subsystems: 1,
            actual_number_fermion_subsystems: 1,
        })
    );

    let mut mo = MixedOperator::new(1, 1, 0);
    assert_eq!(mo.current_number_spins(), vec![0_usize]);
    assert_eq!(mo.current_number_bosonic_modes(), vec![0_usize]);
    assert_eq!(mo.current_number_fermionic_modes(), Vec::<usize>::new());

    let err = mo.set(pp_2, CalculatorComplex::from(0.5));
    assert_eq!(
        err,
        Err(StruqtureError::MismatchedNumberSubsystems {
            target_number_spin_subsystems: 1,
            target_number_boson_subsystems: 1,
            target_number_fermion_subsystems: 0,
            actual_number_spin_subsystems: 1,
            actual_number_boson_subsystems: 1,
            actual_number_fermion_subsystems: 1,
        })
    );
}

// Test the add_operator_product function of the MixedOperator
#[test]
fn internal_map_add_operator_product() {
    let pp_2: MixedProduct = MixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedOperator::new(1, 1, 1);

    mo.add_operator_product(pp_2.clone(), CalculatorComplex::from(0.5))
        .unwrap();
    assert_eq!(mo.get(&pp_2), &CalculatorComplex::from(0.5));
    mo.add_operator_product(pp_2.clone(), CalculatorComplex::from(-0.5))
        .unwrap();
    assert_eq!(mo.get(&pp_2), &CalculatorComplex::from(0.0));
}

#[test]
fn fail_add_operator_product() {
    let pp_2: MixedProduct = MixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();

    let mut mo = MixedOperator::new(0, 1, 1);
    let err = mo.add_operator_product(pp_2.clone(), CalculatorComplex::from(0.5));
    assert_eq!(
        err,
        Err(StruqtureError::MismatchedNumberSubsystems {
            target_number_spin_subsystems: 0,
            target_number_boson_subsystems: 1,
            target_number_fermion_subsystems: 1,
            actual_number_spin_subsystems: 1,
            actual_number_boson_subsystems: 1,
            actual_number_fermion_subsystems: 1,
        })
    );

    let mut mo = MixedOperator::new(1, 0, 1);
    let err = mo.add_operator_product(pp_2.clone(), CalculatorComplex::from(0.5));
    assert_eq!(
        err,
        Err(StruqtureError::MismatchedNumberSubsystems {
            target_number_spin_subsystems: 1,
            target_number_boson_subsystems: 0,
            target_number_fermion_subsystems: 1,
            actual_number_spin_subsystems: 1,
            actual_number_boson_subsystems: 1,
            actual_number_fermion_subsystems: 1,
        })
    );

    let mut mo = MixedOperator::new(1, 1, 0);
    let err = mo.add_operator_product(pp_2, CalculatorComplex::from(0.5));
    assert_eq!(
        err,
        Err(StruqtureError::MismatchedNumberSubsystems {
            target_number_spin_subsystems: 1,
            target_number_boson_subsystems: 1,
            target_number_fermion_subsystems: 0,
            actual_number_spin_subsystems: 1,
            actual_number_boson_subsystems: 1,
            actual_number_fermion_subsystems: 1,
        })
    );
}

// Test the hermitian_conjugate and is_natural_hermitian functions of the MixedProduct
#[test]
fn hermitian_test() {
    let pp_0: MixedProduct = MixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut test_new = MixedOperator::new(1, 1, 1);
    test_new.set(pp_0, CalculatorComplex::from(0.5)).unwrap();

    assert_eq!(test_new.hermitian_conjugate(), test_new.clone());
}

// Test the negative operation: -MixedOperator
#[test]
fn negative_mo() {
    let pp_0: MixedProduct = MixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo_0 = MixedOperator::new(1, 1, 1);
    mo_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut mo_0_minus = MixedOperator::new(1, 1, 1);
    mo_0_minus
        .add_operator_product(pp_0, CalculatorComplex::from(-1.0))
        .unwrap();

    assert_eq!(-mo_0, mo_0_minus);
}

// Test the addition: PauliOperator + PauliOperator
#[test]
fn add_so_so() {
    let pp_0: MixedProduct = MixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let pp_1: MixedProduct = MixedProduct::new(
        [PauliProduct::new().x(1)],
        [BosonProduct::new([1], [2]).unwrap()],
        [FermionProduct::new([1], [3]).unwrap()],
    )
    .unwrap();
    let mut mo_0 = MixedOperator::new(1, 1, 1);
    mo_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut mo_1 = MixedOperator::new(1, 1, 1);
    mo_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(-1.0))
        .unwrap();
    let mut mo_0_1 = MixedOperator::new(1, 1, 1);
    mo_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    mo_0_1
        .add_operator_product(pp_1, CalculatorComplex::from(-1.0))
        .unwrap();

    assert_eq!(mo_0 + mo_1, Ok(mo_0_1));
}

// Test the addition: PauliOperator + PauliOperator
#[test]
fn sub_so_so() {
    let pp_0: MixedProduct = MixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let pp_1: MixedProduct = MixedProduct::new(
        [PauliProduct::new().x(1)],
        [BosonProduct::new([1], [2]).unwrap()],
        [FermionProduct::new([1], [3]).unwrap()],
    )
    .unwrap();
    let mut mo_0 = MixedOperator::new(1, 1, 1);
    mo_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut mo_1 = MixedOperator::new(1, 1, 1);
    mo_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(-1.0))
        .unwrap();
    let mut mo_0_1 = MixedOperator::new(1, 1, 1);
    mo_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    mo_0_1
        .add_operator_product(pp_1, CalculatorComplex::from(1.0))
        .unwrap();

    assert_eq!(mo_0 - mo_1, Ok(mo_0_1));
}

// Test the multiplication: PauliOperator * Calculatorcomplex
#[test]
fn mul_so_cf() {
    let pp_0: MixedProduct = MixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo_0 = MixedOperator::new(1, 1, 1);
    mo_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(2.0))
        .unwrap();
    let mut mo_0_1 = MixedOperator::new(1, 1, 1);
    mo_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(6.0))
        .unwrap();

    assert_eq!(mo_0 * CalculatorFloat::from(3.0), mo_0_1);
}

// Test the multiplication: PauliOperator * Calculatorcomplex
#[test]
fn mul_so_cc() {
    let pp_0: MixedProduct = MixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo_0 = MixedOperator::new(1, 1, 1);
    mo_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(2.0))
        .unwrap();
    let mut mo_0_1 = MixedOperator::new(1, 1, 1);
    mo_0_1
        .add_operator_product(pp_0, CalculatorComplex::from(6.0))
        .unwrap();

    assert_eq!(mo_0 * CalculatorComplex::from(3.0), mo_0_1);
}

// Test the multiplication: FermionOperator * FermionOperator
#[test]
fn mul_so_so() {
    let pp_0: MixedProduct = MixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let pp_1: MixedProduct = MixedProduct::new(
        [PauliProduct::new().x(1)],
        [BosonProduct::new([1], [2]).unwrap()],
        [FermionProduct::new([1], [3]).unwrap()],
    )
    .unwrap();
    let mut mo_0 = MixedOperator::new(1, 1, 1);
    mo_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut mo_1 = MixedOperator::new(1, 1, 1);
    mo_1.add_operator_product(pp_1.clone(), CalculatorComplex::from(2.0))
        .unwrap();

    let pp_0_1 = (pp_0 * pp_1).unwrap();
    let mut mo_0_1 = MixedOperator::new(1, 1, 1);
    for pp in pp_0_1 {
        mo_0_1
            .add_operator_product(pp.0, CalculatorComplex::from(2.0) * pp.1)
            .unwrap();
    }

    assert_eq!(mo_0 * mo_1, Ok(mo_0_1));
}

// Test the Iter traits of FermionOperator: into_iter, from_iter and extend
#[test]
fn into_iter_from_iter_extend() {
    let pp_0: MixedProduct = MixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let pp_1: MixedProduct = MixedProduct::new(
        [PauliProduct::new().x(1)],
        [BosonProduct::new([1], [2]).unwrap()],
        [FermionProduct::new([1], [3]).unwrap()],
    )
    .unwrap();
    let mut mo_0 = MixedOperator::new(1, 1, 1);
    mo_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(2.0))
        .unwrap();

    let mo_iter = mo_0.clone().into_iter();
    assert_eq!(MixedOperator::from_iter(mo_iter), mo_0);
    let mo_iter = (&mo_0)
        .into_iter()
        .map(|(key, value)| (key.clone(), value.clone()));
    assert_eq!(MixedOperator::from_iter(mo_iter), mo_0);
    let mut mapping: BTreeMap<MixedProduct, CalculatorComplex> = BTreeMap::new();
    mapping.insert(pp_1.clone(), CalculatorComplex::from(0.5));
    let mapping_iter = mapping.into_iter();
    mo_0.extend(mapping_iter);

    let mut mo_0_1 = MixedOperator::new(1, 1, 1);
    let _ = mo_0_1.add_operator_product(pp_0, CalculatorComplex::from(2.0));
    let _ = mo_0_1.add_operator_product(pp_1, CalculatorComplex::from(0.5));

    assert_eq!(mo_0, mo_0_1);
}

// Test the from_iter function of the MixedOperator
#[test]
fn from_iterator() {
    let pp_0: MixedProduct = MixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let pp_1: MixedProduct = MixedProduct::new(
        [PauliProduct::new().x(1)],
        [BosonProduct::new([1], [2]).unwrap()],
        [FermionProduct::new([1], [3]).unwrap()],
    )
    .unwrap();

    // iterator with two items
    let mut mo_0 = MixedOperator::new(1, 1, 1);
    mo_0.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    mo_0.add_operator_product(pp_1.clone(), CalculatorComplex::from(2.0))
        .unwrap();
    let mut iterator: HashMap<MixedProduct, CalculatorComplex> = HashMap::new();
    iterator.insert(pp_0, 1.0.into());
    iterator.insert(pp_1, 2.0.into());
    assert_eq!(
        MixedOperator::from_iter(iterator.iter().map(|(x, y)| (x.clone(), y.clone()))),
        mo_0
    );

    // iterator with no items
    let mo_0 = MixedOperator::new(0, 0, 0);
    let iterator: HashMap<MixedProduct, CalculatorComplex> = HashMap::new();
    assert_eq!(
        MixedOperator::from_iter(iterator.iter().map(|(x, y)| (x.clone(), y.clone()))),
        mo_0
    );
}

#[test]
fn default() {
    assert_eq!(MixedOperator::default(), MixedOperator::new(0, 0, 0));
}

// Test the Hash, Debug and Display traits of PauliProduct
#[test]
fn debug() {
    let pp_0: MixedProduct = MixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedOperator::new(1, 1, 1);
    mo.add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    assert_eq!(
        format!("{mo:?}"),
        "MixedOperator { internal_map: {MixedProduct { spins: [PauliProduct { items: [(2, Z)] }], bosons: [BosonProduct { creators: [0], annihilators: [3] }], fermions: [FermionProduct { creators: [0], annihilators: [3] }] }: CalculatorComplex { re: Float(1.0), im: Float(0.0) }}, n_spins: 1, n_bosons: 1, n_fermions: 1 }"
    );
}

// Test the Hash, Debug and Display traits of PauliProduct
#[test]
fn display() {
    let pp_0: MixedProduct = MixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedOperator::new(1, 1, 1);
    mo.add_operator_product(pp_0, CalculatorComplex::from(1.0))
        .unwrap();
    assert_eq!(
        format!("{mo}"),
        format!(
            "MixedOperator{{\nS2Z:Bc0a3:Fc0a3:: {},\n}}",
            CalculatorComplex::from(1.0)
        )
    );
}

// Test the Clone, PartialEq, PartialOrd and Ord traits of PauliProduct
#[test]
fn clone_partial_eq_partial_ord() {
    let pp_0: MixedProduct = MixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedOperator::new(1, 1, 1);
    mo.add_operator_product(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();

    // Test Clone trait
    assert_eq!(mo.clone(), mo);

    // Test PartialEq trait
    let mut mo_0 = MixedOperator::new(1, 1, 1);
    mo_0.set(pp_0.clone(), CalculatorComplex::from(1.0))
        .unwrap();
    let mut mo_1 = MixedOperator::new(1, 1, 1);
    mo_1.set(pp_0, CalculatorComplex::from(2.0)).unwrap();
    assert!(mo_0 == mo);
    assert!(mo == mo_0);
    assert!(mo_1 != mo);
    assert!(mo != mo_1);
}

#[test]
fn serde_json() {
    let pp: MixedProduct = MixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedOperator::new(1, 1, 1);
    mo.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let serialized = serde_json::to_string(&mo).unwrap();
    let deserialized: MixedOperator = serde_json::from_str(&serialized).unwrap();

    assert_eq!(mo, deserialized);
}

/// Test PauliOperator Serialization and Deserialization traits (readable)
#[test]
fn serde_readable() {
    let pp: MixedProduct = MixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedOperator::new(1, 1, 1);
    mo.set(pp, CalculatorComplex::from(0.5)).unwrap();
    assert_tokens(
        &mo.readable(),
        &[
            Token::Struct {
                name: "MixedOperatorSerialize",
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
            Token::Str("serialisation_meta"),
            Token::Struct {
                name: "StruqtureSerialisationMeta",
                len: 3,
            },
            Token::Str("type_name"),
            Token::Str("MixedOperator"),
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
    let pp: MixedProduct = MixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedOperator::new(1, 1, 1);
    mo.set(pp, CalculatorComplex::from(1.0)).unwrap();

    let config = bincode::config::legacy();
    let serialized = bincode::serde::encode_to_vec(&mo, config).unwrap();
    let (deserialized, _len): (MixedOperator, usize) =
        bincode::serde::decode_from_slice(&serialized, config).unwrap();
    assert_eq!(deserialized, mo);

    let encoded: Vec<u8> = bincode::serde::encode_to_vec(&mo.clone().compact(), config).unwrap();
    let (decoded, _len): (MixedOperator, usize) =
        bincode::serde::decode_from_slice(&encoded, config).unwrap();
    assert_eq!(mo, decoded);
}

#[test]
fn serde_compact() {
    let pp: MixedProduct = MixedProduct::new(
        [PauliProduct::new().z(2)],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [2]).unwrap()],
    )
    .unwrap();
    let mut mo = MixedOperator::new(1, 1, 1);
    mo.set(pp, CalculatorComplex::from(0.5)).unwrap();
    assert_tokens(
        &mo.compact(),
        &[
            Token::Struct {
                name: "MixedOperatorSerialize",
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
                name: "SinglePauliOperator",
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
            Token::Str("serialisation_meta"),
            Token::Struct {
                name: "StruqtureSerialisationMeta",
                len: 3,
            },
            Token::Str("type_name"),
            Token::Str("MixedOperator"),
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

#[cfg(feature = "json_schema")]
#[test]
fn test_mixed_operator_schema() {
    let mut op = MixedOperator::new(2, 1, 1);
    let pp = MixedProduct::new(
        [PauliProduct::new().x(0), PauliProduct::new()],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    op.set(pp, 1.0.into()).unwrap();
    let pp = MixedProduct::new(
        [PauliProduct::new().x(1), PauliProduct::new()],
        [BosonProduct::new([0], [3]).unwrap()],
        [FermionProduct::new([0], [3]).unwrap()],
    )
    .unwrap();
    op.set(pp, "val".into()).unwrap();
    let schema = schemars::schema_for!(MixedOperator);
    let schema_checker = jsonschema::validator_for(&serde_json::to_value(&schema).unwrap())
        .expect("schema is valid");
    let value = serde_json::to_value(&op).unwrap();
    let validation = schema_checker.validate(&value);

    assert!(validation.is_ok());
}

#[cfg(feature = "struqture_1_import")]
#[cfg(feature = "struqture_1_export")]
#[test]
fn test_from_to_struqture_1() {
    let pp_1: struqture_1::mixed_systems::MixedProduct =
        struqture_1::mixed_systems::MixedIndex::new(
            [struqture_1::spins::PauliProduct::from_str("0X").unwrap()],
            [struqture_1::bosons::BosonProduct::from_str("c0a1").unwrap()],
            [
                struqture_1::fermions::FermionProduct::from_str("c0a0").unwrap(),
                struqture_1::fermions::FermionProduct::from_str("c0a1").unwrap(),
            ],
        )
        .unwrap();
    let mut ss_1 = struqture_1::mixed_systems::MixedSystem::new([None], [None], [None, None]);
    struqture_1::OperateOnDensityMatrix::set(&mut ss_1, pp_1.clone(), 1.0.into()).unwrap();

    let pp_2 = MixedProduct::new(
        [PauliProduct::new().x(0)],
        [BosonProduct::new([0], [1]).unwrap()],
        [
            FermionProduct::new([0], [0]).unwrap(),
            FermionProduct::new([0], [1]).unwrap(),
        ],
    )
    .unwrap();
    let mut ss_2 = MixedOperator::new(1, 1, 2);
    ss_2.set(pp_2.clone(), 1.0.into()).unwrap();

    assert!(MixedOperator::from_struqture_1(&ss_1).unwrap() == ss_2);
    assert!(ss_1 == ss_2.to_struqture_1().unwrap());
}
