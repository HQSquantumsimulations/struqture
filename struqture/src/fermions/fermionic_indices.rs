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

use super::FermionIndex;
use crate::mappings::JordanWignerFermionToSpin;
use crate::prelude::*;
use crate::spins::{PauliProduct, SingleSpinOperator, SpinHamiltonian, SpinOperator};
use crate::{
    CorrespondsTo, CreatorsAnnihilators, GetValue, ModeIndex, StruqtureError, SymmetricIndex,
};

use qoqo_calculator::*;
use serde::{
    de::{Error, SeqAccess, Visitor},
    ser::SerializeTuple,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::cmp::Ordering;
use std::{ops::Mul, str::FromStr};
use tinyvec::TinyVec;

/// A product of fermionic creation and annihilation operators
///
/// The FermionProduct is used as an index for non-hermitian, normal ordered fermionic operators.
/// A fermionic operator can be written as a sum over normal ordered products of creation and annihilation operators.
/// The FermionProduct is used as an index when setting or adding new summands to a fermionic operator and when querrying the
/// weight of a product of operators in the sum.
///
/// # Example
///
/// ```rust
/// use struqture::prelude::*;
/// use struqture::fermions::FermionProduct;
///
/// let b_product = FermionProduct::new([0, 1], [0, 1]).unwrap();
/// println!("{}", b_product);
///
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct FermionProduct {
    /// The ordered list of creator indices.
    creators: TinyVec<[usize; 2]>,
    /// The ordered list of annihilator indices.
    annihilators: TinyVec<[usize; 2]>,
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for FermionProduct {
    fn schema_name() -> String {
        "FermionProduct".to_string()
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let tmp_schema = gen.subschema_for::<String>();
        let mut obj = tmp_schema.into_object();
        let meta = obj.metadata();
        meta.description = Some("Represents products of Fermionic creators and annhilators by a string creators (c) or annihilators (a) followed by the modes they are acting on. E.g. c0a1.".to_string());

        schemars::schema::Schema::Object(obj)
    }
}

impl crate::MinSupportedVersion for FermionProduct {}

/// Implementing serde serialization writing directly to string.
///
impl Serialize for FermionProduct {
    /// Serialization function for FermionProduct according to string type.
    ///
    /// # Arguments
    ///
    /// * `self` - FermionProduct to be serialized.
    /// * `serializer` - Serializer used for serialization.
    ///
    /// # Returns
    ///
    /// `S::Ok` - Serialized instance of FermionProduct.
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
impl<'de> Deserialize<'de> for FermionProduct {
    /// Deserialization function for FermionProduct.
    ///
    /// # Arguments
    ///
    /// * `self` - Serialized instance of FermionProduct to be deserialized.
    /// * `deserializer` - Deserializer used for deserialization.
    ///
    /// # Returns
    ///
    /// `DecoherenceProduct` - Deserialized instance of FermionProduct.
    /// `D::Error` - Error in the deserialization process.
    fn deserialize<D>(deserializer: D) -> Result<FermionProduct, D::Error>
    where
        D: Deserializer<'de>,
    {
        let human_readable = deserializer.is_human_readable();
        if human_readable {
            struct TemporaryVisitor;
            impl<'de> Visitor<'de> for TemporaryVisitor {
                type Value = FermionProduct;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("String")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    FermionProduct::from_str(v).map_err(|err| E::custom(format!("{:?}", err)))
                }

                fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    FermionProduct::from_str(v).map_err(|err| E::custom(format!("{:?}", err)))
                }
            }

            deserializer.deserialize_str(TemporaryVisitor)
        } else {
            struct FermionProductVisitor;
            impl<'de> serde::de::Visitor<'de> for FermionProductVisitor {
                type Value = FermionProduct;
                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    std::fmt::Formatter::write_str(
                        formatter,
                        "Tuple of two sequences of unsigned integers",
                    )
                }
                // when variants are marked by String values
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

                    FermionProduct::new(creators, annihilators).map_err(M::Error::custom)
                }
            }
            let pp_visitor = FermionProductVisitor;

            deserializer.deserialize_tuple(2, pp_visitor)
        }
    }
}

impl ModeIndex for FermionProduct {
    /// Creates a new FermionProduct.
    ///
    /// # Arguments
    ///
    /// * `creators` - The creator indices to have in the FermionProduct.
    /// * `annihilators` - The annihilators indices to have in the FermionProduct.
    ///
    /// # Returns
    ///
    /// * `Ok(FermionProduct)` - The new FermionProduct with the given creators and annihilators.
    /// * `Err(StruqtureError::IncorrectlyOrderedIndices)` - Indices given in creators/annihilators are either not normal ordered, or contain a double index specification.
    fn new(
        creators: impl IntoIterator<Item = usize>,
        annihilators: impl IntoIterator<Item = usize>,
    ) -> Result<Self, StruqtureError> {
        let creators: TinyVec<[usize; 2]> = creators.into_iter().collect();
        match creators.windows(2).all(|w| w[0] < w[1]) {
            true => {}
            false => return Err(StruqtureError::IncorrectlyOrderedIndices),
        }

        let annihilators: TinyVec<[usize; 2]> = annihilators.into_iter().collect();
        match annihilators.windows(2).all(|w| w[0] < w[1]) {
            true => {}
            false => return Err(StruqtureError::IncorrectlyOrderedIndices),
        }

        Ok(Self {
            creators,
            annihilators,
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

    /// Creates a pair (FermionProduct, CalculatorComplex).
    ///
    /// The first item is the valid FermionProduct created from the input creators and annihilators.
    /// The second term is the input CalculatorComplex transformed according to the valid order of creators and annihilators.
    ///
    /// # Arguments
    ///
    /// * `creators` - The creator indices to have in the FermionProduct.
    /// * `annihilators` - The annihilators indices to have in the FermionProduct.
    /// * `value` - The CalculatorComplex to transform.
    ///
    /// # Returns
    ///
    /// * `Ok((FermionProduct, CalculatorComplex))` - The valid FermionProduct and the corresponding transformed CalculatorComplex.
    /// * `Err(StruqtureError::IndicesContainDoubles)` - Indices given in either creators or annihilators contain a double index specification.
    fn create_valid_pair(
        creators: impl IntoIterator<Item = usize>,
        annihilators: impl IntoIterator<Item = usize>,
        value: qoqo_calculator::CalculatorComplex,
    ) -> Result<(Self, qoqo_calculator::CalculatorComplex), StruqtureError> {
        let creators: TinyVec<[usize; 2]> = creators.into_iter().collect();
        let (new_creators, contains_double, parity_c) = sort_and_signal(creators);
        if contains_double {
            return Err(StruqtureError::IndicesContainDoubles {});
        }

        let annihilators: TinyVec<[usize; 2]> = annihilators.into_iter().collect();
        let (new_annihilators, contains_double, parity_a) = sort_and_signal(annihilators);
        if contains_double {
            return Err(StruqtureError::IndicesContainDoubles {});
        }

        let value = if (parity_c + parity_a) % 2 != 0 {
            value * -1.0
        } else {
            value
        };
        Ok((
            Self {
                creators: new_creators,
                annihilators: new_annihilators,
            },
            value,
        ))
    }
}

impl FermionIndex for FermionProduct {}

impl CorrespondsTo<FermionProduct> for FermionProduct {
    /// Gets the FermionProduct corresponding to self (here, itself).
    ///
    /// # Returns
    ///
    /// * `FermionProduct` - The FermionProduct corresponding to Self.
    fn corresponds_to(&self) -> FermionProduct {
        self.clone()
    }
}

impl CorrespondsTo<HermitianFermionProduct> for FermionProduct {
    /// Gets the HermitianFermionProduct corresponding Self.
    ///
    /// # Returns
    ///
    /// * `HermitianFermionProduct` - The HermitianFermionProduct corresponding to Self.
    fn corresponds_to(&self) -> HermitianFermionProduct {
        if self.creators().min() > self.annihilators().min() {
            HermitianFermionProduct {
                creators: self.annihilators.clone(),
                annihilators: self.creators.clone(),
            }
        } else {
            HermitianFermionProduct {
                creators: self.creators.clone(),
                annihilators: self.annihilators.clone(),
            }
        }
    }
}

impl SymmetricIndex for FermionProduct {
    // From trait
    fn hermitian_conjugate(&self) -> (Self, f64) {
        let mut creators = self.annihilators.clone();
        creators.reverse();
        let mut annihilators = self.creators.clone();
        annihilators.reverse();
        let (new, value) = FermionProduct::create_valid_pair(creators, annihilators, 1.0.into())
            .expect("Bug: somehow commuted through and got a complex value");

        (
            new,
            *value
                .re
                .float()
                .expect("Bug: somehow commuted through and got a complex value"),
        )
    }

    // From trait
    fn is_natural_hermitian(&self) -> bool {
        self.creators == self.annihilators
    }
}

/// Implements the multiplication function of FermionProduct by FermionProduct.
///
impl Mul<FermionProduct> for FermionProduct {
    type Output = Vec<(FermionProduct, f64)>;
    /// Implement `*` for FermionProduct and FermionProduct.
    ///
    /// # Arguments
    ///
    /// * `other` - The FermionProduct to multiply by.
    ///
    /// # Returns
    ///
    /// * `Vec<(FermionProduct, f64)>` - The two FermionProducts multiplied.
    ///
    /// # Panics
    ///
    /// * Unexpectedly failed construction of FermionProduct creation internal struqture bug.
    /// * Bug: somehow commuted through and got a complex value.
    /// * Internal bug in `create_valid_pair`.
    fn mul(self, rhs: FermionProduct) -> Self::Output {
        let mut output_vec: Vec<(FermionProduct, f64)> = Vec::new();

        let commuted_creators_annihilators =
            commute_creator_annihilator_fermionic(&self.annihilators, &rhs.creators);
        for ((new_creators, mut new_annihilators), prefac) in commuted_creators_annihilators {
            let mut tmp_creators = self.creators.clone();
            tmp_creators.extend(new_creators.into_iter());
            new_annihilators.extend(rhs.annihilators().copied());
            match FermionProduct::create_valid_pair(tmp_creators, new_annihilators, prefac.into()) {
                Ok((tmp_fermion_product, sign)) => {
                    output_vec.push((
                        tmp_fermion_product,
                        *sign
                            .re
                            .float()
                            .expect("Bug: somehow commuted through and got a complex value"),
                    ));
                }
                Err(StruqtureError::IndicesContainDoubles) => continue,
                _ => panic!("Internal bug in `create_valid_pair`"),
            }
        }
        output_vec
    }
}

impl Mul<Vec<FermionProduct>> for FermionProduct {
    type Output = Vec<(FermionProduct, f64)>;
    /// Implement `*` for FermionProduct and a vector of FermionProducts.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector of FermionProducts to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - The result of the multiplication.
    ///
    /// # Panics
    ///
    /// * Unexpectedly failed construction of FermionProduct creation internal struqture bug.
    /// * Bug: somehow commuted through and got a complex value.
    /// * Internal bug in `create_valid_pair`.
    fn mul(self, rhs: Vec<FermionProduct>) -> Self::Output {
        let mut output_vec: Vec<(FermionProduct, f64)> = Vec::new();
        for rh_bp in rhs.iter() {
            output_vec.append(&mut (self.clone() * rh_bp.clone()))
        }
        output_vec
    }
}

impl Mul<FermionProduct> for Vec<FermionProduct> {
    type Output = Vec<(FermionProduct, f64)>;
    /// Implement `*` for a vector of FermionProducts and a FermionProduct.
    ///
    /// # Arguments
    ///
    /// * `other` - The FermionProduct to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - The result of the multiplication.
    ///
    /// # Panics
    ///
    /// * Unexpectedly failed construction of FermionProduct creation internal struqture bug.
    /// * Bug: somehow commuted through and got a complex value.
    /// * Internal bug in `create_valid_pair`.
    fn mul(self, rhs: FermionProduct) -> Self::Output {
        let mut output_vec: Vec<(FermionProduct, f64)> = Vec::new();
        for lh_bp in self {
            output_vec.append(&mut (lh_bp * rhs.clone()))
        }
        output_vec
    }
}

impl Mul<HermitianFermionProduct> for FermionProduct {
    type Output = Vec<(FermionProduct, f64)>;
    /// Implement `*` for a FermionProduct and a HermitianFermionProduct.
    ///
    /// # Arguments
    ///
    /// * `other` - The HermitianFermionProduct to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - The result of the multiplication.
    ///
    /// # Panics
    ///
    /// * Could not convert rhs to FermionProduct.
    /// * Unexpectedly failed construction of FermionProduct creation internal struqture bug.
    /// * Bug: somehow commuted through and got a complex value.
    /// * Internal bug in `create_valid_pair`.
    fn mul(self, rhs: HermitianFermionProduct) -> Self::Output {
        let mut output_vec: Vec<(FermionProduct, f64)> = Vec::new();

        let mut right_to_mul: Vec<(FermionProduct, f64)> = Vec::new();
        let hfp_to_fp = FermionProduct::new(rhs.creators, rhs.annihilators)
            .expect("Could not convert rhs into a FermionProduct");
        right_to_mul.push((hfp_to_fp.clone(), 1.0));
        if !hfp_to_fp.is_natural_hermitian() {
            right_to_mul.push(hfp_to_fp.hermitian_conjugate());
        }
        for (right, rsign) in right_to_mul {
            let res_vec: Vec<(FermionProduct, f64)> = self.clone() * right;
            for (fp, val) in res_vec.iter() {
                output_vec.push((fp.clone(), val * rsign * 1.0));
            }
        }
        output_vec
    }
}

impl GetValue<FermionProduct> for FermionProduct {
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
    /// * `Self` - The corresponding FermionProduct.
    fn get_key(index: &FermionProduct) -> Self {
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
        _index: &FermionProduct,
        value: qoqo_calculator::CalculatorComplex,
    ) -> qoqo_calculator::CalculatorComplex {
        value
    }
}

/// Implements the format function (Display trait) of FermionProduct.
///
impl std::fmt::Display for FermionProduct {
    /// Formats the FermionProduct using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted FermionProduct.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string: String = String::new();
        if self.creators.is_empty() & self.annihilators.is_empty() {
            string.push('I'); // empty is just identity
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

impl FromStr for FermionProduct {
    type Err = StruqtureError;
    /// Constructs a FermionProduct from a string.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to convert.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The successfully converted FermionProduct.
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
                            _ => return Err(StruqtureError::FromStringFailed{msg: format!("Used operator {} that is neither 'c' nor 'a' in FermionProduct::from_str", op)})
                        }
                    }
                    Err(_) => return Err(StruqtureError::FromStringFailed{msg: format!("Index of Fermion operator {} is not a FermionProduct::from_str", index)}),
                }
            }
            Self::new(creators, annihilators)
        }
    }
}

/// A hermitian product of fermionic creation and annihilation operators
///
/// The HermitianFermionProduct is used as an index for hermitian, normal ordered fermionic operators.
/// A fermionic operator can be written as a sum over normal ordered products of creation and annihilation operators.
/// The FermionProduct is used as an index when setting or adding new summands to a fermionic operator and when querying the
/// weight of a product of operators in the sum.
///
/// # Example
///
/// ```rust
/// use struqture::prelude::*;
/// use struqture::fermions::HermitianFermionProduct;
///
/// let f_product = HermitianFermionProduct::new([0, 1], [0, 1]).unwrap();
/// println!("{}", f_product);
///
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct HermitianFermionProduct {
    /// The ordered list of creator indices.
    creators: TinyVec<[usize; 2]>,
    /// The ordered list of annihilator indices.
    annihilators: TinyVec<[usize; 2]>,
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for HermitianFermionProduct {
    fn schema_name() -> String {
        "HermitianFermionProduct".to_string()
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let tmp_schema = gen.subschema_for::<String>();
        let mut obj = tmp_schema.into_object();
        let meta = obj.metadata();
        meta.description = Some("Represents products of Fermionic creators and annhilators by a string creators (c) or annihilators (a) followed by the modes they are acting on. E.g. c0a1.".to_string());

        schemars::schema::Schema::Object(obj)
    }
}

impl crate::MinSupportedVersion for HermitianFermionProduct {}

/// Implementing serde serialization writing directly to string.
///
impl Serialize for HermitianFermionProduct {
    /// Serialization function for FermionProduct according to string type.
    ///
    /// # Arguments
    ///
    /// * `self` - FermionProduct to be serialized.
    /// * `serializer` - Serializer used for serialization.
    ///
    /// # Returns
    ///
    /// `S::Ok` - Serialized instance of FermionProduct.
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
impl<'de> Deserialize<'de> for HermitianFermionProduct {
    /// Deserialization function for FermionProduct.
    ///
    /// # Arguments
    ///
    /// * `self` - Serialized instance of FermionProduct to be deserialized.
    /// * `deserializer` - Deserializer used for deserialization.
    ///
    /// # Returns
    ///
    /// `DecoherenceProduct` - Deserialized instance of FermionProduct.
    /// `D::Error` - Error in the deserialization process.
    fn deserialize<D>(deserializer: D) -> Result<HermitianFermionProduct, D::Error>
    where
        D: Deserializer<'de>,
    {
        let human_readable = deserializer.is_human_readable();
        if human_readable {
            struct TemporaryVisitor;
            impl<'de> Visitor<'de> for TemporaryVisitor {
                type Value = HermitianFermionProduct;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("String")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    HermitianFermionProduct::from_str(v)
                        .map_err(|err| E::custom(format!("{:?}", err)))
                }

                fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    HermitianFermionProduct::from_str(v)
                        .map_err(|err| E::custom(format!("{:?}", err)))
                }
            }

            deserializer.deserialize_str(TemporaryVisitor)
        } else {
            struct FermionProductVisitor;
            impl<'de> serde::de::Visitor<'de> for FermionProductVisitor {
                type Value = HermitianFermionProduct;
                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    std::fmt::Formatter::write_str(
                        formatter,
                        "Tuple of two sequences of unsigned integers",
                    )
                }
                // when variants are marked by String values
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

                    HermitianFermionProduct::new(creators, annihilators).map_err(M::Error::custom)
                }
            }
            let pp_visitor = FermionProductVisitor;

            deserializer.deserialize_tuple(2, pp_visitor)
        }
    }
}

impl ModeIndex for HermitianFermionProduct {
    /// Creates a new HermitianFermionProduct.
    ///
    /// # Arguments
    ///
    /// * `creators` - The creator indices to have in the HermitianFermionProduct.
    /// * `annihilators` - The annihilators indices to have in the HermitianFermionProduct.
    ///
    /// # Returns
    ///
    /// * `Ok(HermitianFermionProduct)` - The new HermitianFermionProduct with the given creators and annihilators.
    /// * `Err(StruqtureError::IncorrectlyOrderedIndices)` - Indices given in creators/annihilators are either not normal ordered, or contain a double index specification.
    /// * `Err(StruqtureError::CreatorsAnnihilatorsMinimumIndex)` - The minimum index of the creators is larger than the minimum index of the annihilators.
    fn new(
        creators: impl IntoIterator<Item = usize>,
        annihilators: impl IntoIterator<Item = usize>,
    ) -> Result<Self, StruqtureError> {
        let creators: TinyVec<[usize; 2]> = creators.into_iter().collect();
        match creators.windows(2).all(|w| w[0] < w[1]) {
            true => {}
            false => return Err(StruqtureError::IncorrectlyOrderedIndices),
        }

        let annihilators: TinyVec<[usize; 2]> = annihilators.into_iter().collect();
        match annihilators.windows(2).all(|w| w[0] < w[1]) {
            true => {}
            false => return Err(StruqtureError::IncorrectlyOrderedIndices),
        }

        let mut number_equal_indices = 0;

        for (creator, annihilator) in creators.iter().zip(annihilators.iter()) {
            match annihilator.cmp(creator) {
                std::cmp::Ordering::Less => {
                    return Err(StruqtureError::CreatorsAnnihilatorsMinimumIndex {
                        creators_min: Some(*creator),
                        annihilators_min: Some(*annihilator),
                    });
                }
                std::cmp::Ordering::Greater => break,
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
            creators,
            annihilators,
        })
    }

    /// Gets the creator indices of the HermitianFermionProduct.
    ///
    /// # Returns
    ///
    /// * `usize` - The creator indices in the HermitianFermionProduct.
    fn creators(&self) -> std::slice::Iter<usize> {
        self.creators.iter()
    }

    /// Gets the annihilator indices of the HermitianFermionProduct.
    ///
    /// # Returns
    ///
    /// * `usize` - The annihilator indices in the HermitianFermionProduct.
    fn annihilators(&self) -> std::slice::Iter<usize> {
        self.annihilators.iter()
    }

    /// Creates a pair (HermitianFermionProduct, CalculatorComplex).
    ///
    /// The first item is the valid HermitianFermionProduct created from the input creators and annihilators.
    /// The second term is the input CalculatorComplex transformed according to the valid order of creators and annihilators.
    ///
    /// # Arguments
    ///
    /// * `creators` - The creator indices to have in the HermitianFermionProduct.
    /// * `annihilators` - The annihilators indices to have in the HermitianFermionProduct.
    /// * `value` - The CalculatorComplex to transform.
    ///
    /// # Returns
    ///
    /// * `Ok((HermitianFermionProduct, CalculatorComplex))` - The valid HermitianFermionProduct and the corresponding transformed CalculatorComplex.
    /// * `Err(StruqtureError::IndicesContainDoubles)` - Indices given in either creators or annihilators contain a double index specification.
    fn create_valid_pair(
        creators: impl IntoIterator<Item = usize>,
        annihilators: impl IntoIterator<Item = usize>,
        value: qoqo_calculator::CalculatorComplex,
    ) -> Result<(Self, qoqo_calculator::CalculatorComplex), StruqtureError> {
        let creators: TinyVec<[usize; 2]> = creators.into_iter().collect();
        let (new_creators, contains_double, parity_c) = sort_and_signal(creators);
        if contains_double {
            return Err(StruqtureError::IndicesContainDoubles {});
        }

        let annihilators: TinyVec<[usize; 2]> = annihilators.into_iter().collect();
        let (new_annihilators, contains_double, parity_a) = sort_and_signal(annihilators);
        if contains_double {
            return Err(StruqtureError::IndicesContainDoubles {});
        }

        let value = if (parity_c + parity_a) % 2 != 0 {
            value * -1.0
        } else {
            value
        };
        let mut hermitian_conjugate = false;
        let mut number_equal_indices = 0;
        for (creator, annihilator) in new_creators.iter().zip(new_annihilators.iter()) {
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
        if new_creators.len() > number_equal_indices
            && new_annihilators.len() == number_equal_indices
        {
            hermitian_conjugate = true;
        }
        if hermitian_conjugate {
            Ok((
                Self {
                    creators: new_annihilators,
                    annihilators: new_creators,
                },
                value.conj(),
            ))
        } else {
            Ok((
                Self {
                    creators: new_creators,
                    annihilators: new_annihilators,
                },
                value,
            ))
        }
    }
}

impl FermionIndex for HermitianFermionProduct {}

impl CorrespondsTo<HermitianFermionProduct> for HermitianFermionProduct {
    /// Gets the HermitianFermionProduct corresponding to self (here, itself).
    ///
    /// # Returns
    ///
    /// * `HermitianFermionProduct` - The HermitianFermionProduct corresponding to Self.
    fn corresponds_to(&self) -> HermitianFermionProduct {
        self.clone()
    }
}

impl CorrespondsTo<FermionProduct> for HermitianFermionProduct {
    /// Gets the FermionProduct corresponding to Self.
    ///
    /// # Returns
    ///
    /// * `FermionProduct` - The FermionProduct corresponding to Self.
    fn corresponds_to(&self) -> FermionProduct {
        FermionProduct {
            creators: self.creators.clone(),
            annihilators: self.annihilators.clone(),
        }
    }
}

impl SymmetricIndex for HermitianFermionProduct {
    // From trait
    fn hermitian_conjugate(&self) -> (Self, f64) {
        (self.clone(), 1.0)
    }

    // From trait
    fn is_natural_hermitian(&self) -> bool {
        self.creators == self.annihilators
    }
}

/// Implements the multiplication function of HermitianFermionProduct by HermitianFermionProduct.
///
impl Mul<HermitianFermionProduct> for HermitianFermionProduct {
    type Output = Vec<(FermionProduct, f64)>;

    /// Implement `*` for HermitianFermionProduct and HermitianFermionProduct.
    ///
    /// # Arguments
    ///
    /// * `other` - The HermitianFermionProduct to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - The two HermitianFermionProducts multiplied.
    ///
    /// # Panics
    ///
    /// * Could not convert self to FermionProduct.
    /// * Could not convert rhs to FermionProduct.
    /// * Unexpectedly failed construction of HermitianFermionProduct creation internal struqture bug.
    /// * Bug: somehow commuted through and got a complex value.
    /// * Internal bug in `create_valid_pair`.
    fn mul(self, rhs: HermitianFermionProduct) -> Self::Output {
        let mut output_vec: Vec<(FermionProduct, f64)> = Vec::new();

        let mut left_to_mul: Vec<(FermionProduct, f64)> = Vec::new();
        let fp_left = FermionProduct::new(self.creators, self.annihilators)
            .expect("Could not convert self into a FermionProduct");
        left_to_mul.push((fp_left.clone(), 1.0));
        if !fp_left.is_natural_hermitian() {
            left_to_mul.push(fp_left.hermitian_conjugate());
        }

        let mut right_to_mul: Vec<(FermionProduct, f64)> = Vec::new();
        let fp_right = FermionProduct::new(rhs.creators, rhs.annihilators)
            .expect("Could not convert rhs into a FermionProduct");
        right_to_mul.push((fp_right.clone(), 1.0));
        if !fp_right.is_natural_hermitian() {
            right_to_mul.push(fp_right.hermitian_conjugate());
        }

        for (left, lsign) in left_to_mul {
            for (right, rsign) in right_to_mul.clone() {
                let res_vec: Vec<(FermionProduct, f64)> = left.clone() * right;
                for (fp, val) in res_vec.iter() {
                    output_vec.push((fp.clone(), val * rsign * lsign));
                }
            }
        }
        output_vec
    }
}

/// Implements the multiplication function of a vector of FermionProducts by a HermitianFermionProduct.
///
impl Mul<HermitianFermionProduct> for Vec<FermionProduct> {
    type Output = Vec<(FermionProduct, f64)>;

    /// Implement `*` for a vector of FermionProducts and a HermitianFermionProduct.
    ///
    /// # Arguments
    ///
    /// * `other` - The HermitianFermionProduct to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - The result of the multiplication.
    ///
    /// # Panics
    ///
    /// * Could not convert rhs to FermionProduct.
    /// * Unexpectedly failed construction of HermitianFermionProduct creation internal struqture bug.
    /// * Bug: somehow commuted through and got a complex value.
    /// * Internal bug in `create_valid_pair`.
    fn mul(self, rhs: HermitianFermionProduct) -> Self::Output {
        let mut output_vec: Vec<(FermionProduct, f64)> = Vec::new();
        for lh_bp in self {
            output_vec.append(&mut (lh_bp * rhs.clone()))
        }
        output_vec
    }
}

/// Implements the multiplication function of a HermitianFermionProduct by a reference FermionProduct.
///
impl Mul<&FermionProduct> for HermitianFermionProduct {
    type Output = Vec<(FermionProduct, f64)>;

    /// Implement `*` for a HermitianFermionProduct and a reference FermionProduct.
    ///
    /// # Arguments
    ///
    /// * `other` - The reference FermionProduct to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - The result of the multiplication.
    ///
    /// # Panics
    ///
    /// * Could not convert self to FermionProduct.
    /// * Unexpectedly failed construction of HermitianFermionProduct creation internal struqture bug.
    /// * Bug: somehow commuted through and got a complex value.
    /// * Internal bug in `create_valid_pair`.
    fn mul(self, rhs: &FermionProduct) -> Self::Output {
        let mut output_vec: Vec<(FermionProduct, f64)> = Vec::new();

        let mut left_to_mul: Vec<(FermionProduct, f64)> = Vec::new();
        let hfp_to_fp = FermionProduct::new(self.creators, self.annihilators)
            .expect("Could not convert self into a FermionProduct");
        left_to_mul.push((hfp_to_fp.clone(), 1.0));
        if !hfp_to_fp.is_natural_hermitian() {
            left_to_mul.push(hfp_to_fp.hermitian_conjugate());
        }
        for (left, lsign) in left_to_mul {
            let res_vec: Vec<(FermionProduct, f64)> = left.clone() * rhs.clone();
            for (fp, val) in res_vec.iter() {
                output_vec.push((fp.clone(), val * 1.0 * lsign));
            }
        }
        output_vec
    }
}

/// Implements the multiplication function of a HermitianFermionProduct by a vector of FermionProducts.
///
impl Mul<Vec<FermionProduct>> for HermitianFermionProduct {
    type Output = Vec<(FermionProduct, f64)>;

    /// Implement `*` for a HermitianFermionProduct and a vector of FermionProducts.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector of FermionProducts to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - The result of the multiplication.
    ///
    /// # Panics
    ///
    /// * Could not convert self to FermionProduct.
    /// * Unexpectedly failed construction of HermitianFermionProduct creation internal struqture bug.
    /// * Bug: somehow commuted through and got a complex value.
    /// * Internal bug in `create_valid_pair`.
    fn mul(self, rhs: Vec<FermionProduct>) -> Self::Output {
        let mut output_vec: Vec<(FermionProduct, f64)> = Vec::new();
        for rh_bp in rhs.iter() {
            output_vec.append(&mut (self.clone() * rh_bp))
        }
        output_vec
    }
}

/// Trait for transforming value stored at index I when using index of different type T to read out value
///
impl GetValue<HermitianFermionProduct> for HermitianFermionProduct {
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
    /// * `Self` - The corresponding HermitianFermionProduct.
    fn get_key(index: &HermitianFermionProduct) -> Self {
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
        _index: &HermitianFermionProduct,
        value: qoqo_calculator::CalculatorComplex,
    ) -> qoqo_calculator::CalculatorComplex {
        value
    }
}

impl GetValue<FermionProduct> for HermitianFermionProduct {
    type ValueIn = CalculatorComplex;
    type ValueOut = CalculatorComplex;

    /// Gets the HermitianFermionProduct corresponding to the input FermionProduct.
    ///
    /// # Arguments
    ///
    /// * `index` - The FermionProduct of which to get the corresponding HermitianFermionProduct.
    ///
    /// # Returns
    ///
    /// * `Self` - The corresponding HermitianFermionProduct.
    fn get_key(index: &FermionProduct) -> Self {
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

    /// Gets the transformed value corresponding to the input FermionProduct and value.
    ///
    /// # Arguments
    ///
    /// * `index` - The FermionProduct to transform the value by.
    /// * `value` - The value to be transformed.
    ///
    /// # Returns
    ///
    /// * `CalculatorComplex` - The transformed value.
    fn get_transform(
        index: &FermionProduct,
        value: qoqo_calculator::CalculatorComplex,
    ) -> qoqo_calculator::CalculatorComplex {
        if index.creators().min() > index.annihilators().min() {
            value.conj()
        } else {
            value
        }
    }
}

impl GetValue<HermitianFermionProduct> for FermionProduct {
    type ValueIn = CalculatorComplex;
    type ValueOut = CalculatorComplex;

    /// Gets the FermionProduct corresponding to the input HermitianFermionProduct.
    ///
    /// # Arguments
    ///
    /// * `index` - The index for which to get the corresponding Product.
    ///
    /// # Returns
    ///
    /// * `Self` - The corresponding FermionProduct.
    fn get_key(index: &HermitianFermionProduct) -> Self {
        Self {
            creators: index.creators.clone(),
            annihilators: index.annihilators.clone(),
        }
    }

    /// Gets the transformed value corresponding to the input HermitianFermionProduct and value (here, itself).
    ///
    /// # Arguments
    ///
    /// * `index` - The HermitianFermionProduct to transform the value by.
    /// * `value` - The value to be transformed.
    ///
    /// # Returns
    ///
    /// * `CalculatorComplex` - The transformed value.
    fn get_transform(
        _index: &HermitianFermionProduct,
        value: qoqo_calculator::CalculatorComplex,
    ) -> qoqo_calculator::CalculatorComplex {
        value
    }
}

/// Implements the format function (Display trait) of HermitianFermionProduct.
///
impl std::fmt::Display for HermitianFermionProduct {
    /// Formats the HermitianFermionProduct using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted HermitianFermionProduct.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string: String = String::new();
        if self.creators.is_empty() & self.annihilators.is_empty() {
            string.push('I'); // empty modes is just the identity
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

impl FromStr for HermitianFermionProduct {
    type Err = StruqtureError;

    /// Constructs a HermitianFermionProduct from a string.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to convert.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The successfully converted HermitianFermionProduct.
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
                            _ => return Err(StruqtureError::FromStringFailed{msg: format!("Used operator {} that is neither 'c' nor 'a' in HermitianFermionProduct::from_str", op)})
                        }
                    }
                    Err(_) => return Err(StruqtureError::FromStringFailed{msg: format!("Index of Fermion operator {} is not a HermitianFermionProduct::from_str", index)}),
                }
            }
            Self::new(creators, annihilators)
        }
    }
}

// Helper functions
/// Re-sorts indices for creators or annihilators for normal ordering and signals parity of the reordering and whether any term occurs twice
fn sort_and_signal(indices: TinyVec<[usize; 2]>) -> (TinyVec<[usize; 2]>, bool, usize) {
    let mut parity: usize = 0;
    let mut contain_double = false;
    let mut local_indices = indices;
    for outer_counter in 0..local_indices.len() {
        for inner_counter in (0..outer_counter).rev() {
            match local_indices[inner_counter].cmp(&local_indices[inner_counter + 1]) {
                Ordering::Greater => {
                    local_indices.swap(inner_counter, inner_counter + 1);
                    parity += 1;
                }
                Ordering::Equal => {
                    contain_double = true;
                    break;
                }
                Ordering::Less => {
                    break;
                }
            }
        }
    }
    (local_indices, contain_double, parity)
}

impl JordanWignerFermionToSpin for FermionProduct {
    type Output = SpinOperator;

    /// Implements JordanWignerFermionToSpin for a FermionProduct.
    ///
    /// The convention used is that |0> represents an empty fermionic state (spin-orbital),
    /// and |1> represents an occupied fermionic state.
    ///
    /// # Returns
    ///
    /// `SpinOperator` - The spin operator that results from the transformation.
    ///
    /// # Panics
    ///
    /// * Unexpectedly failed to add a PauliProduct to a SpinOperator internal struqture bug.
    /// * Internal bug in `add_operator_product`.
    fn jordan_wigner(&self) -> Self::Output {
        let number_creators = self.number_creators();
        let number_annihilators = self.number_annihilators();
        let mut spin_operator = SpinOperator::new();

        let mut id = PauliProduct::new();
        id = id.set_pauli(0, SingleSpinOperator::Identity);
        spin_operator
            .add_operator_product(id, CalculatorComplex::new(1.0, 0.0))
            .expect("Internal bug in add_operator_product.");

        // Jordan-Wigner strings are inserted every second lowering (raising) operator, in even or
        // odd positions depending on the parity of the total number of creation (annihilation)
        // operators.
        let mut previous = 0;
        for (index, site) in self.creators().enumerate() {
            if index % 2 != number_creators % 2 {
                for i in previous..*site {
                    spin_operator = spin_operator * PauliProduct::new().z(i)
                }
            }
            spin_operator = spin_operator * _lowering_operator(site);
            previous = *site;
        }

        previous = 0;
        for (index, site) in self.annihilators().enumerate() {
            if index % 2 != number_annihilators % 2 {
                for i in previous..*site {
                    spin_operator = spin_operator * PauliProduct::new().z(i)
                }
            }
            spin_operator = spin_operator * _raising_operator(site);
            previous = *site;
        }
        spin_operator
    }
}

impl JordanWignerFermionToSpin for HermitianFermionProduct {
    type Output = SpinHamiltonian;

    /// Implements JordanWignerFermionToSpin for a HermitianFermionProduct.
    ///
    /// The convention used is that |0> represents an empty fermionic state (spin-orbital),
    /// and |1> represents an occupied fermionic state.
    ///
    /// # Returns
    ///
    /// `SpinHamiltonian` - The spin operator that results from the transformation.
    ///
    /// # Panics
    ///
    /// * Internal bug in `add_operator_product`.
    fn jordan_wigner(&self) -> Self::Output {
        let number_creators = self.number_creators();
        let number_annihilators = self.number_annihilators();
        let mut spin_operator = SpinOperator::new();

        let mut id = PauliProduct::new();
        id = id.set_pauli(0, SingleSpinOperator::Identity);
        spin_operator
            .add_operator_product(id, CalculatorComplex::new(1.0, 0.0))
            .expect("Internal bug in add_operator_product.");

        // Jordan-Wigner strings are inserted every second lowering (raising) operator, in even or
        // odd positions depending on the parity of the total number of creation (annihilation)
        // operators.
        let mut previous = 0;
        for (index, site) in self.creators().enumerate() {
            if index % 2 != number_creators % 2 {
                for i in previous..*site {
                    spin_operator = spin_operator * PauliProduct::new().z(i)
                }
            }
            spin_operator = spin_operator * _lowering_operator(site);
            previous = *site;
        }

        previous = 0;
        for (index, site) in self.annihilators().enumerate() {
            if index % 2 != number_annihilators % 2 {
                for i in previous..*site {
                    spin_operator = spin_operator * PauliProduct::new().z(i)
                }
            }
            spin_operator = spin_operator * _raising_operator(site);
            previous = *site;
        }

        // Spin terms with imaginary coefficients are dropped, and
        // real coefficients are doubled.
        if !self.is_natural_hermitian() {
            let mut out = SpinHamiltonian::new();
            for (product, coeff) in spin_operator.iter() {
                if coeff.im == 0.0.into() {
                    out.add_operator_product(product.clone(), coeff.re.clone() * 2)
                        .expect("Internal bug in add_operator_product.");
                }
            }
            return out;
        }
        SpinHamiltonian::try_from(spin_operator).expect(
            "Error in conversion from SpinOperator to
SpinHamiltonian, despite the internal check that the HermitianFermionProduct in the jordan-wigner
transform is hermitian.",
        )
    }
}

fn _lowering_operator(i: &usize) -> SpinOperator {
    let mut out = SpinOperator::new();
    out.add_operator_product(PauliProduct::new().x(*i), CalculatorComplex::new(0.5, 0.0))
        .expect("Internal bug in add_operator_product.");
    out.add_operator_product(PauliProduct::new().y(*i), CalculatorComplex::new(0.0, -0.5))
        .expect("Internal bug in add_operator_product.");
    out
}
fn _raising_operator(i: &usize) -> SpinOperator {
    let mut out = SpinOperator::new();
    out.add_operator_product(PauliProduct::new().x(*i), CalculatorComplex::new(0.5, 0.0))
        .expect("Internal bug in add_operator_product.");
    out.add_operator_product(PauliProduct::new().y(*i), CalculatorComplex::new(0.0, 0.5))
        .expect("Internal bug in add_operator_product.");
    out
}

// When constructing multiplication with commute_creator remember to skip all products with double creators or double annihilators
/// Assumes both annihilators_left and creators_right are sorted.
type MulVec = Vec<((TinyVec<[usize; 2]>, TinyVec<[usize; 2]>), f64)>;
#[allow(unused)]
fn commute_creator_annihilator_fermionic(
    annihilators_left: &[usize],
    creators_right: &[usize],
) -> Vec<(CreatorsAnnihilators, f64)> {
    let mut result: Vec<(CreatorsAnnihilators, f64)> = Vec::new();
    // Total parity swapping all creators past all annihilators (multiplication)
    let orig_parity = if (creators_right.len() * annihilators_left.len()) % 2 == 0 {
        1.0
    } else {
        -1.0
    };
    let mut found = false;

    for (cindex, creator) in creators_right.iter().enumerate() {
        if let Some((aindex, _)) = annihilators_left
            .iter()
            .enumerate()
            .find(|(_, an)| *an == creator)
        {
            // need to swap past all annihilators before (aindex) and all creators after fitting creator (creators_right.len() - cindex - 1)
            let offset_parity = if (annihilators_left.len() - aindex + cindex - 1) % 2 == 0 {
                1.0
            } else {
                -1.0
            };
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
            let recursed_result: MulVec =
                commute_creator_annihilator_fermionic(&recurse_annihilators, &recurse_creators)
                    .into_iter()
                    .map(|((c, a), parity)| ((c, a), parity * offset_parity))
                    .collect();
            result.extend(recursed_result.iter().cloned());
            // The parity for the case where `creator` and the corresponding annihilator are commuted (with extra minus sign)
            // and not eliminated. creator is swapped with all creators in front of it plus all annihilators and annihilator is
            // swapped with all annihilators past it + all creators
            let commuted_parity =
                if (2 * annihilators_left.len() + creators_right.len() - aindex + cindex - 2) % 2
                    == 0
                {
                    1.0
                } else {
                    -1.0
                };
            for ((mut c, mut a), p) in recursed_result.clone() {
                c.insert(0, *creator);
                a.push(*creator);
                result.push(((c, a), p * offset_parity * commuted_parity))
            }
            found = true;
            break;
        }
    }
    if !found {
        result.push((
            (
                creators_right.iter().copied().collect(),
                annihilators_left.iter().copied().collect(),
            ),
            orig_parity,
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
     vec![((tiny_vec!([usize; 2] => 1, 3, 5), tiny_vec!([usize; 2] => 0, 2, 4)), -1.0)]; "0,2,4 - 1,3,5")]
    #[test_case(tiny_vec!([usize; 2] => 0), tiny_vec!([usize; 2] => 0),
     vec![((tiny_vec!([usize; 2] => 0), tiny_vec!([usize; 2] => 0)), -1.0), ((tiny_vec!([usize; 2]), tiny_vec!([usize; 2])), 1.0)]; "0, - 0")]
    // `commute_creator_annihilator_fermionic` will not reorder the indices in creators or in annihilators, hence one of the results here being [30, 1] with a parity of -1
    #[test_case(tiny_vec!([usize; 2] => 20), tiny_vec!([usize; 2]),
     vec![((tiny_vec!([usize; 2]), tiny_vec!([usize; 2] => 20)), 1.0)]; "20 - empty")]
    #[test_case(tiny_vec!([usize; 2] => 1,20), tiny_vec!([usize; 2] => 1,30),
     vec![((tiny_vec!([usize; 2] => 1,30), tiny_vec!([usize; 2] => 20,1)), -1.0), ((tiny_vec!([usize; 2] => 30), tiny_vec!([usize; 2] => 20)), 1.0)]; "1,20 - 1,30")]
    #[test_case(tiny_vec!([usize; 2] => 1,2,20), tiny_vec!([usize; 2] => 1,2,30),
     vec![((tiny_vec!([usize; 2] => 30), tiny_vec!([usize; 2] => 20)), 1.0), ((tiny_vec!([usize; 2] => 2,30), tiny_vec!([usize; 2] => 20,2)), -1.0),
          ((tiny_vec!([usize; 2] => 1,30), tiny_vec!([usize; 2] => 20,1)), -1.0), ((tiny_vec!([usize; 2] => 1,2,30), tiny_vec!([usize; 2] => 20,2,1)), 1.0)]; "1,2,20 - 1,2,30")]
    #[test_case(tiny_vec!([usize; 2] => 10,20,30), tiny_vec!([usize; 2] => 10,30),
    vec![((tiny_vec!([usize; 2]), tiny_vec!([usize; 2] => 20)), 1.0), ((tiny_vec!([usize; 2] => 30), tiny_vec!([usize; 2] => 20,30)), 1.0),
        ((tiny_vec!([usize; 2] => 10), tiny_vec!([usize; 2] => 20,10)), 1.0), ((tiny_vec!([usize; 2] => 10,30), tiny_vec!([usize; 2] => 20,30,10)), 1.0)]; "10,20,30 - 10,30")]
    #[test_case(tiny_vec!([usize; 2] => 10,20,30), tiny_vec!([usize; 2] => 10,30,40),
    vec![((tiny_vec!([usize; 2] => 40), tiny_vec!([usize; 2] => 20)), -1.0), ((tiny_vec!([usize; 2] => 30,40), tiny_vec!([usize; 2] => 20,30)), 1.0),
        ((tiny_vec!([usize; 2] => 10,40), tiny_vec!([usize; 2] => 20,10)), 1.0), ((tiny_vec!([usize; 2] => 10,30,40), tiny_vec!([usize; 2] => 20,30,10)), -1.0)]; "10,20,30 - 10,30,40")]
    fn commute_fermionic(
        annihilators_left: TinyVec<[usize; 2]>,
        creators_right: TinyVec<[usize; 2]>,
        expected: Vec<((ModeTinyVec, ModeTinyVec), f64)>,
    ) {
        let result = commute_creator_annihilator_fermionic(&annihilators_left, &creators_right);
        assert_eq!(result.len(), expected.len());
        for pair in expected {
            assert!(result.contains(&pair));
        }
    }
}
