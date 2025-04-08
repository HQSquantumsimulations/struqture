# Mixed Systems

## Building blocks

All the mixed operators are expressed based on products of mixed indices which contain spin terms, bosonic terms and fermionic terms. The spin terms respect Pauli operator cyclicity, the bosonic terms respect bosonic commutation relations, and the fermionic terms respect fermionic anti-commutation relations.

These products respect the following relations:
\\[
    -i \sigma^x \sigma^y \sigma^z = I
\\]
\\[ \lbrack c_{b, k}^{\dagger}, c_{b, j}^{\dagger} \rbrack = 0, \\\\
    \lbrack c_{b, k}, c_{b, j} \rbrack = 0, \\\\
    \lbrack c_{b, k}, c_{b, j}^{\dagger} \rbrack = \delta_{k, j}. \\]
\\[ \lbrace c_{f, k}^{\dagger}, c_{f, j}^{\dagger} \rbrace = 0, \\\\
    \lbrace c_{f, k}, c_{f, j} \rbrace = 0, \\\\
    \lbrace c_{f, k}, c_{f, j}^{\dagger} \rbrace = \delta_{k, j}. \\]

with 
\\(c_b^{\dagger}\\) the bosonic creation operator, \\(c_b\\) the bosonic annihilation operator, \\(\lbrack ., . \rbrack\\) the bosonic commutation relations, \\(c_f^{\dagger}\\) the fermionic creation operator, \\(c_f\\) the fermionic annihilation operator, and \\(\lbrace ., . \rbrace\\) the fermionic anti-commutation relations.

### MixedProducts

MixedProducts are combinations of `PauliProducts`, `BosonProducts` and `FermionProducts`.

### HermitianMixedProducts

HermitianMixedProducts are the hermitian equivalent of MixedProducts. This means that even though they are constructed the same (see the `Examples` section), they internally store both that term and its hermitian conjugate. 

### MixedDecoherenceProducts

MixedDecoherenceProducts are combinations of `DecoherenceProducts`, `BosonProducts` and `FermionProducts`.

### Example

The operator product is constructed by passing an array/a list of spin terms, an array/a list of bosonic terms and an array/a list of fermionic terms.

```python
from struqture_py import mixed_systems, bosons, spins, fermions

# Building the spin term sigma^x_0 sigma^z_1
pp = spins.PauliProduct().x(0).z(1)
# Building the bosonic term c_b^{\dagger}_1 * c_b^{\dagger}_2 * c_b_2
bp = bosons.BosonProduct([1, 2], [2])
# Building the fermionic term c_f^{\dagger}_0 * c_f^{\dagger}_1 * c_f_0 * c_f_1
fp = fermions.FermionProduct([0, 1], [0, 1])

# Building the term sigma^x_0 sigma^z_1 c_b^{\dagger}_1 * c_b^{\dagger}_2
# * c_b_2 * c_f^{\dagger}_0 * c_f^{\dagger}_1 * c_f_0 * c_f_1
hmp = mixed_systems.MixedProduct([pp], [bp], [fp])

# Building the term sigma^x_0 sigma^z_1 c_b^{\dagger}_1 * c_b^{\dagger}_2 *
# c_b_2 * c_f^{\dagger}_0 * c_f^{\dagger}_1 * c_f_0 * c_f_1  +  h.c.
hmp = mixed_systems.HermitianMixedProduct([pp], [bp], [fp])


# Building the spin term sigma^x_0 sigma^z_1
dp = spins.DecoherenceProduct().x(0).z(1)
# Building the bosonic term c_b^{\dagger}_1 * c_b^{\dagger}_2 * c_b_2
bp = bosons.BosonProduct([1, 2], [0, 1])
# Building the fermionic term c_f^{\dagger}_0 * c_f^{\dagger}_1 * c_f_0 * c_f_1
fp = fermions.FermionProduct([0, 1], [0, 1])

# This will work
mdp = mixed_systems.MixedDecoherenceProduct([dp], [bp], [fp])
```

## Operators and Hamiltonians

Complex objects are constructed from operator products are `MixedOperators` and `MixedHamiltonians`
(for more information, [see also](../container_types/operators_hamiltonians_and_systems.md)).

These `MixedOperators` and `MixedHamiltonians` represent operators or Hamiltonians such as:
\\[ \hat{H} = \sum_j \alpha_j \prod_k \sigma_{j, k} \prod_{l, m} c_{b, l, j}^{\dagger} c_{b, m, j} \prod_{r, s} c_{f, r, j}^{\dagger} c_{f, s, j} \\]
with commutation relations and cyclicity respected.

From a programming perspective the operators and Hamiltonians are HashMaps or Dictionaries with `MixedProducts` or `HermitianMixedProducts` (respectively) as keys and the coefficients \\(\alpha_j\\) as values. 

In `struqture` we distinguish between mixed operators and Hamiltonians to avoid introducing unphysical behaviour by accident.
While both are sums over normal ordered mixed products (stored as HashMaps of products with a complex prefactor), Hamiltonians are guaranteed to be hermitian to avoid introducing unphysical behaviour by accident. In a mixed Hamiltonian , this means that the sums of products are sums of hermitian mixed products (we have not only the \\(c^{\dagger}c\\) terms but also their hermitian conjugate) and the on-diagonal terms are required to have real prefactors. We also require the smallest index of the creators to be smaller than the smallest index of the annihilators.

For `MixedOperators` and `MixedHamiltonians`, we need to specify the number of spin subsystems, bosonic subsystems and fermionic subsystems exist in the operator/Hamiltonian . See the example for more information.

### Example

Here is an example of how to build a `MixedOperator` and a `MixedHamiltonian`:

```python
from qoqo_calculator_pyo3 import CalculatorComplex
from struqture_py import bosons, fermions, spins, mixed_systems

operator = mixed_systems.MixedHamiltonian(2, 1, 1)

# Building the spin term sigma^x_0 sigma^z_1
pp_0 = spins.PauliProduct().x(0).z(1)
# Building the spin term sigma^y_0
pp_1 = spins.PauliProduct().y(0)
# Building the bosonic term c_b^{\dagger}_1 * c_b^{\dagger}_2 * c_b_2
bp = bosons.BosonProduct([1, 2], [2])
# Building the fermionic term c_f^{\dagger}_0 * c_f^{\dagger}_1 * c_f_0 * c_f_1
fp = fermions.FermionProduct([0, 1], [0, 1])

# This will work
hmp = mixed_systems.HermitianMixedProduct([pp_0, pp_1], [bp], [fp])
operator.add_operator_product(hmp, CalculatorComplex.from_pair(1.0, 1.5))
print(operator)

# This will not work, as the number of subsystems of the
# operator and product do not match.
hmp_error = mixed_systems.HermitianMixedProduct([pp_0, pp_1], [], [fp])
value = CalculatorComplex.from_pair(1.0, 1.5)
# operator.add_operator_product(hmp_error, value)  # Uncomment me!
```

## Noise operators

We describe decoherence by representing it with the Lindblad equation.
The Lindblad equation is a master equation determining the time evolution of the density matrix.
It is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) =-i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]
with the rate matrix \\(\Gamma_{j,k}\\) and the Lindblad operator \\(L_{j}\\).
To describe the pure noise part of the Lindblad equation one needs the rate matrix in a well defined basis of Lindblad operators.
We use `MixedDecoherenceProducts` as the operator basis. To describe mixed noise we use the Lindblad equation with \\(\hat{H}=0\\).

The rate matrix and with it the Lindblad noise model is saved as a sum over pairs of `MixedDecoherenceProducts`, giving the operators acting from the left and right on the density matrix.
In programming terms the object `MixedLindbladNoiseOperators` is given by a HashMap or Dictionary with the tuple (`MixedDecoherenceProduct`, `MixedDecoherenceProduct`) as keys and the entries in the rate matrix as values.

### Example
Here, we add the terms \\(L_0 = \left( \sigma_0^x \sigma_1^z \right) \left( c_{b, 1}^{\dagger} c_{b, 1} \right) \left( c_{f, 0}^{\dagger} c_{f, 1}^{\dagger} c_{f, 0} c_{f, 1} \right)\\) and \\(L_1 = \left( \sigma_0^x \sigma_1^z \right) \left( c_{b, 1}^{\dagger} c_{b, 1} \right) \left( c_{f, 0}^{\dagger} c_{f, 1}^{\dagger} c_{f, 0} c_{f, 1} \right)\\) with coefficient 1.0:
\\( 1.0 \left( L_0 \rho L_1^{\dagger} - \frac{1}{2} \\{ L_1^{\dagger} L_0, \rho \\} \right) \\)

```python
from qoqo_calculator_pyo3 import CalculatorComplex
from struqture_py import bosons, fermions, spins, mixed_systems

operator = mixed_systems.MixedLindbladNoiseOperator(1, 1, 1)

# Building the spin term sigma^x_0 sigma^z_1
pp_0 = spins.DecoherenceProduct().x(0).z(1)
# Building the bosonic term c_b^{\dagger}_0 * c_b_0
bp = bosons.BosonProduct([0], [0])
# Building the fermionic term c_f^{\dagger}_0 * c_f^{\dagger}_1 * c_f_0 * c_f_1
fp = fermions.FermionProduct([0, 1], [0, 1])

# Building the term sigma^x_0 sigma^z_1
# * c_b^{\dagger}_0 * c_b_0 * c_f^{\dagger}_0 * c_f^{\dagger}_1 * c_f_0 * c_f_1
mdp = mixed_systems.MixedDecoherenceProduct([pp], [bp], [fp])

# Adding in the mixed decoherence product
operator.add_operator_product(
    (mdp, mdp), CalculatorComplex.from_pair(1.0, 1.5))
print(operator)

# In python we can also use the string representation
operator = mixed_systems.MixedLindbladNoiseOperator(1, 1, 1)
operator.add_operator_product((str(mdp), str(mdp)), 1.0+1.5*1j)
print(operator)
```

## Open systems

Physically open systems are quantum systems coupled to an environment that can often be described using Lindblad type of noise.
The Lindblad master equation is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) =-i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]

In `struqture` they are composed of a Hamiltonian (MixedHamiltonian) and noise (MixedLindbladNoiseOperator).

### Example

```python
from qoqo_calculator_pyo3 import CalculatorComplex
from struqture_py import bosons, fermions, spins, mixed_systems

open_system = mixed_systems.MixedLindbladOpenSystem(1, 1, 1)

# Building the spin term sigma^x_0 sigma^z_1
pp = spins.PauliProduct().x(0).z(1)
# Building the bosonic term c_b^{\dagger}_1 * c_b^{\dagger}_2 * c_b_2
bp = bosons.BosonProduct([1, 2], [2])
# Building the fermionic term c_f^{\dagger}_0 * c_f^{\dagger}_1 * c_f_0 * c_f_1
fp = fermions.FermionProduct([0, 1], [0, 1])

# Building the term sigma^x_0 sigma^z_1 * c_b^{\dagger}_1
# * c_b^{\dagger}_2 * c_b_2 * c_f^{\dagger}_0 * c_f^{\dagger}_1 * c_f_0 * c_f_1
# + h.c.
hmp = mixed_systems.HermitianMixedProduct([pp], [bp], [fp])

# Building the spin term sigma^x_0 sigma^z_1
dp = spins.DecoherenceProduct().x(0).z(1)
# Building the bosonic term c_b^{\dagger}_1 * c_b_1
bp = bosons.BosonProduct([1], [1])
# Building the fermionic term c_f^{\dagger}_0 * c_f^{\dagger}_1 * c_f_0 * c_f_1
fp = fermions.FermionProduct([0, 1], [0, 1])

# Building the term sigma^x_0 sigma^z_1 * c_b^{\dagger}_1
# * c_b_1 * c_f^{\dagger}_0 * c_f^{\dagger}_1 * c_f_0 * c_f_1
mdp = mixed_systems.MixedDecoherenceProduct([dp], [bp], [fp])

# Adding in the system term
open_system.system_add_operator_product(hmp, CalculatorComplex.from_pair(2.0, 0.0))
# Adding in the noise term
open_system.noise_add_operator_product((mdp, mdp), CalculatorComplex.from_pair(0.0, 1.0))

print(open_system)
```
