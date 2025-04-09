## Noise operators

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

### Example
Here, we add the terms \\(L_0 = \left( \sigma_0^x \sigma_1^z \right) \left( c_{b, 1}^{\dagger} c_{b, 1} \right) \left( c_{f, 0}^{\dagger} c_{f, 1}^{\dagger} c_{f, 0} c_{f, 1} \right)\\) and \\(L_1 = \left( \sigma_0^x \sigma_1^z \right) \left( c_{b, 1}^{\dagger} c_{b, 1} \right) \left( c_{f, 0}^{\dagger} c_{f, 1}^{\dagger} c_{f, 0} c_{f, 1} \right)\\) with coefficient 1.0:
\\( 1.0 \left( L_0 \rho L_1^{\dagger} - \frac{1}{2} \\{ L_1^{\dagger} L_0, \rho \\} \right) \\)

```python
from struqture_py import bosons, fermions, spins, mixed_systems

operator = mixed_systems.MixedLindbladNoiseOperator(1, 1, 1)

# Building the spin term sigma^x_0 sigma^z_1
pp_0 = spins.DecoherenceProduct().x(0).z(1)
# Building the bosonic term c_b^{\dagger}_0 * c_b_0
bp = bosons.BosonProduct([0], [0])
# Building the fermionic term c_f^{\dagger}_0 * c_f^{\dagger}_1 * c_f_0 * c_f_1
fp = fermions.FermionProduct([0, 1], [0, 1])

# Building the term sigma^x_0 sigma^z_1
# * c_b^{\dagger}_0 * c_b_0 * c_f^{\dagger}_0 * c_f^{\dagger}_1 * c_f_0 * c_f_1
mdp = mixed_systems.MixedDecoherenceProduct([pp_0], [bp], [fp])

# Adding in the mixed decoherence product
operator.add_operator_product((mdp, mdp), 1.0 + 1.5j)
print(operator)

# In python we can also use the string representation
operator = mixed_systems.MixedLindbladNoiseOperator(1, 1, 1)
operator.add_operator_product((str(mdp), str(mdp)), 1.0 + 1.5 * 1j)
print(operator)
```

## Open systems

Physically open systems are quantum systems coupled to an environment that can often be described using Lindblad type of noise.
The Lindblad master equation is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) =-i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]

In `struqture` they are composed of a Hamiltonian (MixedHamiltonian) and noise (MixedLindbladNoiseOperator).

### Example

```python
from struqture_py import bosons, fermions, spins, mixed_systems

open_system = mixed_systems.MixedLindbladOpenSystem(1, 1, 1)

# Building the spin term sigma^x_0 sigma^z_1
pp = spins.PauliProduct().x(0).z(1)
# Building the bosonic term c_b^{\dagger}_1 * c_b^{\dagger}_2 * c_b_2
bp = bosons.BosonProduct([1, 2], [2])
# Building the fermionic term c_f^{\dagger}_0 * c_f^{\dagger}_1 * c_f_0 * c_f_1
fp = fermions.FermionProduct([0, 1], [0, 1])

# Building the term sigma^x_0 sigma^z_1 * c_b^{\dagger}_1
# * c_b^{\dagger}_2 * c_b_2 * c_f^{\dagger}_0 * c_f^{\dagger}_1 * c_f_0 * c_f_1
# + h.c.
hmp = mixed_systems.HermitianMixedProduct([pp], [bp], [fp])

# Building the spin term sigma^x_0 sigma^z_1
dp = spins.DecoherenceProduct().x(0).z(1)
# Building the bosonic term c_b^{\dagger}_1 * c_b_1
bp = bosons.BosonProduct([1], [1])
# Building the fermionic term c_f^{\dagger}_0 * c_f^{\dagger}_1 * c_f_0 * c_f_1
fp = fermions.FermionProduct([0, 1], [0, 1])

# Building the term sigma^x_0 sigma^z_1 * c_b^{\dagger}_1
# * c_b_1 * c_f^{\dagger}_0 * c_f^{\dagger}_1 * c_f_0 * c_f_1
mdp = mixed_systems.MixedDecoherenceProduct([dp], [bp], [fp])

# Adding in the system term
open_system.system_add_operator_product(hmp, 2.0)
# Adding in the noise term
open_system.noise_add_operator_product((mdp, mdp), 1.0)

print(open_system)
```
