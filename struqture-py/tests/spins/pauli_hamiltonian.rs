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
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use qoqo_calculator_pyo3::{CalculatorComplexWrapper, CalculatorFloatWrapper};
use struqture::spins::{PauliHamiltonian, PauliProduct};
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture::{OperateOnDensityMatrix, SpinIndex};
use struqture_py::spins::{PauliHamiltonianWrapper, PauliOperatorWrapper};
use test_case::test_case;

// helper functions
fn new_system(py: Python) -> Bound<PauliHamiltonianWrapper> {
    let system_type = py.get_type::<PauliHamiltonianWrapper>();
    system_type
        .call0()
        .unwrap()
        .downcast::<PauliHamiltonianWrapper>()
        .unwrap()
        .to_owned()
}
fn new_operator(py: Python) -> Bound<PauliOperatorWrapper> {
    let system_type = py.get_type::<PauliOperatorWrapper>();
    system_type
        .call0()
        .unwrap()
        .downcast::<PauliOperatorWrapper>()
        .unwrap()
        .to_owned()
}

/// Test default function of PauliHamiltonianWrapper
#[test]
fn test_default_partialeq_debug_clone() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let new_system = new_system(py);
        new_system
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();
        let system_wrapper = new_system.extract::<PauliHamiltonianWrapper>().unwrap();

        // PartialEq
        let helper_ne: bool = PauliHamiltonianWrapper::new() != system_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = PauliHamiltonianWrapper::new() == PauliHamiltonianWrapper::new();
        assert!(helper_eq);

        // Clone
        assert_eq!(system_wrapper.clone(), system_wrapper);

        // Debug
        assert_eq!(
            format!("{:?}", PauliHamiltonianWrapper::new()),
            "PauliHamiltonianWrapper { internal: PauliHamiltonian { internal_map: {} } }"
        );

        // Number of spins
        let comp_op = new_system.call_method0("current_number_spins").unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (1,)).unwrap()).unwrap();
        assert!(comparison);
    })
}

/// Test current_number_spins function of PauliHamiltonian
#[test]
fn test_number_spins_current() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_system(py);
        system
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();

        let number_system = system.call_method0("current_number_spins").unwrap();

        let comparison =
            bool::extract_bound(&number_system.call_method1("__eq__", (1_u64,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test empty_clone function of PauliHamiltonian
#[test]
fn test_empty_clone() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_system(py);
        let none_system = system.call_method0("empty_clone").unwrap();
        let comparison =
            bool::extract_bound(&none_system.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison);

        let system = new_system(py);
        let some_system = system.call_method0("empty_clone").unwrap();
        let comparison =
            bool::extract_bound(&some_system.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test hermitian_conjugate function of PauliHamiltonian
#[test]
fn test_hermitian_conj() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_system(py);
        system
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();

        let conjugate = system.call_method0("hermitian_conjugate").unwrap();
        let comparison =
            bool::extract_bound(&conjugate.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test set and get functions of PauliHamiltonian
#[test]
fn spin_system_test_set_get() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type::<PauliHamiltonianWrapper>();
        let system = new_system.call0().unwrap();
        system.downcast::<PauliHamiltonianWrapper>().unwrap();
        system.call_method1("set", ("0X", 0.1)).unwrap();
        system.call_method1("set", ("1Z", 0.2)).unwrap();
        system.call_method1("set", ("3Y", 0.05)).unwrap();

        // test access at index 0
        let comp_op = system.call_method1("get", ("0X",)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.1,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system.call_method1("get", ("1Z",)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.2,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system.call_method1("get", ("3Y",)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.05,)).unwrap()).unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system.call_method1("get", ("2X",)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
        assert!(comparison);

        // Try_set error 1: Key (PauliProduct) cannot be converted from string
        let error = system.call_method1("set", ("1J", 0.5));
        assert!(error.is_err());

        // Try_set error 2: Value cannot be converted to CalculatorFloat
        let error = system.call_method1("set", ("1Z", vec![0.0]));
        assert!(error.is_err());

        // Try_set error 3: Generic error
        let error = system.call_method1("set", ("1J", 0.5));
        assert!(error.is_err());
    });
}

/// Test add_operator_product and remove functions of PauliHamiltonian
#[test]
fn spin_system_test_add_operator_product_remove() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type::<PauliHamiltonianWrapper>();
        let system = new_system.call0().unwrap();
        system.downcast::<PauliHamiltonianWrapper>().unwrap();
        system
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();
        system
            .call_method1("add_operator_product", ("1Z", 0.2))
            .unwrap();
        system
            .call_method1("add_operator_product", ("3Y", 0.05))
            .unwrap();

        // test access at index 0
        let comp_op = system.call_method1("get", ("0X",)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.1,)).unwrap()).unwrap();
        assert!(comparison);
        system.call_method1("remove", ("0X",)).unwrap();
        let comp_op = system.call_method1("get", ("0X",)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system.call_method1("get", ("1Z",)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.2,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system.call_method1("get", ("3Y",)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.05,)).unwrap()).unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system.call_method1("get", ("2X",)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
        assert!(comparison);

        // Get error
        let error = system.call_method1("get", ("2J",));
        assert!(error.is_err());

        // Try_set error 1: Key (PauliProduct) cannot be converted from string
        let error = system.call_method1("add_operator_product", ("1J", 0.5));
        assert!(error.is_err());

        // Try_set error 2: Value cannot be converted to CalculatorFloat
        let error = system.call_method1("add_operator_product", ("1Z", vec![0.0]));
        assert!(error.is_err());

        // Try_set error 3: Generic error
        let error = system.call_method1("add_operator_product", ("1J", 0.5));
        assert!(error.is_err());
    });
}

/// Test keys function of PauliHamiltonian
#[test]
fn test_keys_values() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_system(py);

        let len_system = system.call_method0("__len__").unwrap();
        let comparison =
            bool::extract_bound(&len_system.call_method1("__eq__", (0_u64,)).unwrap()).unwrap();
        assert!(comparison);
        let empty_system = bool::extract_bound(&system.call_method0("is_empty").unwrap()).unwrap();
        assert!(empty_system);

        system
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();

        let keys_system = system.call_method0("keys").unwrap();
        let comparison =
            bool::extract_bound(&keys_system.call_method1("__eq__", (vec!["0X"],)).unwrap())
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

#[test_case(1.0;"real")]
fn test_truncate(re: f64) {
    pyo3::Python::with_gil(|py| {
        let system = new_system(py);
        system
            .call_method1(
                "add_operator_product",
                (
                    "0X",
                    CalculatorFloatWrapper {
                        internal: CalculatorFloat::from(100.0 * re),
                    },
                ),
            )
            .unwrap();
        system
            .call_method1(
                "add_operator_product",
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
                "add_operator_product",
                (
                    "2Z",
                    CalculatorFloatWrapper {
                        internal: CalculatorFloat::from(re),
                    },
                ),
            )
            .unwrap();
        system
            .call_method1(
                "add_operator_product",
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
                "add_operator_product",
                (
                    "0X",
                    CalculatorFloatWrapper {
                        internal: CalculatorFloat::from(100.0 * re),
                    },
                ),
            )
            .unwrap();
        test_system1
            .call_method1(
                "add_operator_product",
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
                "add_operator_product",
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
                "add_operator_product",
                (
                    "0X",
                    CalculatorFloatWrapper {
                        internal: CalculatorFloat::from(100.0 * re),
                    },
                ),
            )
            .unwrap();
        test_system2
            .call_method1(
                "add_operator_product",
                (
                    "0X1Z",
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

/// Test add magic method function of PauliHamiltonian
#[test]
fn test_neg() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_system(py);
        system_0
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();
        let system_1 = new_system(py);
        system_1
            .call_method1("add_operator_product", ("0X", -0.1))
            .unwrap();

        let negated = system_0.call_method0("__neg__").unwrap();
        let comparison =
            bool::extract_bound(&negated.call_method1("__eq__", (system_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of PauliHamiltonian
#[test]
fn test_add() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_system(py);
        system_0
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();
        let system_1 = new_system(py);
        system_1
            .call_method1("add_operator_product", ("1Z", 0.2))
            .unwrap();
        let system_0_1 = new_system(py);
        system_0_1
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();
        system_0_1
            .call_method1("add_operator_product", ("1Z", 0.2))
            .unwrap();

        let added = system_0.call_method1("__add__", (system_1,)).unwrap();
        let comparison =
            bool::extract_bound(&added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of PauliHamiltonian
#[test]
fn test_sub() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_system(py);
        system_0
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();
        let system_1 = new_system(py);
        system_1
            .call_method1("add_operator_product", ("1Z", 0.2))
            .unwrap();
        let system_0_1 = new_system(py);
        system_0_1
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();
        system_0_1
            .call_method1("add_operator_product", ("1Z", -0.2))
            .unwrap();

        let added = system_0.call_method1("__sub__", (system_1,)).unwrap();
        let comparison =
            bool::extract_bound(&added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of PauliHamiltonian
#[test]
fn test_mul_cf() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_system(py);
        system_0
            .call_method1("add_operator_product", ("0X", 0.1_f64))
            .unwrap();

        let system_0_1 = new_operator(py);
        system_0_1
            .call_method1("add_operator_product", ("0X", 0.2))
            .unwrap();

        let added = system_0.call_method1("__mul__", (2.0,)).unwrap();
        let comparison =
            bool::extract_bound(&added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of PauliHamiltonian
#[test]
fn test_mul_cc() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_system(py);
        system_0
            .call_method1("add_operator_product", ("0X", 0.1_f64))
            .unwrap();

        let system_0_1 = new_operator(py);
        system_0_1
            .call_method1("add_operator_product", ("0X", Complex64::new(0.0, 0.5)))
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

/// Test add magic method function of PauliHamiltonian
#[test]
fn test_mul_self() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_system(py);
        system_0
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();
        let system_1 = new_system(py);
        system_1
            .call_method1("add_operator_product", ("1Z", 1.0))
            .unwrap();
        let system_0_1 = new_operator(py);
        system_0_1
            .call_method1("add_operator_product", ("0X1Z", 0.1))
            .unwrap();

        let added = system_0.call_method1("__mul__", (system_1,)).unwrap();
        let comparison =
            bool::extract_bound(&added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of PauliHamiltonian
#[test]
fn test_mul_error() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_system(py);
        system_0
            .call_method1("add_operator_product", ("0X", 0.1_f64))
            .unwrap();

        let added = system_0.call_method1("__mul__", (vec![0.0],));
        assert!(added.is_err());
    });
}

/// Test copy and deepcopy functions of PauliHamiltonian
#[test]
fn test_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_system(py);
        system
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();

        let copy_system = system.call_method0("__copy__").unwrap();
        let deepcopy_system = system.call_method1("__deepcopy__", ("",)).unwrap();
        // let copy_deepcopy_param: Bound<pyo3::types::PyString> = system.clone();

        let comparison_copy =
            bool::extract_bound(&copy_system.call_method1("__eq__", (&system,)).unwrap()).unwrap();
        assert!(comparison_copy);
        let comparison_deepcopy =
            bool::extract_bound(&deepcopy_system.call_method1("__eq__", (system,)).unwrap())
                .unwrap();
        assert!(comparison_deepcopy);
    });
}

/// Test to_bincode and from_bincode functions of PauliHamiltonian
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_system(py);
        system
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();

        let serialised = system.call_method0("to_bincode").unwrap();
        let new = new_system(py);
        let deserialised = new
            .call_method1("from_bincode", (serialised.clone(),))
            .unwrap();
        let config = bincode::config::legacy();

        let deserialised_error = new.call_method1(
            "from_bincode",
            (bincode::serde::encode_to_vec("fails", config).unwrap(),),
        );
        assert!(deserialised_error.is_err());

        let deserialised_error = new.call_method1(
            "from_bincode",
            (bincode::serde::encode_to_vec(vec![0], config).unwrap(),),
        );
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
        let new = new_system(py);
        let deserialised_error = new.call_method1("from_bincode", ("J",));
        assert!(deserialised_error.is_err());
    });
}

/// Test to_ and from_json functions of PauliHamiltonian
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_system(py);
        system
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();

        let serialised = system.call_method0("to_json").unwrap();
        let new = new_system(py);
        let deserialised = new
            .call_method1("from_json", (serialised.clone(),))
            .unwrap();

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
        let system = new_system(py);
        system
            .call_method1("add_operator_product", ("0X", 0.1_f64))
            .unwrap();
        let mut rust_system = PauliHamiltonian::new();
        rust_system
            .add_operator_product(PauliProduct::new().x(0), CalculatorFloat::from(0.1))
            .unwrap();
        let to_format = system.call_method1("__format__", ("",)).unwrap();
        let format_op: String = String::extract_bound(&to_format).unwrap();

        let to_repr = system.call_method0("__repr__").unwrap();
        let repr_op: String = String::extract_bound(&to_repr).unwrap();

        let to_str = system.call_method0("__str__").unwrap();
        let str_op: String = String::extract_bound(&to_str).unwrap();

        assert_eq!(format_op, "PauliHamiltonian{\n0X: 1e-1,\n}".to_string());
        assert_eq!(repr_op, "PauliHamiltonian{\n0X: 1e-1,\n}".to_string());
        assert_eq!(str_op, "PauliHamiltonian{\n0X: 1e-1,\n}".to_string());
    });
}

/// Test the __richcmp__ function
#[test]
fn test_richcmp() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_one = new_system(py);
        system_one
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();
        let system_two = new_system(py);
        system_two
            .call_method1("add_operator_product", ("1X", 0.1))
            .unwrap();

        let comparison =
            bool::extract_bound(&system_one.call_method1("__eq__", (&system_two,)).unwrap())
                .unwrap();
        assert!(!comparison);
        let comparison =
            bool::extract_bound(&system_one.call_method1("__eq__", ("0X",)).unwrap()).unwrap();
        assert!(!comparison);

        let comparison =
            bool::extract_bound(&system_one.call_method1("__ne__", (system_two,)).unwrap())
                .unwrap();
        assert!(comparison);
        let comparison =
            bool::extract_bound(&system_one.call_method1("__ne__", ("0X",)).unwrap()).unwrap();
        assert!(comparison);

        let comparison = system_one.call_method1("__ge__", ("0X",));
        assert!(comparison.is_err());
    });
}

/// Test jordan_wigner() method of PauliHamiltonian
#[test]
fn test_jordan_wigner() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let shs = new_system(py);
        shs.call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();
        let fhs = shs.call_method0("jordan_wigner").unwrap();

        let empty = bool::extract_bound(&fhs.call_method0("is_empty").unwrap()).unwrap();
        assert!(!empty);

        let current_number_modes =
            usize::extract_bound(&fhs.call_method0("current_number_modes").unwrap()).unwrap();
        let current_number_spins =
            usize::extract_bound(&shs.call_method0("current_number_spins").unwrap()).unwrap();
        assert_eq!(current_number_modes, current_number_spins)
    });
}

#[cfg(feature = "json_schema")]
#[test]
fn test_json_schema() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new = new_system(py);

        let schema: String =
            String::extract_bound(&new.call_method0("json_schema").unwrap()).unwrap();
        let rust_schema =
            serde_json::to_string_pretty(&schemars::schema_for!(PauliHamiltonian)).unwrap();
        assert_eq!(schema, rust_schema);

        let version: String =
            String::extract_bound(&new.call_method0("current_version").unwrap()).unwrap();
        let rust_version = STRUQTURE_VERSION.to_string();
        assert_eq!(version, rust_version);

        new.call_method1("add_operator_product", ("0Z", 1.0))
            .unwrap();
        let min_version: String =
            String::extract_bound(&new.call_method0("min_supported_version").unwrap()).unwrap();
        let rust_min_version = String::from("2.0.0");
        assert_eq!(min_version, rust_min_version);
    });
}

#[cfg(feature = "struqture_1_import")]
#[test]
fn test_from_json_struqture_1() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let json_string: Bound<pyo3::types::PyString> = pyo3::types::PyString::new(py, "{\"number_spins\":null,\"hamiltonian\":{\"items\":[[\"0Z\",1.0]],\"_struqture_version\":{\"major_version\":1,\"minor_version\":0}}}");
        let sys_2 = new_system(py);
        sys_2
            .call_method1("add_operator_product", ("0Z", 1.0))
            .unwrap();

        let sys_from_1 = sys_2
            .call_method1("from_json_struqture_1", (json_string,))
            .unwrap();
        let equal =
            bool::extract_bound(&sys_2.call_method1("__eq__", (sys_from_1,)).unwrap()).unwrap();
        assert!(equal);

        let error_json_string: Bound<pyo3::types::PyString> = pyo3::types::PyString::new(py, "{{\"number_spins\":null,\"hamiltonian\":{{\"items\":[[\"0Z\",1.0]],\"_struqture_version\":{{\"major_version\":30,\"minor_version\":0}}}}}}");
        let sys_from_1 = sys_2.call_method1("from_json_struqture_1", (error_json_string,));
        assert!(sys_from_1.is_err());
    });
}
