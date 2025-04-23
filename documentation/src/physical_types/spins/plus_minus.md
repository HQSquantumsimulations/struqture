# The {+, -, z} basis

## The basis itself

The {+, -, z} basis is defined as follows:
* I: identity matrix
\\[
\begin{pmatrix}
1 & 0\\\\
0 & 1
\end{pmatrix}
\\]

* +: \\( \sigma^+ = \frac{1}{2} ( \sigma^x + \mathrm{i} \sigma^y ) \\)
\\[
\begin{pmatrix}
0 & 1\\\\
0 & 0
\end{pmatrix}
\\]

* -: \\( \sigma^- = \frac{1}{2} ( \sigma^x - \mathrm{i} \sigma^y ) \\)
\\[
\begin{pmatrix}
0 & 0\\\\
1 & 0
\end{pmatrix}
\\]

* Z: \\( \sigma^z \\) matrix
\\[
\begin{pmatrix}
1 & 0\\\\
0 & -1
\end{pmatrix}
\\]

## Symbolic values and PlusMinusProduct

The following lines of code are equivalent ways to represent these matrices acting on spin indices, when passing them to the operators described in the rest of this section:

```python
from struqture_py.spins import PlusMinusProduct

product = PlusMinusProduct().plus(0).minus(1).z(2)  # these can be chained similarly to PlusMinusProducts
product = "0+1-2Z"
```

## Operators

`PlusMinusOperators` represent operators such as:
\\[
\hat{O} = \sum_{j} \alpha_j \prod_{k=0}^N \sigma_{j, k} \\\\
    \sigma_{j, k} \in \\{ +_k, -_k, Z_k, I_k \\} .
\\]

From a programming perspective the operators are HashMaps or Dictionaries with the `PlusMinusProducts` as keys and the coefficients \\(\alpha_j\\) as values.

### Example

Here is an example of how to build a `PlusMinusOperator`:

```python
from struqture_py import spins

# We would like to build the following operator:
# O = (1 + 1.5 * i) * sigma^+_0 * sigma^z_2

# We start by initializing our PlusMinusOperator
operator = spins.PlusMinusOperator()
# We set the term and value specified above
operator.set("0+2Z", 1.0 + 1.5j)
# We can use the `get` function to check what value/prefactor is stored for 0+2Z
assert operator.get("0+2Z") == complex(1.0, 1.5)
print(operator)

# Please note that the `set` function will set the value given, overwriting any previous value.
# Should you prefer to use and additive method, please use `add_operator_product`:
operator.add_operator_product("0+2Z", 1.0)
# NOTE: this is equivalent to: operator.add_operator_product(PlusMinusProduct().plus(0).z(2), 1.0)
print(operator)

# NOTE: the above values used can also be symbolic.
# Symbolic parameters can be very useful for a variety of reasons, as detailed in the introduction. 
# In order to set a symbolic parameter, we can pass either a string or use the `qoqo_calculator_pyo3` package:
from qoqo_calculator_pyo3 import CalculatorComplex
operator.add_operator_product("0Z1Z", "parameter")
operator.add_operator_product("0Z1Z", CalculatorComplex.from_pair("parameter", 0.0))

```

### Mathematical operations

The available mathematical operations for `PlusMinusOperator` are demonstrated below:

```python
from struqture_py.spins import PlusMinusOperator

# Setting up two test PlusMinusOperators
operator_1 = PlusMinusOperator()
operator_1.add_operator_product("0+", 1.5j)

operator_2 = PlusMinusOperator()
operator_2.add_operator_product("2Z3Z", 0.5)

# Addition & subtraction:
operator_3 = operator_1 - operator_2
operator_3 = operator_3 + operator_1

# Multiplication:
operator_1 = operator_1 * 2.0
operator_4 = operator_1 * operator_2

```

### Matrix representation: spin objects only

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
For noiseless objects (`PlusMinusOperator`), sparse operators and sparse superoperators can be constructed, as we can represent the operator as a wavefunction.

Note that the matrix representation functionality exists only for spin objects, and can't be generated for bosonic, fermionic or mixed system objects.

```python
from struqture_py import spins
from scipy.sparse import coo_matrix

# We start by building the operator we want to represent
operator = spins.PlusMinusOperator()
operator.add_operator_product("0Z1Z", 0.5)

# Using the `sparse_matrix_coo` function, we can
# return the information in scipy coo_matrix form, which can be directly fed in:
python_coo = coo_matrix(operator.sparse_matrix_coo(number_spins=2))
print(python_coo.todense())
```
## Noise Operators

We describe decoherence by representing it with the Lindblad equation.
The Lindblad equation is a master equation determining the time evolution of the density matrix.
It is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) =-i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]
with the rate matrix \\(\Gamma_{j,k}\\) and the Lindblad operator \\(L_{j}\\).

To describe spin noise we use the Lindblad equation with \\(\hat{H}=0\\).

Therefore, to describe the pure noise part of the Lindblad equation one needs the rate matrix in a well defined basis of Lindblad operators.
We use the {+, -, Z} matrices (`PlusMinusProducts`) as the operator basis.

The rate matrix and Lindblad noise model are saved as a sum over pairs of spin terms, giving the operators acting from the left and right on the density matrix.
In programming terms, the object `PlusMinusLindbladNoiseOperator` is given by a HashMap or Dictionary with the tuple (`PlusMinusProduct`, `PlusMinusProduct`) as keys and the entries in the rate matrix as values.

### Example

Here, we add the terms \\( L_0 = \sigma_0^{+} \sigma_1^{z} \\) and \\( L_1 = \sigma_0^{+} \sigma_2^{z} \\) with coefficient 1.0: 
\\[ \hat{O}_{noise}(\rho) = 1.0 \left( L_0 \rho L_1^{\dagger} - \frac{1}{2} \\{ L_1^{\dagger} L_0, \rho \\} \right) \\]

```python
from struqture_py import spins

# We start by initializing the PlusMinusLindbladNoiseOperator
operator = spins.PlusMinusLindbladNoiseOperator()

# Adding in the (0+1Z, 0+2Z) term
operator.set(("0+2Z", "0+2Z"), 1.0+1.5*1j)
print(operator)

# As with the coherent operators, the `set` function overwrites any existing value for the given key (here, a tuple of strings or PlusMinusProducts).
# Should you prefer to use and additive method, please use `add_operator_product`:
operator.add_operator_product(("0+1Z", "0+2Z"), 1.0)
# NOTE: this is equivalent to: operator.add_operator_product((PlusMinusProduct().plus(0).z(1), PlusMinusProduct().plus(0).z(2)), 1.0)

```
