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

Here is an example of how to build a `FermionOperator` and a `FermionHamiltonian`:

```python
from struqture_py import fermions

# We start by initializing our FermionHamiltonian
hamiltonian = fermions.FermionHamiltonian()

# We create a FermionProduct or HermitianFermionProduct
bp = fermions.FermionProduct([0, 1], [0, 2])
hbp = fermions.HermitianFermionProduct([0, 1], [0, 2])

# We set the term and some value of our choosing
hamiltonian.set(bp, 1.0 + 1.5j)
# We can use the `get` function to check what value/prefactor is stored for the FermionProduct
assert hamiltonian.get(bp) == complex(1.0, 1.5)
print(hamiltonian)

# Please note that the `set` function will set the value given, overwriting any previous value.
# Should you prefer to use and additive method, please use `add_operator_product`:
hamiltonian.add_operator_product(bp, 1.0)
print(hamiltonian)

# NOTE: the above values used can also be symbolic.
# Symbolic parameters can be very useful for a variety of reasons, as detailed in the introduction.
# In order to set a symbolic parameter, we can pass either a string or use the `qoqo_calculator_pyo3` package:
from qoqo_calculator_pyo3 import CalculatorComplex

hamiltonian.add_operator_product(hbp, "parameter")
hamiltonian.add_operator_product(hbp, CalculatorComplex.from_pair("parameter", 0.0))
```
