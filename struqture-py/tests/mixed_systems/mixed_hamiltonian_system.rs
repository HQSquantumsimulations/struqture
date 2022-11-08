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

use num_complex::Complex64;
use pyo3::prelude::*;
use qoqo_calculator::CalculatorComplex;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::bosons::BosonProduct;
use struqture::fermions::FermionProduct;
use struqture::mixed_systems::{HermitianMixedProduct, MixedHamiltonianSystem};
use struqture::prelude::MixedIndex;
use struqture::spins::PauliProduct;
use struqture::{ModeIndex, OperateOnDensityMatrix, SpinIndex};
use struqture_py::mixed_systems::{MixedHamiltonianSystemWrapper, MixedSystemWrapper};
use test_case::test_case;

// helper functions
fn new_system(
    py: Python,
    number_spins: Vec<Option<usize>>,
    number_bosons: Vec<Option<usize>>,
    number_fermions: Vec<Option<usize>>,
) -> &PyCell<MixedHamiltonianSystemWrapper> {
    let system_type = py.get_type::<MixedHamiltonianSystemWrapper>();
    system_type
        .call1((number_spins, number_bosons, number_fermions))
        .unwrap()
        .cast_as::<PyCell<MixedHamiltonianSystemWrapper>>()
        .unwrap()
}

/// Test default function of MixedHamiltonianSystemWrapper
#[test]
fn test_default_partialeq_debug_clone() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let new_system = new_system(py, number_spins, number_bosons, number_fermions);
        new_system
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();
        let system_wrapper = new_system
            .extract::<MixedHamiltonianSystemWrapper>()
            .unwrap();

        // PartialEq
        let helper_ne: bool =
            MixedHamiltonianSystemWrapper::new(vec![None], vec![None], vec![None])
                != system_wrapper;
        assert!(helper_ne);
        let helper_eq: bool =
            MixedHamiltonianSystemWrapper::new(vec![None], vec![None], vec![None])
                == MixedHamiltonianSystemWrapper::new(vec![None], vec![None], vec![None]);
        assert!(helper_eq);

        // Clone
        assert_eq!(system_wrapper.clone(), system_wrapper);

        // Debug
        assert_eq!(
            format!("{:?}", MixedHamiltonianSystemWrapper::new(vec![None], vec![None], vec![None])),
            "MixedHamiltonianSystemWrapper { internal: MixedHamiltonianSystem { number_spins: [None], number_bosons: [None], number_fermions: [None], hamiltonian: MixedHamiltonian { internal_map: {}, n_spins: 1, n_bosons: 1, n_fermions: 1 } } }"
        );
    })
}

/// Test number_bosons and current_number_bosons functions of MixedHamiltonianSystem
#[test]
fn test_number_bosons_current() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system = new_system(py, number_spins, number_bosons, number_fermions);
        system
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();

        let number_system = system.call_method0("number_spins").unwrap();
        let current_system = system.call_method0("current_number_spins").unwrap();
        let comparison = bool::extract(
            number_system
                .call_method1("__eq__", (vec![1_u64],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        let comparison = bool::extract(
            current_system
                .call_method1("__eq__", (vec![1_u64],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let number_system = system.call_method0("number_bosonic_modes").unwrap();
        let current_system = system.call_method0("current_number_bosonic_modes").unwrap();
        let comparison = bool::extract(
            number_system
                .call_method1("__eq__", (vec![2_u64],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        let comparison = bool::extract(
            current_system
                .call_method1("__eq__", (vec![2_u64],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let number_system = system.call_method0("number_fermionic_modes").unwrap();
        let current_system = system
            .call_method0("current_number_fermionic_modes")
            .unwrap();
        let comparison = bool::extract(
            number_system
                .call_method1("__eq__", (vec![1_u64],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        let comparison = bool::extract(
            current_system
                .call_method1("__eq__", (vec![1_u64],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    });
}

/// Test empty_clone function of MixedHamiltonianSystem
#[test]
fn test_empty_clone() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system = new_system(py, number_spins, number_bosons, number_fermions);
        let none_system = system.call_method1("empty_clone", (4,)).unwrap();
        let comparison =
            bool::extract(none_system.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison);

        let number_spins: Vec<Option<usize>> = vec![Some(3)];
        let number_bosons: Vec<Option<usize>> = vec![Some(3)];
        let number_fermions: Vec<Option<usize>> = vec![Some(3)];
        let system = new_system(py, number_spins, number_bosons, number_fermions);
        let some_system = system.call_method1("empty_clone", (2,)).unwrap();
        let comparison =
            bool::extract(some_system.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test hermitian_conjugate function of MixedHamiltonianSystem
#[test]
fn test_hermitian_conj() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system = new_system(py, number_spins, number_bosons, number_fermions);
        system
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();

        let conjugate = system.call_method0("hermitian_conjugate").unwrap();
        let comparison =
            bool::extract(conjugate.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test set and get functions of MixedHamiltonianSystem
#[test]
fn boson_system_test_set_get() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type::<MixedHamiltonianSystemWrapper>();
        let number_spins: Vec<Option<usize>> = vec![Some(4)];
        let number_bosons: Vec<Option<usize>> = vec![Some(4)];
        let number_fermions: Vec<Option<usize>> = vec![Some(4)];

        let system = new_system
            .call1((number_spins, number_bosons, number_fermions))
            .unwrap()
            .cast_as::<PyCell<MixedHamiltonianSystemWrapper>>()
            .unwrap();
        system
            .call_method1("set", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();
        system
            .call_method1("set", ("S0Z:Bc2c3a2:Fc0a0:", 0.2))
            .unwrap();
        system
            .call_method1("set", ("S0Z:Bc2c3a2:Fc0a2a3:", 0.05))
            .unwrap();

        // test access at index 0
        let comp_op = system
            .call_method1("get", ("S0Z:Bc0c1a0a1:Fc0a0:",))
            .unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.1,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system.call_method1("get", ("S0Z:Bc2c3a2:Fc0a0:",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.2,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system
            .call_method1("get", ("S0Z:Bc2c3a2:Fc0a2a3:",))
            .unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.05,)).unwrap()).unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system
            .call_method1("get", ("S0Z:Bc0c1a0a1:Fc0a2:",))
            .unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
        assert!(comparison);

        // Try_set error 1: Key (HermitianMixedProduct) cannot be converted from string
        let error = system.call_method1("set", ("d3", 0.5));
        assert!(error.is_err());

        // Try_set error 2: Value cannot be converted to CalculatorComplex
        let error = system.call_method1("set", ("S0Z:Bc2c3a2:Fc0a0:", vec![0.0]));
        assert!(error.is_err());

        // Try_set error 3: Number of bosons in entry exceeds number of bosons in system.
        let error = system.call_method1("set", ("S5X:S7Y:Bc2c3a2:Fc0a2a3:", 0.1));
        assert!(error.is_err());

        // Try_set error 4: Generic error
        let error = system.call_method1("set", (vec![0.0], 0.5));
        assert!(error.is_err());
    });
}

/// Test add_operator_product and remove functions of MixedHamiltonianSystem
#[test]
fn boson_system_test_add_operator_product_remove() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type::<MixedHamiltonianSystemWrapper>();
        let number_spins: Vec<Option<usize>> = vec![Some(4)];
        let number_bosons: Vec<Option<usize>> = vec![Some(4)];
        let number_fermions: Vec<Option<usize>> = vec![Some(4)];
        let system = new_system
            .call1((number_spins, number_bosons, number_fermions))
            .unwrap()
            .cast_as::<PyCell<MixedHamiltonianSystemWrapper>>()
            .unwrap();
        system
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();
        system
            .call_method1("add_operator_product", ("S0Z:Bc2c3a2:Fc0a0:", 0.2))
            .unwrap();
        system
            .call_method1("add_operator_product", ("S0Z:Bc2c3a2:Fc0a2a3:", 0.05))
            .unwrap();

        // test access at index 0
        let comp_op = system
            .call_method1("get", ("S0Z:Bc0c1a0a1:Fc0a0:",))
            .unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.1,)).unwrap()).unwrap();
        assert!(comparison);
        system
            .call_method1("remove", ("S0Z:Bc0c1a0a1:Fc0a0:",))
            .unwrap();
        let comp_op = system
            .call_method1("get", ("S0Z:Bc0c1a0a1:Fc0a0:",))
            .unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system.call_method1("get", ("S0Z:Bc2c3a2:Fc0a0:",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.2,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system
            .call_method1("get", ("S0Z:Bc2c3a2:Fc0a2a3:",))
            .unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.05,)).unwrap()).unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system
            .call_method1("get", ("S0Z1Y:Bc2c3a2:Fc0a2a3:",))
            .unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
        assert!(comparison);

        // Get error
        let error = system.call_method1("get", ("d2",));
        assert!(error.is_err());

        // Try_set error 1: Key (HermitianMixedProduct) cannot be converted from string
        let error = system.call_method1("add_operator_product", ("d2", 0.5));
        assert!(error.is_err());

        // Try_set error 2: Value cannot be converted to CalculatorComplex
        let error = system.call_method1("add_operator_product", ("S0Z:Bc2c3a2:Fc0a0:", vec![0.0]));
        assert!(error.is_err());

        // Try_set error 3: Number of bosons in entry exceeds number of bosons in system.
        let error = system.call_method1("add_operator_product", ("S5X:S7Y:Bc2c3a2:Fc0a2a3:", 0.1));
        assert!(error.is_err());

        // Try_set error 4: Generic error
        let error = system.call_method1("add_operator_product", (vec![0.0], 0.5));
        assert!(error.is_err());
    });
}

/// Test keys function of MixedHamiltonianSystem
#[test]
fn test_keys_values() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system = new_system(py, number_spins, number_bosons, number_fermions);

        let len_system = system.call_method0("__len__").unwrap();
        let comparison =
            bool::extract(len_system.call_method1("__eq__", (0_u64,)).unwrap()).unwrap();
        assert!(comparison);
        let empty_system = bool::extract(system.call_method0("is_empty").unwrap()).unwrap();
        assert!(empty_system);

        system
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();

        let keys_system = system.call_method0("keys").unwrap();
        let comparison = bool::extract(
            keys_system
                .call_method1("__eq__", (vec!["S0Z:Bc0c1a0a1:Fc0a0:"],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let values_system = system.call_method0("values").unwrap();
        let comparison =
            bool::extract(values_system.call_method1("__eq__", (vec![0.1],)).unwrap()).unwrap();
        assert!(comparison);

        let len_system = system.call_method0("__len__").unwrap();
        let comparison =
            bool::extract(len_system.call_method1("__eq__", (1_u64,)).unwrap()).unwrap();
        assert!(comparison);
        let empty_system = bool::extract(system.call_method0("is_empty").unwrap()).unwrap();
        assert!(!empty_system);
    });
}

#[test_case(1.0,0.0;"real")]
fn test_truncate(re: f64, im: f64) {
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];

        let system = new_system(
            py,
            number_spins.clone(),
            number_bosons.clone(),
            number_fermions.clone(),
        );
        system
            .call_method1(
                "add_operator_product",
                (
                    "S0Z:Bc0c1a0a1:Fc0a0:",
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::new(100.0 * re, 100.0 * im),
                    },
                ),
            )
            .unwrap();
        system
            .call_method1(
                "add_operator_product",
                (
                    "S0X:Bc0a0:Fc0a0",
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::new(10.0 * re, 10.0 * im),
                    },
                ),
            )
            .unwrap();
        system
            .call_method1(
                "add_operator_product",
                (
                    "S0X1Z3Z:Bc0a0:Fc0a0a1",
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::new(re, im),
                    },
                ),
            )
            .unwrap();
        system
            .call_method1(
                "add_operator_product",
                (
                    "S0X1Z3Z:Bc0a0:Fc0a0",
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::from("test"),
                    },
                ),
            )
            .unwrap();

        let test_system1 = new_system(
            py,
            number_spins.clone(),
            number_bosons.clone(),
            number_fermions.clone(),
        );
        test_system1
            .call_method1(
                "add_operator_product",
                (
                    "S0Z:Bc0c1a0a1:Fc0a0:",
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::new(100.0 * re, 100.0 * im),
                    },
                ),
            )
            .unwrap();
        test_system1
            .call_method1(
                "add_operator_product",
                (
                    "S0X:Bc0a0:Fc0a0",
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::new(10.0 * re, 10.0 * im),
                    },
                ),
            )
            .unwrap();
        test_system1
            .call_method1(
                "add_operator_product",
                (
                    "S0X1Z3Z:Bc0a0:Fc0a0",
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::from("test"),
                    },
                ),
            )
            .unwrap();

        let test_system2 = new_system(py, number_spins, number_bosons, number_fermions);
        test_system2
            .call_method1(
                "add_operator_product",
                (
                    "S0Z:Bc0c1a0a1:Fc0a0:",
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::new(100.0 * re, 100.0 * im),
                    },
                ),
            )
            .unwrap();
        test_system2
            .call_method1(
                "add_operator_product",
                (
                    "S0X1Z3Z:Bc0a0:Fc0a0",
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::from("test"),
                    },
                ),
            )
            .unwrap();

        let comparison_system1 = system.call_method1("truncate", (5.0_f64,)).unwrap();
        let comparison = bool::extract(
            comparison_system1
                .call_method1("__eq__", (test_system1,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let comparison_system2 = system.call_method1("truncate", (50.0_f64,)).unwrap();
        let comparison = bool::extract(
            comparison_system2
                .call_method1("__eq__", (test_system2,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of MixedHamiltonianSystem
#[test]
fn test_neg() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![Some(2)];
        let number_bosons: Vec<Option<usize>> = vec![Some(2)];
        let number_fermions: Vec<Option<usize>> = vec![Some(2)];
        let system_0 = new_system(
            py,
            number_spins.clone(),
            number_bosons.clone(),
            number_fermions.clone(),
        );
        system_0
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();
        let system_1 = new_system(py, number_spins, number_bosons, number_fermions);
        system_1
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", -0.1))
            .unwrap();

        let negated = system_0.call_method0("__neg__").unwrap();
        let comparison =
            bool::extract(negated.call_method1("__eq__", (system_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of MixedHamiltonianSystem
#[test]
fn test_add() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![Some(4)];
        let number_bosons: Vec<Option<usize>> = vec![Some(4)];
        let number_fermions: Vec<Option<usize>> = vec![Some(4)];
        let system_0 = new_system(
            py,
            number_spins.clone(),
            number_bosons.clone(),
            number_fermions.clone(),
        );
        system_0
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();
        let system_1 = new_system(
            py,
            number_spins.clone(),
            number_bosons.clone(),
            number_fermions.clone(),
        );
        system_1
            .call_method1("add_operator_product", ("S0Z:Bc2c3a2:Fc0a0:", 0.2))
            .unwrap();
        let system_0_1 = new_system(py, number_spins, number_bosons, number_fermions);
        system_0_1
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();
        system_0_1
            .call_method1("add_operator_product", ("S0Z:Bc2c3a2:Fc0a0:", 0.2))
            .unwrap();

        let added = system_0.call_method1("__add__", (system_1,)).unwrap();
        let comparison =
            bool::extract(added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of MixedHamiltonianSystem
#[test]
fn test_sub() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![Some(4)];
        let number_bosons: Vec<Option<usize>> = vec![Some(4)];
        let number_fermions: Vec<Option<usize>> = vec![Some(4)];
        let system_0 = new_system(
            py,
            number_spins.clone(),
            number_bosons.clone(),
            number_fermions.clone(),
        );
        system_0
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();
        let system_1 = new_system(
            py,
            number_spins.clone(),
            number_bosons.clone(),
            number_fermions.clone(),
        );
        system_1
            .call_method1("add_operator_product", ("S0Z:Bc2c3a2:Fc0a0:", 0.2))
            .unwrap();
        let system_0_1 = new_system(py, number_spins, number_bosons, number_fermions);
        system_0_1
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();
        system_0_1
            .call_method1("add_operator_product", ("S0Z:Bc2c3a2:Fc0a0:", -0.2))
            .unwrap();

        let added = system_0.call_method1("__sub__", (system_1,)).unwrap();
        let comparison =
            bool::extract(added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of MixedHamiltonianSystem
#[test]
fn test_mul_cf() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![Some(2)];
        let number_bosons: Vec<Option<usize>> = vec![Some(2)];
        let number_fermions: Vec<Option<usize>> = vec![Some(2)];
        let system_0 = new_system(
            py,
            number_spins.clone(),
            number_bosons.clone(),
            number_fermions.clone(),
        );
        system_0
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1_f64))
            .unwrap();

        let new_mixed_system = py.get_type::<MixedSystemWrapper>();
        let system_0_1 = new_mixed_system
            .call1((number_spins, number_bosons, number_fermions))
            .unwrap()
            .cast_as::<PyCell<MixedSystemWrapper>>()
            .unwrap();
        system_0_1
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.2))
            .unwrap();

        let added = system_0.call_method1("__mul__", (2.0,)).unwrap();
        let comparison =
            bool::extract(added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of MixedHamiltonianSystem
#[test]
fn test_mul_cf_with_conj() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![Some(2)];
        let number_bosons: Vec<Option<usize>> = vec![Some(2)];
        let number_fermions: Vec<Option<usize>> = vec![Some(2)];
        let system_0 = new_system(
            py,
            number_spins.clone(),
            number_bosons.clone(),
            number_fermions.clone(),
        );
        system_0
            .call_method1("add_operator_product", ("S0Z:Bc0a1:Fc0a0:", 0.1_f64))
            .unwrap();

        let new_mixed_system = py.get_type::<MixedSystemWrapper>();
        let system_0_1 = new_mixed_system
            .call1((number_spins, number_bosons, number_fermions))
            .unwrap()
            .cast_as::<PyCell<MixedSystemWrapper>>()
            .unwrap();
        system_0_1
            .call_method1("add_operator_product", ("S0Z:Bc0a1:Fc0a0:", 0.2))
            .unwrap();
        system_0_1
            .call_method1("add_operator_product", ("S0Z:Bc1a0:Fc0a0:", 0.2))
            .unwrap();

        let added = system_0.call_method1("__mul__", (2.0,)).unwrap();
        let comparison =
            bool::extract(added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of MixedHamiltonianSystem CHECK + X
#[test]
fn test_mul_cc() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![Some(2)];
        let number_bosons: Vec<Option<usize>> = vec![Some(2)];
        let number_fermions: Vec<Option<usize>> = vec![Some(2)];
        let system_0 = new_system(
            py,
            number_spins.clone(),
            number_bosons.clone(),
            number_fermions.clone(),
        );
        system_0
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1_f64))
            .unwrap();

        let new_mixed_system = py.get_type::<MixedSystemWrapper>();
        let system_0_1 = new_mixed_system
            .call1((number_spins, number_bosons, number_fermions))
            .unwrap()
            .cast_as::<PyCell<MixedSystemWrapper>>()
            .unwrap();
        system_0_1
            .call_method1(
                "add_operator_product",
                ("S0Z:Bc0c1a0a1:Fc0a0:", Complex64::new(0.0, 0.5)),
            )
            .unwrap();

        let added = system_0
            .call_method1(
                "__mul__",
                (CalculatorComplexWrapper {
                    internal: CalculatorComplex::new(0.0, 5.0),
                },),
            )
            .unwrap();
        let comparison =
            bool::extract(added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of MixedHamiltonianSystem CHECK + X
#[test]
fn test_mul_cc_with_conj() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![Some(2)];
        let number_bosons: Vec<Option<usize>> = vec![Some(2)];
        let number_fermions: Vec<Option<usize>> = vec![Some(2)];
        let system_0 = new_system(
            py,
            number_spins.clone(),
            number_bosons.clone(),
            number_fermions.clone(),
        );
        system_0
            .call_method1("add_operator_product", ("S0Z:Bc0a1:Fc0a0:", 0.1_f64))
            .unwrap();

        let new_mixed_system = py.get_type::<MixedSystemWrapper>();
        let system_0_1 = new_mixed_system
            .call1((number_spins, number_bosons, number_fermions))
            .unwrap()
            .cast_as::<PyCell<MixedSystemWrapper>>()
            .unwrap();
        system_0_1
            .call_method1(
                "add_operator_product",
                ("S0Z:Bc0a1:Fc0a0:", Complex64::new(0.0, 0.5)),
            )
            .unwrap();
        system_0_1
            .call_method1(
                "add_operator_product",
                ("S0Z:Bc1a0:Fc0a0:", Complex64::new(0.0, 0.5)),
            )
            .unwrap();

        let added = system_0
            .call_method1(
                "__mul__",
                (CalculatorComplexWrapper {
                    internal: CalculatorComplex::new(0.0, 5.0),
                },),
            )
            .unwrap();
        let comparison =
            bool::extract(added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of MixedHamiltonianSystem
#[test]
fn test_mul_self() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![Some(4)];
        let number_bosons: Vec<Option<usize>> = vec![Some(4)];
        let number_fermions: Vec<Option<usize>> = vec![Some(4)];
        let system_0 = new_system(
            py,
            number_spins.clone(),
            number_bosons.clone(),
            number_fermions.clone(),
        );
        system_0
            .call_method1("add_operator_product", ("S1X:Bc0a0:Fc2a2", 0.1))
            .unwrap();
        let system_1 = new_system(
            py,
            number_spins.clone(),
            number_bosons.clone(),
            number_fermions.clone(),
        );
        system_1
            .call_method1("add_operator_product", ("S0Z:Bc2c3a2:Fc0a0:", 1.0))
            .unwrap();
        let system_0_1 = new_system(py, number_spins, number_bosons, number_fermions);
        system_0_1
            .call_method1(
                "add_operator_product",
                ("S0Z1X:Bc0c2c3a0a2:Fc0c2a0a2", -0.2),
            )
            .unwrap();
        system_0_1
            .call_method1(
                "add_operator_product",
                ("S0Z1X:Bc0c2a0a2a3:Fc0c2a0a2", -0.2),
            )
            .unwrap();

        let added = system_0.call_method1("__mul__", (system_1,)).unwrap();
        let comparison =
            bool::extract(added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of MixedHamiltonianSystem
#[test]
fn test_mul_error() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![Some(2)];
        let number_bosons: Vec<Option<usize>> = vec![Some(2)];
        let number_fermions: Vec<Option<usize>> = vec![Some(2)];
        let system_0 = new_system(py, number_spins, number_bosons, number_fermions);
        system_0
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1_f64))
            .unwrap();

        let added = system_0.call_method1("__mul__", (vec![0.0],));
        assert!(added.is_err());

        let added = system_0.call_method1(
            "__mul__",
            (MixedHamiltonianSystemWrapper {
                internal: MixedHamiltonianSystem::default(),
            },),
        );
        assert!(added.is_err());
    });
}

/// Test copy and deepcopy functions of MixedHamiltonianSystem
#[test]
fn test_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system = new_system(py, number_spins, number_bosons, number_fermions);
        system
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();

        let copy_system = system.call_method0("__copy__").unwrap();
        let deepcopy_system = system.call_method1("__deepcopy__", ("",)).unwrap();

        let comparison_copy =
            bool::extract(copy_system.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison_copy);
        let comparison_deepcopy =
            bool::extract(deepcopy_system.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison_deepcopy);
    });
}

/// Test to_bincode and from_bincode functions of MixedHamiltonianSystem
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system = new_system(
            py,
            number_spins.clone(),
            number_bosons.clone(),
            number_fermions.clone(),
        );
        system
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();

        let serialised = system.call_method0("to_bincode").unwrap();
        let new = new_system(py, number_spins, number_bosons, number_fermions);
        let deserialised = new.call_method1("from_bincode", (serialised,)).unwrap();

        let deserialised_error =
            new.call_method1("from_bincode", (bincode::serialize("fails").unwrap(),));
        assert!(deserialised_error.is_err());

        let deserialised_error =
            new.call_method1("from_bincode", (bincode::serialize(&vec![0]).unwrap(),));
        assert!(deserialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_bincode");
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_bincode");
        assert!(serialised_error.is_err());

        let comparison =
            bool::extract(deserialised.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison)
    });
}

#[test]
fn test_value_error_bincode() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let new = new_system(py, number_spins, number_bosons, number_fermions);
        let deserialised_error = new.call_method1("from_bincode", ("J",));
        assert!(deserialised_error.is_err());
    });
}

/// Test to_ and from_json functions of MixedHamiltonianSystem
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system = new_system(
            py,
            number_spins.clone(),
            number_bosons.clone(),
            number_fermions.clone(),
        );
        system
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();

        let serialised = system.call_method0("to_json").unwrap();
        let new = new_system(py, number_spins, number_bosons, number_fermions);
        let deserialised = new.call_method1("from_json", (serialised,)).unwrap();

        let deserialised_error =
            new.call_method1("from_json", (serde_json::to_string("fails").unwrap(),));
        assert!(deserialised_error.is_err());

        let deserialised_error =
            new.call_method1("from_json", (serde_json::to_string(&vec![0]).unwrap(),));
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_json");
        assert!(serialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_json");
        assert!(deserialised_error.is_err());

        let comparison =
            bool::extract(deserialised.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison)
    });
}

/// Test the __repr__ and __format__ functions
#[test]
fn test_format_repr() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system = new_system(py, number_spins, number_bosons, number_fermions);
        system
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1_f64))
            .unwrap();
        let mut rust_system = MixedHamiltonianSystem::new(vec![None], vec![None], vec![None]);
        rust_system
            .add_operator_product(
                HermitianMixedProduct::new(
                    vec![PauliProduct::new().z(0)],
                    vec![BosonProduct::new([0], [1]).unwrap()],
                    vec![FermionProduct::new([0], [0]).unwrap()],
                )
                .unwrap(),
                CalculatorComplex::new(0.1, 0.0),
            )
            .unwrap();
        let to_format = system.call_method1("__format__", ("",)).unwrap();
        let format_op: &str = <&str>::extract(to_format).unwrap();

        let to_repr = system.call_method0("__repr__").unwrap();
        let repr_op: &str = <&str>::extract(to_repr).unwrap();

        let to_str = system.call_method0("__str__").unwrap();
        let str_op: &str = <&str>::extract(to_str).unwrap();

        assert_eq!(
        format_op,
        "MixedHamiltonianSystem(\nnumber_spins: 1, \nnumber_bosons: 2, \nnumber_fermions: 1, )\n{S0Z:Bc0c1a0a1:Fc0a0:: (1e-1 + i * 0e0),\n}".to_string()
    );
        assert_eq!(
        repr_op,
        "MixedHamiltonianSystem(\nnumber_spins: 1, \nnumber_bosons: 2, \nnumber_fermions: 1, )\n{S0Z:Bc0c1a0a1:Fc0a0:: (1e-1 + i * 0e0),\n}".to_string()
    );
        assert_eq!(
        str_op,
        "MixedHamiltonianSystem(\nnumber_spins: 1, \nnumber_bosons: 2, \nnumber_fermions: 1, )\n{S0Z:Bc0c1a0a1:Fc0a0:: (1e-1 + i * 0e0),\n}".to_string()
    );
    });
}

/// Test the __richcmp__ function
#[test]
fn test_richcmp() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system_one = new_system(
            py,
            number_spins.clone(),
            number_bosons.clone(),
            number_fermions.clone(),
        );
        system_one
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();
        let system_two = new_system(py, number_spins, number_bosons, number_fermions);
        system_two
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a2:", 0.1))
            .unwrap();

        let comparison =
            bool::extract(system_one.call_method1("__eq__", (system_two,)).unwrap()).unwrap();
        assert!(!comparison);
        let comparison = bool::extract(
            system_one
                .call_method1("__eq__", ("S0Z:Bc0c1a0a1:Fc0a0:",))
                .unwrap(),
        )
        .unwrap();
        assert!(!comparison);

        let comparison =
            bool::extract(system_one.call_method1("__ne__", (system_two,)).unwrap()).unwrap();
        assert!(comparison);
        let comparison = bool::extract(
            system_one
                .call_method1("__ne__", ("S0Z:Bc0c1a0a1:Fc0a0:",))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let comparison = system_one.call_method1("__ge__", ("S0Z:Bc0c1a0a1:Fc0a0:",));
        assert!(comparison.is_err());
    });
}
