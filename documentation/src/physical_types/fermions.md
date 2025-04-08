# Fermions

## Building blocks

All fermionic objects in `struqture` are expressed based on products of fermionic creation and annihilation operators, which respect fermionic anti-commutation relations
\\[ \lbrace c_k^{\dagger}, c_j^{\dagger} \rbrace = 0, \\\\
    \lbrace c_k, c_j \rbrace = 0, \\\\
    \lbrace c_k, c_j^{\dagger} \rbrace = \delta_{k, j}. \\]

### FermionProducts

FermionProducts are simple combinations of fermionic creation and annihilation operators.

### HermitianFermionProducts

HermitianFermionProducts are the hermitian equivalent of FermionProducts. This means that even though they are constructed the same (see the next section, `Examples`), they internally store both that term and its hermitian conjugate. For instance, given the term \\(c^{\dagger}_0 c_1 c_2\\), a FermionProduct would represent \\(c^{\dagger}_0 c_1 c_2\\) while a HermitianFermionProduct would represent \\(c^{\dagger}_0 c_1 c_2 + c^{\dagger}_2 c^{\dagger}_1 c_0\\).

### Example

The operator product is constructed by passing an array or a list of integers to represent the creation indices, and an array or a list of integers to represent the annihilation indices.

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

## Operators and Hamiltonians

Complex objects are constructed from operator products are `FermionOperators` and `FermionHamiltonians`
(for more information, [see also](../container_types/operators_hamiltonians_and_systems.md)).

These `FermionOperators` and `FermionHamiltonians` represent operators or Hamiltonians such as:
\\[ \hat{O} = \sum_{j=0}^N \alpha_j \left( \prod_{k=0}^N f(j, k) \right) \left( \prod_{l=0}^N g(j, l) \right) \\]
with
\\[ f(j, k) = \begin{cases} c_k^{\dagger} \\\\ \mathbb{1} \end{cases} , \\]
\\[ g(j, l) = \begin{cases} c_l \\\\ \mathbb{1} \end{cases} , \\]
and 
\\(c^{\dagger}\\) the fermionionic creation operator, \\(c\\) the fermionionic annihilation operator
\\[ \lbrace c_k^{\dagger}, c_j^{\dagger} \rbrace = 0, \\\\
    \lbrace c_k, c_j \rbrace = 0, \\\\
    \lbrace c_k^{\dagger}, c_j \rbrace = \delta_{k, j}. \\]

For instance, \\(c^{\dagger}_0 c^{\dagger}_1 c_1\\) is a term with a \\(c^{\dagger}\\) term acting on 0, and both a \\(c^{\dagger}\\) term and a \\(c\\) term acting on 1.

From a programming perspective the operators and Hamiltonians are HashMaps or Dictionaries with `FermionProducts` or `HermitianFermionProducts` (respectively) as keys and the coefficients \\(\alpha_j\\) as values. 

In `struqture` we distinguish between fermionic operators and Hamiltonians to avoid introducing unphysical behaviour by accident.
While both are sums over normal ordered fermionic products (stored as HashMaps of products with a complex prefactor), Hamiltonians are guaranteed to be hermitian. In a fermionic Hamiltonian , this means that the sums of products are sums of hermitian fermionic products (we have not only the \\(c^{\dagger}c\\) terms but also their hermitian conjugate) and the on-diagonal terms are required to have real prefactors. 
In the `HermitianFermionProducts`, we only explicitly store one part of the hermitian fermionic product, and we have chosen to store the one which has the smallest index of the creators that is smaller than the smallest index of the annihilators.

### Example

```python
from qoqo_calculator_pyo3 import CalculatorComplex
from struqture_py import fermions

operator = fermions.FermionHamiltonian()

# This will work
hfp = fermions.HermitianFermionProduct([0, 1], [0, 2])
operator.add_operator_product(hfp, CalculatorComplex.from_pair(1.0, 1.5))
hfp = fermions.HermitianFermionProduct([3], [3])
operator.add_operator_product(hfp, CalculatorComplex.from_pair(1.0, 0.0))
print(operator)
```

## Noise operators

We describe decoherence by representing it with the Lindblad equation.
The Lindblad equation is a master equation determining the time evolution of the density matrix.
It is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) = -i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]
with the rate matrix \\(\Gamma_{j,k}\\) and the Lindblad operator \\(L_{j}\\).

To describe fermionic noise we use the Lindblad equation with \\(\hat{H}=0\\).
Therefore, to describe the pure noise part of the Lindblad equation one needs the rate matrix in a well defined basis of Lindblad operators.
We use `FermionProducts` as the operator basis.

The rate matrix and with it the Lindblad noise model is saved as a sum over pairs of `FermionProducts`, giving the operators acting from the left and right on the density matrix.
In programming terms the object `FermionLindbladNoiseOperator` is given by a HashMap or Dictionary with the tuple (`FermionProduct`, `FermionProduct`) as keys and the entries in the rate matrix as values.

### Example
Here, we add the terms \\(L_0 = c^{\dagger}_0 c_0\\) and \\(L_1 = c^{\dagger}_0 c_0\\) with coefficient 1.0:
\\( 1.0 \left( L_0 \rho L_1^{\dagger} - \frac{1}{2} \\{ L_1^{\dagger} L_0, \rho \\} \right) \\)

```python
from qoqo_calculator_pyo3 import CalculatorComplex
from struqture_py import fermions

# Setting up the operator and the product we want to add to it
operator = fermions.FermionLindbladNoiseOperator()
fp = fermions.FermionProduct([0], [0])

# Adding the product to the operator
operator.add_operator_product((fp, fp), CalculatorComplex.from_pair(1.0, 1.5))
print(operator)

# In python we can also use the string representation
operator = fermions.FermionLindbladOpenSystem()
operator.noise_add_operator_product((str(fp), str(fp)), 1.0 + 1.5 * 1j)
print(operator)
```

## Open systems

Physically open systems are quantum systems coupled to an environment that can often be described using Lindblad type of noise.
The Lindblad master equation is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) =-i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]
In `struqture` they are composed of a Hamiltonian (`FermionHamiltonian`) and noise (`FermionLindbladNoiseOperator`).

### Example

```python
from qoqo_calculator_pyo3 import CalculatorComplex
from struqture_py import fermions

open_system = fermions.FermionLindbladOpenSystem()

hfp = fermions.HermitianFermionProduct([0, 1], [0, 2])
fp = fermions.FermionProduct([0], [0])

# Adding the c_0^dag c_1^dag c_0 c_2 term to the system part of the open system
open_system.system_add_operator_product(hfp, CalculatorComplex.from_pair(2.0, 0.0))
# Adding the c_0^dag c_0 part to the noise part of the open system
open_system.noise_add_operator_product(
    (fp, fp), CalculatorComplex.from_pair(0.0, 1.0))

print(open_system)
```
