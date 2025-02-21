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

* `PauliOperator`
* `DecoherenceOperator`
* `PlusMinusOperator`
* `FermionOperator`
* `BosonOperator`
* `MixedOperator`

# Hamiltonians

Hamiltonians are hermitian equivalents to Operators. The operator products for Hamiltonian are hermitian, meaning that the term is stored, as well as its hermitian conjugate. Also, in order for the Hamiltonian to be hermitian, any operator product on the diagonal of the matrix of interactions must be real.


The Hamiltonians in struqture are

* `PauliHamiltonian`
* `FermionHamiltonian`
* `BosonHamiltonian`
* `MixedHamiltonian`

For examples showing how to use `PauliOperator`s, `DecoherenceOperator`s, `PlusMinusOperator`s and `PauliHamiltonian`s, please see the [the spins section](../physical_types/spins.md#examples-1).
For examples showing how to use `FermionOperator`s and `FermionHamiltonian`s, please see the [the fermions section](../physical_types/fermions.md#examples-1).
For examples showing how to use `BosonOperator`s and `BosonHamiltonian`s, please see the [the bosons section](../physical_types/bosons.md#examples-1).
For examples showing how to use `MixedOperator`s and `MixedHamiltonian`s, please see the [the mixed system section](../physical_types/mixed_systems.md#examples-1).
