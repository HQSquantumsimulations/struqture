# Operators and Hamiltonians

In `struqture` we distinguish between operators and Hamiltonians to avoid introducing unphysical behaviour by accident.
`PauliOperators` and `PauliHamiltonians` represent operators or Hamiltonians such as:
\\[
\hat{O} = \sum_{j} \alpha_j \prod_{k=0}^N \sigma_{j, k} \\\\
    \sigma_{j, k} \in \\{ X_k, Y_k, Z_k, I_k \\} .
\\]

While both `PauliOperators` and `PauliHamiltonians` are sums over PauliProducts, Hamiltonians are guaranteed to be hermitian. In a spin Hamiltonian, this means that the prefactor of each index has to be real.

## Example

Here is an example of how to build a `PauliOperator`:

```python
from struqture_py import spins

# We would like to build the following operator:
# O = (1 + 1.5 * i) * sigma^x_0 * sigma^z_2

# We start by initializing our PauliOperator
operator = spins.PauliOperator()
# We set the term and value specified above
operator.set("0X2Z", 1.0 + 1.5j)
# We can use the `get` function to check what value/prefactor is stored for 0X2Z
assert operator.get("0X2Z") == complex(1.0, 1.5)
print(operator)

# Please note that the `set` function will set the value given, overwriting any previous value.
# Should you prefer to use and additive method, please use `add_operator_product`:
operator.add_operator_product("0X2Z", 1.0)
# NOTE: this is equivalent to: operator.add_operator_product(PauliProduct().x(0).z(2), 1.0)
print(operator)

# NOTE: the above values used can also be symbolic.
# Symbolic parameters can be very useful for a variety of reasons, as detailed in the introduction. 
operator.add_operator_product("0Z1Z", "parameter")

```

Here is an example of how to build a `PauliHamiltonian`:
```python
from struqture_py import spins

# We would like to build the following Hamiltonian:
# H = 0.5 * (sigma^x_0 * sigma^x_1 + sigma^y_0 * sigma^y_1)

# We start by initializing our PauliHamiltonian
hamiltonian = spins.PauliHamiltonian()
# We set both of the terms and values specified above
hamiltonian.set("0X1X", 0.5)
hamiltonian.set("0Y1Y", 0.5)

# NOTE: A complex extry is not valid for a PauliHamiltonian, so the following would fail:
hamiltonian.set(pp, 1.0 + 1.5j)

# Please note that the `set` function will set the value given, overwriting any previous value.
# Should you prefer to use and additive method, please use `add_operator_product`:
hamiltonian.add_operator_product("0X2Z", 1.0)
# NOTE: this is equivalent to: hamiltonian.add_operator_product(PauliProduct().x(0).z(2), 1.0)

print(hamiltonian)

# NOTE: the above values used can also be symbolic.
# Symbolic parameters can be very useful for a variety of reasons, as detailed in the introduction. 
hamiltonian.add_operator_product("0Z1Z", "parameter")
```
## Mathematical operations

The available mathematical operations for `PauliOperator` are demonstrated below:

```python
from struqture_py.spins import PauliOperator

# Setting up two test PauliOperators
operator_1 = PauliOperator()
operator_1.add_operator_product("0X", 1.5j)

operator_2 = PauliOperator()
operator_2.add_operator_product("2Z3Z", 0.5)

# Addition & subtraction:
operator_3 = operator_1 - operator_2
operator_3 = operator_3 + operator_1

# Multiplication:
operator_1 = operator_1 * 2.0j
operator_4 = operator_1 * operator_2

```
The same mathematical operations are available for `PauliHamiltonian`. However, please note that multiplying a `PauliHamiltonian` by a complex number or another `PauliHamiltonian` will result in a `PauliOperator`, as the output is no longer guaranteed to be hermitian.
 This is shown in the snippet below.

```python
from struqture_py.spins import PauliHamiltonian

# Setting up two test PauliHamiltonian
operator_1 = PauliHamiltonian()
operator_1.add_operator_product("0X", 1.5)

operator_2 = PauliHamiltonian()
operator_2.add_operator_product("2Z3Z", 0.5)

# Addition & subtraction:
operator_3 = operator_1 - operator_2  # This remains a PauliHamiltonian
operator_3 = operator_3 + operator_1  # This remains a PauliHamiltonian

# Multiplication:
operator_1 = operator_1 * 2.0j  # ! This is now a PauliOperator !
operator_4 = operator_1 * operator_2 # ! This is now a PauliOperator !
```
