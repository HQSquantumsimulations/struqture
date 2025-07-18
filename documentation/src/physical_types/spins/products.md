# Overview

PauliProducts and DecoherenceProducts are the components that users use to create spin terms, e.g. \\( \sigma_0^x \sigma_1^x \\). 
`PauliProducts` can later be combined to create operators or Hamiltonians (see the [coherent dynamics section](./noisefree.md)), while `DecoherenceProducts` can be combined to create noise operators or open systems (see the [decoherent dynamics section](./noisy.md)). 

**NOTE**: all of our higher-level objects accept both PauliProducts/DecoherenceProducts (depending on the object) as well as **symbolic notation**. If the user is just getting started using `struqture`, we recommend using the symbolic notation and skipping this section of the documentation for now, starting instead with the [coherent dynamics section](./noisefree.md).

## PauliProducts

The products are built by setting the operators acting on separate spins.
PauliProducts are combinations of SinglePauliOperators on specific spin indices. These are the `SinglePauliOperators`, or Pauli matrices, that are available for PauliProducts:

## DecoherenceProducts

DecoherenceProducts are products of a decoherence operators acting on single spins. These `SingleDecoherenceOperators`
are almost identical to the `SinglePauliOperators` with the exception of an additional \\(i\\) factor and are well suited to represent decoherence properties

## Example

In Python the separate operators can be set via functions. In the python interface a PauliProduct can often be replaced by its unique string representation.
Note that when using setter methods as in `PauliProduct().x(0)`, the methods set the value of the Pauli operator acting on the corresponding spin and do not 
represent matrix multiplication, so that `PauliProduct().x(0)` is equivalent to `PauliProduct().x(0).x(0)`, the second call to the setter method `x(0)` having no effect.

```python
from struqture_py.spins import PauliProduct, DecoherenceProduct

# We can build single-spin terms:
sigma_x_0 = PauliProduct().x(0)  # sigma_x acting on spin 0
sigma_y_1 = PauliProduct().y(1)  # sigma_y acting on spin 1
sigma_z_2 = PauliProduct().z(2)  # sigma_z acting on spin 2

# As well as two-spin terms:
sigma_x_0_x_1 = PauliProduct().x(0).x(1)  # sigma_x acting on spin 0 and spin 1
sigma_y_1_z_20 = PauliProduct().y(1).z(20)  # sigma_y acting on spin 1 and sigma_z spin 20
# We can also initialize the PauliProducts from string:
sigma_y_1_z_20 = PauliProduct.from_string("1Y20Z")

# We can chain as many of these as we'd like!
# A product of a X acting on spin 0, a Y acting on spin 3 and a Z acting on spin 20
pp = PauliProduct().x(0).y(3).z(20)
# This is equivalent to the string representation
pp_string = str(pp)

# The same functionality is available for DecoherenceProducts.
# **NOTE**: The name of the y() becomes .iy() for DecoherenceProducts to match the change in matrix representation
# A product of a X acting on spin 0, a iY acting on spin 3 and a Z acting on spin 20
dp = DecoherenceProduct().x(0).iy(3).z(20)
# Often equivalent to the string representation
dp_string = str(dp)
```
