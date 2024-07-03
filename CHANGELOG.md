# Changelog

This changelog track changes to the struqture project starting at version v1.0.0

## 2.0.0-alpha.1

* Additional changes from feedback regarding struqture 2.0

## 2.0.0-alpha.0

* First draft of the changes for struqture 2.0

## 1.8.0

* Added IDE hint support.

## 1.7.1

* Fixed versioning bug

## 1.7.0

* Updated to pyo3 0.21

## 1.6.2

* Updated VersionMissmatch error message

## 1.6.1

* Updated Cargo.lock (particularly mio 0.8.10->0.8.11)

## 1.6.0

* Add optional feature `indexed_map_iterators` switching internal HashMaps to `indexmap` implementation. Using this feature will change the type of iterators returned by `keys`, `values` and `iter` methods.
* Switching Python interface to using `indexed_map_iterators` by default. This emulates the usual Python behavior of returning the elements of dictionary-like objects in the order of insertion.

## 1.5.2

* Updated to pyo3 0.20

## 1.5.1

* Removed print statement from __init__.py file.

## 1.5.0

* Added remap_modes function to fermionic and bosonic indices for the pyo3 interface.

## 1.4.1

* Added remap_modes function to fermionic and bosonic indices in pure Rust.

## 1.4.0

* Fixed bug in Jordan-Wigner transformation for FermionHamiltonian and FermionHamiltonian.
* Added MixedPlusMinusProduct, MixedPlusMinusOperator to mod.rs in struqture-py/src/mixed_systems (fixed import error).
* Added conversion from QubitHamiltonian(System) to PlusMinusOperator.
* Added support for jsonschema in struqture and struqture-py.

## 1.3.1

* Fixed bug allowing the construction of Hermitian operator products with annihilator index lower than creator index when there are leading equal indices.
* Updated pyo3 dependency to 0.19

## 1.3.0

* Added Jordan-Wigner transform to both struqture and struqture-py.

## 1.2.0

* Added MixedPlusMinusProduct and MixedPlusMinusOperator to both struqture and struqture-py.

## 1.1.1

* Fixed failing group when system and noise have the same number of current spins or modes put one of them has not fixed number of spins/modes.

## 1.1.0

* Added support for sigma +, sigma - and sigma z spin basis

## 1.0.1

* Updated to pyo3 0.18 and test-case 3.0

## 1.0.0

* Added `noise_get` and `system_get` getters for all OpenSystems in python interface
* Added a number of particles check to MixedHamiltonian, MixedSystem and MixedLindbladNoiseSystem
