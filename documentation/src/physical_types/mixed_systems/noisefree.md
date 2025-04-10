# Operators and Hamiltonians

Complex objects are constructed from operator products are `MixedOperators` and `MixedHamiltonians`
(for more information, [see also](../container_types/operators_hamiltonians_and_systems.md)).

These `MixedOperators` and `MixedHamiltonians` represent operators or Hamiltonians such as:
\\[ \hat{H} = \sum_j \alpha_j \prod_k \sigma_{j, k} \prod_{l, m} b_{l, j}^{\dagger} b_{m, j} \prod_{r, s} c_{r, j}^{\dagger} c_{s, j} \\]
with commutation relations and cyclicity respected.

From a programming perspective the operators and Hamiltonians are HashMaps or Dictionaries with `MixedProducts` or `HermitianMixedProducts` (respectively) as keys and the coefficients \\(\alpha_j\\) as values. 

In `struqture` we distinguish between mixed operators and Hamiltonians to avoid introducing unphysical behaviour by accident.
While both are sums over normal ordered mixed products (stored as HashMaps of products with a complex prefactor), Hamiltonians are guaranteed to be hermitian to avoid introducing unphysical behaviour by accident. In a mixed Hamiltonian , this means that the sums of products are sums of hermitian mixed products (we have not only the \\(c^{\dagger}c\\) terms but also their hermitian conjugate) and the on-diagonal terms are required to have real prefactors. We also require the smallest index of the creators to be smaller than the smallest index of the annihilators.

For `MixedOperators` and `MixedHamiltonians`, we need to specify the number of spin subsystems, bosonic subsystems and fermionic subsystems exist in the operator/Hamiltonian . See the example for more information.

## Example

Here is an example of how to build a `MixedOperator`:

```python
from struqture_py import bosons, fermions, spins, mixed_systems

# We start by initializing our MixedOperator
operator = mixed_systems.MixedOperator(2, 1, 1)

# We use the different products to create a MixedProduct or HermitianMixedProduct
mp = mixed_systems.MixedProduct.from_string("S0X1Z:S0Y:Bc1c2a2:Fc0c1a0a1")
hmp = mixed_systems.HermitianMixedProduct.from_string("S0X1Z:S0Y:Bc1c2a2:Fc0c1a0a1")

# We set the term and some value of our choosing
operator.set(mp, 1.0 + 1.5j)
# We can use the `get` function to check what value/prefactor is stored for the FermionProduct
assert operator.get(mp) == complex(1.0, 1.5)
print(operator)

# Please note that the `set` function will set the value given, overwriting any previous value.
# Should you prefer to use and additive method, please use `add_operator_product`:
operator.add_operator_product(mp, 1.0)
print(operator)

# NOTE: the above values used can also be symbolic.
# Symbolic parameters can be very useful for a variety of reasons, as detailed in the introduction.
# In order to set a symbolic parameter, we can pass either a string or use the `qoqo_calculator_pyo3` package:
from qoqo_calculator_pyo3 import CalculatorComplex

operator.add_operator_product(hmp, "parameter")
operator.add_operator_product(hmp, CalculatorComplex.from_pair("parameter", 0.0))

# This will not work, as the number of subsystems of the
# hamiltonian and product do not match.
hmp_error = mixed_systems.HermitianMixedProduct.from_string("S0X1Z:S0Y:Fc0c1a0a1")
value = CalculatorComplex.from_pair(1.0, 1.5)
# hamiltonian.add_operator_product(hmp_error, value)  # Uncomment me!
```

Here is an example of how to build a `MixedHamiltonian`:

```python
from struqture_py import mixed_systems

# We start by initializing our MixedHamiltonian
hamiltonian = mixed_systems.MixedHamiltonian(1, 1, 1)
# We set both of the terms and values specified above
hamiltonian.set("S0X:Bc0a0:Fc0a0", 0.5)
hamiltonian.set("S0Y:Bc0a0:Fc1a1", 0.5)

# Please note that the `set` function will set the value given, overwriting any previous value.
# Should you prefer to use and additive method, please use `add_operator_product`:
hamiltonian.add_operator_product("S0X:Bc0a0:Fc0a0", 1.0)

print(hamiltonian)

# NOTE: the above values used can also be symbolic.
# Symbolic parameters can be very useful for a variety of reasons, as detailed in the introduction.
# In order to set a symbolic parameter, we can pass either a string or use the `qoqo_calculator_pyo3` package:
from qoqo_calculator_pyo3 import CalculatorFloat

hamiltonian.add_operator_product("S0X:Bc0a0:Fc0a0", "parameter")
hamiltonian.add_operator_product("S0X:Bc0a0:Fc0a0", CalculatorFloat("parameter"))
```
