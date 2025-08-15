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
# We add the spin-only term into the hamiltonian, with the correct prefactor
hamiltonian.add_operator_product(
    "S1Z:B:", delta / 2.0
)

# Second, H_B:
# We iterate over all the bosonic modes
for k in range(3):
    # We add the boson-only term into the hamiltonian, with the correct prefactor
    hamiltonian.add_operator_product(
        f"S:Bc{k}a{k}:", v_k[k] / 2.0
    )

# Third, H_C: the hermitian conjugate is implicitly stored, we don't need to add it manually
# We iterate over all the bosonic modes
for k in range(3):
    # We add the spin-boson term into the hamiltonian, with the correct prefactor
    hamiltonian.add_operator_product(
        f"S0X:Ba{k}:", omega_k[k]
    )

# Our resulting H:
print(hamiltonian)

# NOTE: the above values used can also be complex, or symbolic.
# Symbolic parameters can be very useful for a variety of reasons, as detailed in the introduction. 
hamiltonian.add_operator_product(hmp, "parameter")
```
