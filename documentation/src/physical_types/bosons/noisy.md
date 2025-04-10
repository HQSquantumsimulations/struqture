# Noise operators

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

## Example

Here, we add the terms \\(L_0 = b^{\dagger}_0 b_0\\) and \\(L_1 = b^{\dagger}_0 b_1\\) with coefficient 1.0:
\\( 1.0 \left( L_0 \rho L_1^{\dagger} - \frac{1}{2} \\{ L_1^{\dagger} L_0, \rho \\} \right) \\)

```python
from struqture_py import bosons

# We start by initializing the BosonLindbladNoiseOperator
operator = bosons.BosonLindbladNoiseOperator()

# Adding in the (b^{\dagger}_0 * b_0, b^{\dagger}_0 * b_1) term
operator.set(("c0a0", "c0a1"), 1.0 + 1.5 * 1j)
print(operator)

# As with the coherent operators, the `set` function overwrites any existing value for the given key (here, a tuple of strings or DecoherenceProducts).
# Should you prefer to use and additive method, please use `add_operator_product`:
operator.add_operator_product(("c0a0", "c0a1"), 1.0)
# NOTE: this is equivalent to: operator.add_operator_product((bosonProduct([0], [0]), bosonProduct([0], [1])), 1.0)
```

# Open systems

Physically open systems are quantum systems coupled to an environment that can often be described using Lindblad type of noise.
The Lindblad master equation is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) =-i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]
In `struqture` they are composed of a Hamiltonian (`BosonHamiltonian`) and noise (`BosonLindbladNoiseOperator`). They have different ways to set terms in Rust and Python:

### Example

```python
from struqture_py import bosons

# We start by initializing our BosonLindbladOpenSystem
open_system = bosons.BosonLindbladOpenSystem()

# Set the c_b^{\dagger}_0 * c_b_0 term into the system part of the open system
open_system.system_set("c0a0", 2.0)
# Set the b^{\dagger}_0 * b^{\dagger}_1 * b_0 * b_1 b^{\dagger}_0 * b^{\dagger}_1 * b_0 * b_2 term into the noise part of the open system
open_system.noise_set(("c0c1a0a1", "c0c1a0a2"), 1.5)

# Please note that the `system_set` and `noise_set` functions will set the values given, overwriting any previous value.
# Should you prefer to use and additive method, please use `system_add_operator_product` and `noise_add_operator_product`:
open_system.system_add_operator_product("c0a0", 2.0)
open_system.noise_add_operator_product(("c0c1a0a1", "c0c1a0a2"), 1.5)

print(open_system)
```

