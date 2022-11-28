# Fermions

## Building blocks

All fermionic objects in struqture are expressed based on products of fermionic creation and annihilation operators, which respect fermionic anti-commutation relations
\\[ \lbrace c_k^{\dagger}, c_j^{\dagger} \rbrace = 0, \\\\
    \lbrace c_k, c_j \rbrace = 0, \\\\
    \lbrace c_k^{\dagger}, c_j \rbrace = \delta_{k, j}. \\]

### FermionProducts

FermionProducts are simple combinations of fermionic creation and annihilation operators.

### HermitianFermionProducts

HermitianFermionProducts are the hermitian equivalent of FermionProducts. This means that even though they are constructed the same (see the next section, `Examples`), they internally store both that term and its hermitian conjugate. For instance, given the term \\(c^{\dagger}_0 c_1 c_2\\), a FermionProduct would represent \\(c^{\dagger}_0 c_1 c_2\\) while a HermitianFermionProduct would represent \\(c^{\dagger}_0 c_1 c_2 + c^{\dagger}_2 c^{\dagger}_1 c_0\\).

### Examples

In both Python and Rust, the operator product is constructed by passing an array or a list of integers to represent the creation indices, and an array or a list of integers to represent the annihilation indices.

Note: (Hermitian)FermionProducts can only been created from the correct ordering of indices (the wrong sequence will return an error) but we have the `create_valid_pair` function to create a valid Product from arbitrary sequences of operators which also transforms an index value according to the anti-commutation and hermitian conjugation rules.

```python
from struqture_py.fermions import FermionProduct, HermitianFermionProduct
from qoqo_calculator_pyo3 import CalculatorComplex

# A product of a creation operator acting on fermionic mode 0 and an
# annihilation operator acting on fermionic mode 20
fp = FermionProduct([0], [20])
# Building the term c^{\dagger}_1 * c^{\dagger}_3 * c_0
fp = FermionProduct.create_valid_pair(
    [3, 1], [0], CalculatorComplex.from_pair(1.0, 0.0))


# A product of a creation operator acting on fermionic mode 0 and an annihilation
# operator acting on fermionic mode 20, as well as a creation operator acting on
# fermionic mode 20 and an annihilation operator acting on fermionic mode 0
hfp = HermitianFermionProduct([0], [20])
# Building the term c^{\dagger}_0 * c^{\dagger}_3 * c_0 + c^{\dagger}_0 * c_3 * c_0
hfp = HermitianFermionProduct.create_valid_pair(
    [3, 0], [0], CalculatorComplex.from_pair(1.0, 0.0))
```

In rust the equivalent string representation cannot be used in function and method arguments.

```rust
use qoqo_calculator::CalculatorComplex;
use struqture::fermions::{FermionProduct, HermitianFermionProduct};
use struqture::prelude::*;

// Building the term c^{\dagger}_0 c_20
let fp_0 = FermionProduct::new([0], [20]).unwrap();
// Building the term c^{\dagger}_1 * c^{\dagger}_3 * c_0
let (fp_1, coeff) = FermionProduct::create_valid_pair(
    [3, 1], [0], CalculatorComplex::from(1.0)).unwrap();


// A product of a creation operator acting on fermionic mode 0 and an annihilation
// operator acting on fermionic mode 20, as well as a creation operator acting on
// fermionic mode 20 and an annihilation operator acting on fermionic mode 0
let fp_0 = HermitianFermionProduct::new([0], [20]).unwrap();
// Building the term c^{\dagger}_0 * c^{\dagger}_3 * c_0 + c^{\dagger}_0 * c_3 * c_0
let (fp_1, coeff) = HermitianFermionProduct::create_valid_pair(
    [3, 0], [0], CalculatorComplex::from(1.0)).unwrap();
```

## Operators and Hamiltonians

Complex objects are constructed from operator products are `FermionOperators` and `FermionHamiltonians`
(for more information, [see also](../container_types/operators_hamiltonians_and_systems.md)).

These `FermionOperators` and `FermionHamiltonians` represent operators or hamiltonians such as:
\\[ \hat{H} = \sum_{j=0}^N \alpha_j \prod_{k, l} c_{k, j}^{\dagger} c_{l,j}  \\]
with 
\\(c^{\dagger}\\) the fermionionic creation operator, \\(c\\) the fermionionic annihilation operator
\\[ \lbrace c_k^{\dagger}, c_j^{\dagger} \rbrace = 0, \\\\
    \lbrace c_k, c_j \rbrace = 0, \\\\
    \lbrace c_k^{\dagger}, c_j \rbrace = \delta_{k, j}. \\]

For instance, \\(c^{\dagger}_0 c^{\dagger}_1 c_1\\) is a term with a \\(c^{\dagger}\\) term acting on 0, and both a \\(c^{\dagger}\\) term and a \\(c\\) term acting on 1.

From a programming perspective the operators and Hamiltonians are HashMaps or Dictionaries with `FermionProducts` or `HermitianFermionProducts` (respectively) as keys and the coefficients \\(\alpha_j\\) as values. 

In struqture we distinguish between fermionic operators and hamiltonians to avoid introducing unphysical behaviour by accident.
While both are sums over normal ordered fermionic products (stored as HashMaps of products with a complex prefactor), hamiltonians are guaranteed to be hermitian to avoid introducing unphysical behaviour by accident. In a fermionic hamiltonian, this means that the sums of products are sums of hermitian fermionic products (we have not only the \\(c^{\dagger}c\\) terms but also their hermitian conjugate) and the on-diagonal terms are required to have real prefactors. We also require the smallest index of the creators to be smaller than the smallest index of the annihilators.

### Examples

Here is an example of how to build a product and using it to build an operator, in Rust:
```rust
use qoqo_calculator::CalculatorComplex;
use struqture::prelude::*;
use struqture::fermions::{
    FermionProduct, FermionOperator, HermitianFermionProduct, FermionHamiltonian
};

// Building the term c^{\dagger}_1 * c^{\dagger}_2 * c_0 * c_1
let fp = FermionProduct::new([1, 2], [0, 1]).unwrap();

// O = (1 + 1.5 * i) * c^{\dagger}_1 * c^{\dagger}_2 * c_0 * c_1
let mut operator = FermionOperator::new();
operator.add_operator_product(fp.clone(), CalculatorComplex::new(1.0, 1.5)).unwrap();
println!("{}", operator);

// Or when overwriting the previous value
let mut operator = FermionOperator::new();
operator.set(fp.clone(), CalculatorComplex::new(1.0, 1.5)).unwrap();
println!("{}", operator);

// A FermionProduct entry is not valid for a FermionHamiltonian
let mut hamiltonian = FermionHamiltonian::new();
// This would fail, as it uses HermitianFermionProducts, not FermionProducts
hamiltonian.add_operator_product(fp, CalculatorComplex::new(1.0, 1.5)).unwrap();
// This is possible
let hfp = HermitianFermionProduct::new([0, 2], [0, 1]).unwrap();
hamiltonian.add_operator_product(hfp, CalculatorComplex::new(1.5, 0.0)).unwrap();
println!("{}", hamiltonian);
```

In python, we need to use a `FermionSystem` and `FermionHamiltonianSystem` instead of a `FermionOperator` and `FermionHamiltonian`. See next section for more details.

## Systems and HamiltonianSystems

Following the intention to avoid unphysical behaviour, FermionSystems and FermionHamiltonianSystems are wrappers around FermionOperators and FermionHamiltonians that allow to explicitly set the number of spins of the systems.
When setting or adding a FermionProduct/HermitianFermionProduct to the systems, it is guaranteed that the fermionic indices involved cannot exceed the number of fermionic modes in the system.
Note that the user can decide to explicitly set the number of fermionic modes to be variable.

### Examples

```rust
use qoqo_calculator::CalculatorComplex;
use struqture::prelude::*;
use struqture::fermions::{HermitianFermionProduct, FermionHamiltonianSystem};

let mut system = FermionHamiltonianSystem::new(Some(3));

// This will work
let hfp = HermitianFermionProduct::new([0, 1], [0, 2]).unwrap();
system.add_operator_product(hfp, CalculatorComplex::new(1.0, 1.5)).unwrap();
println!("{}", system);

// This will not work, as the fermionic index of the HermitianFermionProduct is larger
// than the number of the fermionic modes in the system (the fermionic mode with the
// smallest index is 0).
let hfp_error = HermitianFermionProduct::new([3], [3]).unwrap();
let error = system.add_operator_product(hfp_error, CalculatorComplex::new(1.0, 1.5));
println!("{:?}", error);

// This will work because we leave the number of spins dynamic
let hbf = HermitianFermionProduct::new([0, 1], [0, 2]).unwrap();
let mut system = FermionHamiltonianSystem::new(None);
system.add_operator_product(hbf, CalculatorComplex::new(1.0, 1.5)).unwrap();
```

The equivalent code in python:
```python
from qoqo_calculator_pyo3 import CalculatorComplex
from struqture_py import fermions

system = fermions.FermionHamiltonianSystem(3)

# This will work
hfp = fermions.HermitianFermionProduct([0, 1], [0, 2])
system.add_operator_product(hfp, CalculatorComplex.from_pair(1.0, 1.5))
print(system)

# This will not work, as the fermioncic index of the HermitianFermionProduct is larger
# than the number of the fermionic modes in the system (the fermionic mode with the
# smallest index is 0).
hfp_error = fermions.HermitianFermionProduct([3], [3])
value = CalculatorComplex.from_pair(1.0, 1.5)
# system.add_operator_product(hfp_error, value)  # Uncomment me!

# This will work because we leave the number of spins dynamic
system = fermions.FermionHamiltonianSystem()
hfp = fermions.HermitianFermionProduct([3], [3])
system.add_operator_product(hfp, CalculatorComplex.from_pair(1.0, 0.0))
```

## Noise operators and systems

We describe decoherence by representing it with the Lindblad equation.
The Lindblad equation is a master equation determining the time evolution of the density matrix.
It is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) = -i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]
with the rate matrix \\(\Gamma_{j,k}\\) and the Lindblad operator \\(L_{j}\\).
To describe the pure noise part of the Lindblad equation one needs the rate matrix in a well defined basis of Lindblad operators.
We use `FermionProducts` as the operator base. To describe fermionic noise we use the Lindblad equation with \\(\hat{H}=0\\).

The rate matrix and with it the Lindblad noise model is saved as a sum over pairs of `FermionProducts`, giving the operators acting from the left and right on the density matrix.
In programming terms the object `FermionLindbladNoiseOperator` is given by a HashMap or Dictionary with the tuple (`FermionProduct`, `FermionProduct`) as keys and the entries in the rate matrix as values.

Similarly to FermionOperators, FermionLindbladNoiseOperators have a system equivalent: `FermionLindbladNoiseSystem`, with a number of involved fermionic modes defined by the user. For more information on these, see the [noise container](../container_types/noise_operators_and_systems) chapter.

### Examples
Here, we add the terms \\(L_0 = c^{\dagger}_0 c_0\\) and \\(L_1 = c^{\dagger}_0 c_0\\) with coefficient 1.0:
\\( 1.0 \left( L_0 \rho L_1^{\dagger} - \frac{1}{2} \\{ L_1^{\dagger} L_0, \rho \\} \right) \\)

```rust
use qoqo_calculator::CalculatorComplex;
use struqture::prelude::*;
use struqture::fermions::{FermionProduct, FermionLindbladNoiseSystem};

let mut system = FermionLindbladNoiseSystem::new(Some(3));

let fp = FermionProduct::new([0], [0]).unwrap();

system
    .add_operator_product(
        (fp.clone(), fp.clone()),
        CalculatorComplex::new(1.0, 0.0)
    ).unwrap();
assert_eq!(system.get(&(fp.clone(), fp)), &CalculatorComplex::new(1.0, 0.0));
println!("{}", system);
```

The equivalent code in python:
```python
from qoqo_calculator_pyo3 import CalculatorComplex
from struqture_py import fermions

system = fermions.FermionLindbladNoiseSystem(3)

fp = fermions.FermionProduct([0], [0])

system.add_operator_product((fp, fp), CalculatorComplex.from_pair(1.0, 1.5))
print(system)

# In python we can also use the string representation
system = fermions.FermionLindbladNoiseSystem(3)
system.add_operator_product((str(fp), str(fp)), 1.0+1.5*1j)
print(system)
```

## Open systems

Physically open systems are quantum systems coupled to an environment that can often be described using Lindblad type of noise.
The Lindblad master equation is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) =-i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]
In struqture they are composed of a hamiltonian (FermionHamiltonianSystem) and noise (FermionLindbladNoiseSystem). They have different ways to set terms in Rust and Python:

### Examples

```rust
use qoqo_calculator::CalculatorComplex;
use struqture::prelude::*;
use struqture::fermions::{
    FermionProduct, HermitianFermionProduct, FermionLindbladOpenSystem
};

let mut open_system = FermionLindbladOpenSystem::new(Some(3));

let hfp = HermitianFermionProduct::new([0, 1], [0, 2]).unwrap();
let fp = FermionProduct::new([0], [0]).unwrap();

let system = open_system.system_mut();
system.add_operator_product(hfp, CalculatorComplex::new(2.0, 0.0)).unwrap();

let noise = open_system.noise_mut();
noise
    .add_operator_product(
        (fp.clone(), fp), CalculatorComplex::new(1.0, 0.0)
    ).unwrap();

println!("{}", open_system);
```

The equivalent code in python:
```python
from qoqo_calculator_pyo3 import CalculatorComplex
from struqture_py import fermions

open_system = fermions.FermionLindbladOpenSystem(3)

hfp = fermions.HermitianFermionProduct([0, 1], [0, 2])
fp = fermions.FermionProduct([0], [0])

open_system.system_add_operator_product(hfp, CalculatorComplex.from_pair(2.0, 0.0))
open_system.noise_add_operator_product(
    (fp, fp), CalculatorComplex.from_pair(0.0, 1.0))

print(open_system)
```
