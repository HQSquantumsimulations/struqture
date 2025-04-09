## Noise operators

We describe decoherence by representing it with the Lindblad equation.
The Lindblad equation is a master equation determining the time evolution of the density matrix.
It is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) = -i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]
with the rate matrix \\(\Gamma_{j,k}\\) and the Lindblad operator \\(L_{j}\\).

To describe bosonic noise we use the Lindblad equation with \\(\hat{H}=0\\).
Therefore, to describe the pure noise part of the Lindblad equation one needs the rate matrix in a well defined basis of Lindblad operators.
We use `BosonProducts` as the operator basis.

The rate matrix and with it the Lindblad noise model is saved as a sum over pairs of `BosonProducts`, giving the operators acting from the left and right on the density matrix.
In programming terms the object `BosonLindbladNoiseOperator` is given by a HashMap or Dictionary with the tuple (`BosonProduct`, `BosonProduct`) as keys and the entries in the rate matrix as values.

### Example

Here, we add the terms \\(L_0 = c^{\dagger}_0 c_0\\) and \\(L_1 = c^{\dagger}_0 c_0\\) with coefficient 1.0:
\\( 1.0 \left( L_0 \rho L_1^{\dagger} - \frac{1}{2} \\{ L_1^{\dagger} L_0, \rho \\} \right) \\)

```python
from struqture_py import bosons

# Setting up the operator and the product we want to add to it
operator = bosons.BosonLindbladNoiseOperator()
bp = bosons.BosonProduct([0], [0])

# Adding the product to the operator
operator.add_operator_product((bp, bp), 1.0 + 1.5j)
print(operator)

# In python we can also use the string representation
operator = bosons.BosonLindbladNoiseOperator()
operator.add_operator_product((str(bp), str(bp)), 1.0 + 1.5 * 1j)
print(operator)
```

## Open systems

Physically open systems are quantum systems coupled to an environment that can often be described using Lindblad type of noise.
The Lindblad master equation is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) =-i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]
In `struqture` they are composed of a Hamiltonian (`BosonHamiltonian`) and noise (`BosonLindbladNoiseOperator`). They have different ways to set terms in Rust and Python:

### Example

```python
from qoqo_calculator_pyo3 import CalculatorComplex
from struqture_py import bosons

open_system = bosons.BosonLindbladOpenSystem()

hbp = bosons.HermitianBosonProduct([0, 1], [0, 2])
bp = bosons.BosonProduct([0], [0])

# Adding the c_0^dag c_1^dag c_0 c_2 term to the system part of the open system
open_system.system_add_operator_product(hbp, 2.0)
# Adding the c_0^dag c_0 part to the noise part of the open system
open_system.noise_add_operator_product((bp, bp), 1.0j)

print(open_system)
```

