# Operators and Hamiltonians

Complex objects are constructed from operator products are `BosonOperators` and `BosonHamiltonians`
(for more information, [see also](../../container_types/operators_hamiltonians_and_systems.md)).

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

In `struqture` we distinguish between bosonic operators and Hamiltonians to avoid introducing unphysical behaviour by accident.
While both are sums over normal ordered bosonic products (stored as HashMaps of products with a complex prefactor), Hamiltonians are guaranteed to be hermitian. In a bosonic Hamiltonian , this means that the sums of products are sums of hermitian bosonic products (we have not only the \\(c^{\dagger}c\\) terms but also their hermitian conjugate) and the on-diagonal terms are required to have real prefactors. 
In the `HermitianBosonProducts`, we only explicitly store one part of the hermitian bosonic product, and we have chosen to store the one which has the smallest index of the creators that is smaller than the smallest index of the annihilators.

## Example

Here is an example of how to build a `BosonOperator`:

```python
from struqture_py import bosons

# We start by initializing our BosonOperator
operator = bosons.BosonOperator()

# We create a BosonProduct or HermitianBosonProduct
bp = bosons.BosonProduct.from_string("c0c1a0a2")
hbp = bosons.HermitianBosonProduct.from_string("c0c1a0a2")

# We set the term and some value of our choosing
operator.set(bp, 1.0 + 1.5j)
# We can use the `get` function to check what value/prefactor is stored for the BosonProduct
assert operator.get(bp) == complex(1.0, 1.5)
print(operator)

# Please note that the `set` function will set the value given, overwriting any previous value.
# Should you prefer to use and additive method, please use `add_operator_product`:
operator.add_operator_product(bp, 1.0)
print(operator)

# NOTE: the above values used can also be symbolic.
# Symbolic parameters can be very useful for a variety of reasons, as detailed in the introduction.
# In order to set a symbolic parameter, we can pass either a string or use the `qoqo_calculator_pyo3` package:
from qoqo_calculator_pyo3 import CalculatorComplex

operator.add_operator_product(hbp, "parameter")
operator.add_operator_product(hbp, CalculatorComplex.from_pair("parameter", 0.0))
```


Here is an example of how to build a `BosonHamiltonian`:

```python
from struqture_py import bosons

# We start by initializing our BosonHamiltonian
hamiltonian = bosons.BosonHamiltonian()
# We set both of the terms and values specified above
hamiltonian.set("c0a0", 0.5)
hamiltonian.set("c1a1", 0.5)

# Please note that the `set` function will set the value given, overwriting any previous value.
# Should you prefer to use and additive method, please use `add_operator_product`:
hamiltonian.add_operator_product("c0a0", 1.0)

print(hamiltonian)

# NOTE: the above values used can also be symbolic.
# Symbolic parameters can be very useful for a variety of reasons, as detailed in the introduction.
# In order to set a symbolic parameter, we can pass either a string or use the `qoqo_calculator_pyo3` package:
from qoqo_calculator_pyo3 import CalculatorFloat

hamiltonian.add_operator_product("c0a0", "parameter")
hamiltonian.add_operator_product("c1a1", CalculatorFloat("parameter"))
```
