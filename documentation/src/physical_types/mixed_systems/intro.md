# Mixed Systems

Struqture can be used to represent mixed operators, hamiltonians and open systems, such as:

\\[ \hat{O} = \sum_{j=0}^N \alpha_j \left( \prod_{k=0}^N f(j, k) \right) \left( \prod_{l=0}^N g(j, l) \right) \\]
with
\\[ f(j, k) = \begin{cases} c_k^{\dagger} \\\\ \mathbb{1} \end{cases} , \\]
\\[ g(j, l) = \begin{cases} c_l \\\\ \mathbb{1} \end{cases} , \\]
and 
\\[
    \dot{\rho} = \mathcal{L}(\rho) = -i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]

The simplest way that the user can interact with these matrices is by using symbolic representation: `"S0Z:Bc0a1:Fc0a0"` represents a \\( \sigma^z\ b^{\dagger}\_0 b\_1\ c^{\dagger}\_0\ c\_0 \\) term. In this string representation, each subsystem is defined by its type, and ends with a colon, in order to show where the next subsystem starts. The type is one of three options: "S" if it is a spin subsystem, "B" if it is a bosonic subsystem, and "F" if it is a fermionic subsystem.
This is a very scalable approach, as indices not mentioned in this string representation are assumed to be acted on by the identity operator: `"S7Z:Bc7a25:Fc25a7"` represents a \\( \sigma^{z}\_7\ b^{\dagger}\_7 b\_{25}\ c^{\dagger}\_{25}\ c\_7 \\) term, where all other terms (0 to 6 and 8 to 24) are acted on by \\(I\\).

However, for more fine-grain control over the operators, we invite the user to look into the `MixedProduct`, `HermitianMixedProducts` and `MixedDecoherenceProducts` classes, in the [Building blocks](./products.md) section. If not, please proceed to the [coherent](./noisefree.md) or [decoherent](./noisy.md) dynamics section.
