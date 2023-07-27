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
use std::{cmp::Ordering, collections::HashMap};
#[cfg(feature = "json_schema")]
use struqture::{spins::DecoherenceProduct, STRUQTURE_VERSION};
use struqture_py::spins::DecoherenceProductWrapper;

// helper functions
fn new_pp(py: Python) -> &PyCell<DecoherenceProductWrapper> {
    let pp_type = py.get_type::<DecoherenceProductWrapper>();
    pp_type
        .call0()
        .unwrap()
        .downcast::<PyCell<DecoherenceProductWrapper>>()
        .unwrap()
}

/// Test default function of DecoherenceProductWrapper
#[test]
fn test_default_partialeq_debug_clone() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let pp = new_pp(py);
        let pp_new = pp.call_method1("set_pauli", (0, "X")).unwrap();
        let pp_wrapper = pp_new.extract::<DecoherenceProductWrapper>().unwrap();

        // PartialEq
        let helper_ne: bool = DecoherenceProductWrapper::default() != pp_wrapper;
        assert!(helper_ne);
        let helper_eq: bool =
            DecoherenceProductWrapper::default() == DecoherenceProductWrapper::new();
        assert!(helper_eq);

        // Test PartialOrd trait
        let pp_0 = new_pp(py);
        let new_pp_0 = pp_0.call_method1("set_pauli", (0, "X")).unwrap();
        let pp_wrapper_0 = new_pp_0.extract::<DecoherenceProductWrapper>().unwrap();
        let pp_1 = new_pp(py);
        let new_pp_1 = pp_1.call_method1("set_pauli", (0, "Z")).unwrap();
        let pp_wrapper_1 = new_pp_1.extract::<DecoherenceProductWrapper>().unwrap();

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
            format!("{:?}", DecoherenceProductWrapper::new()),
            "DecoherenceProductWrapper { internal: DecoherenceProduct { items: [] } }"
        );
    })
}

/// Test from_string function of DecoherenceProduct
#[test]
fn test_from_string() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_pp_1 = new_pp(py);
        let mut pp = new_pp_1.call_method1("set_pauli", (0, "X")).unwrap();
        pp = pp.call_method1("set_pauli", (1, "Z")).unwrap();
        pp = pp.call_method1("set_pauli", (3, "iY")).unwrap();

        let new_pp_1 = new_pp(py);
        let string_pp = new_pp_1.call_method1("from_string", ("0X1Z3iY",)).unwrap();
        let comparison = bool::extract(string_pp.call_method1("__eq__", (pp,)).unwrap()).unwrap();
        assert!(comparison);

        let nbr_spins = string_pp.call_method0("current_number_spins").unwrap();
        let comparison = bool::extract(nbr_spins.call_method1("__eq__", (4,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test from_string function of DecoherenceProduct - PyValueError
#[test]
fn test_from_string_error() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_pp_1 = new_pp(py);
        let error_pp = new_pp_1.call_method1("from_string", ("0X1Z3J2Z",)); // PROBLEM 0X1Z3J gets translated, without the last term - ask!
        assert!(error_pp.is_err());
    });
}

/// Test set_pauli and get functions of DecoherenceProduct
#[test]
fn test_set_pauli_get() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_pp_1 = new_pp(py);
        let mut pp = new_pp_1.call_method1("set_pauli", (0, "X")).unwrap();
        pp = pp.call_method1("set_pauli", (1, "Z")).unwrap();
        pp = pp.call_method1("set_pauli", (3, "iY")).unwrap();

        // test access at index 0
        let comp_op = pp.call_method1("get", (0_u64,)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", ("X",)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = pp.call_method1("get", (1_u64,)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", ("Z",)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = pp.call_method1("get", (3_u64,)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", ("iY",)).unwrap()).unwrap();
        assert!(comparison);

        // test setting new operation at index 1
        pp = pp.call_method1("set_pauli", (1_u64, "X")).unwrap();

        let comp_op = pp.call_method1("get", (1_u64,)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", ("X",)).unwrap()).unwrap();
        assert!(comparison);

        let comparison = pp.call_method1("get", (20_u64,)).unwrap();
        assert!(comparison.is_none());
    });
}

/// Test set_pauli function of DecoherenceProduct - error
#[test]
fn test_set_pauli_error() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_pp_1 = new_pp(py);

        let comparison = new_pp_1.call_method1("set_pauli", (0, "J"));
        assert!(comparison.is_err());
    });
}

/// Test x, iy, z functions of DecoherenceProduct
#[test]
fn test_x_iy_z() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_pp_1 = new_pp(py);
        let mut pp = new_pp_1.call_method1("set_pauli", (0, "X")).unwrap();
        pp = pp.call_method1("set_pauli", (1, "Z")).unwrap();
        pp = pp.call_method1("set_pauli", (3, "iY")).unwrap();

        let new_pp_1 = new_pp(py);
        let mut pp_1 = new_pp_1.call_method1("x", (0,)).unwrap();
        pp_1 = pp_1.call_method1("z", (1,)).unwrap();
        pp_1 = pp_1.call_method1("iy", (3,)).unwrap();
        let comparison = bool::extract(pp_1.call_method1("__eq__", (pp,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test keys function of DecoherenceProduct
#[test]
fn test_keys_len_is_empty() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_pp_1 = new_pp(py);

        let is_empty_1 = bool::extract(new_pp_1.call_method0("is_empty").unwrap()).unwrap();
        assert!(is_empty_1);
        let len_1 = new_pp_1.call_method0("__len__").unwrap();
        let comparison = bool::extract(len_1.call_method1("__eq__", (0_u64,)).unwrap()).unwrap();
        assert!(comparison);

        let mut pp = new_pp_1.call_method1("set_pauli", (0, "X")).unwrap();
        pp = pp.call_method1("set_pauli", (1, "Z")).unwrap();
        pp = pp.call_method1("set_pauli", (3, "iY")).unwrap();

        let is_empty_2 = bool::extract(pp.call_method0("is_empty").unwrap()).unwrap();
        assert!(!is_empty_2);
        let len_2 = pp.call_method0("__len__").unwrap();
        let comparison = bool::extract(len_2.call_method1("__eq__", (3_u64,)).unwrap()).unwrap();
        assert!(comparison);

        let keys_pp = pp.call_method0("keys").unwrap();
        let comparison =
            bool::extract(keys_pp.call_method1("__eq__", (vec![0, 1, 3],)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test hermitian_conjugate and is_natural_hermitian functions of PauliProduct
#[test]
fn test_hermitian_conj() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_pp_1 = new_pp(py);
        let mut pp = new_pp_1.call_method1("set_pauli", (0_u64, "X")).unwrap();
        pp = pp.call_method1("set_pauli", (1_u64, "Z")).unwrap();
        pp = pp.call_method1("set_pauli", (3_u64, "iY")).unwrap();

        let hermitian_conjugate_pp = pp.call_method0("hermitian_conjugate").unwrap();
        let comparison = bool::extract(
            hermitian_conjugate_pp
                .call_method1("__eq__", ((pp, -1_f64),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let is_natural_hermitian_pp =
            bool::extract(pp.call_method0("is_natural_hermitian").unwrap()).unwrap();
        assert!(!is_natural_hermitian_pp);
    });
}

/// Test remap_qubits functions of PauliProduct
#[test]
fn test_remap_qubits() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_pp_1 = new_pp(py);
        let mut pp = new_pp_1.call_method1("set_pauli", (0_u64, "X")).unwrap();
        pp = pp.call_method1("set_pauli", (1_u64, "Z")).unwrap();
        pp = pp.call_method1("set_pauli", (3_u64, "iY")).unwrap();

        let mut mapping: HashMap<usize, usize> = HashMap::new();
        mapping.insert(0, 1);
        mapping.insert(1, 3);
        mapping.insert(3, 0);
        let mut remapped_pp = new_pp_1.call_method1("set_pauli", (1_u64, "X")).unwrap();
        remapped_pp = remapped_pp.call_method1("set_pauli", (3_u64, "Z")).unwrap();
        remapped_pp = remapped_pp
            .call_method1("set_pauli", (0_u64, "iY"))
            .unwrap();

        let remapping_pp = pp.call_method1("remap_qubits", (mapping,)).unwrap();
        let comparison =
            bool::extract(remapping_pp.call_method1("__eq__", (remapped_pp,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test concatenate functions of PauliProduct
#[test]
fn test_concatenate() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_pp_1 = new_pp(py);
        let mut pp_0 = new_pp_1.call_method1("set_pauli", (0_u64, "X")).unwrap();
        pp_0 = pp_0.call_method1("set_pauli", (1_u64, "Z")).unwrap();
        pp_0 = pp_0.call_method1("set_pauli", (3_u64, "iY")).unwrap();
        let mut pp_1 = new_pp_1.call_method1("set_pauli", (2_u64, "X")).unwrap();
        pp_1 = pp_1.call_method1("set_pauli", (4_u64, "Z")).unwrap();

        let mut pp_0_1 = new_pp_1.call_method1("set_pauli", (0_u64, "X")).unwrap();
        pp_0_1 = pp_0_1.call_method1("set_pauli", (1_u64, "Z")).unwrap();
        pp_0_1 = pp_0_1.call_method1("set_pauli", (3_u64, "iY")).unwrap();
        pp_0_1 = pp_0_1.call_method1("set_pauli", (2_u64, "X")).unwrap();
        pp_0_1 = pp_0_1.call_method1("set_pauli", (4_u64, "Z")).unwrap();

        let concatenated_pp = pp_0.call_method1("concatenate", (pp_1,)).unwrap();
        let comparison =
            bool::extract(concatenated_pp.call_method1("__eq__", (pp_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test concatenate functions of PauliProduct
#[test]
fn test_concatenate_error() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_pp_1 = new_pp(py);
        let mut pp_0 = new_pp_1.call_method1("set_pauli", (0_u64, "X")).unwrap();
        pp_0 = pp_0.call_method1("set_pauli", (1_u64, "Z")).unwrap();
        pp_0 = pp_0.call_method1("set_pauli", (3_u64, "iY")).unwrap();
        let pp_1 = new_pp_1.call_method1("set_pauli", (0_u64, "X")).unwrap();

        let concatenated_pp = pp_0.call_method1("concatenate", (pp_1,));
        assert!(concatenated_pp.is_err());
    });
}

/// Test concatenate functions of PauliProduct
#[test]
fn test_multiply() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_pp_1 = new_pp(py);
        let mut pp_0 = new_pp_1.call_method1("set_pauli", (0_u64, "X")).unwrap();
        pp_0 = pp_0.call_method1("set_pauli", (1_u64, "Z")).unwrap();
        pp_0 = pp_0.call_method1("set_pauli", (3_u64, "iY")).unwrap();
        let mut pp_1 = new_pp_1.call_method1("set_pauli", (0_u64, "Z")).unwrap();
        pp_1 = pp_1.call_method1("set_pauli", (4_u64, "Z")).unwrap();

        let mut pp_0_1 = new_pp_1.call_method1("set_pauli", (0_u64, "iY")).unwrap();
        pp_0_1 = pp_0_1.call_method1("set_pauli", (1_u64, "Z")).unwrap();
        pp_0_1 = pp_0_1.call_method1("set_pauli", (3_u64, "iY")).unwrap();
        pp_0_1 = pp_0_1.call_method1("set_pauli", (4_u64, "Z")).unwrap();

        let new = new_pp(py);
        let multiplied = new.call_method1("multiply", (pp_0, pp_1)).unwrap();
        let comparison = bool::extract(
            multiplied
                .call_method1("__eq__", ((pp_0_1, Complex64::new(-1.0, 0.0)),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    });
}

/// Test copy and deepcopy functions of DecoherenceProduct
#[test]
fn test_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_pp = new_pp(py);
        let pp = new_pp.call_method1("set_pauli", (0, "X")).unwrap();

        let copy_pp = pp.call_method0("__copy__").unwrap();
        let deepcopy_pp = pp.call_method1("__deepcopy__", ("",)).unwrap();
        // let copy_deepcopy_param = pp.clone();

        let comparison_copy =
            bool::extract(copy_pp.call_method1("__eq__", (pp,)).unwrap()).unwrap();
        assert!(comparison_copy);
        let comparison_deepcopy =
            bool::extract(deepcopy_pp.call_method1("__eq__", (pp,)).unwrap()).unwrap();
        assert!(comparison_deepcopy);
    });
}

/// Test to_bincode and from_bincode functions of DecoherenceProduct
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_pp_1 = new_pp(py);
        let pp = new_pp_1.call_method1("set_pauli", (0, "X")).unwrap();

        let serialised = pp.call_method0("to_bincode").unwrap();
        let new = new_pp(py);
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
            bool::extract(deserialised.call_method1("__eq__", (pp,)).unwrap()).unwrap();
        assert!(comparison)
    });
}

#[test]
fn test_value_error_bincode() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new = new_pp(py);
        let deserialised_error = new.call_method1("from_bincode", ("J",));
        assert!(deserialised_error.is_err());
    });
}

/// Test to_ and from_json functions of DecoherenceProduct
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_pp_1 = new_pp(py);
        let pp = new_pp_1.call_method1("set_pauli", (0, "X")).unwrap();

        let serialised = pp.call_method0("to_json").unwrap();
        let new = new_pp(py);
        let deserialised = new.call_method1("from_json", (serialised,)).unwrap();

        let pp_type = py.get_type::<DecoherenceProductWrapper>();
        let deserialised_error = pp_type.call_method1("from_json", ("fails".to_string(),));
        assert!(deserialised_error.is_err());

        let deserialised_error = pp_type.call_method1("from_json", ("S3".to_string(),));
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_json");
        assert!(serialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_json");
        assert!(deserialised_error.is_err());

        let comparison =
            bool::extract(deserialised.call_method1("__eq__", (pp,)).unwrap()).unwrap();
        assert!(comparison)
    });
}

/// Test the __repr__ and __format__ functions
#[test]
fn test_format_repr() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_pp = new_pp(py);
        let pp = new_pp.call_method1("set_pauli", (0, "X")).unwrap();
        let format_repr = "0X";

        let to_str = pp.call_method0("__str__").unwrap();
        let str_op: &str = <&str>::extract(to_str).unwrap();

        let to_format = pp.call_method1("__format__", ("",)).unwrap();
        let format_op: &str = <&str>::extract(to_format).unwrap();

        let to_repr = pp.call_method0("__repr__").unwrap();
        let repr_op: &str = <&str>::extract(to_repr).unwrap();

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
        let new_pp_1 = new_pp(py);
        let pp_one = new_pp_1.call_method1("set_pauli", (0, "X")).unwrap();
        let new_pp_1 = new_pp(py);
        let pp_two = new_pp_1.call_method1("set_pauli", (1, "X")).unwrap();

        let comparison = bool::extract(pp_one.call_method1("__eq__", (pp_two,)).unwrap()).unwrap();
        assert!(!comparison);
        let comparison = bool::extract(pp_one.call_method1("__eq__", ("0X",)).unwrap()).unwrap();
        assert!(comparison);

        let comparison = bool::extract(pp_one.call_method1("__ne__", (pp_two,)).unwrap()).unwrap();
        assert!(comparison);
        let comparison = bool::extract(pp_one.call_method1("__ne__", ("0X",)).unwrap()).unwrap();
        assert!(!comparison);

        let comparison = pp_one.call_method1("__ge__", ("0X",));
        assert!(comparison.is_err());
    });
}

/// Test hash functions of PauliProduct
#[test]
fn test_hash() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_pp = new_pp(py);
        let pp = new_pp.call_method1("set_pauli", (0_u64, "X")).unwrap();
        let pp_other = new_pp.call_method1("set_pauli", (1_u64, "Z")).unwrap();

        let hash_pp = pp.call_method0("__hash__").unwrap();
        let hash_other_pp = pp_other.call_method0("__hash__").unwrap();

        let equal = bool::extract(hash_pp.call_method1("__eq__", (hash_pp,)).unwrap()).unwrap();
        assert!(equal);
        let not_equal =
            bool::extract(hash_pp.call_method1("__eq__", (hash_other_pp,)).unwrap()).unwrap();
        assert!(!not_equal);
    });
}

/// Test jordan_wigner() method of DecoherenceProduct
#[test]
fn test_jordan_wigner() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_dp = new_pp(py);
        let dp = new_dp.call_method1("set_pauli", (0_u64, "iY")).unwrap();
        let fo = dp.call_method0("jordan_wigner").unwrap();

        let empty = bool::extract(fo.call_method0("is_empty").unwrap()).unwrap();
        assert!(!empty);

        let number_modes = usize::extract(fo.call_method0("number_modes").unwrap()).unwrap();
        let number_spins =
            usize::extract(dp.call_method0("current_number_spins").unwrap()).unwrap();
        assert_eq!(number_modes, number_spins)
    });
}

#[cfg(feature = "json_schema")]
#[test]
fn test_json_schema() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new = new_pp(py);

        let schema: String = String::extract(new.call_method0("json_schema").unwrap()).unwrap();
        let rust_schema =
            serde_json::to_string_pretty(&schemars::schema_for!(DecoherenceProduct)).unwrap();
        assert_eq!(schema, rust_schema);

        let version: String =
            String::extract(new.call_method0("current_version").unwrap()).unwrap();
        let rust_version = STRUQTURE_VERSION.to_string();
        assert_eq!(version, rust_version);

        let pp = new.call_method1("set_pauli", (0_u64, "Z")).unwrap();
        let min_version: String =
            String::extract(pp.call_method0("min_supported_version").unwrap()).unwrap();
        let rust_min_version = String::from("1.0.0");
        assert_eq!(min_version, rust_min_version);
    });
}
