// Copyright © 2024 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use crate::StruqtureError;
use std::fmt::Display;
use std::str::FromStr;

use crate::STRUQTURE_VERSION;

/// Struct encoding serialisation meta information for struqture objects.
///
/// This struct is meant to be used when checking if a struqture object
/// can be serialised or deserialised into the correct struqture type.
///
/// For this purpose the struct contains
///
/// * The exact type of the struqture object.
/// * The minimum struqture version required to deserialise this object.
/// * The struqture version used to create the object.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
pub struct StruqtureSerialisationMeta {
    /// The exact type of the struqture object.
    // This cannot be a StruqtureType directly because we have no way to
    // deserialise into a default type of StruqtureType when the
    // type is not known (trying to deserialise an object from
    // future struqture versions).
    pub(crate) type_name: String,
    /// The minimum struqture version required to deserialise this object in semver format.
    pub(crate) min_version: (usize, usize, usize),
    /// The struqture version used to create the object in semver format.
    pub(crate) version: String,
}

/// Struct encoding serialisation meta information for targets for deserialisation.
///
/// This struct is meant to be used when checking if a struqture object
/// can be serialised or deserialised into the correct struqture type.
///
/// For this purpose the struct contains
///
/// * The exact type of the struqture object.
/// * The struqture version used to create the object.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TargetSerialisationMeta {
    /// The exact type of the struqture object.
    // This cannot be a StruqtureType directly because we have no way to
    // deserialise into a default type of StruqtureType when the
    // type is not known (trying to deserialise an object from
    // future struqture versions).
    pub(crate) type_name: String,

    /// The struqture version used to create the object in semver format.
    pub(crate) version: String,
}

/// Checks if a source object (e.g. something deserialised from JSON) can be deserialized into the tager type.
pub fn check_can_be_deserialised(
    target: &TargetSerialisationMeta,
    source: &StruqtureSerialisationMeta,
) -> Result<(), StruqtureError> {
    if target.type_name != source.type_name {
        return Err(StruqtureError::TypeMissmatch {
            source_type: source.type_name.clone(),
            target_type: target.type_name.clone(),
        });
    }
    let (target_supported_major, target_supported_minor, _) =
        semver_to_tuple(target.version.as_str())?;
    if target_supported_major as usize != source.min_version.0 {
        return Err(StruqtureError::NewVersionMissmatch {
            library_major_version: target_supported_major,
            library_minor_version: target_supported_minor,
            data_major_version: source.min_version.0 as u32,
            data_minor_version: source.min_version.1 as u32,
            name_type: source.type_name.clone(),
        });
    }
    if (target_supported_minor as usize) < source.min_version.1 {
        return Err(StruqtureError::NewVersionMissmatch {
            library_major_version: target_supported_major,
            library_minor_version: target_supported_minor,
            data_major_version: source.min_version.0 as u32,
            data_minor_version: source.min_version.1 as u32,
            name_type: source.type_name.clone(),
        });
    }
    Ok(())
}

/// Trait for implementing a function to determine the minimum supported version of struqture required.
pub trait SerializationSupport {
    /// Returns the minimum version of struqture required to deserialize this object.
    ///
    /// # Returns
    /// (majon_verision, minor_version, patch_version)
    fn min_supported_version(&self) -> (usize, usize, usize) {
        (2, 0, 0)
    }

    /// Returns the StruqtureType of the object.
    ///
    /// # Returns
    /// StruqtureType - The StruqtureType of the object.
    fn struqture_type() -> StruqtureType;

    /// Returns the StruqtureSerialisationMeta of the object.
    fn struqture_serialisation_meta(&self) -> StruqtureSerialisationMeta {
        StruqtureSerialisationMeta {
            type_name: Self::struqture_type().to_string(),
            min_version: self.min_supported_version(),
            version: STRUQTURE_VERSION.to_string(),
        }
    }

    /// Returns the StruqtureSerialisationMeta of the object.
    fn target_serialisation_meta() -> TargetSerialisationMeta {
        TargetSerialisationMeta {
            type_name: Self::struqture_type().to_string(),
            version: STRUQTURE_VERSION.to_string(),
        }
    }
}

/// Helper function to convert a struqture version in semver format to a tuple.
fn semver_to_tuple(version: &str) -> Result<(u32, u32, u32), StruqtureError> {
    let rsplit: Vec<&str> = version.splitn(3, '.').collect();
    if !rsplit.len() == 3 {
        return Err(StruqtureError::GenericError {
            msg: format!("Invalid semver version: {}", version),
        });
    }
    let major_version = u32::from_str(rsplit[0]).map_err(|_| StruqtureError::GenericError {
        msg: format!("Invalid semver version: {}", version),
    })?;
    let minor_version = u32::from_str(rsplit[1]).map_err(|_| StruqtureError::GenericError {
        msg: format!("Invalid semver version: {}", version),
    })?;
    let patch_version: Vec<&str> = rsplit[2].splitn(2, '-').collect();
    let patch_version =
        u32::from_str(patch_version[0]).map_err(|_| StruqtureError::GenericError {
            msg: format!("Invalid semver version: {}", version),
        })?;
    Ok((major_version, minor_version, patch_version))
}

/// Enum of all struqture types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum StruqtureType {
    PauliProduct,
    QubitOperator,
    QubitHamiltonian,
    QubitLindbladNoiseOperator,
    QubitLindbladOpenSystem,
    PlusMinusOperator,
    PlusMinusLindbladNoiseOperator,
    PlusMinusProduct,
    DecoherenceOperator,
    DecoherenceProduct,
    FermionOperator,
    FermionHamiltonian,
    FermionLindbladNoiseOperator,
    FermionLindbladOpenSystem,
    FermionProduct,
    HermitianFermionProduct,
    BosonHamiltonian,
    BosonOperator,
    BosonLindbladNoiseOperator,
    BosonLindbladOpenSystem,
    HermitianBosonProduct,
    BosonProduct,
    MixedOperator,
    MixedHamiltonian,
    MixedLindbladNoiseOperator,
    MixedLindbladOpenSystem,
    MixedPlusMinusOperator,
    MixedDecoherenceProduct,
    MixedProduct,
    HermitianMixedProduct,
    MixedPlusMinusProduct,
}

impl Display for StruqtureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StruqtureType::PauliProduct => write!(f, "PauliProduct"),
            StruqtureType::QubitOperator => write!(f, "QubitOperator"),
            StruqtureType::QubitHamiltonian => write!(f, "QubitHamiltonian"),
            StruqtureType::QubitLindbladNoiseOperator => write!(f, "QubitLindbladNoiseOperator"),
            StruqtureType::QubitLindbladOpenSystem => write!(f, "QubitLindbladOpenSystem"),
            StruqtureType::PlusMinusOperator => write!(f, "PlusMinusOperator"),
            StruqtureType::PlusMinusLindbladNoiseOperator => {
                write!(f, "PlusMinusLindbladNoiseOperator")
            }
            StruqtureType::PlusMinusProduct => write!(f, "PlusMinusProduct"),
            StruqtureType::DecoherenceOperator => write!(f, "DecoherenceOperator"),
            StruqtureType::DecoherenceProduct => write!(f, "DecoherenceProduct"),
            StruqtureType::FermionOperator => write!(f, "FermionOperator"),
            StruqtureType::FermionHamiltonian => write!(f, "FermionHamiltonian"),
            StruqtureType::FermionLindbladNoiseOperator => {
                write!(f, "FermionLindbladNoiseOperator")
            }
            StruqtureType::FermionLindbladOpenSystem => write!(f, "FermionLindbladOpenSystem"),
            StruqtureType::FermionProduct => write!(f, "FermionProduct"),
            StruqtureType::HermitianFermionProduct => write!(f, "HermitianFermionProduct"),
            StruqtureType::BosonHamiltonian => write!(f, "BosonHamiltonian"),
            StruqtureType::BosonOperator => write!(f, "BosonOperator"),
            StruqtureType::BosonLindbladNoiseOperator => write!(f, "BosonLindbladNoiseOperator"),
            StruqtureType::BosonLindbladOpenSystem => write!(f, "BosonLindbladOpenSystem"),
            StruqtureType::HermitianBosonProduct => write!(f, "HermitianBosonProduct"),
            StruqtureType::BosonProduct => write!(f, "BosonProduct"),
            StruqtureType::MixedOperator => write!(f, "MixedOperator"),
            StruqtureType::MixedHamiltonian => write!(f, "MixedHamiltonian"),
            StruqtureType::MixedLindbladNoiseOperator => write!(f, "MixedLindbladNoiseOperator"),
            StruqtureType::MixedLindbladOpenSystem => write!(f, "MixedLindbladOpenSystem"),
            StruqtureType::MixedPlusMinusOperator => write!(f, "MixedPlusMinusOperator"),
            StruqtureType::MixedDecoherenceProduct => write!(f, "MixedDecoherenceProduct"),
            StruqtureType::MixedProduct => write!(f, "MixedProduct"),
            StruqtureType::HermitianMixedProduct => write!(f, "HermitianMixedProduct"),
            StruqtureType::MixedPlusMinusProduct => write!(f, "MixedPlusMinusProduct"),
        }
    }
}
