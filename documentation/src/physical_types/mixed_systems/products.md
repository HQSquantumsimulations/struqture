# Overview

All the mixed operators are expressed based on products of mixed indices which contain spin terms, bosonic terms and fermionic terms. The spin terms respect Pauli operator cyclicity, the bosonic terms respect bosonic commutation relations, and the fermionic terms respect fermionic anti-commutation relations.

These products respect the following relations:
\\[
    -i \sigma^x \sigma^y \sigma^z = I
\\]
\\[ \lbrack b_{k}^{\dagger}, b_{j}^{\dagger} \rbrack = 0, \\\\
    \lbrack b_{k}, b_{j} \rbrack = 0, \\\\
    \lbrack b_{k}, b_{j}^{\dagger} \rbrack = \delta_{k, j}. \\]
\\[ \lbrace c_{k}^{\dagger}, c_{j}^{\dagger} \rbrace = 0, \\\\
    \lbrace c_{k}, c_{j} \rbrace = 0, \\\\
    \lbrace c_{k}, c_{j}^{\dagger} \rbrace = \delta_{k, j}. \\]

with 
\\(b^{\dagger}\\) the bosonic creation operator, \\(b\\) the bosonic annihilation operator, \\(\lbrack ., . \rbrack\\) the bosonic commutation relations, \\(c^{\dagger}\\) the fermionic creation operator, \\(c\\) the fermionic annihilation operator, and \\(\lbrace ., . \rbrace\\) the fermionic anti-commutation relations.

**NOTE**: all of our higher-level objects accept both MixedProducts/HermitianMixedProducts/MixedDecoherenceProducts (depending on the object) as well as **symbolic notation**. If the user is just getting started using `struqture`, we recommend using the symbolic notation and skipping this section of the documentation for now, starting instead with the [coherent dynamics section](./noisefree.md).

## MixedProducts

MixedProducts are combinations of `PauliProducts`, `BosonProducts` and `FermionProducts`.

## HermitianMixedProducts

HermitianMixedProducts are the hermitian equivalent of MixedProducts. This means that even though they are constructed the same (see the `Examples` section), they internally store both that term and its hermitian conjugate. 

## MixedDecoherenceProducts

MixedDecoherenceProducts are combinations of `DecoherenceProducts`, `BosonProducts` and `FermionProducts`.

## Example

The operator product is constructed by passing an array/a list of spin terms, an array/a list of bosonic terms and an array/a list of fermionic terms.

```python
from struqture_py import mixed_systems, bosons, spins, fermions

# Building the spin term sigma^x_0 sigma^z_1
pp = spins.PauliProduct().x(0).z(1)
# Building the bosonic term b^{\dagger}_1 * b^{\dagger}_2 * b_2
bp = bosons.BosonProduct([1, 2], [2])
# Building the fermionic term c^{\dagger}_0 * c^{\dagger}_1 * c_0 * c_1
fp = fermions.FermionProduct([0, 1], [0, 1])

# Building the term sigma^x_0 sigma^z_1 b^{\dagger}_1 * b^{\dagger}_2
# * b_2 * c^{\dagger}_0 * c^{\dagger}_1 * c_0 * c_1
hmp = mixed_systems.MixedProduct([pp], [bp], [fp])

# Building the term sigma^x_0 sigma^z_1 c^{\dagger}_1 * c^{\dagger}_2 *
# c_2 * c^{\dagger}_0 * c^{\dagger}_1 * c_0 * c_1  +  h.c.
hmp = mixed_systems.HermitianMixedProduct([pp], [bp], [fp])


# Building the spin term sigma^x_0 sigma^z_1
dp = spins.DecoherenceProduct().x(0).z(1)
# Building the bosonic term b^{\dagger}_1 * b^{\dagger}_2 * b_2
bp = bosons.BosonProduct([1, 2], [0, 1])
# Building the fermionic term c^{\dagger}_0 * c^{\dagger}_1 * c_0 * c_1
fp = fermions.FermionProduct([0, 1], [0, 1])

# This will work
mdp = mixed_systems.MixedDecoherenceProduct([dp], [bp], [fp])
```
