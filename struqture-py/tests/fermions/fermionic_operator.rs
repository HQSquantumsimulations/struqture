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
use struqture::fermions::{FermionOperator, FermionProduct};
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture::{ModeIndex, OperateOnDensityMatrix};
use struqture_py::fermions::FermionOperatorWrapper;
use test_case::test_case;

// helper functions
fn new_system(py: Python) -> &PyCell<FermionOperatorWrapper> {
    let system_type = py.get_type::<FermionOperatorWrapper>();
    system_type
        .call0()
        .unwrap()
        .downcast::<PyCell<FermionOperatorWrapper>>()
        .unwrap()
}

/// Test default function of FermionOperatorWrapper
#[test]
fn test_default_partialeq_debug_clone() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let new_system = new_system(py);
        new_system
            .call_method1("add_operator_product", ("c0c1a0a1", 0.1))
            .unwrap();
        let system_wrapper = new_system.extract::<FermionOperatorWrapper>().unwrap();

        // PartialEq
        let helper_ne: bool = FermionOperatorWrapper::new() != system_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = FermionOperatorWrapper::new() == FermionOperatorWrapper::new();
        assert!(helper_eq);

        // Clone
        assert_eq!(system_wrapper.clone(), system_wrapper);

        // Debug
        assert_eq!(
            format!("{:?}", FermionOperatorWrapper::new()),
            "FermionOperatorWrapper { internal: FermionOperator { internal_map: {} } }"
        );

        // Number of fermions
        let comp_op = new_system.call_method0("number_modes").unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (2,)).unwrap()).unwrap();
        assert!(comparison);
    })
}

/// Test number_fermions function of FermionOperator
#[test]
fn test_number_fermions_current() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_system(py);
        system
            .call_method1("add_operator_product", ("c0c1a0a1", 0.1))
            .unwrap();

        let number_system = system.call_method0("number_modes").unwrap();

        let comparison =
            bool::extract(number_system.call_method1("__eq__", (2_u64,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test empty_clone function of FermionOperator
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

/// Test hermitian_conjugate function of FermionOperator
#[test]
fn test_hermitian_conj() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_system(py);
        system
            .call_method1("add_operator_product", ("c0c1a0a1", 0.1))
            .unwrap();

        let conjugate = system.call_method0("hermitian_conjugate").unwrap();
        let comparison =
            bool::extract(conjugate.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test set and get functions of FermionOperator
#[test]
fn fermion_system_test_set_get() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type::<FermionOperatorWrapper>();
        let system = new_system
            .call0()
            .unwrap()
            .downcast::<PyCell<FermionOperatorWrapper>>()
            .unwrap();
        system.call_method1("set", ("c0c1a0a1", 0.1)).unwrap();
        system.call_method1("set", ("c2c3a1", 0.2)).unwrap();
        system.call_method1("set", ("c0a2a3", 0.05)).unwrap();

        // test access at index 0
        let comp_op = system.call_method1("get", ("c0c1a0a1",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.1,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system.call_method1("get", ("c2c3a1",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.2,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system.call_method1("get", ("c0a2a3",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.05,)).unwrap()).unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system.call_method1("get", ("c0a2",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
        assert!(comparison);

        // Try_set error 1: Key (FermionProduct) cannot be converted from string
        let error = system.call_method1("set", ("d3", 0.5));
        assert!(error.is_err());

        // Try_set error 2: Value cannot be converted to CalculatorComplex
        let error = system.call_method1("set", ("c2c3a1", vec![0.0]));
        assert!(error.is_err());

        // Try_set error 3: Generic error
        let error = system.call_method1("set", (vec![0.0], 0.5));
        assert!(error.is_err());
    });
}

/// Test add_operator_product and remove functions of FermionOperator
#[test]
fn fermion_system_test_add_operator_product_remove() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type::<FermionOperatorWrapper>();
        let system = new_system
            .call0()
            .unwrap()
            .downcast::<PyCell<FermionOperatorWrapper>>()
            .unwrap();
        system
            .call_method1("add_operator_product", ("c0c1a0a1", 0.1))
            .unwrap();
        system
            .call_method1("add_operator_product", ("c2c3a1", 0.2))
            .unwrap();
        system
            .call_method1("add_operator_product", ("c0a2a3", 0.05))
            .unwrap();

        // test access at index 0
        let comp_op = system.call_method1("get", ("c0c1a0a1",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.1,)).unwrap()).unwrap();
        assert!(comparison);
        system.call_method1("remove", ("c0c1a0a1",)).unwrap();
        let comp_op = system.call_method1("get", ("c0c1a0a1",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system.call_method1("get", ("c2c3a1",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.2,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system.call_method1("get", ("c0a2a3",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.05,)).unwrap()).unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system.call_method1("get", ("c3",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
        assert!(comparison);

        // Get error
        let error = system.call_method1("get", ("d2",));
        assert!(error.is_err());

        // Try_set error 1: Key (FermionProduct) cannot be converted from string
        let error = system.call_method1("add_operator_product", ("d2", 0.5));
        assert!(error.is_err());

        // Try_set error 2: Value cannot be converted to CalculatorComplex
        let error = system.call_method1("add_operator_product", ("c2c3a1", vec![0.0]));
        assert!(error.is_err());

        // Try_set error 3: Generic error
        let error = system.call_method1("add_operator_product", (vec![0.0], 0.5));
        assert!(error.is_err());
    });
}

/// Test keys function of FermionOperator
#[test]
fn test_keys_values() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_system(py);

        let len_system = system.call_method0("__len__").unwrap();
        let comparison =
            bool::extract(len_system.call_method1("__eq__", (0_u64,)).unwrap()).unwrap();
        assert!(comparison);
        let empty_system = bool::extract(system.call_method0("is_empty").unwrap()).unwrap();
        assert!(empty_system);

        system
            .call_method1("add_operator_product", ("c0c1a0a1", 0.1))
            .unwrap();

        let keys_system = system.call_method0("keys").unwrap();
        let comparison = bool::extract(
            keys_system
                .call_method1("__eq__", (vec!["c0c1a0a1"],))
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
#[test_case(0.0,1.0;"imag")]
#[test_case(0.7,0.7;"mixed")]
fn test_truncate(re: f64, im: f64) {
    pyo3::Python::with_gil(|py| {
        let system = new_system(py);
        system
            .call_method1(
                "add_operator_product",
                (
                    "c0c1a0a1",
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
                    "c0c1a2",
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
                    "c2a1a2",
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
                    "c0c1c2c3a0a1a2",
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::from("test"),
                    },
                ),
            )
            .unwrap();

        let test_system1 = new_system(py);
        test_system1
            .call_method1(
                "add_operator_product",
                (
                    "c0c1a0a1",
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
                    "c0c1a2",
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
                    "c0c1c2c3a0a1a2",
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::from("test"),
                    },
                ),
            )
            .unwrap();

        let test_system2 = new_system(py);
        test_system2
            .call_method1(
                "add_operator_product",
                (
                    "c0c1a0a1",
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
                    "c0c1c2c3a0a1a2",
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

#[test]
fn test_separate() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pmp = new_system(py);
        pmp.call_method1("add_operator_product", ("c0a0", 1.0))
            .unwrap();
        pmp.call_method1("add_operator_product", ("c0c1a0", 1.0))
            .unwrap();
        pmp.call_method1("add_operator_product", ("c0c2a0", 1.0))
            .unwrap();

        let pmp_rem = new_system(py);
        pmp_rem
            .call_method1("add_operator_product", ("c0a0", 1.0))
            .unwrap();

        let pmp_sys = new_system(py);
        pmp_sys
            .call_method1("add_operator_product", ("c0c1a0", 1.0))
            .unwrap();
        pmp_sys
            .call_method1("add_operator_product", ("c0c2a0", 1.0))
            .unwrap();

        let result = pmp
            .call_method1("separate_into_n_terms", ((2, 1),))
            .unwrap();
        let equal = bool::extract(
            result
                .call_method1("__eq__", ((pmp_sys, pmp_rem),))
                .unwrap(),
        )
        .unwrap();
        assert!(equal);
    })
}

/// Test add magic method function of FermionOperator
#[test]
fn test_neg() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_system(py);
        system_0
            .call_method1("add_operator_product", ("c0c1a0a1", 0.1))
            .unwrap();
        let system_1 = new_system(py);
        system_1
            .call_method1("add_operator_product", ("c0c1a0a1", -0.1))
            .unwrap();

        let negated = system_0.call_method0("__neg__").unwrap();
        let comparison =
            bool::extract(negated.call_method1("__eq__", (system_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of FermionOperator
#[test]
fn test_add() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_system(py);
        system_0
            .call_method1("add_operator_product", ("c0c1a0a1", 0.1))
            .unwrap();
        let system_1 = new_system(py);
        system_1
            .call_method1("add_operator_product", ("c2c3a1", 0.2))
            .unwrap();
        let system_0_1 = new_system(py);
        system_0_1
            .call_method1("add_operator_product", ("c0c1a0a1", 0.1))
            .unwrap();
        system_0_1
            .call_method1("add_operator_product", ("c2c3a1", 0.2))
            .unwrap();

        let added = system_0.call_method1("__add__", (system_1,)).unwrap();
        let comparison =
            bool::extract(added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of FermionOperator
#[test]
fn test_sub() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_system(py);
        system_0
            .call_method1("add_operator_product", ("c0c1a0a1", 0.1))
            .unwrap();
        let system_1 = new_system(py);
        system_1
            .call_method1("add_operator_product", ("c2c3a1", 0.2))
            .unwrap();
        let system_0_1 = new_system(py);
        system_0_1
            .call_method1("add_operator_product", ("c0c1a0a1", 0.1))
            .unwrap();
        system_0_1
            .call_method1("add_operator_product", ("c2c3a1", -0.2))
            .unwrap();

        let added = system_0.call_method1("__sub__", (system_1,)).unwrap();
        let comparison =
            bool::extract(added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of FermionOperator
#[test]
fn test_mul_cf() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_system(py);
        system_0
            .call_method1("add_operator_product", ("c0c1a0a1", 0.1_f64))
            .unwrap();

        let system_0_1 = new_system(py);
        system_0_1
            .call_method1("add_operator_product", ("c0c1a0a1", 0.2))
            .unwrap();

        let added = system_0.call_method1("__mul__", (2.0,)).unwrap();
        let comparison =
            bool::extract(added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of FermionOperator
#[test]
fn test_mul_cc() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_system(py);
        system_0
            .call_method1("add_operator_product", ("c0c1a0a1", 0.1_f64))
            .unwrap();

        let system_0_1 = new_system(py);
        system_0_1
            .call_method1(
                "add_operator_product",
                ("c0c1a0a1", Complex64::new(0.0, 0.5)),
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

/// Test add magic method function of FermionOperator
#[test]
fn test_mul_self() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_system(py);
        system_0
            .call_method1("add_operator_product", ("c0c1a0", 0.1))
            .unwrap();
        let system_1 = new_system(py);
        system_1
            .call_method1("add_operator_product", ("c2c3a1", 1.0))
            .unwrap();
        let system_0_1 = new_system(py);
        system_0_1
            .call_method1("add_operator_product", ("c0c1c2c3a0a1", 0.1))
            .unwrap();

        let added = system_0.call_method1("__mul__", (system_1,)).unwrap();
        let comparison =
            bool::extract(added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of FermionOperator
#[test]
fn test_mul_error() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_system(py);
        system_0
            .call_method1("add_operator_product", ("c0c1a0a1", 0.1_f64))
            .unwrap();

        let added = system_0.call_method1("__mul__", (vec![0.0],));
        assert!(added.is_err());
    });
}

/// Test copy and deepcopy functions of FermionOperator
#[test]
fn test_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_system(py);
        system
            .call_method1("add_operator_product", ("c0c1a0a1", 0.1))
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

/// Test to_bincode and from_bincode functions of FermionOperator
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_system(py);
        system
            .call_method1("add_operator_product", ("c0c1a0a1", 0.1))
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

/// Test to_ and from_json functions of FermionOperator
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_system(py);
        system
            .call_method1("add_operator_product", ("c0c1a0a1", 0.1))
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
        let system = new_system(py);
        system
            .call_method1("add_operator_product", ("c0c1a0a1", 0.1_f64))
            .unwrap();
        let mut rust_system = FermionOperator::new();
        rust_system
            .add_operator_product(
                FermionProduct::new(vec![0, 1], vec![0, 1]).unwrap(),
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
            "FermionOperator{\nc0c1a0a1: (1e-1 + i * 0e0),\n}".to_string()
        );
        assert_eq!(
            repr_op,
            "FermionOperator{\nc0c1a0a1: (1e-1 + i * 0e0),\n}".to_string()
        );
        assert_eq!(
            str_op,
            "FermionOperator{\nc0c1a0a1: (1e-1 + i * 0e0),\n}".to_string()
        );
    });
}

/// Test the __richcmp__ function
#[test]
fn test_richcmp() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_one = new_system(py);
        system_one
            .call_method1("add_operator_product", ("c0c1a0a1", 0.1))
            .unwrap();
        let system_two = new_system(py);
        system_two
            .call_method1("add_operator_product", ("c0a2", 0.1))
            .unwrap();

        let comparison =
            bool::extract(system_one.call_method1("__eq__", (system_two,)).unwrap()).unwrap();
        assert!(!comparison);
        let comparison =
            bool::extract(system_one.call_method1("__eq__", ("c0c1a0a1",)).unwrap()).unwrap();
        assert!(!comparison);

        let comparison =
            bool::extract(system_one.call_method1("__ne__", (system_two,)).unwrap()).unwrap();
        assert!(comparison);
        let comparison =
            bool::extract(system_one.call_method1("__ne__", ("c0c1a0a1",)).unwrap()).unwrap();
        assert!(comparison);

        let comparison = system_one.call_method1("__ge__", ("c0c1a0a1",));
        assert!(comparison.is_err());
    });
}

/// Test jordan_wigner() method of FermionOperator
#[test]
fn test_jordan_wigner() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let fs = new_system(py);
        fs.call_method1("add_operator_product", ("c0c1a0a1", 0.1))
            .unwrap();
        let ss = fs.call_method0("jordan_wigner").unwrap();

        let empty = bool::extract(ss.call_method0("is_empty").unwrap()).unwrap();
        assert!(!empty);

        let number_modes = usize::extract(fs.call_method0("number_modes").unwrap()).unwrap();
        let number_spins = usize::extract(ss.call_method0("number_spins").unwrap()).unwrap();
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
            serde_json::to_string_pretty(&schemars::schema_for!(FermionOperator)).unwrap();
        assert_eq!(schema, rust_schema);

        let version: String =
            String::extract(new.call_method0("current_version").unwrap()).unwrap();
        let rust_version = STRUQTURE_VERSION.to_string();
        assert_eq!(version, rust_version);

        new.call_method1("add_operator_product", ("c0a0", 1.0))
            .unwrap();
        let min_version: String =
            String::extract(new.call_method0("min_supported_version").unwrap()).unwrap();
        let rust_min_version = String::from("2.0.0");
        assert_eq!(min_version, rust_min_version);
    });
}
