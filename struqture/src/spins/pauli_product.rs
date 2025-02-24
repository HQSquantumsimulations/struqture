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
use crate::prelude::*;
use crate::spins::{PauliOperator, PlusMinusOperator};
use crate::{CorrespondsTo, GetValue, SpinIndex, StruqtureError, SymmetricIndex};
use num_complex::Complex64;
use qoqo_calculator::CalculatorComplex;
use serde::de::{Deserializer, Error, SeqAccess, Visitor};
use serde::ser::{SerializeSeq, Serializer};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::iter::{FromIterator, IntoIterator};
use std::ops::Mul;
use std::str::FromStr;
use tinyvec::{TinyVec, TinyVecIterator};
const INTERNAL_BUG_ADD_OPERATOR_PRODUCT: &str = "Internal bug in add_operator_product.";

/// Single Spin operators for PauliProducts:
///
/// I: identity matrix
/// $$
/// \begin{pmatrix}
/// 1 & 0\\\\
/// 0 & 1
/// \end{pmatrix}
/// $$
///
/// X: pauli x matrix
/// $$
/// \begin{pmatrix}
/// 0 & 1\\\\
/// 1 & 0
/// \end{pmatrix}
/// $$
///
/// Y: pauli y matrix
/// $$
/// \begin{pmatrix}
/// 0 & -i\\\\
/// i & 0
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
#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default,
)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub enum SinglePauliOperator {
    #[default]
    Identity,
    X,
    Y,
    Z,
}

/// Creates a SinglePauliOperator from an &str representation.
///
/// # Arguments
///
/// * `s` - The string (&str) to be converted to a SinglePauliOperator.
///
/// # Returns
///
/// * `Ok(Self)` - The SinglePauliOperator of the input string.
/// * `Err(StruqtureError::IncorrectPauliEntry)` - The pauli matrix being set is not in [\"I\", \"X\", \"Y\", \"Z\"].
///
impl FromStr for SinglePauliOperator {
    type Err = StruqtureError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "I" => Ok(SinglePauliOperator::Identity),
            "X" => Ok(SinglePauliOperator::X),
            "Y" => Ok(SinglePauliOperator::Y),
            "Z" => Ok(SinglePauliOperator::Z),
            _ => Err(StruqtureError::IncorrectPauliEntry {
                pauli: s.to_string(),
            }),
        }
    }
}

/// Implements the fmt function (Display trait) of SinglePauliOperator.
///
impl fmt::Display for SinglePauliOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SinglePauliOperator::Identity => write!(f, "I"),
            SinglePauliOperator::X => write!(f, "X"),
            SinglePauliOperator::Y => write!(f, "Y"),
            SinglePauliOperator::Z => write!(f, "Z"),
        }
    }
}

/// Functions for the SinglePauliOperator
///
impl SinglePauliOperator {
    /// Implements multiplication function for a SinglePauliOperator by a SinglePauliOperator.
    ///
    /// # Arguments
    ///
    /// * `left` - left-hand SinglePauliOperator to be multiplied.
    /// * `right` - right-hand SinglePauliOperator to be multiplied.
    pub fn multiply(left: SinglePauliOperator, right: SinglePauliOperator) -> (Self, Complex64) {
        let (result, coeff): (SinglePauliOperator, Complex64) = match (left, right) {
            (SinglePauliOperator::Identity, x) => (x, Complex64::new(1.0, 0.0)),
            (x, SinglePauliOperator::Identity) => (x, Complex64::new(1.0, 0.0)),
            (SinglePauliOperator::X, SinglePauliOperator::X) => {
                (SinglePauliOperator::Identity, Complex64::new(1.0, 0.0))
            }
            (SinglePauliOperator::X, SinglePauliOperator::Y) => {
                (SinglePauliOperator::Z, Complex64::new(0.0, 1.0))
            }
            (SinglePauliOperator::X, SinglePauliOperator::Z) => {
                (SinglePauliOperator::Y, Complex64::new(0.0, -1.0))
            }
            (SinglePauliOperator::Y, SinglePauliOperator::X) => {
                (SinglePauliOperator::Z, Complex64::new(0.0, -1.0))
            }
            (SinglePauliOperator::Y, SinglePauliOperator::Y) => {
                (SinglePauliOperator::Identity, Complex64::new(1.0, 0.0))
            }
            (SinglePauliOperator::Y, SinglePauliOperator::Z) => {
                (SinglePauliOperator::X, Complex64::new(0.0, 1.0))
            }
            (SinglePauliOperator::Z, SinglePauliOperator::X) => {
                (SinglePauliOperator::Y, Complex64::new(0.0, 1.0))
            }
            (SinglePauliOperator::Z, SinglePauliOperator::Y) => {
                (SinglePauliOperator::X, Complex64::new(0.0, -1.0))
            }
            (SinglePauliOperator::Z, SinglePauliOperator::Z) => {
                (SinglePauliOperator::Identity, Complex64::new(1.0, 0.0))
            }
        };
        (result, coeff)
    }
}

/// PauliProducts are combinations of SinglePauliOperators on specific qubits.
///
/// This is a representation of products of pauli matrices acting on qubits, in order to build the terms of a hamiltonian.
/// For instance, to represent the term $ \sigma_0^{x} \sigma_2^{x} $ :
/// ` PauliProduct::new().x(0).x(2) `
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use struqture::spins::{PauliProduct, SinglePauliOperator};
///
/// let mut pp = PauliProduct::new();
///
/// // Method 1 to add to PauliProduct:
/// pp = pp.set_pauli(0, SinglePauliOperator::X);
/// // Method 2 to add to PauliProduct:
/// pp = pp.z(1);
/// // These methods are equal:
/// assert_eq!(pp.clone().x(2), pp.clone().set_pauli(2, SinglePauliOperator::X));
///
/// // Access what you set:
/// assert_eq!(pp.get(&0).unwrap(), &SinglePauliOperator::X);
/// ```
///
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PauliProduct {
    /// The internal dictionary of pauli matrices (I, X, Y, Z) and qubits
    items: TinyVec<[(usize, SinglePauliOperator); 5]>,
}
/// Implementing serde serialization writing directly to string.
///
impl Serialize for PauliProduct {
    /// Serialization function for PauliProduct according to string type.
    ///
    /// # Arguments
    ///
    /// * `self` - PauliProduct to be serialized.
    /// * `serializer` - Serializer used for serialization.
    ///
    /// # Returns
    ///
    /// `S::Ok` - Serialized instance of PauliProduct.
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

#[cfg(feature = "json_schema")]
use schemars;

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for PauliProduct {
    fn schema_name() -> String {
        "struqture::spins::PauliProduct".to_string()
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let tmp_schema = gen.subschema_for::<String>();
        let mut obj = tmp_schema.into_object();
        let meta = obj.metadata();
        meta.description = Some("Represents products of Pauli Operators by a string of spin numbers followed by pauli operators. E.g. 0X10Y13Z14X.".to_string());

        schemars::schema::Schema::Object(obj)
    }
}

impl crate::SerializationSupport for PauliProduct {
    fn struqture_type() -> crate::StruqtureType {
        crate::StruqtureType::PauliProduct
    }
}

/// Deserializing directly from string.
///
impl<'de> Deserialize<'de> for PauliProduct {
    /// Deserialization function for PauliProduct.
    ///
    /// # Arguments
    ///
    /// * `self` - Serialized instance of PauliProduct to be deserialized.
    /// * `deserializer` - Deserializer used for deserialization.
    ///
    /// # Returns
    ///
    /// `PauliProduct` - Deserialized instance of PauliProduct.
    /// `D::Error` - Error in the deserialization process.
    fn deserialize<D>(deserializer: D) -> Result<PauliProduct, D::Error>
    where
        D: Deserializer<'de>,
    {
        let human_readable = deserializer.is_human_readable();
        if human_readable {
            struct TemporaryVisitor;
            impl<'de> Visitor<'de> for TemporaryVisitor {
                type Value = PauliProduct;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("String")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    PauliProduct::from_str(v).map_err(|err| E::custom(format!("{:?}", err)))
                }

                fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    PauliProduct::from_str(v).map_err(|err| E::custom(format!("{:?}", err)))
                }
            }

            deserializer.deserialize_str(TemporaryVisitor)
        } else {
            struct PauliProductVisitor;
            impl<'de> serde::de::Visitor<'de> for PauliProductVisitor {
                type Value = PauliProduct;
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    fmt::Formatter::write_str(formatter, "Identifier of PauliProduct variant")
                }
                // when variants are marked by String values
                fn visit_seq<M>(self, mut access: M) -> Result<Self::Value, M::Error>
                where
                    M: SeqAccess<'de>,
                {
                    let mut pp = PauliProduct::new();
                    while let Some(item) = access.next_element()? {
                        let entry: Entry = item;
                        pp = pp.set_pauli(entry.0 .0, entry.0 .1);
                    }
                    Ok(pp)
                }
            }
            #[derive(Deserialize)]
            #[serde(transparent)]
            struct Entry((usize, SinglePauliOperator));
            let pp_visitor = PauliProductVisitor;

            deserializer.deserialize_seq(pp_visitor)
        }
    }
}

impl SpinIndex for PauliProduct {
    type SingleSpinType = SinglePauliOperator;

    // From trait
    fn new() -> Self {
        PauliProduct {
            items: TinyVec::<[(usize, SinglePauliOperator); 5]>::with_capacity(5),
        }
    }

    // From trait
    fn set_pauli(self, index: usize, pauli: SinglePauliOperator) -> Self {
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
                    SinglePauliOperator::Identity => {
                        let _x = pp.items.remove(vecindex);
                    }
                    _ => pp.items[vecindex] = (insertindex, pauli),
                }
            } else {
                match pauli {
                    SinglePauliOperator::Identity => (),
                    _ => {
                        pp.items.insert(vecindex, (index, pauli));
                    }
                }
            }
        } else {
            match pauli {
                SinglePauliOperator::Identity => (),
                _ => {
                    pp.items.push((index, pauli));
                }
            }
        }
        pp
    }

    // From trait
    fn get(&self, index: &usize) -> Option<&SinglePauliOperator> {
        self.items
            .iter()
            .find_map(|(key, value)| if key == index { Some(value) } else { None })
    }

    // From trait
    fn iter(&self) -> std::slice::Iter<(usize, SinglePauliOperator)> {
        match &self.items {
            TinyVec::Heap(x) => x.iter(),
            TinyVec::Inline(x) => x.iter(),
        }
    }

    // From trait
    fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> PauliProduct {
        let mut mutable_internal: TinyVec<[(usize, SinglePauliOperator); 5]> =
            TinyVec::<[(usize, SinglePauliOperator); 5]>::with_capacity(10);

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
        PauliProduct {
            items: mutable_internal,
        }
    }

    // From trait
    fn multiply(left: PauliProduct, right: PauliProduct) -> (Self, Complex64) {
        left * right
    }

    // From trait
    fn concatenate(&self, other: PauliProduct) -> Result<PauliProduct, StruqtureError> {
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
        Ok(PauliProduct { items: return_list })
    }
}

/// Implements Ord for PauliProducts; length then lexicographic sorting
///
/// Using Rust's "Derived" ordering provides only lexicographical ordering.
/// Here we explicitly augment this to include the length of the PauliProduct
/// for comparison. For an example operator set: `[1X, 2Y, 1X2Y, 2X3Y, 1X2X3Z]`,
/// this would be ordered under this definition. Under the old behaviour this
/// set would order as: `[1X, 1X2X3Z, 1X2Y, 2X3Y, 2Y]` which is less readable.
///
/// # Arguments
///
/// * `self` - PauliProduct to be ordered.
///
/// # Returns
///
/// `Ordering` - The ordering result
impl Ord for PauliProduct {
    fn cmp(&self, other: &Self) -> Ordering {
        let me: &TinyVec<[(usize, SinglePauliOperator); 5]> = &(self.items);
        let them: &TinyVec<[(usize, SinglePauliOperator); 5]> = &(other.items);

        match me.len().cmp(&them.len()) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => me.cmp(them), // If lengths are equal use lexicographic
            Ordering::Greater => Ordering::Greater,
        }
    }
}

/// This method returns an ordering between `self` and `other` values if one exists.
impl PartialOrd for PauliProduct {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CorrespondsTo<PauliProduct> for PauliProduct {
    /// Gets the PauliProduct corresponding to self (here, itself).
    ///
    /// # Returns
    ///
    /// * `PauliProduct` - The PauliProduct corresponding to Self.
    fn corresponds_to(&self) -> PauliProduct {
        self.clone()
    }
}

impl SymmetricIndex for PauliProduct {
    // From trait
    fn hermitian_conjugate(&self) -> (Self, f64) {
        (self.clone(), 1.0)
    }

    // From trait
    fn is_natural_hermitian(&self) -> bool {
        true
    }
}

/// Implements the multiplication function of PauliProduct by PauliProduct.
///
impl Mul<PauliProduct> for PauliProduct {
    type Output = (Self, Complex64);
    /// Implement `*` for PauliProduct and PauliProduct.
    ///
    /// # Arguments
    ///
    /// * `other` - The PauliProduct to multiply by.
    ///
    /// # Returns
    ///
    /// * `(Self, Complex64)` - The two PauliProducts multiplied and the resulting prefactor.
    ///
    /// # Panics
    ///
    /// * Unexpectedly failed construction of PauliProduct creation internal struqture bug.
    fn mul(self, rhs: PauliProduct) -> Self::Output {
        let mut factor = Complex64::new(1.0, 0.0);
        let mut return_product = PauliProduct::new();
        for (key, left_operator) in self.iter() {
            match rhs.get(key) {
                Some(right_operator) => {
                    let (tmp_product, tmp_factor) =
                        SinglePauliOperator::multiply(*left_operator, *right_operator);
                    factor *= tmp_factor;
                    return_product = return_product.set_pauli(*key, tmp_product);
                }
                None => {
                    return_product = return_product.set_pauli(*key, *left_operator);
                }
            }
        }
        for (key, right_operator) in rhs.iter().filter(|(key, _)| self.get(key).is_none()) {
            return_product = return_product.set_pauli(*key, *right_operator);
        }

        (return_product, factor)
    }
}

impl GetValue<PauliProduct> for PauliProduct {
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
    /// * `Self` - The corresponding PauliProduct.
    fn get_key(index: &PauliProduct) -> Self {
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
        _index: &PauliProduct,
        value: qoqo_calculator::CalculatorComplex,
    ) -> qoqo_calculator::CalculatorComplex {
        value
    }
}

/// Functions for the PauliProduct
///
impl PauliProduct {
    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_export")]
    pub fn to_struqture_1(&self) -> Result<struqture_1::spins::PauliProduct, StruqtureError> {
        let self_string = self.to_string();
        let struqture_1_product = struqture_1::spins::PauliProduct::from_str(&self_string)
            .map_err(|err| StruqtureError::GenericError {
                msg: format!("{}", err),
            })?;
        Ok(struqture_1_product)
    }

    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_import")]
    pub fn from_struqture_1(
        value: &struqture_1::spins::PauliProduct,
    ) -> Result<Self, StruqtureError> {
        let value_string = value.to_string();
        let pauli_product = Self::from_str(&value_string)?;
        Ok(pauli_product)
    }

    /// Sets a new entry for SinglePauliOperator X in the internal dictionary. This function consumes Self.
    ///
    /// # Arguments
    ///
    /// * `index` - Index of set object.
    ///
    /// # Returns
    ///
    /// * `Self` - The entry was correctly set and the PauliProduct is returned.
    pub fn x(self, index: usize) -> Self {
        self.set_pauli(index, SinglePauliOperator::X)
    }

    /// Sets a new entry for SinglePauliOperator Y in the internal dictionary. This function consumes Self.
    ///
    /// # Arguments
    ///
    /// * `index` - Index of set object.
    ///
    /// # Returns
    ///
    /// * `Self` - The entry was correctly set and the PauliProduct is returned.
    pub fn y(self, index: usize) -> Self {
        self.set_pauli(index, SinglePauliOperator::Y)
    }

    /// Sets a new entry for SinglePauliOperator Z in the internal dictionary. This function consumes Self.
    ///
    /// # Arguments
    ///
    /// * `index` - Index of set object.
    ///
    /// # Returns
    ///
    /// * `Self` - The entry was correctly set and the PauliProduct is returned.
    pub fn z(self, index: usize) -> Self {
        self.set_pauli(index, SinglePauliOperator::Z)
    }

    /// Creates a new PauliProduct with pre-allocated capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The capacity of the PauliProduct to create.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) PauliProduct.
    pub fn with_capacity(cap: usize) -> Self {
        PauliProduct {
            items: TinyVec::<[(usize, SinglePauliOperator); 5]>::with_capacity(cap),
        }
    }
}

/// Implements the default function (Default trait) of PauliProduct (an empty PauliProduct).
///
impl Default for PauliProduct {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for PauliProduct {
    type Err = StruqtureError;
    /// Constructs a PauliProduct from a string.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to convert.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The successfully converted PauliProduct.
    /// * `Err(StruqtureError::IncorrectPauliEntry)` - The pauli matrix being set is not in [\"I\", \"X\", \"Y\", \"Z\"].
    /// * `Err(StruqtureError::FromStringFailed)` - Using {} instead of unsigned integer as spin index.
    /// * `Err(StruqtureError::FromStringFailed)` - At least one spin index is used more than once.
    ///
    /// # Panics
    ///
    /// * Cannot compare two unsigned integers internal error in struqture.spins.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "I" || s.is_empty() {
            Ok(Self::new()) // If the string is identity then it's an empty PauliProduct
        } else {
            if !s.starts_with(char::is_numeric) {
                return Err(StruqtureError::FromStringFailed {
                    msg: format!("Missing spin index in the following PauliProduct: {}", s),
                });
            }
            let mut internal: TinyVec<[(usize, SinglePauliOperator); 5]> =
                TinyVec::<[(usize, SinglePauliOperator); 5]>::with_capacity(10);

            let value = s.to_string();
            let vec_paulis = value.split(char::is_numeric).filter(|s| !s.is_empty());
            let vec_indices = value.split(char::is_alphabetic).filter(|s| !s.is_empty());

            for (index, pauli) in vec_indices.zip(vec_paulis) {
                match index.parse() {
                    Ok(num) => {
                        let spin: SinglePauliOperator = SinglePauliOperator::from_str(pauli)?;
                        match spin {
                            SinglePauliOperator::Identity => (),
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
            // iteration using the "overlapping windows" iterator. Non-trivial example:"1X2Z3X2Y"
            // Note that .all() short-circuits if a single element is false (this is a good thing).
            match internal.windows(2).all(|w| w[0].0 < w[1].0) {
                true => Ok(PauliProduct { items: internal }),
                false => Err(StruqtureError::FromStringFailed {
                    msg: "At least one spin index is used more than once.".to_string(),
                }),
            }
        }
    }
}

/// Implements the format function (Display trait) of PauliProduct.
///
impl fmt::Display for PauliProduct {
    /// Formats the PauliProduct using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted PauliProduct.
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

/// Implements the into_iter function (IntoIterator trait) of PauliProduct.
///
impl IntoIterator for PauliProduct {
    type Item = (usize, SinglePauliOperator);

    type IntoIter = TinyVecIterator<[(usize, SinglePauliOperator); 5]>;
    /// Returns the PauliProduct in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The PauliProduct in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of PauliProduct.
///
impl FromIterator<(usize, SinglePauliOperator)> for PauliProduct {
    /// Returns the object in PauliProduct form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the PauliProduct.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in PauliProduct form.
    fn from_iter<I: IntoIterator<Item = (usize, SinglePauliOperator)>>(iter: I) -> Self {
        let mut pp = PauliProduct::new();
        for (index, pauli) in iter {
            pp = pp.set_pauli(index, pauli);
        }
        pp
    }
}

/// Implements the extend function (Extend trait) of PauliProduct.
///
impl Extend<(usize, SinglePauliOperator)> for PauliProduct {
    /// Extends the PauliProduct by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the PauliProduct.
    fn extend<I: IntoIterator<Item = (usize, SinglePauliOperator)>>(&mut self, iter: I) {
        let mut pp = self.clone();
        for (index, pauli) in iter {
            pp = pp.set_pauli(index, pauli);
        }
        *self = pp;
    }
}

impl JordanWignerSpinToFermion for PauliProduct {
    type Output = FermionOperator;

    /// Implements JordanWignerSpinToFermion for a PauliProduct.
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
    fn jordan_wigner(&self) -> Self::Output {
        let mut qubit_operator = PauliOperator::new();
        qubit_operator
            .add_operator_product(self.clone(), 1.0.into())
            .expect(INTERNAL_BUG_ADD_OPERATOR_PRODUCT);

        let plus_minus_operator = PlusMinusOperator::from(qubit_operator);
        plus_minus_operator.jordan_wigner()
    }
}
