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

use super::{MixedIndex, MixedProduct};
use crate::bosons::BosonProduct;
use crate::fermions::FermionProduct;
use crate::spins::{PauliProduct, PlusMinusProduct};
use crate::{ModeIndex, StruqtureError, SymmetricIndex};
use itertools::Itertools;
use num_complex::Complex64;
use serde::{
    de::{Error, SeqAccess, Visitor},
    ser::SerializeTuple,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::str::FromStr;
use tinyvec::TinyVec;

/// A mixed product of pauli products, boson products and fermion products.
///
/// A [crate::spins::PlusMinusProduct] is a representation of products of pauli matrices acting on qubits. It is used in order to build the corresponding spin terms of a hamiltonian.
///
/// A [crate::bosons::BosonProduct] is a product of bosonic creation and annihilation operators.
/// It is used as an index for non-hermitian, normal ordered bosonic operators.
///
/// A [crate::fermions::FermionProduct] is a product of fermionic creation and annihilation operators.
/// It is used as an index for non-hermitian, normal ordered fermionic operators.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use struqture::spins::PlusMinusProduct;
/// use struqture::bosons::BosonProduct;
/// use struqture::fermions::FermionProduct;
/// use struqture::mixed_systems::MixedPlusMinusProduct;
///
/// let m_product = MixedPlusMinusProduct::new([PlusMinusProduct::new().z(0)], [BosonProduct::new([0], [1]).unwrap()], [FermionProduct::new([0], [0]).unwrap()]);
/// println!("{}", m_product);
///
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct MixedPlusMinusProduct {
    /// List of spin sub-indices
    pub(crate) spins: TinyVec<[PlusMinusProduct; 2]>,
    /// List of boson sub-indices
    pub(crate) bosons: TinyVec<[BosonProduct; 2]>,
    /// List of fermion sub-indices
    pub(crate) fermions: TinyVec<[FermionProduct; 2]>,
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for MixedPlusMinusProduct {
    fn schema_name() -> String {
        "MixedPlusMinusProduct".to_string()
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let tmp_schema = gen.subschema_for::<String>();
        let mut obj = tmp_schema.into_object();
        let meta = obj.metadata();
        meta.description = Some("Represents products of Spin operators and Bosonic and Fermionic creators and annhilators by a string. Spin Operators  +, - and Z are preceeded and creators (c) and annihilators (a) are followed by the modes they are acting on. E.g. :S0+1+:Bc0a1:Fc0a2:.".to_string());

        schemars::schema::Schema::Object(obj)
    }
}

impl crate::SerializationSupport for MixedPlusMinusProduct {
    fn struqture_type() -> crate::StruqtureType {
        crate::StruqtureType::MixedPlusMinusProduct
    }
}

impl Serialize for MixedPlusMinusProduct {
    /// Serialization function for MixedPlusMinusProduct according to string type.
    ///
    /// # Arguments
    ///
    /// * `self` - MixedPlusMinusProduct to be serialized.
    /// * `serializer` - Serializer used for serialization.
    ///
    /// # Returns
    ///
    /// `S::Ok` - Serialized instance of MixedPlusMinusProduct.
    /// `S::Error` - Error in the serialization process.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let readable = serializer.is_human_readable();
        if readable {
            serializer.serialize_str(&self.to_string())
        } else {
            let mut tuple = serializer.serialize_tuple(3)?;
            tuple.serialize_element(&self.spins.as_slice())?;
            tuple.serialize_element(&self.bosons.as_slice())?;
            tuple.serialize_element(&self.fermions.as_slice())?;
            tuple.end()
        }
    }
}

/// Deserializing directly from string.
///
impl<'de> Deserialize<'de> for MixedPlusMinusProduct {
    /// Deserialization function for MixedPlusMinusProduct.
    ///
    /// # Arguments
    ///
    /// * `self` - Serialized instance of MixedPlusMinusProduct to be deserialized.
    /// * `deserializer` - Deserializer used for deserialization.
    ///
    /// # Returns
    ///
    /// `DecoherenceProduct` - Deserialized instance of MixedPlusMinusProduct.
    /// `D::Error` - Error in the deserialization process.
    fn deserialize<D>(deserializer: D) -> Result<MixedPlusMinusProduct, D::Error>
    where
        D: Deserializer<'de>,
    {
        let human_readable = deserializer.is_human_readable();
        if human_readable {
            struct TemporaryVisitor;
            impl<'de> Visitor<'de> for TemporaryVisitor {
                type Value = MixedPlusMinusProduct;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("String")
                }

                fn visit_str<E>(self, v: &str) -> Result<MixedPlusMinusProduct, E>
                where
                    E: serde::de::Error,
                {
                    MixedPlusMinusProduct::from_str(v)
                        .map_err(|err| E::custom(format!("{:?}", err)))
                }

                fn visit_borrowed_str<E>(self, v: &'de str) -> Result<MixedPlusMinusProduct, E>
                where
                    E: serde::de::Error,
                {
                    MixedPlusMinusProduct::from_str(v)
                        .map_err(|err| E::custom(format!("{:?}", err)))
                }
            }

            deserializer.deserialize_str(TemporaryVisitor)
        } else {
            struct MixedProductVisitor;
            impl<'de> serde::de::Visitor<'de> for MixedProductVisitor {
                type Value = MixedPlusMinusProduct;
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
                    let spins: TinyVec<[PlusMinusProduct; 2]> = match access.next_element()? {
                        Some(x) => x,
                        None => {
                            return Err(M::Error::custom("Missing spin sequence".to_string()));
                        }
                    };
                    let bosons: TinyVec<[BosonProduct; 2]> = match access.next_element()? {
                        Some(x) => x,
                        None => {
                            return Err(M::Error::custom("Missing boson sequence".to_string()));
                        }
                    };
                    let fermions: TinyVec<[FermionProduct; 2]> = match access.next_element()? {
                        Some(x) => x,
                        None => {
                            return Err(M::Error::custom("Missing fermion sequence".to_string()));
                        }
                    };

                    Ok(MixedPlusMinusProduct {
                        spins,
                        bosons,
                        fermions,
                    })
                }
            }
            let pp_visitor = MixedProductVisitor;

            deserializer.deserialize_tuple(3, pp_visitor)
        }
    }
}

impl MixedPlusMinusProduct {
    /// Creates a new MixedPlusMinusProduct.
    ///
    /// # Arguments
    ///
    /// * `spins` - Products of pauli operators acting on qubits.
    /// * `bosons` - Products of bosonic creation and annihilation operators.
    /// * `fermions` - Products of fermionic creation and annihilation operators.
    ///
    /// # Returns
    ///
    /// * Ok(`Self`) - a new MixedPlusMinusProduct with the input of spins and bosons.
    pub fn new(
        spins: impl IntoIterator<Item = PlusMinusProduct>,
        bosons: impl IntoIterator<Item = BosonProduct>,
        fermions: impl IntoIterator<Item = FermionProduct>,
    ) -> Self {
        Self {
            spins: spins.into_iter().collect(),
            bosons: bosons.into_iter().collect(),
            fermions: fermions.into_iter().collect(),
        }
    }

    /// Gets the spin Products of Self.
    ///
    /// # Returns
    ///
    /// * `Iter<PlusMinusProduct>` - The spin Products in Self.
    pub fn spins(&self) -> std::slice::Iter<PlusMinusProduct> {
        self.spins.iter()
    }

    /// Gets the boson Products of Self.
    ///
    /// # Returns
    ///
    /// * `Iter<BosonProduct>` - The boson Products in Self.
    pub fn bosons(&self) -> std::slice::Iter<BosonProduct> {
        self.bosons.iter()
    }

    /// Gets the fermion Products of Self.
    ///
    /// # Returns
    ///
    /// * `Iter<FermionProduct>` - The fermion Products in Self.
    pub fn fermions(&self) -> std::slice::Iter<FermionProduct> {
        self.fermions.iter()
    }

    /// Returns the current number of spins each subsystem acts upon.
    ///
    /// # Returns
    ///
    /// * `Vec<usize>` - Number of spins in each spin sub-system.
    pub fn current_number_spins(&self) -> Vec<usize> {
        self.spins().map(|s| s.current_number_spins()).collect()
    }

    /// Returns the current number of bosonic modes each subsystem acts upon.
    ///
    /// # Returns
    ///
    /// * `Vec<usize>` - Number of bosons in each boson sub-system.
    pub fn current_number_bosonic_modes(&self) -> Vec<usize> {
        self.bosons().map(|b| b.current_number_modes()).collect()
    }

    /// Returns the current number of fermionic modes each subsystem acts upon.
    ///
    /// # Returns
    ///
    /// * `Vec<usize>` - Number of fermions in each fermion sub-system.
    pub fn current_number_fermionic_modes(&self) -> Vec<usize> {
        self.fermions().map(|f| f.current_number_modes()).collect()
    }

    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_export")]
    pub fn to_struqture_1(
        &self,
    ) -> Result<struqture_1::mixed_systems::MixedPlusMinusProduct, StruqtureError> {
        let self_string = self.to_string();
        let struqture_1_product = struqture_1::mixed_systems::MixedPlusMinusProduct::from_str(
            &self_string,
        )
        .map_err(|err| StruqtureError::GenericError {
            msg: format!("{}", err),
        })?;
        Ok(struqture_1_product)
    }

    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_import")]
    pub fn from_struqture_1(
        value: &struqture_1::mixed_systems::MixedPlusMinusProduct,
    ) -> Result<Self, StruqtureError> {
        let value_string = value.to_string();
        let pauli_product = Self::from_str(&value_string)?;
        Ok(pauli_product)
    }
}

impl From<MixedProduct> for Vec<(MixedPlusMinusProduct, Complex64)> {
    /// Converts a MixedProduct into a vector of tuples of (MixedPlusMinusProduct, Complex64).
    ///
    /// # Arguments
    ///
    /// * `value` - The MixedProduct to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The MixedProduct converted into a vector of tuples of (MixedPlusMinusProduct, Complex64).
    fn from(value: MixedProduct) -> Self {
        let mut return_vec: Vec<(MixedPlusMinusProduct, Complex64)> = Vec::new();
        let mut spins_vec: Vec<Vec<(PlusMinusProduct, Complex64)>> = Vec::new();
        for mixed_product in value.spins() {
            let conversion = Vec::<(PlusMinusProduct, Complex64)>::from(mixed_product.clone());
            spins_vec.push(conversion);
        }

        // converted: list of entries with n subsystem PP (in vec) and prefactor
        let mut converted: Vec<(Vec<PlusMinusProduct>, Complex64)> = Vec::new();
        for (mp, prefactor) in spins_vec[0].clone() {
            converted.push((vec![mp], prefactor))
        }
        for element in spins_vec.iter().skip(1) {
            let mut new_converted = Vec::new();
            for ((left, prefactor), (right, right_factor)) in
                converted.iter().cartesian_product(element)
            {
                let mut new_vec = left.clone();
                new_vec.push(right.clone());
                new_converted.push((new_vec, prefactor * right_factor))
            }
            converted = new_converted;
        }

        for (vec_mp, cc) in converted {
            return_vec.push((
                MixedPlusMinusProduct::new(
                    vec_mp,
                    value.bosons().cloned(),
                    value.fermions().cloned(),
                ),
                cc,
            ));
        }
        return_vec
    }
}

impl TryFrom<MixedPlusMinusProduct> for Vec<(MixedProduct, Complex64)> {
    type Error = StruqtureError;
    /// Converts a MixedPlusMinusProduct into a vector of tuples of (MixedProduct, Complex64).
    ///
    /// # Arguments
    ///
    /// * `value` - The MixedPlusMinusProduct to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The MixedPlusMinusProduct converted into a vector of tuples of (MixedProduct, Complex64).
    fn try_from(value: MixedPlusMinusProduct) -> Result<Self, Self::Error> {
        let mut return_vec: Vec<(MixedProduct, Complex64)> = Vec::new();
        let mut spins_vec: Vec<Vec<(PauliProduct, Complex64)>> = Vec::new();
        for mixed_product in value.spins() {
            let conversion = Vec::<(PauliProduct, Complex64)>::from(mixed_product.clone());
            spins_vec.push(conversion);
        }

        // converted: list of entries with n subsystem PP (in vec) and prefactor
        let mut converted: Vec<(Vec<PauliProduct>, Complex64)> = Vec::new();
        for (mp, prefactor) in spins_vec[0].clone() {
            converted.push((vec![mp], prefactor))
        }
        for element in spins_vec.iter().skip(1) {
            let mut new_converted = Vec::new();
            for ((left, prefactor), (right, right_factor)) in
                converted.iter().cartesian_product(element)
            {
                let mut new_vec = left.clone();
                new_vec.push(right.clone());
                new_converted.push((new_vec, prefactor * right_factor))
            }
            converted = new_converted;
        }

        for (vec_mp, cc) in converted {
            return_vec.push((
                MixedProduct::new(vec_mp, value.bosons().cloned(), value.fermions().cloned())?,
                cc,
            ));
        }
        Ok(return_vec)
    }
}

impl FromStr for MixedPlusMinusProduct {
    type Err = StruqtureError;
    /// Constructs a MixedPlusMinusProduct from a string.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to convert.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The successfully converted MixedPlusMinusProduct.
    /// * `Err(StruqtureError::ParsingError)` - Encountered subsystem that is neither spin, nor boson, nor fermion.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut spins: TinyVec<[PlusMinusProduct; 2]> =
            TinyVec::<[PlusMinusProduct; 2]>::with_capacity(2);
        let mut bosons: TinyVec<[BosonProduct; 2]> = TinyVec::<[BosonProduct; 2]>::with_capacity(2);
        let mut fermions: TinyVec<[FermionProduct; 2]> =
            TinyVec::<[FermionProduct; 2]>::with_capacity(2);
        let subsystems = s.split(':').filter(|s| !s.is_empty());
        for subsystem in subsystems {
            if let Some(rest) = subsystem.strip_prefix('S') {
                spins.push(PlusMinusProduct::from_str(rest)?);
            } else if let Some(rest) = subsystem.strip_prefix('B') {
                bosons.push(BosonProduct::from_str(rest)?);
            } else if let Some(rest) = subsystem.strip_prefix('F') {
                fermions.push(FermionProduct::from_str(rest)?);
            } else {
                return Err(StruqtureError::ParsingError {
                    target_type: "MixedPlusMinusProduct".to_string(),
                    msg: format!(
                        "Encountered subsystem that is neither spin, nor boson, nor fermion: {}",
                        subsystem
                    ),
                });
            }
        }

        Ok(Self {
            spins,
            bosons,
            fermions,
        })
    }
}

impl SymmetricIndex for MixedPlusMinusProduct {
    // From trait
    fn hermitian_conjugate(&self) -> (Self, f64) {
        let mut coefficient = 1.0;

        let mut new_spins = self.spins.clone();
        for spin in new_spins.iter_mut() {
            let (conj_spin, coeff) = spin.hermitian_conjugate();
            *spin = conj_spin;
            coefficient *= coeff;
        }
        let mut new_bosons = self.bosons.clone();
        for boson in new_bosons.iter_mut() {
            let (conj_boson, coeff) = boson.hermitian_conjugate();
            *boson = conj_boson;
            coefficient *= coeff;
        }
        let mut new_fermions = self.fermions.clone();
        for fermion in new_fermions.iter_mut() {
            let (conj_fermion, coeff) = fermion.hermitian_conjugate();
            *fermion = conj_fermion;
            coefficient *= coeff;
        }
        (
            Self {
                spins: new_spins,
                bosons: new_bosons,
                fermions: new_fermions,
            },
            coefficient,
        )
    }

    // From trait
    fn is_natural_hermitian(&self) -> bool {
        self.bosons.iter().all(|b| b.is_natural_hermitian())
            && self.fermions.iter().all(|f| f.is_natural_hermitian())
    }
}

/// Implements the format function (Display trait) of MixedPlusMinusProduct.
///
impl std::fmt::Display for MixedPlusMinusProduct {
    /// Formats the MixedPlusMinusProduct using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted MixedPlusMinusProduct.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string: String = String::new();
        for spin in self.spins() {
            string.push_str(format!("S{}:", spin).as_str());
        }
        for boson in self.bosons() {
            string.push_str(format!("B{}:", boson).as_str());
        }
        for fermion in self.fermions() {
            string.push_str(format!("F{}:", fermion).as_str());
        }
        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::Itertools;
    use test_case::test_case;

    #[test_case("", &[], &[]; "empty")]
    #[test_case(":S0+1+:", &[], &[PlusMinusProduct::from_str("0+1+").unwrap()]; "single spin systems")]
    #[test_case(":S0+1+:S0Z:", &[], &[PlusMinusProduct::from_str("0+1+").unwrap(), PlusMinusProduct::from_str("0Z").unwrap()]; "two spin systems")]
    #[test_case(":S0+1+:Bc0a1:", &[BosonProduct::from_str("c0a1").unwrap()], &[PlusMinusProduct::from_str("0+1+").unwrap()]; "spin-boson systems")]
    fn from_string(stringformat: &str, bosons: &[BosonProduct], spins: &[PlusMinusProduct]) {
        let test_new = <MixedPlusMinusProduct as std::str::FromStr>::from_str(stringformat);
        assert!(test_new.is_ok());
        let res = test_new.unwrap();
        let empty_bosons: Vec<BosonProduct> = bosons.to_vec();
        let res_bosons: Vec<BosonProduct> = res.bosons.iter().cloned().collect_vec();
        assert_eq!(res_bosons, empty_bosons);
        let empty_spins: Vec<PlusMinusProduct> = spins.to_vec();
        let res_spins: Vec<PlusMinusProduct> = res.spins.iter().cloned().collect_vec();
        assert_eq!(res_spins, empty_spins);
    }
}
