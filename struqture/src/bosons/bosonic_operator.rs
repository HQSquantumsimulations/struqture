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

use super::{BosonHamiltonian, OperateOnBosons};
use crate::bosons::BosonProduct;
use crate::{
    GetValue, ModeIndex, OperateOnDensityMatrix, OperateOnModes, OperateOnState, StruqtureError,
    SymmetricIndex,
};
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

use indexmap::map::{Entry, Iter};
use indexmap::IndexMap;

/// BosonOperators are combinations of BosonProducts with specific CalculatorComplex coefficients.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::bosons::{OperateOnBosons, BosonProduct};
/// use struqture::bosons::BosonOperator;
/// let mut bo = BosonOperator::new();
///
/// // Representing the opetator $ 1/2 b_0^{dagger} + 1/5 b_1 $
/// // Creating a BosonProduct with a creation operator acting on mode 0 and no annihilation operators
/// let bp_0 = BosonProduct::new([0],[]).unwrap();
/// // Creating a BosonProduct with a annihilation operator acting on mode 1 and no creation operators
/// let bp_1 = BosonProduct::new([],[1]).unwrap();
/// bo.set(bp_0.clone(), CalculatorComplex::from(0.5)).unwrap();
/// bo.set(bp_1.clone(), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(bo.get(&bp_0), &CalculatorComplex::from(0.5));
/// assert_eq!(bo.get(&bp_1), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "BosonOperatorSerialize")]
#[serde(into = "BosonOperatorSerialize")]
pub struct BosonOperator {
    /// The internal HashMap of BosonProducts and coefficients (CalculatorComplex)
    internal_map: IndexMap<BosonProduct, CalculatorComplex>,
}

impl crate::SerializationSupport for BosonOperator {
    fn struqture_type() -> crate::StruqtureType {
        crate::StruqtureType::BosonOperator
    }
}
#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for BosonOperator {
    fn schema_name() -> String {
        "BosonOperator".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <BosonOperatorSerialize>::json_schema(gen)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
struct BosonOperatorSerialize {
    items: Vec<(BosonProduct, CalculatorFloat, CalculatorFloat)>,
    serialisation_meta: crate::StruqtureSerialisationMeta,
}

impl TryFrom<BosonOperatorSerialize> for BosonOperator {
    type Error = StruqtureError;
    fn try_from(value: BosonOperatorSerialize) -> Result<Self, Self::Error> {
        let target_serialisation_meta =
            <Self as crate::SerializationSupport>::target_serialisation_meta();
        crate::check_can_be_deserialised(&target_serialisation_meta, &value.serialisation_meta)?;
        let new_noise_op: BosonOperator = value
            .items
            .into_iter()
            .map(|(key, real, imag)| (key, CalculatorComplex { re: real, im: imag }))
            .collect();
        Ok(new_noise_op)
    }
}

impl From<BosonOperator> for BosonOperatorSerialize {
    fn from(value: BosonOperator) -> Self {
        let serialisation_meta = crate::SerializationSupport::struqture_serialisation_meta(&value);
        let new_noise_op: Vec<(BosonProduct, CalculatorFloat, CalculatorFloat)> = value
            .into_iter()
            .map(|(key, val)| (key, val.re, val.im))
            .collect();
        Self {
            items: new_noise_op,
            serialisation_meta,
        }
    }
}

impl<'a> OperateOnDensityMatrix<'a> for BosonOperator {
    type Index = BosonProduct;
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

    /// Overwrites an existing entry or sets a new entry in the BosonOperator with the given (BosonProduct key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The BosonProduct key to set in the BosonOperator.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the BosonOperator.
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
                Entry::Occupied(val) => Ok(Some(val.shift_remove())),
                Entry::Vacant(_) => Ok(None),
            }
        }
    }
}

impl OperateOnState<'_> for BosonOperator {}

impl<'a> OperateOnModes<'a> for BosonOperator {
    /// Gets the maximum index of the BosonOperator.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of bosons in the BosonOperator.
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
}

impl OperateOnBosons<'_> for BosonOperator {}

/// Implements the default function (Default trait) of BosonOperator (an empty BosonOperator).
///
impl Default for BosonOperator {
    fn default() -> Self {
        Self::new()
    }
}

/// Functions for the BosonOperator
///
impl BosonOperator {
    /// Creates a new BosonOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) BosonOperator.
    pub fn new() -> Self {
        BosonOperator {
            internal_map: IndexMap::new(),
        }
    }

    /// Creates a new BosonOperator with pre-allocated capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The pre-allocated capacity of the operator.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) BosonOperator.
    pub fn with_capacity(capacity: usize) -> Self {
        BosonOperator {
            internal_map: IndexMap::with_capacity(capacity),
        }
    }

    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_export")]
    pub fn to_struqture_1(&self) -> Result<struqture_1::bosons::BosonSystem, StruqtureError> {
        let mut new_boson_system = struqture_1::bosons::BosonSystem::new(None);
        for (key, val) in self.iter() {
            let one_key = key.to_struqture_1()?;
            let _ = struqture_1::OperateOnDensityMatrix::set(
                &mut new_boson_system,
                one_key,
                val.clone(),
            );
        }
        Ok(new_boson_system)
    }

    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_import")]
    pub fn from_struqture_1(
        value: &struqture_1::bosons::BosonSystem,
    ) -> Result<Self, StruqtureError> {
        let mut new_operator = Self::new();
        for (key, val) in struqture_1::OperateOnDensityMatrix::iter(value) {
            let self_key = BosonProduct::from_struqture_1(key)?;
            let _ = new_operator.set(self_key, val.clone());
        }
        Ok(new_operator)
    }
}

impl From<BosonHamiltonian> for BosonOperator {
    /// Converts a BosonHamiltonian into a BosonOperator.
    ///
    /// # Arguments
    ///
    /// * `hamiltonian` - The BosonHamiltonian to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The BosonHamiltonian converted into a BosonOperator.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from(hamiltonian: BosonHamiltonian) -> Self {
        let mut internal = BosonOperator::new();
        for (key, value) in hamiltonian.into_iter() {
            let bp = BosonProduct::get_key(&key);
            internal
                .add_operator_product(bp.clone(), value.clone())
                .expect("Internal error in add_operator_product.");
            if !key.is_natural_hermitian() {
                let bp_conj = bp.hermitian_conjugate();
                internal
                    .add_operator_product(BosonProduct::get_key(&bp_conj.0), value * bp_conj.1)
                    .expect("Internal error in add_operator_product.");
            }
        }
        internal
    }
}

/// Implements the negative sign function of BosonOperator.
///
impl ops::Neg for BosonOperator {
    type Output = BosonOperator;
    /// Implement minus sign for BosonOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The BosonOperator * -1.
    fn neg(self) -> Self {
        let mut internal = self.internal_map.clone();
        for key in self.keys() {
            internal.insert(key.clone(), internal[key].clone() * -1.0);
        }
        BosonOperator {
            internal_map: internal,
        }
    }
}

/// Implements the plus function of BosonOperator by BosonOperator.
///
impl ops::Add<BosonOperator> for BosonOperator {
    type Output = Self;
    /// Implements `+` (add) for two BosonOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The BosonOperator to be added.
    ///
    /// # Returns
    ///
    /// * `Self` - The two BosonOperators added together.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn add(mut self, other: BosonOperator) -> Self {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key, value)
                .expect("Internal error in add_operator_product");
        }
        self
    }
}

/// Implements the minus function of BosonOperator by BosonOperator.
///
impl ops::Sub<BosonOperator> for BosonOperator {
    type Output = Self;
    /// Implements `-` (subtract) for two BosonOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The BosonOperator to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Self` - The two BosonOperators subtracted.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn sub(mut self, other: BosonOperator) -> Self {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key, value * -1.0)
                .expect("Internal error in add_operator_product");
        }
        self
    }
}

/// Implements the multiplication function of BosonOperator by CalculatorComplex/CalculatorFloat.
///
impl<T> ops::Mul<T> for BosonOperator
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for BosonOperator and CalculatorComplex/CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex or CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The BosonOperator multiplied by the CalculatorComplex/CalculatorFloat.
    fn mul(self, other: T) -> Self {
        let other_cc = Into::<CalculatorComplex>::into(other);
        let mut internal = self.internal_map.clone();
        for key in self.keys() {
            internal.insert(key.clone(), internal[key].clone() * other_cc.clone());
        }
        BosonOperator {
            internal_map: internal,
        }
    }
}

/// Implements the multiplication function of BosonOperator by BosonOperator.
///
impl ops::Mul<BosonOperator> for BosonOperator {
    type Output = Self;
    /// Implement `*` for BosonOperator and BosonOperator.
    ///
    /// # Arguments
    ///
    /// * `other` - The BosonOperator to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - The two BosonOperators multiplied.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn mul(self, other: BosonOperator) -> Self {
        let mut op = BosonOperator::with_capacity(self.len() * other.len());
        for (bps, vals) in self {
            for (bpo, valo) in other.iter() {
                let boson_products = bps.clone() * bpo.clone();
                let coefficient = Into::<CalculatorComplex>::into(valo) * vals.clone();
                for prod in boson_products {
                    op.add_operator_product(prod, coefficient.clone())
                        .expect("Internal bug in add_operator_product");
                }
            }
        }
        op
    }
}

/// Implements the into_iter function (IntoIterator trait) of BosonOperator.
///
impl IntoIterator for BosonOperator {
    type Item = (BosonProduct, CalculatorComplex);
    type IntoIter = indexmap::map::IntoIter<BosonProduct, CalculatorComplex>;
    /// Returns the BosonOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The BosonOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference BosonOperator.
///
impl<'a> IntoIterator for &'a BosonOperator {
    type Item = (&'a BosonProduct, &'a CalculatorComplex);
    type IntoIter = Iter<'a, BosonProduct, CalculatorComplex>;

    /// Returns the reference BosonOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference BosonOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of BosonOperator.
///
impl FromIterator<(BosonProduct, CalculatorComplex)> for BosonOperator {
    /// Returns the object in BosonOperator form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the BosonOperator.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in BosonOperator form.
    fn from_iter<I: IntoIterator<Item = (BosonProduct, CalculatorComplex)>>(iter: I) -> Self {
        let mut so = BosonOperator::new();
        for (pp, cc) in iter {
            so.add_operator_product(pp, cc)
                .expect("Internal error in add_operator_product");
        }
        so
    }
}

/// Implements the extend function (Extend trait) of BosonOperator.
///
impl Extend<(BosonProduct, CalculatorComplex)> for BosonOperator {
    /// Extends the BosonOperator by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the BosonOperator.
    fn extend<I: IntoIterator<Item = (BosonProduct, CalculatorComplex)>>(&mut self, iter: I) {
        for (pp, cc) in iter {
            self.add_operator_product(pp, cc)
                .expect("Internal error in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of BosonOperator.
///
impl fmt::Display for BosonOperator {
    /// Formats the BosonOperator using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted BosonOperator.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "BosonOperator{\n".to_string();
        for (key, val) in self.iter() {
            writeln!(output, "{}: {},", key, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::STRUQTURE_VERSION;
    use serde_test::{assert_tokens, Configure, Token};

    // Test the Clone and PartialEq traits of BosonOperator
    #[test]
    fn so_from_sos() {
        let pp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
        let sos = BosonOperatorSerialize {
            items: vec![(pp.clone(), 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "BosonOperator".to_string(),
                min_version: (2, 0, 0),
                version: STRUQTURE_VERSION.to_string(),
            },
        };
        let mut so = BosonOperator::new();
        so.set(pp, CalculatorComplex::from(0.5)).unwrap();

        assert_eq!(BosonOperator::try_from(sos.clone()).unwrap(), so);
        assert_eq!(BosonOperatorSerialize::from(so), sos);
    }
    // Test the Clone and PartialEq traits of BosonOperator
    #[test]
    fn clone_partial_eq() {
        let pp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
        let sos = BosonOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "BosonOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        // Test Clone trait
        assert_eq!(sos.clone(), sos);

        // Test PartialEq trait
        let pp_1: BosonProduct = BosonProduct::new([0], [0]).unwrap();
        let sos_1 = BosonOperatorSerialize {
            items: vec![(pp_1, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "BosonOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };
        let pp_2: BosonProduct = BosonProduct::new([1], [0]).unwrap();
        let sos_2 = BosonOperatorSerialize {
            items: vec![(pp_2, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "BosonOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };
        assert!(sos_1 == sos);
        assert!(sos == sos_1);
        assert!(sos_2 != sos);
        assert!(sos != sos_2);
    }

    // Test the Debug trait of BosonOperator
    #[test]
    fn debug() {
        let pp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
        let sos = BosonOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "BosonOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_eq!(
            format!("{:?}", sos),
            "BosonOperatorSerialize { items: [(BosonProduct { creators: [0], annihilators: [0] }, Float(0.5), Float(0.0))], serialisation_meta: StruqtureSerialisationMeta { type_name: \"BosonOperator\", min_version: (2, 0, 0), version: \"2.0.0\" } }"
        );
    }

    /// Test BosonOperator Serialization and Deserialization traits (readable)
    #[test]
    fn serde_readable() {
        let pp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
        let sos = BosonOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "BosonOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_tokens(
            &sos.readable(),
            &[
                Token::Struct {
                    name: "BosonOperatorSerialize",
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
                Token::Str("serialisation_meta"),
                Token::Struct {
                    name: "StruqtureSerialisationMeta",
                    len: 3,
                },
                Token::Str("type_name"),
                Token::Str("BosonOperator"),
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

    /// Test BosonOperator Serialization and Deserialization traits (compact)
    #[test]
    fn serde_compact() {
        let pp: BosonProduct = BosonProduct::new([0], [0]).unwrap();
        let sos = BosonOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "BosonOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_tokens(
            &sos.compact(),
            &[
                Token::Struct {
                    name: "BosonOperatorSerialize",
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
                Token::Str("serialisation_meta"),
                Token::Struct {
                    name: "StruqtureSerialisationMeta",
                    len: 3,
                },
                Token::Str("type_name"),
                Token::Str("BosonOperator"),
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
