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
use std::cmp::Ordering;
use struqture::mixed_systems::MixedProduct;
use struqture::prelude::*;
#[cfg(feature = "json_schema")]
use struqture::STRUQTURE_VERSION;
use struqture::{
    bosons::BosonProduct, fermions::FermionProduct, mixed_systems::HermitianMixedProduct,
    prelude::MixedIndex, spins::PauliProduct,
};
use struqture_py::bosons::BosonProductWrapper;
use struqture_py::fermions::FermionProductWrapper;
use struqture_py::mixed_systems::{HermitianMixedProductWrapper, MixedProductWrapper};
use struqture_py::spins::PauliProductWrapper;

// helper functions
fn new_pp(
    py: Python,
    spin_sub: Vec<String>,
    boson_sub: Vec<String>,
    fermion_sub: Vec<String>,
) -> &PyCell<HermitianMixedProductWrapper> {
    let pp_type = py.get_type::<HermitianMixedProductWrapper>();
    pp_type
        .call1((spin_sub, boson_sub, fermion_sub))
        .unwrap()
        .downcast::<PyCell<HermitianMixedProductWrapper>>()
        .unwrap()
}

/// Test default function of HermitianMixedProductWrapper
#[test]
fn test_default_partialeq_debug_clone() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let pp = new_pp(
            py,
            vec!["0Z".into()],
            vec!["c0a1".into()],
            vec!["c0a0".into()],
        );
        let pp_wrapper = pp.extract::<HermitianMixedProductWrapper>().unwrap();

        // PartialEq
        let helper_ne: bool = HermitianMixedProductWrapper::default() != pp_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = HermitianMixedProductWrapper::default()
            == HermitianMixedProductWrapper::new(vec![], vec![], vec![]).unwrap();
        assert!(helper_eq);

        // Test PartialOrd trait
        let pp_0 = new_pp(
            py,
            vec!["0Z".into()],
            vec!["c0a1".into()],
            vec!["c0a0".into()],
        );
        let pp_wrapper_0 = pp_0.extract::<HermitianMixedProductWrapper>().unwrap();
        let pp_1 = new_pp(
            py,
            vec!["0X".into()],
            vec!["c0a1".into()],
            vec!["c0a0".into()],
        );
        let pp_wrapper_1 = pp_1.extract::<HermitianMixedProductWrapper>().unwrap();

        assert_eq!(pp_wrapper_0.partial_cmp(&pp_wrapper), Some(Ordering::Equal));
        assert_eq!(pp_wrapper.partial_cmp(&pp_wrapper_0), Some(Ordering::Equal));
        assert_eq!(pp_wrapper_1.partial_cmp(&pp_wrapper), Some(Ordering::Less));
        assert_eq!(
            pp_wrapper.partial_cmp(&pp_wrapper_1),
            Some(Ordering::Greater)
        );

        assert_eq!(pp_wrapper_0.cmp(&pp_wrapper), Ordering::Equal);
        assert_eq!(pp_wrapper.cmp(&pp_wrapper_0), Ordering::Equal);
        assert_eq!(pp_wrapper_1.cmp(&pp_wrapper), Ordering::Less);
        assert_eq!(pp_wrapper.cmp(&pp_wrapper_1), Ordering::Greater);

        // Clone
        assert_eq!(pp_wrapper.clone(), pp_wrapper);

        // Debug

        assert_eq!(
            format!("{:?}", HermitianMixedProductWrapper { internal: HermitianMixedProduct::new(vec![PauliProduct::new().z(0)], vec![BosonProduct::new([0], [1]).unwrap()], vec![FermionProduct::new([0], [0]).unwrap()]).unwrap() }),
            "HermitianMixedProductWrapper { internal: HermitianMixedProduct { spins: [PauliProduct { items: [(0, Z)] }], bosons: [BosonProduct { creators: [0], annihilators: [1] }], fermions: [FermionProduct { creators: [0], annihilators: [0] }] } }"
        );
    })
}

/// Test new function of HermitianMixedProductWrapper
#[test]
fn test_new_no_error() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pp_type = py.get_type::<HermitianMixedProductWrapper>();

        let pp = pp_type.call1((vec!["0Z"], vec!["c0a1"], vec!["c0a0"]));
        assert!(pp.is_ok());
    });
}

/// Test create_valid_pair functions of HermitianMixedProduct
#[test]
fn test_new_errors() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pp = py.get_type::<HermitianMixedProductWrapper>();

        let valid = pp.call1((
            vec!["0J"],
            vec!["c0a0"],
            vec!["c0a0"],
            Complex64::new(1.0, 2.0),
        ));
        assert!(valid.is_err());

        let valid = pp.call1((vec!["0X"], vec!["c0j1"], vec!["c0a0"]));
        assert!(valid.is_err());

        let valid = pp.call1((vec!["0X"], vec!["c0a1"], vec!["c0j0"]));
        assert!(valid.is_err());

        let valid = pp.call1((vec!["0X"], vec!["c1a0"], vec!["c0a0"]));
        assert!(valid.is_err());
    });
}

/// Test from_string function of HermitianMixedProduct
#[test]
fn test_from_string() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pp = new_pp(
            py,
            vec!["0Z".into()],
            vec!["c0a1".into()],
            vec!["c0a0".into()],
        );

        let string_pp = pp
            .call_method1("from_string", (":S0Z:Bc0a1:Fc0a0:",))
            .unwrap();
        let comparison = bool::extract(string_pp.call_method1("__eq__", (pp,)).unwrap()).unwrap();
        assert!(comparison);

        let nbr_spins = string_pp.call_method0("current_number_spins").unwrap();
        let comparison =
            bool::extract(nbr_spins.call_method1("__eq__", ([1_u64],)).unwrap()).unwrap();
        assert!(comparison);

        let nbr_spins = string_pp
            .call_method0("current_number_bosonic_modes")
            .unwrap();
        let comparison =
            bool::extract(nbr_spins.call_method1("__eq__", ([2_u64],)).unwrap()).unwrap();
        assert!(comparison);

        let nbr_spins = string_pp
            .call_method0("current_number_fermionic_modes")
            .unwrap();
        let comparison =
            bool::extract(nbr_spins.call_method1("__eq__", ([1_u64],)).unwrap()).unwrap();
        assert!(comparison);

        let comp_op = string_pp.call_method0("spins").unwrap();
        let noise_type = py.get_type::<PauliProductWrapper>();
        let spins = noise_type
            .call0()
            .unwrap()
            .downcast::<PyCell<PauliProductWrapper>>()
            .unwrap()
            .call_method1("z", ((0),))
            .unwrap();
        let comparison =
            bool::extract(comp_op.call_method1("__eq__", (vec![spins],)).unwrap()).unwrap();
        assert!(comparison);

        let comp_op = string_pp.call_method0("bosons").unwrap();
        let noise_type = py.get_type::<BosonProductWrapper>();
        let bosons = noise_type
            .call1(([0], [1]))
            .unwrap()
            .downcast::<PyCell<BosonProductWrapper>>()
            .unwrap();
        let comparison =
            bool::extract(comp_op.call_method1("__eq__", (vec![bosons],)).unwrap()).unwrap();
        assert!(comparison);

        let comp_op = string_pp.call_method0("fermions").unwrap();
        let noise_type = py.get_type::<FermionProductWrapper>();
        let fermions = noise_type
            .call1(([0], [0]))
            .unwrap()
            .downcast::<PyCell<FermionProductWrapper>>()
            .unwrap();
        let comparison =
            bool::extract(comp_op.call_method1("__eq__", (vec![fermions],)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test from_string function of HermitianMixedProduct - PyValueError
#[test]
fn test_from_string_error() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new_pp_1 = new_pp(py, vec![], vec![], vec![]);
        let error_pp = new_pp_1.call_method1("from_string", ("0X1Z3J",));
        assert!(error_pp.is_err());
    });
}

/// Test hermitian_conjugate and is_natural_hermitian functions of HermitianMixedProduct
#[test]
fn test_hermitian_conj() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pp = new_pp(
            py,
            vec!["0Z".into()],
            vec!["c0a1".into()],
            vec!["c0a0".into()],
        );
        let hermitian_conjugate_pp = pp.call_method0("hermitian_conjugate").unwrap();
        let comparison = bool::extract(
            hermitian_conjugate_pp
                .call_method1("__eq__", ((pp, 1_f64),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let is_natural_hermitian_pp =
            bool::extract(pp.call_method0("is_natural_hermitian").unwrap()).unwrap();
        assert!(!is_natural_hermitian_pp);
    });
}

/// Test create_valid_pair functions of HermitianMixedProduct
#[test]
fn test_create_valid_pair() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pp = new_pp(
            py,
            vec!["0Z".into()],
            vec!["c0a1".into()],
            vec!["c0a0".into()],
        );

        let valid = pp
            .call_method1(
                "create_valid_pair",
                (
                    vec!["0Z"],
                    vec!["c0a1"],
                    vec!["c0a0"],
                    Complex64::new(1.0, 2.0),
                ),
            )
            .unwrap();
        let comparison = bool::extract(
            valid
                .call_method1("__eq__", ((pp, Complex64::new(1.0, 2.0)),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    });
}

/// Test create_valid_pair functions of HermitianMixedProduct
#[test]
fn test_create_valid_pair_errors() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pp = new_pp(
            py,
            vec!["0Z".into()],
            vec!["c0a1".into()],
            vec!["c0a0".into()],
        );

        let valid = pp.call_method1(
            "create_valid_pair",
            (
                vec!["0J"],
                vec!["c0a1"],
                vec!["c0a0"],
                Complex64::new(1.0, 2.0),
            ),
        );
        assert!(valid.is_err());

        let valid = pp.call_method1(
            "create_valid_pair",
            (
                vec!["0X"],
                vec!["c0j1"],
                vec!["c0a0"],
                Complex64::new(1.0, 2.0),
            ),
        );
        assert!(valid.is_err());

        let valid = pp.call_method1(
            "create_valid_pair",
            (
                vec!["0X"],
                vec!["c0a1"],
                vec!["c0j0"],
                Complex64::new(1.0, 2.0),
            ),
        );
        assert!(valid.is_err());

        let valid = pp.call_method1(
            "create_valid_pair",
            (vec!["0X"], vec!["c0a1"], vec!["c0a0"], vec!["fail"]),
        );
        assert!(valid.is_err());

        let valid = pp.call_method1(
            "create_valid_pair",
            (vec!["0X"], vec!["c0a1"], vec!["c1a0"], vec!["c0a0"]),
        );
        assert!(valid.is_err());

        // let valid = pp.call_method1(
        //     "create_valid_pair",
        //     (vec!["0X"], vec!["c0a1"], vec!["c1"], Complex64::new(1.0, 0.0),),
        // );
        // assert!(valid.is_err());

        let valid = pp.call_method1(
            "create_valid_pair",
            (
                vec!["0X"],
                vec!["c0a0"],
                vec!["c0a0"],
                Complex64::new(1.0, 2.0),
            ),
        );
        assert!(valid.is_err());
    });
}

/// Test __mul__ functions of HermitianMixedProduct
#[test]
fn test_multiply() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pp_0 = new_pp(
            py,
            vec!["1X".into()],
            vec!["c0a0a1".into(), "c0a0a1".into()],
            vec!["c0a0a1".into(), "c0a0a1".into()],
        );
        let pp_1 = new_pp(
            py,
            vec!["0Z".into()],
            vec!["c0a0a1".into(), "c0a0a1".into()],
            vec!["c0a0a1".into(), "c0a0a1".into()],
        );

        // Comparison
        let creators_left: &[usize] = &[0];
        let annihilators_left: &[usize] = &[0, 1];
        let spins_left = PauliProduct::new().x(1);
        let bosons_left =
            BosonProduct::new(creators_left.to_vec(), annihilators_left.to_vec()).unwrap();
        let fermions_left =
            FermionProduct::new(creators_left.to_vec(), annihilators_left.to_vec()).unwrap();

        let creators_right: &[usize] = &[0];
        let annihilators_right: &[usize] = &[0, 1];
        let spins_right = PauliProduct::new().z(0);
        let bosons_right =
            BosonProduct::new(creators_right.to_vec(), annihilators_right.to_vec()).unwrap();
        let fermions_right =
            FermionProduct::new(creators_right.to_vec(), annihilators_right.to_vec()).unwrap();

        let mut comparison: Vec<(MixedProductWrapper, Complex64)> = Vec::new();
        let left_mp = MixedProduct::new(
            [spins_left.clone()],
            [bosons_left.clone(), bosons_right.clone()],
            [fermions_left.clone(), fermions_right.clone()],
        )
        .unwrap();
        let left_mp_conj = MixedProduct::new(
            [spins_left],
            [
                bosons_left.hermitian_conjugate().0,
                bosons_right.hermitian_conjugate().0,
            ],
            [
                fermions_left.hermitian_conjugate().0,
                fermions_right.hermitian_conjugate().0,
            ],
        )
        .unwrap();
        let right_mp = MixedProduct::new(
            [spins_right.clone()],
            [bosons_right.clone(), bosons_left.clone()],
            [fermions_right.clone(), fermions_left.clone()],
        )
        .unwrap();
        let right_mp_conj = MixedProduct::new(
            [spins_right],
            [
                bosons_right.hermitian_conjugate().0,
                bosons_left.hermitian_conjugate().0,
            ],
            [
                fermions_right.hermitian_conjugate().0,
                fermions_left.hermitian_conjugate().0,
            ],
        )
        .unwrap();
        for lhs in [left_mp, left_mp_conj] {
            for rhs in [right_mp.clone(), right_mp_conj.clone()] {
                let result = (lhs.clone() * rhs).unwrap();
                for vec in result {
                    comparison.push((MixedProductWrapper { internal: vec.0 }, vec.1));
                }
            }
        }

        let multiplied = pp_0.call_method1("__mul__", (pp_1,)).unwrap();
        let result =
            bool::extract(multiplied.call_method1("__eq__", (comparison,)).unwrap()).unwrap();
        assert!(result);

        let pp_error = new_pp(
            py,
            vec!["1X".into(), "2Z".into()],
            vec!["c2a3".into()],
            vec!["c0c1".into()],
        );
        let multiplied = pp_0.call_method1("__mul__", (pp_error,));
        assert!(multiplied.is_err());
    });
}

/// Test copy and deepcopy functions of HermitianMixedProduct
#[test]
fn test_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pp = new_pp(
            py,
            vec!["0Z".into()],
            vec!["c0a1".into()],
            vec!["c0a0".into()],
        );

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

/// Test to_bincode and from_bincode functions of HermitianMixedProduct
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pp = new_pp(
            py,
            vec!["0Z".into()],
            vec!["c0a1".into()],
            vec!["c0a0".into()],
        );

        let serialised = pp.call_method0("to_bincode").unwrap();
        let new = new_pp(
            py,
            vec!["0Z".into()],
            vec!["c0a1".into()],
            vec!["c0a0".into()],
        );
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
        let new = new_pp(
            py,
            vec!["0Z".into()],
            vec!["c0a1".into()],
            vec!["c0a0".into()],
        );
        let deserialised_error = new.call_method1("from_bincode", ("J",));
        assert!(deserialised_error.is_err());
    });
}

/// Test to_ and from_json functions of HermitianMixedProduct
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pp = new_pp(
            py,
            vec!["0Z".into()],
            vec!["c0a1".into()],
            vec!["c0a0".into()],
        );

        let serialised = pp.call_method0("to_json").unwrap();
        let new = new_pp(
            py,
            vec!["0Z".into()],
            vec!["c0a1".into()],
            vec!["c0a0".into()],
        );
        let deserialised = new.call_method1("from_json", (serialised,)).unwrap();

        let deserialised_error = new.call_method1("from_json", ("fails".to_string(),));
        assert!(deserialised_error.is_err());

        let deserialised_error = new.call_method1("from_json", (0,));
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
        let pp = new_pp(
            py,
            vec!["0Z".into()],
            vec!["c0a1".into()],
            vec!["c0a0".into()],
        );
        let format_repr = "S0Z:Bc0a1:Fc0a0:";

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
        let pp_one = new_pp(
            py,
            vec!["0Z".into()],
            vec!["c0a1".into()],
            vec!["c0a0".into()],
        );
        let pp_two = new_pp(
            py,
            vec!["1Z".into()],
            vec!["c0a1".into()],
            vec!["c0a0".into()],
        );

        let comparison = bool::extract(pp_one.call_method1("__eq__", (pp_two,)).unwrap()).unwrap();
        assert!(!comparison);
        let comparison = bool::extract(
            pp_one
                .call_method1("__eq__", ("S0Z:Bc0a1:Fc0a0:",))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let comparison = bool::extract(pp_one.call_method1("__ne__", (pp_two,)).unwrap()).unwrap();
        assert!(comparison);
        let comparison = bool::extract(
            pp_one
                .call_method1("__ne__", ("S0Z:Bc0a1:Fc0a0:",))
                .unwrap(),
        )
        .unwrap();
        assert!(!comparison);

        let comparison = pp_one.call_method1("__ge__", ("S0Z:Bc0a1:Fc0a0:",));
        assert!(comparison.is_err());
    });
}

/// Test hash functions of HermitianMixedProduct
#[test]
fn test_hash() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pp = new_pp(
            py,
            vec!["0Z".into()],
            vec!["c0a1".into()],
            vec!["c0a0".into()],
        );
        let pp_other = new_pp(
            py,
            vec!["1Z".into()],
            vec!["c0a1".into()],
            vec!["c0a0".into()],
        );

        let hash_pp = pp.call_method0("__hash__").unwrap();
        let hash_other_pp = pp_other.call_method0("__hash__").unwrap();

        let equal = bool::extract(hash_pp.call_method1("__eq__", (hash_pp,)).unwrap()).unwrap();
        assert!(equal);
        let not_equal =
            bool::extract(hash_pp.call_method1("__eq__", (hash_other_pp,)).unwrap()).unwrap();
        assert!(!not_equal);
    });
}

#[cfg(feature = "json_schema")]
#[test]
fn test_json_schema() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let new = new_pp(
            py,
            vec!["0Z".to_string()],
            vec!["c0a0".to_string()],
            vec!["c1a1".to_string()],
        );

        let schema: String = String::extract(new.call_method0("json_schema").unwrap()).unwrap();
        let rust_schema =
            serde_json::to_string_pretty(&schemars::schema_for!(HermitianMixedProduct)).unwrap();
        assert_eq!(schema, rust_schema);

        let version: String =
            String::extract(new.call_method0("current_version").unwrap()).unwrap();
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
        let pp_2 = new_pp(
            py,
            vec!["0Z".to_string()],
            vec!["c0a0".to_string()],
            vec!["c1a1".to_string()],
        );
        let pp_1: struqture_one::mixed_systems::HermitianMixedProduct =
            struqture_one::mixed_systems::MixedIndex::new(
                [struqture_one::spins::PauliProduct::from_str("0Z").unwrap()],
                [struqture_one::bosons::BosonProduct::from_str("c0a0").unwrap()],
                [struqture_one::fermions::FermionProduct::from_str("c1a1").unwrap()],
            )
            .unwrap();

        let result =
            HermitianMixedProductWrapper::from_pyany_to_struqture_one(pp_2.as_ref().into())
                .unwrap();
        assert_eq!(result, pp_1);
    });
}

#[cfg(feature = "struqture_1_import")]
#[test]
fn test_from_json_struqture_one() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let json_string: &PyAny = pyo3::types::PyString::new(py, "\"S0Z:Bc1a1:Fc0a0\"").into();
        let pp_2 = new_pp(
            py,
            vec!["0Z".to_string()],
            vec!["c1a1".to_string()],
            vec!["c0a0".to_string()],
        );

        let pp_from_1 = pp_2
            .call_method1("from_json_struqture_one", (json_string,))
            .unwrap();
        let equal = bool::extract(pp_2.call_method1("__eq__", (pp_from_1,)).unwrap()).unwrap();
        assert!(equal);

        let error_json_string: &PyAny =
            pyo3::types::PyString::new(py, "\"S0Z:Bc1a1:Fc0b0\"").into();
        let pp_from_1 = pp_2.call_method1("from_json_struqture_one", (error_json_string,));
        assert!(pp_from_1.is_err());
    });
}
