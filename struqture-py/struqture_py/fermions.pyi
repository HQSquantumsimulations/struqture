# This is an auto generated file containing only the documentation.
# You can find the full implementation on this page:
# https://github.com/HQSquantumsimulations/struqture

"""
Fermions module of struqture Python interface

Module for representing fermionic indices (FermionProduct and HermitianFermionProduct), fermionic systems (FermionOperator and FermionHamiltonian),
and Lindblad type fermionic open systems (FermionLindbladNoiseOperator, FermionLindbladOpenSystem).
Module for representing fermionic indices (FermionProduct and HermitianFermionProduct), fermionic systems (FermionOperator and FermionHamiltonian),
and Lindblad type fermionic open systems (FermionLindbladNoiseOperator, FermionLindbladOpenSystem).

.. autosummary::
    :toctree: generated/

    FermionProduct
    HermitianFermionProduct
    FermionOperator
    FermionHamiltonian
    FermionLindbladNoiseOperator
    FermionOperator
    FermionHamiltonian
    FermionLindbladNoiseOperator
    FermionLindbladOpenSystem

"""

from .struqture_py import ProductType, SystemType, NoiseType
from typing import Optional, List, Tuple, Set, Union

class FermionProduct(ProductType):
    """
A product of fermionic creation and annihilation operators.

The FermionProduct is used as an index for non-hermitian, normal ordered fermionic operators.
A fermionic operator can be written as a sum over normal ordered products of creation and annihilation operators.
The FermionProduct is used as an index when setting or adding new summands to a fermionic operator and when querrying the
weight of a product of operators in the sum.

Args:
    creators (List[int]): List of creator sub-indices.
    annihilators (List[int]): List of annihilator sub-indices.

Returns:
    self: The new (empty) FermionProduct.

Examples
--------

.. code-block:: python

    from struqture_py.fermions import FermionProduct
    import numpy.testing as npt
    # For instance, to represent $c_0a_0$
    fp = FermionProduct([0], [0])
    npt.assert_equal(fp.creators(), [0])
    npt.assert_equal(fp.annihilators(), [0])
    
"""

    def __init__(self, creators: List[int], annihilators: List[int]):
