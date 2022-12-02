# Open Systems

Open systems represent a full system and environment. Mathematically, this means that a LindbladOpenSystem represents the entire Lindblad equation. The Lindblad equation is a master equation determining the time evolution of the density matrix:
\\[
     \dot{\rho} = \mathcal{L}(\rho) =-i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]
with the Hamiltonian of the system \\(\hat{H}\\), the rate matrix \\(\Gamma_{j,k}\\), and the Lindblad operator \\(L_{j}\\).

Each LindbladOpenSystem is therefore composed of a HamiltonianSystem:
\\[
    -i \[\hat{H}, \rho\]
\\]

and a LindbladNoiseSystem:
\\[
    \sum_{j,k} \Gamma_{j,k} \left( L_{j} \rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho\\} \right)
\\]


The open systems in struqture are

* `SpinLindbladOpenSystem`
* `BosonLindbladOpenSystem`
* `FermionLindbladOpenSystem`
* `MixedLindbladOpenSystem`
