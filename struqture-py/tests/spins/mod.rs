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

mod decoherence_product;

mod pauli_product;

mod plus_minus_product;

mod plus_minus_operator;

mod plus_minus_noise_operator;

mod pauli_operator;

mod pauli_hamiltonian;

mod pauli_noise_operator;

mod pauli_open_system;

use pyo3::prelude::*;
use qoqo_calculator::CalculatorFloat;
use qoqo_calculator_pyo3::CalculatorFloatWrapper;
// helper function to convert CalculatorFloat into a python object
fn convert_cf_to_pyobject(py: Python, parameter: CalculatorFloat) -> Bound<CalculatorFloatWrapper> {
    let parameter_type = py.get_type::<CalculatorFloatWrapper>();
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
