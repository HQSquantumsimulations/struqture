# Noise Operators and Open Systems

We describe decoherence by representing it with the Lindblad equation.
The Lindblad equation is a master equation determining the time evolution of the density matrix.
It is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) =-i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]
with the rate matrix \\(\Gamma_{j,k}\\) and the Lindblad operator \\(L_{j}\\).

To describe spin noise we use the Lindblad equation with \\(\hat{H}=0\\).

Therefore, to describe the pure noise part of the Lindblad equation one needs the rate matrix in a well defined basis of Lindblad operators.
We use the modified Pauli matrices {X, iY, Z} (`DecoherenceProducts`) as the operator basis.

The rate matrix and Lindblad noise model are saved as a sum over pairs of spin terms, giving the operators acting from the left and right on the density matrix.
In programming terms, the object `PauliLindbladNoiseOperator` is given by a HashMap or Dictionary with the tuple (`DecoherenceProduct`, `DecoherenceProduct`) as keys and the entries in the rate matrix as values.

### Example

Here, we add the terms \\( L_0 = \sigma_0^{x} \sigma_1^{z} \\) and \\( L_1 = \sigma_0^{x} \sigma_2^{z} \\) with coefficient 1.0: 
\\[ \hat{O}_{noise}(\rho) = 1.0 \left( L_0 \rho L_1^{\dagger} - \frac{1}{2} \\{ L_1^{\dagger} L_0, \rho \\} \right) \\]

```python
from struqture_py import spins

# We start by initializing the PauliLindbladNoiseOperator
operator = spins.PauliLindbladNoiseOperator()

# Adding in the (0X1Z, 0X2Z) term
operator.set(("0X2Z", "0X2Z"), 1.0+1.5*1j)
print(operator)

# As with the coherent operators, the `set` function overwrites any existing value for the given key (here, a tuple of strings or DecoherenceProducts).
# Should you prefer to use and additive method, please use `add_operator_product`:
operator.add_operator_product(("0X1Z", "0X2Z"), 1.0)
# NOTE: this is equivalent to: operator.add_operator_product((PauliProduct().x(0).z(1), PauliProduct().x(0).z(2)), 1.0)

```

## Open systems

Open systems are quantum systems coupled to an environment that can often be described using Lindblad-type noise.
The Lindblad master equation is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) =-i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]
In `struqture` they are composed of a hamiltonian (`PauliHamiltonian`) and noise (`PauliLindbladNoiseOperator`), representing the first and second parts of the equation (respectively).

### Example

```python
from qoqo_calculator_pyo3 import CalculatorComplex, CalculatorFloat
from struqture_py import spins

# We start by initializing our PauliLindbladOpenSystem
open_system = spins.PauliLindbladOpenSystem()

# Set the sigma_1^z term into the system part of the open system
open_system.system_set("1Z", 2.0)
# Set the sigma_0^x sigma_2^z term into the noise part of the open system
open_system.noise_set(("0X2Z", "0X2Z"), 1.5)

# Please note that the `system_set` and `noise_set` functions will set the values given, overwriting any previous value.
# Should you prefer to use and additive method, please use `system_add_operator_product` and `noise_add_operator_product`:
open_system.system_add_operator_product("1Z", 2.0)
open_system.noise_add_operator_product(("0X2Z", "0X2Z"), 1.5)

print(open_system)
```

## Matrix representation: spin objects only

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
