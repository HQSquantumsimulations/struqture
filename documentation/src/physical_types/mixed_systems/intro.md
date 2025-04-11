# Mixed Systems

Struqture can be used to represent mixed operators, hamiltonians and open systems, such as:
\\[ \hat{H} = \sum_j \alpha_j \prod_k \sigma_{j, k} \prod_{l, m} b_{l, j}^{\dagger} b_{m, j} \prod_{r, s} c_{r, j}^{\dagger} c_{s, j} \\]
with commutation relations and cyclicity respected.

The simplest way that the user can interact with these matrices is by using symbolic representation: `"S0Z:Bc0a1:Fc0a0"` represents a \\( \sigma^z\ b^{\dagger}\_0 b\_1\ c^{\dagger}\_0\ c\_0 \\) term. In this string representation, each subsystem is defined by its type, and ends with a colon, in order to show where the next subsystem starts. The type is one of three options: "S" if it is a spin subsystem, "B" if it is a bosonic subsystem, and "F" if it is a fermionic subsystem.
This is a very scalable approach, as indices not mentioned in this string representation are assumed to be acted on by the identity operator: `"S7Z:Bc7a25:Fc25a7"` represents a \\( \sigma^{z}\_7\ b^{\dagger}\_7 b\_{25}\ c^{\dagger}\_{25}\ c\_7 \\) term, where all other terms (0 to 6 and 8 to 24) are acted on by \\(I\\).

However, for more fine-grain control over the operators, we invite the user to look into the `MixedProduct`, `HermitianMixedProducts` and `MixedDecoherenceProducts` classes, in the [Building blocks](./products.md) section. If not, please proceed to the [coherent](./noisefree.md) or [decoherent](./noisy.md) dynamics section.
