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

use super::{FermionHamiltonian, OperateOnFermions};
use crate::fermions::FermionProduct;
use crate::mappings::JordanWignerFermionToSpin;
use crate::spins::SpinOperator;
use crate::{
    GetValue, ModeIndex, OperateOnDensityMatrix, OperateOnModes, OperateOnState, StruqtureError,
    StruqtureVersionSerializable, SymmetricIndex, MINIMUM_STRUQTURE_VERSION,
};
// use itertools::Itertools;
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Entry, Iter, Keys, Values};
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

/// FermionOperators are combinations of FermionProducts with specific CalculatorComplex coefficients.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::fermions::{OperateOnFermions, FermionProduct};
/// use struqture::fermions::FermionOperator;
/// let mut fo = FermionOperator::new();
///
/// // Representing the opetator $ 1/2 b_0^{dagger} + 1/5 b_1 $
/// // Creating a FermionProduct with a creation operator acting on mode 0 and no annihilation operators
/// let fp_0 = FermionProduct::new([0],[]).unwrap();
/// // Creating a FermionProduct with a annihilation operator acting on mode 1 and no creation operators
/// let fp_1 = FermionProduct::new([],[1]).unwrap();
/// fo.set(fp_0.clone(), CalculatorComplex::from(0.5)).unwrap();
/// fo.set(fp_1.clone(), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(fo.get(&fp_0), &CalculatorComplex::from(0.5));
/// assert_eq!(fo.get(&fp_1), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "FermionOperatorSerialize")]
#[serde(into = "FermionOperatorSerialize")]
pub struct FermionOperator {
    /// The internal HashMap of FermionProducts and coefficients (CalculatorComplex)
    internal_map: HashMap<FermionProduct, CalculatorComplex>,
}
impl crate::MinSupportedVersion for FermionOperator {}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for FermionOperator {
    fn schema_name() -> String {
        "FermionOperator".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <FermionOperatorSerialize>::json_schema(gen)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
struct FermionOperatorSerialize {
    items: Vec<(FermionProduct, CalculatorFloat, CalculatorFloat)>,
    _struqture_version: StruqtureVersionSerializable,
}

impl From<FermionOperatorSerialize> for FermionOperator {
    fn from(value: FermionOperatorSerialize) -> Self {
        let new_noise_op: FermionOperator = value
            .items
            .into_iter()
            .map(|(key, real, imag)| (key, CalculatorComplex { re: real, im: imag }))
            .collect();
        new_noise_op
    }
}

impl From<FermionOperator> for FermionOperatorSerialize {
    fn from(value: FermionOperator) -> Self {
        let new_noise_op: Vec<(FermionProduct, CalculatorFloat, CalculatorFloat)> = value
            .into_iter()
            .map(|(key, val)| (key, val.re, val.im))
            .collect();
        let current_version = StruqtureVersionSerializable {
            major_version: MINIMUM_STRUQTURE_VERSION.0,
            minor_version: MINIMUM_STRUQTURE_VERSION.1,
        };
        Self {
            items: new_noise_op,
            _struqture_version: current_version,
        }
    }
}

impl<'a> OperateOnDensityMatrix<'a> for FermionOperator {
    type Index = FermionProduct;
    type Value = CalculatorComplex;
    type IteratorType = Iter<'a, Self::Index, Self::Value>;
    type KeyIteratorType = Keys<'a, Self::Index, Self::Value>;
    type ValueIteratorType = Values<'a, Self::Index, Self::Value>;

    // From trait
    fn get(&self, key: &FermionProduct) -> &CalculatorComplex {
        match self.internal_map.get(key) {
            Some(value) => value,
            None => &CalculatorComplex::ZERO,
        }
    }

    // From trait
    fn iter(&'a self) -> Self::IteratorType {
        self.internal_map.iter()
    }

    // From trait
    fn keys(&'a self) -> Self::KeyIteratorType {
        self.internal_map.keys()
    }

    // From trait
    fn values(&'a self) -> Self::ValueIteratorType {
        self.internal_map.values()
    }

    // From trait
    fn remove(&mut self, key: &Self::Index) -> Option<Self::Value> {
        self.internal_map.remove(key)
    }

    // From trait
    fn empty_clone(&self, capacity: Option<usize>) -> Self {
        match capacity {
            Some(cap) => Self::with_capacity(cap),
            None => Self::new(),
        }
    }

    /// Overwrites an existing entry or sets a new entry in the FermionOperator with the given (FermionProduct key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The FermionProduct key to set in the FermionOperator.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the FermionOperator.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(CalculatorComplex))` - The key existed, this is the value it had before it was set with the value input.
    /// * `Ok(None)` - The key did not exist, it has been set with its corresponding value.
    fn set(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<Option<Self::Value>, StruqtureError> {
        if value != CalculatorComplex::ZERO {
            Ok(self.internal_map.insert(key, value))
        } else {
            match self.internal_map.entry(key) {
                Entry::Occupied(val) => Ok(Some(val.remove())),
                Entry::Vacant(_) => Ok(None),
            }
        }
    }
}

impl<'a> OperateOnState<'a> for FermionOperator {}

impl<'a> OperateOnModes<'a> for FermionOperator {
    // From trait
    fn current_number_modes(&'a self) -> usize {
        let mut max_mode: usize = 0;
        if !self.is_empty() {
            for key in self.keys() {
                let maxk = key.current_number_modes();
                if maxk > max_mode {
                    max_mode = maxk;
                }
            }
        }
        max_mode
    }

    /// Gets the maximum index of the FermionOperator.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of fermions in the FermionOperator.
    fn number_modes(&'a self) -> usize {
        self.current_number_modes()
    }
}

impl<'a> OperateOnFermions<'a> for FermionOperator {}

/// Implements the default function (Default trait) of FermionOperator (an empty FermionOperator).
///
impl Default for FermionOperator {
    fn default() -> Self {
        Self::new()
    }
}

/// Functions for the FermionOperator
///
impl FermionOperator {
    /// Creates a new FermionOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) FermionOperator.
    pub fn new() -> Self {
        FermionOperator {
            internal_map: HashMap::new(),
        }
    }

    /// Creates a new FermionOperator with pre-allocated capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The pre-allocated capacity of the operator.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) FermionOperator.
    pub fn with_capacity(capacity: usize) -> Self {
        FermionOperator {
            internal_map: HashMap::with_capacity(capacity),
        }
    }

    /// Separate self into an operator with the terms of given number of creation and annihilation operators and an operator with the remaining operations
    ///
    /// # Arguments
    ///
    /// * `number_creators_annihilators` - Number of creation and annihilation terms to filter for in the keys.
    ///
    /// # Returns
    ///
    /// `Ok((separated, remainder))` - Operator with the noise terms where number_creators_annihilators matches the number of spins the operator product acts on and Operator with all other contributions.
    pub fn separate_into_n_terms(
        &self,
        number_creators_annihilators: (usize, usize),
    ) -> Result<(Self, Self), StruqtureError> {
        let mut separated = Self::default();
        let mut remainder = Self::default();
        for (prod, val) in self.iter() {
            if (prod.creators().len(), prod.annihilators().len()) == number_creators_annihilators {
                separated.add_operator_product(prod.clone(), val.clone())?;
            } else {
                remainder.add_operator_product(prod.clone(), val.clone())?;
            }
        }
        Ok((separated, remainder))
    }
}

impl From<FermionHamiltonian> for FermionOperator {
    /// Converts a FermionHamiltonian into a FermionOperator.
    ///
    /// # Arguments
    ///
    /// * `hamiltonian` - The FermionHamiltonian to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The FermionHamiltonian converted into a FermionOperator.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from(hamiltonian: FermionHamiltonian) -> Self {
        let mut internal = FermionOperator::new();
        for (key, value) in hamiltonian.into_iter() {
            let bp = FermionProduct::get_key(&key);
            internal
                .add_operator_product(bp.clone(), value.clone())
                .expect("Internal bug in add_operator_product");
            if !key.is_natural_hermitian() {
                let bp_conj = bp.hermitian_conjugate();
                internal
                    .add_operator_product(FermionProduct::get_key(&bp_conj.0), value * bp_conj.1)
                    .expect("Internal error in add_operator_product");
            }
        }
        internal
    }
}

/// Implements the negative sign function of FermionOperator.
///
impl ops::Neg for FermionOperator {
    type Output = FermionOperator;
    /// Implement minus sign for FermionOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The FermionOperator * -1.
    fn neg(self) -> Self {
        let mut internal = self.internal_map.clone();
        for key in self.keys() {
            internal.insert(key.clone(), internal[key].clone() * -1.0);
        }
        FermionOperator {
            internal_map: internal,
        }
    }
}

/// Implements the plus function of FermionOperator by FermionOperator.
///
impl ops::Add<FermionOperator> for FermionOperator {
    type Output = Self;
    /// Implements `+` (add) for two FermionOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The FermionOperator to be added.
    ///
    /// # Returns
    ///
    /// * `Self` - The two FermionOperators added together.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn add(mut self, other: FermionOperator) -> Self {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key, value)
                .expect("Internal bug in add_operator_product");
        }
        self
    }
}

/// Implements the minus function of FermionOperator by FermionOperator.
///
impl ops::Sub<FermionOperator> for FermionOperator {
    type Output = Self;
    /// Implements `-` (subtract) for two FermionOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The FermionOperator to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Self` - The two FermionOperators subtracted.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn sub(mut self, other: FermionOperator) -> Self {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key, value * -1.0)
                .expect("Internal bug in add_operator_product");
        }
        self
    }
}

/// Implements the multiplication function of FermionOperator by CalculatorComplex/CalculatorFloat.
///
impl<T> ops::Mul<T> for FermionOperator
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for FermionOperator and CalculatorComplex/CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex or CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The FermionOperator multiplied by the CalculatorComplex/CalculatorFloat.
    fn mul(self, other: T) -> Self {
        let other_cc = Into::<CalculatorComplex>::into(other);
        let mut internal = self.internal_map.clone();
        for key in self.keys() {
            internal.insert(key.clone(), internal[key].clone() * other_cc.clone());
        }
        FermionOperator {
            internal_map: internal,
        }
    }
}

/// Implements the multiplication function of FermionOperator by FermionOperator.
///
impl ops::Mul<FermionOperator> for FermionOperator {
    type Output = Self;
    /// Implement `*` for FermionOperator and FermionOperator.
    ///
    /// # Arguments
    ///
    /// * `other` - The FermionOperator to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - The two FermionOperators multiplied.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn mul(self, other: FermionOperator) -> Self {
        let mut op = FermionOperator::with_capacity(self.len() * other.len());
        for (bps, vals) in self {
            for (bpo, valo) in other.iter() {
                let fermion_products = bps.clone() * bpo.clone();
                let coefficient = Into::<CalculatorComplex>::into(valo) * vals.clone();
                for (prod, coeff) in fermion_products {
                    op.add_operator_product(prod, coefficient.clone() * coeff)
                        .expect("Internal bug in add_operator_product");
                }
            }
        }
        op
    }
}

/// Implements the into_iter function (IntoIterator trait) of FermionOperator.
///
impl IntoIterator for FermionOperator {
    type Item = (FermionProduct, CalculatorComplex);
    type IntoIter = std::collections::hash_map::IntoIter<FermionProduct, CalculatorComplex>;
    /// Returns the FermionOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The FermionOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference FermionOperator.
///
impl<'a> IntoIterator for &'a FermionOperator {
    type Item = (&'a FermionProduct, &'a CalculatorComplex);
    type IntoIter = Iter<'a, FermionProduct, CalculatorComplex>;

    /// Returns the reference FermionOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The FermionOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of FermionOperator.
///
impl FromIterator<(FermionProduct, CalculatorComplex)> for FermionOperator {
    /// Returns the object in FermionOperator form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the FermionOperator.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in FermionOperator form.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from_iter<I: IntoIterator<Item = (FermionProduct, CalculatorComplex)>>(iter: I) -> Self {
        let mut so = FermionOperator::new();
        for (pp, cc) in iter {
            so.add_operator_product(pp, cc)
                .expect("Internal bug in add_operator_product");
        }
        so
    }
}

/// Implements the extend function (Extend trait) of FermionOperator.
///
impl Extend<(FermionProduct, CalculatorComplex)> for FermionOperator {
    /// Extends the FermionOperator by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the FermionOperator.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn extend<I: IntoIterator<Item = (FermionProduct, CalculatorComplex)>>(&mut self, iter: I) {
        for (pp, cc) in iter {
            self.add_operator_product(pp, cc)
                .expect("Internal bug in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of FermionOperator.
///
impl fmt::Display for FermionOperator {
    /// Formats the FermionOperator using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted FermionOperator.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "FermionOperator{\n".to_string();
        for (key, val) in self.iter() {
            writeln!(output, "{}: {},", key, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}

impl JordanWignerFermionToSpin for FermionOperator {
    type Output = SpinOperator;

    /// Implements JordanWignerFermionToSpin for a FermionOperator.
    ///
    /// The convention used is that |0> represents an empty fermionic state (spin-orbital),
    /// and |1> represents an occupied fermionic state.
    ///
    /// # Returns
    ///
    /// `SpinOperator` - The spin operator that results from the transformation.
    fn jordan_wigner(&self) -> Self::Output {
        let mut out = SpinOperator::new();
        for fp in self.keys() {
            out = out + fp.jordan_wigner() * self.get(fp);
        }
        out
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_test::{assert_tokens, Configure, Token};

    // Test the Clone and PartialEq traits of SpinOperator
    #[test]
    fn so_from_sos() {
        let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
        let sos = FermionOperatorSerialize {
            items: vec![(pp.clone(), 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        let mut so = FermionOperator::new();
        so.set(pp, CalculatorComplex::from(0.5)).unwrap();

        assert_eq!(FermionOperator::from(sos.clone()), so);
        assert_eq!(FermionOperatorSerialize::from(so), sos);
    }
    // Test the Clone and PartialEq traits of SpinOperator
    #[test]
    fn clone_partial_eq() {
        let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
        let sos = FermionOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        // Test Clone trait
        assert_eq!(sos.clone(), sos);

        // Test PartialEq trait
        let pp_1: FermionProduct = FermionProduct::new([0], [0]).unwrap();
        let sos_1 = FermionOperatorSerialize {
            items: vec![(pp_1, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        let pp_2: FermionProduct = FermionProduct::new([1], [0]).unwrap();
        let sos_2 = FermionOperatorSerialize {
            items: vec![(pp_2, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        assert!(sos_1 == sos);
        assert!(sos == sos_1);
        assert!(sos_2 != sos);
        assert!(sos != sos_2);
    }

    // Test the Debug trait of SpinOperator
    #[test]
    fn debug() {
        let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
        let sos = FermionOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        assert_eq!(
            format!("{:?}", sos),
            "FermionOperatorSerialize { items: [(FermionProduct { creators: [0], annihilators: [0] }, Float(0.5), Float(0.0))], _struqture_version: StruqtureVersionSerializable { major_version: 1, minor_version: 0 } }"
        );
    }

    /// Test SpinOperator Serialization and Deserialization traits (readable)
    #[test]
    fn serde_readable() {
        let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
        let sos = FermionOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        assert_tokens(
            &sos.readable(),
            &[
                Token::Struct {
                    name: "FermionOperatorSerialize",
                    len: 2,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 3 },
                Token::Str("c0a0"),
                Token::F64(0.5),
                Token::F64(0.0),
                Token::TupleEnd,
                Token::SeqEnd,
                Token::Str("_struqture_version"),
                Token::Struct {
                    name: "StruqtureVersionSerializable",
                    len: 2,
                },
                Token::Str("major_version"),
                Token::U32(1),
                Token::Str("minor_version"),
                Token::U32(0),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    /// Test SpinOperator Serialization and Deserialization traits (compact)
    #[test]
    fn serde_compact() {
        let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
        let sos = FermionOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        assert_tokens(
            &sos.compact(),
            &[
                Token::Struct {
                    name: "FermionOperatorSerialize",
                    len: 2,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 3 },
                Token::Tuple { len: 2 },
                Token::Seq { len: Some(1) },
                Token::U64(0),
                Token::SeqEnd,
                Token::Seq { len: Some(1) },
                Token::U64(0),
                Token::SeqEnd,
                Token::TupleEnd,
                Token::NewtypeVariant {
                    name: "CalculatorFloat",
                    variant: "Float",
                },
                Token::F64(0.5),
                Token::NewtypeVariant {
                    name: "CalculatorFloat",
                    variant: "Float",
                },
                Token::F64(0.0),
                Token::TupleEnd,
                Token::SeqEnd,
                Token::Str("_struqture_version"),
                Token::Struct {
                    name: "StruqtureVersionSerializable",
                    len: 2,
                },
                Token::Str("major_version"),
                Token::U32(1),
                Token::Str("minor_version"),
                Token::U32(0),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
