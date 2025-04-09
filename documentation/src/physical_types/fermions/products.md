## Overview

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

# A product of a creation operator acting on fermionic mode 0 and an
# annihilation operator acting on fermionic mode 20
fp = FermionProduct([0], [20])
# Building the term c^{\dagger}_1 * c^{\dagger}_3 * c_0
fp = FermionProduct.create_valid_pair([3, 1], [0], 1.0)


# A product of a creation operator acting on fermionic mode 0 and an annihilation
# operator acting on fermionic mode 20, as well as a creation operator acting on
# fermionic mode 20 and an annihilation operator acting on fermionic mode 0
hfp = HermitianFermionProduct([0], [20])
# Building the term c^{\dagger}_0 * c^{\dagger}_3 * c_0 + c^{\dagger}_0 * c_3 * c_0
hfp = HermitianFermionProduct.create_valid_pair([3, 0], [0], 1.0)
```
