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

# We start by initializing our MixedHamiltonian
hamiltonian = mixed_systems.MixedHamiltonian(2, 1, 1)

# Building the spin term sigma^x_0 sigma^z_1
pp_0 = spins.PauliProduct().x(0).z(1)
# Building the spin term sigma^y_0
pp_1 = spins.PauliProduct().y(0)
# Building the bosonic term c_b^{\dagger}_1 * c_b^{\dagger}_2 * c_b_2
bp = bosons.BosonProduct([1, 2], [2])
# Building the fermionic term c_f^{\dagger}_0 * c_f^{\dagger}_1 * c_f_0 * c_f_1
fp = fermions.FermionProduct([0, 1], [0, 1])

# We use the different products to create a MixedProduct or HermitianMixedProduct
mp = mixed_systems.MixedProduct([pp_0, pp_1], [bp], [fp])
hmp = mixed_systems.HermitianMixedProduct([pp_0, pp_1], [bp], [fp])

# We set the term and some value of our choosing
hamiltonian.set(mp, 1.0 + 1.5j)
# We can use the `get` function to check what value/prefactor is stored for the FermionProduct
assert hamiltonian.get(mp) == complex(1.0, 1.5)
print(hamiltonian)

# Please note that the `set` function will set the value given, overwriting any previous value.
# Should you prefer to use and additive method, please use `add_operator_product`:
hamiltonian.add_operator_product(mp, 1.0)
print(hamiltonian)

# NOTE: the above values used can also be symbolic.
# Symbolic parameters can be very useful for a variety of reasons, as detailed in the introduction.
# In order to set a symbolic parameter, we can pass either a string or use the `qoqo_calculator_pyo3` package:
from qoqo_calculator_pyo3 import CalculatorComplex

hamiltonian.add_operator_product(hmp, "parameter")
hamiltonian.add_operator_product(hmp, CalculatorComplex.from_pair("parameter", 0.0))

# This will not work, as the number of subsystems of the
# hamiltonian and product do not match.
hmp_error = mixed_systems.HermitianMixedProduct([pp_0, pp_1], [], [fp])
value = CalculatorComplex.from_pair(1.0, 1.5)
# hamiltonian.add_operator_product(hmp_error, value)  # Uncomment me!
```
