# Introduction

Struqture is a Rust (struqture) and Python (struqture-py) library by [HQS Quantum Simulations](https://quantumsimulations.de/) to represent quantum mechanical operators, Hamiltonians and open quantum systems.
The library supports building [spin](physical_types/spins.md) objects, [fermionic](physical_types/fermions.md) objects, [bosonic](physical_types/bosons.md) objects and [mixed system](physical_types/mixed_systems.md) objects that contain arbitrary many spin, fermionic and bosonic subsystems.

Struqture has been developed to create and exchange definitions of operators, Hamiltonians and open systems. A special focus is the use as input to quantum computing simulation software.

To best support this use case, struqture has a number of design goals:

* Support for arbitrary spin, bosonic, fermionic and mixed systems
* Full serialisation support to json and other formats
* Preventing construction of unphysical objects by using well defined types for all objects in struqture
* Support of symbolic values in operators, Hamiltonians and open systems

Following these design goals, we prioritize using distinctive types to construct objects over a less verbose syntax.
Similarly the support of symbolic expression leads to a trade-off in speed compared to an implementation using only floating point values.
The symbolic expression support is achieved by using CalculatorComplex and CalculatorFloat values instead of complex and float values (respectively), which are imported frrom [qoqo_calculator] (https://github.com/HQSquantumsimulations/qoqo_calculator).
Struqture is designed to also support the construction and (de)serialisation of large systems but for the use in numeric algorithms we recommend transforming Operators and Hamiltonians into a sparse matrix form.

This documentation is split into two parts. The [first part](physical_types/intro.md) covers the basic usage for spins, bosons, fermions and mixed systems. The [second part](container_types/intro.md) covers the shared design patterns between spins, bosons, fermions and mixed systems. A real-world [example](example.md) is also included in.

Note: the package will be faster in Rust than Python, as Rust is a compiled language. This should only make a big difference, however, if you are performing hundreds of multiplication operations and a large amount of getter/setter calls. 

## Installation

### Python

You can install `struqture_py` from PyPi. For x86 Linux, Windows and macOS systems pre-built wheels are available.
On other platforms a local Rust toolchain is required to compile the Python source distribution.

```bash
pip install struqture-py
```

### Rust

You can use struqture in your Rust project by adding 

```TOML
struqture = { version = "1.0.1" }
```

 to your Cargo.toml file.

## API Documentation

This user documentation is intended to give a high level overview of the design and usage of struqture. For a full list of the available data types and functions see the API-Documentaions of [struqture](https://docs.rs/struqture/) and [struqture-py](https://hqsquantumsimulations.github.io/struqture/pyapi/index.html).
