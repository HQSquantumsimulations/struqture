# Operators

Operators act on a state space using HashMaps (Dictionaries) of operator products and values.

For qubits, the operators represent
\\[ 
\hat{O} = \sum_{j} \alpha_j \prod_{k=0}^N \sigma_{j, k} \\\\
    \sigma_{j, k} \in \\{ X_k, Y_k, Z_k, I_k \\}
\\]
where the \\(\sigma_{j, k}\\) are `SinglePauliOperators`.

For bosons, the operators represent
\\[ \hat{O} = \sum_{j=0}^N \alpha_j \prod_{k, l} c_{k, j}^{\dagger} c_{l, j} \\]
with 
\\(c^{\dagger}\\) the bosonic creation operator, \\(c\\) the bosonic annihilation operator 
\\[ \lbrack c_k^{\dagger}, c_j^{\dagger} \rbrack = 0, \\\\
    \lbrack c_k, c_j \rbrack = 0, \\\\
    \lbrack c_k^{\dagger}, c_j \rbrack = \delta_{k, j}. \\]

For fermions, the operators represent
\\[ \hat{O} = \sum_{j=0}^N \alpha_j \prod_{k, l} c_{k, j}^{\dagger} c_{l,j}  \\]
with 
\\(c^{\dagger}\\) the fermionionic creation operator, \\(c\\) the fermionionic annihilation operator
\\[ \lbrace c_k^{\dagger}, c_j^{\dagger} \rbrace = 0, \\\\
    \lbrace c_k, c_j \rbrace = 0, \\\\
    \lbrace c_k^{\dagger}, c_j \rbrace = \delta_{k, j}. \\]


The operators in struqture are

* `QubitOperator`
* `DecoherenceOperator`
* `PlusMinusOperator`
* `FermionOperator`
* `BosonOperator`
* `MixedOperator`

# Hamiltonians

Hamiltonians are hermitian equivalents to Operators. The operator products for Hamiltonian are hermitian, meaning that the term is stored, as well as its hermitian conjugate. Also, in order for the Hamiltonian to be hermitian, any operator product on the diagonal of the matrix of interactions must be real.


The Hamiltonians in struqture are

* `QubitHamiltonian`
* `FermionHamiltonian`
* `BosonHamiltonian`
* `MixedHamiltonian`