# Noise Operators

We describe decoherence by representing it with the Lindblad equation.
The Lindblad equation is a master equation determining the time evolution of the density matrix.
For pure noise terms it is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) =-i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]
with the rate matrix \\(\Gamma_{j,k}\\) and the Lindblad operator \\(L_{j}\\).

Each Lindblad operator is an operator product (in the qubit case, a decoherence operator product - for more information see [spins container](../physical_types/spins) chapter). LindbladNoiseOperators are built as HashMaps (Dictionaries) of Lindblad operators and values, in order to build the non-coherent part of the Lindblad master equation:
\\[
    \sum_{j,k} \Gamma_{j,k} \left( L_{j} \rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho\\} \right)
\\].


The noise operators in struqture are

* `PauliLindbladNoiseOperator`
* `BosonLindbladNoiseOperator`
* `FermionLindbladNoiseOperator`
* `MixedLindbladNoiseOperator`

For examples showing how to use `PauliLindbladNoiseOperator`s, please see the [the spins section](../physical_types/spins.md#examples-2).
For examples showing how to use `FermionLindbladNoiseOperator`s, please see the [the fermions section](../physical_types/fermions.md#examples-2).
For examples showing how to use `BosonLindbladNoiseOperator`s, please see the [the bosons section](../physical_types/bosons.md#examples-2).
For examples showing how to use `MixedLindbladNoiseOperator`s, please see the [the mixed system section](../physical_types/mixed_systems.md#examples-2).
