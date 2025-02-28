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

#[cfg(feature = "doc_generator")]
use pyo3::{
    types::{PyAnyMethods, PyDict, PyDictMethods, PyModule},
    PyResult, Python,
};

#[cfg(feature = "doc_generator")]
fn str_to_type(res: &str, class_name: &str) -> Option<String> {
    let replacements = [
        ("uint", "int"),
        ("self", class_name),
        ("list", "List"),
        ("dict", "Dict"),
        ("tuple", "Tuple"),
        ("set", "Set"),
        ("circuit", "Circuit"),
        ("Option[", "Optional["),
        ("optional", "Optional"),
        ("operation", "Operation"),
        ("CalculatorFloat", "Union[float, int, str]"),
        ("CalculatorComplex", "Union[float, int, str, complex]"),
        ("Product type", "ProductType"),
        ("System type", "SystemType"),
        ("Noise type", "NoiseType"),
        ("np.", "numpy."),
    ];

    let mut result = res.to_owned();
    for (old, new) in replacements {
        result = result.replace(old, new);
    }
    Some(result)
}

#[cfg(feature = "doc_generator")]
fn extract_type(string: &str, class_name: &str) -> Option<String> {
    use regex::Regex;

    let pattern = r"\(([a-zA-Z_\[\] ,|]+?)\)";
    let re = Regex::new(pattern).unwrap();
    if let Some(captures) = re.captures(string) {
        if let Some(res) = captures.get(1).map(|s| s.as_str()) {
            return str_to_type(res, class_name);
        }
    }
    None
}

#[cfg(feature = "doc_generator")]
fn collect_args_from_doc(doc: &str, class_name: &str) -> Vec<String> {
    let args_vec: Vec<_> = doc
        .split('\n')
        .skip_while(|&line| line.trim() != "Args:")
        .skip(1)
        .skip_while(|line| line.is_empty())
        .take_while(|line| !line.is_empty())
        .collect();
    args_vec
        .iter()
        .filter(|&line| line.contains(':') && line.trim().starts_with(char::is_alphabetic))
        .map(|&line| {
            let arg_type = extract_type(line, class_name);
            format!(
                "{}{}",
                line.trim().split_once([' ', ':']).unwrap_or(("", "")).0,
                arg_type
                    .map(|arg_type| format!(": {}", arg_type))
                    .unwrap_or_default()
            )
        })
        .collect()
}

#[cfg(feature = "doc_generator")]
fn collect_return_from_doc(doc: &str, class_name: &str) -> String {
    let args_vec: Vec<_> = doc
        .split('\n')
        .skip_while(|&line| line.trim() != "Returns:")
        .skip(1)
        .take(1)
        .filter(|&line| line.contains(':') && line.trim().starts_with(char::is_alphabetic))
        .collect();
    if args_vec.is_empty() {
        "".to_owned()
    } else if let Some(ret) = str_to_type(
        args_vec[0].trim().split_once([':']).unwrap_or(("", "")).0,
        class_name,
    ) {
        format!(" -> {}", ret)
    } else {
        "".to_owned()
    }
}

#[cfg(feature = "doc_generator")]
const TYPING_POTENTIAL_IMPORTS: &[&str] =
    &["Optional", "List", "Tuple", "Dict", "Set", "Union", "Any"];

#[cfg(feature = "doc_generator")]
fn create_doc(module: &str) -> PyResult<String> {
    let mut main_doc = "".to_owned();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<String> {
        let python_module = PyModule::import(py, module)?;
        let dict = python_module.as_ref().getattr("__dict__")?;
        let module_doc = python_module
            .as_ref()
            .getattr("__doc__")?
            .extract::<String>()?;
        let r_dict = dict.downcast::<PyDict>()?;
        for (fn_name, func) in r_dict.iter() {
            let name = fn_name.str()?.extract::<String>()?;
            if name.starts_with("__") {
                continue;
            }
            let doc = func.getattr("__doc__")?.extract::<String>()?;
            let args = collect_args_from_doc(doc.as_str(), name.as_str()).join(", ");
            main_doc.push_str(&format!(
                    "class {name}{}:\n    \"\"\"\n{doc}\n\"\"\"\n\n    def __init__(self{}):\n       return\n\n",
                    if name.contains("Product") { "(ProductType)"} else if name.contains("System") { "(SystemType)"} else if name.contains("Noise") { "(NoiseType)"} else { "" },
                    if args.is_empty() { "".to_owned() } else { format!(", {}", args) },
                ));
            let class_dict = func.getattr("__dict__")?;
            let items = class_dict.call_method0("items")?;
            let dict_obj = py.import("builtins")?.call_method1("dict", (items,))?;
            let class_r_dict = dict_obj.as_ref().downcast::<PyDict>()?;
            for (class_fn_name, meth) in class_r_dict.iter() {
                let meth_name = class_fn_name.str()?.extract::<String>()?;
                let meth_doc = if meth_name.as_str().starts_with("__") {
                    continue;
                } else {
                    let tmp_doc = meth
                        .getattr("__doc__")?
                        .extract::<String>()
                        .unwrap_or_default();
                    if tmp_doc.starts_with("staticmethod(function) -> method") {
                        meth.getattr("__func__")?
                            .getattr("__doc__")?
                            .extract::<String>()
                            .unwrap_or_default()
                    } else {
                        tmp_doc
                    }
                };
                let meth_args = collect_args_from_doc(meth_doc.as_str(), name.as_str()).join(", ");
                main_doc.push_str(&format!(
                        "    def {meth_name}(self{}){}: # type: ignore\n        \"\"\"\n{meth_doc}\n\"\"\"\n\n",
                        if meth_args.is_empty() { "".to_owned() } else { format!(", {}", meth_args) },
                        collect_return_from_doc(
                            meth_doc.as_str(),
                            name.as_str(),
                        )
                    ));
            }
        }
        let typing_imports: Vec<&str> = TYPING_POTENTIAL_IMPORTS
            .iter()
            .filter(|&type_str| main_doc.contains(&type_str.to_string()))
            .copied()
            .collect();
        Ok(
            format!("# This is an auto generated file containing only the documentation.\n# You can find the full implementation on this page:\n# https://github.com/HQSquantumsimulations/struqture\n\n\"\"\"\n{}\n\"\"\"\n\nfrom .struqture_py import ProductType, SystemType, NoiseType\n{}{}{}\n{}",
                module_doc,
                if main_doc.lines().any(|line| line.contains("numpy") && !line.contains("import")) { "import numpy\n" } else { "" },
                if typing_imports.is_empty() { "".to_owned() } else {format!("from typing import {}\n", typing_imports.join(", "))},
                if module.eq("struqture_py.mixed_systems") { "from .bosons import *\nfrom .fermions import *\nfrom .spins import *\n" } else { "" },
                main_doc
            ),
        )
    })
}

fn main() {
    pyo3_build_config::add_extension_module_link_args();
    #[cfg(feature = "doc_generator")]
    {
        for &module in ["spins", "mixed_systems", "fermions", "bosons"].iter() {
            let qoqo_doc = create_doc(&format!("struqture_py.{module}"))
                .expect("Could not generate documentation.");
            let out_dir = std::path::PathBuf::from(format!("struqture_py/{}.pyi", module));
            std::fs::write(&out_dir, qoqo_doc).expect("Could not write to file");
        }
    }
}
