# How to use struqture

In this part of the user documentation we show the basic usage of operators, Hamiltonians and open systems for spins, bosons, fermions and mixed systems.
Stuqture is designed with the same patterns to construct objects across all classes, for ease of use.

## Getting started

The documentation is split up by type of operators the user would like to create:

* [spins](./spins.md)
* [bosons](./bosons.md)
* [fermions](./fermions.md)
* [mixed systems](./mixed_systems.md)

## A note on symbolic parameters

For all operators, Hamiltonians and open systems in struqture, the user can set (key, value) inputs. For instance, in a `BosonOperator`, the user adds in `BosonProduct` terms (keys) with their complex prefactors (values). These values, regardless of the struqture object, can be either a number (float or complex, depending on the operator) or a string. We refer to this as a "symbolic parameter". This can be a great advantage to a more advanced user, who wishes to create, for instance, a Hamiltonian with a varying parameter. By encoding this parameter as a symbolic parameter, the user can replace this parameter with a new value when iterating through the list of values for the varying parameter, rather than having to create a new Hamiltonian at each step in the iteration.
