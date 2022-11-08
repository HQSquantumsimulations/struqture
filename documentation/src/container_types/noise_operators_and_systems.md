# Noise Operators

We describe decoherence by representing it with the Lindblad equation.
The Lindblad is a master equation determining the time evolution of the density matrix.
For pure noise terms it is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) =-i \[H, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]
with the rate matrix \\(\Gamma_{j,k}\\) and the Lindblad operator \\(L_{j}\\).

Each Lindblad operator is an operator product (in the spins case, a decoherence operator product - for more information see [spins container](../physical_types/spins) chapter). LindbladNoiseOperators are built as HashMaps (Dictionaries) of Lindblad operators and values, in order to build the non-coherent part of the Lindblad master equation:
\\[
    \sum_{j,k} \Gamma_{j,k} \left( L_{j} \rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho\\} \right)
\\].


The noise operators in struqture are

* `SpinLindbladNoiseOperator`
* `BosonLindbladNoiseOperator`
* `FermionLindbladNoiseOperator`
* `MixedLindbladNoiseOperator`

# Noise Systems

Noise systems are noise operators with a fixed size, which is given by the user in the `new` function.


The noise systems in struqture are

* `SpinLindbladNoiseSystem`
* `BosonLindbladNoiseSystem`
* `FermionLindbladNoiseSystem`
* `MixedLindbladNoiseSystem`
