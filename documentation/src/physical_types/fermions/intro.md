# Fermions

Struqture can be used to represent Fermion operators, hamiltonians and open systems, such as:

\\[ \hat{O} = \sum_{j=0}^N \alpha_j \left( \prod_{k=0}^N f(j, k) \right) \left( \prod_{l=0}^N g(j, l) \right) \\]
with
\\[ f(j, k) = \begin{cases} c_k^{\dagger} \\\\ \mathbb{1} \end{cases} , \\]
\\[ g(j, l) = \begin{cases} c_l \\\\ \mathbb{1} \end{cases} , \\]
and 
\\[
    \dot{\rho} = \mathcal{L}(\rho) = -i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]

The simplest way that the user can interact with these matrices is by using symbolic representation: `"c0a0"` represents a \\( c^{\dagger}\_0\ c\_0 \\) term.
**Note:** Here c is used in the equations to represent any fermion operator while b was used to represent a boson operator. However, in string serialisation strquture uses the convention that `c` always represents a creation operator, whether in the fermionic or bosonic degrees of freedom and `a` always represents an annihilation operator.
This is a very scalable approach, as indices not mentioned in this string representation are assumed to be acted on by the identity operator: `"c7a25"` represents a \\( c^{\dagger}\_7 c\_{25} \\) term, where all other terms (0 to 6 and 8 to 24) are acted on by \\(I\\).

However, for more fine-grain control over the operators, we invite the user to look into the `FermionProduct` and `HermitianFermionProduct` classes, in the [Building blocks](./products.md) section. Otherwise please proceed to the [coherent](./noisefree.md) or [decoherent](./noisy.md) dynamics section.
