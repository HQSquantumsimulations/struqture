# Applied example

In this example, we will create the spin-boson Hamiltonian we have used for open-system research in our [paper](https://arxiv.org/abs/2210.12138), for 1 spin and 3 bosonic modes.

The Hamiltonian reads as follows:
\\[
    \hat{H} = \hat{H}_S + \hat{H}_B + \hat{H}_C
\\]

with the spin (system) Hamiltonian \\(\hat{H}_S\\) :

\\[
    \hat{H} = \frac {\hbar \Delta} {2} \sigma^z_0,
\\]

the bosonic bath Hamiltonian \\(\hat{H}_B\\) :

\\[ 
    \hat{H} = \sum_{k=0}^2 \hbar \omega_k c_k^{\dagger} c_k,
\\]

and the coupling between system and bath \\(\hat{H}_C\\) :

\\[ 
    \hat{H} = \sigma_0^x \sum_{k=0}^2 \frac {v_k} {2} \left( c_k + c_k^{\dagger} \right)
\\]

For simplicity, we will set \\(\hbar\\) to 1.0 for this example.

Implementation:
```python
# We start by importing the Hamiltonian class, and the Product classes we will need:
# BosonProduct and PauliProduct for the terms in the Hamiltonian defined above,
# and HermitianMixedProduct to add them into the MixedHamiltonian.
from struqture_py.bosons import BosonProduct
from struqture_py.mixed_systems import (
    HermitianMixedProduct, MixedHamiltonian,
)
from struqture_py.spins import PauliProduct

# We initialize the Hamiltonian class: it should contain one spin system and one boson system, but
# no fermion systems
hamiltonian = MixedHamiltonian(1, 1, 0)

# Setting up constants:
delta = 1.0
omega_k = [2.0, 3.0, 4.0]
v_k = [5.0, 6.0, 7.0]

# First, we build H_S.
# We initialize our PauliProduct and add on a sigma_z term on spin 1
pp = PauliProduct().z(1)
# This term is a spin-only term, so the bosonic part of it will be empty
bp = BosonProduct([], [])
# We create a HermitianMixedProduct from the spin and bosonic terms 
hmp = HermitianMixedProduct([pp], [bp], [])
# We add the created HermitianMixedProduct containing all the operator information into the hamiltonian, with the correct prefactor
hamiltonian.add_operator_product(
    hmp, delta / 2.0
)

# Second, H_B:
# This term is a boson-only term, so the spin part of it will be empty
pp = PauliProduct()
# We iterate over all the bosonic modes
for k in range(3):
    # We initialize our BosonProduct with the creation operator acting on k and the annihilation operator acting on k
    bp = BosonProduct([k], [k])
    # We create a HermitianMixedProduct from the spin and bosonic terms 
    hmp = HermitianMixedProduct([pp], [bp], [])
    # We add the created HermitianMixedProduct containing all the operator information into the hamiltonian, with the correct prefactor
    hamiltonian.add_operator_product(
        hmp, v_k[k] / 2.0
    )

# Third, H_C: the hermitian conjugate is implicitly stored, we don't need to add it manually
# We create the spin part of the term by initializing a PauliProduct and adding a sigma_x term acting on spin 0
pp = PauliProduct().x(0)
# We iterate over all the bosonic modes
for k in range(3):
    # We initialize our BosonProduct with the annihilation operator acting on k
    bp = BosonProduct([], [k])
    # We create a HermitianMixedProduct from the spin and bosonic terms 
    hmp = HermitianMixedProduct([pp], [bp], [])
    # We add the created HermitianMixedProduct containing all the operator information into the hamiltonian, with the correct prefactor
    hamiltonian.add_operator_product(
        hmp, omega_k[k]
    )

# Our resulting H:
print(hamiltonian)

# NOTE: the above values used can also be complex, or symbolic.
# Symbolic parameters can be very useful for a variety of reasons, as detailed in the introduction. 
# In order to set a symbolic parameter, we can pass either a string or use the `qoqo_calculator_pyo3` package:
from qoqo_calculator_pyo3 import CalculatorComplex
hamiltonian.add_operator_product(hmp, "parameter")
# The syntax below is particularly useful for building non-hermitian operators, such as MixedOperators, as the imaginary part can then be non-zero
hamiltonian.add_operator_product(hmp, CalculatorComplex.from_pair("parameter", 0.0))
```
