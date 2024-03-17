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
use pyo3::types::IntoPyDict;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use std::cmp::Ordering;
#[cfg(feature = "json_schema")]
use struqture::{bosons::BosonProduct, STRUQTURE_VERSION};
use struqture_py::bosons::BosonProductWrapper;

// helper functions
fn new_pp(
    py: Python,
    creators: Vec<usize>,
    annihilators: Vec<usize>,
) -> Bound<BosonProductWrapper> {
    let pp_type = py.get_type::<BosonProductWrapper>();
    pp_type
        .call1((creators, annihilators))
        .unwrap()
        .downcast::<BosonProductWrapper>()
        .unwrap()
        .to_owned()
}

/// Test default function of BosonProductWrapper
#[test]
fn test_default_partialeq_debug_clone() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let pp = new_pp(py, vec![0, 1], vec![1, 2]);
        let pp_wrapper = pp.extract::<BosonProductWrapper>().unwrap();

        // PartialEq
        let helper_ne: bool = BosonProductWrapper::default() != pp_wrapper;
        assert!(helper_ne);
        let helper_eq: bool =
            BosonProductWrapper::default() == BosonProductWrapper::new(vec![], vec![]).unwrap();
        assert!(helper_eq);

        // Test PartialOrd trait
        let pp_0 = new_pp(py, vec![0, 1], vec![1, 2]);
        let pp_wrapper_0 = pp_0.extract::<BosonProductWrapper>().unwrap();
        let pp_1 = new_pp(py, vec![0, 1], vec![1, 3]);
        let pp_wrapper_1 = pp_1.extract::<BosonProductWrapper>().unwrap();

        assert_eq!(pp_wrapper_0.partial_cmp(&pp_wrapper), Some(Ordering::Equal));
        assert_eq!(pp_wrapper.partial_cmp(&pp_wrapper_0), Some(Ordering::Equal));
        assert_eq!(
            pp_wrapper_1.partial_cmp(&pp_wrapper),
            Some(Ordering::Greater)
        );
        assert_eq!(pp_wrapper.partial_cmp(&pp_wrapper_1), Some(Ordering::Less));

        assert_eq!(pp_wrapper_0.cmp(&pp_wrapper), Ordering::Equal);
        assert_eq!(pp_wrapper.cmp(&pp_wrapper_0), Ordering::Equal);
        assert_eq!(pp_wrapper_1.cmp(&pp_wrapper), Ordering::Greater);
        assert_eq!(pp_wrapper.cmp(&pp_wrapper_1), Ordering::Less);

        // Clone
        assert_eq!(pp_wrapper.clone(), pp_wrapper);

        // Debug

        assert_eq!(
            format!("{:?}", BosonProductWrapper::new(vec![0, 1], vec![1, 2]).unwrap()),
            "BosonProductWrapper { internal: BosonProduct { creators: [0, 1], annihilators: [1, 2] } }"
        );
    })
}

/// Test new function of BosonProductWrapper
#[test]
fn test_new_no_error() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pp_type = py.get_type::<BosonProductWrapper>();
        let pp = pp_type.call1(([0_u64, 1_u64], [1_u64, 1_u64]));
        assert!(pp.is_ok());
    });
}

/// Test from_string function of BosonProduct
#[test]
fn test_from_string() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pp = new_pp(py, vec![0, 1], vec![1, 2]);

        let string_pp = pp.call_method1("from_string", ("c0c1a1a2",)).unwrap();
        let comparison =
            bool::extract_bound(&string_pp.call_method1("__eq__", (pp,)).unwrap()).unwrap();
        assert!(comparison);

        let nbr_spins = string_pp.call_method0("number_modes").unwrap();
        let comparison =
            bool::extract_bound(&nbr_spins.call_method1("__eq__", (3_u64,)).unwrap()).unwrap();
        assert!(comparison);

        let nbr_spins = string_pp.call_method0("number_creators").unwrap();
        let comparison =
            bool::extract_bound(&nbr_spins.call_method1("__eq__", (2_u64,)).unwrap()).unwrap();
        assert!(comparison);

        let nbr_spins = string_pp.call_method0("number_annihilators").unwrap();
        let comparison =
            bool::extract_bound(&nbr_spins.call_method1("__eq__", (2_u64,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test from_string function of BosonProduct - PyValueError
#[test]
fn test_from_string_error() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_pp_1 = new_pp(py, vec![0, 1], vec![1, 2]);
        let error_pp = new_pp_1.call_method1("from_string", ("0X1Z3J",));
        assert!(error_pp.is_err());
    });
}

/// Test creators and annihilators functions of BosonProduct
#[test]
fn test_creators_annihilators_create_valid_pair() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pp = new_pp(py, vec![0, 1], vec![1, 2]);

        let valid = pp
            .call_method1(
                "create_valid_pair",
                (
                    vec![0_u64, 1_u64],
                    vec![1_u64, 2_u64],
                    Complex64::new(1.0, 2.0),
                ),
            )
            .unwrap();
        let comparison = bool::extract_bound(
            &valid
                .call_method1("__eq__", ((&pp, Complex64::new(1.0, 2.0)),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let nbr_spins = pp.call_method0("number_modes").unwrap();
        let comparison =
            bool::extract_bound(&nbr_spins.call_method1("__eq__", (3_u64,)).unwrap()).unwrap();
        assert!(comparison);

        let nbr_spins = pp.call_method0("number_creators").unwrap();
        let comparison =
            bool::extract_bound(&nbr_spins.call_method1("__eq__", (2_u64,)).unwrap()).unwrap();
        assert!(comparison);

        let nbr_spins = pp.call_method0("number_annihilators").unwrap();
        let comparison =
            bool::extract_bound(&nbr_spins.call_method1("__eq__", (2_u64,)).unwrap()).unwrap();
        assert!(comparison);

        let nbr_spins = pp.call_method0("creators").unwrap();
        let comparison =
            bool::extract_bound(&nbr_spins.call_method1("__eq__", (vec![0, 1],)).unwrap()).unwrap();
        assert!(comparison);

        let nbr_spins = pp.call_method0("annihilators").unwrap();
        let comparison =
            bool::extract_bound(&nbr_spins.call_method1("__eq__", (vec![1, 2],)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test remap_modes function of BosonProduct
#[test]
fn test_remap_modes() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let bp = new_pp(py, vec![0, 1], vec![]);
        let remapped_bp = new_pp(py, vec![2, 3], vec![]);
        let expected_coeff = CalculatorComplexWrapper {
            internal: 1.0.into(),
        };
        let remap_dict = [(0, 3), (1, 2), (2, 0), (3, 1)].into_py_dict(py).unwrap();
        let results = bp.call_method1("remap_modes", (remap_dict,)).unwrap();
        let comparison = bool::extract_bound(
            &results
                .call_method1("__eq__", ((remapped_bp, expected_coeff),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    });
}

/// Test hermitian_conjugate and is_natural_hermitian functions of BosonProduct
#[test]
fn test_hermitian_conj() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pp = new_pp(py, vec![0, 1], vec![1, 2]);
        let conjugated_pp = new_pp(py, vec![1, 2], vec![0, 1]);

        let hermitian_conjugate_pp = pp.call_method0("hermitian_conjugate").unwrap();
        let comparison = bool::extract_bound(
            &hermitian_conjugate_pp
                .call_method1("__eq__", ((conjugated_pp, 1_f64),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let is_natural_hermitian_pp =
            bool::extract_bound(&pp.call_method0("is_natural_hermitian").unwrap()).unwrap();
        assert!(!is_natural_hermitian_pp);
    });
}

/// Test concatenate functions of BosonProduct
#[test]
fn test_multiply() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pp_0 = new_pp(py, vec![84, 95], vec![10, 20]);
        let pp_1 = new_pp(py, vec![10], vec![43, 78]);
        let pp_mul_1 = new_pp(py, vec![84, 95], vec![20, 43, 78]);
        let pp_mul_2 = new_pp(py, vec![10, 84, 95], vec![10, 20, 43, 78]);

        let multiplied = pp_0.call_method1("__mul__", (pp_1,)).unwrap();
        let comparison = bool::extract_bound(
            &multiplied
                .call_method1("__eq__", (vec![pp_mul_1, pp_mul_2],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    });
}

/// Test copy and deepcopy functions of BosonProduct
#[test]
fn test_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pp = new_pp(py, vec![0, 1], vec![1, 2]);

        let copy_pp = pp.call_method0("__copy__").unwrap();
        let deepcopy_pp = pp.call_method1("__deepcopy__", ("",)).unwrap();
        // let copy_deepcopy_param = pp.clone();

        let comparison_copy =
            bool::extract_bound(&copy_pp.call_method1("__eq__", (&pp,)).unwrap()).unwrap();
        assert!(comparison_copy);
        let comparison_deepcopy =
            bool::extract_bound(&deepcopy_pp.call_method1("__eq__", (pp,)).unwrap()).unwrap();
        assert!(comparison_deepcopy);
    });
}

/// Test to_bincode and from_bincode functions of BosonProduct
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pp = new_pp(py, vec![0, 1], vec![1, 2]);

        let serialised = pp.call_method0("to_bincode").unwrap();
        let new = new_pp(py, vec![0, 1], vec![1, 2]);
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
            bool::extract_bound(&deserialised.call_method1("__eq__", (pp,)).unwrap()).unwrap();
        assert!(comparison)
    });
}

#[test]
fn test_value_error_bincode() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new = new_pp(py, vec![0, 1], vec![1, 2]);
        let deserialised_error = new.call_method1("from_bincode", ("J",));
        assert!(deserialised_error.is_err());
    });
}

/// Test to_ and from_json functions of BosonProduct
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pp = new_pp(py, vec![0, 1], vec![1, 2]);

        let serialised = pp.call_method0("to_json").unwrap();
        let new = new_pp(py, vec![0, 1], vec![1, 2]);
        let deserialised = new.call_method1("from_json", (&serialised,)).unwrap();

        let deserialised_error = new.call_method1("from_json", ("fails".to_string(),));
        assert!(deserialised_error.is_err());

        let deserialised_error = new.call_method1("from_json", (0,));
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_json");
        assert!(serialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_json");
        assert!(deserialised_error.is_err());

        let comparison =
            bool::extract_bound(&deserialised.call_method1("__eq__", (pp,)).unwrap()).unwrap();
        assert!(comparison)
    });
}

/// Test the __repr__ and __format__ functions
#[test]
fn test_format_repr() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pp = new_pp(py, vec![0, 1], vec![1, 2]);
        let format_repr = "c0c1a1a2";

        let to_str = pp.call_method0("__str__").unwrap();
        let str_op: String = String::extract_bound(&to_str).unwrap();

        let to_format = pp.call_method1("__format__", ("",)).unwrap();
        let format_op: String = String::extract_bound(&to_format).unwrap();

        let to_repr = pp.call_method0("__repr__").unwrap();
        let repr_op: String = String::extract_bound(&to_repr).unwrap();

        assert_eq!(str_op, format_repr);
        assert_eq!(format_op, format_repr);
        assert_eq!(repr_op, format_repr);
    });
}

/// Test the __richcmp__ function
#[test]
fn test_richcmp() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pp_one = new_pp(py, vec![0, 1], vec![1, 2]);
        let pp_two = new_pp(py, vec![1, 2], vec![1, 2]);

        let comparison =
            bool::extract_bound(&pp_one.call_method1("__eq__", (&pp_two,)).unwrap()).unwrap();
        assert!(!comparison);
        let comparison =
            bool::extract_bound(&pp_one.call_method1("__eq__", ("c0c1a1a2",)).unwrap()).unwrap();
        assert!(comparison);

        let comparison =
            bool::extract_bound(&pp_one.call_method1("__ne__", (pp_two,)).unwrap()).unwrap();
        assert!(comparison);
        let comparison =
            bool::extract_bound(&pp_one.call_method1("__ne__", ("c0c1a1a2",)).unwrap()).unwrap();
        assert!(!comparison);

        let comparison = pp_one.call_method1("__ge__", ("c0c1a1a2",));
        assert!(comparison.is_err());
    });
}

/// Test hash functions of BosonProduct
#[test]
fn test_hash() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pp = new_pp(py, vec![0, 1], vec![1, 2]);
        let pp_other = new_pp(py, vec![1, 2], vec![1, 2]);

        let hash_pp = pp.call_method0("__hash__").unwrap();
        let hash_other_pp = pp_other.call_method0("__hash__").unwrap();

        let equal =
            bool::extract_bound(&hash_pp.call_method1("__eq__", (&hash_pp,)).unwrap()).unwrap();
        assert!(equal);
        let not_equal =
            bool::extract_bound(&hash_pp.call_method1("__eq__", (hash_other_pp,)).unwrap())
                .unwrap();
        assert!(!not_equal);
    });
}

#[cfg(feature = "json_schema")]
#[test]
fn test_json_schema() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new = new_pp(py, vec![0], vec![0]);

        let schema: String =
            String::extract_bound(&new.call_method0("json_schema").unwrap()).unwrap();
        let rust_schema =
            serde_json::to_string_pretty(&schemars::schema_for!(BosonProduct)).unwrap();
        assert_eq!(schema, rust_schema);

        let version: String =
            String::extract_bound(&new.call_method0("current_version").unwrap()).unwrap();
        let rust_version = STRUQTURE_VERSION.to_string();
        assert_eq!(version, rust_version);

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
        let pp_2 = new_pp(py, vec![0, 1], vec![1, 2]);

        let result =
            BosonProductWrapper::from_pyany_to_struqture_one(pp_2.as_ref().into()).unwrap();
        assert_eq!(
            result,
            struqture_one::bosons::BosonProduct::from_str("c0c1a1a2").unwrap()
        );
    });
}

#[cfg(feature = "struqture_1_import")]
#[test]
fn test_from_json_struqture_one() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let json_string: &PyAny = pyo3::types::PyString::new(py, "\"c0a1\"").into();
        let pp_2 = new_pp(py, vec![0], vec![1]);

        let pp_from_1 = pp_2
            .call_method1("from_json_struqture_one", (json_string,))
            .unwrap();
        let equal = bool::extract(pp_2.call_method1("__eq__", (pp_from_1,)).unwrap()).unwrap();
        assert!(equal);

        let error_json_string: &PyAny = pyo3::types::PyString::new(py, "\"c0b1\"").into();
        let pp_from_1 = pp_2.call_method1("from_json_struqture_one", (error_json_string,));
        assert!(pp_from_1.is_err());
    });
}
