# Changelog

This changelog track changes to the struqture project starting at version v1.0.0

## Unreleased

## 2.4.1

* Fixed the documentation

## 2.4.0

* Added an OpenFermion interface.
* Updated minimum supported Python version to 3.10.

## 2.3.2

* Improved documentation.

## 2.3.1

* Fixed link in the user documentation.

## 2.3.0

* Improved applied examples in user documentation.
* Updated user documentation to be more beginner friendly
* Added pretty print function

## 2.2.4

* Clarified user documentation by removing references to qoqo_calculator_pyo3 in the examples and added a section on how struqture differs to other tools.

## 2.2.3

* Updated struqture 1 dependency to 1.13.

## 2.2.2

* Added tilde `~` for struqture-py-macros dependency of struqture-py.

## 2.2.1

* Updated to qoqo-calculator 1.7.1, jsonschema 0.32, nalgebra 0.34, criterion 0.7.
* Stardardised versioning by removing tildes `~`.

## 2.2.0

* Updated to pyo3 0.25.
* Updated to schemars 1.0.
* Updated to bincode 2.0.

## 2.1.1

* Fixed links in user documentation introduction.
* Fixed API documentation example snippets.

## 2.1.0

* Added boson to spin mapping for BosonLindbladNoiseOperator.
* Added `direct_boson_spin_mapping` and `direct_boson_spin_mapping` functions for `BosonHamiltonian` and `BosonProduct` in both struqture and struqture-py.
* Removed unused errors from lib.rs and fixed typo in error name in lib.rs.

## 2.0.1

* Updated to pyo3 0.24.
* Fixed links in READMEs.

## 2.0.0

* Please see the [Migration Guide](./Migration_Guide.md) for an overview of the changes for the end-user (Python).

### Renamed in 2.0.0

* Renamed all `Spin` objects to `Pauli`.
* Renamed all `System` objects to `Operator`.
* Renamed `to_mixed_system` and `from_mixed_system` to `to_mixed_operator` and `from_mixed_operator`.
* Renamed `to_spin`/`from_spin` functions to `to_pauli`/`from_pauli`.

### Changed in 2.0.0

* Made the `number_spins` field in the `sparse_matrix` methods not optional.
* Removed the `System` layer: we now only use the `Operator` objects. **NOTE**: This means we now longer have a maximum number of particles required when instantiating a struct.
* Fixed a bug when creating a Product from a bad JSON.
* Changed the `cmp` method of `PauliProduct` to use the size of the product, then the qubit index and then the Pauli terms.
* Added links to examples in container types of the user documentation
* Removed `sparse_lindblad_entries` and `unitary_sparse_matrix_coo` functions.
* Removed the `separate_into_n_terms` function from all objects except the `FermionHamiltonian` and `PauliNoiseOperator` structs.
* Fixed documentation folder structure.

### Updated in 2.0.0

* Major user documentation update.
* Updated to pyo3 0.23 (and qoqo_calculator 1.5.0). Now with support for python 3.13 (pyo3 0.22 update).
* Updated dependencies: jsonschema (0.18 -> 0.28), ndarray (0.15 -> 0.16), thiserror (1.0 -> 2.0), itertools (0.13 -> 0.14), qoqo-calculator (1.2 -> 1.3).
* Updated minimum supported Rust version from 1.57 to 1.76.
* Updated minimum supported Python version from 3.8 to 3.9.


## 1.12.2

* Added a readme in struqture-py.
* Updated the `deny.toml` to the cargo-deny 1.18 standard.

## 1.12.1

* Updated to struqture 2.0 v2.0.0-alpha.11.
* Moved the struqture 2.0 conversion code from struqture to struqture-py, thereby removing the struqture 2.0 dependency of struqture.

## 1.12.0

* Updated to pyo3 0.23 (includes updating to qoqo-calculator 1.5.0 and struqture 2.0.0-alpha.10).
* Updated to new struqture 2.0 naming (Qubit -> Pauli).
* Switched from `from_struqture_2` to `from_json_struqture_2` in the `unstable_struqture_2_import` feature.
* Added qoqo/.cargo/config file with aarch64 and x86_64 targets for macos.

## 1.11.1

* Updated to struqture 2.0.0-alpha.7.

## 1.11.0

* Updated dependencies: jsonschema (0.18 -> 0.28), ndarray (0.15 -> 0.16), thiserror (1.0 -> 2.0), itertools (0.13 -> 0.14).
* Updated minimum supported Rust version from 1.57 to 1.76.
* Updated minimum supported Python version from 3.8 to 3.9.

## 1.10.1

* Fixed a build issue in 1.10.0.

## 1.10.0

* Updated to pyo3 0.22 and python 3.13.

## 1.9.2

* Fixed a bug when creating a Product from a bad JSON.

## 1.9.0 - 1.9.1

* Added methods to convert from struqture 2.0.0-alpha.3

## 1.8.0

* Added IDE hint support.

## 1.7.1

* Fixed versioning bug.

## 1.7.0

* Updated to pyo3 0.21.

## 1.6.2

* Updated VersionMissmatch error message.

## 1.6.1

* Updated Cargo.lock (particularly mio 0.8.10->0.8.11).

## 1.6.0

* Add optional feature `indexed_map_iterators` switching internal HashMaps to `indexmap` implementation. Using this feature will change the type of iterators returned by `keys`, `values` and `iter` methods.
* Switching Python interface to using `indexed_map_iterators` by default. This emulates the usual Python behavior of returning the elements of dictionary-like objects in the order of insertion.

## 1.5.2

* Updated to pyo3 0.20.

## 1.5.1

* Removed print statement from __init__.py file.

## 1.5.0

* Added remap_modes function to fermionic and bosonic indices for the pyo3 interface.

## 1.4.1

* Added remap_modes function to fermionic and bosonic indices in pure Rust.

## 1.4.0

* Fixed bug in Jordan-Wigner transformation for FermionHamiltonian and FermionHamiltonian.
* Added MixedPlusMinusProduct, MixedPlusMinusOperator to mod.rs in struqture-py/src/mixed_systems (fixed import error).
* Added conversion from SpinHamiltonian(System) to PlusMinusOperator.
* Added support for jsonschema in struqture and struqture-py.

## 1.3.1

* Fixed bug allowing the construction of Hermitian operator products with annihilator index lower than creator index when there are leading equal indices.
* Updated pyo3 dependency to 0.19.

## 1.3.0

* Added Jordan-Wigner transform to both struqture and struqture-py.

## 1.2.0

* Added MixedPlusMinusProduct and MixedPlusMinusOperator to both struqture and struqture-py.

## 1.1.1

* Fixed failing group when system and noise have the same number of current spins or modes put one of them has not fixed number of spins/modes.

## 1.1.0

* Added support for sigma +, sigma - and sigma z spin basis.

## 1.0.1

* Updated to pyo3 0.18 and test-case 3.0.

## 1.0.0

* Added `noise_get` and `system_get` getters for all OpenSystems in python interface.
* Added a number of particles check to MixedHamiltonian, MixedSystem and MixedLindbladNoiseSystem.
