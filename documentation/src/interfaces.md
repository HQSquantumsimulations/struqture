# Interface to Qutip

Struqture has an interface to the [QuTiP](https://qutip.org/) package that can be used to transform struqture spin objects to qutip objects for simulation purposes. 

It is a separate package from struqture that can be installed with:

```bash
pip install struqture-qutip-interface
```

More information can be found on the [struqture-qutip-interface github page](https://github.com/HQSquantumsimulations/struqture-qutip-interface/tree/main)

# Interface to OpenFermion

Struqture also has an interface to the [OpenFermion](https://quantumai.google/openfermion) package, allowing users to switch from one package to the other.

OpenFermion is an open-source library for compiling and analyzing quantum algorithms to simulate fermionic systems, including quantum chemistry. Among other functionalities, this version features data structures and tools for obtaining and manipulating representations of fermionic and qubit Hamiltonians.
This interface is aimed at any user already using openfermion who wants to create and use Hamiltonians with struqture. It can also be of use to users using struqture to define Hamiltonians and want to use [cirq](https://quantumai.google/cirq), for which OpenFermion formalism is needed.

For now only the conversion to and from `PauliHamiltonian` is implemented with the functions `struqture_to_openfermion` and `openfermion_to_struqture`.

