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

use nalgebra as na;
use num_complex::Complex64;

mod decoherence_product;
pub use decoherence_product::*;

mod pauli_product;
pub use pauli_product::*;

mod plus_minus_product;
pub use plus_minus_product::*;

mod decoherence_operator;
pub use decoherence_operator::*;

mod spin_operator;
pub use spin_operator::*;

mod plus_minus_operator;
pub use plus_minus_operator::*;

mod spin_hamiltonian;
pub use spin_hamiltonian::*;

mod spin_system;
pub use spin_system::*;

mod spin_hamiltonian_system;
pub use spin_hamiltonian_system::*;

mod spin_noise_operator;
pub use spin_noise_operator::*;

mod spin_noise_system;
pub use spin_noise_system::*;

mod spin_open_system;
pub use spin_open_system::*;

fn create_na_matrix_from_operator_list(operators: &[&str]) -> na::DMatrix<Complex64> {
    let cc1 = Complex64::new(1.0, 0.0);
    let cc0 = Complex64::new(0.0, 0.0);
    let cci = Complex64::new(0.0, 1.0);

    let identity: na::DMatrix<Complex64> = na::DMatrix::identity(2, 2);
    let x: na::DMatrix<Complex64> = na::dmatrix![cc0, cc1; cc1, cc0];
    let y: na::DMatrix<Complex64> = na::dmatrix![cc0, -cci; cci, cc0];
    let z: na::DMatrix<Complex64> = na::dmatrix![cc1, cc0; cc0, -cc1];

    let mut matrix = match operators[0] {
        "I" => identity.clone(),
        "X" => x.clone(),
        "Y" => y.clone(),
        "Z" => z.clone(),
        _ => panic!("Ill defined pauli product"),
    };

    for op in operators[1..].iter() {
        match *op {
            "I" => matrix = matrix.kronecker(&identity),
            "X" => matrix = matrix.kronecker(&x),
            "Y" => matrix = matrix.kronecker(&y),
            "Z" => matrix = matrix.kronecker(&z),
            _ => panic!("Ill defined pauli product"),
        }
    }
    matrix
}

fn create_na_matrix_from_decoherence_list(operators: &[&str]) -> na::DMatrix<Complex64> {
    let cc1 = Complex64::new(1.0, 0.0);
    let cc0 = Complex64::new(0.0, 0.0);

    let identity: na::DMatrix<Complex64> = na::DMatrix::identity(2, 2);
    let x: na::DMatrix<Complex64> = na::dmatrix![cc0, cc1; cc1, cc0];
    let iy: na::DMatrix<Complex64> = na::dmatrix![cc0, cc1; -cc1, cc0];
    let z: na::DMatrix<Complex64> = na::dmatrix![cc1, cc0; cc0, -cc1];

    let mut matrix = match operators[0] {
        "I" => identity.clone(),
        "X" => x.clone(),
        "iY" => iy.clone(),
        "Z" => z.clone(),
        _ => panic!("Ill defined pauli product"),
    };

    for op in operators[1..].iter() {
        match *op {
            "I" => matrix = matrix.kronecker(&identity),
            "X" => matrix = matrix.kronecker(&x),
            "iY" => matrix = matrix.kronecker(&iy),
            "Z" => matrix = matrix.kronecker(&z),
            _ => panic!("Ill defined pauli product"),
        }
    }
    matrix
}
