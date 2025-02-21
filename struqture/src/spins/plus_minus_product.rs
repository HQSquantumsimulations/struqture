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

use crate::fermions::{FermionOperator, FermionProduct};
use crate::mappings::JordanWignerSpinToFermion;
use crate::prelude::*;
use crate::{StruqtureError, SymmetricIndex};
use num_complex::Complex64;
use qoqo_calculator::*;
use serde::de::{Deserializer, Error, SeqAccess, Visitor};
use serde::ser::{SerializeSeq, Serializer};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::iter::{FromIterator, IntoIterator};
use std::str::FromStr;
use tinyvec::{TinyVec, TinyVecIterator};

use super::{DecoherenceProduct, PauliProduct, SingleDecoherenceOperator, SinglePauliOperator};

const INTERNAL_BUG_ADD_OPERATOR_PRODUCT: &str =
    "Internal bug in add_operator_product for FermionOperator.";
const INTERNAL_BUG_NEW_FERMION_PRODUCT: &str = "Internal bug in FermionProduct::new";

/// Single Spin operators for PlusMinusProducts:
///
/// I: identity matrix
/// $$
/// \begin{pmatrix}
/// 1 & 0\\\\
/// 0 & 1
/// \end{pmatrix}
/// $$
///
/// Plus: sigma plus matrix
/// $$
/// \begin{pmatrix}
/// 0 & 1\\\\
/// 0 & 0
/// \end{pmatrix}
/// $$
///
/// Minus: sigma minus matrix
/// $$
/// \begin{pmatrix}
/// 0 & 0\\\\
/// 1 & 0
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
pub enum SinglePlusMinusOperator {
    Identity,
    Plus,
    Minus,
    Z,
}

/// Creates a SinglePlusMinusOperator from an &str representation.
///
/// # Arguments
///
/// * `s` - The string (&str) to be converted to a SinglePlusMinusOperator.
///
/// # Returns
///
/// * `Ok(Self)` - The SinglePlusMinusOperator of the input string.
/// * `Err(StruqtureError::IncorrectPauliEntry)` - The pauli matrix being set is not in [\"I\", \"X\", \"Y\", \"Z\"].
///
impl FromStr for SinglePlusMinusOperator {
    type Err = StruqtureError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "I" => Ok(SinglePlusMinusOperator::Identity),
            "+" => Ok(SinglePlusMinusOperator::Plus),
            "-" => Ok(SinglePlusMinusOperator::Minus),
            "Z" => Ok(SinglePlusMinusOperator::Z),
            _ => Err(StruqtureError::IncorrectPauliEntry {
                pauli: s.to_string(),
            }),
        }
    }
}

/// Implements the default function (Default trait) of SinglePlusMinusOperator (an Identity SinglePlusMinusOperator).
///
impl Default for SinglePlusMinusOperator {
    fn default() -> Self {
        SinglePlusMinusOperator::Identity
    }
}

/// Implements the fmt function (Display trait) of SinglePlusMinusOperator.
///
impl fmt::Display for SinglePlusMinusOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SinglePlusMinusOperator::Identity => write!(f, "I"),
            SinglePlusMinusOperator::Plus => write!(f, "+"),
            SinglePlusMinusOperator::Minus => write!(f, "-"),
            SinglePlusMinusOperator::Z => write!(f, "Z"),
        }
    }
}

/// Functions for the SinglePlusMinusOperator
///
impl SinglePlusMinusOperator {
    /// Implements multiplication function for a SinglePlusMinusOperator by a SinglePlusMinusOperator.
    ///
    /// # Arguments
    ///
    /// * `left` - left-hand SinglePlusMinusOperator to be multiplied.
    /// * `right` - right-hand SinglePlusMinusOperator to be multiplied.
    pub fn multiply(
        left: SinglePlusMinusOperator,
        right: SinglePlusMinusOperator,
    ) -> Vec<(Self, Complex64)> {
        match (left, right) {
            (SinglePlusMinusOperator::Identity, x) => vec![(x, Complex64::new(1.0, 0.0))],
            (x, SinglePlusMinusOperator::Identity) => vec![(x, Complex64::new(1.0, 0.0))],
            (SinglePlusMinusOperator::Plus, SinglePlusMinusOperator::Plus) => {
                vec![]
            }
            (SinglePlusMinusOperator::Plus, SinglePlusMinusOperator::Minus) => {
                vec![
                    (SinglePlusMinusOperator::Z, Complex64::new(0.5, 0.0)),
                    (SinglePlusMinusOperator::Identity, Complex64::new(0.5, 0.0)),
                ]
            }
            (SinglePlusMinusOperator::Plus, SinglePlusMinusOperator::Z) => {
                vec![(SinglePlusMinusOperator::Plus, Complex64::new(-1.0, 0.0))]
            }
            (SinglePlusMinusOperator::Minus, SinglePlusMinusOperator::Plus) => {
                vec![
                    (SinglePlusMinusOperator::Z, Complex64::new(-0.5, 0.0)),
                    (SinglePlusMinusOperator::Identity, Complex64::new(0.5, 0.0)),
                ]
            }
            (SinglePlusMinusOperator::Minus, SinglePlusMinusOperator::Minus) => {
                vec![]
            }
            (SinglePlusMinusOperator::Minus, SinglePlusMinusOperator::Z) => {
                vec![(SinglePlusMinusOperator::Minus, Complex64::new(1.0, 0.0))]
            }
            (SinglePlusMinusOperator::Z, SinglePlusMinusOperator::Plus) => {
                vec![(SinglePlusMinusOperator::Plus, Complex64::new(1.0, 0.0))]
            }
            (SinglePlusMinusOperator::Z, SinglePlusMinusOperator::Minus) => {
                vec![(SinglePlusMinusOperator::Minus, Complex64::new(-1.0, 0.0))]
            }
            (SinglePlusMinusOperator::Z, SinglePlusMinusOperator::Z) => {
                vec![(SinglePlusMinusOperator::Identity, Complex64::new(1.0, 0.0))]
            }
        }
    }
}

impl From<SinglePlusMinusOperator> for Vec<(SinglePauliOperator, Complex64)> {
    /// Converts a SinglePlusMinusOperator into a vector of tuples of (SinglePauliOperator, Complex64).
    ///
    /// # Arguments
    ///
    /// * `val` - The SinglePlusMinusOperator to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The SinglePlusMinusOperator converted into a vector of tuples of (SinglePauliOperator, Complex64).
    fn from(val: SinglePlusMinusOperator) -> Self {
        match val {
            SinglePlusMinusOperator::Identity => {
                vec![(SinglePauliOperator::Identity, Complex64::new(1.0, 0.0))]
            }
            SinglePlusMinusOperator::Plus => vec![
                (SinglePauliOperator::X, Complex64::new(0.5, 0.0)),
                (SinglePauliOperator::Y, Complex64::new(0.0, 0.5)),
            ],
            SinglePlusMinusOperator::Minus => vec![
                (SinglePauliOperator::X, Complex64::new(0.5, 0.0)),
                (SinglePauliOperator::Y, Complex64::new(0.0, -0.5)),
            ],
            SinglePlusMinusOperator::Z => vec![(SinglePauliOperator::Z, Complex64::new(1.0, 0.0))],
        }
    }
}

impl From<SinglePauliOperator> for Vec<(SinglePlusMinusOperator, Complex64)> {
    /// Converts a SinglePauliOperator into a vector of tuples of (SinglePlusMinusOperator, Complex64).
    ///
    /// # Arguments
    ///
    /// * `val` - The SinglePauliOperator to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The SinglePauliOperator converted into a vector of tuples of (SinglePlusMinusOperator, Complex64).
    fn from(val: SinglePauliOperator) -> Self {
        match val {
            SinglePauliOperator::Identity => {
                vec![(SinglePlusMinusOperator::Identity, Complex64::new(1.0, 0.0))]
            }
            SinglePauliOperator::X => vec![
                (SinglePlusMinusOperator::Plus, Complex64::new(1.0, 0.0)),
                (SinglePlusMinusOperator::Minus, Complex64::new(1.0, 0.0)),
            ],
            SinglePauliOperator::Y => vec![
                (SinglePlusMinusOperator::Plus, Complex64::new(0.0, -1.0)),
                (SinglePlusMinusOperator::Minus, Complex64::new(0.0, 1.0)),
            ],
            SinglePauliOperator::Z => vec![(SinglePlusMinusOperator::Z, Complex64::new(1.0, 0.0))],
        }
    }
}

impl From<SinglePlusMinusOperator> for Vec<(SingleDecoherenceOperator, Complex64)> {
    /// Converts a SinglePlusMinusOperator into a vector of tuples of (SingleDecoherenceOperator, Complex64).
    ///
    /// # Arguments
    ///
    /// * `val` - The SinglePlusMinusOperator to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The SinglePlusMinusOperator converted into a vector of tuples of (SingleDecoherenceOperator, Complex64).
    fn from(val: SinglePlusMinusOperator) -> Self {
        match val {
            SinglePlusMinusOperator::Identity => vec![(
                SingleDecoherenceOperator::Identity,
                Complex64::new(1.0, 0.0),
            )],
            SinglePlusMinusOperator::Plus => vec![
                (SingleDecoherenceOperator::X, Complex64::new(0.5, 0.0)),
                (SingleDecoherenceOperator::IY, Complex64::new(0.5, 0.0)),
            ],
            SinglePlusMinusOperator::Minus => vec![
                (SingleDecoherenceOperator::X, Complex64::new(0.5, 0.0)),
                (SingleDecoherenceOperator::IY, Complex64::new(-0.5, 0.0)),
            ],
            SinglePlusMinusOperator::Z => {
                vec![(SingleDecoherenceOperator::Z, Complex64::new(1.0, 0.0))]
            }
        }
    }
}

impl From<SingleDecoherenceOperator> for Vec<(SinglePlusMinusOperator, Complex64)> {
    /// Converts a SingleDecoherenceOperator into a vector of tuples of (SinglePlusMinusOperator, Complex64).
    ///
    /// # Arguments
    ///
    /// * `val` - The SingleDecoherenceOperator to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The SingleDecoherenceOperator converted into a vector of tuples of (SinglePlusMinusOperator, Complex64).
    fn from(val: SingleDecoherenceOperator) -> Self {
        match val {
            SingleDecoherenceOperator::Identity => {
                vec![(SinglePlusMinusOperator::Identity, Complex64::new(1.0, 0.0))]
            }
            SingleDecoherenceOperator::X => vec![
                (SinglePlusMinusOperator::Plus, Complex64::new(1.0, 0.0)),
                (SinglePlusMinusOperator::Minus, Complex64::new(1.0, 0.0)),
            ],
            SingleDecoherenceOperator::IY => vec![
                (SinglePlusMinusOperator::Plus, Complex64::new(1.0, 0.0)),
                (SinglePlusMinusOperator::Minus, Complex64::new(-1.0, 0.0)),
            ],
            SingleDecoherenceOperator::Z => {
                vec![(SinglePlusMinusOperator::Z, Complex64::new(1.0, 0.0))]
            }
        }
    }
}

impl From<PauliProduct> for Vec<(PlusMinusProduct, Complex64)> {
    /// Converts a PauliProduct into a vector of tuples of (PlusMinusProduct, Complex64).
    ///
    /// # Arguments
    ///
    /// * `value` - The PauliProduct to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The PauliProduct converted into a vector of tuples of (PlusMinusProduct, Complex64).
    fn from(value: PauliProduct) -> Self {
        let mut new_vec: Vec<(PlusMinusProduct, Complex64)> =
            vec![(PlusMinusProduct::new(), Complex64::new(1.0, 0.0))];
        for (index, single) in value.iter() {
            let temp_vec: Vec<(SinglePlusMinusOperator, Complex64)> = (*single).into();
            let mut temp_new_vec: Vec<(PlusMinusProduct, Complex64)> = Vec::new();
            for (new_op, new_prefactor) in temp_vec {
                for (product, prefactor) in new_vec.iter() {
                    let product = product.clone().set_pauli(*index, new_op);
                    temp_new_vec.push((product, new_prefactor * prefactor))
                }
            }
            new_vec = temp_new_vec;
        }
        new_vec
    }
}

impl From<DecoherenceProduct> for Vec<(PlusMinusProduct, Complex64)> {
    /// Converts a DecoherenceProduct into a vector of tuples of (PlusMinusProduct, Complex64).
    ///
    /// # Arguments
    ///
    /// * `value` - The DecoherenceProduct to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The DecoherenceProduct converted into a vector of tuples of (PlusMinusProduct, Complex64).
    fn from(value: DecoherenceProduct) -> Self {
        let mut new_vec: Vec<(PlusMinusProduct, Complex64)> =
            vec![(PlusMinusProduct::new(), Complex64::new(1.0, 0.0))];
        for (index, single) in value.iter() {
            let temp_vec: Vec<(SinglePlusMinusOperator, Complex64)> = (*single).into();
            let mut temp_new_vec: Vec<(PlusMinusProduct, Complex64)> = Vec::new();
            for (new_op, new_prefactor) in temp_vec {
                for (product, prefactor) in new_vec.iter() {
                    let product = product.clone().set_pauli(*index, new_op);
                    temp_new_vec.push((product, new_prefactor * prefactor))
                }
            }
            new_vec = temp_new_vec;
        }
        new_vec
    }
}

impl From<PlusMinusProduct> for Vec<(PauliProduct, Complex64)> {
    /// Converts a PlusMinusProduct into a vector of tuples of (PauliProduct, Complex64).
    ///
    /// # Arguments
    ///
    /// * `value` - The PlusMinusProduct to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The PlusMinusProduct converted into a vector of tuples of (PauliProduct, Complex64).
    fn from(value: PlusMinusProduct) -> Self {
        let mut new_vec: Vec<(PauliProduct, Complex64)> =
            vec![(PauliProduct::new(), Complex64::new(1.0, 0.0))];
        for (index, single) in value.iter() {
            let temp_vec: Vec<(SinglePauliOperator, Complex64)> = (*single).into();
            let mut temp_new_vec: Vec<(PauliProduct, Complex64)> = Vec::new();
            for (new_op, new_prefactor) in temp_vec {
                for (product, prefactor) in new_vec.iter() {
                    let product = product.clone().set_pauli(*index, new_op);
                    temp_new_vec.push((product, new_prefactor * prefactor))
                }
            }
            new_vec = temp_new_vec;
        }
        new_vec
    }
}

impl From<PlusMinusProduct> for Vec<(DecoherenceProduct, Complex64)> {
    /// Converts a PlusMinusProduct into a vector of tuples of (DecoherenceProduct, Complex64).
    ///
    /// # Arguments
    ///
    /// * `value` - The PlusMinusProduct to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The PlusMinusProduct converted into a vector of tuples of (DecoherenceProduct, Complex64).
    fn from(value: PlusMinusProduct) -> Self {
        let mut new_vec: Vec<(DecoherenceProduct, Complex64)> =
            vec![(DecoherenceProduct::new(), Complex64::new(1.0, 0.0))];
        for (index, single) in value.iter() {
            let temp_vec: Vec<(SingleDecoherenceOperator, Complex64)> = (*single).into();
            let mut temp_new_vec: Vec<(DecoherenceProduct, Complex64)> = Vec::new();
            for (new_op, new_prefactor) in temp_vec {
                for (product, prefactor) in new_vec.iter() {
                    let product = product.clone().set_pauli(*index, new_op);
                    temp_new_vec.push((product, new_prefactor * prefactor))
                }
            }
            new_vec = temp_new_vec;
        }
        new_vec
    }
}

/// PlusMinusProducts are combinations of SinglePlusMinusOperators on specific qubits.
///
/// This is a representation of products of sigma plus, sigma minus and sigma z matrices acting on qubits,
/// in order to build the terms of a hamiltonian.
/// For instance, to represent the term $ \sigma_0^{+} \sigma_2^{-} $ :
/// ` PlusMinusProduct::new().plus(0).minus(2) `
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use struqture::spins::{PlusMinusProduct, SinglePlusMinusOperator};
///
/// let mut pp = PlusMinusProduct::new();
///
/// // Method 1 to add to PlusMinusProduct:
/// pp = pp.set_pauli(0, SinglePlusMinusOperator::Plus);
/// // Method 2 to add to PlusMinusProduct:
/// pp = pp.plus(1);
/// // These methods are equal:
/// assert_eq!(pp.clone().plus(2), pp.clone().set_pauli(2, SinglePlusMinusOperator::Plus));
///
/// // Access what you set:
/// assert_eq!(pp.get(&0).unwrap(), &SinglePlusMinusOperator::Plus);
/// ```
///
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PlusMinusProduct {
    /// The internal dictionary of pauli matrices (I, Plus, Minus, Z) and qubits
    items: TinyVec<[(usize, SinglePlusMinusOperator); 5]>,
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for PlusMinusProduct {
    fn schema_name() -> String {
        "struqture::spins::PlusMinusProduct".to_string()
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let tmp_schema = gen.subschema_for::<String>();
        let mut obj = tmp_schema.into_object();
        let meta = obj.metadata();
        meta.description = Some("Represents products of Plus Minus Spin Operators (Plus, Minus, Z) by a string of spin numbers followed by pauli operators. E.g. 0+10-13Z14+.".to_string());

        schemars::schema::Schema::Object(obj)
    }
}

impl crate::SerializationSupport for PlusMinusProduct {
    fn struqture_type() -> crate::StruqtureType {
        crate::StruqtureType::PlusMinusProduct
    }
}

/// Implementing serde serialization writing directly to string.
///
impl Serialize for PlusMinusProduct {
    /// Serialization function for PlusMinusProduct according to string type.
    ///
    /// # Arguments
    ///
    /// * `self` - PlusMinusProduct to be serialized.
    /// * `serializer` - Serializer used for serialization.
    ///
    /// # Returns
    ///
    /// `S::Ok` - Serialized instance of PlusMinusProduct.
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
impl<'de> Deserialize<'de> for PlusMinusProduct {
    /// Deserialization function for PlusMinusProduct.
    ///
    /// # Arguments
    ///
    /// * `self` - Serialized instance of PlusMinusProduct to be deserialized.
    /// * `deserializer` - Deserializer used for deserialization.
    ///
    /// # Returns
    ///
    /// `PlusMinusProduct` - Deserialized instance of PlusMinusProduct.
    /// `D::Error` - Error in the deserialization process.
    fn deserialize<D>(deserializer: D) -> Result<PlusMinusProduct, D::Error>
    where
        D: Deserializer<'de>,
    {
        let human_readable = deserializer.is_human_readable();
        if human_readable {
            struct TemporaryVisitor;
            impl<'de> Visitor<'de> for TemporaryVisitor {
                type Value = PlusMinusProduct;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("String")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    PlusMinusProduct::from_str(v).map_err(|err| E::custom(format!("{:?}", err)))
                }

                fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    PlusMinusProduct::from_str(v).map_err(|err| E::custom(format!("{:?}", err)))
                }
            }

            deserializer.deserialize_str(TemporaryVisitor)
        } else {
            struct PlusMinusProductVisitor;
            impl<'de> serde::de::Visitor<'de> for PlusMinusProductVisitor {
                type Value = PlusMinusProduct;
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    fmt::Formatter::write_str(formatter, "Identifier of PlusMinusProduct variant")
                }
                // when variants are marked by String values
                fn visit_seq<M>(self, mut access: M) -> Result<Self::Value, M::Error>
                where
                    M: SeqAccess<'de>,
                {
                    let mut pp = PlusMinusProduct::new();
                    while let Some(item) = access.next_element()? {
                        let entry: Entry = item;
                        pp = pp.set_pauli(entry.0 .0, entry.0 .1);
                    }
                    Ok(pp)
                }
            }
            #[derive(Deserialize)]
            #[serde(transparent)]
            struct Entry((usize, SinglePlusMinusOperator));
            let pp_visitor = PlusMinusProductVisitor;

            deserializer.deserialize_seq(pp_visitor)
        }
    }
}

impl PlusMinusProduct {
    /// Creates a new Self typed object.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) instance of type `Self`.
    pub fn new() -> Self {
        PlusMinusProduct {
            items: TinyVec::<[(usize, SinglePlusMinusOperator); 5]>::with_capacity(5),
        }
    }

    /// Sets a new entry in Self. This function consumes Self.
    ///
    /// # Arguments
    ///
    /// * `index` - Index of set object.
    /// * `pauli` - Value of set object.
    ///
    /// # Returns
    ///
    /// * `Self` - The entry was correctly set and the new object is returned.
    pub fn set_pauli(self, index: usize, pauli: SinglePlusMinusOperator) -> Self {
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
                    SinglePlusMinusOperator::Identity => {
                        let _x = pp.items.remove(vecindex);
                    }
                    _ => pp.items[vecindex] = (insertindex, pauli),
                }
            } else {
                match pauli {
                    SinglePlusMinusOperator::Identity => (),
                    _ => {
                        pp.items.insert(vecindex, (index, pauli));
                    }
                }
            }
        } else {
            match pauli {
                SinglePlusMinusOperator::Identity => (),
                _ => {
                    pp.items.push((index, pauli));
                }
            }
        }
        pp
    }

    /// Gets the pauli matrix corresponding to the index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of qubit to get the pauli matrix for.
    ///
    /// # Returns
    ///
    /// * `Some(&SinglePlusMinusOperator)` - The key exists and its corresponding value is returned.
    /// * `None` - The key does not exist in Self.
    pub fn get(&self, index: &usize) -> Option<&SinglePlusMinusOperator> {
        self.items
            .iter()
            .find_map(|(key, value)| if key == index { Some(value) } else { None })
    }

    /// Returns the iterator form of Self.
    ///
    /// # Returns
    ///
    /// * `Iter<usize, SinglePlusMinusOperator>` - The iterator form of Self.
    pub fn iter(&self) -> std::slice::Iter<(usize, SinglePlusMinusOperator)> {
        match &self.items {
            TinyVec::Heap(x) => x.iter(),
            TinyVec::Inline(x) => x.iter(),
        }
    }

    /// Returns maximum index in Self.
    ///
    /// # Returns
    ///
    /// * `usize` - Maximum index.
    pub fn current_number_spins(&self) -> usize {
        if let Some((max, _)) = self.iter().last() {
            *max + 1
        } else {
            0
        }
    }

    /// Returns the length of the PlusMinusProduct object.
    ///
    /// # Returns
    ///
    /// * `usize` - The length of the PlusMinusProduct object.
    pub fn len(&self) -> usize {
        self.iter().len()
    }

    /// Returns whether the PlusMinusProduct object is empty or not.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the PlusMinusProduct object is empty or not.
    pub fn is_empty(&self) -> bool {
        self.iter().len() == 0
    }

    /// Remaps the qubits in a clone instance of Self.
    ///
    /// # Arguments
    ///
    /// * `mapping` - The map containing the {qubit: qubit} mapping to use.
    ///
    /// # Returns
    ///
    /// * `Self` -  The new object with the qubits remapped from Self.
    pub fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> PlusMinusProduct {
        let mut mutable_internal: TinyVec<[(usize, SinglePlusMinusOperator); 5]> =
            TinyVec::<[(usize, SinglePlusMinusOperator); 5]>::with_capacity(10);

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
        PlusMinusProduct {
            items: mutable_internal,
        }
    }

    /// Returns the concatenation of two Self typed objects with no overlapping qubits.
    ///
    /// # Arguments
    ///
    /// * `other` - The object to concatenate Self with.
    ///
    /// Returns
    ///
    /// * `Ok(Self)` - The concatenated objects.
    /// * `Err(StruqtureError::ProductIndexAlreadyOccupied)` - Cannot assign pauli matrix to index as it is already occupied.
    pub fn concatenate(&self, other: PlusMinusProduct) -> Result<PlusMinusProduct, StruqtureError> {
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
        Ok(PlusMinusProduct { items: return_list })
    }
}

/// Implements Ord for PlusMinusProduct; length then lexicographic sorting
///
/// Using Rust's "Derived" ordering provides only lexicographical ordering.
/// Here we explicitly augment this to include the length of the PlusMinusProduct
/// for comparison. For an example operator set: `[1+, 2-, 1+2-, 2+3-, 1+2+3-]`,
/// this would be ordered under this definition. Under the old behaviour this
/// set would order as: `[1+, 1+2+3Z, 1+2-, 2+3-, 2-]` which is less readable.
///
/// # Arguments
///
/// * `self` - PlusMinusProduct to be ordered.
///
/// # Returns
///
/// `Ordering` - The ordering result
impl Ord for PlusMinusProduct {
    fn cmp(&self, other: &Self) -> Ordering {
        let me: &TinyVec<[(usize, SinglePlusMinusOperator); 5]> = &(self.items);
        let them: &TinyVec<[(usize, SinglePlusMinusOperator); 5]> = &(other.items);

        match me.len().cmp(&them.len()) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => me.cmp(them), // If lengths are equal use lexicographic
            Ordering::Greater => Ordering::Greater,
        }
    }
}

/// This method returns an ordering between `self` and `other` values if one exists.
impl PartialOrd for PlusMinusProduct {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Functions for the PlusMinusProduct
///
impl PlusMinusProduct {
    /// Sets a new entry for SinglePlusMinusOperator Plus in the internal dictionary. This function consumes Self.
    ///
    /// # Arguments
    ///
    /// * `index` - Index of set object.
    ///
    /// # Returns
    ///
    /// * `Self` - The entry was correctly set and the PlusMinusProduct is returned.
    pub fn plus(self, index: usize) -> Self {
        self.set_pauli(index, SinglePlusMinusOperator::Plus)
    }

    /// Sets a new entry for SinglePlusMinusOperator Minus in the internal dictionary. This function consumes Self.
    ///
    /// # Arguments
    ///
    /// * `index` - Index of set object.
    ///
    /// # Returns
    ///
    /// * `Self` - The entry was correctly set and the PlusMinusProduct is returned.
    pub fn minus(self, index: usize) -> Self {
        self.set_pauli(index, SinglePlusMinusOperator::Minus)
    }

    /// Sets a new entry for SinglePlusMinusOperator Z in the internal dictionary. This function consumes Self.
    ///
    /// # Arguments
    ///
    /// * `index` - Index of set object.
    ///
    /// # Returns
    ///
    /// * `Self` - The entry was correctly set and the PlusMinusProduct is returned.
    pub fn z(self, index: usize) -> Self {
        self.set_pauli(index, SinglePlusMinusOperator::Z)
    }

    /// Creates a new PlusMinusProduct with pre-allocated capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The capacity of the PlusMinusProduct to create.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) PlusMinusProduct.
    pub fn with_capacity(cap: usize) -> Self {
        PlusMinusProduct {
            items: TinyVec::<[(usize, SinglePlusMinusOperator); 5]>::with_capacity(cap),
        }
    }
    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_export")]
    pub fn to_struqture_1(&self) -> Result<struqture_1::spins::PlusMinusProduct, StruqtureError> {
        let self_string = self.to_string();
        let struqture_1_product = struqture_1::spins::PlusMinusProduct::from_str(&self_string)
            .map_err(|err| StruqtureError::GenericError {
                msg: format!("{}", err),
            })?;
        Ok(struqture_1_product)
    }

    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_import")]
    pub fn from_struqture_1(
        value: &struqture_1::spins::PlusMinusProduct,
    ) -> Result<Self, StruqtureError> {
        let value_string = value.to_string();
        let pauli_product = Self::from_str(&value_string)?;
        Ok(pauli_product)
    }
}

impl SymmetricIndex for PlusMinusProduct {
    // From trait
    fn hermitian_conjugate(&self) -> (Self, f64) {
        let mut new_plus_minus = PlusMinusProduct::with_capacity(self.items.len());
        for (index, single) in self.iter() {
            match single {
                SinglePlusMinusOperator::Identity => (),
                SinglePlusMinusOperator::Plus => {
                    new_plus_minus
                        .items
                        .push((*index, SinglePlusMinusOperator::Minus));
                }
                SinglePlusMinusOperator::Minus => {
                    new_plus_minus
                        .items
                        .push((*index, SinglePlusMinusOperator::Plus));
                }
                SinglePlusMinusOperator::Z => {
                    new_plus_minus
                        .items
                        .push((*index, SinglePlusMinusOperator::Z));
                }
            }
        }
        (new_plus_minus, 1.0)
    }

    // From trait
    fn is_natural_hermitian(&self) -> bool {
        self.iter().all(|(_, single)| match single {
            SinglePlusMinusOperator::Identity => true,
            SinglePlusMinusOperator::Plus => false,
            SinglePlusMinusOperator::Minus => false,
            SinglePlusMinusOperator::Z => true,
        })
    }
}

/// Implements the default function (Default trait) of PlusMinusProduct (an empty PlusMinusProduct).
///
impl Default for PlusMinusProduct {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for PlusMinusProduct {
    type Err = StruqtureError;
    /// Constructs a PlusMinusProduct from a string.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to convert.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The successfully converted PlusMinusProduct.
    /// * `Err(StruqtureError::IncorrectPauliEntry)` - The pauli matrix being set is not in [\"I\", \"X\", \"Y\", \"Z\"].
    /// * `Err(StruqtureError::FromStringFailed)` - Using {} instead of unsigned integer as spin index.
    /// * `Err(StruqtureError::FromStringFailed)` - At least one spin index is used more than once.
    ///
    /// # Panics
    ///
    /// * Cannot compare two unsigned integers internal error in struqture.spins.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "I" || s.is_empty() {
            Ok(Self::new()) // If the string is identity then it's an empty PlusMinusProduct
        } else {
            if !s.starts_with(char::is_numeric) {
                return Err(StruqtureError::FromStringFailed {
                    msg: format!(
                        "Missing spin index in the following PlusMinusProduct: {}",
                        s
                    ),
                });
            }
            let mut internal: TinyVec<[(usize, SinglePlusMinusOperator); 5]> =
                TinyVec::<[(usize, SinglePlusMinusOperator); 5]>::with_capacity(10);

            let value = s.to_string();
            let vec_paulis = value.split(char::is_numeric).filter(|s| !s.is_empty());
            let vec_indices = value
                .split(|c| char::is_alphabetic(c) || char::is_ascii_punctuation(&c))
                .filter(|s| !s.is_empty());

            for (index, pauli) in vec_indices.zip(vec_paulis) {
                match index.parse() {
                    Ok(num) => {
                        let spin: SinglePlusMinusOperator =
                            SinglePlusMinusOperator::from_str(pauli)?;
                        match spin {
                            SinglePlusMinusOperator::Identity => (),
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

            // We now check that all the internal elements are strictly increasing after sorting.
            // We could test this criteria while sorting, but this would require augmenting an
            // existing sorting routine, which is fraught with peril, instead we just do a linear
            // iteration using the "overlapping windows" iterator. Non-trivial example:"1+2-3+2-"
            // Note that .all() short-circuits if a single element is false (this is a good thing).
            match internal.windows(2).all(|w| w[0].0 < w[1].0) {
                true => Ok(PlusMinusProduct { items: internal }),
                false => Err(StruqtureError::FromStringFailed {
                    msg: "At least one spin index is used more than once.".to_string(),
                }),
            }
        }
    }
}

/// Implements the format function (Display trait) of PlusMinusProduct.
///
impl fmt::Display for PlusMinusProduct {
    /// Formats the PlusMinusProduct using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted PlusMinusProduct.
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

/// Implements the into_iter function (IntoIterator trait) of PlusMinusProduct.
///
impl IntoIterator for PlusMinusProduct {
    type Item = (usize, SinglePlusMinusOperator);

    type IntoIter = TinyVecIterator<[(usize, SinglePlusMinusOperator); 5]>;
    /// Returns the PlusMinusProduct in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The PlusMinusProduct in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of PlusMinusProduct.
///
impl FromIterator<(usize, SinglePlusMinusOperator)> for PlusMinusProduct {
    /// Returns the object in PlusMinusProduct form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the PlusMinusProduct.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in PlusMinusProduct form.
    fn from_iter<I: IntoIterator<Item = (usize, SinglePlusMinusOperator)>>(iter: I) -> Self {
        let mut pp = PlusMinusProduct::new();
        for (index, pauli) in iter {
            pp = pp.set_pauli(index, pauli);
        }
        pp
    }
}

/// Implements the extend function (Extend trait) of PlusMinusProduct.
///
impl Extend<(usize, SinglePlusMinusOperator)> for PlusMinusProduct {
    /// Extends the PlusMinusProduct by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the PlusMinusProduct.
    fn extend<I: IntoIterator<Item = (usize, SinglePlusMinusOperator)>>(&mut self, iter: I) {
        let mut pp = self.clone();
        for (index, pauli) in iter {
            pp = pp.set_pauli(index, pauli);
        }
        *self = pp;
    }
}

// Helper function to build fermion operators of the form 1 - 2a^{dagger}_pa_p
#[inline]
fn _jw_string_term(i: &usize) -> FermionOperator {
    let mut fermion_id = FermionOperator::new();
    fermion_id
        .add_operator_product(
            FermionProduct::new([], []).expect(INTERNAL_BUG_ADD_OPERATOR_PRODUCT),
            1.0.into(),
        )
        .expect(INTERNAL_BUG_NEW_FERMION_PRODUCT);
    let mut jw_string_term = FermionOperator::new();
    jw_string_term
        .add_operator_product(
            FermionProduct::new([*i], [*i]).expect(INTERNAL_BUG_NEW_FERMION_PRODUCT),
            CalculatorComplex::new(-2.0, 0.0),
        )
        .expect(INTERNAL_BUG_ADD_OPERATOR_PRODUCT);
    fermion_id + jw_string_term
}

impl JordanWignerSpinToFermion for PlusMinusProduct {
    type Output = FermionOperator;

    /// Implements JordanWignerSpinToFermion for a PlusMinusProduct.
    ///
    /// The convention used is that |0> represents an empty fermionic state (spin-orbital),
    /// and |1> represents an occupied fermionic state.
    ///
    /// # Returns
    ///
    /// `FermionOperator` - The fermion operator that results from the transformation.
    ///
    /// # Panics
    ///
    /// * Internal bug in `add_operator_product`
    /// * Internal bug in `FermionProduct::new`
    fn jordan_wigner(&self) -> Self::Output {
        let mut fermion_operator = FermionOperator::new();
        fermion_operator
            .add_operator_product(
                FermionProduct::new([], []).expect(INTERNAL_BUG_ADD_OPERATOR_PRODUCT),
                1.0.into(),
            )
            .expect(INTERNAL_BUG_NEW_FERMION_PRODUCT);

        for (index, op) in self.iter() {
            match op {
                SinglePlusMinusOperator::Plus => {
                    for qubit in 0..*index {
                        fermion_operator = fermion_operator * _jw_string_term(&qubit);
                    }
                    let mut last_term = FermionOperator::new();
                    last_term
                        .add_operator_product(
                            FermionProduct::new([], [*index])
                                .expect(INTERNAL_BUG_NEW_FERMION_PRODUCT),
                            1.0.into(),
                        )
                        .expect(INTERNAL_BUG_ADD_OPERATOR_PRODUCT);
                    fermion_operator = fermion_operator * last_term;
                }
                SinglePlusMinusOperator::Minus => {
                    for qubit in 0..*index {
                        fermion_operator = fermion_operator * _jw_string_term(&qubit);
                    }
                    let mut last_term = FermionOperator::new();
                    last_term
                        .add_operator_product(
                            FermionProduct::new([*index], [])
                                .expect(INTERNAL_BUG_NEW_FERMION_PRODUCT),
                            1.0.into(),
                        )
                        .expect(INTERNAL_BUG_ADD_OPERATOR_PRODUCT);
                    fermion_operator = fermion_operator * last_term;
                }
                SinglePlusMinusOperator::Z => {
                    fermion_operator = fermion_operator * _jw_string_term(index);
                }
                _ => {}
            }
        }
        fermion_operator
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fermions::{FermionOperator, FermionProduct};
    use qoqo_calculator::CalculatorComplex;

    #[test]
    fn test_jw_string_term() {
        let mut fermion_id = FermionOperator::new();
        fermion_id
            .add_operator_product(
                FermionProduct::new([], []).expect(INTERNAL_BUG_ADD_OPERATOR_PRODUCT),
                1.0.into(),
            )
            .expect(INTERNAL_BUG_NEW_FERMION_PRODUCT);
        let fermion_number = FermionProduct::new([3], [3]).unwrap();
        let mut res = FermionOperator::new();
        res.add_operator_product(fermion_number, CalculatorComplex::new(-2.0, 0.0))
            .unwrap();
        res = fermion_id + res;

        assert_eq!(_jw_string_term(&3), res);
    }
}
