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
For more information over the operators, we invite the user to look into the `FermionProduct` and `HermitianFermionProduct` classes, in the [Building blocks](./products.md) section. Then, please proceed to the [coherent](./noisefree.md) or [decoherent](./noisy.md) dynamics section.
