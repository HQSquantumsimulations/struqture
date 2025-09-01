# Matrix Representation

All spin-objects can be converted into sparse matrices with the following convention.
If \\(M_2\\) corresponds to the matrix acting on spin 2 and \\(M_1\\) corresponds to the matrix acting on spin 1 the total matrix \\(M\\) acting on spins 0 to 2 is given by
\\[
    M = M_2 \otimes M_1 \otimes \mathbb{1}
\\]
For an \\(N\\)-spin operator a term acts on the \\(2^N\\) dimensional space of state vectors.
A superoperator operates on the \\(4^N\\) dimensional space of flattened density-matrices.
struqture uses the convention that density matrices are flattened in row-major order
\\[
    \rho = \begin{pmatrix} a & b \\\\ c & d \end{pmatrix} => \vec{\rho} = \begin{pmatrix} a \\\\ b \\\\ c \\\\ d \end{pmatrix}
\\]

## Hamiltonians and Operators

For noiseless objects (`PauliOperator`, `PauliHamiltonian`), sparse operators and sparse superoperators can be constructed, as we can represent the operator as a wavefunction.

Note that the matrix representation functionality exists only for spin objects, and can't be generated for bosonic, fermionic or mixed system objects.

```python
from struqture_py import spins
from scipy.sparse import coo_matrix

# We start by building the operator we want to represent
operator = spins.PauliOperator()
operator.add_operator_product("0Z1Z", 0.5)

# Using the `sparse_matrix_coo` function, we can
# return the information in scipy coo_matrix form, which can be directly fed in:
python_coo = coo_matrix(operator.sparse_matrix_coo(number_spins=2))
print(python_coo.todense())
```

## Open Systems and Noise Operators

For operators with noise (`PauliLindbladNoiseOperator`, `PauliLindbladOpenSystem`), however, we can only represent them as density matrices and can therefore only construct sparse superoperators.

Note that the matrix representation functionality exists only for spin objects, and can't be generated for bosonic, fermionic or mixed system objects.

```python
from struqture_py import spins
from scipy.sparse import coo_matrix

# We start by building the noise operator we want to represent
operator = spins.PauliLindbladNoiseOperator()
operator.add_operator_product(("0X2Z", "0X2Z"), 1.0 + 1.5j)

# Using the `sparse_matrix_coo` function, we can
# return the information in scipy coo_matrix form, which can be directly fed in:
python_coo = coo_matrix(operator.sparse_matrix_superoperator_coo(number_spins=3))
print(python_coo.todense())
```
