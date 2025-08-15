# Operators and Hamiltonians

`BosonOperators` and `BosonHamiltonians` represent operators or Hamiltonians such as:
\\[ \hat{O} = \sum_{j=0}^N \alpha_j \left( \prod_{k=0}^N f(j, k) \right) \left( \prod_{l=0}^N g(j, l) \right) \\]
with
\\[ f(j, k) = \begin{cases} b_k^{\dagger} \\\\ \mathbb{1} \end{cases} , \\]
\\[ g(j, l) = \begin{cases} b_l \\\\ \mathbb{1} \end{cases} , \\]
and 
\\(b^{\dagger}\\) the bosonic creation operator, \\(c\\) the bosonic annihilation operator 
\\[ \lbrack b_k^{\dagger}, b_j^{\dagger} \rbrack = 0, \\\\
    \lbrack b_k, b_j \rbrack = 0, \\\\
    \lbrack b_k^{\dagger}, b_j \rbrack = \delta_{k, j}. \\]


From a programming perspective the operators and Hamiltonians are HashMaps or Dictionaries with `BosonProducts` or `HermitianBosonProducts` (respectively) as keys and the coefficients \\(\alpha_j\\) as values. 

In `struqture` we distinguish between bosonic operators and Hamiltonians to avoid introducing unphysical behaviour by accident.
While both are sums over normal ordered bosonic products (stored as dictionaries of products with a complex prefactor), Hamiltonians are guaranteed to be hermitian. In a bosonic Hamiltonian, this means that the sums of products are sums of hermitian bosonic products (we have not only the \\(b^{\dagger}b\\) terms but also their hermitian conjugate) and the on-diagonal terms are required to have real prefactors.
In the `HermitianBosonProducts`, we only explicitly store one part of the hermitian bosonic product, and we have chosen to store the one which has the smallest index of the creators that is smaller than the smallest index of the annihilators. For instance, if the user would like to define a  \\(b_0^{\dagger} + b_0\\) term, they would create this object: `HermitianBosonProduct([], [0])`. The second part of the term is stored implicitly by the code.

## Example

Here is an example of how to build a `BosonOperator`:

```python
from struqture_py import bosons

# We start by initializing our BosonOperator
operator = bosons.BosonOperator()

# We set the term and some value of our choosing
operator.set("c0c1a0a2", 1.0 + 1.5j)
# We can use the `get` function to check what value/prefactor is stored for the BosonProduct
assert operator.get("c0c1a0a2") == complex(1.0, 1.5)
print(operator)

# Please note that the `set` function will set the value given, overwriting any previous value.
# Should you prefer to use and additive method, please use `add_operator_product`:
operator.add_operator_product("c0c1a0a2", 1.0)
print(operator)
# NOTE: this is equivalent to: operator.add_operator_product(BosonProduct([0, 1], [0, 2]))

# NOTE: the above values used can also be symbolic.
# Symbolic parameters can be very useful for a variety of reasons, as detailed in the introduction.
operator.add_operator_product(hbp, "parameter")
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
hamiltonian.add_operator_product("c0a0", "parameter")
```
