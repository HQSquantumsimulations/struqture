# Operators and Systems

Operators and systems act on a state space using HashMaps (Dictionaries) of operator products and values. The distinction between operators and systems is that systems are given a fixed system size by the user when creating the object.

For spins, the operators and systems represent
\\[ 
\hat{O} = \sum_{j} \alpha_j \prod_{k=0}^N \sigma_{j, k} \\\\
    \sigma_{j, k} \in \\{ X_k, Y_k, Z_k, I_k \\}
\\]
where the \\(\sigma_{j, k}\\) are `SinglePauliOperators`.

For bosons, the operators and systems represent
\\[ \hat{O} = \sum_{j=0}^N \alpha_j \prod_{k, l} c_{k, j}^{\dagger} c_{l, j} \\]
with 
\\(c^{\dagger}\\) the bosonic creation operator, \\(c\\) the bosonic annihilation operator 
\\[ \lbrack c_k^{\dagger}, c_j^{\dagger} \rbrack = 0, \\\\
    \lbrack c_k, c_j \rbrack = 0, \\\\
    \lbrack c_k^{\dagger}, c_j \rbrack = \delta_{k, j}. \\]

For fermions, the operators and systems represent
\\[ \hat{O} = \sum_{j=0}^N \alpha_j \prod_{k, l} c_{k, j}^{\dagger} c_{l,j}  \\]
with 
\\(c^{\dagger}\\) the fermionionic creation operator, \\(c\\) the fermionionic annihilation operator
\\[ \lbrace c_k^{\dagger}, c_j^{\dagger} \rbrace = 0, \\\\
    \lbrace c_k, c_j \rbrace = 0, \\\\
    \lbrace c_k^{\dagger}, c_j \rbrace = \delta_{k, j}. \\]



The operators and systems in struqture are

* `SpinOperator`
* `SpinSystem`
* `DecoherenceOperator`
* `FermionOperator`
* `FermionSystem`
* `BosonOperator`
* `BosonSystem`
* `MixedOperator`
* `MixedSystem`

# Hamiltonians and HamiltonianSystems

Hamiltonians are hermitian equivalents to Operators, and HamiltonionSystems are the hermitian equivalents to Systems. The operator products for Hamiltonian and Hamiltonian systems are hermitian, meaning that the term is stored, as well as its hermitian conjugate. Also, in order for the Hamiltonian to be hermitian, any operator product on the diagonal of the matrix of interactions must be real.


The Hamiltonians and Hamiltonian systems in struqture are

* `SpinHamiltonian`
* `SpinHamiltonianSystem`
* `FermionHamiltonian`
* `FermionHamiltonianSystem`
* `BosonHamiltonian`
* `BosonHamiltonianSystem`
* `MixedHamiltonian`
* `MixedHamiltonianSystem`
