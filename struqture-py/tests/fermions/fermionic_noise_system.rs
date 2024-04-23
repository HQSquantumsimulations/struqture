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
#[cfg(feature = "json_schema")]
use struqture::{fermions::FermionLindbladNoiseSystem, STRUQTURE_VERSION};
use struqture_py::fermions::{FermionLindbladNoiseSystemWrapper, FermionProductWrapper};
use test_case::test_case;

// helper functions
fn new_noisesystem(py: Python) -> Bound<FermionLindbladNoiseSystemWrapper> {
    let system_type = py.get_type_bound::<FermionLindbladNoiseSystemWrapper>();
    let number_modes: Option<usize> = None;
    system_type
        .call1((number_modes,))
        .unwrap()
        .downcast::<FermionLindbladNoiseSystemWrapper>()
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

/// Test default function of FermionLindbladNoiseSystemWrapper
#[test]
fn test_default_partialeq_debug_clone() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let system_type = py.get_type_bound::<FermionLindbladNoiseSystemWrapper>();
        let binding = system_type.call1((4_usize,)).unwrap();
        let new_system = binding
            .downcast::<FermionLindbladNoiseSystemWrapper>()
            .unwrap();
        let system_wrapper = new_system
            .extract::<FermionLindbladNoiseSystemWrapper>()
            .unwrap();

        // PartialEq
        let helper_ne: bool = FermionLindbladNoiseSystemWrapper::default() != system_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = FermionLindbladNoiseSystemWrapper::default()
            == FermionLindbladNoiseSystemWrapper::new(None);
        assert!(helper_eq);

        // Clone
        assert_eq!(system_wrapper.clone(), system_wrapper);

        // Debug
        assert_eq!(
            format!("{:?}", FermionLindbladNoiseSystemWrapper::new(None)),
            "FermionLindbladNoiseSystemWrapper { internal: FermionLindbladNoiseSystem { number_modes: None, operator: FermionLindbladNoiseOperator { internal_map: {} } } }"
        );

        // Number of fermions
        let comp_op = new_system.call_method0("number_modes").unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (4,)).unwrap()).unwrap();
        assert!(comparison);

        let comp_op = new_system.call_method0("current_number_modes").unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (4,)).unwrap()).unwrap();
        assert!(comparison);
    })
}

/// Test number_modes and current_number_modes functions of FermionSystem
#[test]
fn test_number_modes_current() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_noisesystem(py);
        system
            .call_method1("add_operator_product", (("c0a0", "c0a0"), 0.1))
            .unwrap();

        let number_system = system.call_method0("number_modes").unwrap();
        let current_system = system.call_method0("current_number_modes").unwrap();

        let comparison =
            bool::extract_bound(&number_system.call_method1("__eq__", (1_u64,)).unwrap()).unwrap();
        assert!(comparison);
        let comparison =
            bool::extract_bound(&current_system.call_method1("__eq__", (1_u64,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test empty_clone function of FermionSystem
#[test]
fn test_empty_clone() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_modes: Option<usize> = None;
        let system = new_noisesystem(py);
        let none_system = system.call_method1("empty_clone", (number_modes,)).unwrap();
        let comparison =
            bool::extract_bound(&none_system.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison);

        let number_modes: Option<usize> = Some(3);
        let system = new_noisesystem(py);
        let some_system = system.call_method1("empty_clone", (number_modes,)).unwrap();
        let comparison =
            bool::extract_bound(&some_system.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add_operator_product and remove functions of FermionSystem
#[test]
fn fermion_system_test_add_operator_product_remove() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_system = py.get_type_bound::<FermionLindbladNoiseSystemWrapper>();
        let number_modes: Option<usize> = Some(4);
        let binding = new_system.call1((number_modes,)).unwrap();
        let system = binding
            .downcast::<FermionLindbladNoiseSystemWrapper>()
            .unwrap();
        system
            .call_method1("add_operator_product", (("c0a0", "c0a0"), 0.1))
            .unwrap();
        system
            .call_method1("add_operator_product", (("c0a0", "c0a1"), 0.2))
            .unwrap();
        system
            .call_method1("add_operator_product", (("c0a0", "c3"), 0.05))
            .unwrap();

        // test access at index 0
        let comp_op = system.call_method1("get", (("c0a0", "c0a0"),)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.1,)).unwrap()).unwrap();
        assert!(comparison);
        system.call_method1("remove", (("c0a0", "c0a0"),)).unwrap();
        let comp_op = system.call_method1("get", (("c0a0", "c0a0"),)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system.call_method1("get", (("c0a0", "c0a1"),)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.2,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system.call_method1("get", (("c0a0", "c3"),)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.05,)).unwrap()).unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system.call_method1("get", (("c0a0", "c2"),)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
        assert!(comparison);

        // Get error
        let error = system.call_method1("get", (("j2", "c0a0"),));
        assert!(error.is_err());

        // Try_set error 1: Key (PauliProduct) cannot be converted from string
        let error = system.call_method1("add_operator_product", (("j1", "c0a0"), 0.5));
        assert!(error.is_err());

        // Try_set error 2: Value cannot be converted to CalculatorComplex
        let error = system.call_method1("add_operator_product", (("c0a0", "c0a1"), vec![0.0]));
        assert!(error.is_err());

        // Try_set error 3: Number of fermions in entry exceeds number of fermions in system.
        let error = system.call_method1("add_operator_product", (("c0a0", "c5"), 0.1));
        assert!(error.is_err());

        // Try_set error 4: Generic error
        let error = system.call_method1("add_operator_product", (("c0a0", "j1"), 0.5));
        assert!(error.is_err());
    });
}

/// Test keys function of FermionSystem
#[test]
fn test_keys_values() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_noisesystem(py);

        let len_system = system.call_method0("__len__").unwrap();
        let comparison =
            bool::extract_bound(&len_system.call_method1("__eq__", (0_u64,)).unwrap()).unwrap();
        assert!(comparison);
        let empty_system = bool::extract_bound(&system.call_method0("is_empty").unwrap()).unwrap();
        assert!(empty_system);

        system
            .call_method1("add_operator_product", (("c0a0", "c0a0"), 0.1))
            .unwrap();

        let keys_system = system.call_method0("keys").unwrap();
        let comparison = bool::extract_bound(
            &keys_system
                .call_method1("__eq__", (vec![("c0a0", "c0a0")],))
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
#[test_case(0.0,1.0;"imag")]
#[test_case(0.7,0.7;"mixed")]
fn test_truncate(re: f64, im: f64) {
    pyo3::Python::with_gil(|py| {
        let system = new_noisesystem(py);
        system
            .call_method1(
                "add_operator_product",
                (
                    ("c0a0", "c0a0"),
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
                    ("a1", "a1"),
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
                    ("c0a0", "c2a2"),
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
                    ("c0c1a0a2", "c0c1a0a2"),
                    CalculatorFloatWrapper {
                        internal: CalculatorFloat::from("test"),
                    },
                ),
            )
            .unwrap();

        let test_system1 = new_noisesystem(py);
        test_system1
            .call_method1(
                "add_operator_product",
                (
                    ("c0a0", "c0a0"),
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
                    ("a1", "a1"),
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
                    ("c0c1a0a2", "c0c1a0a2"),
                    CalculatorFloatWrapper {
                        internal: CalculatorFloat::from("test"),
                    },
                ),
            )
            .unwrap();

        let test_system2 = new_noisesystem(py);
        test_system2
            .call_method1(
                "add_operator_product",
                (
                    ("c0a0", "c0a0"),
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
                    ("c0c1a0a2", "c0c1a0a2"),
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

#[test]
fn test_separate() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pmp = new_noisesystem(py);
        pmp.call_method1("add_operator_product", (("c0a0", "c0a0"), 1.0))
            .unwrap();
        pmp.call_method1("add_operator_product", (("c0c1a0", "c0a0"), 1.0))
            .unwrap();
        pmp.call_method1("add_operator_product", (("c0c1a0", "c0c1a0"), 1.0))
            .unwrap();
        pmp.call_method1("add_operator_product", (("c0c2a0", "c0c1a0"), 1.0))
            .unwrap();

        let pmp_rem = new_noisesystem(py);
        pmp_rem
            .call_method1("add_operator_product", (("c0a0", "c0a0"), 1.0))
            .unwrap();
        pmp_rem
            .call_method1("add_operator_product", (("c0c1a0", "c0a0"), 1.0))
            .unwrap();

        let pmp_sys = new_noisesystem(py);
        pmp_sys
            .call_method1("add_operator_product", (("c0c1a0", "c0c1a0"), 1.0))
            .unwrap();
        pmp_sys
            .call_method1("add_operator_product", (("c0c2a0", "c0c1a0"), 1.0))
            .unwrap();

        let result = pmp
            .call_method1("separate_into_n_terms", ((2, 1), (2, 1)))
            .unwrap();
        let equal = bool::extract_bound(
            &result
                .call_method1("__eq__", ((pmp_sys, pmp_rem),))
                .unwrap(),
        )
        .unwrap();
        assert!(equal);
    })
}

/// Test add magic method function of FermionSystem
#[test]
fn test_neg() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_noisesystem(py);
        system_0
            .call_method1("add_operator_product", (("c0a0", "c0a0"), 0.1))
            .unwrap();
        let system_1 = new_noisesystem(py);
        system_1
            .call_method1("add_operator_product", (("c0a0", "c0a0"), -0.1))
            .unwrap();

        let negated = system_0.call_method0("__neg__").unwrap();
        let comparison =
            bool::extract_bound(&negated.call_method1("__eq__", (system_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of FermionSystem
#[test]
fn test_add() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_noisesystem(py);
        system_0
            .call_method1("add_operator_product", (("c0a0", "c0a0"), 0.1))
            .unwrap();
        let system_1 = new_noisesystem(py);
        system_1
            .call_method1("add_operator_product", (("c0a1", "c0a0"), 0.2))
            .unwrap();
        let system_0_1 = new_noisesystem(py);
        system_0_1
            .call_method1("add_operator_product", (("c0a0", "c0a0"), 0.1))
            .unwrap();
        system_0_1
            .call_method1("add_operator_product", (("c0a1", "c0a0"), 0.2))
            .unwrap();

        let added = system_0.call_method1("__add__", (system_1,)).unwrap();
        let comparison =
            bool::extract_bound(&added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of FermionSystem
#[test]
fn test_sub() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_noisesystem(py);
        system_0
            .call_method1("add_operator_product", (("c0a0", "c0a0"), 0.1))
            .unwrap();
        let system_1 = new_noisesystem(py);
        system_1
            .call_method1("add_operator_product", (("c0a1", "c0a0"), 0.2))
            .unwrap();
        let system_0_1 = new_noisesystem(py);
        system_0_1
            .call_method1("add_operator_product", (("c0a0", "c0a0"), 0.1))
            .unwrap();
        system_0_1
            .call_method1("add_operator_product", (("c0a1", "c0a0"), -0.2))
            .unwrap();

        let added = system_0.call_method1("__sub__", (system_1,)).unwrap();
        let comparison =
            bool::extract_bound(&added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of FermionSystem
#[test]
fn test_mul_cf() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_noisesystem(py);
        system_0
            .call_method1("add_operator_product", (("c0a0", "c0a0"), 0.1_f64))
            .unwrap();

        let system_0_1 = new_noisesystem(py);
        system_0_1
            .call_method1("add_operator_product", (("c0a0", "c0a0"), 0.2))
            .unwrap();

        let added = system_0.call_method1("__mul__", (2.0,)).unwrap();
        let comparison =
            bool::extract_bound(&added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of FermionSystem
#[test]
fn test_mul_cc() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_noisesystem(py);
        system_0
            .call_method1("add_operator_product", (("c0a0", "c0a0"), 0.1_f64))
            .unwrap();

        let system_0_1 = new_noisesystem(py);
        system_0_1
            .call_method1(
                "add_operator_product",
                (("c0a0", "c0a0"), Complex64::new(0.0, 0.5)),
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

/// Test copy and deepcopy functions of FermionLindbladNoiseSystem
#[test]
fn test_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_noisesystem(py);
        system
            .call_method1(
                "add_operator_product",
                (
                    ("c0a0", "c0a0"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
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

/// Test to_bincode and from_bincode functions of FermionLindbladNoiseSystem
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_noisesystem(py);
        system
            .call_method1(
                "add_operator_product",
                (
                    ("c0a0", "c0a0"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();

        let serialised = system.call_method0("to_bincode").unwrap();
        let new = new_noisesystem(py);
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
        let new = new_noisesystem(py);
        let deserialised_error = new.call_method1("from_bincode", ("J",));
        assert!(deserialised_error.is_err());
    });
}

/// Test to_ and from_json functions of FermionLindbladNoiseSystem
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_noisesystem(py);
        system
            .call_method1(
                "add_operator_product",
                (
                    ("c0a0", "c0a0"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();

        let serialised = system.call_method0("to_json").unwrap();
        let new = new_noisesystem(py);
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
        let system = new_noisesystem(py);
        system
            .call_method1(
                "add_operator_product",
                (
                    ("c0a0", "c0a0"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        let mut rust_system = FermionLindbladNoiseSystemWrapper::new(None);
        let pp_type = py.get_type_bound::<FermionProductWrapper>();
        let binding = pp_type.call1(([0], [0])).unwrap();
        let new_pp = binding.downcast::<FermionProductWrapper>().unwrap();

        rust_system
            .add_operator_product(
                (new_pp.as_gil_ref().into(), new_pp.as_gil_ref().into()),
                &convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
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
            "FermionLindbladNoiseSystem(1){\n(c0a0, c0a0): (1e-1 + i * 0e0),\n}".to_string()
        );
        assert_eq!(
            repr_op,
            "FermionLindbladNoiseSystem(1){\n(c0a0, c0a0): (1e-1 + i * 0e0),\n}".to_string()
        );
        assert_eq!(
            str_op,
            "FermionLindbladNoiseSystem(1){\n(c0a0, c0a0): (1e-1 + i * 0e0),\n}".to_string()
        );
    });
}

/// Test keys function of FermionLindbladNoiseSystem
#[test]
fn test_keys_noise() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_noisesystem(py);
        system
            .call_method1(
                "add_operator_product",
                (
                    ("c0a0", "c0a0"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();

        let keys_system = system.call_method0("keys").unwrap();
        let comparison = bool::extract_bound(
            &keys_system
                .call_method1("__eq__", (vec![("c0a0".to_string(), "c0a0".to_string())],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    });
}

/// Test the __richcmp__ function
#[test]
fn test_richcmp() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_one = new_noisesystem(py);
        system_one
            .call_method1(
                "add_operator_product",
                (
                    ("c1", "c0a1"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        let system_two = new_noisesystem(py);
        system_two
            .call_method1(
                "add_operator_product",
                (
                    ("c0a0", "c0a1"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();

        let comparison =
            bool::extract_bound(&system_one.call_method1("__eq__", (&system_two,)).unwrap())
                .unwrap();
        assert!(!comparison);
        let comparison =
            bool::extract_bound(&system_one.call_method1("__eq__", ("c0a0",)).unwrap()).unwrap();
        assert!(!comparison);

        let comparison =
            bool::extract_bound(&system_one.call_method1("__ne__", (system_two,)).unwrap())
                .unwrap();
        assert!(comparison);
        let comparison =
            bool::extract_bound(&system_one.call_method1("__ne__", ("c0a0",)).unwrap()).unwrap();
        assert!(comparison);

        let comparison = system_one.call_method1("__ge__", ("c0a0",));
        assert!(comparison.is_err());
    });
}

/// Test jordan_wigner() method of FermionLindbladNoiseSystem
#[test]
fn test_jordan_wigner() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let flns = new_noisesystem(py);
        flns.call_method1("add_operator_product", (("c1", "c0a1"), 0.1))
            .unwrap();
        let slns = flns.call_method0("jordan_wigner").unwrap();

        let empty = bool::extract_bound(&slns.call_method0("is_empty").unwrap()).unwrap();
        assert!(!empty);

        let number_modes =
            usize::extract_bound(&flns.call_method0("current_number_modes").unwrap()).unwrap();
        let number_spins =
            usize::extract_bound(&slns.call_method0("current_number_spins").unwrap()).unwrap();
        assert_eq!(number_modes, number_spins)
    });
}

#[cfg(feature = "json_schema")]
#[test]
fn test_json_schema() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new = new_noisesystem(py);

        let schema: String =
            String::extract_bound(&new.call_method0("json_schema").unwrap()).unwrap();
        let rust_schema =
            serde_json::to_string_pretty(&schemars::schema_for!(FermionLindbladNoiseSystem))
                .unwrap();
        assert_eq!(schema, rust_schema);

        let version: String =
            String::extract_bound(&new.call_method0("current_version").unwrap()).unwrap();
        let rust_version = STRUQTURE_VERSION.to_string();
        assert_eq!(version, rust_version);

        new.call_method1("add_operator_product", (("c0a0", "c0a0"), 1.0))
            .unwrap();
        let min_version: String =
            String::extract_bound(&new.call_method0("min_supported_version").unwrap()).unwrap();
        let rust_min_version = String::from("1.0.0");
        assert_eq!(min_version, rust_min_version);
    });
}
