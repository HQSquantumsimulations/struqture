# Large Spin Example


In this example we split the sites into $n_1$ sites that experience a strong interaction and $n_2$ sites that experience a weak interaction. More precisely, we consider the Hamiltonian

\\[
H = - J_\mathrm{strong} \sum_{j=0}^{n_1 - 2} \vec{S}_j \vec{S}_{j+1}
  + J_\mathrm{weak} \sum_{j < k} R_{jk} \, \vec{S}_j \vec{S}_k
  \,,
\\]
where $J_\mathrm{stong}, J_\mathrm{weak} \in \mathbb{R}$ and $R \in \mathbb{R}^{n \times n}$. The matrix $R$ is a random matrix with values between $0$ and $1$.

The Hamiltonian is defined on the space $V = V_1 \otimes V_2$, where $V_1$ contains all possible spin states on the first $n_1$ sites and
$V_2$ contains the states where at most one spin of the remaining sites is in the $\uparrow$ state.


```python
# Imports
import numpy as np
from scipy.linalg import toeplitz
from scipy.sparse.linalg import eigsh
import matplotlib.pyplot as plt
from struqture_py.spins import PauliHamiltonian, PauliOperator
from hqs_quantum_solver.spins import VectorSpace, Operator, struqture_term

# Parameters
sites_part1 = 12
sites_part2 = 50
J_strong = 1
J_weak = 0.1

# Vector spaces
## Creating the vector space $V_1$ is straight forward. We only need to call the VectorSpace constructor with the appropriate arguments.
v1 = VectorSpace(sites=sites_part1, total_spin_z="all")
## To create the space $V_2$, we need to combine two spaces. The first space just represents the state where all spins are in the $\downarrow$ state, and the second space represents the states where just one spin is in the $\uparrow$ state. We want a vector space that represents the states represented by both spaces combined. Using the `|` (or the `.span` method) the desired space is constructed from the individual spaces. Note that the total spin is measured in units of $\tfrac{1}{2}$.
v2 = (VectorSpace(sites=sites_part2, total_spin_z=-sites_part2)
      | VectorSpace(sites=sites_part2, total_spin_z=-sites_part2 + 2))
## We can use the `*` operator to obtain $V = V_1 \otimes V_2$.
v = v1 * v2

# We now set up the Hamiltonian, as specified above, using struqture
rng = np.random.default_rng(314159)
r_matrix = rng.uniform(0, 1, (v.sites, v.sites))

struqture_hamiltonian = PauliHamiltonian()
for i in range(0, sites_part1 - 2):
    struqture_hamiltonian.set(f"{i}X{i+1}X", -J_strong)
    struqture_hamiltonian.set(f"{i}Y{i+1}Y", -J_strong)
    struqture_hamiltonian.set(f"{i}Z{i+1}Z", -J_strong)

for i in range(sites_part2):
    for j in range(sites_part2 + 1, sites_part2):
        struqture_hamiltonian.add_operator_product(f"{i}X{j}X", J_weak * r_matrix[(i, j)])
        struqture_hamiltonian.add_operator_product(f"{i}Y{j}Y", J_weak * r_matrix[(i, j)])
        struqture_hamiltonian.add_operator_product(f"{i}Z{j}Z", J_weak * r_matrix[(i, j)])
## We can convert it to the HQS Quantum Solver format
H = Operator(struqture_term(struqture_hamiltonian), domain=v, strict=False)

## As a simple consistency check, we can check that the operator `H` is hermitian.
abs(H.tocsr() - H.tocsr().T.conj()).max()

# We can now inspect the Hamiltonian by computing the eigenvectors corresponding to the smallest eigenvalues of the operator.
eigvals, eigvecs = eigsh(H, k=20, which='SA')

# Now, we compute the expectation values $\langle S^z_j \rangle$ for $j = 0, \dots, n_1 + n_2 - 1$, as follows.
observables = []
for j in range(v.sites):
    struqture_observable = PauliOperator()
    struqture_observable.set(f"{j}Z", 1.0)
    observables.append(Operator(struqture_term(struqture_observable), domain=v))

def spin_z_expectation(psi):
    return [np.vecdot(psi, o.dot(psi)) for o in observables]

# Lastly, we plot the result.
plt.figure()
plt.title('Groundstate')
plt.xlabel('Sites')
plt.ylabel(r'$\langle S^z \rangle$')
plt.ylim(-1, 1)
plt.plot(spin_z_expectation(eigvecs[:,0]), 'x')
plt.show()
```
