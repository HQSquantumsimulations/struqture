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

use super::{HermitianMixedProduct, MixedIndex, MixedOperator, OperateOnMixedSystems};
use crate::{
    ModeIndex, OperateOnDensityMatrix, OperateOnState, SpinIndex, StruqtureError,
    StruqtureVersionSerializable, SymmetricIndex, MINIMUM_STRUQTURE_VERSION,
};
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Entry, Iter, Keys, Values};
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

/// MixedHamiltonians are combinations of HermitianMixedProducts with specific CalculatorComplex coefficients.
///
/// This is a representation of sums of Pauli products with weightings in order to build a full Hamiltonian.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::mixed_systems::{HermitianMixedProduct, MixedHamiltonian};
/// use struqture::spins::PauliProduct;
/// use struqture::bosons::BosonProduct;
/// use struqture::fermions::FermionProduct;
///
/// let mut sh = MixedHamiltonian::new(1, 1, 1);
///
/// let pp_1: HermitianMixedProduct = HermitianMixedProduct::new([PauliProduct::new().x(0),], [BosonProduct::new([], [1]).unwrap()], [FermionProduct::new([0], [1]).unwrap()]).unwrap();
/// let pp_0: HermitianMixedProduct = HermitianMixedProduct::new([PauliProduct::new().z(0),], [BosonProduct::new([0], [1]).unwrap()], [FermionProduct::new([0], [0]).unwrap()]).unwrap();
/// sh.set(pp_1.clone(), CalculatorComplex::from(0.5)).unwrap();
/// sh.set(pp_0.clone(), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(sh.get(&pp_1), &CalculatorComplex::from(0.5));
/// assert_eq!(sh.get(&pp_0), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "MixedHamiltonianSerialize")]
#[serde(into = "MixedHamiltonianSerialize")]
pub struct MixedHamiltonian {
    /// The internal HashMap of HermitianMixedProducts and coefficients (CalculatorFloat)
    internal_map: HashMap<HermitianMixedProduct, CalculatorComplex>,
    /// Number of Spin subsystems
    n_spins: usize,
    /// Number of Boson subsystems
    n_bosons: usize,
    /// Number of Fermion subsystems
    n_fermions: usize,
}

impl crate::MinSupportedVersion for MixedHamiltonian {}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for MixedHamiltonian {
    fn schema_name() -> String {
        "MixedHamiltonian".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <MixedHamiltonianSerialize>::json_schema(gen)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
struct MixedHamiltonianSerialize {
    items: Vec<(HermitianMixedProduct, CalculatorFloat, CalculatorFloat)>,
    n_spins: usize,
    n_bosons: usize,
    n_fermions: usize,
    _struqture_version: StruqtureVersionSerializable,
}

impl From<MixedHamiltonianSerialize> for MixedHamiltonian {
    fn from(value: MixedHamiltonianSerialize) -> Self {
        let mut new_noise_op =
            MixedHamiltonian::new(value.n_spins, value.n_bosons, value.n_fermions);
        for (key, real, imag) in value.items.iter() {
            let _ =
                new_noise_op.add_operator_product(key.clone(), CalculatorComplex::new(real, imag));
        }
        new_noise_op
    }
}

impl From<MixedHamiltonian> for MixedHamiltonianSerialize {
    fn from(value: MixedHamiltonian) -> Self {
        let new_noise_op: Vec<(HermitianMixedProduct, CalculatorFloat, CalculatorFloat)> = value
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

impl<'a> OperateOnDensityMatrix<'a> for MixedHamiltonian {
    type Index = HermitianMixedProduct;
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
    fn remove(&mut self, key: &HermitianMixedProduct) -> Option<CalculatorComplex> {
        self.internal_map.remove(key)
    }

    // From trait
    fn empty_clone(&self, capacity: Option<usize>) -> Self {
        match capacity {
            Some(cap) => Self::with_capacity(self.n_spins, self.n_bosons, self.n_fermions, cap),
            None => Self::new(self.n_spins, self.n_bosons, self.n_fermions),
        }
    }

    /// Overwrites an existing entry or sets a new entry in the MixedHamiltonian with the given (HermitianMixedProduct key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The HermitianMixedProduct key to set in the MixedHamiltonian.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the MixedHamiltonian.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(CalculatorComplex))` - The key existed, this is the value it had before it was set with the value input.
    /// * `Ok(None)` - The key did not exist, it has been set with its corresponding value.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and key do not match.
    /// * `Err(StruqtureError::NonHermitianOperator)` - Key is naturally hermitian (on-diagonal term), but its corresponding value is not real.
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
}

impl<'a> OperateOnState<'a> for MixedHamiltonian {
    // From trait
    fn hermitian_conjugate(&self) -> Self {
        self.clone()
    }
}

impl<'a> OperateOnMixedSystems<'a> for MixedHamiltonian {
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

/// Implements the default function (Default trait) of MixedHamiltonian (an empty MixedHamiltonian).
///
impl Default for MixedHamiltonian {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

/// Functions for the MixedHamiltonian
///
impl MixedHamiltonian {
    /// Creates a new MixedHamiltonian.
    ///
    /// # Arguments:
    ///
    /// * `n_spins` - Number of spin sub-systems
    /// * `n_bosons` - Number of bosonic sub-systems
    /// * `n_fermions` - Number of fermionic sub-systems
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) MixedHamiltonian.
    pub fn new(n_spins: usize, n_bosons: usize, n_fermions: usize) -> Self {
        MixedHamiltonian {
            internal_map: HashMap::new(),
            n_spins,
            n_bosons,
            n_fermions,
        }
    }

    /// Creates a new MixedHamiltonian with capacity.
    ///
    /// # Arguments
    ///
    /// * `n_spins` - The number of spin sub-systems.
    /// * `n_bosons` - The number of boson sub-systems.
    /// * `n_fermions` - The number of fermion sub-systems.
    /// * `capacity` - The pre-allocated capacity of the hamiltonian.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) MixedHamiltonian.
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

/// Implements the negative sign function of MixedHamiltonian.
///
impl ops::Neg for MixedHamiltonian {
    type Output = MixedHamiltonian;
    /// Implement minus sign for MixedHamiltonian.
    ///
    /// # Returns
    ///
    /// * `Self` - The MixedHamiltonian * -1.
    fn neg(self) -> Self {
        let mut internal = self.internal_map.clone();
        let n_spins = self.n_spins;
        let n_bosons = self.n_bosons;
        let n_fermions = self.n_fermions;
        for key in self.keys() {
            internal.insert(key.clone(), internal[key].clone() * -1.0);
        }
        MixedHamiltonian {
            internal_map: internal,
            n_spins,
            n_bosons,
            n_fermions,
        }
    }
}

/// Implements the plus function of MixedHamiltonian by MixedHamiltonian.
///
impl<T, V> ops::Add<T> for MixedHamiltonian
where
    T: IntoIterator<Item = (HermitianMixedProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `+` (add) for two MixedHamiltonians.
    ///
    /// # Arguments
    ///
    /// * `other` - The MixedHamiltonian to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two MixedHamiltonians added together.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and key do not match.
    /// * `Err(StruqtureError::NonHermitianOperator)` - Key is naturally hermitian (on-diagonal term), but its corresponding value is not real.
    fn add(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value))?;
        }
        Ok(self)
    }
}

/// Implements the minus function of MixedHamiltonian by MixedHamiltonian.
///
impl<T, V> ops::Sub<T> for MixedHamiltonian
where
    T: IntoIterator<Item = (HermitianMixedProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `-` (subtract) for two MixedHamiltonians.
    ///
    /// # Arguments
    ///
    /// * `other` - The MixedHamiltonian to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two MixedHamiltonians subtracted.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and key do not match.
    /// * `Err(StruqtureError::NonHermitianOperator)` - Key is naturally hermitian (on-diagonal term), but its corresponding value is not real.
    fn sub(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value) * -1.0)?;
        }
        Ok(self)
    }
}

/// Implements the multiplication function of MixedHamiltonian by CalculatorComplex/CalculatorFloat.
///
impl<T> ops::Mul<T> for MixedHamiltonian
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for MixedHamiltonian and CalculatorComplex/CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex or CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The MixedHamiltonian multiplied by the CalculatorComplex/CalculatorFloat.
    fn mul(self, other: T) -> Self {
        let other_cc = Into::<CalculatorComplex>::into(other);
        let mut internal = self.internal_map.clone();
        let n_spins = self.n_spins;
        let n_bosons = self.n_bosons;
        let n_fermions = self.n_fermions;
        for key in self.keys() {
            internal.insert(key.clone(), internal[key].clone() * other_cc.clone());
        }
        MixedHamiltonian {
            internal_map: internal,
            n_spins,
            n_bosons,
            n_fermions,
        }
    }
}

/// Implements the multiplication function of MixedHamiltonian by MixedHamiltonian.
///
impl ops::Mul<MixedHamiltonian> for MixedHamiltonian {
    type Output = Result<MixedOperator, StruqtureError>;
    /// Implement `*` for MixedHamiltonian and MixedHamiltonian.
    ///
    /// # Arguments
    ///
    /// * `other` - The MixedHamiltonian to multiply by.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two MixedHamiltonians multiplied.
    /// * `Err(StruqtureError::MissmatchedNumberSubsystems)` - Number of subsystems in system and key do not match.
    /// * `Err(StruqtureError::NonHermitianOperator)` - Key is naturally hermitian (on-diagonal term), but its corresponding value is not real.
    fn mul(self, other: MixedHamiltonian) -> Self::Output {
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

/// Implements the into_iter function (IntoIterator trait) of MixedHamiltonian.
///
impl IntoIterator for MixedHamiltonian {
    type Item = (HermitianMixedProduct, CalculatorComplex);
    type IntoIter = std::collections::hash_map::IntoIter<HermitianMixedProduct, CalculatorComplex>;
    /// Returns the MixedHamiltonian in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The MixedHamiltonian in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference MixedHamiltonian.
///
impl<'a> IntoIterator for &'a MixedHamiltonian {
    type Item = (&'a HermitianMixedProduct, &'a CalculatorComplex);
    type IntoIter = Iter<'a, HermitianMixedProduct, CalculatorComplex>;

    /// Returns the reference MixedHamiltonian in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference MixedHamiltonian in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of MixedHamiltonian.
///
impl FromIterator<(HermitianMixedProduct, CalculatorComplex)> for MixedHamiltonian {
    /// Returns the object in MixedHamiltonian form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the MixedHamiltonian.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in MixedHamiltonian form.
    ///
    /// # Panics
    ///
    /// * Internal bug in set.
    /// * Internal bug in add_operator_product.
    fn from_iter<I: IntoIterator<Item = (HermitianMixedProduct, CalculatorComplex)>>(
        iter: I,
    ) -> Self {
        let mut iterator = iter.into_iter();
        match iterator.next() {
            Some(first_element) => {
                let spins = first_element.0.spins().len();
                let bosons = first_element.0.bosons().len();
                let fermions = first_element.0.fermions().len();
                let mut mh = MixedHamiltonian::new(spins, bosons, fermions);
                mh.set(first_element.0, first_element.1)
                    .expect("Internal error in set");
                for (pair, cc) in iterator {
                    mh.add_operator_product(pair, cc)
                        .expect("Internal error in add_operator_product");
                }
                mh
            }
            None => MixedHamiltonian::new(0, 0, 0),
        }
    }
}

/// Implements the extend function (Extend trait) of MixedHamiltonian.
///
impl Extend<(HermitianMixedProduct, CalculatorComplex)> for MixedHamiltonian {
    /// Extends the MixedHamiltonian by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the MixedHamiltonian.
    ///
    /// # Panics
    ///
    /// * Internal bug in add_operator_product.
    fn extend<I: IntoIterator<Item = (HermitianMixedProduct, CalculatorComplex)>>(
        &mut self,
        iter: I,
    ) {
        for (pp, cc) in iter {
            self.add_operator_product(pp, cc)
                .expect("Internal error in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of MixedHamiltonian.
///
impl fmt::Display for MixedHamiltonian {
    /// Formats the MixedHamiltonian using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted MixedHamiltonian.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "MixedHamiltonian{\n".to_string();
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
        let pp: HermitianMixedProduct = HermitianMixedProduct::new(
            [PauliProduct::new().z(2)],
            [BosonProduct::new([0], [3]).unwrap()],
            [FermionProduct::new([0], [2]).unwrap()],
        )
        .unwrap();
        let sos = MixedHamiltonianSerialize {
            items: vec![(pp.clone(), 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        let mut so = MixedHamiltonian::new(1, 1, 1);
        so.set(pp, CalculatorComplex::from(0.5)).unwrap();

        assert_eq!(MixedHamiltonian::from(sos.clone()), so);
        assert_eq!(MixedHamiltonianSerialize::from(so), sos);
    }
    // Test the Clone and PartialEq traits of SpinOperator
    #[test]
    fn clone_partial_eq() {
        let pp: HermitianMixedProduct = HermitianMixedProduct::new(
            [PauliProduct::new().z(2)],
            [BosonProduct::new([0], [3]).unwrap()],
            [FermionProduct::new([0], [2]).unwrap()],
        )
        .unwrap();
        let sos = MixedHamiltonianSerialize {
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
        let pp_1: HermitianMixedProduct = HermitianMixedProduct::new(
            [PauliProduct::new().z(2)],
            [BosonProduct::new([0], [3]).unwrap()],
            [FermionProduct::new([0], [2]).unwrap()],
        )
        .unwrap();
        let sos_1 = MixedHamiltonianSerialize {
            items: vec![(pp_1, 0.5.into(), 0.0.into())],
            n_spins: 1,
            n_bosons: 1,
            n_fermions: 1,
            _struqture_version: StruqtureVersionSerializable {
                major_version: 1,
                minor_version: 0,
            },
        };
        let pp_2: HermitianMixedProduct = HermitianMixedProduct::new(
            [PauliProduct::new().z(0)],
            [BosonProduct::new([0], [3]).unwrap()],
            [FermionProduct::new([0], [2]).unwrap()],
        )
        .unwrap();
        let sos_2 = MixedHamiltonianSerialize {
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
        let pp: HermitianMixedProduct = HermitianMixedProduct::new(
            [PauliProduct::new().z(2)],
            [BosonProduct::new([0], [3]).unwrap()],
            [FermionProduct::new([0], [2]).unwrap()],
        )
        .unwrap();
        let sos = MixedHamiltonianSerialize {
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
            "MixedHamiltonianSerialize { items: [(HermitianMixedProduct { spins: [PauliProduct { items: [(2, Z)] }], bosons: [BosonProduct { creators: [0], annihilators: [3] }], fermions: [FermionProduct { creators: [0], annihilators: [2] }] }, Float(0.5), Float(0.0))], n_spins: 1, n_bosons: 1, n_fermions: 1, _struqture_version: StruqtureVersionSerializable { major_version: 1, minor_version: 0 } }"
        );
    }

    /// Test SpinOperator Serialization and Deserialization traits (readable)
    #[test]
    fn serde_readable() {
        let pp: HermitianMixedProduct = HermitianMixedProduct::new(
            [PauliProduct::new().z(2)],
            [BosonProduct::new([0], [3]).unwrap()],
            [FermionProduct::new([0], [2]).unwrap()],
        )
        .unwrap();
        let sos = MixedHamiltonianSerialize {
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
                    name: "MixedHamiltonianSerialize",
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
        let pp: HermitianMixedProduct = HermitianMixedProduct::new(
            [PauliProduct::new().z(2)],
            [BosonProduct::new([0], [3]).unwrap()],
            [FermionProduct::new([0], [2]).unwrap()],
        )
        .unwrap();
        let sos = MixedHamiltonianSerialize {
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
                    name: "MixedHamiltonianSerialize",
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
