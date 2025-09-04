# Symbolic parameters

Symbolic parameters, or parametrisation of the classes, can be very useful for users creating Hamiltonians or operators with different coupling/on-site/... strengths.

## Example

For instance, if a user would like to define the following Hamiltonian:

\\[
    \hat{H} = J \sum_{i, j} \sigma_x^i \sigma_x^j + \alpha \sum_i \sigma_z^i
\\]

where the values for \\(J\\) range between [-1.0, 1.0] and the values for \\( \alpha \\) range between [0.2, 0.4], the user might try to define all these Hamiltonians one at a time. However, with struqture, the user can define the Hamiltonian once, passing symbolic parameters `"J"` and `"alpha"` as the coefficients, and substituting them for the correct values in situ, later. This is shown below.

```python
from struqture_py.spins import PauliHamiltonian
import py_alqorithms

# Defining the base Hamiltonian
hamiltonian = PauliHamiltonian()
number_spins = 4
for i in range(number_spins):
    for j in (i + 1, number_spins):
        hamiltonian.add_operator_product(f"{i}X{j}X", "J")
for i in range(number_spins):
    hamiltonian.add_operator_product(f"{i}Z", "alpha")

print(hamiltonian)

# This Hamiltonian can then be used later in the following fashion:
algorithm = py_alqorithms.QSWAPAlgorithm(1)
time = 1.0
for (j_coupling, alpha_coupling) in zip(range(-1.0, 1.0), range(0.2, 0.4)):
    # turn the hamiltonian into a quantum Circuit (see the qoqo documentation) using the HQS Quantum Libraries
    circuit = algorithm.create_circuit(hamiltonian, time)
    circuit_to_run = circuit.substitute_parameters({"J": j_coupling, "alpha": alpha_coupling})
    # run circuit on hardware or on a simulator
```

## Complex Symbolic Parameters

A note on complex parameters: we have several classes (`PauliOperator`, `PauliLindbladNoiseOperator`) that take complex parameters when building them.
These can also be made symbolic:
* if the entire parameter (real + imaginary parts) is symbolic, the same code as above can be used: a string is passed instead of a number when adding a product to the operator.
* if only a part (real part or imaginary part) is symbolic, while the other part is a number, the user must use the CalculatorComplex class (`qoqo_calculator_pyo3` package). This is demonstrated below.

```python
from struqture_py.spins import PauliOperator
from qoqo_calculator_pyo3 import CalculatorComplex

operator = PauliOperator()
operator.add_operator_product("0Z1Z", CalculatorComplex.from_pair("a", 1.0))  # This sets: a + i * 1.0
operator.add_operator_product("0X", CalculatorComplex.from_pair(1.0, "a"))  # This set 1.0 + i * a
operator.add_operator_product("1X", CalculatorComplex.from_pair("b", "c")) # This sets b + i * c. Please note that b and c will need to be substituted separately.
```
