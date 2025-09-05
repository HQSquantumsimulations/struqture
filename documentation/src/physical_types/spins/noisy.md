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

Here, we add the terms \\( L_0 = \sigma_0^{x} \sigma_1^{z} \\) and \\( L_1 = \sigma_0^{x} \sigma_2^{z} \\) with coefficient `1.0`: 
\\[ \hat{O}_{noise}(\rho) = 1.0 \left( L_0 \rho L_1^{\dagger} - \frac{1}{2} \\{ L_1^{\dagger} L_0, \rho \\} \right) \\]

```python
from struqture_py import spins

# We start by initializing the PauliLindbladNoiseOperator
noise_operator = spins.PauliLindbladNoiseOperator()

# Adding in the (0X1Z, 0X2Z) term
noise_operator.set(("0X1Z", "0X2Z"), 1.0)
print(noise_operator)

# As with the coherent operators, the `set` function overwrites any existing value for the given key (here, a tuple of strings or DecoherenceProducts).
# Should you prefer to use and additive method, please use `add_operator_product`:
noise_operator.add_operator_product(("0X1Z", "0X2Z"), 1.0)
# NOTE: this is equivalent to: noise_operator.add_operator_product((PauliProduct().x(0).z(1), PauliProduct().x(0).z(2)), 1.0)

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
