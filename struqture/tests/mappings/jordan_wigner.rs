// Copyright © 2020-2022 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use qoqo_calculator::CalculatorComplex;

use struqture::prelude::*;
use struqture::fermions::FermionProduct;
use struqture::spins::{PauliProduct,
                       SpinOperator,
                       SingleSpinOperator,
};
use struqture::mappings::JordanWignerFermionToSpin;

#[test]
fn test_jw_fermion_to_spin() {

    let creators = &[1];
    let annihilators = &[2];
    let fp = FermionProduct::new(creators.to_vec(), annihilators.to_vec()).unwrap();
    let pp_1 = PauliProduct::new().y(1).x(2);
    let pp_2 = PauliProduct::new().x(1).y(2);
    let pp_3 = PauliProduct::new().y(1).y(2);
    let pp_4 = PauliProduct::new().x(1).x(2);
    let mut so = SpinOperator::new();
    so.add_operator_product(pp_1.clone(), CalculatorComplex::new(0.0, -0.25)).unwrap();
    so.add_operator_product(pp_2.clone(), CalculatorComplex::new(0.0, 0.25)).unwrap();
    so.add_operator_product(pp_3.clone(), CalculatorComplex::new(0.25, 0.0)).unwrap();
    so.add_operator_product(pp_4.clone(), CalculatorComplex::new(0.25, 0.0)).unwrap();

    assert_eq!(fp.jordan_wigner(), so);

    let fp = FermionProduct::new([], []).unwrap();
    let mut so = SpinOperator::new();
    let mut id = PauliProduct::new();
    id = id.set_pauli(0, SingleSpinOperator::Identity);
    so.add_operator_product(id.clone(), CalculatorComplex::new(1.0, 0.0)).unwrap();
    
    assert_eq!(fp.jordan_wigner(), so)
}

