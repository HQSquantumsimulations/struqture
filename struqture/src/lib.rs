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

// #![deny(missing_docs)]
// #![warn(private_intra_doc_links)]
// #![warn(missing_crate_level_docs)]
// #![warn(missing_doc_code_examples)]
// #![warn(private_doc_tests)]
// #![deny(missing_debug_implementations)]

use num_complex::Complex64;
use qoqo_calculator::CalculatorComplex;
use qoqo_calculator::CalculatorError;
use qoqo_calculator::CalculatorFloat;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
use std::str::FromStr;
use thiserror::Error;
pub const STRUQTURE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const MINIMUM_STRUQTURE_VERSION: (u32, u32, u32) = (1, 0, 0);
use tinyvec::TinyVec;
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Debug,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(try_from = "StruqtureVersionSerializable")]
#[serde(into = "StruqtureVersionSerializable")]
struct StruqtureVersion;

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
/// # StruqtureVersion
///
/// The minimal version of struqture needed to deserialize this object.
struct StruqtureVersionSerializable {
    /// The semver major version of struqture
    major_version: u32,
    /// The semver minor version of struqture
    minor_version: u32,
}

impl TryFrom<StruqtureVersionSerializable> for StruqtureVersion {
    type Error = StruqtureError;

    fn try_from(value: StruqtureVersionSerializable) -> Result<Self, Self::Error> {
        let mut rsplit = STRUQTURE_VERSION.split('.').take(2);
        let major_version = u32::from_str(
            rsplit
                .next()
                .expect("Internal error: Version not conforming to semver"),
        )
        .expect("Internal error: Major version is not unsigned integer.");
        let minor_version = u32::from_str(
            rsplit
                .next()
                .expect("Internal error: Version not conforming to semver"),
        )
        .expect("Internal error: Minor version is not unsigned integer.");
        if major_version != value.major_version {
            return Err(StruqtureError::VersionMissmatch {
                library_major_version: major_version,
                library_minor_version: minor_version,
                data_major_version: value.major_version,
                data_minor_version: value.minor_version,
            });
        }
        if major_version == 0 {
            if minor_version != value.minor_version {
                return Err(StruqtureError::VersionMissmatch {
                    library_major_version: major_version,
                    library_minor_version: minor_version,
                    data_major_version: value.major_version,
                    data_minor_version: value.minor_version,
                });
            }
        } else if minor_version < value.minor_version {
            return Err(StruqtureError::VersionMissmatch {
                library_major_version: major_version,
                library_minor_version: minor_version,
                data_major_version: value.major_version,
                data_minor_version: value.minor_version,
            });
        }

        Ok(StruqtureVersion)
    }
}

impl From<StruqtureVersion> for StruqtureVersionSerializable {
    fn from(_: StruqtureVersion) -> Self {
        let mut rsplit = STRUQTURE_VERSION.split('.').take(2);
        let major_version = u32::from_str(
            rsplit
                .next()
                .expect("Internal error: Version not conforming to semver"),
        )
        .expect("Internal error: Major version is not unsigned integer.");
        let minor_version = u32::from_str(
            rsplit
                .next()
                .expect("Internal error: Version not conforming to semver"),
        )
        .expect("Internal error: Minor version is not unsigned integer.");
        StruqtureVersionSerializable {
            major_version,
            minor_version,
        }
    }
}

/// Errors that can occur in struqture.
#[derive(Debug, Error, PartialEq)]
pub enum StruqtureError {
    /// Error when remapping qubits fails because qubit in operation is not in keys of BTreeMap.
    #[error("The qubit remapping failed for this qubit: {key:?}.")]
    RemappingFailed {
        /// Key that has caused the remapping to fail.
        key: usize,
    },
    /// Error when using from_str.
    #[error("The from_str function failed {msg} string")]
    FromStringFailed { msg: String },
    /// Error the Pauli matrix set is not in the allowed matrices.
    #[error("The pauli matrix being set is not in [\"I\", \"X\", \"Y\", \"Z\"] (PauliProduct object) or [\"I\", \"X\", \"iY\", \"Z\"] (DecoherenceProduct object): {pauli:?}")]
    IncorrectPauliEntry {
        /// Incorrect Pauli matrix trying to be added.
        pauli: String,
    },
    /// Error when adding a key to an object as the index key already exists.
    #[error("Cannot assign pauli matrix to index {index:?} as it is already occupied")]
    ProductIndexAlreadyOccupied {
        /// Index that is occupied.
        index: usize,
    },
    /// Error when adding a key to an object as the SpinIndex object key already exists.
    #[error("Cannot assign pauli matrix to index {index:?} as it is already occupied")]
    OperatorIndexAlreadyOccupied {
        /// Index that is occupied.
        index: String,
    },
    /// Error when index of SpinIndex object exceeds that of the Spin(Hamiltonian)System.
    #[error("Index of SpinIndex object exceeds that of the Spin(Hamiltonian)System")]
    NumberSpinsExceeded,
    /// Error when number of spins between system and noise missmatched.
    #[error("Number of spins between system and noise missmatched")]
    MissmatchedNumberSpins,
    /// Error when number of modes between system and noise missmatched.
    #[error("Number of modes between system and noise missmatched")]
    MissmatchedNumberModes,
    /// Error when the number of subsystems in a mixed system does not match.
    #[error("Number of subsystems does not match. target: {target_number_spin_subsystems} spin {target_number_boson_subsystems} boson {target_number_fermion_subsystems} fermion; actual: {actual_number_spin_subsystems} spin {actual_number_boson_subsystems} boson {actual_number_fermion_subsystems} fermion ")]
    MissmatchedNumberSubsystems {
        target_number_spin_subsystems: usize,
        target_number_boson_subsystems: usize,
        target_number_fermion_subsystems: usize,
        actual_number_spin_subsystems: usize,
        actual_number_boson_subsystems: usize,
        actual_number_fermion_subsystems: usize,
    },
    /// Error when the indices of the object being added are not Normal Ordered.
    #[error("Indices are not normal ordered: {index_j:?} is larger than index {index_i:?}")]
    IndicesNotNormalOrdered {
        /// Index i of the object.
        index_i: usize,
        /// Index j of the object.
        index_j: usize,
    },
    /// Error when the indices of the object being added are not Normal Ordered.
    #[error(
        "Indices given in either creators or annihilators contain a double index specification"
    )]
    IndicesContainDoubles,
    /// Error when the creator indices of the object being added are not Normal Ordered or contain a double.
    #[error(
        "Indices given in creators/annihilators are either not normal ordered, or contain a double index specification"
    )]
    IncorrectlyOrderedIndices,
    /// Error when index of (Hermitian)BosonProduct exceeds that of the Boson(Hamiltonian)System.
    #[error("Index of (Hermitian)BosonProduct exceeds that of the Boson(Hamiltonian)System")]
    NumberModesExceeded,
    /// Error when the minimum index of the creators of the object is larger than the minimum index of the annihilators object.
    #[error("The minimum index of the creators {creators_min:?} is larger than the minimum index of the annihilators {annihilators_min:?}")]
    CreatorsAnnihilatorsMinimumIndex {
        /// Minimum index of the creators.
        creators_min: Option<usize>,
        /// Minimum index of the annihilators.
        annihilators_min: Option<usize>,
    },
    /// Error when the key is naturally hermitian (on-diagonal term), but its corresponding value is not real.
    #[error(
        "Key is naturally hermitian (on-diagonal term), but its corresponding value is not real."
    )]
    NonHermitianOperator,
    /// Error when parsing from str
    #[error("Error parsing str into {target_type}: {msg}")]
    ParsingError { target_type: String, msg: String },
    /// Error when trying to deserialize struqture data created with an incompatible version of struqture
    #[error("Trying to deserialize data created with incompatible version of struqture Library version: {library_major_version}.{library_minor_version} Data version: {data_major_version}.{data_minor_version}. Try to convert data with struqture data conversion tool.")]
    VersionMissmatch {
        /// Major version of the library
        library_major_version: u32,
        /// Minor version of the library
        library_minor_version: u32,
        /// Major version of the data
        data_major_version: u32,
        /// Minor version of the data
        data_minor_version: u32,
    },
    /// Transparent propagation of CalculatorError.
    #[error(transparent)]
    CalculatorError(#[from] CalculatorError),
    /// Error when trying to insert identities into noise operators
    #[error("Lindblad operators need to be traceless.")]
    InvalidLindbladTerms,
    /// Gerneric Error in struqture.
    #[error("Error occured: {msg}")]
    GenericError {
        /// Error message
        msg: String,
    },
}

/// Complex sparse matrix in coordinate (COO) format.
///
/// Input in the form (value_vector, (row_index_vector, column_index_vector))
pub type CooSparseMatrix = (Vec<Complex64>, (Vec<usize>, Vec<usize>));

/// Real sparse matrix in coordinate (COO) format.
///
/// Input in the form (value_vector, (row_index_vector, column_index_vector))
pub type CooSparseMatrixReal = (Vec<f64>, (Vec<usize>, Vec<usize>));

/// Trait for all hermitian indices
pub trait SymmetricIndex:
    std::hash::Hash + Eq + Sized + Clone + std::fmt::Debug + std::fmt::Display + FromStr + Default
{
    /// Returns the hermitian conjugate of Self and its prefactor.
    ///
    /// # Returns
    ///
    /// * `(Self, f64)` - The hermitian conjugate of Self and its prefactor.
    fn hermitian_conjugate(&self) -> (Self, f64);

    /// Returns whether Self is naturally hermitian.
    ///
    /// For spin objects, this is true when applying the hermitian conjugation does not change the sign.
    /// For bosonic and fermionic objects, this is true when creators == annihilators.
    /// For mixed objects, this is true when all of the spin, bosonic and fermionic parts' `is_naturally_hermitian` functions evaluate to true.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether Self is naturally hermitian or not.
    fn is_natural_hermitian(&self) -> bool;
}

/// Trait for all index types requires converting between index types
pub trait SpinIndex:
    SymmetricIndex
    + std::hash::Hash
    + Eq
    + Sized
    + Clone
    + std::fmt::Debug
    + std::fmt::Display
    + FromStr
    + Default
    + serde::Serialize
where
    Self::SingleSpinType: Copy,
{
    /// Type of operators on single spin in a SpinIndex.
    ///
    /// This can either be a [crate::spins::SingleSpinOperator] (`I`, `X`, `Y` or `Z`)
    /// or a [crate::spins::SingleOperator/Hamiltonian] (`I`, `X`, `iY` or `Z`)
    type SingleSpinType;

    /// Creates a new Self typed object.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) instance of type `Self`.
    fn new() -> Self;

    /// Sets a new entry in Self. This function consumes Self.
    ///
    /// # Arguments
    ///
    /// * `index` - Index of set object.
    /// * `pauli` - Value of set object.
    ///
    /// # Returns
    ///
    /// * `Self` - The entry was correctly set and the new object is returned.
    fn set_pauli(self, index: usize, pauli: Self::SingleSpinType) -> Self;

    /// Gets the pauli matrix corresponding to the index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of qubit to get the pauli matrix for.
    ///
    /// # Returns
    ///
    /// * `Some(&SingleSpinType)` - The key exists and its corresponding value is returned.
    /// * `None` - The key does not exist in Self.
    fn get(&self, index: &usize) -> Option<&Self::SingleSpinType>;

    /// Returns the iterator form of Self.
    ///
    /// # Returns
    ///
    /// * `Iter<usize, SingleSpinType>` - The iterator form of Self.
    fn iter(&self) -> std::slice::Iter<(usize, Self::SingleSpinType)>;

    /// Returns maximum index in Self.
    ///
    /// # Returns
    ///
    /// * `usize` - Maximum index.
    fn current_number_spins(&self) -> usize {
        if let Some((max, _)) = self.iter().last() {
            *max + 1
        } else {
            0
        }
    }

    /// Returns the length of the SpinIndex object.
    ///
    /// # Returns
    ///
    /// * `usize` - The length of the SpinIndex object.
    fn len(&self) -> usize {
        self.iter().len()
    }

    /// Returns whether the SpinIndex object is empty or not.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the SpinIndex object is empty or not.
    fn is_empty(&self) -> bool {
        self.iter().len() == 0
    }

    /// Remaps the qubits in a clone instance of Self.
    ///
    /// # Arguments
    ///
    /// * `mapping` - The map containing the {qubit: qubit} mapping to use.
    ///
    /// # Returns
    ///
    /// * `Self` -  The new object with the qubits remapped from Self.
    fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Self;

    /// Implements multiplication function for a Self typed object by a Self typed object.
    ///
    /// # Arguments
    ///
    /// * `left` - Left-hand Self typed object to be multiplied.
    /// * `right` - Right-hand Self typed object to be multiplied.
    ///
    /// Returns
    ///
    /// * `(Self, Complex64)` - The multiplied objects and the resulting prefactor.
    fn multiply(left: Self, right: Self) -> (Self, Complex64);

    /// Returns the concatenation of two Self typed objects with no overlapping qubits.
    ///
    /// # Arguments
    ///
    /// * `other` - The object to concatenate Self with.
    ///
    /// Returns
    ///
    /// * `Ok(Self)` - The concatenated objects.
    /// * `Err(StruqtureError::ProductIndexAlreadyOccupied)` - Cannot assign pauli matrix to index as it is already occupied.
    fn concatenate(&self, other: Self) -> Result<Self, StruqtureError>;
}

/// Trait for all index types requires converting between index types
pub trait ModeIndex:
    SymmetricIndex
    + std::hash::Hash
    + Eq
    + Sized
    + Clone
    + std::fmt::Debug
    + std::fmt::Display
    + FromStr
    + Default
    + serde::Serialize
{
    // Document locally
    fn new(
        creators: impl IntoIterator<Item = usize>,
        annihilators: impl IntoIterator<Item = usize>,
    ) -> Result<Self, StruqtureError>;

    /// Gets the number of creator indices of Self.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of creator indices in Self.
    fn number_creators(&self) -> usize {
        self.creators().len()
    }

    /// Gets the number of annihilator indices of Self.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of annihilator indices in Self.
    fn number_annihilators(&self) -> usize {
        self.annihilators().len()
    }

    /// Gets the creator indices of Self.
    ///
    /// # Returns
    ///
    /// * `Iter<usize>` - The creator indices in Self.
    fn creators(&self) -> std::slice::Iter<usize>;

    /// Gets the annihilator indices of Self.
    ///
    /// # Returns
    ///
    /// * `Iter<usize>` - The annihilator indices in Self.
    fn annihilators(&self) -> std::slice::Iter<usize>;

    // Document locally
    fn create_valid_pair(
        creators: impl IntoIterator<Item = usize>,
        annihilators: impl IntoIterator<Item = usize>,
        value: CalculatorComplex,
    ) -> Result<(Self, CalculatorComplex), StruqtureError>;

    /// Returns the maximal number of modes the Index (operator product) acts on.
    ///
    /// A ModeIndex acts on a state space of unknown dimension.
    /// There is only a lower bound of the dimension or number of modes based on the
    /// maximal mode the product of operators in the index acts on.
    ///
    /// For example an index consisting of one creator acting on mode 0 would have
    /// a current_number_modes of one. An index consisting of one annhihilator acting on 3
    /// would have current_number_modes of four.
    fn current_number_modes(&self) -> usize {
        let max_c = match self.creators().max() {
            Some(x) => x + 1,
            None => 0,
        };
        let max_a = match self.annihilators().max() {
            Some(x) => x + 1,
            None => 0,
        };
        max_c.max(max_a)
    }

    /// Remap modes according to an input dictionary.
    ///
    /// # Arguments
    ///
    /// `reordering_dictionary` - The dictionary specifying the remapping. It must represent a permutation.
    ///
    /// # Returns
    ///
    /// `(Self, CalculatorComplex)` - The instance of Self with modes remapped, and the sign resulting from symmetry/antisymmetry.
    fn remap_modes(
        &self,
        reordering_dictionary: &HashMap<usize, usize>,
    ) -> Result<(Self, CalculatorComplex), StruqtureError> {
        let mut keys: Vec<usize> = reordering_dictionary.keys().cloned().collect();
        keys.sort();
        let mut values: Vec<usize> = reordering_dictionary.values().cloned().collect();
        values.sort();

        if keys != values {
            return Err(StruqtureError::GenericError {
                msg: "Input dictionary must be a permutation.".to_string(),
            });
        }

        let mut remapped_creators: Vec<usize> = vec![];
        let mut remapped_annihilators: Vec<usize> = vec![];

        for creator_index in self.creators() {
            remapped_creators.push(match reordering_dictionary.get(creator_index) {
                Some(x) => *x,
                None => *creator_index,
            })
        }
        for annihilator_index in self.annihilators() {
            remapped_annihilators.push(match reordering_dictionary.get(annihilator_index) {
                Some(x) => *x,
                None => *annihilator_index,
            })
        }
        let (remapped_index, new_coeff) =
            Self::create_valid_pair(remapped_creators, remapped_annihilators, 1.0.into()).map_err(
                |_| StruqtureError::GenericError {
                    msg: "Remapping dictionary should be a permutation of the indices.".to_string(),
                },
            )?;
        Ok((remapped_index, new_coeff))
    }
}

/// Trait for transforming value stored at index I when using index of different type T to read out value
/// e.g. Hermitian Hamiltonian H but we access H[NOIndex(2,1)] -> H[HermitianIndex(1,2)].conj()
pub trait GetValue<T> {
    type ValueIn;
    type ValueOut;

    // Document locally
    fn get_key(index: &T) -> Self;

    // Document locally
    fn get_transform(index: &T, value: Self::ValueIn) -> Self::ValueOut;
}

/// Trait linking indices of lower symmetry to one with higher symmetry
///
/// Several indices of a lower symmetry (e.g. normal ordered products of creators and annihilators)
/// can correspond to the same index of a higher symmetry (e.g. Hermitian products where
/// a pair of hermitian conjugated products of creators and annihilators is indexed by a single index)
///
/// ```
/// use struqture::prelude::*;
/// use struqture::CorrespondsTo;
/// use struqture::bosons::{BosonProduct, HermitianBosonProduct};
///
/// let hbp = HermitianBosonProduct::new([0, 2, 4], [1, 3, 5]).unwrap();
/// let bp_1 = BosonProduct::new([0, 2, 4], [1, 3, 5]).unwrap();
/// let bp_2 = BosonProduct::new([1, 3, 5], [0, 2, 4]).unwrap();
/// let bp1_coresponds: HermitianBosonProduct = bp_1.corresponds_to();
/// let bp2_coresponds: HermitianBosonProduct = bp_2.corresponds_to();
///
/// assert_eq!(hbp, bp1_coresponds);
/// assert_eq!(hbp, bp2_coresponds);
/// ```
pub trait CorrespondsTo<T> {
    // Document locally
    fn corresponds_to(&self) -> T;
}

/// Helper trait to allow truncation of values below threshold.
/// Should eventually be ported to qoqo_calculator like this
/// and be implemented for CalculatorFloat, CaclulatorComplex, f64 and Complexf64
pub trait TruncateTrait: Sized {
    /// Truncates values mapping discarded values to None.
    ///
    /// Values with an absolute value under the threshold are mapped to None.
    /// Values that are not removed are mapped to Some(value).
    /// For complex values the threshold is applied to real and imaginary part separately.
    /// All symbolic values are considered to be above the threshold.
    ///
    /// # Arguments
    ///
    /// * `threshold` - The threshold for inclusion.
    ///
    /// # Returns
    ///
    /// * `Some(Self)` - The truncated version of Self.
    /// * `None` - Nothing was left in Self below the threshold.
    fn truncate(&self, threshold: f64) -> Option<Self>;
}

impl TruncateTrait for CalculatorComplex {
    fn truncate(&self, threshold: f64) -> Option<Self> {
        match (&self.re, &self.im) {
            (CalculatorFloat::Str(_), _) => Some(self.clone()),
            (_, CalculatorFloat::Str(_)) => Some(self.clone()),
            (CalculatorFloat::Float(re), CalculatorFloat::Float(im)) => {
                let new_re = if re.abs() >= threshold { *re } else { 0.0 };
                let new_im = if im.abs() >= threshold { *im } else { 0.0 };
                if Complex64::new(new_re, new_im).norm() >= threshold {
                    Some(CalculatorComplex::new(new_re, new_im))
                } else {
                    None
                }
            }
        }
    }
}

impl TruncateTrait for CalculatorFloat {
    fn truncate(&self, threshold: f64) -> Option<Self> {
        match &self {
            CalculatorFloat::Str(_) => Some(self.clone()),
            CalculatorFloat::Float(f) => {
                if f.abs() >= threshold {
                    Some(self.clone())
                } else {
                    None
                }
            }
        }
    }
}

/// Helper trait to allow hermitian conjugation of values
/// Should eventually be ported to qoqo_calculator like this
/// and be implemented for CalculatorFloat, CaclulatorComplex, f64 and Complexf64
pub trait ConjugationTrait: Sized {
    /// Conjugates all values in Self.
    ///
    /// # Returns
    ///
    /// * `Self` - The conjugated version of Self.
    fn conjugate(&self) -> Self;
}

impl ConjugationTrait for CalculatorComplex {
    fn conjugate(&self) -> Self {
        self.conj()
    }
}

impl ConjugationTrait for CalculatorFloat {
    fn conjugate(&self) -> Self {
        self.clone()
    }
}

/// Trait for all objects that can act on a quantum density matrix like a superoperator.
///
/// # Example
/// ```
/// use qoqo_calculator::CalculatorComplex;
/// use std::collections::HashMap;
/// use struqture::prelude::*;
/// use struqture::spins::{OperateOnSpins, PauliProduct, SpinOperator};
///
/// let mut so = SpinOperator::new();
/// let pp_0z = PauliProduct::new().z(0);
/// so.add_operator_product(pp_0z.clone(), CalculatorComplex::from(0.2)).unwrap();
/// let mut mapping: HashMap<PauliProduct, CalculatorComplex> = HashMap::new();
/// mapping.insert(pp_0z.clone(), CalculatorComplex::from(0.2));
///
/// // Functions provided in this :
/// assert_eq!(so.get(&pp_0z), &CalculatorComplex::from(0.2));
/// for (item_so, item_map) in so.iter().zip(mapping.iter()) {
///     assert_eq!(item_so, item_map);
/// }
/// for (key_so, key_map) in so.keys().zip(mapping.keys()) {
///     assert_eq!(key_so, key_map);
/// }
/// for (val_so, val_map) in so.values().zip(mapping.values()) {
///     assert_eq!(val_so, val_map);
/// }
/// assert_eq!(so.len(), 1_usize);
/// assert_eq!(so.is_empty(), false);
/// ```
///
pub trait OperateOnDensityMatrix<'a>:
    IntoIterator<Item = (Self::Index, Self::Value)>
    + FromIterator<(Self::Index, Self::Value)>
    + Extend<(Self::Index, Self::Value)>
    + PartialEq
    + Clone
    + Mul<CalculatorFloat>
    + Mul<CalculatorComplex>
    + Add
    + Sub
    + std::fmt::Display
    + serde::Serialize
    + serde::Deserialize<'a>
where
    Self: 'a,
    &'a Self: IntoIterator,
    Self::Index: Clone,
    Self::Value: Mul<f64, Output = Self::Value>,
    Self::Value: Add<Self::Value, Output = Self::Value>,
    Self::Value: Clone,
    Self::Value: TruncateTrait,
    Self::IteratorType: ExactSizeIterator<Item = (&'a Self::Index, &'a Self::Value)>,
    Self::KeyIteratorType: ExactSizeIterator<Item = &'a Self::Index>,
    Self::ValueIteratorType: ExactSizeIterator<Item = &'a Self::Value>,
{
    type Index;
    type Value;
    type IteratorType;
    type KeyIteratorType;
    type ValueIteratorType;

    /// Gets the Self::Value typed coefficient corresponding to the key.
    ///
    /// # Arguments
    ///
    /// * `key` - The Self::Index key for which to retrieve the value.
    ///
    /// # Returns
    ///
    /// *  Value at key (or 0.0).
    fn get(&self, key: &Self::Index) -> &Self::Value;

    /// Returns the iterator form of Self.
    ///
    /// # Returns
    ///
    /// * `Iter<'_, Self::Index, Self::Value>` - Self in iterator form.
    fn iter(&'a self) -> Self::IteratorType;

    /// Returns the unsorted keys in Self.
    ///
    /// # Returns
    ///
    /// * `Keys<'_, Self::Index, Self::Value>` - The sequence of keys of Self.
    fn keys(&'a self) -> Self::KeyIteratorType;

    /// Returns the unsorted values in Self.
    ///
    /// # Returns
    ///
    /// * `Values<'_, Self::Index, Self::Value>` - The sequence of values of Self.
    fn values(&'a self) -> Self::ValueIteratorType;

    /// Returns number of entries in object.
    ///
    /// # Returns
    ///
    /// * `usize` - The length of the object's internal_map.
    fn len(&'a self) -> usize {
        self.iter().len()
    }

    /// Returns true if object contains no values.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the object is empty or not.
    fn is_empty(&'a self) -> bool {
        self.len() == 0
    }

    /// Removes the value of the Self::Index object key.
    ///
    /// # Arguments
    ///
    /// * `key` - The Self::Index object to remove from Self.
    ///
    /// # Returns
    ///
    /// * `Some(Self::Value)` - Key existed, this is the value it had before it was removed.
    /// * `None` - Key did not exist.
    fn remove(&mut self, key: &Self::Index) -> Option<Self::Value>;

    /// Returns an instance of Self that has no entries but clones all other properties, with the given capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The capacity of the object to create.
    ///
    /// # Returns
    ///
    /// * `Self` - An empty clone with the same properties as Self, with the given capacity.
    fn empty_clone(&self, capacity: Option<usize>) -> Self;

    // Document locally
    fn set(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<Option<Self::Value>, StruqtureError>;

    // Document locally
    fn add_operator_product(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<(), StruqtureError> {
        let old = self.get(&key).clone();
        self.set(key, value + old)?;
        Ok(())
    }

    /// Truncates Self by returning a copy without entries under a threshold.
    ///
    /// Entries with an absolute value under the threshold are removed from the copy of the object that is returned.
    /// For entries with complex coefficients the threshold is applied to real and imaginary part separately.
    /// All symbolic values are considered to be above the threshold.
    ///
    /// # Arguments
    ///
    /// * `threshold` - The threshold for inclusion.
    ///
    /// # Returns
    ///
    /// * `Self` - The truncated version of Self.
    fn truncate(&'a self, threshold: f64) -> Self {
        let mut new_self = self.empty_clone(Some(self.len()));
        new_self.extend(self.iter().filter_map(|(k, v)| {
            v.truncate(threshold)
                .map(|v_truncated| (k.clone(), v_truncated))
        }));
        new_self
    }
}

/// Trait for representing complete open systems
pub trait OpenSystem<'a>:
    Add + Sub + PartialEq + Clone + std::fmt::Display + serde::Serialize + serde::Deserialize<'a>
where
    Self::System: OperateOnState<'a>,
    Self::System: 'a,
    &'a Self::System: IntoIterator,
    <<Self as OpenSystem<'a>>::System as OperateOnDensityMatrix<'a>>::Index: Clone,
    <<Self as OpenSystem<'a>>::System as OperateOnDensityMatrix<'a>>::Index: SymmetricIndex,
    <<Self as OpenSystem<'a>>::System as OperateOnDensityMatrix<'a>>::Value:
        Mul<f64, Output = <<Self as OpenSystem<'a>>::System as OperateOnDensityMatrix<'a>>::Value>,
    <<Self as OpenSystem<'a>>::System as OperateOnDensityMatrix<'a>>::Value: Add<
        <<Self as OpenSystem<'a>>::System as OperateOnDensityMatrix<'a>>::Value,
        Output = <<Self as OpenSystem<'a>>::System as OperateOnDensityMatrix<'a>>::Value,
    >,
    <<Self as OpenSystem<'a>>::System as OperateOnDensityMatrix<'a>>::Value: Clone,
    <<Self as OpenSystem<'a>>::System as OperateOnDensityMatrix<'a>>::Value: TruncateTrait,
    <<Self as OpenSystem<'a>>::System as OperateOnDensityMatrix<'a>>::Value: ConjugationTrait,
    <<Self as OpenSystem<'a>>::System as OperateOnDensityMatrix<'a>>::IteratorType:
        ExactSizeIterator<
            Item = (
                &'a <<Self as OpenSystem<'a>>::System as OperateOnDensityMatrix<'a>>::Index,
                &'a <<Self as OpenSystem<'a>>::System as OperateOnDensityMatrix<'a>>::Value,
            ),
        >,
    <<Self as OpenSystem<'a>>::System as OperateOnDensityMatrix<'a>>::KeyIteratorType:
        ExactSizeIterator<
            Item = &'a <<Self as OpenSystem<'a>>::System as OperateOnDensityMatrix<'a>>::Index,
        >,
    <<Self as OpenSystem<'a>>::System as OperateOnDensityMatrix<'a>>::ValueIteratorType:
        ExactSizeIterator<
            Item = &'a <<Self as OpenSystem<'a>>::System as OperateOnDensityMatrix<'a>>::Value,
        >,
    Self::Noise: OperateOnDensityMatrix<'a>,
    Self::Noise: 'a,
    &'a Self::Noise: IntoIterator,
    <<Self as OpenSystem<'a>>::Noise as OperateOnDensityMatrix<'a>>::Index: Clone,
    <<Self as OpenSystem<'a>>::Noise as OperateOnDensityMatrix<'a>>::Value:
        Mul<f64, Output = <<Self as OpenSystem<'a>>::Noise as OperateOnDensityMatrix<'a>>::Value>,
    <<Self as OpenSystem<'a>>::Noise as OperateOnDensityMatrix<'a>>::Value: Add<
        <<Self as OpenSystem<'a>>::Noise as OperateOnDensityMatrix<'a>>::Value,
        Output = <<Self as OpenSystem<'a>>::Noise as OperateOnDensityMatrix<'a>>::Value,
    >,
    <<Self as OpenSystem<'a>>::Noise as OperateOnDensityMatrix<'a>>::Value: Clone,
    <<Self as OpenSystem<'a>>::Noise as OperateOnDensityMatrix<'a>>::Value: TruncateTrait,
    <<Self as OpenSystem<'a>>::Noise as OperateOnDensityMatrix<'a>>::Value: ConjugationTrait,
    <<Self as OpenSystem<'a>>::Noise as OperateOnDensityMatrix<'a>>::IteratorType:
        ExactSizeIterator<
            Item = (
                &'a <<Self as OpenSystem<'a>>::Noise as OperateOnDensityMatrix<'a>>::Index,
                &'a <<Self as OpenSystem<'a>>::Noise as OperateOnDensityMatrix<'a>>::Value,
            ),
        >,
    <<Self as OpenSystem<'a>>::Noise as OperateOnDensityMatrix<'a>>::KeyIteratorType:
        ExactSizeIterator<
            Item = &'a <<Self as OpenSystem<'a>>::Noise as OperateOnDensityMatrix<'a>>::Index,
        >,
    <<Self as OpenSystem<'a>>::Noise as OperateOnDensityMatrix<'a>>::ValueIteratorType:
        ExactSizeIterator<
            Item = &'a <<Self as OpenSystem<'a>>::Noise as OperateOnDensityMatrix<'a>>::Value,
        >,
{
    type System;
    type Noise;

    /// Returns the Self::Noise of the OpenSystem object.
    ///
    /// # Returns
    ///
    /// * `&Self::Noise` - The Self::Noise of the OpenSystem object.
    fn noise(&self) -> &Self::Noise;

    /// Returns the Self::System of the OpenSystem object.
    ///
    /// # Returns
    ///
    /// * `&Self::System` - The Self::System of the OpenSystem object.
    fn system(&self) -> &Self::System;

    /// Returns a mutable reference of the Self::Noise of the OpenSystem object.
    ///
    /// # Returns
    ///
    /// * `&Self::Noise` - The Self::Noise of the OpenSystem object.
    fn noise_mut(&mut self) -> &mut Self::Noise;

    /// Returns a mutable reference of the Self::System of the OpenSystem object.
    ///
    /// # Returns
    ///
    /// * `&Self::System` - The Self::System of the OpenSystem object.
    fn system_mut(&mut self) -> &mut Self::System;

    /// Returns a tuple of the system (Self::System) and the noise (Self::Noise) of the OpenSystem.
    ///
    /// # Returns
    ///
    /// * `(Self::System, Self::Noise)` - The system and noise of the OpenSystem.
    fn ungroup(self) -> (Self::System, Self::Noise);

    // Document locally
    fn group(system: Self::System, noise: Self::Noise) -> Result<Self, StruqtureError>;

    /// Returns an instance of Self that has no entries but clones all other properties, with the given capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The capacity of the object to create.
    ///
    /// # Returns
    ///
    /// * `Self` - An empty clone with the same properties as Self, with the given capacity.
    fn empty_clone(&self) -> Self;

    /// Truncates Self by returning a copy without entries under a threshold.
    ///
    /// Entries with an absolute value under the threshold are removed from the copy of the object that is returned.
    /// For entries with complex coefficients the threshold is applied to real and imaginary part separately.
    /// All symbolic values are considered to be above the threshold.
    ///
    /// # Arguments
    ///
    /// * `threshold` - The threshold for inclusion.
    ///
    /// # Returns
    ///
    /// * `Self` - The truncated version of Self.
    fn truncate(&'a self, threshold: f64) -> Self {
        let new_system = self.system().truncate(threshold);
        let new_noise = self.noise().truncate(threshold);
        Self::group(new_system, new_noise)
            .expect("Internal error: System and Noise size unexpectedly do not match")
    }
}

/// Trait for all objects that can act on a quantum state like an operator.
///
/// # Example
/// ```
/// use qoqo_calculator::CalculatorComplex;
/// use std::collections::HashMap;
/// use struqture::prelude::*;
/// use struqture::spins::{OperateOnSpins, PauliProduct, SpinOperator};
///
/// let mut so = SpinOperator::new();
/// let pp_0z = PauliProduct::new().z(0);
/// so.add_operator_product(pp_0z.clone(), CalculatorComplex::from(0.2)).unwrap();
///
/// // Functions provided in this :
/// assert_eq!(so.hermitian_conjugate(), so);
/// ```
///
pub trait OperateOnState<'a>:
    OperateOnDensityMatrix<'a>
    + IntoIterator<Item = (Self::Index, Self::Value)>
    + FromIterator<(Self::Index, Self::Value)>
    + Extend<(Self::Index, Self::Value)>
    + PartialEq
    + Clone
    + Mul<CalculatorFloat>
    + Mul<CalculatorComplex>
    + Add
    + Sub
where
    Self: 'a,
    &'a Self: IntoIterator,
    Self::Index: Clone,
    Self::Index: SymmetricIndex,
    Self::Value: Mul<f64, Output = Self::Value>,
    Self::Value: Add<Self::Value, Output = Self::Value>,
    Self::Value: Clone,
    Self::Value: TruncateTrait,
    Self::Value: ConjugationTrait,
    Self::IteratorType: ExactSizeIterator<Item = (&'a Self::Index, &'a Self::Value)>,
    Self::KeyIteratorType: ExactSizeIterator<Item = &'a Self::Index>,
    Self::ValueIteratorType: ExactSizeIterator<Item = &'a Self::Value>,
{
    /// Returns the hermitian conjugate of Self.
    ///
    /// # Returns
    ///
    /// * `Self` - The hermitian conjugate of Self.
    fn hermitian_conjugate(&'a self) -> Self {
        let mut new_self = self.empty_clone(Some(self.len()));
        new_self.extend(self.iter().map(|(k, v)| {
            let (new_key, conjugation_prefactor) = k.hermitian_conjugate();
            (new_key, v.conjugate() * conjugation_prefactor)
        }));
        new_self
    }
}

/// Trait for bosonic or fermionic modes.
///
/// # Example
/// ```
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::prelude::*;
/// use std::collections::HashMap;
/// use struqture::bosons::{HermitianBosonProduct, BosonHamiltonian};
///
/// let mut sh = BosonHamiltonian::new();
///
/// // Functions provided in this :
/// assert_eq!(sh.current_number_modes(), 0);
/// assert_eq!(sh.number_modes(), 0);
///
/// let pp_0z = HermitianBosonProduct::new([0], [0]).unwrap();
/// sh.add_operator_product(pp_0z.clone(), CalculatorComplex::from(0.2)).unwrap();
///
/// assert_eq!(sh.current_number_modes(), 1);
/// assert_eq!(sh.number_modes(), 1);
/// ```
///
pub trait OperateOnModes<'a>: PartialEq + Clone + Mul<CalculatorFloat> + Add + Sub {
    /// Return maximum index in Self.
    ///
    /// # Returns
    ///
    /// * `usize` - Maximum index.
    fn current_number_modes(&'a self) -> usize;

    // Document locally
    fn number_modes(&'a self) -> usize;
}

/// Shorthand type notation for a tuple of lists of indices of creators and annihilators
type CreatorsAnnihilators = (TinyVec<[usize; 2]>, TinyVec<[usize; 2]>);

pub mod bosons;
pub mod fermions;
pub mod mappings;
pub mod mixed_systems;
pub mod prelude;
pub mod spins;

/// Shorhand type for TinyVec representation of creators or annihilators
#[cfg(test)]
type ModeTinyVec = TinyVec<[usize; 2]>;

/// Trait for implementing a function to determine the minimum supported version of struqture required.
pub trait MinSupportedVersion {
    /// Returns the minimum version of struqture required to deserialize this object.
    ///
    /// # Returns
    /// (majon_verision, minor_version, patch_version)
    fn min_supported_version() -> (usize, usize, usize) {
        (1, 0, 0)
    }
}
