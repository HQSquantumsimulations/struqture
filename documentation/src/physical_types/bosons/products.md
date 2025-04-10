# Overview

All bosonic objects in `struqture` are expressed based on products of bosonic creation and annihilation operators, which respect bosonic commutation relations
\\[ \lbrack b_k^{\dagger}, b_j^{\dagger} \rbrack = 0, \\\\
    \lbrack b_k, b_j \rbrack = 0, \\\\
    \lbrack b_k, b_j^{\dagger} \rbrack = \delta_{k, j}. \\]

## BosonProducts

BosonProducts are simple combinations of bosonic creation and annihilation operators.

## HermitianBosonProducts

HermitianBosonProducts are the hermitian equivalent of BosonProducts. This means that even though they are constructed the same (see the next section, `Examples`), they internally store both that term and its hermitian conjugate. For instance, given the term \\(b^{\dagger}_0 b_1 b_2\\), a BosonProduct would represent \\(b^{\dagger}_0 b_1 b_2\\) while a HermitianBosonProduct would represent \\(c^{\dagger}_0 b_1 b_2 + b^{\dagger}_2 b^{\dagger}_1 b_0\\).

## Example

The operator product is constructed by passing an array or a list of integers to represent the creation indices, and an array or a list of integers to represent the annihilation indices.

Note: (Hermitian)BosonProducts can only been created from the correct ordering of indices (the wrong sequence will return an error) but we have the `create_valid_pair` function to create a valid Product from arbitrary sequences of operators which also transforms an index value according to the commutation and hermitian conjugation rules.

```python
from struqture_py.bosons import BosonProduct, HermitianBosonProduct

# A product of a creation operator acting on bosonic mode 0 and an annihilation operator
# acting on bosonic mode 20
bp = BosonProduct([0], [20])
# Building the term b^{\dagger}_1 * b^{\dagger}_3 * b_0
bp = BosonProduct.create_valid_pair([3, 1], [0], 1.0)


# A product of a creation operator acting on bosonic mode 0 and an annihilation
# operator acting on bosonic mode 20, as well as a creation operator acting on
# bosonic mode 20 and an annihilation operator acting on bosonic mode 0
hbp = HermitianBosonProduct([0], [20])
# Building the term b^{\dagger}_0 * b^{\dagger}_3 * b_0 + b^{\dagger}_0 * b_3 * b_0
hbp = HermitianBosonProduct.create_valid_pair([3, 0], [0], 1.0)
```
