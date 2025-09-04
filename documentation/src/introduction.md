# Introduction

Struqture is a Rust library with a Python interface `struqture-py` by [HQS Quantum Simulations](https://quantumsimulations.de/) to define and store Hamiltonians, quantum mechanical operators and open systems.
The library supports building [spin](physical_types/spins/intro.md) objects, as well as other degrees of freedom.

Struqture has been developed to create and exchange definitions of Hamiltonians and operators. A special focus is the definition of large quantum systems as an input to quantum simulation. Therefore, it is strongly built around symbolic definition, wherein the user defines their Hamiltonian (for instance) as they would do so by writing it down.

<!-- TODO: modify figure to have a hand-written H top left and struqture representation bottom right. -->
 <img src="./images/docu_graphic.png" alt="struqture" width="90%">

## Advantages of struqture

Compared with Qiskit and QuTiP, struqture uses a sparse, human‑readable operator notation that records only non‑identity factors. We show an example for a 12-spin system below.

### Compared to Qiskit

We define an operator acting on a 12-spin system in struqture:
```python
operator.set("0X12X", 1.0)
```
This compares to a definition of the same term in Qiskit, which is written as 
```python
SparsePauliOp("XIIIIIIIIIIIX")
```

### Compared to QuTiP

If we define the same operator shown above in QuTiP, it would be built as follows. 
```python
tensor(sigmax(), qeye(2), qeye(2), qeye(2), qeye(2), qeye(2), qeye(2), qeye(2), qeye(2), qeye(2), qeye(2), sigmax())
```

Please note that this will built the full, dense matrix for 12 spins, which will be slow to handle.

By keeping operators symbolic and not storing full matrices, struqture scales to Hamiltonians with far more sites; when needed, it can generate the (super)operator matrix on demand, whereas QuTiP tracks matrix representations by default.

### Parametrization of operators

Additionally, struqture allows for native symbolic parameters, so the user can define a [parameterized Hamiltonian](./physical_types/spins/symbolic.md) once and substitute numerical values later.

## Installation

### Python

You can install `struqture_py` from PyPi. For Linux, Windows and macOS systems pre-built wheels are available.

```bash
pip install struqture-py
```

### Rust

You can use `struqture` in your Rust project by adding 

```TOML
struqture = { version = "2.0.1" }
```

 to your Cargo.toml file.

## API Documentation

This user documentation is intended to give a high level overview of the design and usage of struqture. For a full list of the available data types and functions see the API-Documentation of [struqture-py](https://hqsquantumsimulations.github.io/struqture/python_api_docs/generated/struqture_py.html) and [struqture](https://docs.rs/struqture/).
