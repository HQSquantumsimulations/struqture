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

use super::{OperateOnSpins, SpinOperator};
use crate::fermions::FermionOperator;
use crate::mappings::JordanWignerSpinToFermion;
use crate::spins::DecoherenceProduct;
use crate::{
    OperateOnDensityMatrix, OperateOnState, SpinIndex, StruqtureError,
    StruqtureVersionSerializable, SymmetricIndex, MINIMUM_STRUQTURE_VERSION,
};
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Entry, Iter, Keys, Values};
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

/// DecoherenceOperators are combinations of DecoherenceProducts with specific CalculatorComplex coefficients.
///
/// This is a representation of sums of pauli products with weightings, in order to build a full hamiltonian.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::spins::{DecoherenceProduct, DecoherenceOperator};
///
/// let mut so = DecoherenceOperator::new();
///
/// // Representing the hamiltonian $ 1/2 \sigma_0^{x} \sigma_1^{x} + 1/5 \sigma_0^{z} $
/// let pp_01 = DecoherenceProduct::new().x(0).x(1);
/// let pp_0 = DecoherenceProduct::new().z(0);
/// so.add_operator_product(pp_01.clone(), CalculatorComplex::from(0.5)).unwrap();
/// so.add_operator_product(pp_0.clone(), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(so.get(&pp_01), &CalculatorComplex::from(0.5));
/// assert_eq!(so.get(&pp_0), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(from = "DecoherenceOperatorSerialize")]
#[serde(into = "DecoherenceOperatorSerialize")]

pub struct DecoherenceOperator {
    /// The internal HashMap of DecoherenceProducts and coefficients (CalculatorComplex)
    internal_map: HashMap<DecoherenceProduct, CalculatorComplex>,
}

impl crate::MinSupportedVersion for DecoherenceOperator {}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for DecoherenceOperator {
    fn schema_name() -> String {
        "DecoherenceOperator".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <DecoherenceOperatorSerialize>::json_schema(gen)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
struct DecoherenceOperatorSerialize {
    /// The internal map representing the noise terms
    items: Vec<(DecoherenceProduct, CalculatorFloat, CalculatorFloat)>,
    _struqture_version: StruqtureVersionSerializable,
}

impl From<DecoherenceOperatorSerialize> for DecoherenceOperator {
    fn from(value: DecoherenceOperatorSerialize) -> Self {
        let new_noise_op: DecoherenceOperator = value
            .items
            .into_iter()
            .map(|(key, real, imag)| (key, CalculatorComplex { re: real, im: imag }))
            .collect();
        new_noise_op
    }
}

impl From<DecoherenceOperator> for DecoherenceOperatorSerialize {
    fn from(value: DecoherenceOperator) -> Self {
        let new_noise_op: Vec<(DecoherenceProduct, CalculatorFloat, CalculatorFloat)> = value
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

impl<'a> OperateOnDensityMatrix<'a> for DecoherenceOperator {
    type IteratorType = Iter<'a, Self::Index, Self::Value>;
    type KeyIteratorType = Keys<'a, Self::Index, Self::Value>;
    type ValueIteratorType = Values<'a, Self::Index, Self::Value>;
    type Value = CalculatorComplex;
    type Index = DecoherenceProduct;

    // From trait
    fn get(&self, key: &Self::Index) -> &Self::Value {
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

    /// Overwrites an existing entry or sets a new entry in the DecoherenceOperator with the given (DecoherenceProduct key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The DecoherenceProduct key to set in the DecoherenceOperator.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the DecoherenceOperator.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(CalculatorComplex))` - The key existed, this is the value it had before it was set with the value input.
    /// * `Ok(None)` - The key did not exist, it has been set with its corresponding value.
    /// * `Err(StruqtureError::NumberSpinsExceeded)` - Index of DecoherenceProduct exceeds that of the DecoherenceOperator.
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

impl<'a> OperateOnState<'a> for DecoherenceOperator {
    // From trait
    fn hermitian_conjugate(&self) -> Self {
        let mut new_operator = Self::with_capacity(self.len());
        for (product, value) in self.iter() {
            let (new_product, prefactor) = product.hermitian_conjugate();
            new_operator
                .add_operator_product(new_product, value.conj() * prefactor)
                .expect("Internal bug in add_operator_product");
        }
        new_operator
    }
}

impl<'a> OperateOnSpins<'a> for DecoherenceOperator {
    /// Returns maximum index in DecoherenceOperator internal_map.
    ///
    /// # Returns
    ///
    /// * `usize` - Maximum index.
    fn current_number_spins(&self) -> usize {
        let mut max_mode: usize = 0;
        if !self.internal_map.is_empty() {
            for key in self.internal_map.keys() {
                if key.current_number_spins() > max_mode {
                    max_mode = key.current_number_spins()
                }
            }
        }
        max_mode
    }

    /// Gets the maximum index of the DecoherenceOperator.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of spins in the DecoherenceOperator.
    fn number_spins(&self) -> usize {
        self.current_number_spins()
    }
}

// The following traits are intentionally not implemented:
// impl<'a> ToSparseMatrixOperator<'a> for DecoherenceOperator {}
// impl<'a> ToSparseMatrixSuperOperator<'a> for DecoherenceOperator {

/// Implements the default function (Default trait) of DecoherenceOperator (an empty DecoherenceOperator).
///
impl Default for DecoherenceOperator {
    fn default() -> Self {
        Self::new()
    }
}

/// Functions for the DecoherenceOperator
///
impl DecoherenceOperator {
    /// Creates a new DecoherenceOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) DecoherenceOperator.
    pub fn new() -> Self {
        DecoherenceOperator {
            internal_map: HashMap::new(),
        }
    }

    /// Creates a new DecoherenceOperator with pre-allocated capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The pre-allocated capacity of the operator.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) DecoherenceOperator.
    pub fn with_capacity(capacity: usize) -> Self {
        DecoherenceOperator {
            internal_map: HashMap::with_capacity(capacity),
        }
    }

    /// Separate self into an operator with the terms of given number of spins and an operator with the remaining operations
    ///
    /// # Arguments
    ///
    /// * `number_spins` - Number of spins to filter for in the keys.
    ///
    /// # Returns
    ///
    /// `Ok((separated, remainder))` - Operator with the noise terms where number_spins matches the number of spins the operator product acts on and Operator with all other contributions.
    pub fn separate_into_n_terms(
        &self,
        number_spins: usize,
    ) -> Result<(Self, Self), StruqtureError> {
        let mut separated = Self::default();
        let mut remainder = Self::default();
        for (prod, val) in self.iter() {
            if prod.len() == number_spins {
                separated.add_operator_product(prod.clone(), val.clone())?;
            } else {
                remainder.add_operator_product(prod.clone(), val.clone())?;
            }
        }
        Ok((separated, remainder))
    }
}

/// Implements the negative sign function of DecoherenceOperator.
///
impl ops::Neg for DecoherenceOperator {
    type Output = DecoherenceOperator;
    /// Implement minus sign for DecoherenceOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The DecoherenceOperator * -1.
    fn neg(self) -> Self {
        let mut internal = HashMap::with_capacity(self.len());
        for (key, val) in self {
            internal.insert(key.clone(), val.neg());
        }
        DecoherenceOperator {
            internal_map: internal,
        }
    }
}

/// Implements the plus function of DecoherenceOperator by DecoherenceOperator.
///
impl<T, V> ops::Add<T> for DecoherenceOperator
where
    T: IntoIterator<Item = (DecoherenceProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implements `+` (add) for two DecoherenceOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The DecoherenceOperator to be added.
    ///
    /// # Returns
    ///
    /// * `Self` - The two DecoherenceOperators added together.
    fn add(mut self, other: T) -> Self {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value))
                .expect("Internal bug in add_operator_product");
        }
        self
    }
}

/// Implements the minus function of DecoherenceOperator by DecoherenceOperator.
///
impl<T, V> ops::Sub<T> for DecoherenceOperator
where
    T: IntoIterator<Item = (DecoherenceProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implements `-` (subtract) for two DecoherenceOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The DecoherenceOperator to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Self` - The two DecoherenceOperators subtracted.
    fn sub(mut self, other: T) -> Self {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value) * -1.0)
                .expect("Internal bug in add_operator_product");
        }
        self
    }
}

/// Implements the multiplication function of DecoherenceOperator by CalculatorComplex/CalculatorFloat.
///
impl<T> ops::Mul<T> for DecoherenceOperator
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for DecoherenceOperator and CalculatorComplex/CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex or CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The DecoherenceOperator multiplied by the CalculatorComplex/CalculatorFloat.
    fn mul(self, other: T) -> Self {
        let other_cc = Into::<CalculatorComplex>::into(other);
        let mut internal = HashMap::with_capacity(self.len());
        for (key, val) in self {
            internal.insert(key, val * other_cc.clone());
        }
        DecoherenceOperator {
            internal_map: internal,
        }
    }
}

/// Implements the multiplication function of DecoherenceOperator by DecoherenceOperator.
///
impl ops::Mul<DecoherenceOperator> for DecoherenceOperator {
    type Output = Self;
    /// Implement `*` for DecoherenceOperator and DecoherenceOperator.
    ///
    /// # Arguments
    ///
    /// * `other` - The DecoherenceOperator to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - The two DecoherenceOperators multiplied.
    fn mul(self, other: DecoherenceOperator) -> Self {
        let mut spin_op = DecoherenceOperator::with_capacity(self.len() * other.len());
        for (pps, vals) in self {
            for (ppo, valo) in other.iter() {
                let (ppp, coefficient) = pps.clone() * ppo.clone();
                let coefficient =
                    Into::<CalculatorComplex>::into(valo) * coefficient * vals.clone();
                spin_op
                    .add_operator_product(ppp, coefficient)
                    .expect("Internal bug in add_operator_product");
            }
        }
        spin_op
    }
}

/// Implements the into_iter function (IntoIterator trait) of DecoherenceOperator.
///
impl IntoIterator for DecoherenceOperator {
    type Item = (DecoherenceProduct, CalculatorComplex);
    type IntoIter = std::collections::hash_map::IntoIter<DecoherenceProduct, CalculatorComplex>;
    /// Returns the DecoherenceOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The DecoherenceOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference DecoherenceOperator.
///
impl<'a> IntoIterator for &'a DecoherenceOperator {
    type Item = (&'a DecoherenceProduct, &'a CalculatorComplex);
    type IntoIter = Iter<'a, DecoherenceProduct, CalculatorComplex>;

    /// Returns the reference DecoherenceOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference DecoherenceOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of DecoherenceOperator.
///
impl FromIterator<(DecoherenceProduct, CalculatorComplex)> for DecoherenceOperator {
    /// Returns the object in DecoherenceOperator form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the DecoherenceOperator.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in DecoherenceOperator form.
    fn from_iter<I: IntoIterator<Item = (DecoherenceProduct, CalculatorComplex)>>(iter: I) -> Self {
        let mut so = DecoherenceOperator::new();
        for (pp, cc) in iter {
            so.add_operator_product(pp, cc)
                .expect("Internal bug in add_operator_product");
        }
        so
    }
}

/// Implements the extend function (Extend trait) of DecoherenceOperator.
///
impl Extend<(DecoherenceProduct, CalculatorComplex)> for DecoherenceOperator {
    /// Extends the DecoherenceOperator by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the DecoherenceOperator.
    fn extend<I: IntoIterator<Item = (DecoherenceProduct, CalculatorComplex)>>(&mut self, iter: I) {
        for (pp, cc) in iter {
            self.add_operator_product(pp, cc)
                .expect("Internal bug in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of DecoherenceOperator.
///
impl fmt::Display for DecoherenceOperator {
    /// Formats the DecoherenceOperator using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted DecoherenceOperator.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "DecoherenceOperator{\n".to_string();
        for (key, val) in self.iter() {
            writeln!(output, "{}: {},", key, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}

impl From<SpinOperator> for DecoherenceOperator {
    /// Converts a SpinOperator into a DecoherenceProduct.
    ///
    /// # Arguments
    ///
    /// * `op` - The SpinOperator to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The SpinOperator converted into a DecoherenceProduct.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from(op: SpinOperator) -> Self {
        let mut out = DecoherenceOperator::new();
        for prod in op.keys() {
            let (new_prod, new_coeff) = DecoherenceProduct::spin_to_decoherence(prod.clone());
            out.add_operator_product(new_prod, op.get(prod).clone() * new_coeff)
                .expect("Internal error in add_operator_product");
        }
        out
    }
}

impl JordanWignerSpinToFermion for DecoherenceOperator {
    type Output = FermionOperator;

    /// Implements JordanWignerSpinToFermion for a DecoherenceOperator.
    ///
    /// The convention used is that |0> represents an empty fermionic state (spin-orbital),
    /// and |1> represents an occupied fermionic state.
    ///
    /// # Returns
    ///
    /// `FermionOperator` - The fermionic operator that results from the transformation.
    fn jordan_wigner(&self) -> Self::Output {
        let mut out = FermionOperator::new();
        for (dp, value) in self.iter() {
            out = out + dp.jordan_wigner() * value;
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
        let pp: DecoherenceProduct = DecoherenceProduct::new().z(0);
        let sos = DecoherenceOperatorSerialize {
            items: vec![(pp.clone(), 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        let mut so = DecoherenceOperator::new();
        so.set(pp, CalculatorComplex::from(0.5)).unwrap();

        assert_eq!(DecoherenceOperator::from(sos.clone()), so);
        assert_eq!(DecoherenceOperatorSerialize::from(so), sos);
    }
    // Test the Clone and PartialEq traits of SpinOperator
    #[test]
    fn clone_partial_eq() {
        let pp: DecoherenceProduct = DecoherenceProduct::new().z(0);
        let sos = DecoherenceOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        // Test Clone trait
        assert_eq!(sos.clone(), sos);

        // Test PartialEq trait
        let pp_1: DecoherenceProduct = DecoherenceProduct::new().z(0);
        let sos_1 = DecoherenceOperatorSerialize {
            items: vec![(pp_1, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        let pp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
        let sos_2 = DecoherenceOperatorSerialize {
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
        let pp: DecoherenceProduct = DecoherenceProduct::new().z(0);
        let sos = DecoherenceOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        assert_eq!(
            format!("{:?}", sos),
            "DecoherenceOperatorSerialize { items: [(DecoherenceProduct { items: [(0, Z)] }, Float(0.5), Float(0.0))], _struqture_version: StruqtureVersionSerializable { major_version: 1, minor_version: 0 } }"
        );
    }

    /// Test SpinOperator Serialization and Deserialization traits (readable)
    #[test]
    fn serde_readable() {
        let pp = DecoherenceProduct::new().x(0);
        let sos = DecoherenceOperatorSerialize {
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
                    name: "DecoherenceOperatorSerialize",
                    len: 2,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 3 },
                Token::Str("0X"),
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
        let pp = DecoherenceProduct::new().x(0);
        let sos = DecoherenceOperatorSerialize {
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
                    name: "DecoherenceOperatorSerialize",
                    len: 2,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 3 },
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 2 },
                Token::U64(0),
                Token::UnitVariant {
                    name: "SingleDecoherenceOperator",
                    variant: "X",
                },
                Token::TupleEnd,
                Token::SeqEnd,
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
