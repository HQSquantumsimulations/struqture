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

use pyo3::prelude::*;
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use qoqo_calculator_pyo3::{CalculatorComplexWrapper, CalculatorFloatWrapper};
use struqture::spins::{
    DecoherenceProduct, PauliProduct, SpinHamiltonian, SpinLindbladNoiseOperator,
    SpinLindbladOpenSystem,
};
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture::{OpenSystem, OperateOnDensityMatrix, SpinIndex};
use struqture_py::spins::{
    SpinHamiltonianWrapper, SpinLindbladNoiseOperatorWrapper, SpinLindbladOpenSystemWrapper,
};
use test_case::test_case;

// helper functions
fn new_system(py: Python) -> &PyCell<SpinLindbladOpenSystemWrapper> {
    let system_type = py.get_type::<SpinLindbladOpenSystemWrapper>();
    system_type
        .call0()
        .unwrap()
        .downcast::<PyCell<SpinLindbladOpenSystemWrapper>>()
        .unwrap()
}

// helper function to convert CalculatorFloat into a python object
fn convert_cf_to_pyobject(
    py: Python,
    parameter: CalculatorFloat,
) -> &PyCell<CalculatorFloatWrapper> {
    let parameter_type = py.get_type::<CalculatorFloatWrapper>();
    match parameter {
        CalculatorFloat::Float(x) => parameter_type
            .call1((x,))
            .unwrap()
            .downcast::<PyCell<CalculatorFloatWrapper>>()
            .unwrap(),
        CalculatorFloat::Str(x) => parameter_type
            .call1((x,))
            .unwrap()
            .downcast::<PyCell<CalculatorFloatWrapper>>()
            .unwrap(),
    }
}

/// Test number_spins function of SpinSystem
#[test]
fn test_number_spins_current() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_system(py);
        system
            .call_method1("noise_add_operator_product", (("0X", "0X"), 0.1))
            .unwrap();

        let number_system = system.call_method0("number_spins").unwrap();

        let comparison =
            bool::extract(number_system.call_method1("__eq__", (1_u64,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test empty_clone function of SpinSystem
#[test]
fn test_empty_clone() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_system(py);
        let none_system = system.call_method0("empty_clone").unwrap();
        let comparison =
            bool::extract(none_system.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison);

        let system = new_system(py);
        let some_system = system.call_method0("empty_clone").unwrap();
        let comparison =
            bool::extract(some_system.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add_operator_product and remove functions of SpinSystem
#[test]
fn spin_system_test_add_operator_product_remove_system() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type::<SpinLindbladOpenSystemWrapper>();
        let system = new_system
            .call0()
            .unwrap()
            .downcast::<PyCell<SpinLindbladOpenSystemWrapper>>()
            .unwrap();
        system
            .call_method1("system_add_operator_product", ("0X", 0.1))
            .unwrap();
        system
            .call_method1("system_add_operator_product", ("1Z", 0.2))
            .unwrap();
        system
            .call_method1("system_add_operator_product", ("3Y", 0.05))
            .unwrap();

        // test access at index 0
        let comp_op = system.call_method1("system_get", ("0X",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.1,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system.call_method1("system_get", ("1Z",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.2,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system.call_method1("system_get", ("3Y",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.05,)).unwrap()).unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system.call_method1("system_get", ("2X",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
        assert!(comparison);

        // Get error
        let error = system.call_method1("system_get", ("2J",));
        assert!(error.is_err());

        // Try_set error 1: Key (PauliProduct) cannot be converted from string
        let error = system.call_method1("system_add_operator_product", ("1J", 0.5));
        assert!(error.is_err());

        // Try_set error 2: Value cannot be converted to CalculatorComplex
        let error = system.call_method1("system_add_operator_product", ("1Z", vec![0.0]));
        assert!(error.is_err());

        // Try_set error 3: Generic error
        let error = system.call_method1("system_add_operator_product", ("1J", 0.5));
        assert!(error.is_err());
    });
}

/// Test add_operator_product and remove functions of SpinSystem
#[test]
fn spin_system_test_add_operator_product_remove_noise() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type::<SpinLindbladOpenSystemWrapper>();
        let system = new_system
            .call0()
            .unwrap()
            .downcast::<PyCell<SpinLindbladOpenSystemWrapper>>()
            .unwrap();
        system
            .call_method1("noise_add_operator_product", (("0X", "0X"), 0.1))
            .unwrap();
        system
            .call_method1("noise_add_operator_product", (("0X", "1Z"), 0.2))
            .unwrap();
        system
            .call_method1("noise_add_operator_product", (("0X", "3iY"), 0.05))
            .unwrap();

        // test access at index 0
        let comp_op = system.call_method1("noise_get", (("0X", "0X"),)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.1,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system.call_method1("noise_get", (("0X", "1Z"),)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.2,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system.call_method1("noise_get", (("0X", "3iY"),)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.05,)).unwrap()).unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system.call_method1("noise_get", (("0X", "2X"),)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
        assert!(comparison);

        // Get error
        let error = system.call_method1("noise_get", (("2J", "0X"),));
        assert!(error.is_err());

        // Try_set error 1: Key (PauliProduct) cannot be converted from string
        let error = system.call_method1("noise_add_operator_product", (("1J", "0X"), 0.5));
        assert!(error.is_err());

        // Try_set error 2: Value cannot be converted to CalculatorComplex
        let error = system.call_method1("noise_add_operator_product", (("0X", "1Z"), vec![0.0]));
        assert!(error.is_err());

        // Try_set error 3: Generic error
        let error = system.call_method1("noise_add_operator_product", (("0X", "1J"), 0.5));
        assert!(error.is_err());
    });
}

/// Test add magic method function of SpinSystem
#[test]
fn test_neg() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_system(py);
        system_0
            .call_method1("noise_add_operator_product", (("0X", "0X"), 0.1))
            .unwrap();
        let system_1 = new_system(py);
        system_1
            .call_method1("noise_add_operator_product", (("0X", "0X"), -0.1))
            .unwrap();

        let negated = system_0.call_method0("__neg__").unwrap();
        let comparison =
            bool::extract(negated.call_method1("__eq__", (system_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of SpinSystem
#[test]
fn test_add() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_system(py);
        system_0
            .call_method1("noise_add_operator_product", (("0X", "0X"), 0.1))
            .unwrap();
        let system_1 = new_system(py);
        system_1
            .call_method1("noise_add_operator_product", (("1Z", "0X"), 0.2))
            .unwrap();
        let system_0_1 = new_system(py);
        system_0_1
            .call_method1("noise_add_operator_product", (("0X", "0X"), 0.1))
            .unwrap();
        system_0_1
            .call_method1("noise_add_operator_product", (("1Z", "0X"), 0.2))
            .unwrap();

        let added = system_0.call_method1("__add__", (system_1,)).unwrap();
        let comparison =
            bool::extract(added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of SpinSystem
#[test]
fn test_sub() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_system(py);
        system_0
            .call_method1("noise_add_operator_product", (("0X", "0X"), 0.1))
            .unwrap();
        let system_1 = new_system(py);
        system_1
            .call_method1("noise_add_operator_product", (("1Z", "0X"), 0.2))
            .unwrap();
        let system_0_1 = new_system(py);
        system_0_1
            .call_method1("noise_add_operator_product", (("0X", "0X"), 0.1))
            .unwrap();
        system_0_1
            .call_method1("noise_add_operator_product", (("1Z", "0X"), -0.2))
            .unwrap();

        let added = system_0.call_method1("__sub__", (system_1,)).unwrap();
        let comparison =
            bool::extract(added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of SpinSystem
#[test]
fn test_mul_cf() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_system(py);
        system_0
            .call_method1("noise_add_operator_product", (("0X", "0X"), 0.1_f64))
            .unwrap();

        let system_0_1 = new_system(py);
        system_0_1
            .call_method1("noise_add_operator_product", (("0X", "0X"), 0.2))
            .unwrap();

        let added = system_0.call_method1("__mul__", (2.0,)).unwrap();
        let comparison =
            bool::extract(added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test default function of SpinLindbladOpenSystemWrapper
#[test]
fn test_default_partialeq_debug_clone() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let system = new_system(py);
        let mut new_sys = system
            .call_method1(
                "system_add_operator_product",
                ("0X", convert_cf_to_pyobject(py, CalculatorFloat::from(0.1))),
            )
            .unwrap();
        new_sys = new_sys
            .call_method1(
                "noise_add_operator_product",
                (
                    ("0X", "0X"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        let system_wrapper = new_sys.extract::<SpinLindbladOpenSystemWrapper>().unwrap();

        // PartialEq
        let helper_ne: bool = SpinLindbladOpenSystemWrapper::default() != system_wrapper;
        assert!(helper_ne);
        let helper_eq: bool =
            SpinLindbladOpenSystemWrapper::default() == SpinLindbladOpenSystemWrapper::new();
        assert!(helper_eq);

        // Clone
        assert_eq!(system_wrapper.clone(), system_wrapper);

        // Debug
        assert_eq!(
            format!("{:?}", SpinLindbladOpenSystemWrapper::new()),
            "SpinLindbladOpenSystemWrapper { internal: SpinLindbladOpenSystem { system: SpinHamiltonian { internal_map: {} }, noise: SpinLindbladNoiseOperator { internal_map: {} } } }"
        );

        // Number of spins
        let comp_op = new_sys.call_method0("number_spins").unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (1,)).unwrap()).unwrap();
        assert!(comparison);

        // System
        let comp_op = new_sys.call_method0("system").unwrap();
        let system_type = py.get_type::<SpinHamiltonianWrapper>();
        let spin_system = system_type
            .call0()
            .unwrap()
            .downcast::<PyCell<SpinHamiltonianWrapper>>()
            .unwrap();
        spin_system
            .call_method1(
                "add_operator_product",
                ("0X", convert_cf_to_pyobject(py, CalculatorFloat::from(0.1))),
            )
            .unwrap();
        let comparison =
            bool::extract(comp_op.call_method1("__eq__", (spin_system,)).unwrap()).unwrap();
        assert!(comparison);

        // Noise
        let comp_op = new_sys.call_method0("noise").unwrap();
        let noise_type = py.get_type::<SpinLindbladNoiseOperatorWrapper>();
        let noise = noise_type
            .call0()
            .unwrap()
            .downcast::<PyCell<SpinLindbladNoiseOperatorWrapper>>()
            .unwrap();
        noise
            .call_method1(
                "add_operator_product",
                (
                    ("0X", "0X"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (noise,)).unwrap()).unwrap();
        assert!(comparison);

        // Ungroup + group
        let comp_op_ungroup = new_sys.call_method0("ungroup").unwrap();

        let noise_type = py.get_type::<SpinLindbladNoiseOperatorWrapper>();
        let noise = noise_type
            .call0()
            .unwrap()
            .downcast::<PyCell<SpinLindbladNoiseOperatorWrapper>>()
            .unwrap();
        noise
            .call_method1(
                "add_operator_product",
                (
                    ("0X", "0X"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();

        let system_type = py.get_type::<SpinHamiltonianWrapper>();
        let spin_system = system_type
            .call0()
            .unwrap()
            .downcast::<PyCell<SpinHamiltonianWrapper>>()
            .unwrap();
        spin_system
            .call_method1(
                "add_operator_product",
                ("0X", convert_cf_to_pyobject(py, CalculatorFloat::from(0.1))),
            )
            .unwrap();

        let comparison = bool::extract(
            comp_op_ungroup
                .call_method1("__eq__", ((spin_system, noise),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        let comp_op_group = new_system(py)
            .call_method1("group", (spin_system, noise))
            .unwrap();
        let comparison =
            bool::extract(comp_op_group.call_method1("__eq__", (new_sys,)).unwrap()).unwrap();
        assert!(comparison);
    })
}

/// Test set_pauli_product and get_pauli_product functions of SpinLindbladOpenSystem
#[test]
fn test_set_pauli_get_pauli() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type::<SpinLindbladOpenSystemWrapper>();
        let new_system_1 = new_system
            .call0()
            .unwrap()
            .downcast::<PyCell<SpinLindbladOpenSystemWrapper>>()
            .unwrap();
        let mut system = new_system_1
            .call_method1(
                "system_set",
                ("0X", convert_cf_to_pyobject(py, CalculatorFloat::from(0.1))),
            )
            .unwrap();
        system = system
            .call_method1(
                "system_set",
                ("1Z", convert_cf_to_pyobject(py, CalculatorFloat::from(0.2))),
            )
            .unwrap();
        system = system
            .call_method1(
                "system_set",
                (
                    "3Y",
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),
                ),
            )
            .unwrap();

        // test access at index 0
        let comp_op = system.call_method1("system_get", ("0X",)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system.call_method1("system_get", ("1Z",)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.2)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system.call_method1("system_get", ("3Y",)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system.call_method1("system_get", ("2X",)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.0)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Try_set error 1: Key (PauliProduct) cannot be converted from string
        let error = system.call_method1(
            "system_set",
            ("1J", convert_cf_to_pyobject(py, CalculatorFloat::from(0.5))),
        );
        assert!(error.is_err());

        // Try_set error 2: Value cannot be converted to CalculatorFloat
        let error = system.call_method1("system_set", ("1Z", vec![0.0]));
        assert!(error.is_err());

        // // Try_set error 3: Generic error
        // let error = system.call_method1("system_set", ("1J", 0.5));
        // assert!(error.is_err());
    });
}

/// Test set_noise and get_noise functions of SpinLindbladOpenSystem
#[test]
fn test_set_noise_get_noise() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type::<SpinLindbladOpenSystemWrapper>();
        let system = new_system
            .call0()
            .unwrap()
            .downcast::<PyCell<SpinLindbladOpenSystemWrapper>>()
            .unwrap();
        system
            .call_method1(
                "noise_set",
                (
                    ("0X", "1Z"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        system
            .call_method1(
                "noise_set",
                (
                    ("1Z", "3iY"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.2)),
                ),
            )
            .unwrap();
        system
            .call_method1(
                "noise_set",
                (
                    ("0X", "0X"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),
                ),
            )
            .unwrap();

        // test access at index 0
        let comp_op = system.call_method1("noise_get", (("0X", "1Z"),)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system.call_method1("noise_get", (("1Z", "3iY"),)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.2)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system.call_method1("noise_get", (("0X", "0X"),)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system.call_method1("noise_get", (("2iY", "2iY"),)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.0)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Get error 1a: Key left (PauliProduct) cannot be converted from string
        let error = system.call_method1("noise_get", (("1+0Z", "1Z"),));
        assert!(error.is_err()); // same error as in ADP - Ask!

        // Get error 1b: Key right (PauliProduct) cannot be converted from string
        let error = system.call_method1("noise_get", (("1Z", "1+0Z"),));
        assert!(error.is_err()); // same error as in ADP - Ask!

        // Try_set error 1a: Key left (PauliProduct) cannot be converted from string
        let error = system.call_method1(
            "set",
            (
                ("1+0Z", "1Z"),
                convert_cf_to_pyobject(py, CalculatorFloat::from(0.5)),
            ),
        );
        assert!(error.is_err()); // same error as in ADP - Ask!

        // Try_set error 1b: Key right (PauliProduct) cannot be converted from string
        let error = system.call_method1(
            "set",
            (
                ("1Z", "1+0Z"),
                convert_cf_to_pyobject(py, CalculatorFloat::from(0.5)),
            ),
        );
        assert!(error.is_err()); // same error as in ADP - Ask!

        // Try_set error 2: Value cannot be converted to CalculatorFloat
        let error = system.call_method1("set", (("0X", "0X"), vec![0.0]));
        assert!(error.is_err());

        // // Try_set error 3: Generic error
        // let error = system.call_method1("set", ("1J", 0.5));
        // assert!(error.is_err());
    });
}

/// Test try_set_pauli_product and get_pauli_product functions of SpinLindbladOpenSystem
#[test]
fn test_try_set_pauli_get_pauli() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type::<SpinLindbladOpenSystemWrapper>();
        let new_system_1 = new_system
            .call0()
            .unwrap()
            .downcast::<PyCell<SpinLindbladOpenSystemWrapper>>()
            .unwrap();
        let mut system = new_system_1
            .call_method1(
                "system_set",
                ("0X", convert_cf_to_pyobject(py, CalculatorFloat::from(0.1))),
            )
            .unwrap();
        system = system
            .call_method1(
                "system_set",
                ("1Z", convert_cf_to_pyobject(py, CalculatorFloat::from(0.2))),
            )
            .unwrap();
        system = system
            .call_method1(
                "system_set",
                (
                    "3Y",
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),
                ),
            )
            .unwrap();

        // test access at index 0
        let comp_op = system.call_method1("system_get", ("0X",)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system.call_method1("system_get", ("1Z",)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.2)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system.call_method1("system_get", ("3Y",)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system.call_method1("system_get", ("2X",)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.0)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Try_set error 1: Key (PauliProduct) cannot be converted from string
        let error = system.call_method1(
            "set",
            ("1J", convert_cf_to_pyobject(py, CalculatorFloat::from(0.5))),
        );
        assert!(error.is_err());

        // Try_set error 2: Value cannot be converted to CalculatorFloat
        let error = system.call_method1("set", ("1Z", vec![0.0]));
        assert!(error.is_err());
    });
}

/// Test try_set_noise and get_noise functions of SpinLindbladOpenSystem
#[test]
fn test_try_set_noise_get_noise() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type::<SpinLindbladOpenSystemWrapper>();
        let new_system_1 = new_system
            .call0()
            .unwrap()
            .downcast::<PyCell<SpinLindbladOpenSystemWrapper>>()
            .unwrap();
        let mut system = new_system_1
            .call_method1(
                "noise_set",
                (
                    ("0X", "1Z"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        system = system
            .call_method1(
                "noise_set",
                (
                    ("1Z", "3iY"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.2)),
                ),
            )
            .unwrap();
        system = system
            .call_method1(
                "noise_set",
                (
                    ("0X", "0X"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),
                ),
            )
            .unwrap();

        // test access at index 0
        let comp_op = system.call_method1("noise_get", (("0X", "1Z"),)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system.call_method1("noise_get", (("1Z", "3iY"),)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.2)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system.call_method1("noise_get", (("0X", "0X"),)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system.call_method1("noise_get", (("2iY", "2iY"),)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.0)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Try_set error 1a: Key left (PauliProduct) cannot be converted from string
        let error = system.call_method1(
            "noise_set",
            (
                ("1+0Z", "1Z"),
                convert_cf_to_pyobject(py, CalculatorFloat::from(0.5)),
            ),
        );
        assert!(error.is_err()); // same error as in ADP - Ask!

        // Try_set error 1b: Key right (PauliProduct) cannot be converted from string
        let error = system.call_method1(
            "noise_set",
            (
                ("1Z", "1+0Z"),
                convert_cf_to_pyobject(py, CalculatorFloat::from(0.5)),
            ),
        );
        assert!(error.is_err()); // same error as in ADP - Ask!

        // Try_set error 2: Value cannot be converted to CalculatorFloat
        let error = system.call_method1("noise_set", (("0X", "0X"), vec![0.0]));
        assert!(error.is_err());
    });
}

/// Test add_pauli_product and get_pauli_product functions of SpinLindbladOpenSystem
#[test]
fn test_add_pauli_get_pauli() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type::<SpinLindbladOpenSystemWrapper>();
        let new_system_1 = new_system
            .call0()
            .unwrap()
            .downcast::<PyCell<SpinLindbladOpenSystemWrapper>>()
            .unwrap();
        let mut system = new_system_1
            .call_method1(
                "system_add_operator_product",
                ("0X", convert_cf_to_pyobject(py, CalculatorFloat::from(0.1))),
            )
            .unwrap();
        system = system
            .call_method1(
                "system_add_operator_product",
                ("1Z", convert_cf_to_pyobject(py, CalculatorFloat::from(0.2))),
            )
            .unwrap();
        system = system
            .call_method1(
                "system_add_operator_product",
                (
                    "3Y",
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),
                ),
            )
            .unwrap();

        // test access at index 0
        let comp_op = system.call_method1("system_get", ("0X",)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system.call_method1("system_get", ("1Z",)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.2)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system.call_method1("system_get", ("3Y",)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system.call_method1("system_get", ("2X",)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.0)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Get error
        let error = system.call_method1("system_get", ("2J",));
        assert!(error.is_err());

        // Try_set error 1: Key (PauliProduct) cannot be converted from string
        let error = system.call_method1(
            "system_add_operator_product",
            ("1J", convert_cf_to_pyobject(py, CalculatorFloat::from(0.5))),
        );
        assert!(error.is_err());

        // Try_set error 2: Value cannot be converted to CalculatorFloat
        let error = system.call_method1("system_add_operator_product", ("1Z", vec![0.0]));
        assert!(error.is_err());

        // // Try_set error 3: Generic error
        // let error = system.call_method1("system_add_operator_product", ("1J", 0.5));
        // assert!(error.is_err());
    });
}

/// Test add_noise and get_noise functions of SpinLindbladOpenSystem
#[test]
fn test_add_noise_get_noise() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type::<SpinLindbladOpenSystemWrapper>();
        let new_system_1 = new_system
            .call0()
            .unwrap()
            .downcast::<PyCell<SpinLindbladOpenSystemWrapper>>()
            .unwrap();
        let mut system = new_system_1
            .call_method1(
                "noise_add_operator_product",
                (
                    ("0X", "1Z"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        system = system
            .call_method1(
                "noise_add_operator_product",
                (
                    ("1Z", "3iY"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.2)),
                ),
            )
            .unwrap();
        system = system
            .call_method1(
                "noise_add_operator_product",
                (
                    ("0X", "0X"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),
                ),
            )
            .unwrap();

        // test access at index 0
        let comp_op = system.call_method1("noise_get", (("0X", "1Z"),)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system.call_method1("noise_get", (("1Z", "3iY"),)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.2)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system.call_method1("noise_get", (("0X", "0X"),)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.05)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system.call_method1("noise_get", (("2iY", "2iY"),)).unwrap();
        let comparison = bool::extract(
            comp_op
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::from(0.0)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Try_set error 1a: Key left (PauliProduct) cannot be converted from string
        let error = system.call_method1(
            "noise_add_operator_product",
            (
                ("1+0Z", "1Z"),
                convert_cf_to_pyobject(py, CalculatorFloat::from(0.5)),
            ),
        );
        assert!(error.is_err()); // same error as in ADP - Ask!

        // Try_set error 1b: Key right (PauliProduct) cannot be converted from string
        let error = system.call_method1(
            "noise_add_operator_product",
            (
                ("1Z", "1+0Z"),
                convert_cf_to_pyobject(py, CalculatorFloat::from(0.5)),
            ),
        );
        assert!(error.is_err()); // same error as in ADP - Ask!

        // Try_set error 2: Value cannot be converted to CalculatorFloat
        let error = system.call_method1("noise_add_operator_product", (("0X", "0X"), vec![0.0]));
        assert!(error.is_err());

        // // Try_set error 3: Generic error
        // let error = system.call_method1("noise_add_operator_product", ("1J", 0.5));
        // assert!(error.is_err());
    });
}

#[test_case(1.0,0.0;"real")]
#[test_case(0.0,1.0;"imag")]
#[test_case(0.7,0.7;"mixed")]
fn test_truncate(re: f64, im: f64) {
    pyo3::Python::with_gil(|py| {
        let system = new_system(py);
        system
            .call_method1(
                "noise_add_operator_product",
                (
                    ("0X", "0X"),
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
                    "1Y",
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
                    ("0X", "2Z"),
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
                    "0X1Z",
                    CalculatorFloatWrapper {
                        internal: CalculatorFloat::from("test"),
                    },
                ),
            )
            .unwrap();

        let test_system1 = new_system(py);
        test_system1
            .call_method1(
                "noise_add_operator_product",
                (
                    ("0X", "0X"),
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
                    "1Y",
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
                    "0X1Z",
                    CalculatorFloatWrapper {
                        internal: CalculatorFloat::from("test"),
                    },
                ),
            )
            .unwrap();

        let test_system2 = new_system(py);
        test_system2
            .call_method1(
                "noise_add_operator_product",
                (
                    ("0X", "0X"),
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
                    "0X1Z",
                    CalculatorFloatWrapper {
                        internal: CalculatorFloat::from("test"),
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

/// Test copy and deepcopy functions of SpinLindbladOpenSystem
#[test]
fn test_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = new_system(py);
        let mut system = new_system
            .call_method1(
                "system_add_operator_product",
                ("0X", convert_cf_to_pyobject(py, CalculatorFloat::from(0.1))),
            )
            .unwrap();
        system = system
            .call_method1(
                "noise_add_operator_product",
                (
                    ("0X", "0X"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();

        let copy_system = system.call_method0("__copy__").unwrap();
        let deepcopy_system = system.call_method1("__deepcopy__", ("",)).unwrap();
        // let copy_deepcopy_param: &PyAny = system.clone();

        let comparison_copy =
            bool::extract(copy_system.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison_copy);
        let comparison_deepcopy =
            bool::extract(deepcopy_system.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison_deepcopy);
    });
}

/// Test to_bincode and from_bincode functions of SpinLindbladOpenSystem
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system_1 = new_system(py);
        let mut system = new_system_1
            .call_method1(
                "system_add_operator_product",
                ("0X", convert_cf_to_pyobject(py, CalculatorFloat::from(0.1))),
            )
            .unwrap();
        system = system
            .call_method1(
                "noise_add_operator_product",
                (
                    ("0X", "0X"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();

        let serialised = system.call_method0("to_bincode").unwrap();
        let new = new_system(py);
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
        let new = new_system(py);
        let deserialised_error = new.call_method1("from_bincode", ("J",));
        assert!(deserialised_error.is_err());
    });
}

/// Test to_ and from_json functions of SpinLindbladOpenSystem
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system_1 = new_system(py);
        let mut system = new_system_1
            .call_method1(
                "system_add_operator_product",
                ("0X", convert_cf_to_pyobject(py, CalculatorFloat::from(0.1))),
            )
            .unwrap();
        system = system
            .call_method1(
                "noise_add_operator_product",
                (
                    ("0X", "0X"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();

        let serialised = system.call_method0("to_json").unwrap();
        let new = new_system(py);
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
        let new_system = new_system(py);
        let mut system = new_system
            .call_method1(
                "system_add_operator_product",
                ("0X", convert_cf_to_pyobject(py, CalculatorFloat::from(0.1))),
            )
            .unwrap();
        system = system
            .call_method1(
                "noise_add_operator_product",
                (
                    ("1X", "1iY"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        let mut rust_system =
            SpinLindbladOpenSystem::group(SpinHamiltonian::new(), SpinLindbladNoiseOperator::new())
                .unwrap();
        rust_system
            .system_mut()
            .add_operator_product(PauliProduct::new().x(0), CalculatorFloat::from(0.1))
            .unwrap();
        let _ = rust_system.noise_mut().add_operator_product(
            (
                DecoherenceProduct::new().x(1),
                DecoherenceProduct::new().iy(1),
            ),
            CalculatorComplex::from(0.1),
        );

        let test_string =
        "SpinLindbladOpenSystem{\nSystem: {\n0X: 1e-1,\n}\nNoise: {\n(1X, 1iY): (1e-1 + i * 0e0),\n}\n}"
            .to_string();

        let to_format = system.call_method1("__format__", ("",)).unwrap();
        let format_op: &str = <&str>::extract(to_format).unwrap();
        assert_eq!(format_op, test_string);

        let to_repr = system.call_method0("__repr__").unwrap();
        let repr_op: &str = <&str>::extract(to_repr).unwrap();
        assert_eq!(repr_op, test_string);

        let to_str = system.call_method0("__str__").unwrap();
        let str_op: &str = <&str>::extract(to_str).unwrap();
        assert_eq!(str_op, test_string);
    });
}

/// Test the __richcmp__ function
#[test]
fn test_richcmp() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system_1 = new_system(py);
        let mut system_one = new_system_1
            .call_method1(
                "system_add_operator_product",
                ("0X", convert_cf_to_pyobject(py, CalculatorFloat::from(0.1))),
            )
            .unwrap();
        system_one = system_one
            .call_method1(
                "noise_add_operator_product",
                (
                    ("1X", "1iY"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        let new_system_1 = new_system(py);
        let mut system_two = new_system_1
            .call_method1(
                "system_add_operator_product",
                ("1X", convert_cf_to_pyobject(py, CalculatorFloat::from(0.1))),
            )
            .unwrap();
        system_two = system_two
            .call_method1(
                "noise_add_operator_product",
                (
                    ("0X", "1iY"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();

        let comparison =
            bool::extract(system_one.call_method1("__eq__", (system_two,)).unwrap()).unwrap();
        assert!(!comparison);
        let comparison =
            bool::extract(system_one.call_method1("__eq__", ("0X",)).unwrap()).unwrap();
        assert!(!comparison);

        let comparison =
            bool::extract(system_one.call_method1("__ne__", (system_two,)).unwrap()).unwrap();
        assert!(comparison);
        let comparison =
            bool::extract(system_one.call_method1("__ne__", ("0X",)).unwrap()).unwrap();
        assert!(comparison);

        let comparison = system_one.call_method1("__ge__", ("0X",));
        assert!(comparison.is_err());
    });
}

/// Test jordan_wigner() method of SpinLindbladOpenSystem
#[test]
fn test_jordan_wigner() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let slos = new_system(py);
        slos.call_method1("system_add_operator_product", ("0X", 0.1))
            .unwrap();
        slos.call_method1("noise_add_operator_product", (("0X", "1iY"), 0.1))
            .unwrap();
        let flos = slos.call_method0("jordan_wigner").unwrap();

        let number_modes = usize::extract(flos.call_method0("number_modes").unwrap()).unwrap();
        let number_spins = usize::extract(slos.call_method0("number_spins").unwrap()).unwrap();
        assert_eq!(number_modes, number_spins)
    });
}

#[cfg(feature = "json_schema")]
#[test]
fn test_json_schema() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new = new_system(py);

        let schema: String = String::extract(new.call_method0("json_schema").unwrap()).unwrap();
        let rust_schema =
            serde_json::to_string_pretty(&schemars::schema_for!(SpinLindbladOpenSystem)).unwrap();
        assert_eq!(schema, rust_schema);

        let version: String =
            String::extract(new.call_method0("current_version").unwrap()).unwrap();
        let rust_version = STRUQTURE_VERSION.to_string();
        assert_eq!(version, rust_version);

        new.call_method1("noise_add_operator_product", (("0Z", "1Z"), 1.0))
            .unwrap();
        let min_version: String =
            String::extract(new.call_method0("min_supported_version").unwrap()).unwrap();
        let rust_min_version = String::from("2.0.0");
        assert_eq!(min_version, rust_min_version);
    });
}

#[cfg(feature = "struqture_1_export")]
#[test]
fn test_from_pyany_to_struqture_one() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        use std::str::FromStr;
        let new_system_1 = new_system(py);
        let mut sys_2 = new_system_1
            .call_method1(
                "system_add_operator_product",
                ("0X", convert_cf_to_pyobject(py, CalculatorFloat::from(0.1))),
            )
            .unwrap();
        sys_2 = sys_2
            .call_method1(
                "noise_add_operator_product",
                (
                    ("0iY", "0iY"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();

        let pp_1 = struqture_one::spins::PauliProduct::from_str("0X").unwrap();
        let dp_1 = struqture_one::spins::DecoherenceProduct::from_str("0iY").unwrap();
        let mut sys_1 = struqture_one::spins::SpinLindbladOpenSystem::new(None);
        let system_mut_1 = struqture_one::OpenSystem::system_mut(&mut sys_1);
        struqture_one::OperateOnDensityMatrix::set(system_mut_1, pp_1.clone(), 0.1.into()).unwrap();
        let noise_mut_1 = struqture_one::OpenSystem::noise_mut(&mut sys_1);
        struqture_one::OperateOnDensityMatrix::set(
            noise_mut_1,
            (dp_1.clone(), dp_1.clone()),
            0.1.into(),
        )
        .unwrap();

        let result =
            SpinLindbladOpenSystemWrapper::from_pyany_to_struqture_one(sys_2.into()).unwrap();
        assert_eq!(result, sys_1);
    });
}

#[cfg(feature = "struqture_1_import")]
#[test]
fn test_from_json_struqture_one() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let json_string: &PyAny = pyo3::types::PyString::new(py, "{\"system\":{\"number_spins\":null,\"hamiltonian\":{\"items\":[[\"0Z\",1.0]],\"_struqture_version\":{\"major_version\":1,\"minor_version\":0}}},\"noise\":{\"number_spins\":null,\"operator\":{\"items\":[[\"0X\",\"0X\",1.0,0.0]],\"_struqture_version\":{\"major_version\":1,\"minor_version\":0}}}}").into();
        let sys_2 = new_system(py);
        sys_2
            .call_method1("system_add_operator_product", ("0Z", 1.0))
            .unwrap();
        sys_2
            .call_method1("noise_add_operator_product", (("0X", "0X"), 1.0))
            .unwrap();

        let sys_from_1 = sys_2
            .call_method1("from_json_struqture_one", (json_string,))
            .unwrap();
        let equal = bool::extract(sys_2.call_method1("__eq__", (sys_from_1,)).unwrap()).unwrap();
        assert!(equal);

        let error_json_string: &PyAny = pyo3::types::PyString::new(py, "{\"system\":{\"number_spins\":null,\"hamiltonian\":{\"items\":[[\"0Z\",1.0]],\"_struqture_version\":{\"major_version\":1,\"minor_version\":0}}},\"noise\":{\"number_spins\":null,\"operator\":{\"items\":[[\"0X\",\"0X\",1.0,0.0]],\"_struqture_version\":{\"major_version\":3-,\"minor_version\":0}}}}").into();
        let sys_from_1 = sys_2.call_method1("from_json_struqture_one", (error_json_string,));
        assert!(sys_from_1.is_err());
    });
}
