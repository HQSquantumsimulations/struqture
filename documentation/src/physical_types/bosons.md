# Bosons

## Building blocks

All bosonic objects in struqture are expressed based on products of bosonic creation and annihilation operators, which respect bosonic commutation relations
\\[ \lbrack c_k^{\dagger}, c_j^{\dagger} \rbrack = 0, \\\\
    \lbrack c_k, c_j \rbrack = 0, \\\\
    \lbrack c_k, c_j^{\dagger} \rbrack = \delta_{k, j}. \\]

### BosonProducts

BosonProducts are simple combinations of bosonic creation and annihilation operators.

### HermitianBosonProducts

HermitianBosonProducts are the hermitian equivalent of BosonProducts. This means that even though they are constructed the same (see the next section, `Examples`), they internally store both that term and its hermitian conjugate. For instance, given the term \\(c^{\dagger}_0 c_1 c_2\\), a BosonProduct would represent \\(c^{\dagger}_0 c_1 c_2\\) while a HermitianBosonProduct would represent \\(c^{\dagger}_0 c_1 c_2 + c^{\dagger}_2 c^{\dagger}_1 c_0\\).

### Example

The operator product is constructed by passing an array or a list of integers to represent the creation indices, and an array or a list of integers to represent the annihilation indices.

Note: (Hermitian)BosonProducts can only been created from the correct ordering of indices (the wrong sequence will return an error) but we have the `create_valid_pair` function to create a valid Product from arbitrary sequences of operators which also transforms an index value according to the commutation and hermitian conjugation rules.

```python
from struqture_py.bosons import BosonProduct, HermitianBosonProduct
from qoqo_calculator_pyo3 import CalculatorComplex

# A product of a creation operator acting on bosonic mode 0 and an annihilation operator
# acting on bosonic mode 20
bp = BosonProduct([0], [20])
# Building the term c^{\dagger}_1 * c^{\dagger}_3 * c_0
bp = BosonProduct.create_valid_pair([3, 1], [0], CalculatorComplex.from_pair(1.0, 0.0))


# A product of a creation operator acting on bosonic mode 0 and an annihilation
# operator acting on bosonic mode 20, as well as a creation operator acting on
# bosonic mode 20 and an annihilation operator acting on bosonic mode 0
hbp = HermitianBosonProduct([0], [20])
# Building the term c^{\dagger}_0 * c^{\dagger}_3 * c_0 + c^{\dagger}_0 * c_3 * c_0
hbp = HermitianBosonProduct.create_valid_pair(
    [3, 0], [0], CalculatorComplex.from_pair(1.0, 0.0))
```

## Operators and Hamiltonians

Complex objects are constructed from operator products are `BosonOperators` and `BosonHamiltonians`
(for more information, [see also](../container_types/operators_hamiltonians_and_systems.md)).

These `BosonOperators` and `BosonHamiltonians` represent operators or Hamiltonians such as:
\\[ \hat{O} = \sum_{j=0}^N \alpha_j \left( \prod_{k=0}^N f(j, k) \right) \left( \prod_{l=0}^N g(j, l) \right) \\]
with
\\[ f(j, k) = \begin{cases} c_k^{\dagger} \\\\ \mathbb{1} \end{cases} , \\]
\\[ g(j, l) = \begin{cases} c_l \\\\ \mathbb{1} \end{cases} , \\]
and 
\\(c^{\dagger}\\) the bosonic creation operator, \\(c\\) the bosonic annihilation operator 
\\[ \lbrack c_k^{\dagger}, c_j^{\dagger} \rbrack = 0, \\\\
    \lbrack c_k, c_j \rbrack = 0, \\\\
    \lbrack c_k^{\dagger}, c_j \rbrack = \delta_{k, j}. \\]


From a programming perspective the operators and Hamiltonians are HashMaps or Dictionaries with `BosonProducts` or `HermitianBosonProducts` (respectively) as keys and the coefficients \\(\alpha_j\\) as values. 

In struqture we distinguish between bosonic operators and Hamiltonians to avoid introducing unphysical behaviour by accident.
While both are sums over normal ordered bosonic products (stored as HashMaps of products with a complex prefactor), Hamiltonians are guaranteed to be hermitian. In a bosonic Hamiltonian , this means that the sums of products are sums of hermitian bosonic products (we have not only the \\(c^{\dagger}c\\) terms but also their hermitian conjugate) and the on-diagonal terms are required to have real prefactors. 
In the `HermitianBosonProducts`, we only explicitly store one part of the hermitian bosonic product, and we have chosen to store the one which has the smallest index of the creators that is smaller than the smallest index of the annihilators.

### Example

Here is an example of how to build a `BosonOperator` and a `BosonHamiltonian`:

```python
from qoqo_calculator_pyo3 import CalculatorComplex
from struqture_py import bosons

system = bosons.BosonHamiltonian()

# This will work
hbp = bosons.HermitianBosonProduct([0, 1], [0, 2])
system.add_operator_product(hbp, CalculatorComplex.from_pair(1.0, 1.5))
hbp = bosons.HermitianBosonProduct([3], [3])
system.add_operator_product(hbp, CalculatorComplex.from_pair(1.0, 0.0))
print(system)
```

## Noise operators

We describe decoherence by representing it with the Lindblad equation.
The Lindblad equation is a master equation determining the time evolution of the density matrix.
It is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) = -i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]
with the rate matrix \\(\Gamma_{j,k}\\) and the Lindblad operator \\(L_{j}\\).

To describe bosonic noise we use the Lindblad equation with \\(\hat{H}=0\\).
Therefore, to describe the pure noise part of the Lindblad equation one needs the rate matrix in a well defined basis of Lindblad operators.
We use `BosonProducts` as the operator basis.

The rate matrix and with it the Lindblad noise model is saved as a sum over pairs of `BosonProducts`, giving the operators acting from the left and right on the density matrix.
In programming terms the object `BosonLindbladNoiseOperator` is given by a HashMap or Dictionary with the tuple (`BosonProduct`, `BosonProduct`) as keys and the entries in the rate matrix as values.

### Example

Here, we add the terms \\(L_0 = c^{\dagger}_0 c_0\\) and \\(L_1 = c^{\dagger}_0 c_0\\) with coefficient 1.0:
\\( 1.0 \left( L_0 \rho L_1^{\dagger} - \frac{1}{2} \\{ L_1^{\dagger} L_0, \rho \\} \right) \\)

```python
from qoqo_calculator_pyo3 import CalculatorComplex
from struqture_py import bosons

# Setting up the operator and the product we want to add to it
operator = bosons.BosonLindbladNoiseOperator()
bp = bosons.BosonProduct([0], [0])

# Adding the product to the operator
operator.add_operator_product((bp, bp), CalculatorComplex.from_pair(1.0, 1.5))
print(operator)

# In python we can also use the string representation
operator = bosons.BosonLindbladNoiseOperator()
operator.add_operator_product((str(bp), str(bp)), 1.0+1.5*1j)
print(operator)
```

## Open systems

Physically open systems are quantum systems coupled to an environment that can often be described using Lindblad type of noise.
The Lindblad master equation is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) =-i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]
In struqture they are composed of a Hamiltonian (`BosonHamiltonian`) and noise (`BosonLindbladNoiseOperator`). They have different ways to set terms in Rust and Python:

### Example

```python
from qoqo_calculator_pyo3 import CalculatorComplex
from struqture_py import bosons

open_system = bosons.BosonLindbladOpenSystem()

hbp = bosons.HermitianBosonProduct([0, 1], [0, 2])
bp = bosons.BosonProduct([0], [0])

# Adding the c_0^dag c_1^dag c_0 c_2 term to the system part of the open system
open_system.system_add_operator_product(hbp, CalculatorComplex.from_pair(2.0, 0.0))
# Adding the c_0^dag c_0 part to the noise part of the open system
open_system.noise_add_operator_product(
    (bp, bp), CalculatorComplex.from_pair(0.0, 1.0))

print(open_system)
```
