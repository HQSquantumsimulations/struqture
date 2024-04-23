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

// use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use qoqo_calculator_pyo3::{CalculatorComplexWrapper, CalculatorFloatWrapper};
use struqture::bosons::BosonProduct;
use struqture::fermions::FermionProduct;
use struqture::mixed_systems::{
    HermitianMixedProduct, MixedDecoherenceProduct, MixedHamiltonianSystem,
    MixedLindbladNoiseSystem, MixedLindbladOpenSystem,
};
use struqture::prelude::MixedIndex;
use struqture::spins::{DecoherenceProduct, PauliProduct};
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture::{ModeIndex, OpenSystem, OperateOnDensityMatrix, SpinIndex};
use struqture_py::mixed_systems::{
    MixedHamiltonianSystemWrapper, MixedLindbladNoiseSystemWrapper, MixedLindbladOpenSystemWrapper,
};
use test_case::test_case;

// helper functions
fn new_system(
    py: Python,
    number_spins: Vec<Option<usize>>,
    number_bosons: Vec<Option<usize>>,
    number_fermions: Vec<Option<usize>>,
) -> Bound<MixedLindbladOpenSystemWrapper> {
    let system_type = py.get_type_bound::<MixedLindbladOpenSystemWrapper>();
    system_type
        .call1((number_spins, number_bosons, number_fermions))
        .unwrap()
        .downcast::<MixedLindbladOpenSystemWrapper>()
        .unwrap()
        .to_owned()
}

// helper function to convert CalculatorFloat into a python object
fn convert_cf_to_pyobject(py: Python, parameter: CalculatorFloat) -> Bound<CalculatorFloatWrapper> {
    let parameter_type = py.get_type_bound::<CalculatorFloatWrapper>();
    match parameter {
        CalculatorFloat::Float(x) => parameter_type
            .call1((x,))
            .unwrap()
            .downcast::<CalculatorFloatWrapper>()
            .unwrap()
            .to_owned(),
        CalculatorFloat::Str(x) => parameter_type
            .call1((x,))
            .unwrap()
            .downcast::<CalculatorFloatWrapper>()
            .unwrap()
            .to_owned(),
    }
}

/// Test number_modes and current_number_modes functions of MixedSystem
#[test]
fn test_number_modes_current() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system = new_system(py, number_spins, number_bosons, number_fermions);
        system
            .call_method1(
                "noise_add_operator_product",
                (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"), 0.1),
            )
            .unwrap();

        let number_system = system.call_method0("number_spins").unwrap();
        let current_system = system.call_method0("current_number_spins").unwrap();
        let comparison = bool::extract_bound(
            &number_system
                .call_method1("__eq__", (vec![1_u64],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        let comparison = bool::extract_bound(
            &current_system
                .call_method1("__eq__", (vec![1_u64],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let number_system = system.call_method0("number_bosonic_modes").unwrap();
        let current_system = system.call_method0("current_number_bosonic_modes").unwrap();
        let comparison = bool::extract_bound(
            &number_system
                .call_method1("__eq__", (vec![2_u64],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        let comparison = bool::extract_bound(
            &current_system
                .call_method1("__eq__", (vec![2_u64],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let number_system = system.call_method0("number_fermionic_modes").unwrap();
        let current_system = system
            .call_method0("current_number_fermionic_modes")
            .unwrap();
        let comparison = bool::extract_bound(
            &number_system
                .call_method1("__eq__", (vec![1_u64],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        let comparison = bool::extract_bound(
            &current_system
                .call_method1("__eq__", (vec![1_u64],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    });
}

/// Test empty_clone function of MixedSystem
#[test]
fn test_empty_clone() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system = new_system(py, number_spins, number_bosons, number_fermions);
        let none_system = system.call_method0("empty_clone").unwrap();
        let comparison =
            bool::extract_bound(&none_system.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison);

        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system = new_system(py, number_spins, number_bosons, number_fermions);
        let some_system = system.call_method0("empty_clone").unwrap();
        let comparison =
            bool::extract_bound(&some_system.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add_operator_product and remove functions of MixedSystem
#[test]
fn mixed_system_test_add_operator_product_remove_system() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type_bound::<MixedLindbladOpenSystemWrapper>();
        let number_modes: Option<usize> = Some(4);
        let binding = new_system
            .call1((vec![number_modes], vec![number_modes], vec![number_modes]))
            .unwrap();
        let system = binding
            .downcast::<MixedLindbladOpenSystemWrapper>()
            .unwrap();
        system
            .call_method1("system_add_operator_product", ("S0Z:Bc0a1:Fc0a0:", 0.1))
            .unwrap();
        system
            .call_method1("system_add_operator_product", ("S0Z:Bc0a1:Fc0a2:", 0.2))
            .unwrap();
        system
            .call_method1("system_add_operator_product", ("S0Z:Bc3a3:Fc0a2:", 0.05))
            .unwrap();

        // test access at index 0
        let comp_op = system
            .call_method1("system_get", ("S0Z:Bc0a1:Fc0a0:",))
            .unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.1,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system
            .call_method1("system_get", ("S0Z:Bc0a1:Fc0a2:",))
            .unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.2,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system
            .call_method1("system_get", ("S0Z:Bc3a3:Fc0a2:",))
            .unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.05,)).unwrap()).unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system
            .call_method1("system_get", ("S0Z:Bc2a2:Fc0a1:",))
            .unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
        assert!(comparison);

        // Get error
        let error = system.call_method1("system_get", ("j2",));
        assert!(error.is_err());

        // Try_set error 1: Key (MixedProduct) cannot be converted from string
        let error = system.call_method1("system_add_operator_product", ("j1", 0.5));
        assert!(error.is_err());

        // Try_set error 2: Value cannot be converted to CalculatorComplex
        let error = system.call_method1(
            "system_add_operator_product",
            ("S0Z:Bc0a1:Fc0a2:", vec![0.0]),
        );
        assert!(error.is_err());

        // Try_set error 3: Number of Mixeds in entry exceeds number of Mixeds in system.
        let error =
            system.call_method1("system_add_operator_product", ("S5Z:S6X:Bc2a2:Fc0a1:", 0.1));
        assert!(error.is_err());

        // Try_set error 4: Generic error
        let error = system.call_method1("system_add_operator_product", ("j1", 0.5));
        assert!(error.is_err());
    });
}

/// Test add_operator_product and remove functions of MixedSystem
#[test]
fn mixed_system_test_add_operator_product_remove_noise() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type_bound::<MixedLindbladOpenSystemWrapper>();
        let number_modes: Option<usize> = Some(4);
        let binding = new_system
            .call1((vec![number_modes], vec![number_modes], vec![number_modes]))
            .unwrap();
        let system = binding
            .downcast::<MixedLindbladOpenSystemWrapper>()
            .unwrap();
        system
            .call_method1(
                "noise_add_operator_product",
                (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"), 0.1),
            )
            .unwrap();
        system
            .call_method1(
                "noise_add_operator_product",
                (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a2:"), 0.2),
            )
            .unwrap();
        system
            .call_method1(
                "noise_add_operator_product",
                (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc3a3:Fc0a2:"), 0.05),
            )
            .unwrap();

        // test access at index 0
        let comp_op = system
            .call_method1("noise_get", (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"),))
            .unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.1,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system
            .call_method1("noise_get", (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a2:"),))
            .unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.2,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system
            .call_method1("noise_get", (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc3a3:Fc0a2:"),))
            .unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.05,)).unwrap()).unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system
            .call_method1("noise_get", (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc2a2:Fc0a1:"),))
            .unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
        assert!(comparison);

        // Get error
        let error = system.call_method1("noise_get", (("j2", "S0Z:Bc0a1:Fc0a0:"),));
        assert!(error.is_err());

        // Try_set error 1: Key (MixedProduct) cannot be converted from string
        let error = system.call_method1(
            "noise_add_operator_product",
            (("j1", "S0Z:Bc0a1:Fc0a0:"), 0.5),
        );
        assert!(error.is_err());

        // Try_set error 2: Value cannot be converted to CalculatorComplex
        let error = system.call_method1(
            "noise_add_operator_product",
            (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a2:"), vec![0.0]),
        );
        assert!(error.is_err());

        // Try_set error 3: Number of Mixeds in entry exceeds number of Mixeds in system.
        let error = system.call_method1(
            "noise_add_operator_product",
            (("S0Z:Bc0a1:Fc0a0:", "S5Z:S6X:Bc2a2:Fc0a1:"), 0.1),
        );
        assert!(error.is_err());

        // Try_set error 4: Generic error
        let error = system.call_method1(
            "noise_add_operator_product",
            (("S0Z:Bc0a1:Fc0a0:", "j1"), 0.5),
        );
        assert!(error.is_err());
    });
}

/// Test add magic method function of MixedSystem
#[test]
fn test_neg() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system_0 = new_system(py, number_spins, number_bosons, number_fermions);
        system_0
            .call_method1(
                "noise_add_operator_product",
                (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"), 0.1),
            )
            .unwrap();
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system_1 = new_system(py, number_spins, number_bosons, number_fermions);
        system_1
            .call_method1(
                "noise_add_operator_product",
                (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"), -0.1),
            )
            .unwrap();

        let negated = system_0.call_method0("__neg__").unwrap();
        let comparison =
            bool::extract_bound(&negated.call_method1("__eq__", (system_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of MixedSystem
#[test]
fn test_add() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system_0 = new_system(py, number_spins, number_bosons, number_fermions);
        system_0
            .call_method1(
                "noise_add_operator_product",
                (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"), 0.1),
            )
            .unwrap();
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system_1 = new_system(py, number_spins, number_bosons, number_fermions);
        system_1
            .call_method1(
                "noise_add_operator_product",
                (("S0Z:Bc0a1:Fc0a2:", "S0Z:Bc0a1:Fc0a0:"), 0.2),
            )
            .unwrap();
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system_0_1 = new_system(py, number_spins, number_bosons, number_fermions);
        system_0_1
            .call_method1(
                "noise_add_operator_product",
                (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"), 0.1),
            )
            .unwrap();
        system_0_1
            .call_method1(
                "noise_add_operator_product",
                (("S0Z:Bc0a1:Fc0a2:", "S0Z:Bc0a1:Fc0a0:"), 0.2),
            )
            .unwrap();

        let added = system_0.call_method1("__add__", (system_1,)).unwrap();
        let comparison =
            bool::extract_bound(&added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of MixedSystem
#[test]
fn test_sub() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system_0 = new_system(py, number_spins, number_bosons, number_fermions);
        system_0
            .call_method1(
                "noise_add_operator_product",
                (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"), 0.1),
            )
            .unwrap();
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system_1 = new_system(py, number_spins, number_bosons, number_fermions);
        system_1
            .call_method1(
                "noise_add_operator_product",
                (("S0Z:Bc0a1:Fc0a2:", "S0Z:Bc0a1:Fc0a0:"), 0.2),
            )
            .unwrap();
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system_0_1 = new_system(py, number_spins, number_bosons, number_fermions);
        system_0_1
            .call_method1(
                "noise_add_operator_product",
                (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"), 0.1),
            )
            .unwrap();
        system_0_1
            .call_method1(
                "noise_add_operator_product",
                (("S0Z:Bc0a1:Fc0a2:", "S0Z:Bc0a1:Fc0a0:"), -0.2),
            )
            .unwrap();

        let added = system_0.call_method1("__sub__", (system_1,)).unwrap();
        let comparison =
            bool::extract_bound(&added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of MixedSystem
#[test]
fn test_mul_cf() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system_0 = new_system(py, number_spins, number_bosons, number_fermions);
        system_0
            .call_method1(
                "noise_add_operator_product",
                (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"), 0.1_f64),
            )
            .unwrap();

        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system_0_1 = new_system(py, number_spins, number_bosons, number_fermions);
        system_0_1
            .call_method1(
                "noise_add_operator_product",
                (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"), 0.2),
            )
            .unwrap();

        let added = system_0.call_method1("__mul__", (2.0,)).unwrap();
        let comparison =
            bool::extract_bound(&added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test default function of MixedLindbladOpenSystemWrapper
#[test]
fn test_default_partialeq_debug_clone() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system = new_system(py, number_spins, number_bosons, number_fermions);
        let mut new_sys = system
            .call_method1(
                "system_add_operator_product",
                (
                    "S0Z:Bc0a1:Fc0a0:",
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        new_sys = new_sys
            .call_method1(
                "noise_add_operator_product",
                (
                    ("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        let system_wrapper = new_sys.extract::<MixedLindbladOpenSystemWrapper>().unwrap();

        // Clone
        assert_eq!(system_wrapper.clone(), system_wrapper);

        // Debug
        assert_eq!(
            format!("{:?}", MixedLindbladOpenSystemWrapper::new(vec![None], vec![None], vec![None])),
            "MixedLindbladOpenSystemWrapper { internal: MixedLindbladOpenSystem { system: MixedHamiltonianSystem { number_spins: [None], number_bosons: [None], number_fermions: [None], hamiltonian: MixedHamiltonian { internal_map: {}, n_spins: 1, n_bosons: 1, n_fermions: 1 } }, noise: MixedLindbladNoiseSystem { number_spins: [None], number_bosons: [None], number_fermions: [None], operator: MixedLindbladNoiseOperator { internal_map: {}, n_spins: 1, n_bosons: 1, n_fermions: 1 } } } }"
        );

        // Number of modes

        let number_system = system.call_method0("number_spins").unwrap();
        let current_system = system.call_method0("current_number_spins").unwrap();
        let comparison = bool::extract_bound(
            &number_system
                .call_method1("__eq__", (vec![1_u64],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        let comparison = bool::extract_bound(
            &current_system
                .call_method1("__eq__", (vec![1_u64],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let number_system = system.call_method0("number_bosonic_modes").unwrap();
        let current_system = system.call_method0("current_number_bosonic_modes").unwrap();
        let comparison = bool::extract_bound(
            &number_system
                .call_method1("__eq__", (vec![2_u64],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        let comparison = bool::extract_bound(
            &current_system
                .call_method1("__eq__", (vec![2_u64],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let number_system = system.call_method0("number_fermionic_modes").unwrap();
        let current_system = system
            .call_method0("current_number_fermionic_modes")
            .unwrap();
        let comparison = bool::extract_bound(
            &number_system
                .call_method1("__eq__", (vec![1_u64],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        let comparison = bool::extract_bound(
            &current_system
                .call_method1("__eq__", (vec![1_u64],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // System
        let comp_op = new_sys.call_method0("system").unwrap();
        let system_type = py.get_type_bound::<MixedHamiltonianSystemWrapper>();
        let number_modes: Option<usize> = None;
        let mixed_system = system_type
            .call1((vec![number_modes], vec![number_modes], vec![number_modes]))
            .unwrap();
        mixed_system
            .downcast::<MixedHamiltonianSystemWrapper>()
            .unwrap()
            .call_method1(
                "add_operator_product",
                (
                    "S0Z:Bc0a1:Fc0a0:",
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (mixed_system,)).unwrap()).unwrap();
        assert!(comparison);

        // Noise
        let comp_op = new_sys.call_method0("noise").unwrap();
        let noise_type = py.get_type_bound::<MixedLindbladNoiseSystemWrapper>();
        let noise = noise_type.call0().unwrap();
        noise
            .downcast::<MixedLindbladNoiseSystemWrapper>()
            .unwrap()
            .call_method1(
                "add_operator_product",
                (
                    ("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (noise,)).unwrap()).unwrap();
        assert!(comparison);

        // Ungroup + group
        let comp_op_ungroup = new_sys.call_method0("ungroup").unwrap();

        let noise_type = py.get_type_bound::<MixedLindbladNoiseSystemWrapper>();
        let noise = noise_type.call0().unwrap();
        noise
            .downcast::<MixedLindbladNoiseSystemWrapper>()
            .unwrap()
            .call_method1(
                "add_operator_product",
                (
                    ("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();

        let system_type = py.get_type_bound::<MixedHamiltonianSystemWrapper>();
        let number_modes: Option<usize> = None;
        let mixed_system = system_type
            .call1((vec![number_modes], vec![number_modes], vec![number_modes]))
            .unwrap();
        mixed_system
            .downcast::<MixedHamiltonianSystemWrapper>()
            .unwrap()
            .call_method1(
                "add_operator_product",
                (
                    "S0Z:Bc0a1:Fc0a0:",
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();

        let comparison = bool::extract_bound(
            &comp_op_ungroup
                .call_method1("__eq__", ((&system, &noise),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let comp_op_group = new_system(py, number_spins, number_bosons, number_fermions)
            .call_method1("group", (system, noise))
            .unwrap();
        let comparison =
            bool::extract_bound(&comp_op_group.call_method1("__eq__", (new_sys,)).unwrap())
                .unwrap();
        assert!(comparison);
    })
}

/// Test set_pauli_product and get_pauli_product functions of MixedLindbladOpenSystem
#[test]
fn test_set_pauli_get_pauli() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type_bound::<MixedLindbladOpenSystemWrapper>();
        let new_system_1 = new_system.call0().unwrap();
        let mut system = new_system_1
            .downcast::<MixedLindbladOpenSystemWrapper>()
            .unwrap()
            .call_method1(
                "system_set",
                (
                    "S0Z:Bc0a1:Fc0a0:",
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        system = system
            .call_method1(
                "system_set",
                (
                    "S0Z:Bc0a1:Fc0a2:",
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.2)),
                ),
            )
            .unwrap();
        system = system
            .call_method1(
                "system_set",
                (
                    "S0Z:Bc3a3:Fc0a2:",
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),
                ),
            )
            .unwrap();

        // test access at index 0
        let comp_op = system
            .call_method1("system_get", ("S0Z:Bc0a1:Fc0a0:",))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system
            .call_method1("system_get", ("S0Z:Bc0a1:Fc0a2:",))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.2)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system
            .call_method1("system_get", ("S0Z:Bc3a3:Fc0a2:",))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system
            .call_method1("system_get", ("S0Z:Bc2a2:Fc0a1:",))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.0)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Try_set error 1: Key (MixedProduct) cannot be converted from string
        let error = system.call_method1(
            "system_set",
            ("j1", convert_cf_to_pyobject(py, CalculatorFloat::from(0.5))),
        );
        assert!(error.is_err());

        // Try_set error 2: Value cannot be converted to CalculatorFloat
        let error = system.call_method1("system_set", ("S0Z:Bc0a1:Fc0a2:", vec![0.0]));
        assert!(error.is_err());

        // // Try_set error 3: Generic error
        // let error = system.call_method1("system_set", ("j1", 0.5));
        // assert!(error.is_err());
    });
}

/// Test set_noise and get_noise functions of MixedLindbladOpenSystem
#[test]
fn test_set_noise_get_noise() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type_bound::<MixedLindbladOpenSystemWrapper>();
        let system = new_system.call0().unwrap();
        system
            .downcast::<MixedLindbladOpenSystemWrapper>()
            .unwrap()
            .call_method1(
                "noise_set",
                (
                    ("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a2:"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        system
            .call_method1(
                "noise_set",
                (
                    ("S0Z:Bc0a1:Fc0a2:", "S0Z:Bc3a3:Fc0a2:"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.2)),
                ),
            )
            .unwrap();
        system
            .call_method1(
                "noise_set",
                (
                    ("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),
                ),
            )
            .unwrap();

        // test access at index 0
        let comp_op = system
            .call_method1("noise_get", (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a2:"),))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system
            .call_method1("noise_get", (("S0Z:Bc0a1:Fc0a2:", "S0Z:Bc3a3:Fc0a2:"),))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.2)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system
            .call_method1("noise_get", (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"),))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system
            .call_method1("noise_get", (("S0X:Bc2a2:Fc0a1a3:", "S0X:Bc2a2:Fc0a1a3:"),))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.0)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Get error 1a: Key left (MixedProduct) cannot be converted from string
        let error = system.call_method1("noise_get", (("1+c0a1", "S0Z:Bc0a1:Fc0a2:"),));
        assert!(error.is_err()); // same error as in ADP - Ask!

        // Get error 1b: Key right (MixedProduct) cannot be converted from string
        let error = system.call_method1("noise_get", (("S0Z:Bc0a1:Fc0a2:", "1+c0a1"),));
        assert!(error.is_err()); // same error as in ADP - Ask!

        // Try_set error 1a: Key left (MixedProduct) cannot be converted from string
        let error = system.call_method1(
            "set",
            (
                ("1+c0a1", "S0Z:Bc0a1:Fc0a2:"),
                convert_cf_to_pyobject(py, CalculatorFloat::from(0.5)),
            ),
        );
        assert!(error.is_err()); // same error as in ADP - Ask!

        // Try_set error 1b: Key right (MixedProduct) cannot be converted from string
        let error = system.call_method1(
            "set",
            (
                ("S0Z:Bc0a1:Fc0a2:", "1+c0a1"),
                convert_cf_to_pyobject(py, CalculatorFloat::from(0.5)),
            ),
        );
        assert!(error.is_err()); // same error as in ADP - Ask!

        // Try_set error 2: Value cannot be converted to CalculatorFloat
        let error =
            system.call_method1("set", (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"), vec![0.0]));
        assert!(error.is_err());

        // // Try_set error 3: Generic error
        // let error = system.call_method1("set", ("j1", 0.5));
        // assert!(error.is_err());
    });
}

/// Test try_set_pauli_product and get_pauli_product functions of MixedLindbladOpenSystem
#[test]
fn test_try_set_pauli_get_pauli() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type_bound::<MixedLindbladOpenSystemWrapper>();
        let new_system_1 = new_system.call0().unwrap();
        let mut system = new_system_1
            .downcast::<MixedLindbladOpenSystemWrapper>()
            .unwrap()
            .call_method1(
                "system_set",
                (
                    "S0Z:Bc0a1:Fc0a0:",
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        system = system
            .call_method1(
                "system_set",
                (
                    "S0Z:Bc0a1:Fc0a2:",
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.2)),
                ),
            )
            .unwrap();
        system = system
            .call_method1(
                "system_set",
                (
                    "S0Z:Bc3a3:Fc0a2:",
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),
                ),
            )
            .unwrap();

        // test access at index 0
        let comp_op = system
            .call_method1("system_get", ("S0Z:Bc0a1:Fc0a0:",))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system
            .call_method1("system_get", ("S0Z:Bc0a1:Fc0a2:",))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.2)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system
            .call_method1("system_get", ("S0Z:Bc3a3:Fc0a2:",))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system
            .call_method1("system_get", ("S0Z:Bc2a2:Fc0a1:",))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.0)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Try_set error 1: Key (MixedProduct) cannot be converted from string
        let error = system.call_method1(
            "set",
            ("j1", convert_cf_to_pyobject(py, CalculatorFloat::from(0.5))),
        );
        assert!(error.is_err());

        // Try_set error 2: Value cannot be converted to CalculatorFloat
        let error = system.call_method1("set", ("S0Z:Bc0a1:Fc0a2:", vec![0.0]));
        assert!(error.is_err());
    });
}

/// Test try_set_noise and get_noise functions of MixedLindbladOpenSystem
#[test]
fn test_try_set_noise_get_noise() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type_bound::<MixedLindbladOpenSystemWrapper>();
        let new_system_1 = new_system.call0().unwrap();
        let mut system = new_system_1
            .downcast::<MixedLindbladOpenSystemWrapper>()
            .unwrap()
            .call_method1(
                "noise_set",
                (
                    ("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a2:"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        system = system
            .call_method1(
                "noise_set",
                (
                    ("S0Z:Bc0a1:Fc0a2:", "S0Z:Bc3a3:Fc0a2:"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.2)),
                ),
            )
            .unwrap();
        system = system
            .call_method1(
                "noise_set",
                (
                    ("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),
                ),
            )
            .unwrap();

        // test access at index 0
        let comp_op = system
            .call_method1("noise_get", (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a2:"),))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system
            .call_method1("noise_get", (("S0Z:Bc0a1:Fc0a2:", "S0Z:Bc3a3:Fc0a2:"),))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.2)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system
            .call_method1("noise_get", (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"),))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system
            .call_method1("noise_get", (("S0X:Bc2a2:Fc0a1a3:", "S0X:Bc2a2:Fc0a1a3:"),))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.0)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Try_set error 1a: Key left (MixedProduct) cannot be converted from string
        let error = system.call_method1(
            "noise_set",
            (
                ("1+c0a1", "S0Z:Bc0a1:Fc0a2:"),
                convert_cf_to_pyobject(py, CalculatorFloat::from(0.5)),
            ),
        );
        assert!(error.is_err()); // same error as in ADP - Ask!

        // Try_set error 1b: Key right (MixedProduct) cannot be converted from string
        let error = system.call_method1(
            "noise_set",
            (
                ("S0Z:Bc0a1:Fc0a2:", "1+c0a1"),
                convert_cf_to_pyobject(py, CalculatorFloat::from(0.5)),
            ),
        );
        assert!(error.is_err()); // same error as in ADP - Ask!

        // Try_set error 2: Value cannot be converted to CalculatorFloat
        let error = system.call_method1(
            "noise_set",
            (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"), vec![0.0]),
        );
        assert!(error.is_err());
    });
}

/// Test add_pauli_product and get_pauli_product functions of MixedLindbladOpenSystem
#[test]
fn test_add_pauli_get_pauli() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type_bound::<MixedLindbladOpenSystemWrapper>();
        let new_system_1 = new_system.call0().unwrap();
        let mut system = new_system_1
            .downcast::<MixedLindbladOpenSystemWrapper>()
            .unwrap()
            .call_method1(
                "system_add_operator_product",
                (
                    "S0Z:Bc0a1:Fc0a0:",
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        system = system
            .call_method1(
                "system_add_operator_product",
                (
                    "S0Z:Bc0a1:Fc0a2:",
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.2)),
                ),
            )
            .unwrap();
        system = system
            .call_method1(
                "system_add_operator_product",
                (
                    "S0Z:Bc3a3:Fc0a2:",
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),
                ),
            )
            .unwrap();

        // test access at index 0
        let comp_op = system
            .call_method1("system_get", ("S0Z:Bc0a1:Fc0a0:",))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system
            .call_method1("system_get", ("S0Z:Bc0a1:Fc0a2:",))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.2)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system
            .call_method1("system_get", ("S0Z:Bc3a3:Fc0a2:",))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system
            .call_method1("system_get", ("S0Z:Bc2a2:Fc0a1:",))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.0)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Get error
        let error = system.call_method1("system_get", ("j2",));
        assert!(error.is_err());

        // Try_set error 1: Key (MixedProduct) cannot be converted from string
        let error = system.call_method1(
            "system_add_operator_product",
            ("j1", convert_cf_to_pyobject(py, CalculatorFloat::from(0.5))),
        );
        assert!(error.is_err());

        // Try_set error 2: Value cannot be converted to CalculatorFloat
        let error = system.call_method1(
            "system_add_operator_product",
            ("S0Z:Bc0a1:Fc0a2:", vec![0.0]),
        );
        assert!(error.is_err());

        // // Try_set error 3: Generic error
        // let error = system.call_method1("system_add_operator_product", ("j1", 0.5));
        // assert!(error.is_err());
    });
}

/// Test add_noise and get_noise functions of MixedLindbladOpenSystem
#[test]
fn test_add_noise_get_noise() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type_bound::<MixedLindbladOpenSystemWrapper>();
        let new_system_1 = new_system.call0().unwrap();
        let mut system = new_system_1
            .downcast::<MixedLindbladOpenSystemWrapper>()
            .unwrap()
            .call_method1(
                "noise_add_operator_product",
                (
                    ("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a2:"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        system = system
            .call_method1(
                "noise_add_operator_product",
                (
                    ("S0Z:Bc0a1:Fc0a2:", "S0Z:Bc3a3:Fc0a2:"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.2)),
                ),
            )
            .unwrap();
        system = system
            .call_method1(
                "noise_add_operator_product",
                (
                    ("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),
                ),
            )
            .unwrap();

        // test access at index 0
        let comp_op = system
            .call_method1("noise_get", (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a2:"),))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system
            .call_method1("noise_get", (("S0Z:Bc0a1:Fc0a2:", "S0Z:Bc3a3:Fc0a2:"),))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.2)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system
            .call_method1("noise_get", (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"),))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system
            .call_method1("noise_get", (("S0X:Bc2a2:Fc0a1a3:", "S0X:Bc2a2:Fc0a1a3:"),))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.0)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Try_set error 1a: Key left (MixedProduct) cannot be converted from string
        let error = system.call_method1(
            "noise_add_operator_product",
            (
                ("1+c0a1", "S0Z:Bc0a1:Fc0a2:"),
                convert_cf_to_pyobject(py, CalculatorFloat::from(0.5)),
            ),
        );
        assert!(error.is_err()); // same error as in ADP - Ask!

        // Try_set error 1b: Key right (MixedProduct) cannot be converted from string
        let error = system.call_method1(
            "noise_add_operator_product",
            (
                ("S0Z:Bc0a1:Fc0a2:", "1+c0a1"),
                convert_cf_to_pyobject(py, CalculatorFloat::from(0.5)),
            ),
        );
        assert!(error.is_err()); // same error as in ADP - Ask!

        // Try_set error 2: Value cannot be converted to CalculatorFloat
        let error = system.call_method1(
            "noise_add_operator_product",
            (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"), vec![0.0]),
        );
        assert!(error.is_err());

        // // Try_set error 3: Generic error
        // let error = system.call_method1("noise_add_operator_product", ("j1", 0.5));
        // assert!(error.is_err());
    });
}

#[test_case(1.0,0.0;"real")]
#[test_case(0.0,1.0;"imag")]
#[test_case(0.7,0.7;"mixed")]
fn test_truncate(re: f64, im: f64) {
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let system = new_system(py, number_spins, number_bosons, number_fermions);
        system
            .call_method1(
                "noise_add_operator_product",
                (
                    ("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"),
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::new(100.0 * re, 100.0 * im),
                    },
                ),
            )
            .unwrap();
        system
            .call_method1(
                "system_add_operator_product",
                (
                    "S0X:Bc0a0:Fc0a1:",
                    CalculatorFloatWrapper {
                        internal: CalculatorFloat::from(10.0 * re),
                    },
                ),
            )
            .unwrap();
        system
            .call_method1(
                "noise_add_operator_product",
                (
                    ("S0Z:Bc0a1:Fc0a0:", "S0X:Bc2a2:Fc0a1a3:"),
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::new(re, im),
                    },
                ),
            )
            .unwrap();
        system
            .call_method1(
                "system_add_operator_product",
                (
                    "S0X:Bc0a0:Fc0a3:",
                    CalculatorFloatWrapper {
                        internal: CalculatorFloat::from("test"),
                    },
                ),
            )
            .unwrap();

        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let test_system1 = new_system(py, number_spins, number_bosons, number_fermions);
        test_system1
            .call_method1(
                "noise_add_operator_product",
                (
                    ("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"),
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::new(100.0 * re, 100.0 * im),
                    },
                ),
            )
            .unwrap();
        test_system1
            .call_method1(
                "system_add_operator_product",
                (
                    "S0X:Bc0a0:Fc0a1:",
                    CalculatorFloatWrapper {
                        internal: CalculatorFloat::from(10.0 * re),
                    },
                ),
            )
            .unwrap();
        test_system1
            .call_method1(
                "system_add_operator_product",
                (
                    "S0X:Bc0a0:Fc0a3:",
                    CalculatorFloatWrapper {
                        internal: CalculatorFloat::from("test"),
                    },
                ),
            )
            .unwrap();

        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let test_system2 = new_system(py, number_spins, number_bosons, number_fermions);
        test_system2
            .call_method1(
                "noise_add_operator_product",
                (
                    ("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"),
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::new(100.0 * re, 100.0 * im),
                    },
                ),
            )
            .unwrap();
        test_system2
            .call_method1(
                "system_add_operator_product",
                (
                    "S0X:Bc0a0:Fc0a3:",
                    CalculatorFloatWrapper {
                        internal: CalculatorFloat::from("test"),
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

/// Test copy and deepcopy functions of MixedLindbladOpenSystem
#[test]
fn test_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let new_system = new_system(py, number_spins, number_bosons, number_fermions);
        let mut system = new_system
            .call_method1(
                "system_add_operator_product",
                (
                    "S0Z:Bc0a1:Fc0a0:",
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        system = system
            .call_method1(
                "noise_add_operator_product",
                (
                    ("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();

        let copy_system = system.call_method0("__copy__").unwrap();
        let deepcopy_system = system.call_method1("__deepcopy__", ("",)).unwrap();
        // let copy_deepcopy_param: &PyAny = system.clone();

        let comparison_copy =
            bool::extract_bound(&copy_system.call_method1("__eq__", (&system,)).unwrap()).unwrap();
        assert!(comparison_copy);
        let comparison_deepcopy =
            bool::extract_bound(&deepcopy_system.call_method1("__eq__", (system,)).unwrap())
                .unwrap();
        assert!(comparison_deepcopy);
    });
}

/// Test to_bincode and from_bincode functions of MixedLindbladOpenSystem
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let new_system_1 = new_system(py, number_spins, number_bosons, number_fermions);
        let mut system = new_system_1
            .call_method1(
                "system_add_operator_product",
                (
                    "S0Z:Bc0a1:Fc0a0:",
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        system = system
            .call_method1(
                "noise_add_operator_product",
                (
                    ("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();

        let serialised = system.call_method0("to_bincode").unwrap();
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
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
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let new = new_system(py, number_spins, number_bosons, number_fermions);
        let deserialised_error = new.call_method1("from_bincode", ("j",));
        assert!(deserialised_error.is_err());
    });
}

/// Test to_ and from_json functions of MixedLindbladOpenSystem
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let new_system_1 = new_system(py, number_spins, number_bosons, number_fermions);
        let mut system = new_system_1
            .call_method1(
                "system_add_operator_product",
                (
                    "S0Z:Bc0a1:Fc0a0:",
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        system = system
            .call_method1(
                "noise_add_operator_product",
                (
                    ("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();

        let serialised = system.call_method0("to_json").unwrap();
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
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
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let new_system = new_system(py, number_spins, number_bosons, number_fermions);
        let mut system = new_system
            .call_method1(
                "system_add_operator_product",
                (
                    "S0Z:Bc0a1:Fc0a0:",
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        system = system
            .call_method1(
                "noise_add_operator_product",
                (
                    ("S0X:Bc0a0:Fc0a1:", "S0X:Bc0a0:Fc0a1:"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        let mut rust_system = MixedLindbladOpenSystem::group(
            MixedHamiltonianSystem::new(vec![None], vec![None], vec![None]),
            MixedLindbladNoiseSystem::new(vec![None], vec![None], vec![None]),
        )
        .unwrap();
        rust_system
            .system_mut()
            .add_operator_product(
                HermitianMixedProduct::new(
                    [PauliProduct::new().z(0)],
                    [BosonProduct::new([0], [1]).unwrap()],
                    [FermionProduct::new([0], [0]).unwrap()],
                )
                .unwrap(),
                CalculatorComplex::from(0.1),
            )
            .unwrap();
        let _ = rust_system.noise_mut().add_operator_product(
            (
                MixedDecoherenceProduct::new(
                    [DecoherenceProduct::new().x(0)],
                    [BosonProduct::new([0], [0]).unwrap()],
                    [FermionProduct::new([0], [1]).unwrap()],
                )
                .unwrap(),
                MixedDecoherenceProduct::new(
                    [DecoherenceProduct::new().x(0)],
                    [BosonProduct::new([0], [0]).unwrap()],
                    [FermionProduct::new([0], [1]).unwrap()],
                )
                .unwrap(),
            ),
            CalculatorComplex::from(0.1),
        );

        let test_string =
        "MixedLindbladOpenSystem{\nSystem: {\nMixedHamiltonianSystem(\nnumber_spins: 1, \nnumber_bosons: 2, \nnumber_fermions: 1, )\n{S0Z:Bc0a1:Fc0a0:: (1e-1 + i * 0e0),\n}}\nNoise: {\nMixedLindbladNoiseSystem(\nnumber_spins: 1, \nnumber_bosons: 1, \nnumber_fermions: 2, )\n{(S0X:Bc0a0:Fc0a1:, S0X:Bc0a0:Fc0a1:): (1e-1 + i * 0e0),\n}}\n}"
            .to_string();

        let to_format = system.call_method1("__format__", ("",)).unwrap();
        let format_op: String = String::extract_bound(&to_format).unwrap();
        assert_eq!(format_op, test_string);

        let to_repr = system.call_method0("__repr__").unwrap();
        let repr_op: String = String::extract_bound(&to_repr).unwrap();
        assert_eq!(repr_op, test_string);

        let to_str = system.call_method0("__str__").unwrap();
        let str_op: String = String::extract_bound(&to_str).unwrap();
        assert_eq!(str_op, test_string);
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
        let new_system_1 = new_system(py, number_spins, number_bosons, number_fermions);
        let mut system_one = new_system_1
            .call_method1(
                "system_add_operator_product",
                (
                    "S0Z:Bc0a1:Fc0a0:",
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        system_one = system_one
            .call_method1(
                "noise_add_operator_product",
                (
                    ("S0Z:Bc0a1:Fc0a1:", "S0X:Bc0a0:Fc0a1:"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        let number_spins: Vec<Option<usize>> = vec![None];
        let number_bosons: Vec<Option<usize>> = vec![None];
        let number_fermions: Vec<Option<usize>> = vec![None];
        let new_system_1 = new_system(py, number_spins, number_bosons, number_fermions);
        let mut system_two = new_system_1
            .call_method1(
                "system_add_operator_product",
                (
                    "S0Z:Bc0a1:Fc0a1:",
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        system_two = system_two
            .call_method1(
                "noise_add_operator_product",
                (
                    ("S0Z:Bc0a1:Fc0a0:", "S0X:Bc0a0:Fc0a1:"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();

        let comparison =
            bool::extract_bound(&system_one.call_method1("__eq__", (&system_two,)).unwrap())
                .unwrap();
        assert!(!comparison);
        let comparison = bool::extract_bound(
            &system_one
                .call_method1("__eq__", ("S0Z:Bc0a1:Fc0a0:",))
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
                .call_method1("__ne__", ("S0Z:Bc0a1:Fc0a0:",))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let comparison = system_one.call_method1("__ge__", ("S0Z:Bc0a1:Fc0a0:",));
        assert!(comparison.is_err());
    });
}

#[cfg(feature = "json_schema")]
#[test]
fn test_json_schema() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new = new_system(py, vec![None], vec![None], vec![None]);

        let schema: String =
            String::extract_bound(&new.call_method0("json_schema").unwrap()).unwrap();
        let rust_schema =
            serde_json::to_string_pretty(&schemars::schema_for!(MixedLindbladOpenSystem)).unwrap();
        assert_eq!(schema, rust_schema);

        let version: String =
            String::extract_bound(&new.call_method0("current_version").unwrap()).unwrap();
        let rust_version = STRUQTURE_VERSION.to_string();
        assert_eq!(version, rust_version);

        new.call_method1(
            "noise_add_operator_product",
            (("S0Z:Bc0a1:Fc0a0:", "S0Z:Bc0a1:Fc0a0:"), 1.0),
        )
        .unwrap();
        let min_version: String =
            String::extract_bound(&new.call_method0("min_supported_version").unwrap()).unwrap();
        let rust_min_version = String::from("1.0.0");
        assert_eq!(min_version, rust_min_version);
    });
}
