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

use super::{BosonOperator, BosonProduct, HermitianBosonProduct, ModeIndex, OperateOnBosons};
use crate::{
    GetValue, OperateOnDensityMatrix, OperateOnModes, OperateOnState, StruqtureError,
    StruqtureVersionSerializable, SymmetricIndex, MINIMUM_STRUQTURE_VERSION,
};
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Entry, Iter, Keys, Values};
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

/// BosonHamiltonians are combinations of HermitianBosonProducts with specific CalculatorComplex coefficients.
///
/// This is a representation of sums of creation and annihilation operators with weightings (and their hermitian conjugates), in order to build a full hamiltonian.
/// BosonHamiltonian is the hermitian equivalent of BosonOperator.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::bosons::{ HermitianBosonProduct, BosonHamiltonian};
/// use struqture::prelude::*;
///
/// let mut sh = BosonHamiltonian::new();
///
/// let bp_0 = HermitianBosonProduct::new([0], [1]).unwrap();
/// let bp_1 = HermitianBosonProduct::new([], [0]).unwrap();
/// sh.set(bp_0.clone(), CalculatorComplex::from(0.5)).unwrap();
/// sh.set(bp_1.clone(), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(sh.get(&bp_0), &CalculatorComplex::from(0.5));
/// assert_eq!(sh.get(&bp_1), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "BosonHamiltonianSerialize")]
#[serde(into = "BosonHamiltonianSerialize")]
pub struct BosonHamiltonian {
    /// The internal HashMap of HermitianBosonProducts and coefficients (CalculatorComplex)
    internal_map: HashMap<HermitianBosonProduct, CalculatorComplex>,
}

impl crate::MinSupportedVersion for BosonHamiltonian {}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for BosonHamiltonian {
    fn schema_name() -> String {
        "BosonHamiltonian".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <BosonHamiltonianSerialize>::json_schema(gen)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
struct BosonHamiltonianSerialize {
    items: Vec<(HermitianBosonProduct, CalculatorFloat, CalculatorFloat)>,
    _struqture_version: StruqtureVersionSerializable,
}

impl From<BosonHamiltonianSerialize> for BosonHamiltonian {
    fn from(value: BosonHamiltonianSerialize) -> Self {
        let new_noise_op: BosonHamiltonian = value
            .items
            .into_iter()
            .map(|(key, real, imag)| (key, CalculatorComplex { re: real, im: imag }))
            .collect();
        new_noise_op
    }
}

impl From<BosonHamiltonian> for BosonHamiltonianSerialize {
    fn from(value: BosonHamiltonian) -> Self {
        let new_noise_op: Vec<(HermitianBosonProduct, CalculatorFloat, CalculatorFloat)> = value
            .into_iter()
            .map(|(key, val)| (key, val.re, val.im))
            .collect();
        let current_version = StruqtureVersionSerializable {
            major_version: MINIMUM_STRUQTURE_VERSION.0,
            minor_version: MINIMUM_STRUQTURE_VERSION.1,
        };
        Self {
            items: new_noise_op,
            _struqture_version: current_version,
        }
    }
}

impl<'a> OperateOnDensityMatrix<'a> for BosonHamiltonian {
    type Index = HermitianBosonProduct;
    type Value = CalculatorComplex;
    type IteratorType = Iter<'a, Self::Index, Self::Value>;
    type KeyIteratorType = Keys<'a, Self::Index, Self::Value>;
    type ValueIteratorType = Values<'a, Self::Index, Self::Value>;

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

    /// Overwrites an existing entry or sets a new entry in the BosonHamiltonian with the given (HermitianBosonProduct key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The HermitianBosonProduct key to set in the BosonHamiltonian.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the BosonHamiltonian.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(CalculatorComplex))` - The key existed, this is the value it had before it was set with the value input.
    /// * `Ok(None)` - The key did not exist, it has been set with its corresponding value.
    /// * `Err(StruqtureError::NonHermitianOperator)` - Key is naturally hermitian (on-diagonal term), but its corresponding value is not real.
    fn set(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<Option<Self::Value>, StruqtureError> {
        if value.re != CalculatorFloat::ZERO || value.im != CalculatorFloat::ZERO {
            // Catch on diagonals with non-zero imaginary values
            if key.is_natural_hermitian() && value.im != CalculatorFloat::ZERO {
                Err(StruqtureError::NonHermitianOperator)
            } else {
                Ok(self.internal_map.insert(key, value))
            }
        } else {
            match self.internal_map.entry(key) {
                Entry::Occupied(val) => Ok(Some(val.remove())),
                Entry::Vacant(_) => Ok(None),
            }
        }
    }

    /// Adds a new (HermitianBosonProduct key, CalculatorComplex value) pair to the BosonHamiltonian.
    ///
    /// # Arguments
    ///
    /// * `key` - The HermitianBosonProduct key to added to the BosonHamiltonian.
    /// * `value` - The corresponding CalculatorComplex value to add for the key in the BosonHamiltonian.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The (key, value) pair was successfully added.
    /// * `Err(StruqtureError::NonHermitianOperator)` - Key is naturally hermitian (on-diagonal term), but its corresponding value is not real.
    fn add_operator_product(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<(), StruqtureError> {
        let old = self.get(&key).clone();
        let new_val = value + old;
        if key.is_natural_hermitian() && new_val.im != CalculatorFloat::ZERO {
            Err(StruqtureError::NonHermitianOperator)
        } else {
            self.set(key, new_val)?;
            Ok(())
        }
    }
}

impl<'a> OperateOnState<'a> for BosonHamiltonian {
    // From trait
    fn hermitian_conjugate(&self) -> Self {
        self.clone()
    }
}

impl<'a> OperateOnModes<'a> for BosonHamiltonian {
    /// Returns maximum index in BosonHamiltonian internal_map.
    ///
    /// # Returns
    ///
    /// * `usize` - Maximum index.
    fn current_number_modes(&self) -> usize {
        let mut max_mode: usize = 0;
        if !self.internal_map.is_empty() {
            for key in self.internal_map.keys() {
                if key.current_number_modes() > max_mode {
                    max_mode = key.current_number_modes()
                }
            }
        }
        max_mode
    }

    /// Gets the maximum index of the BosonHamiltonian.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of modes in the BosonHamiltonian.
    fn number_modes(&self) -> usize {
        self.current_number_modes()
    }
}

impl<'a> OperateOnBosons<'a> for BosonHamiltonian {}

/// Implements the default function (Default trait) of BosonHamiltonian (an empty BosonHamiltonian).
///
impl Default for BosonHamiltonian {
    fn default() -> Self {
        Self::new()
    }
}

/// Functions for the BosonHamiltonian
///
impl BosonHamiltonian {
    /// Creates a new BosonHamiltonian.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) BosonHamiltonian.
    pub fn new() -> Self {
        BosonHamiltonian {
            internal_map: HashMap::new(),
        }
    }

    /// Creates a new BosonHamiltonian with pre-allocated capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The pre-allocated capacity of the hamiltonian.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) BosonHamiltonian.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            internal_map: HashMap::with_capacity(capacity),
        }
    }

    /// Separate self into an operator with the terms of given number of creation and annihilation operators and an operator with the remaining operations
    ///
    /// # Arguments
    ///
    /// * `number_creators_annihilators` - Number of creation and annihilation terms to filter for in the keys.
    ///
    /// # Returns
    ///
    /// `Ok((separated, remainder))` - Operator with the noise terms where number_creators_annihilators matches the number of spins the operator product acts on and Operator with all other contributions.
    pub fn separate_into_n_terms(
        &self,
        number_creators_annihilators: (usize, usize),
    ) -> Result<(Self, Self), StruqtureError> {
        let mut separated = Self::default();
        let mut remainder = Self::default();
        for (prod, val) in self.iter() {
            if (prod.creators().len(), prod.annihilators().len()) == number_creators_annihilators {
                separated.add_operator_product(prod.clone(), val.clone())?;
            } else {
                remainder.add_operator_product(prod.clone(), val.clone())?;
            }
        }
        Ok((separated, remainder))
    }
}

impl TryFrom<BosonOperator> for BosonHamiltonian {
    type Error = StruqtureError;
    /// Tries to convert a BosonOperator into a BosonHamiltonian.
    ///
    /// # Arguments
    ///
    /// * `hamiltonian` - The BosonOperator to try to convert.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The BosonOperator converted into a BosonHamiltonian.
    /// * `Err(StruqtureError::CreatorsAnnihilatorsMinimumIndex)` - The minimum index of the creators is larger than the minimum index of the annihilators.
    /// * `Err(StruqtureError::NonHermitianOperator)` - Key is naturally hermitian (on-diagonal term), but its corresponding value is not real.
    fn try_from(hamiltonian: BosonOperator) -> Result<Self, StruqtureError> {
        let mut internal = BosonHamiltonian::new();
        for (key, value) in hamiltonian.into_iter() {
            if key.creators().min() > key.annihilators().min() {
                return Err(StruqtureError::CreatorsAnnihilatorsMinimumIndex {
                    creators_min: key.creators().min().cloned(),
                    annihilators_min: key.annihilators().min().cloned(),
                });
            } else {
                let bp = HermitianBosonProduct::get_key(&key);
                internal.add_operator_product(bp, value)?;
            }
        }
        Ok(internal)
    }
}

/// Implements the negative sign function of BosonHamiltonian.
///
impl ops::Neg for BosonHamiltonian {
    type Output = BosonHamiltonian;
    /// Implement minus sign for BosonHamiltonian.
    ///
    /// # Returns
    ///
    /// * `Self` - The BosonHamiltonian * -1.
    fn neg(self) -> Self {
        let mut internal = self.internal_map.clone();
        for key in self.keys() {
            internal.insert(key.clone(), internal[key].clone() * -1.0);
        }
        BosonHamiltonian {
            internal_map: internal,
        }
    }
}

/// Implements the plus function of BosonHamiltonian by BosonHamiltonian.
///
impl<T, V> ops::Add<T> for BosonHamiltonian
where
    T: IntoIterator<Item = (HermitianBosonProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `+` (add) for two BosonHamiltonians.
    ///
    /// # Arguments
    ///
    /// * `other` - The BosonHamiltonian to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two BosonHamiltonians added together.
    /// * `Err(StruqtureError::NonHermitianOperator)` - Key is naturally hermitian (on-diagonal term), but its corresponding value is not real.
    fn add(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value))?;
        }
        Ok(self)
    }
}

/// Implements the minus function of BosonHamiltonian by BosonHamiltonian.
///
impl<T, V> ops::Sub<T> for BosonHamiltonian
where
    T: IntoIterator<Item = (HermitianBosonProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `-` (subtract) for two BosonHamiltonians.
    ///
    /// # Arguments
    ///
    /// * `other` - The BosonHamiltonian to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two BosonHamiltonians added together.
    /// * `Err(StruqtureError::NonHermitianOperator)` - Key is naturally hermitian (on-diagonal term), but its corresponding value is not real.
    fn sub(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value) * -1.0)?;
        }
        Ok(self)
    }
}

/// Implements the multiplication function of BosonHamiltonian by CalculatorFloat.
///
impl ops::Mul<CalculatorFloat> for BosonHamiltonian {
    type Output = Self;
    /// Implement `*` for BosonHamiltonian and CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The BosonHamiltonian multiplied by the CalculatorFloat.
    fn mul(self, other: CalculatorFloat) -> Self {
        let mut internal = self.internal_map.clone();
        for key in self.keys() {
            internal.insert(key.clone(), internal[key].clone() * other.clone());
        }
        BosonHamiltonian {
            internal_map: internal,
        }
    }
}

/// Implements the multiplication function of BosonHamiltonian by CalculatorComplex.
///
impl ops::Mul<CalculatorComplex> for BosonHamiltonian {
    type Output = BosonOperator;
    /// Implement `*` for BosonHamiltonian and CalculatorComplex.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex by which to multiply.
    ///
    /// # Returns
    ///
    /// * `BosonOperator` - The BosonHamiltonian multiplied by the CalculatorComplex.
    ///
    /// # Panics
    ///
    /// * Internal bug in BosonProduct::new.
    /// * Internal bug in add_operator_product.
    fn mul(self, other: CalculatorComplex) -> BosonOperator {
        let mut new_out = BosonOperator::with_capacity(self.len());
        for (key, val) in self {
            if key.is_natural_hermitian() {
                let new_key =
                    BosonProduct::new(key.creators().copied(), key.annihilators().copied())
                        .expect("Internal bug in BosonProduct::new");
                new_out
                    .add_operator_product(new_key, other.clone() * val)
                    .expect("Internal bug in add_operator_product");
            } else {
                let new_key =
                    BosonProduct::new(key.creators().copied(), key.annihilators().copied())
                        .expect("Internal bug in BosonProduct::new");
                new_out
                    .add_operator_product(new_key, other.clone() * val.clone())
                    .expect("Internal bug in add_operator_product");
                let (key_tmp, prefactor) = key.hermitian_conjugate();
                let new_key =
                    BosonProduct::new(key_tmp.annihilators().copied(), key_tmp.creators().copied())
                        .expect("Internal bug in BosonProduct::new");
                new_out
                    .add_operator_product(new_key, other.clone() * val * prefactor)
                    .expect("Internal bug in add_operator_product");
            }
        }
        new_out
    }
}

/// Implements the multiplication function of BosonHamiltonian by BosonHamiltonian.
///
impl ops::Mul<BosonHamiltonian> for BosonHamiltonian {
    type Output = BosonOperator;
    /// Implement `*` for BosonHamiltonian and BosonHamiltonian.
    ///
    /// # Arguments
    ///
    /// * `other` - The BosonHamiltonian by which to multiply.
    ///
    /// # Returns
    ///
    /// * `BosonOperator` - The two BosonHamiltonians multiplied.
    ///
    /// # Panics
    ///
    /// * Internal bug in add_operator_product.
    fn mul(self, other: BosonHamiltonian) -> BosonOperator {
        let mut op = BosonOperator::with_capacity(self.len() * other.len());
        for (bps, vals) in self {
            for (bpo, valo) in other.iter() {
                let boson_products = bps.clone() * bpo.clone();
                let coefficient = Into::<CalculatorComplex>::into(valo) * vals.clone();
                for b in boson_products {
                    op.add_operator_product(b, coefficient.clone())
                        .expect("Internal bug in add_operator_product");
                }
            }
        }
        op
    }
}

/// Implements the into_iter function (IntoIterator trait) of BosonHamiltonian.
///
impl IntoIterator for BosonHamiltonian {
    type Item = (HermitianBosonProduct, CalculatorComplex);
    type IntoIter = std::collections::hash_map::IntoIter<HermitianBosonProduct, CalculatorComplex>;
    /// Returns the BosonHamiltonian in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The BosonHamiltonian in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference BosonHamiltonian.
///
impl<'a> IntoIterator for &'a BosonHamiltonian {
    type Item = (&'a HermitianBosonProduct, &'a CalculatorComplex);

    type IntoIter = Iter<'a, HermitianBosonProduct, CalculatorComplex>;

    /// Returns the reference BosonHamiltonian in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The BosonHamiltonian in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of BosonHamiltonian.
///
impl FromIterator<(HermitianBosonProduct, CalculatorComplex)> for BosonHamiltonian {
    /// Returns the object in BosonHamiltonian form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the BosonHamiltonian.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in BosonHamiltonian form.
    ///
    /// # Panics
    ///
    /// * Internal bug in add_operator_product.
    fn from_iter<I: IntoIterator<Item = (HermitianBosonProduct, CalculatorComplex)>>(
        iter: I,
    ) -> Self {
        let mut so = BosonHamiltonian::new();
        for (pp, cc) in iter {
            so.add_operator_product(pp, cc)
                .expect("Internal error in add_operator_product");
        }
        so
    }
}

/// Implements the extend function (Extend trait) of BosonHamiltonian.
///
impl Extend<(HermitianBosonProduct, CalculatorComplex)> for BosonHamiltonian {
    /// Extends the BosonHamiltonian by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the BosonHamiltonian.
    ///
    /// # Panics
    ///
    /// * Internal bug in add_operator_product.
    fn extend<I: IntoIterator<Item = (HermitianBosonProduct, CalculatorComplex)>>(
        &mut self,
        iter: I,
    ) {
        for (pp, cc) in iter {
            self.add_operator_product(pp, cc)
                .expect("Internal bug in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of BosonHamiltonian.
///
impl fmt::Display for BosonHamiltonian {
    /// Formats the BosonHamiltonian using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted BosonHamiltonian.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "BosonHamiltonian{\n".to_string();
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
    use serde_test::{assert_tokens, Configure, Token};

    // Test the Clone and PartialEq traits of SpinOperator
    #[test]
    fn so_from_sos() {
        let pp: HermitianBosonProduct = HermitianBosonProduct::new([0], [0]).unwrap();
        let sos = BosonHamiltonianSerialize {
            items: vec![(pp.clone(), 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        let mut so = BosonHamiltonian::new();
        so.set(pp, CalculatorComplex::from(0.5)).unwrap();

        assert_eq!(BosonHamiltonian::from(sos.clone()), so);
        assert_eq!(BosonHamiltonianSerialize::from(so), sos);
    }
    // Test the Clone and PartialEq traits of SpinOperator
    #[test]
    fn clone_partial_eq() {
        let pp: HermitianBosonProduct = HermitianBosonProduct::new([0], [0]).unwrap();
        let sos = BosonHamiltonianSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        // Test Clone trait
        assert_eq!(sos.clone(), sos);

        // Test PartialEq trait
        let pp_1: HermitianBosonProduct = HermitianBosonProduct::new([0], [0]).unwrap();
        let sos_1 = BosonHamiltonianSerialize {
            items: vec![(pp_1, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        let pp_2: HermitianBosonProduct = HermitianBosonProduct::new([0], [1]).unwrap();
        let sos_2 = BosonHamiltonianSerialize {
            items: vec![(pp_2, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        assert!(sos_1 == sos);
        assert!(sos == sos_1);
        assert!(sos_2 != sos);
        assert!(sos != sos_2);
    }

    // Test the Debug trait of SpinOperator
    #[test]
    fn debug() {
        let pp: HermitianBosonProduct = HermitianBosonProduct::new([0], [0]).unwrap();
        let sos = BosonHamiltonianSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        assert_eq!(
            format!("{:?}", sos),
            "BosonHamiltonianSerialize { items: [(HermitianBosonProduct { creators: [0], annihilators: [0] }, Float(0.5), Float(0.0))], _struqture_version: StruqtureVersionSerializable { major_version: 1, minor_version: 0 } }"
        );
    }

    /// Test SpinOperator Serialization and Deserialization traits (readable)
    #[test]
    fn serde_readable() {
        let pp: HermitianBosonProduct = HermitianBosonProduct::new([0], [0]).unwrap();
        let sos = BosonHamiltonianSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        assert_tokens(
            &sos.readable(),
            &[
                Token::Struct {
                    name: "BosonHamiltonianSerialize",
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
                Token::Str("_struqture_version"),
                Token::Struct {
                    name: "StruqtureVersionSerializable",
                    len: 2,
                },
                Token::Str("major_version"),
                Token::U32(1),
                Token::Str("minor_version"),
                Token::U32(0),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    /// Test SpinOperator Serialization and Deserialization traits (compact)
    #[test]
    fn serde_compact() {
        let pp: HermitianBosonProduct = HermitianBosonProduct::new([0], [0]).unwrap();
        let sos = BosonHamiltonianSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        assert_tokens(
            &sos.compact(),
            &[
                Token::Struct {
                    name: "BosonHamiltonianSerialize",
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
                Token::Str("_struqture_version"),
                Token::Struct {
                    name: "StruqtureVersionSerializable",
                    len: 2,
                },
                Token::Str("major_version"),
                Token::U32(1),
                Token::Str("minor_version"),
                Token::U32(0),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
