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

use num_complex::Complex64;
use pyo3::prelude::*;
use qoqo_calculator::CalculatorComplex;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::bosons::BosonProduct;
use struqture::fermions::FermionProduct;
use struqture::mixed_systems::{HermitianMixedProduct, MixedHamiltonian};
use struqture::prelude::MixedIndex;
use struqture::spins::PauliProduct;
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture::{ModeIndex, OperateOnDensityMatrix, SpinIndex};
use struqture_py::mixed_systems::{MixedHamiltonianWrapper, MixedOperatorWrapper};
use test_case::test_case;

// helper functions
fn new_system(
    py: Python,
    number_spins: usize,
    number_bosons: usize,
    number_fermions: usize,
) -> &PyCell<MixedHamiltonianWrapper> {
    let system_type = py.get_type::<MixedHamiltonianWrapper>();
    system_type
        .call1((number_spins, number_bosons, number_fermions))
        .unwrap()
        .downcast::<PyCell<MixedHamiltonianWrapper>>()
        .unwrap()
        .to_owned()
}

// helper functions
fn new_operator(
    py: Python,
    number_spins: usize,
    number_bosons: usize,
    number_fermions: usize,
) -> &PyCell<MixedOperatorWrapper> {
    let system_type = py.get_type::<MixedOperatorWrapper>();
    system_type
        .call1((number_spins, number_bosons, number_fermions))
        .unwrap()
        .downcast::<PyCell<MixedOperatorWrapper>>()
        .unwrap()
        .to_owned()
}

/// Test default function of MixedHamiltonianWrapper
#[test]
fn test_default_partialeq_debug_clone() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;
        let new_system = new_system(py, number_spins, number_bosons, number_fermions);
        new_system
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();
        let system_wrapper = new_system.extract::<MixedHamiltonianWrapper>().unwrap();

        // PartialEq
        let helper_ne: bool = MixedHamiltonianWrapper::new(1, 1, 1) != system_wrapper;
        assert!(helper_ne);
        let helper_eq: bool =
            MixedHamiltonianWrapper::new(1, 1, 1) == MixedHamiltonianWrapper::new(1, 1, 1);
        assert!(helper_eq);

        // Clone
        assert_eq!(system_wrapper.clone(), system_wrapper);

        // Debug
        assert_eq!(
            format!("{:?}", MixedHamiltonianWrapper::new(1, 1, 1)),
            "MixedHamiltonianWrapper { internal: MixedHamiltonian { internal_map: {}, n_spins: 1, n_bosons: 1, n_fermions: 1 } }"
        );
    })
}

/// Test number_bosons and number_bosons functions of MixedHamiltonian
#[test]
fn test_number_bosons_current() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;
        let system = new_system(py, number_spins, number_bosons, number_fermions);
        system
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();

        let number_system = system.call_method0("number_spins").unwrap();
        let comparison = bool::extract(
            number_system
                .call_method1("__eq__", (vec![1_u64],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let number_system = system.call_method0("number_bosonic_modes").unwrap();
        let comparison = bool::extract(
            number_system
                .call_method1("__eq__", (vec![2_u64],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let number_system = system.call_method0("number_fermionic_modes").unwrap();
        let comparison = bool::extract(
            number_system
                .call_method1("__eq__", (vec![1_u64],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    });
}

/// Test empty_clone function of MixedHamiltonian
#[test]
fn test_empty_clone() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;
        let system = new_system(py, number_spins, number_bosons, number_fermions);
        let none_system = system.call_method1("empty_clone", (4,)).unwrap();
        let comparison =
            bool::extract_bound(&none_system.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison);

        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;
        let system = new_system(py, number_spins, number_bosons, number_fermions);
        let some_system = system.call_method1("empty_clone", (2,)).unwrap();
        let comparison =
            bool::extract_bound(&some_system.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test hermitian_conjugate function of MixedHamiltonian
#[test]
fn test_hermitian_conj() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;
        let system = new_system(py, number_spins, number_bosons, number_fermions);
        system
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();

        let conjugate = system.call_method0("hermitian_conjugate").unwrap();
        let comparison =
            bool::extract_bound(&conjugate.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test set and get functions of MixedHamiltonian
#[test]
fn boson_system_test_set_get() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type::<MixedHamiltonianWrapper>();
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;

        let binding = new_system
            .call1((number_spins, number_bosons, number_fermions))
            .unwrap()
            .downcast::<PyCell<MixedHamiltonianWrapper>>()
            .unwrap();
        let system = binding.downcast::<MixedHamiltonianSystemWrapper>().unwrap();
        system
            .call_method1("set", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();
        system
            .call_method1("set", ("S0Z:Bc2a2a3:Fc0a0:", 0.2))
            .unwrap();
        system
            .call_method1("set", ("S0Z:Bc2a2a3:Fc0a2a3:", 0.05))
            .unwrap();

        // test access at index 0
        let comp_op = system
            .call_method1("get", ("S0Z:Bc0c1a0a1:Fc0a0:",))
            .unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.1,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system.call_method1("get", ("S0Z:Bc2a2a3:Fc0a0:",)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.2,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system
            .call_method1("get", ("S0Z:Bc2a2a3:Fc0a2a3:",))
            .unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.05,)).unwrap()).unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system
            .call_method1("get", ("S0Z:Bc0c1a0a1:Fc0a2:",))
            .unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
        assert!(comparison);

        // Try_set error 1: Key (HermitianMixedProduct) cannot be converted from string
        let error = system.call_method1("set", ("d3", 0.5));
        assert!(error.is_err());

        // Try_set error 2: Value cannot be converted to CalculatorComplex
        let error = system.call_method1("set", ("S0Z:Bc2a2a3:Fc0a0:", vec![0.0]));
        assert!(error.is_err());

        // Try_set error 3: Generic error
        let error = system.call_method1("set", (vec![0.0], 0.5));
        assert!(error.is_err());
    });
}

/// Test add_operator_product and remove functions of MixedHamiltonian
#[test]
fn boson_system_test_add_operator_product_remove() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type::<MixedHamiltonianWrapper>();
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;
        let system = new_system
            .call1((number_spins, number_bosons, number_fermions))
            .unwrap()
            .downcast::<PyCell<MixedHamiltonianWrapper>>()
            .unwrap();
        let system = binding.downcast::<MixedHamiltonianSystemWrapper>().unwrap();
        system
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();
        system
            .call_method1("add_operator_product", ("S0Z:Bc2a2a3:Fc0a0:", 0.2))
            .unwrap();
        system
            .call_method1("add_operator_product", ("S0Z:Bc2a2a3:Fc0a2a3:", 0.05))
            .unwrap();

        // test access at index 0
        let comp_op = system
            .call_method1("get", ("S0Z:Bc0c1a0a1:Fc0a0:",))
            .unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.1,)).unwrap()).unwrap();
        assert!(comparison);
        system
            .call_method1("remove", ("S0Z:Bc0c1a0a1:Fc0a0:",))
            .unwrap();
        let comp_op = system
            .call_method1("get", ("S0Z:Bc0c1a0a1:Fc0a0:",))
            .unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system.call_method1("get", ("S0Z:Bc2a2a3:Fc0a0:",)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.2,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system
            .call_method1("get", ("S0Z:Bc2a2a3:Fc0a2a3:",))
            .unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.05,)).unwrap()).unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system
            .call_method1("get", ("S0Z1Y:Bc2a2a3:Fc0a2a3:",))
            .unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
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

        // Try_set error 3: Generic error
        let error = system.call_method1("add_operator_product", (vec![0.0], 0.5));
        assert!(error.is_err());
    });
}

/// Test keys function of MixedHamiltonian
#[test]
fn test_keys_values() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;
        let system = new_system(py, number_spins, number_bosons, number_fermions);

        let len_system = system.call_method0("__len__").unwrap();
        let comparison =
            bool::extract_bound(&len_system.call_method1("__eq__", (0_u64,)).unwrap()).unwrap();
        assert!(comparison);
        let empty_system = bool::extract_bound(&system.call_method0("is_empty").unwrap()).unwrap();
        assert!(empty_system);

        system
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();

        let keys_system = system.call_method0("keys").unwrap();
        let comparison = bool::extract_bound(
            &keys_system
                .call_method1("__eq__", (vec!["S0Z:Bc0c1a0a1:Fc0a0:"],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let values_system = system.call_method0("values").unwrap();
        let comparison =
            bool::extract_bound(&values_system.call_method1("__eq__", (vec![0.1],)).unwrap())
                .unwrap();
        assert!(comparison);

        let len_system = system.call_method0("__len__").unwrap();
        let comparison =
            bool::extract_bound(&len_system.call_method1("__eq__", (1_u64,)).unwrap()).unwrap();
        assert!(comparison);
        let empty_system = bool::extract_bound(&system.call_method0("is_empty").unwrap()).unwrap();
        assert!(!empty_system);
    });
}

#[test_case(1.0,0.0;"real")]
fn test_truncate(re: f64, im: f64) {
    pyo3::Python::with_gil(|py| {
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;

        let system = new_system(py, number_spins, number_bosons, number_fermions);
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

        let test_system1 = new_system(py, number_spins, number_bosons, number_fermions);
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
        let comparison = bool::extract_bound(
            &comparison_system1
                .call_method1("__eq__", (test_system1,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let comparison_system2 = system.call_method1("truncate", (50.0_f64,)).unwrap();
        let comparison = bool::extract_bound(
            &comparison_system2
                .call_method1("__eq__", (test_system2,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of MixedHamiltonian
#[test]
fn test_neg() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;
        let system_0 = new_system(py, number_spins, number_bosons, number_fermions);
        system_0
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();
        let system_1 = new_system(py, number_spins, number_bosons, number_fermions);
        system_1
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", -0.1))
            .unwrap();

        let negated = system_0.call_method0("__neg__").unwrap();
        let comparison =
            bool::extract_bound(&negated.call_method1("__eq__", (system_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of MixedHamiltonian
#[test]
fn test_add() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;
        let system_0 = new_system(py, number_spins, number_bosons, number_fermions);
        system_0
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();
        let system_1 = new_system(py, number_spins, number_bosons, number_fermions);
        system_1
            .call_method1("add_operator_product", ("S0Z:Bc2a2a3:Fc0a0:", 0.2))
            .unwrap();
        let system_0_1 = new_system(py, number_spins, number_bosons, number_fermions);
        system_0_1
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();
        system_0_1
            .call_method1("add_operator_product", ("S0Z:Bc2a2a3:Fc0a0:", 0.2))
            .unwrap();

        let added = system_0.call_method1("__add__", (system_1,)).unwrap();
        let comparison =
            bool::extract_bound(&added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of MixedHamiltonian
#[test]
fn test_sub() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;
        let system_0 = new_system(py, number_spins, number_bosons, number_fermions);
        system_0
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();
        let system_1 = new_system(py, number_spins, number_bosons, number_fermions);
        system_1
            .call_method1("add_operator_product", ("S0Z:Bc2a2a3:Fc0a0:", 0.2))
            .unwrap();
        let system_0_1 = new_system(py, number_spins, number_bosons, number_fermions);
        system_0_1
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();
        system_0_1
            .call_method1("add_operator_product", ("S0Z:Bc2a2a3:Fc0a0:", -0.2))
            .unwrap();

        let added = system_0.call_method1("__sub__", (system_1,)).unwrap();
        let comparison =
            bool::extract_bound(&added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of MixedHamiltonian
#[test]
fn test_mul_cf() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;
        let system_0 = new_system(py, number_spins, number_bosons, number_fermions);
        system_0
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1_f64))
            .unwrap();

        let new_mixed_system = py.get_type::<MixedOperatorWrapper>();
        let system_0_1 = new_mixed_system
            .call1((number_spins, number_bosons, number_fermions))
            .unwrap()
            .downcast::<PyCell<MixedOperatorWrapper>>()
            .unwrap();
        system_0_1
            .downcast::<MixedSystemWrapper>()
            .unwrap()
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.2))
            .unwrap();

        let added = system_0.call_method1("__mul__", (2.0,)).unwrap();
        let comparison =
            bool::extract_bound(&added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of MixedHamiltonian
#[test]
fn test_mul_cf_with_conj() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;
        let system_0 = new_system(py, number_spins, number_bosons, number_fermions);
        system_0
            .call_method1("add_operator_product", ("S0Z:Bc0a1:Fc0a0:", 0.1_f64))
            .unwrap();

        let new_mixed_system = py.get_type::<MixedOperatorWrapper>();
        let system_0_1 = new_mixed_system
            .call1((number_spins, number_bosons, number_fermions))
            .unwrap()
            .downcast::<PyCell<MixedOperatorWrapper>>()
            .unwrap();
        system_0_1
            .downcast::<MixedSystemWrapper>()
            .unwrap()
            .call_method1("add_operator_product", ("S0Z:Bc0a1:Fc0a0:", 0.2))
            .unwrap();
        system_0_1
            .call_method1("add_operator_product", ("S0Z:Bc1a0:Fc0a0:", 0.2))
            .unwrap();

        let added = system_0.call_method1("__mul__", (2.0,)).unwrap();
        let comparison =
            bool::extract_bound(&added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of MixedHamiltonian CHECK + X
#[test]
fn test_mul_cc() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;
        let system_0 = new_system(py, number_spins, number_bosons, number_fermions);
        system_0
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1_f64))
            .unwrap();

        let new_mixed_system = py.get_type::<MixedOperatorWrapper>();
        let system_0_1 = new_mixed_system
            .call1((number_spins, number_bosons, number_fermions))
            .unwrap()
            .downcast::<PyCell<MixedOperatorWrapper>>()
            .unwrap();
        system_0_1
            .downcast::<MixedSystemWrapper>()
            .unwrap()
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
            bool::extract_bound(&added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of MixedHamiltonian CHECK + X
#[test]
fn test_mul_cc_with_conj() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;
        let system_0 = new_system(py, number_spins, number_bosons, number_fermions);
        system_0
            .call_method1("add_operator_product", ("S0Z:Bc0a1:Fc0a0:", 0.1_f64))
            .unwrap();

        let new_mixed_system = py.get_type::<MixedOperatorWrapper>();
        let system_0_1 = new_mixed_system
            .call1((number_spins, number_bosons, number_fermions))
            .unwrap()
            .downcast::<PyCell<MixedOperatorWrapper>>()
            .unwrap();
        system_0_1
            .downcast::<MixedSystemWrapper>()
            .unwrap()
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
            bool::extract_bound(&added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of MixedHamiltonian
#[test]
fn test_mul_self() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;
        let system_0 = new_system(py, number_spins, number_bosons, number_fermions);
        system_0
            .call_method1("add_operator_product", ("S1X:Ba0:Fa2", 0.1))
            .unwrap();
        let system_1 = new_system(py, number_spins, number_bosons, number_fermions);
        system_1
            .call_method1("add_operator_product", ("S0Z:Ba1:Fa3:", 1.0))
            .unwrap();
        let system_0_1 = new_operator(py, number_spins, number_bosons, number_fermions);
        system_0_1
            .call_method1("add_operator_product", ("S0Z1X:Ba0a1:Fa2a3", 0.1))
            .unwrap();
        system_0_1
            .call_method1("add_operator_product", ("S0Z1X:Bc0c1:Fc2c3", 0.1))
            .unwrap();
        system_0_1
            .call_method1("add_operator_product", ("S0Z1X:Bc0a1:Fc2a3", 0.1))
            .unwrap();
        system_0_1
            .call_method1("add_operator_product", ("S0Z1X:Bc1a0:Fc3a2", -0.1))
            .unwrap();

        let added = system_0.call_method1("__mul__", (system_1,)).unwrap();
        let comparison =
            bool::extract_bound(&added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of MixedHamiltonian
#[test]
fn test_mul_error() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;
        let system_0 = new_system(py, number_spins, number_bosons, number_fermions);
        system_0
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1_f64))
            .unwrap();

        let added = system_0.call_method1("__mul__", (vec![0.0],));
        assert!(added.is_err());
    });
}

/// Test copy and deepcopy functions of MixedHamiltonian
#[test]
fn test_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;
        let system = new_system(py, number_spins, number_bosons, number_fermions);
        system
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();

        let copy_system = system.call_method0("__copy__").unwrap();
        let deepcopy_system = system.call_method1("__deepcopy__", ("",)).unwrap();

        let comparison_copy =
            bool::extract_bound(&copy_system.call_method1("__eq__", (&system,)).unwrap()).unwrap();
        assert!(comparison_copy);
        let comparison_deepcopy =
            bool::extract_bound(&deepcopy_system.call_method1("__eq__", (system,)).unwrap())
                .unwrap();
        assert!(comparison_deepcopy);
    });
}

/// Test to_bincode and from_bincode functions of MixedHamiltonian
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;
        let system = new_system(py, number_spins, number_bosons, number_fermions);
        system
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();

        let serialised = system.call_method0("to_bincode").unwrap();
        let new = new_system(py, number_spins, number_bosons, number_fermions);
        let deserialised = new.call_method1("from_bincode", (&serialised,)).unwrap();

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
            bool::extract_bound(&deserialised.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison)
    });
}

#[test]
fn test_value_error_bincode() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;
        let new = new_system(py, number_spins, number_bosons, number_fermions);
        let deserialised_error = new.call_method1("from_bincode", ("J",));
        assert!(deserialised_error.is_err());
    });
}

/// Test to_ and from_json functions of MixedHamiltonian
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;
        let system = new_system(py, number_spins, number_bosons, number_fermions);
        system
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();

        let serialised = system.call_method0("to_json").unwrap();
        let new = new_system(py, number_spins, number_bosons, number_fermions);
        let deserialised = new.call_method1("from_json", (&serialised,)).unwrap();

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
            bool::extract_bound(&deserialised.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison)
    });
}

/// Test the __repr__ and __format__ functions
#[test]
fn test_format_repr() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;
        let system = new_system(py, number_spins, number_bosons, number_fermions);
        system
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1_f64))
            .unwrap();
        let mut rust_system = MixedHamiltonian::new(1, 1, 1);
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
        let format_op: String = String::extract_bound(&to_format).unwrap();

        let to_repr = system.call_method0("__repr__").unwrap();
        let repr_op: String = String::extract_bound(&to_repr).unwrap();

        let to_str = system.call_method0("__str__").unwrap();
        let str_op: String = String::extract_bound(&to_str).unwrap();

        assert_eq!(
            format_op,
            "MixedHamiltonian{\nS0Z:Bc0c1a0a1:Fc0a0:: (1e-1 + i * 0e0),\n}".to_string()
        );
        assert_eq!(
            repr_op,
            "MixedHamiltonian{\nS0Z:Bc0c1a0a1:Fc0a0:: (1e-1 + i * 0e0),\n}".to_string()
        );
        assert_eq!(
            str_op,
            "MixedHamiltonian{\nS0Z:Bc0c1a0a1:Fc0a0:: (1e-1 + i * 0e0),\n}".to_string()
        );
    });
}

/// Test the __richcmp__ function
#[test]
fn test_richcmp() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: usize = 1;
        let number_bosons: usize = 1;
        let number_fermions: usize = 1;
        let system_one = new_system(py, number_spins, number_bosons, number_fermions);
        system_one
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 0.1))
            .unwrap();
        let system_two = new_system(py, number_spins, number_bosons, number_fermions);
        system_two
            .call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a2:", 0.1))
            .unwrap();

        let comparison =
            bool::extract_bound(&system_one.call_method1("__eq__", (&system_two,)).unwrap())
                .unwrap();
        assert!(!comparison);
        let comparison = bool::extract_bound(
            &system_one
                .call_method1("__eq__", ("S0Z:Bc0c1a0a1:Fc0a0:",))
                .unwrap(),
        )
        .unwrap();
        assert!(!comparison);

        let comparison =
            bool::extract_bound(&system_one.call_method1("__ne__", (system_two,)).unwrap())
                .unwrap();
        assert!(comparison);
        let comparison = bool::extract_bound(
            &system_one
                .call_method1("__ne__", ("S0Z:Bc0c1a0a1:Fc0a0:",))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let comparison = system_one.call_method1("__ge__", ("S0Z:Bc0c1a0a1:Fc0a0:",));
        assert!(comparison.is_err());
    });
}

#[cfg(feature = "json_schema")]
#[test]
fn test_json_schema() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new = new_system(py, 1, 1, 1);

        let schema: String =
            String::extract_bound(&new.call_method0("json_schema").unwrap()).unwrap();
        let rust_schema =
            serde_json::to_string_pretty(&schemars::schema_for!(MixedHamiltonian)).unwrap();
        assert_eq!(schema, rust_schema);

        let version: String =
            String::extract_bound(&new.call_method0("current_version").unwrap()).unwrap();
        let rust_version = STRUQTURE_VERSION.to_string();
        assert_eq!(version, rust_version);

        new.call_method1("add_operator_product", ("S0Z:Bc0c1a0a1:Fc0a0:", 1.0))
            .unwrap();
        let min_version: String =
            String::extract(new.call_method0("min_supported_version").unwrap()).unwrap();
        let rust_min_version = String::from("2.0.0");
        assert_eq!(min_version, rust_min_version);
    });
}

#[cfg(feature = "unstable_struqture_2_import")]
#[test]
fn test_from_json_struqture_1() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let json_string: Bound<pyo3::types::PyString> = pyo3::types::PyString::new(py, "{\"items\":[[\"S0Z:Bc1a1:Fc0a0:\",1.0,0.0]],\"n_spins\":1,\"n_bosons\":1,\"n_fermions\":1,\"serialisation_meta\":{\"type_name\":\"MixedHamiltonian\",\"min_version\":[2,0,0],\"version\":\"2.0.0-alpha.9\"}}");
        let sys_2 = new_system(py, vec![None], vec![None], vec![None]);
        sys_2
            .call_method1("add_operator_product", ("S0Z:Bc1a1:Fc0a0", 1.0))
            .unwrap();

        let sys_from_1 = sys_2
            .call_method1("from_json_struqture_2", (json_string,))
            .unwrap();
        let equal =
            bool::extract_bound(&sys_2.call_method1("__eq__", (sys_from_1,)).unwrap()).unwrap();
        assert!(equal);

        let error_json_string: Bound<pyo3::types::PyString> = pyo3::types::PyString::new(py, "{\"items\":[[\"S0Z:Bc1a1:Fc0a0:\",1.0,0.0]],\"n_spins\":1,\"n_bosons\":1,\"n_fermions\":1,\"serialisation_meta\":{\"type_name\":\"MixedHamiltonian\",\"min_version\":[30,0,0],\"version\":\"2.0.0-alpha.9\"}}");
        let sys_from_1 = sys_2.call_method1("from_json_struqture_2", (error_json_string,));
        assert!(sys_from_1.is_err());
    });
}
