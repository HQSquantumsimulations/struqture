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

use super::{BosonProduct, OperateOnBosons};
use crate::{ModeIndex, OperateOnDensityMatrix, OperateOnModes, StruqtureError};
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

use indexmap::map::{Entry, Iter};
use indexmap::IndexMap;

/// BosonLindbladNoiseOperators represent noise interactions in the Lindblad equation.
///
/// In the Lindblad equation, Linblad noise operator L_i are not limited to [crate::bosons::BosonProduct] style operators.
/// We use ([crate::bosons::BosonProduct], [crate::bosons::BosonProduct]) as a unique basis.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::bosons::{BosonProduct, BosonLindbladNoiseOperator};
///
/// let mut system = BosonLindbladNoiseOperator::new();
///
/// // Set noise terms:
/// let bp_0_1 = BosonProduct::new([0], [1]).unwrap();
/// let bp_0 = BosonProduct::new([], [0]).unwrap();
/// system.set((bp_0_1.clone(), bp_0_1.clone()), CalculatorComplex::from(0.5)).unwrap();
/// system.set((bp_0.clone(), bp_0.clone()), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(system.current_number_modes(), 2_usize);
/// assert_eq!(system.get(&(bp_0_1.clone(), bp_0_1.clone())), &CalculatorComplex::from(0.5));
/// assert_eq!(system.get(&(bp_0.clone(), bp_0.clone())), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "BosonLindbladNoiseOperatorSerialize")]
#[serde(into = "BosonLindbladNoiseOperatorSerialize")]
pub struct BosonLindbladNoiseOperator {
    /// The internal map representing the noise terms
    internal_map: IndexMap<(BosonProduct, BosonProduct), CalculatorComplex>,
}

impl crate::SerializationSupport for BosonLindbladNoiseOperator {
    fn struqture_type() -> crate::StruqtureType {
        crate::StruqtureType::BosonLindbladNoiseOperator
    }
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for BosonLindbladNoiseOperator {
    fn schema_name() -> String {
        "BosonLindbladNoiseOperator".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <BosonLindbladNoiseOperatorSerialize>::json_schema(gen)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
struct BosonLindbladNoiseOperatorSerialize {
    /// The vector representing the internal map of the BosonLindbladNoiseOperator
    items: Vec<(BosonProduct, BosonProduct, CalculatorFloat, CalculatorFloat)>,
    serialisation_meta: crate::StruqtureSerialisationMeta,
}

impl TryFrom<BosonLindbladNoiseOperatorSerialize> for BosonLindbladNoiseOperator {
    type Error = StruqtureError;
    fn try_from(value: BosonLindbladNoiseOperatorSerialize) -> Result<Self, Self::Error> {
        let target_serialisation_meta =
            <Self as crate::SerializationSupport>::target_serialisation_meta();
        crate::check_can_be_deserialised(&target_serialisation_meta, &value.serialisation_meta)?;
        let new_noise_op: BosonLindbladNoiseOperator = value
            .items
            .into_iter()
            .map(|(left, right, real, imag)| {
                ((left, right), CalculatorComplex { re: real, im: imag })
            })
            .collect();
        Ok(new_noise_op)
    }
}

impl From<BosonLindbladNoiseOperator> for BosonLindbladNoiseOperatorSerialize {
    fn from(value: BosonLindbladNoiseOperator) -> Self {
        let serialisation_meta = crate::SerializationSupport::struqture_serialisation_meta(&value);
        let new_noise_op: Vec<(BosonProduct, BosonProduct, CalculatorFloat, CalculatorFloat)> =
            value
                .into_iter()
                .map(|((left, right), val)| (left, right, val.re, val.im))
                .collect();
        Self {
            items: new_noise_op,
            serialisation_meta,
        }
    }
}

impl<'a> OperateOnDensityMatrix<'a> for BosonLindbladNoiseOperator {
    type Index = (BosonProduct, BosonProduct);
    type Value = CalculatorComplex;

    // From trait
    fn get(&self, key: &Self::Index) -> &Self::Value {
        match self.internal_map.get(key) {
            Some(value) => value,
            None => &CalculatorComplex::ZERO,
        }
    }

    // From trait
    fn iter(&'a self) -> impl ExactSizeIterator<Item = (&'a Self::Index, &'a Self::Value)> {
        self.internal_map.iter()
    }

    // From trait
    fn keys(&'a self) -> impl ExactSizeIterator<Item = &'a Self::Index> {
        self.internal_map.keys()
    }

    // From trait
    fn values(&'a self) -> impl ExactSizeIterator<Item = &'a Self::Value> {
        self.internal_map.values()
    }

    // From trait
    fn remove(&mut self, key: &Self::Index) -> Option<Self::Value> {
        self.internal_map.shift_remove(key)
    }

    // From trait
    fn empty_clone(&self, capacity: Option<usize>) -> Self {
        match capacity {
            Some(cap) => Self::with_capacity(cap),
            None => Self::new(),
        }
    }

    /// Overwrites an existing entry or sets a new entry in the BosonLindbladNoiseOperator with the given ((BosonProduct, BosonProduct) key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The (BosonProduct, BosonProduct) key to set in the BosonLindbladNoiseOperator.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the BosonLindbladNoiseOperator.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(CalculatorComplex))` - The key existed, this is the value it had before it was set with the value input.
    /// * `Ok(None)` - The key did not exist, it has been set with its corresponding value.
    /// * `Err(StruqtureError::InvalidLindbladTerms)` - The input contained identities, which are not allowed as Lindblad operators.
    ///
    /// # Panics
    ///
    /// * Internal error in BosonProduct::new
    fn set(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<Option<Self::Value>, StruqtureError> {
        if key.0 == BosonProduct::new([], [])? || key.1 == BosonProduct::new([], [])? {
            return Err(StruqtureError::InvalidLindbladTerms);
        }

        if value != CalculatorComplex::ZERO {
            Ok(self.internal_map.insert(key, value))
        } else {
            match self.internal_map.entry(key) {
                Entry::Occupied(val) => Ok(Some(val.shift_remove())),
                Entry::Vacant(_) => Ok(None),
            }
        }
    }
}

impl<'a> OperateOnModes<'a> for BosonLindbladNoiseOperator {
    /// Gets the maximum index of the BosonLindbladNoiseOperator.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of bosons in the BosonLindbladNoiseOperator.
    fn current_number_modes(&'a self) -> usize {
        let mut max_mode: usize = 0;
        if !self.is_empty() {
            for key in self.keys() {
                let maxk = key
                    .0
                    .current_number_modes()
                    .max(key.1.current_number_modes());
                if maxk > max_mode {
                    max_mode = maxk;
                }
            }
        }
        max_mode
    }
}

impl<'a> OperateOnBosons<'a> for BosonLindbladNoiseOperator {}

/// Implements the default function (Default trait) of BosonLindbladNoiseOperator (an empty BosonLindbladNoiseOperator).
///
impl Default for BosonLindbladNoiseOperator {
    fn default() -> Self {
        Self::new()
    }
}

/// Functions for the BosonLindbladNoiseOperator
///
impl BosonLindbladNoiseOperator {
    /// Creates a new BosonLindbladNoiseOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) BosonLindbladNoiseOperator.
    pub fn new() -> Self {
        BosonLindbladNoiseOperator {
            internal_map: IndexMap::new(),
        }
    }

    /// Creates a new BosonLindbladNoiseOperator with pre-allocated capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The pre-allocated capacity of the operator.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) BosonLindbladNoiseOperator.
    pub fn with_capacity(capacity: usize) -> Self {
        BosonLindbladNoiseOperator {
            internal_map: IndexMap::with_capacity(capacity),
        }
    }

    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_export")]
    pub fn to_struqture_1(
        &self,
    ) -> Result<struqture_1::bosons::BosonLindbladNoiseSystem, StruqtureError> {
        let mut new_boson_system = struqture_1::bosons::BosonLindbladNoiseSystem::new(None);
        for (key, val) in self.iter() {
            let one_key_left = key.0.to_struqture_1()?;
            let one_key_right = key.1.to_struqture_1()?;
            let _ = struqture_1::OperateOnDensityMatrix::set(
                &mut new_boson_system,
                (one_key_left, one_key_right),
                val.clone(),
            );
        }
        Ok(new_boson_system)
    }

    /// Import from struqture_1 format.
    #[cfg(feature = "struqture_1_import")]
    pub fn from_struqture_1(
        value: &struqture_1::bosons::BosonLindbladNoiseSystem,
    ) -> Result<Self, StruqtureError> {
        let mut new_qubit_operator = Self::new();
        for (key, val) in struqture_1::OperateOnDensityMatrix::iter(value) {
            let self_key_left = BosonProduct::from_struqture_1(&key.0)?;
            let self_key_right = BosonProduct::from_struqture_1(&key.1)?;
            let _ = new_qubit_operator.set((self_key_left, self_key_right), val.clone());
        }
        Ok(new_qubit_operator)
    }
}

/// Implements the negative sign function of BosonLindbladNoiseOperator.
///
impl ops::Neg for BosonLindbladNoiseOperator {
    type Output = BosonLindbladNoiseOperator;
    /// Implement minus sign for BosonLindbladNoiseOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The BosonLindbladNoiseOperator * -1.
    fn neg(self) -> Self {
        let mut internal = IndexMap::with_capacity(self.len());
        for (key, val) in self {
            internal.insert(key.clone(), val.neg());
        }
        BosonLindbladNoiseOperator {
            internal_map: internal,
        }
    }
}

/// Implements the plus function of BosonLindbladNoiseOperator by BosonLindbladNoiseOperator.
///
impl<T, V> ops::Add<T> for BosonLindbladNoiseOperator
where
    T: IntoIterator<Item = ((BosonProduct, BosonProduct), V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implements `+` (add) for BosonLindbladNoiseOperator and BosonLindbladNoiseOperator.
    ///
    /// # Arguments
    ///
    /// * `other` - The BosonLindbladNoiseOperator to be added.
    ///
    /// # Returns
    ///
    /// * `Self` - The two BosonLindbladNoiseOperators added together.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn add(mut self, other: T) -> Self {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value))
                .expect("Internal bug in add_operator_product");
        }
        self
    }
}

/// Implements the minus function of BosonLindbladNoiseOperator by BosonLindbladNoiseOperator.
///
impl<T, V> ops::Sub<T> for BosonLindbladNoiseOperator
where
    T: IntoIterator<Item = ((BosonProduct, BosonProduct), V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implements `-` (subtract) for two BosonLindbladNoiseOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The BosonLindbladNoiseOperator to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Self` - The two BosonLindbladNoiseOperators subtracted.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn sub(mut self, other: T) -> Self {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value) * -1.0)
                .expect("Internal bug in add_operator_product");
        }
        self
    }
}

/// Implements the multiplication function of BosonLindbladNoiseOperator by CalculatorComplex.
///
impl<T> ops::Mul<T> for BosonLindbladNoiseOperator
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for BosonLindbladNoiseOperator and CalculatorComplex.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The BosonLindbladNoiseOperator multiplied by the CalculatorComplex.
    fn mul(self, other: T) -> Self {
        let other_cc = Into::<CalculatorComplex>::into(other);
        let mut internal = IndexMap::with_capacity(self.len());
        for (key, val) in self {
            internal.insert(key, val * other_cc.clone());
        }
        BosonLindbladNoiseOperator {
            internal_map: internal,
        }
    }
}

/// Implements the into_iter function (IntoIterator trait) of BosonLindbladNoiseOperator.
///
impl IntoIterator for BosonLindbladNoiseOperator {
    type Item = ((BosonProduct, BosonProduct), CalculatorComplex);
    type IntoIter = indexmap::map::IntoIter<(BosonProduct, BosonProduct), CalculatorComplex>;
    /// Returns the BosonLindbladNoiseOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The BosonLindbladNoiseOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference BosonLindbladNoiseOperator.
///
impl<'a> IntoIterator for &'a BosonLindbladNoiseOperator {
    type Item = (&'a (BosonProduct, BosonProduct), &'a CalculatorComplex);
    type IntoIter = Iter<'a, (BosonProduct, BosonProduct), CalculatorComplex>;

    /// Returns the reference BosonLindbladNoiseOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference BosonLindbladNoiseOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of BosonLindbladNoiseOperator.
///
impl FromIterator<((BosonProduct, BosonProduct), CalculatorComplex)>
    for BosonLindbladNoiseOperator
{
    /// Returns the object in BosonLindbladNoiseOperator form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the BosonLindbladNoiseOperator.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in BosonLindbladNoiseOperator form.
    fn from_iter<I: IntoIterator<Item = ((BosonProduct, BosonProduct), CalculatorComplex)>>(
        iter: I,
    ) -> Self {
        let mut slno = BosonLindbladNoiseOperator::new();
        for (pair, cc) in iter {
            slno.add_operator_product(pair, cc)
                .expect("Internal bug in add_operator_product");
        }
        slno
    }
}

/// Implements the extend function (Extend trait) of BosonLindbladNoiseOperator.
///
impl Extend<((BosonProduct, BosonProduct), CalculatorComplex)> for BosonLindbladNoiseOperator {
    /// Extends the BosonLindbladNoiseOperator by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the BosonLindbladNoiseOperator.
    fn extend<I: IntoIterator<Item = ((BosonProduct, BosonProduct), CalculatorComplex)>>(
        &mut self,
        iter: I,
    ) {
        for (pair, cc) in iter {
            self.add_operator_product(pair, cc)
                .expect("Internal bug in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of BosonLindbladNoiseOperator.
///
impl fmt::Display for BosonLindbladNoiseOperator {
    /// Formats the BosonLindbladNoiseOperator using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted BosonLindbladNoiseOperator.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "BosonLindbladNoiseOperator{\n".to_string();
        for (key, val) in self.iter() {
            writeln!(output, "({}, {}): {},", key.0, key.1, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_test::{assert_tokens, Configure, Token};

    // Test the Clone and PartialEq traits of QubitOperator
    #[test]
    fn so_from_sos() {
        let pp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
        let sos = BosonLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp.clone(), 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "BosonLindbladNoiseOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0-alpha.0".to_string(),
            },
        };
        let mut so = BosonLindbladNoiseOperator::new();
        so.set((pp.clone(), pp), CalculatorComplex::from(0.5))
            .unwrap();

        assert_eq!(
            BosonLindbladNoiseOperator::try_from(sos.clone()).unwrap(),
            so
        );
        assert_eq!(BosonLindbladNoiseOperatorSerialize::from(so), sos);
    }
    // Test the Clone and PartialEq traits of QubitOperator
    #[test]
    fn clone_partial_eq() {
        let pp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
        let sos = BosonLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "BosonLindbladNoiseOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        // Test Clone trait
        assert_eq!(sos.clone(), sos);

        // Test PartialEq trait
        let pp_1: BosonProduct = BosonProduct::new([0], [0]).unwrap();
        let sos_1 = BosonLindbladNoiseOperatorSerialize {
            items: vec![(pp_1.clone(), pp_1, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "BosonLindbladNoiseOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };
        let pp_2: BosonProduct = BosonProduct::new([0], [1]).unwrap();
        let sos_2 = BosonLindbladNoiseOperatorSerialize {
            items: vec![(pp_2.clone(), pp_2, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "BosonLindbladNoiseOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };
        assert!(sos_1 == sos);
        assert!(sos == sos_1);
        assert!(sos_2 != sos);
        assert!(sos != sos_2);
    }

    // Test the Debug trait of QubitOperator
    #[test]
    fn debug() {
        let pp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
        let sos = BosonLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "BosonLindbladNoiseOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_eq!(
            format!("{:?}", sos),
            "BosonLindbladNoiseOperatorSerialize { items: [(BosonProduct { creators: [0], annihilators: [0] }, BosonProduct { creators: [0], annihilators: [0] }, Float(0.5), Float(0.0))], serialisation_meta: StruqtureSerialisationMeta { type_name: \"BosonLindbladNoiseOperator\", min_version: (2, 0, 0), version: \"2.0.0\" } }"
        );
    }

    /// Test QubitOperator Serialization and Deserialization traits (readable)
    #[test]
    fn serde_readable() {
        let pp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
        let sos = BosonLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "BosonLindbladNoiseOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_tokens(
            &sos.readable(),
            &[
                Token::Struct {
                    name: "BosonLindbladNoiseOperatorSerialize",
                    len: 2,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 4 },
                Token::Str("c0a0"),
                Token::Str("c0a0"),
                Token::F64(0.5),
                Token::F64(0.0),
                Token::TupleEnd,
                Token::SeqEnd,
                Token::Str("serialisation_meta"),
                Token::Struct {
                    name: "StruqtureSerialisationMeta",
                    len: 3,
                },
                Token::Str("type_name"),
                Token::Str("BosonLindbladNoiseOperator"),
                Token::Str("min_version"),
                Token::Tuple { len: 3 },
                Token::U64(2),
                Token::U64(0),
                Token::U64(0),
                Token::TupleEnd,
                Token::Str("version"),
                Token::Str("2.0.0"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    /// Test QubitOperator Serialization and Deserialization traits (compact)
    #[test]
    fn serde_compact() {
        let pp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
        let sos = BosonLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "BosonLindbladNoiseOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_tokens(
            &sos.compact(),
            &[
                Token::Struct {
                    name: "BosonLindbladNoiseOperatorSerialize",
                    len: 2,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 4 },
                Token::Tuple { len: 2 },
                Token::Seq { len: Some(1) },
                Token::U64(0),
                Token::SeqEnd,
                Token::Seq { len: Some(1) },
                Token::U64(0),
                Token::SeqEnd,
                Token::TupleEnd,
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
                Token::Str("serialisation_meta"),
                Token::Struct {
                    name: "StruqtureSerialisationMeta",
                    len: 3,
                },
                Token::Str("type_name"),
                Token::Str("BosonLindbladNoiseOperator"),
                Token::Str("min_version"),
                Token::Tuple { len: 3 },
                Token::U64(2),
                Token::U64(0),
                Token::U64(0),
                Token::TupleEnd,
                Token::Str("version"),
                Token::Str("2.0.0"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
