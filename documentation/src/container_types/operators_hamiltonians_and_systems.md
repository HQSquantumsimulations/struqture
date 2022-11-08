# Operators and Systems

Operators and systems operate on state space using HashMaps (Dictionaries) of operator products and values. The distinction between operators and systems is that systems are given a fixed system size by the user when creating the object.

For spins, operators and systems represent
\\[ 
\hat{O} = \sum_{j=0}^N \alpha_j \prod_{k} \sigma(k,j) \\\\
    \sigma(k,j) \in \\{ X, Y, Z, I \\}
\\]
where the \\(\sigma(k,k)\\) are `SinglePauliOperators`.

For bosons, operators and systems represent
\\[ \hat{O} = \sum_{j=0}^N \alpha_j \prod_{k, l} c^{\dagger}(k,j)c(l,j) \\]
with 
\\(c^{\dagger}\\) the bosonic creation operator, \\(c\\) the bosonic annihilation operator 
\\[ \lbrack c^{\dagger}(k), c^{\dagger}(j) \rbrack = 0, \\\\
    \lbrack c(k), c(j) \rbrack = 0, \\\\
    \lbrack c^{\dagger}(k), c(j) \rbrack = \delta (k, j). \\]

For fermions, operators and systems represent
\\[ \hat{O} = \sum_{j=0}^N \alpha_j \prod_{k, l} c^{\dagger}(k,j)c(l,j)  \\]
with 
\\(c^{\dagger}\\) the fermionionic creation operator, \\(c\\) the fermionionic annihilation operator
\\[ \lbrace c^{\dagger}(k), c^{\dagger}(j) \rbrace = 0, \\\\
    \lbrace c(k), c(j) \rbrace = 0, \\\\
    \lbrace c^{\dagger}(k), c(j) \rbrace = \delta (k, j). \\]



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

Hamiltonians are hermitian equivalents to Operators, and HamiltonionSystems are the hermitian equivalents to Systems. The operator products for hamiltonian and hamiltonian systems are hermitian, meaning that the term is stored, as well as its hermitian conjugate. Also, in order for the hamiltonian to be hermitian, any operator product on the diagonal of the matrix of interactions must be real.


The hamiltonians and hamiltonian systems in struqture are

* `SpinHamiltonian`
* `SpinHamiltonianSystem`
* `FermionHamiltonian`
* `FermionHamiltonianSystem`
* `BosonHamiltonian`
* `BosonHamiltonianSystem`
* `MixedHamiltonian`
* `MixedHamiltonianSystem`
