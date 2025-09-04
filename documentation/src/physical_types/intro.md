# How to use struqture objects

Struqture objects may be instantiated directly by the user or imported from a file or database. This section of the documentation explains both approaches and directs you to the remainder of the user guide once you are comfortable with them.

## Creating your own Hamiltonians and operators

To create your own Hamiltonian or operator, simply import the corresponding python class from the `spins` module. For instance, to import an object corresponding to a Hamiltonian composed of spins, which can be represented by Pauli matrices, run: 
```python
from struqture_py.spins import PauliHamiltonian
```
In the [spins](spins/intro.md) section of the documentation we will explore the various classes in the `spins` module.

## Using a struqture Hamiltonian from a file

Struqture objects can be stored as either [`JSON`](https://www.json.org/json-en.html) files or as binary code. We highly recommend using `JSON` files, as they are also human-readable.
Struqture was designed not only to easily instantiate or modify objects, but also to easily transfer them to other users - this is where the `JSON` serialisation comes into play. 

For instance, given a file storing our Hamiltonian, `hamiltonian.json`, we can import it in the following way:
```python
from struqture_py.spins import PauliHamiltonian

# Reading a file: from_json
with open("hamiltonian.json", "r") as f:
    hamiltonian = PauliHamiltonian.from_json(f.read())

print(hamiltonian)

# Writing to a file: to_json
# Should you wish to perform the inverse operation (writing your Hamiltonian to a file), you can
# do so by running the following lines of code:
with open("hamiltonian.json", "w") as f:
    f.write(hamiltonian.to_json())
```

Once a struqture object (e.g. a Hamiltonian or operator) has been loaded from a file, it can be used in the same manner as one created by the user. Refer to the remaining documentation for the available getters, setters, and properties.

## Using a struqture Hamiltonian from a database

In this part of the user documentation we show how to work with a user-selected pre-defined Hamiltonian from our database, e.g. [nmr_database](https://docs.cloud.quantumsimulations.de/hqs-spectrum-tools/subfolder/components/molecule_input/molecular_data.html).

For instance, this code snippet shows how to load the `C6H5NO2` molecule from the `hqs_nmr_parameters` database, and turn it into a struqture object:
```python
from hqs_nmr_parameters import examples, nmr_hamiltonian

parameters = examples.molecules["C6H5NO2"]
hamiltonian = nmr_hamiltonian(parameters=parameters, field=2.5)
```
Once a struqture object (e.g. a Hamiltonian or operator) has been loaded from a databse, it can be used in the same manner as one created by the user. Refer to the remaining documentation for the available getters, setters, and properties.
