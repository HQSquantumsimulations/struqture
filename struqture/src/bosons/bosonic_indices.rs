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

use super::BosonIndex;
use crate::{
    CorrespondsTo, CreatorsAnnihilators, GetValue, ModeIndex, StruqtureError, SymmetricIndex,
};
use qoqo_calculator::CalculatorComplex;
use serde::{
    de::{Error, SeqAccess, Visitor},
    ser::SerializeTuple,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{ops::Mul, str::FromStr};
use tinyvec::TinyVec;

/// A product of bosonic creation and annihilation operators.
///
/// The BosonProduct is used as an index for non-hermitian, normal ordered bosonic operators.
/// A bosonic operator can be written as a sum over normal ordered products of creation and annihilation operators.
/// The BosonProduct is used as an index when setting or adding new summands to a bosonic operator and when querrying the
/// weight of a product of operators in the sum.
///
/// # Example
///
/// ```rust
/// use struqture::prelude::*;
/// use struqture::bosons::BosonProduct;
///
/// let b_product = BosonProduct::new([0,0], [0,1]).unwrap();
/// println!("{}", b_product);
///
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct BosonProduct {
    /// The ordered list of creator indices.
    creators: TinyVec<[usize; 2]>,
    /// The ordered list of annihilator indices.
    annihilators: TinyVec<[usize; 2]>,
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for BosonProduct {
    fn schema_name() -> String {
        "BosonProduct".to_string()
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let tmp_schema = gen.subschema_for::<String>();
        let mut obj = tmp_schema.into_object();
        let meta = obj.metadata();
        meta.description = Some("Represents products of Bosonic creators and annhilators by a string creators (c) or annihilators (a) followed by the modes they are acting on. E.g. c0a1.".to_string());

        schemars::schema::Schema::Object(obj)
    }
}

impl crate::MinSupportedVersion for BosonProduct {}

/// Implementing serde serialization writing directly to string.
///
impl Serialize for BosonProduct {
    /// Serialization function for BosonProduct according to string type.
    ///
    /// # Arguments
    ///
    /// * `self` - BosonProduct to be serialized.
    /// * `serializer` - Serializer used for serialization.
    ///
    /// # Returns
    ///
    /// `S::Ok` - Serialized instance of BosonProduct.
    /// `S::Error` - Error in the serialization process.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let readable = serializer.is_human_readable();
        if readable {
            serializer.serialize_str(&self.to_string())
        } else {
            let mut tuple = serializer.serialize_tuple(2)?;
            tuple.serialize_element(&self.creators)?;
            tuple.serialize_element(&self.annihilators)?;
            tuple.end()
        }
    }
}

/// Deserializing directly from string.
///
impl<'de> Deserialize<'de> for BosonProduct {
    /// Deserialization function for BosonProduct.
    ///
    /// # Arguments
    ///
    /// * `self` - Serialized instance of BosonProduct to be deserialized.
    /// * `deserializer` - Deserializer used for deserialization.
    ///
    /// # Returns
    ///
    /// `DecoherenceProduct` - Deserialized instance of BosonProduct.
    /// `D::Error` - Error in the deserialization process.
    fn deserialize<D>(deserializer: D) -> Result<BosonProduct, D::Error>
    where
        D: Deserializer<'de>,
    {
        let human_readable = deserializer.is_human_readable();
        if human_readable {
            struct TemporaryVisitor;
            impl<'de> Visitor<'de> for TemporaryVisitor {
                type Value = BosonProduct;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("String")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    BosonProduct::from_str(v).map_err(|err| E::custom(format!("{:?}", err)))
                }

                fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    BosonProduct::from_str(v).map_err(|err| E::custom(format!("{:?}", err)))
                }
            }

            deserializer.deserialize_str(TemporaryVisitor)
        } else {
            struct BosonProductVisitor;
            impl<'de> serde::de::Visitor<'de> for BosonProductVisitor {
                type Value = BosonProduct;
                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    std::fmt::Formatter::write_str(
                        formatter,
                        "Tuple of two sequences of unsigned integers",
                    )
                }
                fn visit_seq<M>(self, mut access: M) -> Result<Self::Value, M::Error>
                where
                    M: SeqAccess<'de>,
                {
                    let creators: TinyVec<[usize; 2]> = match access.next_element()? {
                        Some(x) => x,
                        None => {
                            return Err(M::Error::custom("Missing creator sequence".to_string()));
                        }
                    };
                    let annihilators: TinyVec<[usize; 2]> = match access.next_element()? {
                        Some(x) => x,
                        None => {
                            return Err(M::Error::custom(
                                "Missing annihilator sequence".to_string(),
                            ));
                        }
                    };

                    BosonProduct::new(creators, annihilators).map_err(M::Error::custom)
                }
            }
            let pp_visitor = BosonProductVisitor;

            deserializer.deserialize_tuple(2, pp_visitor)
        }
    }
}

impl ModeIndex for BosonProduct {
    /// Creates a new BosonProduct.
    ///
    /// # Arguments
    ///
    /// * `creators` - The creator indices to have in the BosonProduct.
    /// * `annihilators` - The annihilators indices to have in the BosonProduct.
    ///
    /// # Returns
    ///
    /// * `Ok(BosonProduct)` - The new BosonProduct with the given creators and annihilators.
    fn new(
        creators: impl IntoIterator<Item = usize>,
        annihilators: impl IntoIterator<Item = usize>,
    ) -> Result<Self, StruqtureError> {
        let mut creators: TinyVec<[usize; 2]> = creators.into_iter().collect();
        creators.sort_unstable();
        let mut annihilators: TinyVec<[usize; 2]> = annihilators.into_iter().collect();
        annihilators.sort_unstable();

        Ok(Self {
            creators: creators.iter().copied().collect(),
            annihilators: annihilators.iter().copied().collect(),
        })
    }

    // From trait
    fn creators(&self) -> std::slice::Iter<usize> {
        self.creators.iter()
    }

    // From trait
    fn annihilators(&self) -> std::slice::Iter<usize> {
        self.annihilators.iter()
    }

    /// Creates a pair (BosonProduct, CalculatorComplex).
    ///
    /// The first item is the valid BosonProduct created from the input creators and annihilators.
    /// The second term is the input CalculatorComplex transformed according to the valid order of creators and annihilators.
    ///
    /// # Arguments
    ///
    /// * `creators` - The creator indices to have in the BosonProduct.
    /// * `annihilators` - The annihilators indices to have in the BosonProduct.
    /// * `value` - The CalculatorComplex to transform.
    ///
    /// # Returns
    ///
    /// * `Ok((BosonProduct, CalculatorComplex))` - The valid BosonProduct and the corresponding transformed CalculatorComplex.
    fn create_valid_pair(
        creators: impl IntoIterator<Item = usize>,
        annihilators: impl IntoIterator<Item = usize>,
        value: qoqo_calculator::CalculatorComplex,
    ) -> Result<(Self, qoqo_calculator::CalculatorComplex), StruqtureError> {
        let mut creators: TinyVec<[usize; 2]> = creators.into_iter().collect();
        creators.sort_unstable();
        let mut annihilators: TinyVec<[usize; 2]> = annihilators.into_iter().collect();
        annihilators.sort_unstable();
        Ok((
            Self {
                creators,
                annihilators,
            },
            value,
        ))
    }
}

impl BosonIndex for BosonProduct {}

impl CorrespondsTo<BosonProduct> for BosonProduct {
    /// Gets the BosonProduct corresponding to self (here, itself).
    ///
    /// # Returns
    ///
    /// * `BosonProduct` - The BosonProduct corresponding to Self.
    fn corresponds_to(&self) -> BosonProduct {
        self.clone()
    }
}

impl CorrespondsTo<HermitianBosonProduct> for BosonProduct {
    /// Gets the HermitianBosonProduct corresponding to Self.
    ///
    /// # Returns
    ///
    /// * `HermitianBosonProduct` - The HermitianBosonProduct corresponding to Self.
    fn corresponds_to(&self) -> HermitianBosonProduct {
        if self.creators().min() > self.annihilators().min() {
            HermitianBosonProduct {
                creators: self.annihilators.clone(),
                annihilators: self.creators.clone(),
            }
        } else {
            HermitianBosonProduct {
                creators: self.creators.clone(),
                annihilators: self.annihilators.clone(),
            }
        }
    }
}

impl SymmetricIndex for BosonProduct {
    // From trait
    fn hermitian_conjugate(&self) -> (Self, f64) {
        (
            Self {
                annihilators: self.creators.clone(),
                creators: self.annihilators.clone(),
            },
            1.0,
        )
    }

    // From trait
    fn is_natural_hermitian(&self) -> bool {
        self.creators == self.annihilators
    }
}

/// Implements the multiplication function of BosonProduct by BosonProduct.
///
impl Mul<BosonProduct> for BosonProduct {
    type Output = Vec<BosonProduct>;
    /// Implement `*` for BosonProduct and BosonProduct.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The BosonProduct to multiply by.
    ///
    /// # Returns
    ///
    /// * `Vec<BosonProduct>` - The two BosonProducts multiplied.
    ///
    /// # Panics
    ///
    /// * Unexpectedly failed construction of BosonProduct creation, internal struqture bug (create_valid_pair).
    fn mul(self, rhs: BosonProduct) -> Self::Output {
        let mut output_vec: Vec<BosonProduct> = Vec::new();

        let commuted_creators_annihilators =
            commute_creator_annihilator(&self.annihilators, &rhs.creators);
        for (new_creators, mut new_annihilators) in commuted_creators_annihilators {
            let mut tmp_creators = self.creators.clone();
            tmp_creators.extend(new_creators.into_iter());
            new_annihilators.extend(rhs.annihilators().copied());
            let (tmp_boson_product, _) = BosonProduct::create_valid_pair(
                tmp_creators,
                new_annihilators,
                1.0.into(),
            )
            .expect(
                "Unexpectedly failed construction of BosonProduct creation, internal struqture bug (create_valid_pair)",
            );
            output_vec.push(tmp_boson_product);
        }
        output_vec
    }
}

impl Mul<Vec<BosonProduct>> for BosonProduct {
    type Output = Vec<BosonProduct>;
    /// Implement `*` for BosonProduct and a vector of BosonProducts.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The vector of BosonProducts to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - The result of the multiplication.
    ///
    /// # Panics
    ///
    /// * Unexpectedly failed construction of BosonProduct creation internal struqture bug.
    fn mul(self, rhs: Vec<BosonProduct>) -> Self::Output {
        let mut output_vec: Vec<BosonProduct> = Vec::new();
        for rh_bp in rhs.iter() {
            output_vec.append(&mut (self.clone() * rh_bp.clone()))
        }
        output_vec
    }
}

impl Mul<BosonProduct> for Vec<BosonProduct> {
    type Output = Vec<BosonProduct>;
    /// Implement `*` for a vector of BosonProducts and a BosonProduct.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The BosonProduct to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - The result of the multiplication.
    ///
    /// # Panics
    ///
    /// * Unexpectedly failed construction of BosonProduct creation internal struqture bug.
    fn mul(self, rhs: BosonProduct) -> Self::Output {
        let mut output_vec: Vec<BosonProduct> = Vec::new();
        for lh_bp in self {
            output_vec.append(&mut (lh_bp * rhs.clone()))
        }
        output_vec
    }
}

impl Mul<HermitianBosonProduct> for BosonProduct {
    type Output = Vec<BosonProduct>;
    /// Implement `*` for a BosonProduct and a HermitianBosonProduct.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The HermitianBosonProduct to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - The result of the multiplication.
    ///
    /// # Panics
    ///
    /// * Could not convert rhs into a BosonProduct.
    /// * Unexpectedly failed construction of BosonProduct creation internal struqture bug.
    fn mul(self, rhs: HermitianBosonProduct) -> Self::Output {
        let mut output_vec: Vec<BosonProduct> = Vec::new();

        let mut right_to_mul: Vec<BosonProduct> = Vec::new();
        let hbp_to_bp = BosonProduct::new(rhs.creators, rhs.annihilators)
            .expect("Could not convert rhs into a BosonProduct");
        right_to_mul.push(hbp_to_bp.clone());
        if !hbp_to_bp.is_natural_hermitian() {
            right_to_mul.push(hbp_to_bp.hermitian_conjugate().0);
        }

        for right in right_to_mul {
            output_vec.append(&mut (self.clone() * right));
        }
        output_vec
    }
}

/// Trait for transforming value stored at index I when using index of different type T to read out value
///
impl GetValue<BosonProduct> for BosonProduct {
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
    /// * `Self` - The corresponding BosonProduct.
    fn get_key(index: &BosonProduct) -> Self {
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
        _index: &BosonProduct,
        value: qoqo_calculator::CalculatorComplex,
    ) -> qoqo_calculator::CalculatorComplex {
        value
    }
}

/// Implements the format function (Display trait) of BosonProduct.
///
impl std::fmt::Display for BosonProduct {
    /// Formats the BosonProduct using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted BosonProduct.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string: String = String::new();

        if self.creators.is_empty() & self.annihilators.is_empty() {
            string.push('I');
        } else {
            for index in self.creators() {
                string.push_str(format!("c{}", index).as_str());
            }
            for index in self.annihilators() {
                string.push_str(format!("a{}", index).as_str());
            }
        }
        write!(f, "{}", string)
    }
}

impl FromStr for BosonProduct {
    type Err = StruqtureError;
    /// Constructs a BosonProduct from a string.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to convert.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The successfully converted BosonProduct.
    /// * `Err(StruqtureError::FromStringFailed)` - Used operator that is neither 'c' nor 'a'.
    /// * `Err(StruqtureError::FromStringFailed)` - Index in given creators or annihilators is not an integer.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "I" {
            Self::new([], [])
        } else {
            let mut creators: TinyVec<[usize; 2]> = TinyVec::<[usize; 2]>::with_capacity(2);
            let mut annihilators: TinyVec<[usize; 2]> = TinyVec::<[usize; 2]>::with_capacity(2);

            let operators = s.split(char::is_numeric).filter(|s| !s.is_empty());
            let indices = s.split(char::is_alphabetic).filter(|s| !s.is_empty());
            let mut parsing_creators: bool = true;
            for (index, op) in indices.zip(operators) {
                match index.parse() {
                    Ok(num) => {
                        match op{
                            "c" => {if parsing_creators{ creators.push(num);} else{return Err(StruqtureError::IndicesNotNormalOrdered{index_i: num, index_j: num+1})}}
                            "a" => {annihilators.push(num); parsing_creators = false;}
                            _ => return Err(StruqtureError::FromStringFailed{msg: format!("Used operator {} that is neither 'c' nor 'a' in BosonProduct::from_str", op)})
                        }
                    }
                    Err(_) => return Err(StruqtureError::FromStringFailed{msg: format!("Index in given creators or annihilators is not an integer: {}", index)}),
                }
            }
            Self::new(creators, annihilators)
        }
    }
}

/// A hermitian product of bosonic creation and annihilation operators
///
/// The HermitianBosonProduct is used as an index for hermitian, normal ordered bosonic operators. It stores the input creation and annihilation operators,
/// but also implicitly stores its hermitian conjugate, e.g: c_0 c_1 a_0 a_2 + a_2 a_0 c_1 c_0.
/// A bosonic operator can be written as a sum over normal ordered products of creation and annihilation operators.
/// The HermitianBosonProduct is used as an index when setting or adding new summands to a hermitian bosonic operator and when querying the
/// weight of a product of operators in the sum.
///
/// # Example
///
/// ```rust
/// use struqture::prelude::*;
/// use struqture::bosons::HermitianBosonProduct;
///
/// let b_product = HermitianBosonProduct::new([0,0], [0,1]).unwrap();
/// println!("{}", b_product);
///
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct HermitianBosonProduct {
    /// The ordered list of creator indices.
    creators: TinyVec<[usize; 2]>,
    /// The ordered list of annihilator indices.
    annihilators: TinyVec<[usize; 2]>,
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for HermitianBosonProduct {
    fn schema_name() -> String {
        "HermitianBosonProduct".to_string()
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let tmp_schema = gen.subschema_for::<String>();
        let mut obj = tmp_schema.into_object();
        let meta = obj.metadata();
        meta.description = Some("Represents products of Bosonic creators and annhilators by a string creators (c) or annihilators (a) followed by the modes they are acting on. E.g. c0a1.".to_string());

        schemars::schema::Schema::Object(obj)
    }
}

impl crate::MinSupportedVersion for HermitianBosonProduct {}

/// Implementing serde serialization writing directly to string.
///
impl Serialize for HermitianBosonProduct {
    /// Serialization function for HermitianBosonProduct according to string type.
    ///
    /// # Arguments
    ///
    /// * `self` - HermitianBosonProduct to be serialized.
    /// * `serializer` - Serializer used for serialization.
    ///
    /// # Returns
    ///
    /// `S::Ok` - Serialized instance of HermitianBosonProduct.
    /// `S::Error` - Error in the serialization process.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let readable = serializer.is_human_readable();
        if readable {
            serializer.serialize_str(&self.to_string())
        } else {
            let mut tuple = serializer.serialize_tuple(2)?;
            tuple.serialize_element(&self.creators)?;
            tuple.serialize_element(&self.annihilators)?;
            tuple.end()
        }
    }
}

/// Deserializing directly from string.
///
impl<'de> Deserialize<'de> for HermitianBosonProduct {
    /// Deserialization function for HermitianBosonProduct.
    ///
    /// # Arguments
    ///
    /// * `self` - Serialized instance of HermitianBosonProduct to be deserialized.
    /// * `deserializer` - Deserializer used for deserialization.
    ///
    /// # Returns
    ///
    /// `DecoherenceProduct` - Deserialized instance of HermitianBosonProduct.
    /// `D::Error` - Error in the deserialization process.
    fn deserialize<D>(deserializer: D) -> Result<HermitianBosonProduct, D::Error>
    where
        D: Deserializer<'de>,
    {
        let human_readable = deserializer.is_human_readable();
        if human_readable {
            struct TemporaryVisitor;
            impl<'de> Visitor<'de> for TemporaryVisitor {
                type Value = HermitianBosonProduct;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("String")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    HermitianBosonProduct::from_str(v)
                        .map_err(|err| E::custom(format!("{:?}", err)))
                }

                fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    HermitianBosonProduct::from_str(v)
                        .map_err(|err| E::custom(format!("{:?}", err)))
                }
            }

            deserializer.deserialize_str(TemporaryVisitor)
        } else {
            struct BosonProductVisitor;
            impl<'de> serde::de::Visitor<'de> for BosonProductVisitor {
                type Value = HermitianBosonProduct;
                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    std::fmt::Formatter::write_str(
                        formatter,
                        "Tuple of two sequences of unsigned integers",
                    )
                }
                fn visit_seq<M>(self, mut access: M) -> Result<Self::Value, M::Error>
                where
                    M: SeqAccess<'de>,
                {
                    let creators: TinyVec<[usize; 2]> = match access.next_element()? {
                        Some(x) => x,
                        None => {
                            return Err(M::Error::custom("Missing creator sequence".to_string()));
                        }
                    };
                    let annihilators: TinyVec<[usize; 2]> = match access.next_element()? {
                        Some(x) => x,
                        None => {
                            return Err(M::Error::custom(
                                "Missing annihilator sequence".to_string(),
                            ));
                        }
                    };

                    HermitianBosonProduct::new(creators, annihilators).map_err(M::Error::custom)
                }
            }
            let pp_visitor = BosonProductVisitor;

            deserializer.deserialize_tuple(2, pp_visitor)
        }
    }
}

impl ModeIndex for HermitianBosonProduct {
    /// Creates a new HermitianBosonProduct.
    ///
    /// # Arguments
    ///
    /// * `creators` - The creator indices to have in the HermitianBosonProduct.
    /// * `annihilators` - The annihilators indices to have in the HermitianBosonProduct.
    ///
    /// # Returns
    ///
    /// * `Ok(HermitianBosonProduct)` - The new HermitianBosonProduct with the given creators and annihilators.
    /// * `Err(StruqtureError::CreatorsAnnihilatorsMinimumIndex)` - The minimum index of the creators is larger than the minimum index of the annihilators.
    fn new(
        creators: impl IntoIterator<Item = usize>,
        annihilators: impl IntoIterator<Item = usize>,
    ) -> Result<Self, StruqtureError> {
        let mut creators: TinyVec<[usize; 2]> = creators.into_iter().collect();
        creators.sort_unstable();
        let mut annihilators: TinyVec<[usize; 2]> = annihilators.into_iter().collect();
        annihilators.sort_unstable();
        let mut number_equal_indices = 0;
        for (creator, annihilator) in creators.iter().zip(annihilators.iter()) {
            match annihilator.cmp(creator) {
                std::cmp::Ordering::Less => {
                    return Err(StruqtureError::CreatorsAnnihilatorsMinimumIndex {
                        creators_min: Some(*creator),
                        annihilators_min: Some(*annihilator),
                    });
                }
                std::cmp::Ordering::Greater => {
                    break;
                }
                _ => {
                    number_equal_indices += 1;
                }
            }
        }
        if creators.len() > number_equal_indices && annihilators.len() == number_equal_indices {
            return Err(StruqtureError::CreatorsAnnihilatorsMinimumIndex {
                creators_min: creators.iter().nth(number_equal_indices).copied(),
                annihilators_min: None,
            });
        }
        Ok(Self {
            creators: creators.iter().copied().collect(),
            annihilators: annihilators.iter().copied().collect(),
        })
    }

    /// Gets the creator indices of the HermitianBosonProduct.
    ///
    /// # Returns
    ///
    /// * `usize` - The creator indices in the HermitianBosonProduct.
    fn creators(&self) -> std::slice::Iter<usize> {
        self.creators.iter()
    }

    /// Gets the annihilator indices of the HermitianBosonProduct.
    ///
    /// # Returns
    ///
    /// * `usize` - The annihilator indices in the HermitianBosonProduct.
    fn annihilators(&self) -> std::slice::Iter<usize> {
        self.annihilators.iter()
    }

    /// Creates a pair (HermitianBosonProduct, CalculatorComplex).
    ///
    /// The first item is the valid HermitianBosonProduct created from the input creators and annihilators.
    /// The second term is the input CalculatorComplex transformed according to the valid order of creators and annihilators.
    ///
    /// # Arguments
    ///
    /// * `creators` - The creator indices to have in the HermitianBosonProduct.
    /// * `annihilators` - The annihilators indices to have in the HermitianBosonProduct.
    /// * `value` - The CalculatorComplex to transform.
    ///
    /// # Returns
    ///
    /// * `Ok((HermitianBosonProduct, CalculatorComplex))` - The valid HermitianBosonProduct and the corresponding transformed CalculatorComplex.
    fn create_valid_pair(
        creators: impl IntoIterator<Item = usize>,
        annihilators: impl IntoIterator<Item = usize>,
        value: qoqo_calculator::CalculatorComplex,
    ) -> Result<(Self, qoqo_calculator::CalculatorComplex), StruqtureError> {
        let mut creators: TinyVec<[usize; 2]> = creators.into_iter().collect();
        creators.sort_unstable();
        let mut annihilators: TinyVec<[usize; 2]> = annihilators.into_iter().collect();
        annihilators.sort_unstable();
        let mut hermitian_conjugate = false;
        let mut number_equal_indices = 0;
        for (creator, annihilator) in creators.iter().zip(annihilators.iter()) {
            match annihilator.cmp(creator) {
                std::cmp::Ordering::Less => {
                    hermitian_conjugate = true;
                    break;
                }
                std::cmp::Ordering::Greater => break,
                _ => {
                    number_equal_indices += 1;
                }
            }
        }

        if creators.len() > number_equal_indices && annihilators.len() == number_equal_indices {
            hermitian_conjugate = true;
        }
        if hermitian_conjugate {
            Ok((
                Self {
                    creators: annihilators,
                    annihilators: creators,
                },
                value.conj(),
            ))
        } else {
            Ok((
                Self {
                    creators,
                    annihilators,
                },
                value,
            ))
        }
    }
}

impl BosonIndex for HermitianBosonProduct {}

impl CorrespondsTo<HermitianBosonProduct> for HermitianBosonProduct {
    /// Gets the HermitianBosonProduct corresponding to self (here, itself).
    ///
    /// # Returns
    ///
    /// * `HermitianBosonProduct` - The HermitianBosonProduct corresponding to Self.
    fn corresponds_to(&self) -> HermitianBosonProduct {
        self.clone()
    }
}

impl CorrespondsTo<BosonProduct> for HermitianBosonProduct {
    /// Gets the BosonProduct corresponding to Self.
    ///
    /// # Returns
    ///
    /// * `BosonProduct` - The BosonProduct corresponding to Self.
    fn corresponds_to(&self) -> BosonProduct {
        BosonProduct {
            creators: self.creators.clone(),
            annihilators: self.annihilators.clone(),
        }
    }
}

impl SymmetricIndex for HermitianBosonProduct {
    // From trait
    fn hermitian_conjugate(&self) -> (Self, f64) {
        (self.clone(), 1.0)
    }

    // From trait
    fn is_natural_hermitian(&self) -> bool {
        self.creators == self.annihilators
    }
}

/// Implements the multiplication function of HermitianBosonProduct by HermitianBosonProduct.
///
impl Mul<HermitianBosonProduct> for HermitianBosonProduct {
    type Output = Vec<BosonProduct>;

    /// Implement `*` for HermitianBosonProduct and HermitianBosonProduct.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The HermitianBosonProduct to multiply by.
    ///
    /// # Returns
    ///
    /// * `Vec<BosonProduct>` - The two HermitianBosonProducts multiplied.
    ///
    /// # Panics
    ///
    /// * Could not convert self into a BosonProduct.
    /// * Could not convert rhs into a BosonProduct.
    /// * Unexpectedly failed construction of BosonProduct creation, internal struqture bug.
    fn mul(self, rhs: HermitianBosonProduct) -> Self::Output {
        let mut output_vec: Vec<BosonProduct> = Vec::new();

        let mut left_to_mul: Vec<BosonProduct> = Vec::new();
        let bp_left = BosonProduct::new(self.creators, self.annihilators)
            .expect("Could not convert self into a BosonProduct");
        left_to_mul.push(bp_left.clone());
        if !bp_left.is_natural_hermitian() {
            left_to_mul.push(bp_left.hermitian_conjugate().0);
        }

        let mut right_to_mul: Vec<BosonProduct> = Vec::new();
        let bp_right = BosonProduct::new(rhs.creators, rhs.annihilators)
            .expect("Could not convert rhs into a BosonProduct");
        right_to_mul.push(bp_right.clone());
        if !bp_right.is_natural_hermitian() {
            right_to_mul.push(bp_right.hermitian_conjugate().0);
        }

        for left in left_to_mul {
            for right in right_to_mul.clone() {
                output_vec.append(&mut (left.clone() * right));
            }
        }
        output_vec
    }
}

/// Implements the multiplication function of a vector of BosonProducts by a HermitianBosonProduct.
///
impl Mul<HermitianBosonProduct> for Vec<BosonProduct> {
    type Output = Vec<BosonProduct>;

    /// Implement `*` for a vector of BosonProducts and a HermitianBosonProduct.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The HermitianBosonProduct to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - The result of the multiplication.
    ///
    /// # Panics
    ///
    /// * Could not convert rhs into a BosonProduct.
    /// * Unexpectedly failed construction of BosonProduct creation, internal struqture bug.
    fn mul(self, rhs: HermitianBosonProduct) -> Self::Output {
        let mut output_vec: Vec<BosonProduct> = Vec::new();
        for lh_bp in self {
            output_vec.append(&mut (lh_bp * rhs.clone()))
        }
        output_vec
    }
}

/// Implements the multiplication function of a HermitianBosonProduct by a reference BosonProduct.
///
impl Mul<&BosonProduct> for HermitianBosonProduct {
    type Output = Vec<BosonProduct>;
    /// Implement `*` for a HermitianBosonProduct and a reference BosonProduct.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The reference BosonProduct to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - The result of the multiplication.
    ///
    /// # Panics
    ///
    /// * Could not convert self into a BosonProduct.
    /// * Unexpectedly failed construction of BosonProduct creation, internal struqture bug.
    fn mul(self, rhs: &BosonProduct) -> Self::Output {
        let mut output_vec: Vec<BosonProduct> = Vec::new();

        let mut left_to_mul: Vec<BosonProduct> = Vec::new();
        let hbp_to_bp = BosonProduct::new(self.creators, self.annihilators)
            .expect("Could not convert self into a BosonProduct");
        left_to_mul.push(hbp_to_bp.clone());
        if !hbp_to_bp.is_natural_hermitian() {
            left_to_mul.push(hbp_to_bp.hermitian_conjugate().0);
        }

        for left in left_to_mul {
            output_vec.append(&mut (left.clone() * rhs.clone()));
        }
        output_vec
    }
}

/// Implements the multiplication function of a HermitianBosonProduct by a vector of BosonProducts.
///
impl Mul<Vec<BosonProduct>> for HermitianBosonProduct {
    type Output = Vec<BosonProduct>;

    /// Implement `*` for a HermitianBosonProduct and a vector of BosonProducts.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector of BosonProducts to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - The result of the multiplication.
    ///
    /// # Panics
    ///
    /// * Could not convert self into a BosonProduct.
    /// * Unexpectedly failed construction of BosonProduct creation, internal struqture bug.
    fn mul(self, rhs: Vec<BosonProduct>) -> Self::Output {
        let mut output_vec: Vec<BosonProduct> = Vec::new();
        for rh_bp in rhs.iter() {
            output_vec.append(&mut (self.clone() * rh_bp))
        }
        output_vec
    }
}

/// Trait for transforming value stored at index I when using index of different type T to read out value
///
impl GetValue<HermitianBosonProduct> for HermitianBosonProduct {
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
    /// * `Self` - The corresponding HermitianBosonProduct.
    fn get_key(index: &HermitianBosonProduct) -> Self {
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
        _index: &HermitianBosonProduct,
        value: qoqo_calculator::CalculatorComplex,
    ) -> qoqo_calculator::CalculatorComplex {
        value
    }
}

impl GetValue<BosonProduct> for HermitianBosonProduct {
    type ValueIn = CalculatorComplex;
    type ValueOut = CalculatorComplex;

    /// Gets the HermitianBosonProduct corresponding to the input BosonProduct.
    ///
    /// # Arguments
    ///
    /// * `index` - The BosonProduct of which to get the corresponding HermitianBosonProduct.
    ///
    /// # Returns
    ///
    /// * `Self` - The corresponding HermitianBosonProduct.
    fn get_key(index: &BosonProduct) -> Self {
        if index.creators().min() > index.annihilators().min() {
            Self {
                creators: index.annihilators.clone(),
                annihilators: index.creators.clone(),
            }
        } else {
            Self {
                creators: index.creators.clone(),
                annihilators: index.annihilators.clone(),
            }
        }
    }

    /// Gets the transformed value corresponding to the input BosonProduct and value.
    ///
    /// # Arguments
    ///
    /// * `index` - The BosonProduct to transform the value by.
    /// * `value` - The value to be transformed.
    ///
    /// # Returns
    ///
    /// * `CalculatorComplex` - The transformed value.
    fn get_transform(
        index: &BosonProduct,
        value: qoqo_calculator::CalculatorComplex,
    ) -> qoqo_calculator::CalculatorComplex {
        if index.creators().min() > index.annihilators().min() {
            value.conj()
        } else {
            value
        }
    }
}

impl GetValue<HermitianBosonProduct> for BosonProduct {
    type ValueIn = CalculatorComplex;
    type ValueOut = CalculatorComplex;

    /// Gets the BosonProduct corresponding to the input HermitianBosonProduct.
    ///
    /// # Arguments
    ///
    /// * `index` - The index for which to get the corresponding Product.
    ///
    /// # Returns
    ///
    /// * `Self` - The corresponding BosonProduct.
    fn get_key(index: &HermitianBosonProduct) -> Self {
        Self {
            creators: index.creators.clone(),
            annihilators: index.annihilators.clone(),
        }
    }

    /// Gets the transformed value corresponding to the input HermitianBosonProduct and value (here, itself).
    ///
    /// # Arguments
    ///
    /// * `index` - The HermitianBosonProduct to transform the value by.
    /// * `value` - The value to be transformed.
    ///
    /// # Returns
    ///
    /// * `CalculatorComplex` - The transformed value.
    fn get_transform(
        _index: &HermitianBosonProduct,
        value: qoqo_calculator::CalculatorComplex,
    ) -> qoqo_calculator::CalculatorComplex {
        value
    }
}

/// Implements the format function (Display trait) of HermitianBosonProduct.
///
impl std::fmt::Display for HermitianBosonProduct {
    /// Formats the HermitianBosonProduct using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted HermitianBosonProduct.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string: String = String::new();
        if self.creators.is_empty() & self.annihilators.is_empty() {
            string.push('I'); // Empty is just identity
        } else {
            for index in self.creators() {
                string.push_str(format!("c{}", index).as_str());
            }
            for index in self.annihilators() {
                string.push_str(format!("a{}", index).as_str());
            }
        }
        write!(f, "{}", string)
    }
}

impl FromStr for HermitianBosonProduct {
    type Err = StruqtureError;

    /// Constructs a HermitianBosonProduct from a string.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to convert.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The successfully converted HermitianBosonProduct.
    /// * `Err(StruqtureError::IndicesNotNormalOrdered)` - Indices are not normal ordered.
    /// * `Err(StruqtureError::FromStringFailed)` - Used operator that is neither 'c' nor 'a'.
    /// * `Err(StruqtureError::FromStringFailed)` - Index in given creators or annihilators is not an integer.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "I" {
            Self::new([], [])
        } else {
            let mut creators: TinyVec<[usize; 2]> = TinyVec::<[usize; 2]>::with_capacity(2);
            let mut annihilators: TinyVec<[usize; 2]> = TinyVec::<[usize; 2]>::with_capacity(2);

            let operators = s.split(char::is_numeric).filter(|s| !s.is_empty());
            let indices = s.split(char::is_alphabetic).filter(|s| !s.is_empty());
            let mut parsing_creators: bool = true;
            for (index, op) in indices.zip(operators) {
                match index.parse() {
                    Ok(num) => {
                        match op{
                            "c" => {if parsing_creators{ creators.push(num);} else{return Err(StruqtureError::IndicesNotNormalOrdered{index_i: num, index_j: num+1})}}
                            "a" => {annihilators.push(num); parsing_creators = false;}
                            _ => return Err(StruqtureError::FromStringFailed{msg: format!("Used operator {} that is neither 'c' nor 'a' in HermitianBosonProduct::from_str", op)})
                        }
                    }
                    Err(_) => return Err(StruqtureError::FromStringFailed{msg: format!("Index in given creators or annihilators is not an integer: {}", index)}),
                }
            }
            Self::new(creators, annihilators)
        }
    }
}

/// Assumes both annihilators_left and creators_right are sorted.
fn commute_creator_annihilator(
    annihilators_left: &[usize],
    creators_right: &[usize],
) -> Vec<CreatorsAnnihilators> {
    let mut result: Vec<CreatorsAnnihilators> = Vec::new();
    let mut found = false;

    for (cindex, creator) in creators_right.iter().enumerate() {
        for (aindex, _) in annihilators_left
            .iter()
            .enumerate()
            .skip_while(|(_, an)| *an != creator)
            .take_while(|(_, an)| *an == creator)
        {
            let recurse_creators: TinyVec<[usize; 2]> = creators_right
                .iter()
                .enumerate()
                .filter(|(index, _)| *index != cindex)
                .map(|(_, rc)| rc)
                .copied()
                .collect();
            let recurse_annihilators: TinyVec<[usize; 2]> = annihilators_left
                .iter()
                .enumerate()
                .filter(|(index, _)| *index != aindex)
                .map(|(_, rc)| rc)
                .copied()
                .collect();
            let recursed = commute_creator_annihilator(&recurse_annihilators, &recurse_creators);
            result.append(&mut recursed.clone());
            if !found {
                for (mut c, mut a) in recursed {
                    c.push(*creator);
                    a.push(*creator);
                    result.push((c, a))
                }
            }
            found = true
        }
        if found {
            break;
        }
    }
    if !found {
        result.push((
            creators_right.iter().copied().collect(),
            annihilators_left.iter().copied().collect(),
        ));
    }
    result
}

#[cfg(test)]
mod test {
    use crate::ModeTinyVec;

    use super::*;
    use test_case::test_case;
    use tinyvec::tiny_vec;

    #[test_case(tiny_vec!([usize; 2] => 0, 2, 4), tiny_vec!([usize; 2] => 1, 3, 5),
     vec![(tiny_vec!([usize; 2] => 1, 3, 5), tiny_vec!([usize; 2] => 0, 2, 4))]; "0,2,4 - 1,3,5")]
    #[test_case(tiny_vec!([usize; 2] => 0), tiny_vec!([usize; 2] => 0),
     vec![(tiny_vec!([usize; 2] => 0), tiny_vec!([usize; 2] => 0)), (tiny_vec!([usize; 2]), tiny_vec!([usize; 2]))]; "0, - 0")]
    #[test_case(tiny_vec!([usize; 2] => 20), tiny_vec!([usize; 2]),
     vec![(tiny_vec!([usize; 2]), tiny_vec!([usize; 2] => 20))]; "20 - empty")]
    #[test_case(tiny_vec!([usize; 2] => 1,20), tiny_vec!([usize; 2] => 1,30),
     vec![(tiny_vec!([usize; 2] => 30,1), tiny_vec!([usize; 2] => 20,1)), (tiny_vec!([usize; 2] => 30), tiny_vec!([usize; 2] => 20))]; "1,20 - 1,30")]
    #[test_case(tiny_vec!([usize; 2] => 1,2,20), tiny_vec!([usize; 2] => 1,2,30),
     vec![(tiny_vec!([usize; 2] => 30), tiny_vec!([usize; 2] => 20)), (tiny_vec!([usize; 2] => 30,2), tiny_vec!([usize; 2] => 20,2)),
          (tiny_vec!([usize; 2] => 30,1), tiny_vec!([usize; 2] => 20,1)), (tiny_vec!([usize; 2] => 30,2,1), tiny_vec!([usize; 2] => 20,2,1))]; "1,2,20 - 1,2,30")]
    #[test_case(tiny_vec!([usize; 2] => 10,20,30), tiny_vec!([usize; 2] => 10,30),
    vec![(tiny_vec!([usize; 2]), tiny_vec!([usize; 2] => 20)), (tiny_vec!([usize; 2] => 30), tiny_vec!([usize; 2] => 20,30)),
        (tiny_vec!([usize; 2] => 10), tiny_vec!([usize; 2] => 20,10)), (tiny_vec!([usize; 2] => 30,10), tiny_vec!([usize; 2] => 20,30,10))]; "10,20,30 - 10,30")]
    #[test_case(tiny_vec!([usize; 2] => 10,20,30), tiny_vec!([usize; 2] => 10,30,40),
    vec![(tiny_vec!([usize; 2] => 40), tiny_vec!([usize; 2] => 20)), (tiny_vec!([usize; 2] => 40,30), tiny_vec!([usize; 2] => 20,30)),
        (tiny_vec!([usize; 2] => 40,10), tiny_vec!([usize; 2] => 20,10)), (tiny_vec!([usize; 2] => 40,30,10), tiny_vec!([usize; 2] => 20,30,10))]; "10,20,30 - 10,30,40")]
    fn commute(
        annihilators_left: TinyVec<[usize; 2]>,
        creators_right: TinyVec<[usize; 2]>,
        expected: Vec<(ModeTinyVec, ModeTinyVec)>,
    ) {
        let result = commute_creator_annihilator(&annihilators_left, &creators_right);
        assert_eq!(result.len(), expected.len());
        for pair in expected {
            assert!(result.contains(&pair));
        }
    }
}
