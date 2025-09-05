# Large Spin Example



```python
import numpy as np
from scipy.linalg import toeplitz
from scipy.sparse.linalg import eigsh
import matplotlib.pyplot as plt

from hqs_quantum_solver.spins import VectorSpace, Operator, isotropic_interaction, spin_z, struqture_term

# Parameters
sites_part1 = 12
sites_part2 = 50

J_strong = 1
J_weak = 0.1
```