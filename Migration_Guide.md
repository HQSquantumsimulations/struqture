# Migration guide: struqture 1.0 -> struqture 2.0

Struqture 2.0 changes:

1) Changes to naming conventions:

* We have renamed the `System` and `HamiltonianSystem` objects to `Operator` and `Hamiltonian` objects. In struqture 1.0, the `System` object was internally composed of an operator and an associated number of spins or modes. We removed the `System` layer, and now just use the underlying `Operator`/`Hamiltonian` objects. **NOTE**: This layering was not present for the `LindbladOpenSystem` objects, as they are coherent part with a decoherent part. They have therefore not been renamed, also as we commonly refer to "open systems", and not "open operators".
* The spin objects have been renamed, to reflect that they are representations of **qubit** operators rather than **normalised spin** operators. In the names of the objects, the `Spin` part has been replaced with `Pauli`.

These changes are summarised below:

| struqture 1.0 name | struqture 2.0 name |
| ------------------ | ------------------ |
| BosonSystem | BosonOperator |
| BosonHamiltonianSystem | BosonHamiltonian |
| BosonLindbladNoiseSystem | BosonLindbladNoiseOperator |
| FermionSystem | FermionOperator |
| FermionHamiltonianSystem | FermionHamiltonian |
| FermionLindbladNoiseSystem | FermionLindbladNoiseOperator |
| MixedSystem | MixedOperator |
| MixedHamiltonianSystem | MixedHamiltonian |
| MixedLindbladNoiseSystem | MixedLindbladNoiseOperator |
| SpinSystem | PauliOperator |
| SpinHamiltonianSystem | PauliHamiltonian |
| SpinLindbladNoiseSystem | PauliLindbladNoiseOperator |
| SpinLindbladOpenSystem | PauliLindbladOpenSystem |

No other objects have been renamed.

2) Removed or deprecated functionality

* The struqture 2.0 `Operator` and `Hamiltonian` objects do not have a maximum number of particles, and therefore don't take any input in their creation. This means that, for instance, what was previously:
```python
# struqture 1.0
fermionic_operator = FermionSystem(4)
fermionic_hamiltonian = FermionHamiltonianSystem(4)
fermionic_noisy = FermionLindbladNoiseSystem(4)
```
is now:
```python
# struqture 2.0
fermionic_operator = FermionOperator()
fermionic_hamiltonian = FermionHamiltonian()
fermionic_noisy = FermionLindbladNoiseOperator()
```
The exact same holds for the bosonic objects and spin objects.

**NOTE**: For mixed objects, the above changes mean that the initialisation function no longer needs to know how many spins or modes are in each subsystem of the mixed object. However, it does need to know how many subsystems there are in the mixed object. Here is an example:
```python
# struqture 1.0 
hamiltonian = MixedHamiltonianSystem([3, 2], [], [])
```
is now:
```python
# struqture 2.0
hamiltonian = MixedHamiltonian(2, 0, 0)
```

* The `number_modes`, `number_bosonic_modes` and `number_fermionic_modes` have been removed, in favour of the `current_number_modes`, `current_number_bosonic_modes` and `current_number_fermionic_modes` functions. These functions now return the same result, as there is no maximum index set.

* The `number_spins` function has been deprecated, in favour of the `current_number_spins` function. These functions now return the same result, as there is no maximum index set. The `number_spins` function will be removed in a future version of struqture 2.0, but has been left in as deprecated for now, in order to minimise code breakage.

3) Added functionality

* The conversion error message should now be much clearer, and there should no longer be any mistaken conversions from python to rust, as we have re-designed parts of the (de)serialisation.

* We have added a conversion function, to convert a struqture 1.0 object to a struqture 2.0 object: `from_json_struqture_1`. This function ensures users need not re-generate their json files. See the code snippet below.

```python
# given a struqture 1.0 FermionicHamiltonianSystem **json** object "ham_json_struq_1", the following code will return a struqture 2.0 object
hamiltonian = FermionHamiltonian().from_json_struqture_1(ham_json_struq_1)
```

* Additionally, as of struqture 1.9, there are conversion functions to convert from struqture 2.0 to struqture 1.0 objects, which can be used analogously to the ones in the code snippet above. Should you wish to use them, you will need to install struqture manually, using [maturin](https://github.com/PyO3/maturin). When running maturin, please ensure you are installing struqture 1.9 or higher, using the `--features` flag with the following feature specified: `unstable_struqture_2_import`.