# This is an auto generated file containing only the documentation.
# You can find the full implementation on this page:
# https://github.com/HQSquantumsimulations/struqture

"""
Spin module of struqture Python interface

Module for representing spin indices (PauliProduct and DecoherenceProduct), spin systems (PauliOperator and PauliHamiltonian)
and Lindblad type spin open systems (PauliLindbladNoiseOperator and PauliLindbladOpenSystem).

.. autosummary::
    :toctree: generated/

    PauliProduct
    DecoherenceProduct
    PauliOperator
    PauliHamiltonian
    PauliLindbladNoiseOperator
    PauliLindbladOpenSystem

"""

from .struqture_py import ProductType, SystemType, NoiseType
import numpy
from typing import Optional, List, Tuple, Dict, Set, Union

class PauliProduct(ProductType):
    """
PauliProducts are combinations of SinglePauliOperators on specific qubits.

PauliProducts can be used in either noise-free or a noisy system.
They are representations of products of pauli matrices acting on qubits,
in order to build the terms of a hamiltonian.
For instance, to represent the term :math:`\sigma_0^{x}` :math:`\sigma_2^{x}` :

`PauliProduct().x(0).x(2)`.

PauliProduct is  supposed to be used as input for the function `set_pauli_product`,
for instance in the spin system classes PauliLindbladOpenSystem, PauliHamiltonian or PauliOperator,
or in the mixed systems as part of `MixedProduct <mixed_systems.MixedProduct>`
or as part of `HermitianMixedProduct <mixed_systems.HermitianMixedProduct>`.

Returns:

    self: The new, empty PauliProduct.

Examples
--------

.. code-block:: python

    import numpy.testing as npt
    from struqture_py.spins import PauliProduct
    pp = PauliProduct().x(0).y(1).z(2)
    pp = pp.set_pauli(3, "X")
    npt.assert_equal(pp.get(0), "X")
    npt.assert_equal(pp.keys(), [0, 1, 2, 3])

"""

    def __init__(self):
