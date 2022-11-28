# Spins

## Building blocks

All spin objects in struqture are expressed based on products of either Pauli operators (X, Y, Z) or operators suited to express decoherence (X, iY, Z). The products are built by setting the operators acting on separate spins.

### PauliProducts

PauliProducts are combinations of SingleSpinOperators on specific qubits. These are the `SingleSpinOperators`, or Pauli matrices, that are available for PauliProducts:

* I: identity matrix
\\[
I = \begin{pmatrix}
1 & 0\\\\
0 & 1
\end{pmatrix}
\\]

* X: Pauli x matrix
\\[
X = \begin{pmatrix}
0 & 1\\\\
1 & 0
\end{pmatrix}
\\]

* Y: Pauli y matrix
\\[
Y = \begin{pmatrix}
0 & -i\\\\
i & 0
\end{pmatrix}
\\]

* Z: Pauli z matrix
\\[
Z = \begin{pmatrix}
1 & 0\\\\
0 & -1
\end{pmatrix}
\\]

### DecoherenceProducts

DecoherenceProducts are products of a decoherence operators acting on single spins. These `SingleDecoherenceOperators`
are almost identical to the `SinglePauliOperators` with the exception of an additional \\(i\\) factor and are well suited to represent decoherence properties

* I: identity matrix
\\[
\begin{pmatrix}
1 & 0\\\\
0 & 1
\end{pmatrix}
\\]
* X: Pauli X matrix
\\[
\begin{pmatrix}
0 & 1\\\\
1 & 0
\end{pmatrix}
\\]
* iY: Pauli Y matrix multiplied by i
\\[
\begin{pmatrix}
0 & 1 \\\\
-1 & 0
\end{pmatrix}
\\]
* Z: Pauli z matrix
\\[
\begin{pmatrix}
1 & 0\\\\
0 & -1
\end{pmatrix}
\\]

### Examples

In Python the separate operators can be set via functions. In the python interface a PauliProduct can often be replaced by its unique string representation.

```python
from struqture_py.spins import PauliProduct, DecoherenceProduct

# A product of a X acting on spin 0, a Y acting on spin 3 and a Z acting on spin 20
pp = PauliProduct().x(0).y(3).z(20)
# Often equivalent the string representation
pp_string = str(pp)


# A product of a X acting on spin 0, a iY acting on spin 3 and a Z acting on spin 20
dp = DecoherenceProduct().x(0).iy(3).z(20)
# Often equivalent the string representation
dp_string = str(dp)
```

In Rust the user can also import enums for the operators acting on single spins. In rust the equivalent string representation cannot be used in function and method arguments.

```rust
use struqture::prelude::*;
use struqture::spins::{
    DecoherenceProduct, PauliProduct, SingleDecoherenceOperator, SingleSpinOperator,
};

// A product of a X acting on spin 0, a Y acting on spin 3 and a Z acting on spin 20
let pp = PauliProduct::new().x(0).y(3).z(20);
// Constructing with SingleSpinOperator
let pp_equivalent = PauliProduct::new()
    .set_pauli(0, SingleSpinOperator::X)
    .set_pauli(3, SingleSpinOperator::Y)
    .set_pauli(20, SingleSpinOperator::Z);

// A product of a X acting on spin 0, a Y acting on spin 3 and a Z acting on spin 20
let dp = DecoherenceProduct::new().x(0).iy(3).z(20);
// Constructing with SingleSpinOperator
let dp_equivalent = DecoherenceProduct::new()
    .set_pauli(0, SingleDecoherenceOperator::X)
    .set_pauli(3, SingleDecoherenceOperator::IY)
    .set_pauli(20, SingleDecoherenceOperator::Z);
```

## Operators and Hamiltonians

A good example how complex objects are constructed from operator products are `SpinOperators` and `SpinHamiltonians`
(for more information, [see also](../container_types/operators_hamiltonians_and_systems.md)).

These `SpinOperators` and `SpinHamiltonians` represent operators or hamiltonians such as:
\\[
\hat{O} = \sum_{j=0}^N \alpha_j \prod_{k} \sigma^{k}_j \\\\
    \sigma^{k}_j \in \\{ X, Y, Z, I \\}
\\]
where the \\(\sigma^{k}_j\\) are `SinglePauliOperators`.

From a programming perspective the operators and Hamiltonians are HashMaps or Dictionaries with the `PauliProducts` as keys and the coefficients \\(\alpha_j\\) as values.

In struqture we distinguish between spin operators and hamiltonians to avoid introducing unphysical behaviour by accident.
While both are sums over PauliProducts, hamiltonians are guaranteed to be hermitian. In a spin hamiltonian, this means that the prefactor of each `PauliProduct` has to be real.

### Examples

Here is an example of how to build a `SpinOperator` and a `SpinHamiltonian`, in Rust:

```rust
use qoqo_calculator::CalculatorComplex;
use struqture::prelude::*;
use struqture::spins::{PauliProduct, SpinOperator, SpinHamiltonian};

// Building the term sigma^x_0 * sigma^z_2
let pp = PauliProduct::new().x(0).z(2);

// O = (1 + 1.5 * i) * sigma^x_0 * sigma^z_2
let mut operator = SpinOperator::new();
operator.add_operator_product(pp.clone(), CalculatorComplex::new(1.0, 1.5)).unwrap();
assert_eq!(operator.get(&pp), &CalculatorComplex::new(1.0, 1.5));
println!("{}", operator);

// Or when overwriting the previous value
let mut operator = SpinOperator::new();
operator.set(pp.clone(), CalculatorComplex::new(1.0, 1.5)).unwrap();
println!("{}", operator);

// A complex entry is not valid for a SpinHamiltonian
let mut hamiltonian = SpinHamiltonian::new();
// This would fail
hamiltonian.add_operator_product(pp, CalculatorComplex::new(1.0, 1.5)).unwrap();
// This is possible
hamiltonian.add_operator_product(pp, 1.0.into()).unwrap();
println!("{}", hamiltonian);
```

In python, we need to use a `SpinSystem` and `SpinHamiltonianSystem` instead of an`SpinOperator` and `SpinHamiltonian`. See next section for more details.

## Systems and HamiltonianSystems

Following the intention to avoid unphysical behaviour, SpinSystems and SpinHamiltonianSystems are wrappers around SpinOperators and SpinHamiltonians that allow to explicitly set the number of spins of the systems.
When setting or adding a PauliProduct to the systems, it is guaranteed that the spin indices involved cannot exceed the number of spins in the system.
Note that the user can decide to explicitly set the number of spins to be variable.

### Examples

```rust
use qoqo_calculator::CalculatorComplex;
use struqture::prelude::*;
use struqture::spins::{PauliProduct, SpinSystem};

let mut system = SpinSystem::new(Some(3));

// This will work
let pp = PauliProduct::new().x(0).z(2);
system
    .add_operator_product(pp, CalculatorComplex::new(1.0, 1.5))
    .unwrap();
println!("{}", system);

// This will not work, as the spin index of the PauliProduct is larger than
// the number of the spins in the system (the spin with the smallest index is 0).
let pp_error = PauliProduct::new().z(3);
let error = system.add_operator_product(pp_error, CalculatorComplex::new(1.0, 1.5));
println!("{:?}", error);

// This will work because we leave the number of spins dynamic
let mut system = SpinSystem::new(None);
system
    .add_operator_product(PauliProduct::new().z(3), CalculatorComplex::new(1.0, 1.5))
    .unwrap();
```

The equivalent code in python:

```python
from qoqo_calculator_pyo3 import CalculatorComplex
from struqture_py import spins

system = spins.SpinSystem(3)

# This will work
pp = spins.PauliProduct().x(0).z(2)
system.add_operator_product(pp, CalculatorComplex.from_pair(1.0, 1.5))
print(system)

# This will not work, as the spin index of the PauliProduct is larger
# than the number of the spins in the system (the spin with the smallest index is 0).
pp_error = spins.PauliProduct().z(3)
value = CalculatorComplex.from_pair(1.0, 1.5)
# system.add_operator_product(pp_error, value)  # Uncomment me!


# This will work because we leave the number of spins dynamic
system = spins.SpinSystem()
system.add_operator_product(spins.PauliProduct().z(3), 1.0)
```

## Noise operators and systems

We describe decoherence by representing it with the Lindblad equation.
The Lindblad equation is a master equation determining the time evolution of the density matrix.
It is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) =-i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]
with the rate matrix \\(\Gamma_{j,k}\\) and the Lindblad operator \\(L_{j}\\).

To describe the pure noise part of the Lindblad equation one needs the rate matrix in a well defined basis of Lindblad operators.
We use `DecoherenceProducts` as the operator base. To describe spin noise we use the Lindblad equation with \\(\hat{H}=0\\).

The rate matrix and with it the Lindblad noise model is saved as a sum over pairs of `DecoherenceProducts`, giving the operators acting from the left and right on the density matrix.
In programming terms the object `SpinLindbladNoiseOperator` is given by a HashMap or Dictionary with the tuple (`DecoherenceProduct`, `DecoherenceProduct`) as keys and the entries in the rate matrix as values.

Similarly to SpinOperators, SpinLindbladNoiseOperators have a system equivalent: `SpinLindbladNoiseSystem`, with a number of involved spins defined by the user. For more information on these, see the [noise container](../container_types/noise_operators_and_systems) chapter.

### Examples

Here, we add the term \\(\sigma^{x}_0 \sigma^{z}_2\\) with coefficient 1.0: \\(\hat{H} = 1.0 * \sigma^{x}_0 \sigma^{z}_2\\)

```rust
use struqture::prelude::*;
use struqture::spins::{DecoherenceProduct, SpinLindbladNoiseSystem};

let mut system = SpinLindbladNoiseSystem::new(Some(3));

let dp = DecoherenceProduct::new().x(0).z(2);

system.add_operator_product((dp.clone(), dp.clone()), 1.0.into()).unwrap();
assert_eq!(system.get(&(dp.clone(), dp)), &CalculatorComplex::new(1.0, 0.0));
println!("{}", system);
```

The equivalent code in python:

```python
from struqture_py import spins

system = spins.SpinLindbladNoiseSystem(3)

dp = spins.DecoherenceProduct().x(0).z(2)

system.add_operator_product((dp, dp), 1.0+1.5*1j)
print(system)

# In python we can also use the string representation
system = spins.SpinLindbladNoiseSystem(3)
system.add_operator_product((str(dp), str(dp)), 1.0+1.5*1j)
print(system)
```

## Open systems

Physically open systems are quantum systems coupled to an environment that can often be described using Lindblad type of noise.
The Lindblad master equation is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) =-i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]
In struqture they are composed of a hamiltonian (SpinHamiltonianSystem) and noise (SpinLindbladNoiseSystem). They have different ways to set terms in Rust and Python:

### Examples

```rust
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use struqture::prelude::*;
use struqture::spins::{DecoherenceProduct, PauliProduct, SpinLindbladOpenSystem};

let mut open_system = SpinLindbladOpenSystem::new(Some(3));

let pp = PauliProduct::new().z(1);
let dp = DecoherenceProduct::new().x(0).z(2);

let system = open_system.system_mut();
system.add_operator_product(pp, CalculatorFloat::from(2.0)).unwrap();

let noise = open_system.noise_mut();
noise
    .add_operator_product((dp.clone(), dp), CalculatorComplex::new(1.0, 0.0))
    .unwrap();

println!("{}", open_system);
```

The equivalent code in python:

```python
from qoqo_calculator_pyo3 import CalculatorComplex, CalculatorFloat
from struqture_py import spins

open_system = spins.SpinLindbladOpenSystem(3)

pp = spins.PauliProduct().z(1)
dp = spins.DecoherenceProduct().x(0).z(2)

open_system.system_add_operator_product(pp, CalculatorFloat(2.0))
open_system.noise_add_operator_product(
    (dp, dp), CalculatorComplex.from_pair(0.0, 1.0))

print(open_system)
```

## Matrix representation: spin objects only

All spin-objects can be converted into sparse matrices with the following convention.
If \\(M_2\\) corresponds to the matrix acting on spin 2 and \\(M_1\\) corresponds to the matrix acting on spin 1 the total matrix \\(M\\) acting on spins 0 to 2 is given by
\\[
    M = M_2 \otimes M_1 \otimes \mathbb{1}
\\]
For an \\(N\\)-spin system an operator acts on the \\(2^N\\) dimensional space of state vectors.
A superoperator operates on the \\(4^N\\) dimensional space of flattened density-matrices.
struqture uses the convention that density matrices are flattened in row-major order
\\[
    \rho = \begin{pmatrix} a & b \\\\ c & d \end{pmatrix} => \vec{\rho} = \begin{pmatrix} a \\\\ b \\\\ c \\\\ d \end{pmatrix}
\\]
For noiseless objects (SpinSystem, SpinHamiltonianSystem), sparse operators and sparse superoperators can be constructed, while only sparse superoperators can be constructed for systems with noise (SpinLindbladNoiseSystem, SpinLindbladOpenSystem).

Note that the matrix representation functionality exists only for spin objects, and can't be generated for bosonic, fermionic or mixed system objects.

```rust
use qoqo_calculator::CalculatorComplex;
use struqture::prelude::*;
use struqture::spins::{DecoherenceProduct, SpinLindbladNoiseSystem};

let mut system = SpinLindbladNoiseSystem::new(Some(3));

let dp = DecoherenceProduct::new().x(0).z(2);

system
    .add_operator_product((dp.clone(), dp), CalculatorComplex::new(1.0, 0.0))
    .unwrap();

let matrix = system.sparse_matrix_superoperator(Some(3)).unwrap();
println!("{:?}", matrix);
```

The equivalent code in python:

```python
from qoqo_calculator_pyo3 import CalculatorComplex
from struqture_py import spins
from scipy.sparse import coo_matrix

system = spins.SpinLindbladNoiseSystem(3)

dp = spins.DecoherenceProduct().x(0).z(2)
system.add_operator_product((dp, dp), CalculatorComplex.from_pair(1.0, 1.5))
# Using the `sparse_matrix_superoperator_coo` function, you can also
# return the information in scipy coo_matrix form, which can be directly fed in:
python_coo = coo_matrix(system.sparse_matrix_superoperator_coo())
print(python_coo.todense())
```
