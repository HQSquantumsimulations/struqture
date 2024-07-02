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

use crate::fermions::FermionOperator;
use crate::mappings::JordanWignerSpinToFermion;
use crate::spins::SingleQubitOperator;
use crate::{CooSparseMatrix, CorrespondsTo, GetValue, SpinIndex, StruqtureError, SymmetricIndex};
use num_complex::Complex64;
use qoqo_calculator::CalculatorComplex;
use serde::de::{Deserializer, Error, SeqAccess, Visitor};
use serde::ser::{SerializeSeq, Serializer};
use serde::{Deserialize, Serialize};
use std::cmp::{self, Ordering};
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::iter::{FromIterator, IntoIterator};
use std::ops::Mul;
use std::str::FromStr;
use tinyvec::{TinyVec, TinyVecIterator};

use super::PauliProduct;

/// Single Decoherence operators for DecoherenceProducts:
///
/// I: identity matrix
/// $$
/// \begin{pmatrix}
/// 1 & 0\\\\
/// 0 & 1
/// \end{pmatrix}
/// $$
///
/// X: pauli X matrix
/// $$
/// \begin{pmatrix}
/// 0 & 1\\\\
/// 1 & 0
/// \end{pmatrix}
/// $$
///
/// iY: pauli iY matrix
/// $$
/// \begin{pmatrix}
/// 0 & 1 \\\\
/// -1 & 0
/// \end{pmatrix}
/// $$
///
/// Z: pauli z matrix
/// $$
/// \begin{pmatrix}
/// 1 & 0\\\\
/// 0 & -1
/// \end{pmatrix}
/// $$
///
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub enum SingleDecoherenceOperator {
    Identity,
    X,
    IY,
    Z,
}

/// Creates a SingleDecoherenceOperator from an &str representation.
///
/// # Arguments
///
/// * `s` - The string (&str) to be converted to a SingleDecoherenceOperator.
///
/// # Returns
///
/// * `Ok(Self)` - The SingleDecoherenceOperator of the input string.
/// * `Err(StruqtureError::IncorrectPauliEntry)` - The pauli matrix being set is not in [\"I\", \"X\", \"iY\", \"Z\"].
///
impl FromStr for SingleDecoherenceOperator {
    type Err = StruqtureError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "I" => Ok(SingleDecoherenceOperator::Identity),
            "X" => Ok(SingleDecoherenceOperator::X),
            "iY" => Ok(SingleDecoherenceOperator::IY),
            "Z" => Ok(SingleDecoherenceOperator::Z),
            _ => Err(StruqtureError::IncorrectPauliEntry {
                pauli: s.to_string(),
            }),
        }
    }
}

impl Default for SingleDecoherenceOperator {
    fn default() -> Self {
        SingleDecoherenceOperator::Identity
    }
}

/// Implements the fmt function (Display trait) of SingleDecoherenceOperator.
///
impl fmt::Display for SingleDecoherenceOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SingleDecoherenceOperator::Identity => write!(f, "I"),
            SingleDecoherenceOperator::X => write!(f, "X"),
            SingleDecoherenceOperator::IY => write!(f, "iY"),
            SingleDecoherenceOperator::Z => write!(f, "Z"),
        }
    }
}

/// Functions for the SingleDecoherenceOperator
///
impl SingleDecoherenceOperator {
    /// Implements multiplication function for a SingleDecoherenceOperator by a SingleDecoherenceOperator.
    ///
    /// # Arguments
    ///
    /// * `left` - Left-hand SingleDecoherenceOperator to be multiplied.
    /// * `right` - Right-hand SingleDecoherenceOperator to be multiplied.
    pub fn multiply(
        left: SingleDecoherenceOperator,
        right: SingleDecoherenceOperator,
    ) -> (Self, f64) {
        let result_vec: (SingleDecoherenceOperator, f64) = match (left, right) {
            (SingleDecoherenceOperator::Identity, x) => (x, 1.0),
            (x, SingleDecoherenceOperator::Identity) => (x, 1.0),
            (SingleDecoherenceOperator::X, SingleDecoherenceOperator::X) => {
                (SingleDecoherenceOperator::Identity, 1.0)
            }
            (SingleDecoherenceOperator::X, SingleDecoherenceOperator::IY) => {
                (SingleDecoherenceOperator::Z, -1.0)
            }
            (SingleDecoherenceOperator::X, SingleDecoherenceOperator::Z) => {
                (SingleDecoherenceOperator::IY, -1.0)
            }
            (SingleDecoherenceOperator::IY, SingleDecoherenceOperator::X) => {
                (SingleDecoherenceOperator::Z, 1.0)
            }
            (SingleDecoherenceOperator::IY, SingleDecoherenceOperator::IY) => {
                (SingleDecoherenceOperator::Identity, -1.0)
            }
            (SingleDecoherenceOperator::IY, SingleDecoherenceOperator::Z) => {
                (SingleDecoherenceOperator::X, -1.0)
            }
            (SingleDecoherenceOperator::Z, SingleDecoherenceOperator::X) => {
                (SingleDecoherenceOperator::IY, 1.0)
            }
            (SingleDecoherenceOperator::Z, SingleDecoherenceOperator::IY) => {
                (SingleDecoherenceOperator::X, 1.0)
            }
            (SingleDecoherenceOperator::Z, SingleDecoherenceOperator::Z) => {
                (SingleDecoherenceOperator::Identity, 1.0)
            }
        };
        result_vec
    }

    /// Conversion function from SingleDecoherenceOperator to SingleQubitOperator.
    ///
    /// # Arguments
    ///
    /// * `decoherence` - SingleDecoherenceOperator to convert to SingleDecoherenceOperator type.
    ///
    /// # Returns
    ///
    /// * `Vec<(SingleDecoherenceOperator, Complex64)>` - Vector of tuples of SingleDecoherenceOperator with a corresponding Complex64 coefficient.
    pub fn decoherence_to_spin(
        decoherence: SingleDecoherenceOperator,
    ) -> (SingleQubitOperator, Complex64) {
        match decoherence {
            SingleDecoherenceOperator::Identity => {
                (SingleQubitOperator::Identity, Complex64::new(1.0, 0.0))
            }
            SingleDecoherenceOperator::X => (SingleQubitOperator::X, Complex64::new(1.0, 0.0)),
            SingleDecoherenceOperator::IY => (SingleQubitOperator::Y, Complex64::new(0.0, 1.0)),
            SingleDecoherenceOperator::Z => (SingleQubitOperator::Z, Complex64::new(1.0, 0.0)),
        }
    }

    /// Conversion function from SingleQubitOperator to SingleDecoherenceOperator.
    ///
    /// # Arguments
    ///
    /// * `spin` - SingleDecoherenceOperator to convert to SingleDecoherenceOperator type.
    ///
    /// # Returns
    ///
    /// * `Vec<(SingleDecoherenceOperator, Complex64)>` - Vector of tuples of SingleDecoherenceOperator with a corresponding Complex64 coefficient.
    pub fn spin_to_decoherence(
        spin: SingleQubitOperator,
    ) -> (SingleDecoherenceOperator, Complex64) {
        match spin {
            SingleQubitOperator::Identity => (
                SingleDecoherenceOperator::Identity,
                Complex64::new(1.0, 0.0),
            ),
            SingleQubitOperator::X => (SingleDecoherenceOperator::X, Complex64::new(1.0, 0.0)),
            SingleQubitOperator::Y => (SingleDecoherenceOperator::IY, Complex64::new(0.0, -1.0)),
            SingleQubitOperator::Z => (SingleDecoherenceOperator::Z, Complex64::new(1.0, 0.0)),
        }
    }

    /// Returns the hermitian conjugate of the DecoherenceOperator.
    ///
    /// # Returns
    ///
    /// `(SingleDecoherenceOperator, f64)` - Tuple of conjugated SingleDecoherenceOperator and float prefactor due to conjugation
    ///                                      (SingleDecoherenceOperator::Minus picks up a minus sign).
    pub fn hermitian_conjugate(&self) -> (Self, f64) {
        match self {
            SingleDecoherenceOperator::Identity => (SingleDecoherenceOperator::Identity, 1.0),
            SingleDecoherenceOperator::Z => (SingleDecoherenceOperator::Z, 1.0),
            SingleDecoherenceOperator::X => (SingleDecoherenceOperator::X, 1.0),
            SingleDecoherenceOperator::IY => (SingleDecoherenceOperator::IY, -1.0),
        }
    }
}

/// DecoherenceProducts are combinations of SingleDecoherenceOperators on specific qubits.
///
/// This is a representation of products of decoherence matrices acting on qubits, in order to build the terms of a hamiltonian.
/// For instance, to represent the term $ \sigma_0^{x} \sigma_2^{z} $ :
/// ` DecoherenceProduct::new().x(0).z(2) `
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use struqture::spins::{DecoherenceProduct, SingleDecoherenceOperator};
///
/// let mut dp = DecoherenceProduct::new();
///
/// // Method 1 to add to DecoherenceProduct:
/// dp = dp.set_pauli(0, SingleDecoherenceOperator::X);
/// // Method 2 to add to DecoherenceProduct
/// dp = dp.z(1);
/// // These methods are equal:
/// assert_eq!(dp.clone().x(2), dp.clone().set_pauli(2, SingleDecoherenceOperator::X));
///
/// // Access what you set:
/// assert_eq!(dp.get(&0).unwrap(), &SingleDecoherenceOperator::X);
/// ```
///
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DecoherenceProduct {
    /// The internal dictionary of pauli matrices (I, X, Y, Z) and qubits
    items: TinyVec<[(usize, SingleDecoherenceOperator); 5]>,
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for DecoherenceProduct {
    fn schema_name() -> String {
        "struqture::spins::DecoherenceProduct".to_string()
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let tmp_schema = gen.subschema_for::<String>();
        let mut obj = tmp_schema.into_object();
        let meta = obj.metadata();
        meta.description = Some("Represents products of Decoherence Operators (X, iY, Z) by a string of spin numbers followed by pauli operators. E.g. 0X10iY13Z14X.".to_string());

        schemars::schema::Schema::Object(obj)
    }
}

impl crate::SerializationSupport for DecoherenceProduct {
    fn struqture_type() -> crate::StruqtureType {
        crate::StruqtureType::DecoherenceProduct
    }
}

/// Implementing serde serialization writing directly to string.
///
impl Serialize for DecoherenceProduct {
    /// Serialization function for DecoherenceProduct according to string type.
    ///
    /// # Arguments
    ///
    /// * `self` - DecoherenceProduct to be serialized.
    /// * `serializer` - Serializer used for serialization.
    ///
    /// # Returns
    ///
    /// `S::Ok` - Serialized instance of DecoherenceProduct.
    /// `S::Error` - Error in the serialization process.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let readable = serializer.is_human_readable();
        if readable {
            serializer.serialize_str(&self.to_string())
        } else {
            let mut sequence = serializer.serialize_seq(Some(self.items.len()))?;
            for item in self.items.iter() {
                sequence.serialize_element(item)?;
            }
            sequence.end()
        }
    }
}

/// Deserializing directly from string.
///
impl<'de> Deserialize<'de> for DecoherenceProduct {
    /// Deserialization function for DecoherenceProduct.
    ///
    /// # Arguments
    ///
    /// * `self` - Serialized instance of DecoherenceProduct to be deserialized.
    /// * `deserializer` - Deserializer used for deserialization.
    ///
    /// # Returns
    ///
    /// `DecoherenceProduct` - Deserialized instance of DecoherenceProduct.
    /// `D::Error` - Error in the deserialization process.
    fn deserialize<D>(deserializer: D) -> Result<DecoherenceProduct, D::Error>
    where
        D: Deserializer<'de>,
    {
        let human_readable = deserializer.is_human_readable();
        if human_readable {
            struct TemporaryVisitor;
            impl<'de> Visitor<'de> for TemporaryVisitor {
                type Value = DecoherenceProduct;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("String")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    DecoherenceProduct::from_str(v).map_err(|err| E::custom(format!("{:?}", err)))
                }

                fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    DecoherenceProduct::from_str(v).map_err(|err| E::custom(format!("{:?}", err)))
                }
            }

            deserializer.deserialize_str(TemporaryVisitor)
        } else {
            struct DecoherenceProductVisitor;
            impl<'de> serde::de::Visitor<'de> for DecoherenceProductVisitor {
                type Value = DecoherenceProduct;
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    fmt::Formatter::write_str(formatter, "Identifier of DecoherenceProduct variant")
                }
                // when variants are marked by String values
                fn visit_seq<M>(self, mut access: M) -> Result<Self::Value, M::Error>
                where
                    M: SeqAccess<'de>,
                {
                    let mut pp = DecoherenceProduct::new();
                    while let Some(item) = access.next_element()? {
                        let entry: Entry = item;
                        pp = pp.set_pauli(entry.0 .0, entry.0 .1);
                    }
                    Ok(pp)
                }
            }
            #[derive(Deserialize)]
            #[serde(transparent)]
            struct Entry((usize, SingleDecoherenceOperator));
            let pp_visitor = DecoherenceProductVisitor;

            deserializer.deserialize_seq(pp_visitor)
        }
    }
}

impl SpinIndex for DecoherenceProduct {
    type SingleSpinType = SingleDecoherenceOperator;

    // From trait
    fn new() -> Self {
        DecoherenceProduct {
            items: TinyVec::<[(usize, SingleDecoherenceOperator); 5]>::with_capacity(5),
        }
    }

    // From trait
    fn set_pauli(self, index: usize, pauli: SingleDecoherenceOperator) -> Self {
        let mut pp = self;
        if let Some((vecindex, insertindex, index_in_use)) =
            pp.items
                .iter()
                .enumerate()
                .find_map(|(vecindex, (innerindex, _))| {
                    if innerindex >= &index {
                        Some((vecindex, *innerindex, innerindex == &index))
                    } else {
                        None
                    }
                })
        {
            if index_in_use {
                match pauli {
                    SingleDecoherenceOperator::Identity => {
                        let _x = pp.items.remove(vecindex);
                    }
                    _ => pp.items[vecindex] = (insertindex, pauli),
                }
            } else {
                match pauli {
                    SingleDecoherenceOperator::Identity => (),
                    _ => {
                        pp.items.insert(vecindex, (index, pauli));
                    }
                }
            }
        } else {
            match pauli {
                SingleDecoherenceOperator::Identity => (),
                _ => {
                    pp.items.push((index, pauli));
                }
            }
        }
        pp
    }

    // From trait
    fn get(&self, index: &usize) -> Option<&SingleDecoherenceOperator> {
        self.items
            .iter()
            .find_map(|(key, value)| if key == index { Some(value) } else { None })
    }

    // From trait
    fn iter(&self) -> std::slice::Iter<(usize, SingleDecoherenceOperator)> {
        self.items.iter()
    }

    // From trait
    fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> DecoherenceProduct {
        let mut mutable_internal: TinyVec<[(usize, SingleDecoherenceOperator); 5]> =
            TinyVec::<[(usize, SingleDecoherenceOperator); 5]>::with_capacity(10);

        for (key, val) in self.iter() {
            mutable_internal.push(match mapping.get(key) {
                Some(x) => (*x, *val),
                None => (*key, *val),
            });
        }
        mutable_internal.sort_by(|(left_index, _), (right_index, _)| {
            left_index
                .partial_cmp(right_index)
                .expect("Cannot compare two unsigned integers internal error in struqture.spins")
        });
        DecoherenceProduct {
            items: mutable_internal,
        }
    }

    // From trait
    fn multiply(left: DecoherenceProduct, right: DecoherenceProduct) -> (Self, Complex64) {
        left * right
    }

    // From trait
    fn concatenate(&self, other: DecoherenceProduct) -> Result<DecoherenceProduct, StruqtureError> {
        let mut return_list = self.items.clone();
        for (key, val) in other.iter() {
            if return_list.iter().any(|(index, _)| index == key) {
                return Err(StruqtureError::ProductIndexAlreadyOccupied { index: *key });
            } else {
                return_list.push((*key, *val));
            }
        }
        return_list.sort_by(|(left_index, _), (right_index, _)| {
            left_index
                .partial_cmp(right_index)
                .expect("Cannot compare two unsigned integers internal error in struqture.spins")
        });
        Ok(DecoherenceProduct { items: return_list })
    }
}

/// Implements Ord for DecoherenceProduct; length then lexicographic sorting
///
/// Using Rust's "Derived" ordering provides only lexicographical ordering.
/// Here we explicitly augment this to include the length of the DecoherenceProduct
/// for comparison. For an example operator set: `[1X, 2iY, 1X2iY, 2X3iY, 1X2X3Z]`,
/// this would be ordered under this definition. Under the old behaviour this
/// set would order as: `[1X, 1X2X3Z, 1X2iY, 2X3iY, 2iY]` which is less readable.
///
/// # Arguments
///
/// * `self` - DecoherenceProduct to be ordered.
///
/// # Returns
///
/// `Ordering` - The ordering result
impl Ord for DecoherenceProduct {
    fn cmp(&self, other: &Self) -> Ordering {
        let me: &TinyVec<[(usize, SingleDecoherenceOperator); 5]> = &(self.items);
        let them: &TinyVec<[(usize, SingleDecoherenceOperator); 5]> = &(other.items);

        match me.len().cmp(&them.len()) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => me.cmp(them), // If lengths are equal use lexicographic
            Ordering::Greater => Ordering::Greater,
        }
    }
}

/// This method returns an ordering between `self` and `other` values if one exists.
impl PartialOrd for DecoherenceProduct {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CorrespondsTo<DecoherenceProduct> for DecoherenceProduct {
    /// Gets the DecoherenceProduct corresponding to self (here, itself).
    ///
    /// # Returns
    ///
    /// * `DecoherenceProduct` - The DecoherenceProduct corresponding to Self.
    fn corresponds_to(&self) -> DecoherenceProduct {
        self.clone()
    }
}

impl SymmetricIndex for DecoherenceProduct {
    // From trait
    fn hermitian_conjugate(&self) -> (Self, f64) {
        // Due to SingleDecoherenceOperator::Minus hermitian_conjugate can produce a constant prefactor
        // Hermitian conjugation with a basis of X, iY, Z and I only leads to a sign change for iY and leaves everything else as is
        // if number of iY is even (% 2 == 0) do nothing if odd (%2 == 1) change sign
        let change_sign: bool = self
            .items
            .iter()
            .map(|(_, b)| match *b {
                SingleDecoherenceOperator::IY => 1_usize,
                _ => 0_usize,
            })
            .sum::<usize>()
            % 2
            == 1;
        (
            Self {
                items: self.items.clone(),
            },
            if change_sign { -1_f64 } else { 1_f64 },
        )
    }

    // From trait
    fn is_natural_hermitian(&self) -> bool {
        let (_, a) = self.hermitian_conjugate();
        // Return true when applying the hermitian conjugation
        // does not change the sign.
        a > 0.0
    }
}

/// Implements the multiplication function of DecoherenceProduct by DecoherenceProduct.
///
impl Mul<DecoherenceProduct> for DecoherenceProduct {
    type Output = (Self, Complex64);
    /// Implement `*` for DecoherenceProduct and DecoherenceProduct.
    ///
    /// # Arguments
    ///
    /// * `other` - The DecoherenceProduct to multiply by.
    ///
    /// # Returns
    ///
    /// * `(Self, Complex64)` - The two DecoherenceProducts multiplied and the resulting prefactor.
    ///
    /// # Panics
    ///
    /// * Unexpectedly failed construction of DecoherenceProduct creation internal struqture bug.
    fn mul(self, rhs: DecoherenceProduct) -> Self::Output {
        let mut factor = Complex64::new(1.0, 0.0);
        let mut return_product = DecoherenceProduct::new();
        for (key, left_operator) in self.clone().into_iter() {
            match rhs.get(&key) {
                Some(right_operator) => {
                    let (tmp_product, tmp_factor) =
                        SingleDecoherenceOperator::multiply(left_operator, *right_operator);
                    factor *= tmp_factor;
                    return_product = return_product.set_pauli(key, tmp_product);
                }
                None => {
                    return_product = return_product.set_pauli(key, left_operator);
                }
            }
        }
        for (key, right_operator) in rhs
            .into_iter()
            .filter(|(key_internal, _)| self.get(key_internal).is_none())
        {
            return_product = return_product.set_pauli(key, right_operator);
        }

        (return_product, factor)
    }
}

impl GetValue<DecoherenceProduct> for DecoherenceProduct {
    type ValueIn = CalculatorComplex;
    type ValueOut = CalculatorComplex;
    /// Gets the key corresponding to the input index (here, itself).
    ///
    /// # Arguments
    ///
    /// * `index` - The index for which to get the corresponding Product.
    ///
    /// # Returns
    ///
    /// * `Self` - The corresponding DecoherenceProduct.
    fn get_key(index: &DecoherenceProduct) -> Self {
        index.clone()
    }

    /// Gets the transformed value corresponding to the input index and value (here, itself).
    ///
    /// # Arguments
    ///
    /// * `index` - The index to transform the value by.
    /// * `value` - The value to be transformed.
    ///
    /// # Returns
    ///
    /// * `CalculatorComplex` - The transformed value.
    fn get_transform(
        _index: &DecoherenceProduct,
        value: qoqo_calculator::CalculatorComplex,
    ) -> qoqo_calculator::CalculatorComplex {
        value
    }
}

impl GetValue<(DecoherenceProduct, DecoherenceProduct)>
    for (DecoherenceProduct, DecoherenceProduct)
{
    type ValueIn = CalculatorComplex;
    type ValueOut = CalculatorComplex;

    /// Gets the key corresponding to the input index (here, itself).
    ///
    /// # Arguments
    ///
    /// * `index` - The index for which to get the corresponding (DecoherenceProduct, DecoherenceProduct).
    ///
    /// # Returns
    ///
    /// * `Self` - The corresponding (DecoherenceProduct, DecoherenceProduct).
    fn get_key(index: &(DecoherenceProduct, DecoherenceProduct)) -> Self {
        index.clone()
    }

    /// Gets the transformed value corresponding to the input index and value (here, itself).
    ///
    /// # Arguments
    ///
    /// * `index` - The index to transform the value by.
    /// * `value` - The value to be transformed.
    ///
    /// # Returns
    ///
    /// * `CalculatorComplex` - The transformed value.
    fn get_transform(
        _index: &(DecoherenceProduct, DecoherenceProduct),
        value: qoqo_calculator::CalculatorComplex,
    ) -> qoqo_calculator::CalculatorComplex {
        value
    }
}

/// Functions for the DecoherenceProduct
///
impl DecoherenceProduct {
    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_export")]
    pub fn to_struqture_1(&self) -> Result<struqture_1::spins::DecoherenceProduct, StruqtureError> {
        let self_string = self.to_string();
        let struqture_1_product = struqture_1::spins::DecoherenceProduct::from_str(&self_string)
            .map_err(|err| StruqtureError::GenericError {
                msg: format!("{}", err),
            })?;
        Ok(struqture_1_product)
    }

    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_import")]
    pub fn from_struqture_1(
        value: &struqture_1::spins::DecoherenceProduct,
    ) -> Result<Self, StruqtureError> {
        let value_string = value.to_string();
        let decoh_product = Self::from_str(&value_string)?;
        Ok(decoh_product)
    }

    /// Sets a new entry for SingleDecoherenceOperator X in the internal dictionary. This function consumes Self.
    ///
    /// # Arguments
    ///
    /// * `index` - Index of set object.
    ///
    /// # Returns
    ///
    /// * `Self` - The entry was correctly set and the DecoherenceProduct is returned.
    pub fn x(self, index: usize) -> Self {
        self.set_pauli(index, SingleDecoherenceOperator::X)
    }

    /// Sets a new entry for SingleDecoherenceOperator IY in the internal dictionary. This function consumes Self.
    ///
    /// # Arguments
    ///
    /// * `index` - Index of set object.
    ///
    /// # Returns
    ///
    /// * `Self` - The entry was correctly set and the DecoherenceProduct is returned.
    pub fn iy(self, index: usize) -> Self {
        self.set_pauli(index, SingleDecoherenceOperator::IY)
    }

    /// Sets a new entry for SingleDecoherenceOperator Z in the internal dictionary. This function consumes Self.
    ///
    /// # Arguments
    ///
    /// * `index` - Index of set object.
    ///
    /// # Returns
    ///
    /// * `Self` - The entry was correctly set and the DecoherenceProduct is returned.
    pub fn z(self, index: usize) -> Self {
        self.set_pauli(index, SingleDecoherenceOperator::Z)
    }

    /// Creates a new DecoherenceProduct with capacity.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) DecoherenceProduct.
    pub fn with_capacity(cap: usize) -> Self {
        DecoherenceProduct {
            items: TinyVec::<[(usize, SingleDecoherenceOperator); 5]>::with_capacity(cap),
        }
    }

    /// Implements COO output for DecoherenceProduct.
    ///
    /// Outputs the DecoherenceProduct as a COO matrix in the form (values, (rows, columns))
    /// where for DecoherenceProduct the values are +/- 1, and the rows and columns are
    /// usize.
    ///
    /// # Returns
    ///
    /// `Result<CooSparseMatrix, StruqtureError>` - The COO matrix or an error.
    pub fn to_coo(&self, number_spins: usize) -> Result<CooSparseMatrix, StruqtureError> {
        // Note much of this function inherits the form of the functions it was
        // based on, the code could probably be shortened but lose readibility.

        // Determine length of the decoherence product:
        let dimension = 2usize.pow(number_spins as u32);

        // Pre allocate all the arrays:
        let mut values: Vec<Complex64> = Vec::with_capacity(dimension);
        let mut rows: Vec<usize> = Vec::with_capacity(dimension);
        let mut columns: Vec<usize> = Vec::with_capacity(dimension);

        for row in 0..dimension {
            let (col, val) = self.sparse_matrix_entries_on_row(row)?;
            rows.push(row);
            columns.push(col);
            values.push(val);
        }
        Ok((values, (rows, columns)))
    }
    /// Constructs the sparse matrix entries for one row of the sparse matrix.
    ///
    /// # Arguments
    ///
    /// * `row` - The row for which to get the entries.
    ///
    /// # Returns
    ///
    /// * `Ok((usize, Complex64))` - The matrix representation of the DecoherenceProduct.
    fn sparse_matrix_entries_on_row(
        &self,
        row: usize,
    ) -> Result<(usize, Complex64), StruqtureError> {
        let mut column = row;
        let mut prefac: Complex64 = 1.0.into();
        for (spin_op_index, pauliop) in self.iter() {
            match *pauliop {
                SingleDecoherenceOperator::X => {
                    match row.div_euclid(2usize.pow(*spin_op_index as u32)) % 2 {
                        0 => column += 2usize.pow(*spin_op_index as u32),
                        1 => column -= 2usize.pow(*spin_op_index as u32),
                        _ => panic!("Internal error in constructing matrix"),
                    }
                }
                SingleDecoherenceOperator::IY => {
                    match row.div_euclid(2usize.pow(*spin_op_index as u32)) % 2 {
                        0 => {
                            column += 2usize.pow(*spin_op_index as u32);
                            prefac *= Complex64::new(1.0, 0.0);
                        }
                        1 => {
                            column -= 2usize.pow(*spin_op_index as u32);
                            prefac *= Complex64::new(-1.0, 0.0);
                        }
                        _ => panic!("Internal error in constructing matrix"),
                    };
                }
                SingleDecoherenceOperator::Z => {
                    match row.div_euclid(2usize.pow(*spin_op_index as u32)) % 2 {
                        0 => {
                            prefac *= Complex64::new(1.0, 0.0);
                        }
                        1 => {
                            prefac *= Complex64::new(-1.0, 0.0);
                        }
                        _ => panic!("Internal error in constructing matrix"),
                    };
                }
                SingleDecoherenceOperator::Identity => (),
            }
        }
        Ok((column, prefac))
    }

    /// Conversion function from DecoherenceProduct to (PauliProduct, Complex64) tuple.
    ///
    /// # Arguments
    ///
    /// * `dp` - DecoherenceProduct to convert to (PauliProduct, Complex64) tuple.
    ///
    /// # Returns
    ///
    /// * `(PauliProduct, Complex64)` - Tuple containing corresponding PauliProduct representation and Complex64 coefficient.
    pub fn decoherence_to_spin(dp: DecoherenceProduct) -> (PauliProduct, Complex64) {
        // Original coefficient is unity.
        let mut coeff = Complex64::new(1.0, 0.0);

        // Capacity will be either original 5 or larger
        let cap = cmp::max(5_usize, dp.len());

        // // Initialize empty pp
        let mut new_pp = PauliProduct::with_capacity(cap);

        //Go over each site and populate the Pauli and modify the coefficient:
        for (site, op) in dp {
            // If any of the operators are iY, we will pick up a new factor of i.
            let (pp, newcoeff) = SingleDecoherenceOperator::decoherence_to_spin(op);
            // Update coefficient, could do a match statement, but I don't think this is the biggest issue.
            coeff *= newcoeff;

            // Set the pauli directly, no checks needed.
            new_pp = new_pp.set_pauli(site, pp);
        }

        (new_pp, coeff)
    }

    /// Conversion function from PauliProduct to (DecoherenceProduct, Complex64) tuple.
    ///
    /// # Arguments
    ///
    /// * `pp` - PauliProduct to convert to (DecoherenceProduct, Complex64) tuple.
    ///
    /// # Returns
    ///
    /// * `(PauliProduct, Complex64)` - Tuple containing corresponding PauliProduct representation and Complex64 coefficient.
    pub fn spin_to_decoherence(pp: PauliProduct) -> (DecoherenceProduct, Complex64) {
        // Original coefficient is unity.
        let mut coeff = Complex64::new(1.0, 0.0);

        // Capacity will be either original 5 or larger
        let cap = cmp::max(5_usize, pp.len());

        // Initialize empty pp with capacity
        let mut new_dp = DecoherenceProduct::with_capacity(cap);

        //Go over each site and populate the Pauli and modify the coefficient:
        for (site, op) in pp {
            // If any of the operators are iY, we will pick up a new factor of i.
            let (dp, newcoeff) = SingleDecoherenceOperator::spin_to_decoherence(op);
            // Update coefficient, could do a match statement, but I don't think this is the biggest issue.
            coeff *= newcoeff;

            // Set the decoherence operator directly, no checks needed.
            new_dp = new_dp.set_pauli(site, dp);
        }

        (new_dp, coeff)
    }
}

/// Implements the default function (Default trait) of DecoherenceProduct (an empty DecoherenceProduct).
///
impl Default for DecoherenceProduct {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for DecoherenceProduct {
    type Err = StruqtureError;
    /// Constructs a DecoherenceProduct from a string.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to convert.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The successfully converted DecoherenceProduct.
    /// * `Err(StruqtureError::IncorrectPauliEntry)` - The pauli matrix being set is not in [\"I\", \"X\", \"Y\", \"Z\"].
    /// * `Err(StruqtureError::FromStringFailed)` - Using {} instead of unsigned integer as spin index.
    /// * `Err(StruqtureError::FromStringFailed)` - At least one spin index is used more than once.
    ///
    /// # Panics
    ///
    /// * Cannot compare two unsigned integers internal error in struqture.spins.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "I" {
            Ok(Self::new()) // If identity it's just an empty decoherence product.
        } else {
            let value = s.to_string();
            let vec_paulis = value.split(char::is_numeric).filter(|s| !s.is_empty());
            let vec_indices = value.split(char::is_alphabetic).filter(|s| !s.is_empty());
            let mut internal: TinyVec<[(usize, SingleDecoherenceOperator); 5]> =
                TinyVec::<[(usize, SingleDecoherenceOperator); 5]>::with_capacity(10);
            for (index, pauli) in vec_indices.zip(vec_paulis) {
                match index.parse() {
                    Ok(num) => {
                        let spin: SingleDecoherenceOperator =
                            SingleDecoherenceOperator::from_str(pauli)?;
                        match spin {
                            SingleDecoherenceOperator::Identity => (),
                            _ => {
                                internal.push((num, spin));
                            }
                        }
                    }
                    Err(_) => {
                        return Err(StruqtureError::FromStringFailed {
                            msg: format!(
                                "Using {} instead of unsigned integer as spin index",
                                index
                            ),
                        })
                    }
                }
            }
            internal.sort_by(|(left_index, _), (right_index, _)| {
                left_index.partial_cmp(right_index).expect(
                    "Cannot compare two unsigned integers internal error in struqture.spins",
                )
            });
            Ok(DecoherenceProduct { items: internal })
        }
    }
}

/// Implements the format function (Display trait) of DecoherenceProduct.
///
impl fmt::Display for DecoherenceProduct {
    /// Formats the DecoherenceProduct using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted DecoherenceProduct.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string: String = String::new();
        if self.items.is_empty() {
            string.push('I');
        } else {
            for (index, pauli) in self.items.iter() {
                string.push_str(format!("{}", index).as_str());
                string.push_str(format!("{}", pauli).as_str());
            }
        }
        write!(f, "{}", string)
    }
}

/// Implements the into_iter function (IntoIterator trait) of DecoherenceProduct.
///
impl IntoIterator for DecoherenceProduct {
    type Item = (usize, SingleDecoherenceOperator);
    type IntoIter = TinyVecIterator<[(usize, SingleDecoherenceOperator); 5]>;
    /// Returns the DecoherenceProduct in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The DecoherenceProduct in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of DecoherenceProduct.
///
impl FromIterator<(usize, SingleDecoherenceOperator)> for DecoherenceProduct {
    /// Returns the object in DecoherenceProduct form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the DecoherenceProduct.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in DecoherenceProduct form.
    fn from_iter<I: IntoIterator<Item = (usize, SingleDecoherenceOperator)>>(iter: I) -> Self {
        let mut pp = DecoherenceProduct::new();
        for (index, pauli) in iter {
            pp = pp.set_pauli(index, pauli);
        }
        pp
    }
}

/// Implements the extend function (Extend trait) of DecoherenceProduct.
///
impl Extend<(usize, SingleDecoherenceOperator)> for DecoherenceProduct {
    /// Extends the DecoherenceProduct by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the DecoherenceProduct.
    fn extend<I: IntoIterator<Item = (usize, SingleDecoherenceOperator)>>(&mut self, iter: I) {
        let mut pp = self.clone();
        for (index, pauli) in iter {
            pp = pp.set_pauli(index, pauli);
        }
        *self = pp;
    }
}

impl JordanWignerSpinToFermion for DecoherenceProduct {
    type Output = FermionOperator;

    /// Implements JordanWignerSpinToFermion for a DecoherenceProduct.
    ///
    /// The convention used is that |0> represents an empty fermionic state (spin-orbital),
    /// and |1> represents an occupied fermionic state.
    ///
    /// # Returns
    ///
    /// `FermionOperator` - The fermionic operator that results from the transformation.
    fn jordan_wigner(&self) -> Self::Output {
        let pp = DecoherenceProduct::decoherence_to_spin(self.clone());
        pp.0.jordan_wigner() * pp.1
    }
}
