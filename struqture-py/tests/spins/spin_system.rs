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
use struqture::spins::{PauliProduct, SpinSystem};
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture::{OperateOnDensityMatrix, SpinIndex};
use struqture_py::spins::SpinSystemWrapper;
use test_case::test_case;

// helper functions
fn new_system(py: Python, number_spins: Option<usize>) -> &PyCell<SpinSystemWrapper> {
    let system_type = py.get_type::<SpinSystemWrapper>();
    system_type
        .call1((number_spins,))
        .unwrap()
        .downcast::<PyCell<SpinSystemWrapper>>()
        .unwrap()
}

/// Test default function of SpinSystemWrapper
#[test]
fn test_default_partialeq_debug_clone() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let number_spins: Option<usize> = None;
        let new_system = new_system(py, number_spins);
        new_system
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();
        let system_wrapper = new_system.extract::<SpinSystemWrapper>().unwrap();

        // PartialEq
        let helper_ne: bool = SpinSystemWrapper::new(None) != system_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = SpinSystemWrapper::new(None) == SpinSystemWrapper::new(None);
        assert!(helper_eq);

        // Clone
        assert_eq!(system_wrapper.clone(), system_wrapper);

        // Debug
        assert_eq!(
            format!("{:?}", SpinSystemWrapper::new(None)),
            "SpinSystemWrapper { internal: SpinSystem { number_spins: None, operator: SpinOperator { internal_map: {} } } }"
        );

        // Number of spins
        let comp_op = new_system.call_method0("number_spins").unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (1,)).unwrap()).unwrap();
        assert!(comparison);
    })
}

/// Test number_spins and current_number_spins functions of SpinSystem
#[test]
fn test_number_spins_current() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Option<usize> = None;
        let system = new_system(py, number_spins);
        system
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();

        let number_system = system.call_method0("number_spins").unwrap();
        let current_system = system.call_method0("current_number_spins").unwrap();

        let comparison =
            bool::extract(number_system.call_method1("__eq__", (1_u64,)).unwrap()).unwrap();
        assert!(comparison);
        let comparison =
            bool::extract(current_system.call_method1("__eq__", (1_u64,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test empty_clone function of SpinSystem
#[test]
fn test_empty_clone() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Option<usize> = None;
        let system = new_system(py, number_spins);
        let none_system = system.call_method1("empty_clone", (number_spins,)).unwrap();
        let comparison =
            bool::extract(none_system.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison);

        let number_spins: Option<usize> = Some(3);
        let system = new_system(py, number_spins);
        let some_system = system.call_method1("empty_clone", (number_spins,)).unwrap();
        let comparison =
            bool::extract(some_system.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test hermitian_conjugate function of SpinSystem
#[test]
fn test_hermitian_conj() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Option<usize> = None;
        let system = new_system(py, number_spins);
        system
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();

        let conjugate = system.call_method0("hermitian_conjugate").unwrap();
        let comparison =
            bool::extract(conjugate.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test set and get functions of SpinSystem
#[test]
fn spin_system_test_set_get() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type::<SpinSystemWrapper>();
        let number_spins: Option<usize> = Some(4);
        let system = new_system
            .call1((number_spins,))
            .unwrap()
            .downcast::<PyCell<SpinSystemWrapper>>()
            .unwrap();
        system.call_method1("set", ("0X", 0.1)).unwrap();
        system.call_method1("set", ("1Z", 0.2)).unwrap();
        system.call_method1("set", ("3Y", 0.05)).unwrap();

        // test access at index 0
        let comp_op = system.call_method1("get", ("0X",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.1,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system.call_method1("get", ("1Z",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.2,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system.call_method1("get", ("3Y",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.05,)).unwrap()).unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system.call_method1("get", ("2X",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
        assert!(comparison);

        // Try_set error 1: Key (PauliProduct) cannot be converted from string
        let error = system.call_method1("set", ("1J", 0.5));
        assert!(error.is_err());

        // Try_set error 2: Value cannot be converted to CalculatorComplex
        let error = system.call_method1("set", ("1Z", vec![0.0]));
        assert!(error.is_err());

        // Try_set error 3: Number of spins in entry exceeds number of spins in system.
        let error = system.call_method1("set", ("5Z", 0.1));
        assert!(error.is_err());

        // Try_set error 4: Generic error
        let error = system.call_method1("set", ("1J", 0.5));
        assert!(error.is_err());
    });
}

/// Test add_operator_product and remove functions of SpinSystem
#[test]
fn spin_system_test_add_operator_product_remove() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type::<SpinSystemWrapper>();
        let number_spins: Option<usize> = Some(4);
        let system = new_system
            .call1((number_spins,))
            .unwrap()
            .downcast::<PyCell<SpinSystemWrapper>>()
            .unwrap();
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
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.1,)).unwrap()).unwrap();
        assert!(comparison);
        system.call_method1("remove", ("0X",)).unwrap();
        let comp_op = system.call_method1("get", ("0X",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system.call_method1("get", ("1Z",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.2,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system.call_method1("get", ("3Y",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.05,)).unwrap()).unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system.call_method1("get", ("2X",)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
        assert!(comparison);

        // Get error
        let error = system.call_method1("get", ("2J",));
        assert!(error.is_err());

        // Try_set error 1: Key (PauliProduct) cannot be converted from string
        let error = system.call_method1("add_operator_product", ("1J", 0.5));
        assert!(error.is_err());

        // Try_set error 2: Value cannot be converted to CalculatorComplex
        let error = system.call_method1("add_operator_product", ("1Z", vec![0.0]));
        assert!(error.is_err());

        // Try_set error 3: Number of spins in entry exceeds number of spins in system.
        let error = system.call_method1("add_operator_product", ("5Z", 0.1));
        assert!(error.is_err());

        // Try_set error 4: Generic error
        let error = system.call_method1("add_operator_product", ("1J", 0.5));
        assert!(error.is_err());
    });
}

/// Test keys function of SpinSystem
#[test]
fn test_keys_values() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Option<usize> = None;
        let system = new_system(py, number_spins);

        let len_system = system.call_method0("__len__").unwrap();
        let comparison =
            bool::extract(len_system.call_method1("__eq__", (0_u64,)).unwrap()).unwrap();
        assert!(comparison);
        let empty_system = bool::extract(system.call_method0("is_empty").unwrap()).unwrap();
        assert!(empty_system);

        system
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();

        let keys_system = system.call_method0("keys").unwrap();
        let comparison =
            bool::extract(keys_system.call_method1("__eq__", (vec!["0X"],)).unwrap()).unwrap();
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
        let number_spins: Option<usize> = None;

        let system = new_system(py, number_spins);
        system
            .call_method1(
                "add_operator_product",
                (
                    "0X",
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
                    "1Y",
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
                    "2Z",
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
                    "0X1Z",
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::from("test"),
                    },
                ),
            )
            .unwrap();

        let test_system1 = new_system(py, number_spins);
        test_system1
            .call_method1(
                "add_operator_product",
                (
                    "0X",
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
                    "1Y",
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
                    "0X1Z",
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::from("test"),
                    },
                ),
            )
            .unwrap();

        let test_system2 = new_system(py, number_spins);
        test_system2
            .call_method1(
                "add_operator_product",
                (
                    "0X",
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
                    "0X1Z",
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
        let pmp = new_system(py, None);
        pmp.call_method1("add_operator_product", ("0Z", 1.0))
            .unwrap();
        pmp.call_method1("add_operator_product", ("0Z1Z", 1.0))
            .unwrap();
        pmp.call_method1("add_operator_product", ("0Z1X", 1.0))
            .unwrap();

        let pmp_rem = new_system(py, None);
        pmp_rem
            .call_method1("add_operator_product", ("0Z", 1.0))
            .unwrap();

        let pmp_sys = new_system(py, None);
        pmp_sys
            .call_method1("add_operator_product", ("0Z1Z", 1.0))
            .unwrap();
        pmp_sys
            .call_method1("add_operator_product", ("0Z1X", 1.0))
            .unwrap();

        let result = pmp.call_method1("separate_into_n_terms", (2,)).unwrap();
        let equal = bool::extract(
            result
                .call_method1("__eq__", ((pmp_sys, pmp_rem),))
                .unwrap(),
        )
        .unwrap();
        assert!(equal);
    })
}

/// Test add magic method function of SpinSystem
#[test]
fn test_neg() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Option<usize> = Some(2);
        let system_0 = new_system(py, number_spins);
        system_0
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();
        let system_1 = new_system(py, number_spins);
        system_1
            .call_method1("add_operator_product", ("0X", -0.1))
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
        let number_spins: Option<usize> = Some(2);
        let system_0 = new_system(py, number_spins);
        system_0
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();
        let system_1 = new_system(py, number_spins);
        system_1
            .call_method1("add_operator_product", ("1Z", 0.2))
            .unwrap();
        let system_0_1 = new_system(py, number_spins);
        system_0_1
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();
        system_0_1
            .call_method1("add_operator_product", ("1Z", 0.2))
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
        let number_spins: Option<usize> = Some(2);
        let system_0 = new_system(py, number_spins);
        system_0
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();
        let system_1 = new_system(py, number_spins);
        system_1
            .call_method1("add_operator_product", ("1Z", 0.2))
            .unwrap();
        let system_0_1 = new_system(py, number_spins);
        system_0_1
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();
        system_0_1
            .call_method1("add_operator_product", ("1Z", -0.2))
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
        let number_spins: Option<usize> = Some(2);
        let system_0 = new_system(py, number_spins);
        system_0
            .call_method1("add_operator_product", ("0X", 0.1_f64))
            .unwrap();

        let system_0_1 = new_system(py, number_spins);
        system_0_1
            .call_method1("add_operator_product", ("0X", 0.2))
            .unwrap();

        let added = system_0.call_method1("__mul__", (2.0,)).unwrap();
        let comparison =
            bool::extract(added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of SpinSystem
#[test]
fn test_mul_cc() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Option<usize> = Some(2);
        let system_0 = new_system(py, number_spins);
        system_0
            .call_method1("add_operator_product", ("0X", 0.1_f64))
            .unwrap();

        let system_0_1 = new_system(py, number_spins);
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
            bool::extract(added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of SpinSystem
#[test]
fn test_mul_self() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Option<usize> = Some(2);
        let system_0 = new_system(py, number_spins);
        system_0
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();
        let system_1 = new_system(py, number_spins);
        system_1
            .call_method1("add_operator_product", ("1Z", 1.0))
            .unwrap();
        let system_0_1 = new_system(py, number_spins);
        system_0_1
            .call_method1("add_operator_product", ("0X1Z", 0.1))
            .unwrap();

        let added = system_0.call_method1("__mul__", (system_1,)).unwrap();
        let comparison =
            bool::extract(added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of SpinSystem
#[test]
fn test_mul_error() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Option<usize> = Some(2);
        let system_0 = new_system(py, number_spins);
        system_0
            .call_method1("add_operator_product", ("0X", 0.1_f64))
            .unwrap();

        let added = system_0.call_method1("__mul__", (vec![0.0],));
        assert!(added.is_err());
    });
}

// /// Test test_sparse_lindblad_entries function of SpinSystem
// #[test]
// fn test_sparse_lindblad_entries() {
//     pyo3::prepare_freethreaded_python();
//     let gil = pyo3::Python::acquire_gil();
//     let py = gil.python();
//     let number_spins: Option<usize> = Some(1);
//     let system = new_system(py, number_spins);
//     system
//         .call_method1(
//             "add_operator_product",
//             ("0Z", 1.0_f64),
//         )
//         .unwrap();

//     let matrices = system.call_method0("sparse_lindblad_entries",).unwrap();
//     let res: Vec<(PyCooMatrix, PyCooMatrix, Complex64)> = vec![((array![].to_pyarray(py).to_owned(), (array![].to_pyarray(py).to_owned(), array![].to_pyarray(py).to_owned())), (array![].to_pyarray(py).to_owned(), (array![].to_pyarray(py).to_owned(), array![].to_pyarray(py).to_owned())), Complex64::from(0.0))];
//     // let to_operators_op = matrices
//     //     .downcast::<Vec<(PyCooMatrix, PyCooMatrix, Complex64)>>()
//     //     .unwrap();
//     let comparison =
//         bool::extract(matrices.call_method1("__eq__", (res,)).unwrap()).unwrap();
//     assert!(comparison);
// }

// #[test]
// fn test_unitary_sparse_matrix_coo() {
//     pyo3::prepare_freethreaded_python();
//     let gil = pyo3::Python::acquire_gil();
//     let py = gil.python();
//     let number_spins: Option<usize> = Some(1);
//     let system = new_system(py, number_spins);
//     system
//         .call_method1(
//             "add_operator_product",
//             ("0Z", 1.0_f64),
//         )
//         .unwrap();

//     let matrices = system.call_method0("unitary_sparse_matrix_coo").unwrap();
//     let result_matrix = matrices
//         .downcast::<Internal>()
//         .unwrap();

//     let test_matrix: Internal = (vec![CalculatorComplexWrapper {internal: 1.0.into()}, CalculatorComplexWrapper {internal: (-1.0).into()}], (vec![0, 1], vec![0, 1]));
//     // assert_eq!(result_matrix, test_matrix);
// }

/// Test copy and deepcopy functions of SpinSystem
#[test]
fn test_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Option<usize> = None;
        let system = new_system(py, number_spins);
        system
            .call_method1("add_operator_product", ("0X", 0.1))
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

/// Test to_bincode and from_bincode functions of SpinSystem
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Option<usize> = None;
        let system = new_system(py, number_spins);
        system
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();

        let serialised = system.call_method0("to_bincode").unwrap();
        let new = new_system(py, number_spins);
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
        let number_spins: Option<usize> = None;
        let new = new_system(py, number_spins);
        let deserialised_error = new.call_method1("from_bincode", ("J",));
        assert!(deserialised_error.is_err());
    });
}

/// Test to_ and from_json functions of SpinSystem
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Option<usize> = None;
        let system = new_system(py, number_spins);
        system
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();

        let serialised = system.call_method0("to_json").unwrap();
        let new = new_system(py, number_spins);
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
        let number_spins: Option<usize> = None;
        let system = new_system(py, number_spins);
        system
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();
        let mut rust_system = SpinSystem::new(None);
        rust_system
            .add_operator_product(PauliProduct::new().x(0), CalculatorComplex::new(0.1, 0.0))
            .unwrap();
        let to_format = system.call_method1("__format__", ("",)).unwrap();
        let format_op: &str = <&str>::extract(to_format).unwrap();

        let to_repr = system.call_method0("__repr__").unwrap();
        let repr_op: &str = <&str>::extract(to_repr).unwrap();

        let to_str = system.call_method0("__str__").unwrap();
        let str_op: &str = <&str>::extract(to_str).unwrap();

        assert_eq!(
            format_op,
            "SpinSystem(1){\n0X: (1e-1 + i * 0e0),\n}".to_string()
        );
        assert_eq!(
            repr_op,
            "SpinSystem(1){\n0X: (1e-1 + i * 0e0),\n}".to_string()
        );
        assert_eq!(
            str_op,
            "SpinSystem(1){\n0X: (1e-1 + i * 0e0),\n}".to_string()
        );
    });
}

/// Test the __richcmp__ function
#[test]
fn test_richcmp() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Option<usize> = None;
        let system_one = new_system(py, number_spins);
        system_one
            .call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();
        let system_two = new_system(py, number_spins);
        system_two
            .call_method1("add_operator_product", ("1X", 0.1))
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

/// Test jordan_wigner() method of SpinSystem
#[test]
fn test_jordan_wigner() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let ss = new_system(py, Some(1));
        ss.call_method1("add_operator_product", ("0X", 0.1))
            .unwrap();
        let fs = ss.call_method0("jordan_wigner").unwrap();

        let empty = bool::extract(fs.call_method0("is_empty").unwrap()).unwrap();
        assert!(!empty);

        let number_modes = usize::extract(fs.call_method0("number_modes").unwrap()).unwrap();
        let number_spins =
            usize::extract(ss.call_method0("current_number_spins").unwrap()).unwrap();
        assert_eq!(number_modes, number_spins)
    });
}

#[cfg(feature = "json_schema")]
#[test]
fn test_json_schema() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new = new_system(py, None);

        let schema: String = String::extract(new.call_method0("json_schema").unwrap()).unwrap();
        let rust_schema = serde_json::to_string_pretty(&schemars::schema_for!(SpinSystem)).unwrap();
        assert_eq!(schema, rust_schema);

        let version: String =
            String::extract(new.call_method0("current_version").unwrap()).unwrap();
        let rust_version = STRUQTURE_VERSION.to_string();
        assert_eq!(version, rust_version);

        new.call_method1("add_operator_product", ("0Z", 1.0))
            .unwrap();
        let min_version: String =
            String::extract(new.call_method0("min_supported_version").unwrap()).unwrap();
        let rust_min_version = String::from("1.0.0");
        assert_eq!(min_version, rust_min_version);
    });
}
