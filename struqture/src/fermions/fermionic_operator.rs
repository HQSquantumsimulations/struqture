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

use super::{FermionHamiltonian, OperateOnFermions};
use crate::fermions::FermionProduct;
use crate::mappings::JordanWignerFermionToSpin;
use crate::spins::PauliOperator;
use crate::{
    GetValue, ModeIndex, OperateOnDensityMatrix, OperateOnModes, OperateOnState, StruqtureError,
    SymmetricIndex,
};
// use itertools::Itertools;
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

use indexmap::map::{Entry, Iter};
use indexmap::IndexMap;

/// FermionOperators are combinations of FermionProducts with specific CalculatorComplex coefficients.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::fermions::{OperateOnFermions, FermionProduct};
/// use struqture::fermions::FermionOperator;
/// let mut fo = FermionOperator::new();
///
/// // Representing the opetator $ 1/2 b_0^{dagger} + 1/5 b_1 $
/// // Creating a FermionProduct with a creation operator acting on mode 0 and no annihilation operators
/// let fp_0 = FermionProduct::new([0],[]).unwrap();
/// // Creating a FermionProduct with a annihilation operator acting on mode 1 and no creation operators
/// let fp_1 = FermionProduct::new([],[1]).unwrap();
/// fo.set(fp_0.clone(), CalculatorComplex::from(0.5)).unwrap();
/// fo.set(fp_1.clone(), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(fo.get(&fp_0), &CalculatorComplex::from(0.5));
/// assert_eq!(fo.get(&fp_1), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "FermionOperatorSerialize")]
#[serde(into = "FermionOperatorSerialize")]
pub struct FermionOperator {
    /// The internal HashMap of FermionProducts and coefficients (CalculatorComplex)
    internal_map: IndexMap<FermionProduct, CalculatorComplex>,
}

impl crate::SerializationSupport for FermionOperator {
    fn struqture_type() -> crate::StruqtureType {
        crate::StruqtureType::FermionOperator
    }
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for FermionOperator {
    fn schema_name() -> String {
        "FermionOperator".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <FermionOperatorSerialize>::json_schema(gen)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
struct FermionOperatorSerialize {
    items: Vec<(FermionProduct, CalculatorFloat, CalculatorFloat)>,
    serialisation_meta: crate::StruqtureSerialisationMeta,
}

impl TryFrom<FermionOperatorSerialize> for FermionOperator {
    type Error = StruqtureError;
    fn try_from(value: FermionOperatorSerialize) -> Result<Self, Self::Error> {
        let target_serialisation_meta =
            <Self as crate::SerializationSupport>::target_serialisation_meta();
        crate::check_can_be_deserialised(&target_serialisation_meta, &value.serialisation_meta)?;
        let new_noise_op: FermionOperator = value
            .items
            .into_iter()
            .map(|(key, real, imag)| (key, CalculatorComplex { re: real, im: imag }))
            .collect();
        Ok(new_noise_op)
    }
}

impl From<FermionOperator> for FermionOperatorSerialize {
    fn from(value: FermionOperator) -> Self {
        let serialisation_meta = crate::SerializationSupport::struqture_serialisation_meta(&value);
        let new_noise_op: Vec<(FermionProduct, CalculatorFloat, CalculatorFloat)> = value
            .into_iter()
            .map(|(key, val)| (key, val.re, val.im))
            .collect();
        Self {
            items: new_noise_op,
            serialisation_meta,
        }
    }
}

impl<'a> OperateOnDensityMatrix<'a> for FermionOperator {
    type Index = FermionProduct;
    type Value = CalculatorComplex;

    // From trait
    fn get(&self, key: &FermionProduct) -> &CalculatorComplex {
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

    /// Overwrites an existing entry or sets a new entry in the FermionOperator with the given (FermionProduct key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The FermionProduct key to set in the FermionOperator.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the FermionOperator.
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

impl OperateOnState<'_> for FermionOperator {}

impl<'a> OperateOnModes<'a> for FermionOperator {
    /// Gets the maximum index of the FermionOperator.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of fermions in the FermionOperator.
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

impl OperateOnFermions<'_> for FermionOperator {}

/// Implements the default function (Default trait) of FermionOperator (an empty FermionOperator).
///
impl Default for FermionOperator {
    fn default() -> Self {
        Self::new()
    }
}

/// Functions for the FermionOperator
///
impl FermionOperator {
    /// Creates a new FermionOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) FermionOperator.
    pub fn new() -> Self {
        FermionOperator {
            internal_map: IndexMap::new(),
        }
    }

    /// Creates a new FermionOperator with pre-allocated capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The pre-allocated capacity of the operator.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) FermionOperator.
    pub fn with_capacity(capacity: usize) -> Self {
        FermionOperator {
            internal_map: IndexMap::with_capacity(capacity),
        }
    }

    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_export")]
    pub fn to_struqture_1(&self) -> Result<struqture_1::fermions::FermionSystem, StruqtureError> {
        let mut new_fermion_system = struqture_1::fermions::FermionSystem::new(None);
        for (key, val) in self.iter() {
            let one_key = key.to_struqture_1()?;
            let _ = struqture_1::OperateOnDensityMatrix::set(
                &mut new_fermion_system,
                one_key,
                val.clone(),
            );
        }
        Ok(new_fermion_system)
    }

    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_import")]
    pub fn from_struqture_1(
        value: &struqture_1::fermions::FermionSystem,
    ) -> Result<Self, StruqtureError> {
        let mut new_operator = Self::new();
        for (key, val) in struqture_1::OperateOnDensityMatrix::iter(value) {
            let self_key = FermionProduct::from_struqture_1(key)?;
            let _ = new_operator.set(self_key, val.clone());
        }
        Ok(new_operator)
    }
}

impl From<FermionHamiltonian> for FermionOperator {
    /// Converts a FermionHamiltonian into a FermionOperator.
    ///
    /// # Arguments
    ///
    /// * `hamiltonian` - The FermionHamiltonian to convert.
    ///
    /// # Returns
    ///
    /// * `Self` - The FermionHamiltonian converted into a FermionOperator.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from(hamiltonian: FermionHamiltonian) -> Self {
        let mut internal = FermionOperator::new();
        for (key, value) in hamiltonian.into_iter() {
            let bp = FermionProduct::get_key(&key);
            internal
                .add_operator_product(bp.clone(), value.clone())
                .expect("Internal bug in add_operator_product");
            if !key.is_natural_hermitian() {
                let bp_conj = bp.hermitian_conjugate();
                internal
                    .add_operator_product(FermionProduct::get_key(&bp_conj.0), value * bp_conj.1)
                    .expect("Internal error in add_operator_product");
            }
        }
        internal
    }
}

/// Implements the negative sign function of FermionOperator.
///
impl ops::Neg for FermionOperator {
    type Output = FermionOperator;
    /// Implement minus sign for FermionOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The FermionOperator * -1.
    fn neg(self) -> Self {
        let mut internal = self.internal_map.clone();
        for key in self.keys() {
            internal.insert(key.clone(), internal[key].clone() * -1.0);
        }
        FermionOperator {
            internal_map: internal,
        }
    }
}

/// Implements the plus function of FermionOperator by FermionOperator.
///
impl ops::Add<FermionOperator> for FermionOperator {
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
    fn add(mut self, other: FermionOperator) -> Self {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key, value)
                .expect("Internal bug in add_operator_product");
        }
        self
    }
}

/// Implements the minus function of FermionOperator by FermionOperator.
///
impl ops::Sub<FermionOperator> for FermionOperator {
    type Output = Self;
    /// Implements `-` (subtract) for two FermionOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The FermionOperator to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Self` - The two FermionOperators subtracted.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn sub(mut self, other: FermionOperator) -> Self {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key, value * -1.0)
                .expect("Internal bug in add_operator_product");
        }
        self
    }
}

/// Implements the multiplication function of FermionOperator by CalculatorComplex/CalculatorFloat.
///
impl<T> ops::Mul<T> for FermionOperator
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for FermionOperator and CalculatorComplex/CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex or CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The FermionOperator multiplied by the CalculatorComplex/CalculatorFloat.
    fn mul(self, other: T) -> Self {
        let other_cc = Into::<CalculatorComplex>::into(other);
        let mut internal = self.internal_map.clone();
        for key in self.keys() {
            internal.insert(key.clone(), internal[key].clone() * other_cc.clone());
        }
        FermionOperator {
            internal_map: internal,
        }
    }
}

/// Implements the multiplication function of FermionOperator by FermionOperator.
///
impl ops::Mul<FermionOperator> for FermionOperator {
    type Output = Self;
    /// Implement `*` for FermionOperator and FermionOperator.
    ///
    /// # Arguments
    ///
    /// * `other` - The FermionOperator to multiply by.
    ///
    /// # Returns
    ///
    /// * `Self` - The two FermionOperators multiplied.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn mul(self, other: FermionOperator) -> Self {
        let mut op = FermionOperator::with_capacity(self.len() * other.len());
        for (bps, vals) in self {
            for (bpo, valo) in other.iter() {
                let fermion_products = bps.clone() * bpo.clone();
                let coefficient = Into::<CalculatorComplex>::into(valo) * vals.clone();
                for (prod, coeff) in fermion_products {
                    op.add_operator_product(prod, coefficient.clone() * coeff)
                        .expect("Internal bug in add_operator_product");
                }
            }
        }
        op
    }
}

/// Implements the into_iter function (IntoIterator trait) of FermionOperator.
///
impl IntoIterator for FermionOperator {
    type Item = (FermionProduct, CalculatorComplex);
    type IntoIter = indexmap::map::IntoIter<FermionProduct, CalculatorComplex>;
    /// Returns the FermionOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The FermionOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference FermionOperator.
///
impl<'a> IntoIterator for &'a FermionOperator {
    type Item = (&'a FermionProduct, &'a CalculatorComplex);
    type IntoIter = Iter<'a, FermionProduct, CalculatorComplex>;

    /// Returns the reference FermionOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The FermionOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of FermionOperator.
///
impl FromIterator<(FermionProduct, CalculatorComplex)> for FermionOperator {
    /// Returns the object in FermionOperator form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the FermionOperator.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in FermionOperator form.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from_iter<I: IntoIterator<Item = (FermionProduct, CalculatorComplex)>>(iter: I) -> Self {
        let mut so = FermionOperator::new();
        for (pp, cc) in iter {
            so.add_operator_product(pp, cc)
                .expect("Internal bug in add_operator_product");
        }
        so
    }
}

/// Implements the extend function (Extend trait) of FermionOperator.
///
impl Extend<(FermionProduct, CalculatorComplex)> for FermionOperator {
    /// Extends the FermionOperator by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the FermionOperator.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn extend<I: IntoIterator<Item = (FermionProduct, CalculatorComplex)>>(&mut self, iter: I) {
        for (pp, cc) in iter {
            self.add_operator_product(pp, cc)
                .expect("Internal bug in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of FermionOperator.
///
impl fmt::Display for FermionOperator {
    /// Formats the FermionOperator using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted FermionOperator.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "FermionOperator{\n".to_string();
        for (key, val) in self.iter() {
            writeln!(output, "{}: {},", key, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}

impl JordanWignerFermionToSpin for FermionOperator {
    type Output = PauliOperator;

    /// Implements JordanWignerFermionToSpin for a FermionOperator.
    ///
    /// The convention used is that |0> represents an empty fermionic state (spin-orbital),
    /// and |1> represents an occupied fermionic state.
    ///
    /// # Returns
    ///
    /// `PauliOperator` - The spin operator that results from the transformation.
    fn jordan_wigner(&self) -> Self::Output {
        let mut out = PauliOperator::new();
        for fp in self.keys() {
            out = out + fp.jordan_wigner() * self.get(fp);
        }
        out
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::STRUQTURE_VERSION;
    use serde_test::{assert_tokens, Configure, Token};

    // Test the Clone and PartialEq traits of FermionOperator
    #[test]
    fn so_from_sos() {
        let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
        let sos = FermionOperatorSerialize {
            items: vec![(pp.clone(), 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "FermionOperator".to_string(),
                min_version: (2, 0, 0),
                version: STRUQTURE_VERSION.to_string(),
            },
        };
        let mut so = FermionOperator::new();
        so.set(pp, CalculatorComplex::from(0.5)).unwrap();

        assert_eq!(FermionOperator::try_from(sos.clone()).unwrap(), so);
        assert_eq!(FermionOperatorSerialize::from(so), sos);
    }
    // Test the Clone and PartialEq traits of FermionOperator
    #[test]
    fn clone_partial_eq() {
        let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
        let sos = FermionOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "FermionOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        // Test Clone trait
        assert_eq!(sos.clone(), sos);

        // Test PartialEq trait
        let pp_1: FermionProduct = FermionProduct::new([0], [0]).unwrap();
        let sos_1 = FermionOperatorSerialize {
            items: vec![(pp_1, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "FermionOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };
        let pp_2: FermionProduct = FermionProduct::new([1], [0]).unwrap();
        let sos_2 = FermionOperatorSerialize {
            items: vec![(pp_2, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "FermionOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };
        assert!(sos_1 == sos);
        assert!(sos == sos_1);
        assert!(sos_2 != sos);
        assert!(sos != sos_2);
    }

    // Test the Debug trait of FermionOperator
    #[test]
    fn debug() {
        let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
        let sos = FermionOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "FermionOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_eq!(
            format!("{:?}", sos),
            "FermionOperatorSerialize { items: [(FermionProduct { creators: [0], annihilators: [0] }, Float(0.5), Float(0.0))], serialisation_meta: StruqtureSerialisationMeta { type_name: \"FermionOperator\", min_version: (2, 0, 0), version: \"2.0.0\" } }"
        );
    }

    /// Test FermionOperator Serialization and Deserialization traits (readable)
    #[test]
    fn serde_readable() {
        let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
        let sos = FermionOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "FermionOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_tokens(
            &sos.readable(),
            &[
                Token::Struct {
                    name: "FermionOperatorSerialize",
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
                Token::Str("FermionOperator"),
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

    /// Test FermionOperator Serialization and Deserialization traits (compact)
    #[test]
    fn serde_compact() {
        let pp: FermionProduct = FermionProduct::new([0], [0]).unwrap();
        let sos = FermionOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "FermionOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_tokens(
            &sos.compact(),
            &[
                Token::Struct {
                    name: "FermionOperatorSerialize",
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
                Token::Str("FermionOperator"),
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
