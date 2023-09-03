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
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

pub mod bosons;
pub mod fermions;
pub mod mixed_systems;
pub mod spins;

use thiserror::Error;

/// Errors that can occur in roqoqo.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum StruqturePyError {
    /// Error when remapping qubits fails because qubit in operation is not in keys of BTreeMap.
    #[error("Failed to convert input to PauliProduct")]
    ConversionError,
}

/// Struqture python interface
///
/// `HQS Quantum Simulations <https://quantumsimulations.de>`_ package for representing physical operators.
///
/// Copyright © 2021-2023 HQS Quantum Simulations GmbH. All Rights Reserved.
///
/// .. autosummary::
///     :toctree: generated/
///
///     bosons
///     fermions
///     mixed_systems
///     spins
///
#[pymodule]
fn struqture_py(_py: Python, module: &PyModule) -> PyResult<()> {
    // pyo3_log::init();
    let wrapper1 = wrap_pymodule!(spins::spins);
    module.add_wrapped(wrapper1)?;

    let wrapper2 = wrap_pymodule!(fermions::fermions);
    module.add_wrapped(wrapper2)?;

    let wrapper4 = wrap_pymodule!(mixed_systems::mixed_systems);
    module.add_wrapped(wrapper4)?;

    let wrapper3 = wrap_pymodule!(bosons::bosons);
    module.add_wrapped(wrapper3)?;

    let system = PyModule::import(_py, "sys")?;
    let system_modules: &PyDict = system.getattr("modules")?.downcast()?;
    system_modules.set_item("struqture_py.spins", module.getattr("spins")?)?;
    system_modules.set_item("struqture_py.fermions", module.getattr("fermions")?)?;
    system_modules.set_item(
        "struqture_py.mixed_systems",
        module.getattr("mixed_systems")?,
    )?;
    system_modules.set_item("struqture_py.bosons", module.getattr("bosons")?)?;
    Ok(())
}

use num_complex::Complex64;
use numpy::{IntoPyArray, PyArray1};
use struqture::CooSparseMatrix;
// use pyo3::prelude::*;

pub type PyCooMatrix = (
    Py<PyArray1<Complex64>>,
    (Py<PyArray1<usize>>, Py<PyArray1<usize>>),
);

// Simple wrapper function to convert internal COO matrix to a Python compatible form,
// it expects a CooSparseMatrix so any error handling should be done before using it.
fn to_py_coo(coo: CooSparseMatrix) -> PyResult<PyCooMatrix> {
    Python::with_gil(|py| -> PyResult<PyCooMatrix> {
        let values: Py<PyArray1<Complex64>> = coo.0.into_pyarray(py).to_owned();
        let rows: Py<PyArray1<usize>> = coo.1 .0.into_pyarray(py).to_owned();
        let columns: Py<PyArray1<usize>> = coo.1 .1.into_pyarray(py).to_owned();
        Ok((values, (rows, columns)))
    })
}
