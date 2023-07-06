# Changelog

This changelog track changes to the struqture project starting at version v1.0.0

## 1.3.1

* Fixed bug allowing the construction of Hermitian operator products with annihilator index lower than creator index when there are leading equal indices.

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
* Added a number of particles check to MixedHamiltonianSystem, MixedSystem and MixedLindbladNoiseSystem