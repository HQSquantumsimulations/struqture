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

use super::{
    FermionOperator, FermionProduct, HermitianFermionProduct, ModeIndex, OperateOnFermions,
};
use crate::mappings::JordanWignerFermionToSpin;
use crate::spins::QubitHamiltonian;
use crate::{
    GetValue, OperateOnDensityMatrix, OperateOnModes, OperateOnState, StruqtureError,
    SymmetricIndex,
};
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

use indexmap::map::{Entry, Iter};
use indexmap::IndexMap;

/// FermionHamiltonians are combinations of FermionProducts with specific CalculatorComplex coefficients.
///
/// This is a representation of sums of creation and annihilation operators with weightings, in order to build a full hamiltonian.
/// FermionHamiltonian is the hermitian equivalent of FermionOperator.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::fermions::{ HermitianFermionProduct, FermionHamiltonian};
/// use struqture::prelude::*;
///
/// let mut fh = FermionHamiltonian::new();
///
/// let fp_0_1 = HermitianFermionProduct::new([0], [1]).unwrap();
/// let fp_0 = HermitianFermionProduct::new([], [0]).unwrap();
/// fh.set(fp_0_1.clone(), CalculatorComplex::from(0.5)).unwrap();
/// fh.set(fp_0.clone(), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(fh.get(&fp_0_1), &CalculatorComplex::from(0.5));
/// assert_eq!(fh.get(&fp_0), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "FermionHamiltonianSerialize")]
#[serde(into = "FermionHamiltonianSerialize")]
pub struct FermionHamiltonian {
    /// The internal HashMap of FermionProducts and coefficients (CalculatorComplex)
    internal_map: IndexMap<HermitianFermionProduct, CalculatorComplex>,
}

impl crate::SerializationSupport for FermionHamiltonian {
    fn struqture_type() -> crate::StruqtureType {
        crate::StruqtureType::FermionHamiltonian
    }
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for FermionHamiltonian {
    fn schema_name() -> String {
        "FermionHamiltonian".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <FermionHamiltonianSerialize>::json_schema(gen)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
struct FermionHamiltonianSerialize {
    items: Vec<(HermitianFermionProduct, CalculatorFloat, CalculatorFloat)>,
    serialisation_meta: crate::StruqtureSerialisationMeta,
}

impl TryFrom<FermionHamiltonianSerialize> for FermionHamiltonian {
    type Error = StruqtureError;
    fn try_from(value: FermionHamiltonianSerialize) -> Result<Self, Self::Error> {
        let target_serialisation_meta =
            <Self as crate::SerializationSupport>::target_serialisation_meta();
        crate::check_can_be_deserialised(&target_serialisation_meta, &value.serialisation_meta)?;
        let new_noise_op: FermionHamiltonian = value
            .items
            .into_iter()
            .map(|(key, real, imag)| (key, CalculatorComplex { re: real, im: imag }))
            .collect();
        Ok(new_noise_op)
    }
}

impl From<FermionHamiltonian> for FermionHamiltonianSerialize {
    fn from(value: FermionHamiltonian) -> Self {
        let serialisation_meta = crate::SerializationSupport::struqture_serialisation_meta(&value);
        let new_noise_op: Vec<(HermitianFermionProduct, CalculatorFloat, CalculatorFloat)> = value
            .into_iter()
            .map(|(key, val)| (key, val.re, val.im))
            .collect();
        Self {
            items: new_noise_op,
            serialisation_meta,
        }
    }
}

impl<'a> OperateOnDensityMatrix<'a> for FermionHamiltonian {
    type Index = HermitianFermionProduct;
    type Value = CalculatorComplex;

    // From trait
    fn get(&self, key: &HermitianFermionProduct) -> &CalculatorComplex {
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

    /// Overwrites an existing entry or sets a new entry in the FermionHamiltonian with the given (HermitianFermionProduct key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The HermitianFermionProduct key to set in the FermionHamiltonian.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the FermionHamiltonian.
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
                Entry::Occupied(val) => Ok(Some(val.shift_remove())),
                Entry::Vacant(_) => Ok(None),
            }
        }
    }

    /// Adds a new (HermitianFermionProduct key, CalculatorComplex value) pair to the FermionHamiltonian.
    ///
    /// # Arguments
    ///
    /// * `key` - The HermitianFermionProduct key to added to the FermionHamiltonian.
    /// * `value` - The corresponding CalculatorComplex value to add for the key in the FermionHamiltonian.
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

impl<'a> OperateOnState<'a> for FermionHamiltonian {
    /// Returns the hermitian conjugate of the FermionHamiltonian.
    ///
    /// # Returns
    ///
    /// * `Self` - The hermitian conjugate of Self.
    fn hermitian_conjugate(&self) -> Self {
        self.clone()
    }
}

impl<'a> OperateOnModes<'a> for FermionHamiltonian {
    /// Gets the maximum index of the FermionHamiltonian.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of modes in the FermionHamiltonian.
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
}

impl<'a> OperateOnFermions<'a> for FermionHamiltonian {}

/// Implements the default function (Default trait) of FermionHamiltonian (an empty FermionHamiltonian).
///
impl Default for FermionHamiltonian {
    fn default() -> Self {
        Self::new()
    }
}

/// Functions for the FermionHamiltonian
///
impl FermionHamiltonian {
    /// Creates a new FermionHamiltonian.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) FermionHamiltonian.
    pub fn new() -> Self {
        FermionHamiltonian {
            internal_map: IndexMap::new(),
        }
    }

    /// Creates a new FermionHamiltonian with pre-allocated capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The pre-allocated capacity of the hamiltonian.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) FermionHamiltonian.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            internal_map: IndexMap::with_capacity(capacity),
        }
    }

    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_export")]
    pub fn to_struqture_1(
        &self,
    ) -> Result<struqture_1::fermions::FermionHamiltonianSystem, StruqtureError> {
        let mut new_fermion_system = struqture_1::fermions::FermionHamiltonianSystem::new(None);
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
        value: &struqture_1::fermions::FermionHamiltonianSystem,
    ) -> Result<Self, StruqtureError> {
        let mut new_qubit_operator = Self::new();
        for (key, val) in struqture_1::OperateOnDensityMatrix::iter(value) {
            let self_key = HermitianFermionProduct::from_struqture_1(key)?;
            let _ = new_qubit_operator.set(self_key, val.clone());
        }
        Ok(new_qubit_operator)
    }
}

impl TryFrom<FermionOperator> for FermionHamiltonian {
    type Error = StruqtureError;
    /// Tries to convert a FermionOperator into a FermionHamiltonian.
    ///
    /// # Arguments
    ///
    /// * `hamiltonian` - The FermionOperator to try to convert.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The FermionOperator converted into a FermionHamiltonian.
    /// * `Err(StruqtureError::CreatorsAnnihilatorsMinimumIndex)` - The minimum index of the creators is larger than the minimum index of the annihilators.
    /// * `Err(StruqtureError::NonHermitianOperator)` - Key is naturally hermitian (on-diagonal term), but its corresponding value is not real.
    fn try_from(hamiltonian: FermionOperator) -> Result<Self, StruqtureError> {
        let mut internal = FermionHamiltonian::new();
        for (key, value) in hamiltonian.into_iter() {
            if key.creators().min() > key.annihilators().min() {
                return Err(StruqtureError::CreatorsAnnihilatorsMinimumIndex {
                    creators_min: key.creators().min().cloned(),
                    annihilators_min: key.annihilators().min().cloned(),
                });
            } else {
                let bp = HermitianFermionProduct::get_key(&key);
                internal.add_operator_product(bp, value)?;
            }
        }
        Ok(internal)
    }
}

/// Implements the negative sign function of FermionHamiltonian.
///
impl ops::Neg for FermionHamiltonian {
    type Output = FermionHamiltonian;
    /// Implement minus sign for FermionHamiltonian.
    ///
    /// # Returns
    ///
    /// * `Self` - The FermionHamiltonian * -1.
    fn neg(self) -> Self {
        let mut internal = self.internal_map.clone();
        for key in self.keys() {
            internal.insert(key.clone(), internal[key].clone() * -1.0);
        }
        FermionHamiltonian {
            internal_map: internal,
        }
    }
}

/// Implements the plus function of FermionHamiltonian by FermionHamiltonian.
///
impl<T, V> ops::Add<T> for FermionHamiltonian
where
    T: IntoIterator<Item = (HermitianFermionProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `+` (add) for two FermionHamiltonians.
    ///
    /// # Arguments
    ///
    /// * `other` - The FermionHamiltonian to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two FermionHamiltonians added together.
    /// * `Err(StruqtureError::NonHermitianOperator)` - Key is naturally hermitian (on-diagonal term), but its corresponding value is not real.
    fn add(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value))?;
        }
        Ok(self)
    }
}

/// Implements the minus function of FermionHamiltonian by FermionHamiltonian.
///
impl<T, V> ops::Sub<T> for FermionHamiltonian
where
    T: IntoIterator<Item = (HermitianFermionProduct, V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Result<Self, StruqtureError>;
    /// Implements `-` (subtract) for two FermionHamiltonians.
    ///
    /// # Arguments
    ///
    /// * `other` - The FermionHamiltonian to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The two FermionHamiltonians subtracted.
    /// * `Err(StruqtureError::NonHermitianOperator)` - Key is naturally hermitian (on-diagonal term), but its corresponding value is not real.
    fn sub(mut self, other: T) -> Self::Output {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value) * -1.0)?;
        }
        Ok(self)
    }
}

/// Implements the multiplication function of FermionHamiltonian by CalculatorFloat.
///
impl ops::Mul<CalculatorFloat> for FermionHamiltonian {
    type Output = Self;
    /// Implement `*` for FermionHamiltonian and CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The FermionHamiltonian multiplied by the CalculatorFloat.
    fn mul(self, other: CalculatorFloat) -> Self {
        let mut internal = self.internal_map.clone();
        for key in self.keys() {
            internal.insert(key.clone(), internal[key].clone() * other.clone());
        }
        FermionHamiltonian {
            internal_map: internal,
        }
    }
}

/// Implements the multiplication function of FermionHamiltonian by CalculatorComplex.
///
impl ops::Mul<CalculatorComplex> for FermionHamiltonian {
    type Output = Result<FermionOperator, StruqtureError>;
    /// Implement `*` for FermionHamiltonian and CalculatorComplex.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Ok(FermionOperator)` - The FermionHamiltonian multiplied by the CalculatorComplex.
    fn mul(self, other: CalculatorComplex) -> Self::Output {
        let mut new_out = FermionOperator::with_capacity(self.len());
        for (key, val) in self {
            if key.is_natural_hermitian() {
                let new_key =
                    FermionProduct::new(key.creators().copied(), key.annihilators().copied())?;
                new_out.set(new_key, other.clone() * val)?;
            } else {
                let new_key =
                    FermionProduct::new(key.creators().copied(), key.annihilators().copied())?;
                new_out.add_operator_product(new_key, other.clone() * val.clone())?;
                let (key_tmp, prefactor) = key.hermitian_conjugate();
                let new_key = FermionProduct::new(
                    key_tmp.annihilators().copied(),
                    key_tmp.creators().copied(),
                )?;
                new_out.add_operator_product(new_key, other.clone() * val * prefactor)?;
            }
        }
        Ok(new_out)
    }
}

/// Implements the multiplication function of FermionHamiltonian by FermionHamiltonian.
///
impl ops::Mul<FermionHamiltonian> for FermionHamiltonian {
    type Output = Result<FermionOperator, StruqtureError>;
    /// Implement `*` for FermionHamiltonian and FermionHamiltonian.
    ///
    /// # Arguments
    ///
    /// * `other` - The FermionHamiltonian to multiply by.
    ///
    /// # Returns
    ///
    /// * `Ok(FermionOperator)` - The two FermionHamiltonians multiplied.
    fn mul(self, other: FermionHamiltonian) -> Self::Output {
        let mut op = FermionOperator::with_capacity(self.len() * other.len());
        for (bps, vals) in self {
            for (bpo, valo) in other.iter() {
                let fermion_products = bps.clone() * bpo.clone();
                let coefficient = Into::<CalculatorComplex>::into(valo) * vals.clone();
                for (prod, coeff) in fermion_products {
                    op.add_operator_product(prod, coefficient.clone() * coeff)?;
                }
            }
        }
        Ok(op)
    }
}

/// Implements the into_iter function (IntoIterator trait) of FermionHamiltonian.
///
impl IntoIterator for FermionHamiltonian {
    type Item = (HermitianFermionProduct, CalculatorComplex);
    type IntoIter = indexmap::map::IntoIter<HermitianFermionProduct, CalculatorComplex>;
    /// Returns the FermionHamiltonian in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The FermionHamiltonian in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference FermionHamiltonian.
///
impl<'a> IntoIterator for &'a FermionHamiltonian {
    type Item = (&'a HermitianFermionProduct, &'a CalculatorComplex);
    type IntoIter = Iter<'a, HermitianFermionProduct, CalculatorComplex>;

    /// Returns the reference FermionHamiltonian in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The FermionHamiltonian in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of FermionHamiltonian.
///
impl FromIterator<(HermitianFermionProduct, CalculatorComplex)> for FermionHamiltonian {
    /// Returns the object in FermionHamiltonian form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the FermionHamiltonian.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in FermionHamiltonian form.
    ///
    /// # Panics
    ///
    /// * Internal bug in add_operator_product.
    fn from_iter<I: IntoIterator<Item = (HermitianFermionProduct, CalculatorComplex)>>(
        iter: I,
    ) -> Self {
        let mut so = FermionHamiltonian::new();
        for (pp, cc) in iter {
            so.add_operator_product(pp, cc)
                .expect("Internal bug in add_operator_product");
        }
        so
    }
}

/// Implements the extend function (Extend trait) of FermionHamiltonian.
///
impl Extend<(HermitianFermionProduct, CalculatorComplex)> for FermionHamiltonian {
    /// Extends the FermionHamiltonian by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the FermionHamiltonian.
    ///
    /// # Panics
    ///
    /// * Internal bug in add_operator_product.
    fn extend<I: IntoIterator<Item = (HermitianFermionProduct, CalculatorComplex)>>(
        &mut self,
        iter: I,
    ) {
        for (pp, cc) in iter {
            self.add_operator_product(pp, cc)
                .expect("Internal bug in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of FermionHamiltonian.
///
impl fmt::Display for FermionHamiltonian {
    /// Formats the FermionHamiltonian using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted FermionHamiltonian.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "FermionHamiltonian{\n".to_string();
        for (key, val) in self.iter() {
            writeln!(output, "{}: {},", key, val)?;
        }
        output.push('}');

        write!(f, "{}", output)
    }
}

impl JordanWignerFermionToSpin for FermionHamiltonian {
    type Output = QubitHamiltonian;

    /// Implements JordanWignerFermionToSpin for a FermionHamiltonian.
    ///
    /// The convention used is that |0> represents an empty fermionic state (spin-orbital),
    /// and |1> represents an occupied fermionic state.
    ///
    /// # Returns
    ///
    /// `QubitHamiltonian` - The spin Hamiltonian that results from the transformation.
    fn jordan_wigner(&self) -> Self::Output {
        let mut out = QubitHamiltonian::new();

        for hfp in self.keys() {
            let coeff = self.get(hfp);
            let creators: Vec<usize> = hfp.creators().cloned().collect();
            let annihilators: Vec<usize> = hfp.annihilators().cloned().collect();
            let fp = FermionProduct::new(creators, annihilators)
                .expect("Failed to create FermionProduct from HermitianFermionProduct.");

            if hfp.is_natural_hermitian() {
                out = out + hfp.jordan_wigner() * coeff.re.clone();
            } else {
                let (fp_conj, conjugate_sign) = fp.hermitian_conjugate();

                let qubit_op = fp.jordan_wigner() * coeff.clone()
                    + fp_conj.jordan_wigner() * conjugate_sign * coeff.conj();
                let qubit_hamiltonian = QubitHamiltonian::try_from(qubit_op).expect(
                    "Something went wrong when attempting to cast QubitOperator into QubitHamiltonian.",
                );
                out = out + qubit_hamiltonian;
            }
        }
        out
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_test::{assert_tokens, Configure, Token};

    // Test the Clone and PartialEq traits of QubitOperator
    #[test]
    fn so_from_sos() {
        let pp: HermitianFermionProduct = HermitianFermionProduct::new([0], [0]).unwrap();
        let sos = FermionHamiltonianSerialize {
            items: vec![(pp.clone(), 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "FermionHamiltonian".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0-alpha.0".to_string(),
            },
        };
        let mut so = FermionHamiltonian::new();
        so.set(pp, CalculatorComplex::from(0.5)).unwrap();

        assert_eq!(FermionHamiltonian::try_from(sos.clone()).unwrap(), so);
        assert_eq!(FermionHamiltonianSerialize::from(so), sos);
    }
    // Test the Clone and PartialEq traits of QubitOperator
    #[test]
    fn clone_partial_eq() {
        let pp: HermitianFermionProduct = HermitianFermionProduct::new([0], [0]).unwrap();
        let sos = FermionHamiltonianSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "FermionHamiltonian".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        // Test Clone trait
        assert_eq!(sos.clone(), sos);

        // Test PartialEq trait
        let pp_1: HermitianFermionProduct = HermitianFermionProduct::new([0], [0]).unwrap();
        let sos_1 = FermionHamiltonianSerialize {
            items: vec![(pp_1, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "FermionHamiltonian".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };
        let pp_2: HermitianFermionProduct = HermitianFermionProduct::new([0], [1]).unwrap();
        let sos_2 = FermionHamiltonianSerialize {
            items: vec![(pp_2, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "FermionHamiltonian".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };
        assert!(sos_1 == sos);
        assert!(sos == sos_1);
        assert!(sos_2 != sos);
        assert!(sos != sos_2);
    }

    // Test the Debug trait of QubitOperator
    #[test]
    fn debug() {
        let pp: HermitianFermionProduct = HermitianFermionProduct::new([0], [0]).unwrap();
        let sos = FermionHamiltonianSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "FermionHamiltonian".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_eq!(
            format!("{:?}", sos),
            "FermionHamiltonianSerialize { items: [(HermitianFermionProduct { creators: [0], annihilators: [0] }, Float(0.5), Float(0.0))], serialisation_meta: StruqtureSerialisationMeta { type_name: \"FermionHamiltonian\", min_version: (2, 0, 0), version: \"2.0.0\" } }"
        );
    }

    /// Test QubitOperator Serialization and Deserialization traits (readable)
    #[test]
    fn serde_readable() {
        let pp: HermitianFermionProduct = HermitianFermionProduct::new([0], [0]).unwrap();
        let sos = FermionHamiltonianSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "FermionHamiltonian".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_tokens(
            &sos.readable(),
            &[
                Token::Struct {
                    name: "FermionHamiltonianSerialize",
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
                Token::Str("FermionHamiltonian"),
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

    /// Test QubitOperator Serialization and Deserialization traits (compact)
    #[test]
    fn serde_compact() {
        let pp: HermitianFermionProduct = HermitianFermionProduct::new([0], [0]).unwrap();
        let sos = FermionHamiltonianSerialize {
            items: vec![(pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "FermionHamiltonian".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_tokens(
            &sos.compact(),
            &[
                Token::Struct {
                    name: "FermionHamiltonianSerialize",
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
                Token::Str("FermionHamiltonian"),
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
