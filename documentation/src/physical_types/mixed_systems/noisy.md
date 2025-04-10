# Noise operators

We describe decoherence by representing it with the Lindblad equation.
The Lindblad equation is a master equation determining the time evolution of the density matrix.
It is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) =-i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]
with the rate matrix \\(\Gamma_{j,k}\\) and the Lindblad operator \\(L_{j}\\).
To describe the pure noise part of the Lindblad equation one needs the rate matrix in a well defined basis of Lindblad operators.
We use `MixedDecoherenceProducts` as the operator basis. To describe mixed noise we use the Lindblad equation with \\(\hat{H}=0\\).

The rate matrix and with it the Lindblad noise model is saved as a sum over pairs of `MixedDecoherenceProducts`, giving the operators acting from the left and right on the density matrix.
In programming terms the object `MixedLindbladNoiseOperators` is given by a HashMap or Dictionary with the tuple (`MixedDecoherenceProduct`, `MixedDecoherenceProduct`) as keys and the entries in the rate matrix as values.

## Example
Here, we add the terms \\(L_0 = \left( \sigma_0^x \sigma_1^z \right) \left( b_{1}^{\dagger} b_{1} \right) \left( c_{0}^{\dagger} c_{1}^{\dagger} c_{0} c_{1} \right)\\) and \\(L_1 = \left( \sigma_0^x \sigma_1^z \right) \left( b_{1}^{\dagger} b_{1} \right) \left( c_{0}^{\dagger} c_{1}^{\dagger} c_{0} c_{1} \right)\\) with coefficient 1.0:
\\( 1.0 \left( L_0 \rho L_1^{\dagger} - \frac{1}{2} \\{ L_1^{\dagger} L_0, \rho \\} \right) \\)

```python
from struqture_py import mixed_systems

# We start by initializing the MixedLindbladNoiseOperator
operator = mixed_systems.MixedLindbladNoiseOperator(1, 1, 1)

# Adding in the (sigma^x_0 sigma^z_1 * c_b^{\dagger}_0 * c_b_1 * c_f^{\dagger}_0 * c_f^{\dagger}_1 * c_f_0 * c_f_1,
# sigma^x_0 sigma^z_1 * c_b^{\dagger}_0 * c_b_1 * c_f^{\dagger}_0 * c_f^{\dagger}_1 * c_f_0 * c_f_1) term
operator.set(("S0X1Z:Bc1a1:Fc0c1a0a1", "S0X1Z:Bc1a1:Fc0c1a0a1"), 1.0 + 1.5 * 1j)
print(operator)

# As with the coherent operators, the `set` function overwrites any existing value for the given key (here, a tuple of strings or DecoherenceProducts).
# Should you prefer to use and additive method, please use `add_operator_product`:
operator.add_operator_product(("S0X1Z:Bc1a1:Fc0c1a0a1", "S0X1Z:Bc1a1:Fc0c1a0a1"), 1.0)
# NOTE: this is equivalent to: operator.add_operator_product((FermionProduct([0], [0]), FermionProduct([0], [1])), 1.0)
```

# Open systems

Physically open systems are quantum systems coupled to an environment that can often be described using Lindblad type of noise.
The Lindblad master equation is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) =-i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]

In `struqture` they are composed of a Hamiltonian (MixedHamiltonian) and noise (MixedLindbladNoiseOperator).

## Example

```python
from struqture_py import mixed_systems

# We start by initializing our MixedLindbladOpenSystem
open_system = mixed_systems.MixedLindbladOpenSystem(1, 1, 1)

# Set the sigma^x_0 * c_b^{\dagger}_0 * c_b_0 * c_f^{\dagger}_0 * c_f_0 term into the system part of the open system
open_system.system_set("S0X:Bc0a0:Fc0a0", 2.0)
# Set the sigma^x_0 * i*sigma^y_1 * c_b^{\dagger}_0 * c_b_0 * c_f^{\dagger}_0 * c_f^{\dagger}_1 * c_f_0 * c_f_1
# sigma^x_0 * sigma^z_1 * c_b^{\dagger}_0 * c_b^{\dagger}_1 * c_b_0 * c_b_1 * c_f^{\dagger}_0 * c_f_0 term into the noise part of the open system
open_system.noise_set(("S0X1iY:Bc0a0:Fc0c1a0a1", "S0X1Z:Bc0c1a0a1:Fc0a0"), 1.5)

# Please note that the `system_set` and `noise_set` functions will set the values given, overwriting any previous value.
# Should you prefer to use and additive method, please use `system_add_operator_product` and `noise_add_operator_product`:
open_system.system_add_operator_product("S0X:Bc0a0:Fc0a0", 2.0)
open_system.noise_add_operator_product(("S0X1iY:Bc0a0:Fc0c1a0a1", "S0X1Z:Bc0c1a0a1:Fc0a0"), 1.5)

print(open_system)
```
