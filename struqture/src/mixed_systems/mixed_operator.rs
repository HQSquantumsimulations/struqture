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

use super::{MixedIndex, MixedProduct, OperateOnMixedSystems};
use crate::{
    ModeIndex, OperateOnDensityMatrix, OperateOnState, SpinIndex, StruqtureError,
    StruqtureVersionSerializable, MINIMUM_STRUQTURE_VERSION,
};
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Entry, Iter, Keys, Values};
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

/// MixedOperators are combinations of MixedProducts with specific CalculatorComplex coefficients.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::spins::PauliProduct;
/// use struqture::bosons::BosonProduct;
/// use struqture::fermions::FermionProduct;
/// use struqture::mixed_systems::{MixedProduct, MixedOperator};
///
/// let mut sh = MixedOperator::new(1, 1, 1);
///
/// let mp_1: MixedProduct = MixedProduct::new([PauliProduct::new().x(0),], [BosonProduct::new([0], [1]).unwrap()], [FermionProduct::new([0], [1]).unwrap()]).unwrap();
/// let mp_0: MixedProduct = MixedProduct::new([PauliProduct::new().z(0),], [BosonProduct::new([0], [1]).unwrap()], [FermionProduct::new([0], [1]).unwrap()]).unwrap();
/// sh.set(mp_1.clone(), CalculatorComplex::from(0.5)).unwrap();
/// sh.set(mp_0.clone(), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(sh.get(&mp_1), &CalculatorComplex::from(0.5));
/// assert_eq!(sh.get(&mp_0), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "MixedOperatorSerialize")]
#[serde(into = "MixedOperatorSerialize")]
pub struct MixedOperator {
    /// The internal HashMap of MixedProducts and coefficients (CalculatorComplex)
    internal_map: HashMap<MixedProduct, CalculatorComplex>,
    /// Number of Spin subsystems
    n_spins: usize,
    /// Number of Boson subsystems
    n_bosons: usize,
    /// Number of Fermion subsystems
    n_fermions: usize,
}

impl crate::MinSupportedVersion for MixedOperator {}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for MixedOperator {
    fn schema_name() -> String {
        "MixedOperator".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <MixedOperatorSerialize>::json_schema(gen)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
struct MixedOperatorSerialize {
    items: Vec<(MixedProduct, CalculatorFloat, CalculatorFloat)>,
    n_spins: usize,
    n_bosons: usize,
    n_fermions: usize,
    _struqture_version: StruqtureVersionSerializable,
}

impl From<MixedOperatorSerialize> for MixedOperator {
    fn from(value: MixedOperatorSerialize) -> Self {
        let mut new_noise_op = MixedOperator::new(value.n_spins, value.n_bosons, value.n_fermions);
        for (key, real, imag) in value.items.iter() {
            let _ =
                new_noise_op.add_operator_product(key.clone(), CalculatorComplex::new(real, imag));
        }
        new_noise_op
    }
}

impl From<MixedOperator> for MixedOperatorSerialize {
    fn from(value: MixedOperator) -> Self {
        let new_noise_op: Vec<(MixedProduct, CalculatorFloat, CalculatorFloat)> = value
            .clone()
            .into_iter()
            .map(|(key, val)| (key, val.re, val.im))
            .collect();
        let current_version = StruqtureVersionSerializable {
            major_version: MINIMUM_STRUQTURE_VERSION.0,
            minor_version: MINIMUM_STRUQTURE_VERSION.1,
        };
        Self {
            items: new_noise_op,
            n_spins: value.n_spins,
            n_bosons: value.n_bosons,
            n_fermions: value.n_fermions,
            _struqture_version: current_version,
        }
    }
}

impl<'a> OperateOnDensityMatrix<'a> for MixedOperator {
    type Index = MixedProduct;
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
    fn remove(&mut self, key: &MixedProduct) -> Option<CalculatorComplex> {
        self.internal_map.remove(key)
    }

    // From trait
    fn empty_clone(&self, capacity: Option<usize>) -> Self {
        match capacity {
            Some(cap) => Self::with_capacity(self.n_spins, self.n_bosons, self.n_fermions, cap),
            None => Self::new(self.n_spins, self.n_bosons, self.n_fermions),
        }
    }

    /// Overwrites an existing entry or sets a new entry in the MixedOperator with the given (MixedProduct key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The MixedProduct key to set in the MixedOperator.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the MixedOperator.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(CalculatorComplex))` - The key existed, this is the value it had before it was set with the value input.
    /// * `Ok(None)` - The key did not exist, it has been set with its corresponding value.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and key do not match.
    fn set(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<Option<Self::Value>, StruqtureError> {
        if key.spins().len() != self.n_spins
            || key.bosons().len() != self.n_bosons
            || key.fermions().len() != self.n_fermions
        {
            return Err(StruqtureError::MissmatchedNumberSubsystems {
                target_number_spin_subsystems: self.n_spins,
                target_number_boson_subsystems: self.n_bosons,
                target_number_fermion_subsystems: self.n_fermions,
                actual_number_spin_subsystems: key.spins().len(),
                actual_number_boson_subsystems: key.bosons().len(),
                actual_number_fermion_subsystems: key.fermions().len(),
            });
        }
        if value.re != CalculatorFloat::ZERO || value.im != CalculatorFloat::ZERO {
            // Catch on diagonals with non-zero imaginary values
            Ok(self.internal_map.insert(key, value))
        } else {
            match self.internal_map.entry(key) {
                Entry::Occupied(val) => Ok(Some(val.remove())),
                Entry::Vacant(_) => Ok(None),
            }
        }
    }
}

impl<'a> OperateOnState<'a> for MixedOperator {
    // From trait
    fn hermitian_conjugate(&self) -> Self {
        self.clone()
    }
}

impl<'a> OperateOnMixedSystems<'a> for MixedOperator {
    // From trait
    fn number_spins(&self) -> Vec<usize> {
        self.current_number_spins()
    }

    // From trait
    fn current_number_spins(&self) -> Vec<usize> {
        let mut number_spins: Vec<usize> = (0..self.n_spins).map(|_| 0).collect();
        for key in self.keys() {
            for (index, s) in key.spins().enumerate() {
                let maxk = s.current_number_spins();
                if maxk > number_spins[index] {
                    number_spins[index] = maxk
                }
            }
        }
        number_spins
    }

    // From trait
    fn number_bosonic_modes(&self) -> Vec<usize> {
        self.current_number_bosonic_modes()
    }

    // From trait
    fn current_number_bosonic_modes(&self) -> Vec<usize> {
        let mut number_bosons: Vec<usize> = (0..self.n_bosons).map(|_| 0).collect();
        for key in self.keys() {
            for (index, s) in key.bosons().enumerate() {
                let maxk = s.current_number_modes();
                if maxk > number_bosons[index] {
                    number_bosons[index] = maxk
                }
            }
        }
        number_bosons
    }

    // From trait
    fn number_fermionic_modes(&self) -> Vec<usize> {
        self.current_number_fermionic_modes()
    }

    // From trait
    fn current_number_fermionic_modes(&self) -> Vec<usize> {
        let mut number_fermions: Vec<usize> = (0..self.n_fermions).map(|_| 0).collect();
        for key in self.keys() {
            for (index, s) in key.fermions().enumerate() {
                let maxk = s.current_number_modes();
                if maxk > number_fermions[index] {
                    number_fermions[index] = maxk
                }
            }
        }
        number_fermions
    }
}

/// Implements the default function (Default trait) of MixedOperator (an empty MixedOperator).
///
impl Default for MixedOperator {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

/// Functions for the MixedOperator
///
impl MixedOperator {
    /// Creates a new MixedOperator.
    ///
    /// # Arguments:
    ///
    /// * `n_spins` - Number of spin sub-systems
    /// * `n_bosons` - Number of bosonic sub-systems
    /// * `n_fermions` - Number of fermionic sub-systems
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) MixedOperator.
    pub fn new(n_spins: usize, n_bosons: usize, n_fermions: usize) -> Self {
        MixedOperator {
            internal_map: HashMap::new(),
            n_spins,
            n_bosons,
            n_fermions,
        }
    }

    /// Creates a new MixedOperator with capacity.
    ///
    /// # Arguments
    ///
    /// * `n_spins` - The number of spin sub-systems.
    /// * `n_bosons` - The number of boson sub-systems.
    /// * `n_fermions` - The number of fermion sub-systems.
    /// * `capacity` - The pre-allocated capacity of the operator.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) MixedOperator.
    pub fn with_capacity(
        n_spins: usize,
        n_bosons: usize,
        n_fermions: usize,
        capacity: usize,
    ) -> Self {
        Self {
            internal_map: HashMap::with_capacity(capacity),
            n_spins,
            n_bosons,
            n_fermions,
        }
    }

    // /// Separate self into an operator with the terms of given number of spins, bosons and fermions and an operator with the remaining operations
    // ///
    // /// # Arguments
    // ///
    // /// * `number_particles` - Number of spins, bosons and fermions to filter for in the keys.
    // ///
    // /// # Returns
    // ///
    // /// `Ok((separated, remainder))` - Operator with the noise terms where number_particles matches the number of spins the operator product acts on and Operator with all other contributions.
    // pub fn separate_into_n_terms(
    //     &self,
    //     number_particles: (usize, usize, usize),
    // ) -> Result<(Self, Self), StruqtureError> {
    //     let mut separated = Self::default();
    //     let mut remainder = Self::default();
    //     for (prod, val) in self.iter() {
    //         if (
    //             prod.spins().len(),
    //             prod.bosons().len(),
    //             prod.fermions().len(),
    //         ) == number_particles
    //         {
    //             separated.add_operator_product(prod.clone(), val.clone())?;
    //         } else {
    //             remainder.add_operator_product(prod.clone(), val.clone())?;
    //         }
    //     }
    //     Ok((separated, remainder))
    // }
}

/// Implements the negative sign function of MixedOperator.
///
impl ops::Neg for MixedOperator {
    type Output = MixedOperator;
    /// Implement minus sign for MixedOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The MixedOperator * -1.
    fn neg(self) -> Self {
        let mut internal = self.internal_map.clone();
        let n_spins = self.n_spins;
        let n_bosons = self.n_bosons;
        let n_fermions = self.n_fermions;
        for key in self.keys() {
            internal.insert(key.clone(), internal[key].clone() * -1.0);
        }
        MixedOperator {
            internal_map: internal,
            n_spins,
            n_bosons,
            n_fermions,
        }
    }
}

/// Implements the plus function of MixedOperator by MixedOperator.
///
impl<T, V> ops::Add<T> for MixedOperator
where
    T: IntoIterator<Item = (MixedProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `+` (add) for two MixedOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The MixedOperator to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two MixedOperators added together.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and key do not match.
    fn add(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value))?;
        }
        Ok(self)
    }
}

/// Implements the minus function of MixedOperator by MixedOperator.
///
impl<T, V> ops::Sub<T> for MixedOperator
where
    T: IntoIterator<Item = (MixedProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `-` (subtract) for two MixedOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The MixedOperator to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two MixedOperators subtracted.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and key do not match.
    fn sub(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value) * -1.0)?;
        }
        Ok(self)
    }
}

/// Implements the multiplication function of MixedOperator by CalculatorComplex/CalculatorFloat.
///
impl<T> ops::Mul<T> for MixedOperator
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for MixedOperator and CalculatorComplex/CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex or CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The MixedOperator multiplied by the CalculatorComplex/CalculatorFloat.
    fn mul(self, other: T) -> Self {
        let other_cc = Into::<CalculatorComplex>::into(other);
        let mut internal = self.internal_map.clone();
        let n_spins = self.n_spins;
        let n_bosons = self.n_bosons;
        let n_fermions = self.n_fermions;
        for key in self.keys() {
            internal.insert(key.clone(), internal[key].clone() * other_cc.clone());
        }
        MixedOperator {
            internal_map: internal,
            n_spins,
            n_bosons,
            n_fermions,
        }
    }
}

/// Implements the multiplication function of MixedOperator by MixedOperator.
///
impl ops::Mul<MixedOperator> for MixedOperator {
    type Output = Result<MixedOperator, StruqtureError>;
    /// Implement `*` for MixedOperator and MixedOperator.
    ///
    /// # Arguments
    ///
    /// * `other` - The MixedOperator to multiply by.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two MixedOperators multiplied.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and key do not match.
    fn mul(self, other: MixedOperator) -> Self::Output {
        let mut op = MixedOperator::with_capacity(
            self.n_spins,
            self.n_bosons,
            self.n_fermions,
            self.len() * other.len(),
        );
        for (bps, vals) in self {
            for (bpo, valo) in other.iter() {
                let mixed_products = (bps.clone() * bpo.clone())?;
                let coefficient = Into::<CalculatorComplex>::into(valo) * vals.clone();
                for (b, coeff) in mixed_products {
                    op.add_operator_product(b, coefficient.clone() * coeff)?;
                }
            }
        }
        Ok(op)
    }
}

/// Implements the into_iter function (IntoIterator trait) of MixedOperator.
///
impl IntoIterator for MixedOperator {
    type Item = (MixedProduct, CalculatorComplex);
    type IntoIter = std::collections::hash_map::IntoIter<MixedProduct, CalculatorComplex>;
    /// Returns the MixedOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The MixedOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference MixedOperator.
///
impl<'a> IntoIterator for &'a MixedOperator {
    type Item = (&'a MixedProduct, &'a CalculatorComplex);
    type IntoIter = Iter<'a, MixedProduct, CalculatorComplex>;

    /// Returns the reference MixedOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference MixedOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of MixedOperator.
///
impl FromIterator<(MixedProduct, CalculatorComplex)> for MixedOperator {
    /// Returns the object in MixedOperator form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the MixedOperator.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in MixedOperator form.
    ///
    /// # Panics
    ///
    /// * Internal error in set.
    /// * Internal error in add_operator_product.
    fn from_iter<I: IntoIterator<Item = (MixedProduct, CalculatorComplex)>>(iter: I) -> Self {
        let mut iterator = iter.into_iter();
        match iterator.next() {
            Some(first_element) => {
                let spins = first_element.0.spins().len();
                let bosons = first_element.0.bosons().len();
                let fermions = first_element.0.fermions().len();
                let mut slno = MixedOperator::new(spins, bosons, fermions);
                slno.set(first_element.0, first_element.1)
                    .expect("Internal error in set");
                for (pair, cc) in iterator {
                    slno.add_operator_product(pair, cc)
                        .expect("Internal error in add_operator_product");
                }
                slno
            }
            None => MixedOperator::new(0, 0, 0),
        }
    }
}

/// Implements the extend function (Extend trait) of MixedOperator.
///
impl Extend<(MixedProduct, CalculatorComplex)> for MixedOperator {
    /// Extends the MixedOperator by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the MixedOperator.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn extend<I: IntoIterator<Item = (MixedProduct, CalculatorComplex)>>(&mut self, iter: I) {
        for (pp, cc) in iter {
            self.add_operator_product(pp, cc)
                .expect("Internal error in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of MixedOperator.
///
impl fmt::Display for MixedOperator {
    /// Formats the MixedOperator using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted MixedOperator.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "MixedOperator{\n".to_string();
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
    use crate::bosons::BosonProduct;
    use crate::fermions::FermionProduct;
    use crate::spins::PauliProduct;
    use serde_test::{assert_tokens, Configure, Token};

    // Test the Clone and PartialEq traits of SpinOperator
    #[test]
    fn so_from_sos() {
        let pp: MixedProduct = MixedProduct::new(
            [PauliProduct::new().z(2)],
            [BosonProduct::new([0], [3]).unwrap()],
            [FermionProduct::new([0], [2]).unwrap()],
        )
        .unwrap();
        let sos = MixedOperatorSerialize {
            items: vec![(pp.clone(), 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        let mut so = MixedOperator::new(1, 1, 1);
        so.set(pp, CalculatorComplex::from(0.5)).unwrap();

        assert_eq!(MixedOperator::from(sos.clone()), so);
        assert_eq!(MixedOperatorSerialize::from(so), sos);
    }
    // Test the Clone and PartialEq traits of SpinOperator
    #[test]
    fn clone_partial_eq() {
        let pp: MixedProduct = MixedProduct::new(
            [PauliProduct::new().z(2)],
            [BosonProduct::new([0], [3]).unwrap()],
            [FermionProduct::new([0], [2]).unwrap()],
        )
        .unwrap();
        let sos = MixedOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        // Test Clone trait
        assert_eq!(sos.clone(), sos);

        // Test PartialEq trait
        let pp_1: MixedProduct = MixedProduct::new(
            [PauliProduct::new().z(2)],
            [BosonProduct::new([0], [3]).unwrap()],
            [FermionProduct::new([0], [2]).unwrap()],
        )
        .unwrap();
        let sos_1 = MixedOperatorSerialize {
            items: vec![(pp_1, 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        let pp_2: MixedProduct = MixedProduct::new(
            [PauliProduct::new().z(0)],
            [BosonProduct::new([0], [3]).unwrap()],
            [FermionProduct::new([0], [2]).unwrap()],
        )
        .unwrap();
        let sos_2 = MixedOperatorSerialize {
            items: vec![(pp_2, 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
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
        let pp: MixedProduct = MixedProduct::new(
            [PauliProduct::new().z(2)],
            [BosonProduct::new([0], [3]).unwrap()],
            [FermionProduct::new([0], [2]).unwrap()],
        )
        .unwrap();
        let sos = MixedOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        assert_eq!(
            format!("{:?}", sos),
            "MixedOperatorSerialize { items: [(MixedProduct { spins: [PauliProduct { items: [(2, Z)] }], bosons: [BosonProduct { creators: [0], annihilators: [3] }], fermions: [FermionProduct { creators: [0], annihilators: [2] }] }, Float(0.5), Float(0.0))], n_spins: 1, n_bosons: 1, n_fermions: 1, _struqture_version: StruqtureVersionSerializable { major_version: 1, minor_version: 0 } }"
        );
    }

    /// Test SpinOperator Serialization and Deserialization traits (readable)
    #[test]
    fn serde_readable() {
        let pp: MixedProduct = MixedProduct::new(
            [PauliProduct::new().z(2)],
            [BosonProduct::new([0], [3]).unwrap()],
            [FermionProduct::new([0], [2]).unwrap()],
        )
        .unwrap();
        let sos = MixedOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        assert_tokens(
            &sos.readable(),
            &[
                Token::Struct {
                    name: "MixedOperatorSerialize",
                    len: 5,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 3 },
                Token::Str("S2Z:Bc0a3:Fc0a2:"),
                Token::F64(0.5),
                Token::F64(0.0),
                Token::TupleEnd,
                Token::SeqEnd,
                Token::Str("n_spins"),
                Token::U64(1),
                Token::Str("n_bosons"),
                Token::U64(1),
                Token::Str("n_fermions"),
                Token::U64(1),
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
        let pp: MixedProduct = MixedProduct::new(
            [PauliProduct::new().z(2)],
            [BosonProduct::new([0], [3]).unwrap()],
            [FermionProduct::new([0], [2]).unwrap()],
        )
        .unwrap();
        let sos = MixedOperatorSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };

        assert_tokens(
            &sos.compact(),
            &[
                Token::Struct {
                    name: "MixedOperatorSerialize",
                    len: 5,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 3 },
                Token::Tuple { len: 3 },
                Token::Seq { len: Some(1) },
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 2 },
                Token::U64(2),
                Token::UnitVariant {
                    name: "SingleSpinOperator",
                    variant: "Z",
                },
                Token::TupleEnd,
                Token::SeqEnd,
                Token::SeqEnd,
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 2 },
                Token::Seq { len: Some(1) },
                Token::U64(0),
                Token::SeqEnd,
                Token::Seq { len: Some(1) },
                Token::U64(3),
                Token::SeqEnd,
                Token::TupleEnd,
                Token::SeqEnd,
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 2 },
                Token::Seq { len: Some(1) },
                Token::U64(0),
                Token::SeqEnd,
                Token::Seq { len: Some(1) },
                Token::U64(2),
                Token::SeqEnd,
                Token::TupleEnd,
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
                Token::Str("n_spins"),
                Token::U64(1),
                Token::Str("n_bosons"),
                Token::U64(1),
                Token::Str("n_fermions"),
                Token::U64(1),
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
