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

use super::{FermionProduct, OperateOnFermions};
use crate::{ModeIndex, OperateOnDensityMatrix, OperateOnModes, StruqtureError, StruqtureVersion};
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Entry, Iter, Keys, Values};
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

/// FermionLindbladNoiseOperators represent noise interactions in the Lindblad equation.
///
/// In the Lindblad equation, Linblad noise operator L_i are not limited to [crate::fermions::FermionProduct] style operators.
/// We use ([crate::fermions::FermionProduct], [crate::fermions::FermionProduct]) as a unique basis.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::fermions::{FermionProduct, FermionLindbladNoiseOperator};
///
/// let mut system = FermionLindbladNoiseOperator::new();
///
/// // Set noise terms:
/// let bp_0_1 = FermionProduct::new([0], [1]).unwrap();
/// let bp_0 = FermionProduct::new([], [0]).unwrap();
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
#[serde(from = "FermionLindbladNoiseOperatorSerialize")]
#[serde(into = "FermionLindbladNoiseOperatorSerialize")]
pub struct FermionLindbladNoiseOperator {
    /// The internal map representing the noise terms
    internal_map: HashMap<(FermionProduct, FermionProduct), CalculatorComplex>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct FermionLindbladNoiseOperatorSerialize {
    /// The vector representing the internal map of the FermionLindbladNoiseOperator
    items: Vec<(
        FermionProduct,
        FermionProduct,
        CalculatorFloat,
        CalculatorFloat,
    )>,
    /// The struqture version
    _struqture_version: StruqtureVersion,
}

impl From<FermionLindbladNoiseOperatorSerialize> for FermionLindbladNoiseOperator {
    fn from(value: FermionLindbladNoiseOperatorSerialize) -> Self {
        let new_noise_op: FermionLindbladNoiseOperator = value
            .items
            .into_iter()
            .map(|(left, right, real, imag)| {
                ((left, right), CalculatorComplex { re: real, im: imag })
            })
            .collect();
        new_noise_op
    }
}

impl From<FermionLindbladNoiseOperator> for FermionLindbladNoiseOperatorSerialize {
    fn from(value: FermionLindbladNoiseOperator) -> Self {
        let new_noise_op: Vec<(
            FermionProduct,
            FermionProduct,
            CalculatorFloat,
            CalculatorFloat,
        )> = value
            .into_iter()
            .map(|((left, right), val)| (left, right, val.re, val.im))
            .collect();
        Self {
            items: new_noise_op,
            _struqture_version: StruqtureVersion,
        }
    }
}

impl<'a> OperateOnDensityMatrix<'a> for FermionLindbladNoiseOperator {
    type Index = (FermionProduct, FermionProduct);
    type Value = CalculatorComplex;
    type IteratorType = Iter<'a, (FermionProduct, FermionProduct), CalculatorComplex>;
    type KeyIteratorType = Keys<'a, (FermionProduct, FermionProduct), CalculatorComplex>;
    type ValueIteratorType = Values<'a, (FermionProduct, FermionProduct), CalculatorComplex>;

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

    /// Overwrites an existing entry or sets a new entry in the FermionLindbladNoiseOperator with the given ((FermionProduct, FermionProduct) key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The (FermionProduct, FermionProduct) key to set in the FermionLindbladNoiseOperator.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the FermionLindbladNoiseOperator.
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

impl<'a> OperateOnModes<'a> for FermionLindbladNoiseOperator {
    // From trait
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

    /// Gets the maximum index of the FermionLindbladNoiseOperator.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of bosons in the FermionLindbladNoiseOperator.
    fn number_modes(&'a self) -> usize {
        self.current_number_modes()
    }
}

impl<'a> OperateOnFermions<'a> for FermionLindbladNoiseOperator {}

/// Implements the default function (Default trait) of FermionLindbladNoiseOperator (an empty FermionLindbladNoiseOperator).
///
impl Default for FermionLindbladNoiseOperator {
    fn default() -> Self {
        Self::new()
    }
}

/// Functions for the FermionLindbladNoiseOperator
///
impl FermionLindbladNoiseOperator {
    /// Creates a new FermionLindbladNoiseOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) FermionLindbladNoiseOperator.
    pub fn new() -> Self {
        FermionLindbladNoiseOperator {
            internal_map: HashMap::new(),
        }
    }

    /// Creates a new FermionLindbladNoiseOperator with pre-allocated capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The pre-allocated capacity of the operator.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) FermionLindbladNoiseOperator.
    pub fn with_capacity(capacity: usize) -> Self {
        FermionLindbladNoiseOperator {
            internal_map: HashMap::with_capacity(capacity),
        }
    }
}

/// Implements the negative sign function of FermionLindbladNoiseOperator.
///
impl ops::Neg for FermionLindbladNoiseOperator {
    type Output = FermionLindbladNoiseOperator;
    /// Implement minus sign for FermionOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The FermionOperator * -1.
    fn neg(self) -> Self {
        let mut internal = HashMap::with_capacity(self.len());
        for (key, val) in self {
            internal.insert(key.clone(), val.neg());
        }
        FermionLindbladNoiseOperator {
            internal_map: internal,
        }
    }
}

/// Implements the plus function of FermionLindbladNoiseOperator by FermionLindbladNoiseOperator.
///
impl<T, V> ops::Add<T> for FermionLindbladNoiseOperator
where
    T: IntoIterator<Item = ((FermionProduct, FermionProduct), V)>,
    V: Into<CalculatorComplex>,
{
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
    fn add(mut self, other: T) -> Self {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value))
                .expect("Internal bug in add_operator_product");
        }
        self
    }
}

/// Implements the minus function of FermionLindbladNoiseOperator by FermionLindbladNoiseOperator.
///
impl<T, V> ops::Sub<T> for FermionLindbladNoiseOperator
where
    T: IntoIterator<Item = ((FermionProduct, FermionProduct), V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implements `-` (subtract) for two FermionLindbladNoiseOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The FermionLindbladNoiseOperator to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Self` - The two FermionLindbladNoiseOperators subtracted.
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

/// Implements the multiplication function of FermionLindbladNoiseOperator by CalculatorFloat.
///
impl<T> ops::Mul<T> for FermionLindbladNoiseOperator
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for FermionLindbladNoiseOperator and CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The FermionLindbladNoiseOperator multiplied by the CalculatorFloat.
    fn mul(self, other: T) -> Self {
        let other_cc = Into::<CalculatorComplex>::into(other);
        let mut internal = HashMap::with_capacity(self.len());
        for (key, val) in self {
            internal.insert(key, val * other_cc.clone());
        }
        FermionLindbladNoiseOperator {
            internal_map: internal,
        }
    }
}

/// Implements the into_iter function (IntoIterator trait) of FermionLindbladNoiseOperator.
///
impl IntoIterator for FermionLindbladNoiseOperator {
    type Item = ((FermionProduct, FermionProduct), CalculatorComplex);
    type IntoIter =
        std::collections::hash_map::IntoIter<(FermionProduct, FermionProduct), CalculatorComplex>;
    /// Returns the FermionLindbladNoiseOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The FermionLindbladNoiseOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference FermionLindbladNoiseOperator.
///
impl<'a> IntoIterator for &'a FermionLindbladNoiseOperator {
    type Item = (&'a (FermionProduct, FermionProduct), &'a CalculatorComplex);
    type IntoIter = Iter<'a, (FermionProduct, FermionProduct), CalculatorComplex>;

    /// Returns the reference FermionLindbladNoiseOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference FermionLindbladNoiseOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of FermionLindbladNoiseOperator.
///
impl FromIterator<((FermionProduct, FermionProduct), CalculatorComplex)>
    for FermionLindbladNoiseOperator
{
    /// Returns the object in FermionLindbladNoiseOperator form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the FermionLindbladNoiseOperator.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in FermionLindbladNoiseOperator form.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from_iter<I: IntoIterator<Item = ((FermionProduct, FermionProduct), CalculatorComplex)>>(
        iter: I,
    ) -> Self {
        let mut slno = FermionLindbladNoiseOperator::new();
        for (pair, cc) in iter {
            slno.add_operator_product(pair, cc)
                .expect("Internal bug in add_operator_product");
        }
        slno
    }
}

/// Implements the extend function (Extend trait) of FermionLindbladNoiseOperator.
///
impl Extend<((FermionProduct, FermionProduct), CalculatorComplex)>
    for FermionLindbladNoiseOperator
{
    /// Extends the FermionLindbladNoiseOperator by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the FermionLindbladNoiseOperator.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn extend<I: IntoIterator<Item = ((FermionProduct, FermionProduct), CalculatorComplex)>>(
        &mut self,
        iter: I,
    ) {
        for (pair, cc) in iter {
            self.add_operator_product(pair, cc)
                .expect("Internal bug in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of FermionLindbladNoiseOperator.
///
impl fmt::Display for FermionLindbladNoiseOperator {
    /// Formats the FermionLindbladNoiseOperator using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted FermionLindbladNoiseOperator.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "FermionLindbladNoiseOperator{\n".to_string();
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
    use std::str::FromStr;

    // Test the Clone and PartialEq traits of SpinOperator
    #[test]
    fn so_from_sos() {
        let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
        let sos = FermionLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp.clone(), 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersion,
        };
        let mut so = FermionLindbladNoiseOperator::new();
        so.set((pp.clone(), pp), CalculatorComplex::from(0.5))
            .unwrap();

        assert_eq!(FermionLindbladNoiseOperator::from(sos.clone()), so);
        assert_eq!(FermionLindbladNoiseOperatorSerialize::from(so), sos);
    }
    // Test the Clone and PartialEq traits of SpinOperator
    #[test]
    fn clone_partial_eq() {
        let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
        let sos = FermionLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersion,
        };

        // Test Clone trait
        assert_eq!(sos.clone(), sos);

        // Test PartialEq trait
        let pp_1: FermionProduct = FermionProduct::new([0], [0]).unwrap();
        let sos_1 = FermionLindbladNoiseOperatorSerialize {
            items: vec![(pp_1.clone(), pp_1, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersion,
        };
        let pp_2: FermionProduct = FermionProduct::new([0], [1]).unwrap();
        let sos_2 = FermionLindbladNoiseOperatorSerialize {
            items: vec![(pp_2.clone(), pp_2, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersion,
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
        let sos = FermionLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersion,
        };

        assert_eq!(
            format!("{:?}", sos),
            "FermionLindbladNoiseOperatorSerialize { items: [(FermionProduct { creators: [0], annihilators: [0] }, FermionProduct { creators: [0], annihilators: [0] }, Float(0.5), Float(0.0))], _struqture_version: StruqtureVersion }"
        );
    }

    /// Test SpinOperator Serialization and Deserialization traits (readable)
    #[test]
    fn serde_readable() {
        use crate::STRUQTURE_VERSION;
        let mut rsplit = STRUQTURE_VERSION.split('.').take(2);
        let major_version = u32::from_str(
            rsplit
                .next()
                .expect("Internal error: Version not conforming to semver"),
        )
        .expect("Internal error: Major version is not unsigned integer.");
        let minor_version = u32::from_str(
            rsplit
                .next()
                .expect("Internal error: Version not conforming to semver"),
        )
        .expect("Internal error: Minor version is not unsigned integer.");

        let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
        let sos = FermionLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersion,
        };

        assert_tokens(
            &sos.readable(),
            &[
                Token::Struct {
                    name: "FermionLindbladNoiseOperatorSerialize",
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
                Token::Str("_struqture_version"),
                Token::Struct {
                    name: "StruqtureVersionSerializable",
                    len: 2,
                },
                Token::Str("major_version"),
                Token::U32(major_version),
                Token::Str("minor_version"),
                Token::U32(minor_version),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    /// Test SpinOperator Serialization and Deserialization traits (compact)
    #[test]
    fn serde_compact() {
        use crate::STRUQTURE_VERSION;
        let mut rsplit = STRUQTURE_VERSION.split('.').take(2);
        let major_version = u32::from_str(
            rsplit
                .next()
                .expect("Internal error: Version not conforming to semver"),
        )
        .expect("Internal error: Major version is not unsigned integer.");
        let minor_version = u32::from_str(
            rsplit
                .next()
                .expect("Internal error: Version not conforming to semver"),
        )
        .expect("Internal error: Minor version is not unsigned integer.");

        let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
        let sos = FermionLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersion,
        };

        assert_tokens(
            &sos.compact(),
            &[
                Token::Struct {
                    name: "FermionLindbladNoiseOperatorSerialize",
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
                Token::Str("_struqture_version"),
                Token::Struct {
                    name: "StruqtureVersionSerializable",
                    len: 2,
                },
                Token::Str("major_version"),
                Token::U32(major_version),
                Token::Str("minor_version"),
                Token::U32(minor_version),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
