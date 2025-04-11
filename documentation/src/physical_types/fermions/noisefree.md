# Operators and Hamiltonians

`FermionOperators` and `FermionHamiltonians` represent operators or Hamiltonians such as:
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
While both are sums over normal ordered fermionic products (stored as dictionaries of products with a complex prefactor), Hamiltonians are guaranteed to be hermitian. In a fermionic Hamiltonian, this means that the sums of products are sums of hermitian fermionic products (we have not only the \\(c^{\dagger}c\\) terms but also their hermitian conjugate) and the on-diagonal terms are required to have real prefactors. 
In the `HermitianFermionProducts`, we only explicitly store one part of the hermitian fermionic product, and we have chosen to store the one which has the smallest index of the creators that is smaller than the smallest index of the annihilators. If the user choses \\(c_0^{\dagger}c_0\\) `HermitianFermionProduct([], [0])` will be created while the second part will be stored explicitly.

## Example

Here is an example of how to build a `FermionOperator`:

```python
from struqture_py import fermions

# We start by initializing our FermionOperator
operator = fermions.FermionOperator()

# We set the term and some value of our choosing
operator.set("c0c1a0a2", 1.0 + 1.5j)
# We can use the `get` function to check what value/prefactor is stored for the FermionProduct
assert operator.get("c0c1a0a2") == complex(1.0, 1.5)
print(operator)

# Please note that the `set` function will set the value given, overwriting any previous value.
# Should you prefer to use and additive method, please use `add_operator_product`:
operator.add_operator_product("c0c1a0a2", 1.0)
print(operator)
# NOTE: this is equivalent to: operator.add_operator_product(FermionProduct([0, 1], [0, 2]))

# NOTE: the above values used can also be symbolic.
# Symbolic parameters can be very useful for a variety of reasons, as detailed in the introduction.
# In order to set a symbolic parameter, we can pass either a string or use the `qoqo_calculator_pyo3` package:
from qoqo_calculator_pyo3 import CalculatorComplex

operator.add_operator_product("c0c1a0a2", "parameter")
operator.add_operator_product("c0c1a0a2", CalculatorComplex.from_pair("parameter", 0.0))
```

Here is an example of how to build a `FermionHamiltonian`:

```python
from struqture_py import fermions

# We start by initializing our FermionHamiltonian
hamiltonian = fermions.FermionHamiltonian()
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
