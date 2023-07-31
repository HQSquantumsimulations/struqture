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

use super::{GetValueMixed, HermitianMixedProduct, MixedIndex};
use crate::bosons::BosonProduct;
use crate::fermions::FermionProduct;
use crate::spins::PauliProduct;
use crate::{CorrespondsTo, StruqtureError, SymmetricIndex};
use num_complex::Complex64;
use serde::{
    de::{Error, SeqAccess, Visitor},
    ser::SerializeTuple,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{ops::Mul, str::FromStr};
use tinyvec::TinyVec;

/// A mixed product of pauli products, boson products and fermion products.
///
/// A [crate::spins::PauliProduct] is a representation of products of pauli matrices acting on qubits. It is used in order to build the corresponding spin terms of a hamiltonian.
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
/// use struqture::spins::PauliProduct;
/// use struqture::bosons::BosonProduct;
/// use struqture::fermions::FermionProduct;
/// use struqture::mixed_systems::MixedProduct;
///
/// let m_product = MixedProduct::new([PauliProduct::new().z(0)], [BosonProduct::new([0], [1]).unwrap()], [FermionProduct::new([0], [0]).unwrap()]).unwrap();
/// println!("{}", m_product);
///
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct MixedProduct {
    /// List of spin sub-indices
    pub(crate) spins: TinyVec<[PauliProduct; 2]>,
    /// List of boson sub-indices
    pub(crate) bosons: TinyVec<[BosonProduct; 2]>,
    /// List of fermion sub-indices
    pub(crate) fermions: TinyVec<[FermionProduct; 2]>,
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for MixedProduct {
    fn schema_name() -> String {
        "MixedProduct".to_string()
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let tmp_schema = gen.subschema_for::<String>();
        let mut obj = tmp_schema.into_object();
        let meta = obj.metadata();
        meta.description = Some("Represents products of Spin operators and Bosonic and Fermionic creators and annhilators by a string. Spin Operators  X, Y and Z are preceeded and creators (c) and annihilators (a) are followed by the modes they are acting on. E.g. :S0X1Y:Bc0a0:Fc0a0:.".to_string());

        schemars::schema::Schema::Object(obj)
    }
}

impl crate::MinSupportedVersion for MixedProduct {}

impl Serialize for MixedProduct {
    /// Serialization function for MixedProduct according to string type.
    ///
    /// # Arguments
    ///
    /// * `self` - MixedProduct to be serialized.
    /// * `serializer` - Serializer used for serialization.
    ///
    /// # Returns
    ///
    /// `S::Ok` - Serialized instance of MixedProduct.
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
impl<'de> Deserialize<'de> for MixedProduct {
    /// Deserialization function for MixedProduct.
    ///
    /// # Arguments
    ///
    /// * `self` - Serialized instance of MixedProduct to be deserialized.
    /// * `deserializer` - Deserializer used for deserialization.
    ///
    /// # Returns
    ///
    /// `DecoherenceProduct` - Deserialized instance of MixedProduct.
    /// `D::Error` - Error in the deserialization process.
    fn deserialize<D>(deserializer: D) -> Result<MixedProduct, D::Error>
    where
        D: Deserializer<'de>,
    {
        let human_readable = deserializer.is_human_readable();
        if human_readable {
            struct TemporaryVisitor;
            impl<'de> Visitor<'de> for TemporaryVisitor {
                type Value = MixedProduct;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("String")
                }

                fn visit_str<E>(self, v: &str) -> Result<MixedProduct, E>
                where
                    E: serde::de::Error,
                {
                    MixedProduct::from_str(v).map_err(|err| E::custom(format!("{:?}", err)))
                }

                fn visit_borrowed_str<E>(self, v: &'de str) -> Result<MixedProduct, E>
                where
                    E: serde::de::Error,
                {
                    MixedProduct::from_str(v).map_err(|err| E::custom(format!("{:?}", err)))
                }
            }

            deserializer.deserialize_str(TemporaryVisitor)
        } else {
            struct MixedProductVisitor;
            impl<'de> serde::de::Visitor<'de> for MixedProductVisitor {
                type Value = MixedProduct;
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
                    let spins: TinyVec<[PauliProduct; 2]> = match access.next_element()? {
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

                    Ok(MixedProduct {
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

impl MixedIndex for MixedProduct {
    type SpinIndexType = PauliProduct;
    type BosonicIndexType = BosonProduct;
    type FermionicIndexType = FermionProduct;

    /// Creates a new MixedProduct.
    ///
    /// # Arguments
    ///
    /// * `spins` - Products of pauli operators acting on qubits.
    /// * `bosons` - Products of bosonic creation and annihilation operators.
    /// * `fermions` - Products of fermionic creation and annihilation operators.
    ///
    /// # Returns
    ///
    /// * Ok(`Self`) - a new MixedProduct with the input of spins and bosons.
    fn new(
        spins: impl IntoIterator<Item = Self::SpinIndexType>,
        bosons: impl IntoIterator<Item = Self::BosonicIndexType>,
        fermions: impl IntoIterator<Item = Self::FermionicIndexType>,
    ) -> Result<Self, StruqtureError> {
        Ok(Self {
            spins: spins.into_iter().collect(),
            bosons: bosons.into_iter().collect(),
            fermions: fermions.into_iter().collect(),
        })
    }

    // From trait
    fn spins(&self) -> std::slice::Iter<PauliProduct> {
        self.spins.iter()
    }

    // From trait
    fn bosons(&self) -> std::slice::Iter<BosonProduct> {
        self.bosons.iter()
    }

    // From trait
    fn fermions(&self) -> std::slice::Iter<FermionProduct> {
        self.fermions.iter()
    }

    /// Creates a pair (MixedProduct, CalculatorComplex).
    ///
    /// The first item is the valid MixedProduct created from the input spins, bosons and fermions.
    /// The second term is the input CalculatorComplex transformed according to the valid order of inputs.
    ///
    /// # Arguments
    ///
    /// * `spins` - The PauliProducts to have in the MixedProduct.
    /// * `bosons` - The BosonProducts to have in the MixedProduct.
    /// * `fermions` - The FermionProducts to have in the MixedProduct.
    /// * `value` - The CalculatorComplex to transform.
    ///
    /// # Returns
    ///
    /// * `Ok((MixedProduct, CalculatorComplex))` - The valid MixedProduct and the corresponding transformed CalculatorComplex.
    fn create_valid_pair(
        spins: impl IntoIterator<Item = Self::SpinIndexType>,
        bosons: impl IntoIterator<Item = Self::BosonicIndexType>,
        fermions: impl IntoIterator<Item = Self::FermionicIndexType>,
        value: qoqo_calculator::CalculatorComplex,
    ) -> Result<(Self, qoqo_calculator::CalculatorComplex), StruqtureError> {
        let spins: TinyVec<[PauliProduct; 2]> = spins.into_iter().collect();
        let bosons: TinyVec<[BosonProduct; 2]> = bosons.into_iter().collect();
        let fermions: TinyVec<[FermionProduct; 2]> = fermions.into_iter().collect();
        Ok((
            Self {
                spins,
                bosons,
                fermions,
            },
            value,
        ))
    }
}

impl FromStr for MixedProduct {
    type Err = StruqtureError;
    /// Constructs a MixedProduct from a string.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to convert.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The successfully converted MixedProduct.
    /// * `Err(StruqtureError::ParsingError)` - Encountered subsystem that is neither spin, nor boson, nor fermion.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut spins: TinyVec<[PauliProduct; 2]> = TinyVec::<[PauliProduct; 2]>::with_capacity(2);
        let mut bosons: TinyVec<[BosonProduct; 2]> = TinyVec::<[BosonProduct; 2]>::with_capacity(2);
        let mut fermions: TinyVec<[FermionProduct; 2]> =
            TinyVec::<[FermionProduct; 2]>::with_capacity(2);
        let subsystems = s.split(':').filter(|s| !s.is_empty());
        for subsystem in subsystems {
            if let Some(rest) = subsystem.strip_prefix('S') {
                spins.push(PauliProduct::from_str(rest)?);
            } else if let Some(rest) = subsystem.strip_prefix('B') {
                bosons.push(BosonProduct::from_str(rest)?);
            } else if let Some(rest) = subsystem.strip_prefix('F') {
                fermions.push(FermionProduct::from_str(rest)?);
            } else {
                return Err(StruqtureError::ParsingError {
                    target_type: "MixedIndex".to_string(),
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

impl<'a> GetValueMixed<'a, MixedProduct> for MixedProduct {
    /// Gets the key corresponding to the input index (here, itself).
    ///
    /// # Arguments
    ///
    /// * `index` - The index for which to get the corresponding Product.
    ///
    /// # Returns
    ///
    /// * `Self` - The corresponding MixedProduct.
    fn get_key(index: &MixedProduct) -> Self {
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
        _index: &MixedProduct,
        value: qoqo_calculator::CalculatorComplex,
    ) -> qoqo_calculator::CalculatorComplex {
        value
    }
}

impl<'a> GetValueMixed<'a, HermitianMixedProduct> for MixedProduct {
    /// Gets the key corresponding to the input index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index for which to get the corresponding Product.
    ///
    /// # Returns
    ///
    /// * `Self` - The corresponding MixedProduct.
    fn get_key(index: &HermitianMixedProduct) -> Self {
        Self {
            spins: index.spins().cloned().collect(),
            bosons: index.bosons().cloned().collect(),
            fermions: index.fermions().cloned().collect(),
        }
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
        _index: &HermitianMixedProduct,
        value: qoqo_calculator::CalculatorComplex,
    ) -> qoqo_calculator::CalculatorComplex {
        value
    }
}

impl CorrespondsTo<MixedProduct> for MixedProduct {
    /// Gets the MixedProduct corresponding to self (here, itself).
    ///
    /// # Returns
    ///
    /// * `MixedProduct` - The MixedProduct corresponding to Self.
    fn corresponds_to(&self) -> MixedProduct {
        self.clone()
    }
}

impl SymmetricIndex for MixedProduct {
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

/// Implements the multiplication function of MixedProduct by MixedProduct.
///
impl Mul<MixedProduct> for MixedProduct {
    type Output = Result<Vec<(MixedProduct, Complex64)>, StruqtureError>;

    /// Implement `*` for MixedProduct and MixedProduct.
    ///
    /// # Arguments
    ///
    /// * `other` - The MixedProduct to multiply by.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<(MixedProduct, Complex64)>)` - The two MixedProducts multiplied.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in left and right do not match.
    fn mul(self, rhs: MixedProduct) -> Self::Output {
        if self.spins().len() != rhs.spins().len()
            || self.bosons().len() != rhs.bosons().len()
            || self.fermions().len() != rhs.fermions().len()
        {
            return Err(StruqtureError::MissmatchedNumberSubsystems {
                target_number_spin_subsystems: self.spins().len(),
                target_number_boson_subsystems: self.bosons().len(),
                target_number_fermion_subsystems: self.fermions().len(),
                actual_number_spin_subsystems: rhs.spins().len(),
                actual_number_boson_subsystems: rhs.bosons().len(),
                actual_number_fermion_subsystems: rhs.fermions().len(),
            });
        }
        let mut coefficient = Complex64::new(1.0, 0.0);
        let mut result_vec: Vec<(MixedProduct, Complex64)> = Vec::new();
        let mut tmp_spins: Vec<PauliProduct> = Vec::with_capacity(self.spins().len());
        let mut tmp_bosons: Vec<Vec<BosonProduct>> = Vec::with_capacity(self.bosons().len());
        let mut tmp_fermions: Vec<Vec<(FermionProduct, f64)>> =
            Vec::with_capacity(self.fermions().len());
        for (left, right) in self.spins.into_iter().zip(rhs.spins.into_iter()) {
            let (val, coeff) = left * right;
            tmp_spins.push(val);
            coefficient *= coeff;
        }
        // iterate through boson subsystems and multiply subsystem
        for (left, right) in self.bosons.into_iter().zip(rhs.bosons.into_iter()) {
            let boson_multiplication = left.clone() * right.clone();
            if !tmp_bosons.is_empty() {
                let mut internal_tmp_bosons: Vec<Vec<BosonProduct>> = Vec::new();
                for bp in boson_multiplication.clone() {
                    for tmp_bp in tmp_bosons.iter() {
                        let mut tmp_entry = tmp_bp.clone();
                        tmp_entry.push(bp.clone());
                        internal_tmp_bosons.push(tmp_entry);
                    }
                }
                tmp_bosons = internal_tmp_bosons.clone();
            } else {
                for bp in boson_multiplication.clone() {
                    tmp_bosons.push(vec![bp]);
                }
            }
        }
        for (left, right) in self.fermions.into_iter().zip(rhs.fermions.into_iter()) {
            let fermion_multiplication = left * right;
            if !tmp_fermions.is_empty() {
                let mut internal_tmp_fermions: Vec<Vec<(FermionProduct, f64)>> = Vec::new();
                for fp in fermion_multiplication {
                    for tmp_fp in tmp_fermions.iter() {
                        let mut tmp_entry = tmp_fp.clone();
                        tmp_entry.push(fp.clone());
                        internal_tmp_fermions.push(tmp_entry);
                    }
                }
                tmp_fermions = internal_tmp_fermions;
            } else {
                for fp in fermion_multiplication.clone() {
                    tmp_fermions.push(vec![fp]);
                }
            }
        }

        // Combining results
        for boson in tmp_bosons.clone() {
            if !tmp_fermions.is_empty() {
                for fermion in tmp_fermions.iter() {
                    let mut fermion_vec: Vec<FermionProduct> = Vec::new();
                    let mut sign = Complex64::new(1.0, 0.0);
                    for (f, val) in fermion {
                        fermion_vec.push(f.clone());
                        sign *= val;
                    }
                    result_vec.push((
                        MixedProduct::new(tmp_spins.clone(), boson.clone(), fermion_vec)?,
                        coefficient * sign,
                    ));
                }
            } else {
                result_vec.push((
                    MixedProduct::new(tmp_spins.clone(), boson.clone(), vec![])?,
                    coefficient,
                ));
            }
        }
        if tmp_bosons.is_empty() && !tmp_fermions.is_empty() {
            for fermion in tmp_fermions.iter() {
                let mut fermion_vec: Vec<FermionProduct> = Vec::new();
                let mut sign = Complex64::new(1.0, 0.0);
                for (f, val) in fermion {
                    fermion_vec.push(f.clone());
                    sign *= val;
                }
                result_vec.push((
                    MixedProduct::new(tmp_spins.clone(), [], fermion_vec)?,
                    coefficient * sign,
                ));
            }
        } else if tmp_bosons.is_empty() && tmp_fermions.is_empty() {
            result_vec.push((MixedProduct::new(tmp_spins.clone(), [], [])?, coefficient))
        }

        Ok(result_vec)
    }
}

impl Mul<HermitianMixedProduct> for MixedProduct {
    type Output = Result<Vec<(MixedProduct, Complex64)>, StruqtureError>;

    /// Implement `*` for a MixedProduct and a HermitianMixedProduct.
    ///
    /// # Arguments
    ///
    /// * `other` - The HermitianMixedProduct to multiply by.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<(MixedProduct, Complex64)>)` - The two MixedProducts multiplied.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in left and right do not match.
    ///
    /// # Panics
    ///
    /// * Could not convert rhs into a MixedProduct.
    fn mul(self, rhs: HermitianMixedProduct) -> Self::Output {
        if self.spins().len() != rhs.spins().len()
            || self.bosons().len() != rhs.bosons().len()
            || self.fermions().len() != rhs.fermions().len()
        {
            return Err(StruqtureError::MissmatchedNumberSubsystems {
                target_number_spin_subsystems: self.spins().len(),
                target_number_boson_subsystems: self.bosons().len(),
                target_number_fermion_subsystems: self.fermions().len(),
                actual_number_spin_subsystems: rhs.spins().len(),
                actual_number_boson_subsystems: rhs.bosons().len(),
                actual_number_fermion_subsystems: rhs.fermions().len(),
            });
        }
        let mut result_vec: Vec<(MixedProduct, Complex64)> = Vec::new();

        let mut right_to_mul: Vec<(MixedProduct, f64)> = Vec::new();
        let mhp_right = MixedProduct::new(rhs.spins, rhs.bosons, rhs.fermions)
            .expect("Could not convert rhs into a MixedProduct");
        right_to_mul.push((mhp_right.clone(), 1.0));
        if !mhp_right.is_natural_hermitian() {
            right_to_mul.push(mhp_right.hermitian_conjugate());
        }

        for (rhs, rsign) in right_to_mul {
            let mut coefficient = Complex64::new(rsign, 0.0);
            let mut tmp_spins: Vec<PauliProduct> = Vec::with_capacity(self.spins().len());
            let mut tmp_bosons: Vec<Vec<BosonProduct>> = Vec::with_capacity(self.bosons().len());
            let mut tmp_fermions: Vec<Vec<(FermionProduct, f64)>> =
                Vec::with_capacity(self.fermions().len());
            for (left, right) in self.clone().spins.into_iter().zip(rhs.spins.into_iter()) {
                let (val, coeff) = left * right;
                tmp_spins.push(val);
                coefficient *= coeff;
            }
            // iterate through boson subsystems and multiply subsystem
            for (left, right) in self.clone().bosons.into_iter().zip(rhs.bosons.into_iter()) {
                let boson_multiplication = left.clone() * right.clone();
                if !tmp_bosons.is_empty() {
                    let mut internal_tmp_bosons: Vec<Vec<BosonProduct>> = Vec::new();
                    for bp in boson_multiplication.clone() {
                        for tmp_bp in tmp_bosons.iter() {
                            let mut tmp_entry = tmp_bp.clone();
                            tmp_entry.push(bp.clone());
                            internal_tmp_bosons.push(tmp_entry);
                        }
                    }
                    tmp_bosons = internal_tmp_bosons.clone();
                } else {
                    for bp in boson_multiplication.clone() {
                        tmp_bosons.push(vec![bp]);
                    }
                }
            }
            for (left, right) in self
                .fermions
                .clone()
                .into_iter()
                .zip(rhs.fermions.into_iter())
            {
                let fermion_multiplication = left * right;
                if !tmp_fermions.is_empty() {
                    let mut internal_tmp_fermions: Vec<Vec<(FermionProduct, f64)>> = Vec::new();
                    for fp in fermion_multiplication {
                        for tmp_fp in tmp_fermions.iter() {
                            let mut tmp_entry = tmp_fp.clone();
                            tmp_entry.push(fp.clone());
                            internal_tmp_fermions.push(tmp_entry);
                        }
                    }
                    tmp_fermions = internal_tmp_fermions;
                } else {
                    for fp in fermion_multiplication.clone() {
                        tmp_fermions.push(vec![fp]);
                    }
                }
            }

            // Combining results
            for boson in tmp_bosons.clone() {
                if !tmp_fermions.is_empty() {
                    for fermion in tmp_fermions.iter() {
                        let mut fermion_vec: Vec<FermionProduct> = Vec::new();
                        let mut sign = Complex64::new(1.0, 0.0);
                        for (f, val) in fermion {
                            fermion_vec.push(f.clone());
                            sign *= val;
                        }
                        result_vec.push((
                            MixedProduct::new(tmp_spins.clone(), boson.clone(), fermion_vec)?,
                            coefficient * sign,
                        ));
                    }
                } else {
                    result_vec.push((
                        MixedProduct::new(tmp_spins.clone(), boson.clone(), vec![])?,
                        coefficient,
                    ));
                }
            }
            if tmp_bosons.is_empty() && !tmp_fermions.is_empty() {
                for fermion in tmp_fermions.iter() {
                    let mut fermion_vec: Vec<FermionProduct> = Vec::new();
                    let mut sign = Complex64::new(1.0, 0.0);
                    for (f, val) in fermion {
                        fermion_vec.push(f.clone());
                        sign *= val;
                    }
                    result_vec.push((
                        MixedProduct::new(tmp_spins.clone(), [], fermion_vec)?,
                        coefficient * sign,
                    ));
                }
            } else if tmp_bosons.is_empty() && tmp_fermions.is_empty() {
                result_vec.push((MixedProduct::new(tmp_spins.clone(), [], [])?, coefficient))
            }
        }

        Ok(result_vec)
    }
}

/// Implements the format function (Display trait) of MixedProduct.
///
impl std::fmt::Display for MixedProduct {
    /// Formats the MixedProduct using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted MixedProduct.
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
    #[test_case(":S0X1X:", &[], &[PauliProduct::from_str("0X1X").unwrap()]; "single spin systems")]
    #[test_case(":S0X1X:S0Z:", &[], &[PauliProduct::from_str("0X1X").unwrap(), PauliProduct::from_str("0Z").unwrap()]; "two spin systems")]
    #[test_case(":S0X1X:Bc0a1:", &[BosonProduct::from_str("c0a1").unwrap()], &[PauliProduct::from_str("0X1X").unwrap()]; "spin-boson systems")]
    fn from_string(stringformat: &str, bosons: &[BosonProduct], spins: &[PauliProduct]) {
        let test_new = <MixedProduct as std::str::FromStr>::from_str(stringformat);
        assert!(test_new.is_ok());
        let res = test_new.unwrap();
        let empty_bosons: Vec<BosonProduct> = bosons.to_vec();
        let res_bosons: Vec<BosonProduct> = res.bosons.iter().cloned().collect_vec();
        assert_eq!(res_bosons, empty_bosons);
        let empty_spins: Vec<PauliProduct> = spins.to_vec();
        let res_spins: Vec<PauliProduct> = res.spins.iter().cloned().collect_vec();
        assert_eq!(res_spins, empty_spins);
    }
}
