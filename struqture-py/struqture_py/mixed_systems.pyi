# This is an auto generated file containing only the documentation.
# You can find the full implementation on this page:
# https://github.com/HQSquantumsimulations/struqture

"""
Module for representing mixed physical systems.

A mixed physical system can contain any combination of none, one, or several subsystems
of spin, bosonic, or fermionic types.
For example a mixed system with two spin-subsystems or a mixed system with a bosonic-subsystem and a bosonic-subsystem would both be valid.

This module, here the python inferface for struqture, can be used to represent
mixed quantum indices (MixedProduct, HermitianMixedProduct and MixedDecoherenceProduct),
mixed systems (MixedOperator and MixedHamiltonian) and Lindblad type mixed open systems
(MixedLindbladNoiseOperator and MixedLindbladOpenSystem).
mixed systems (MixedOperator and MixedHamiltonian) and Lindblad type mixed open systems
(MixedLindbladNoiseOperator and MixedLindbladOpenSystem).

.. autosummary::
    :toctree: generated/

    MixedProduct
    HermitianMixedProduct
    MixedDecoherenceProduct
    MixedOperator
    MixedHamiltonian
    MixedLindbladNoiseOperator
    MixedOperator
    MixedHamiltonian
    MixedLindbladNoiseOperator
    MixedLindbladOpenSystem
    MixedPlusMinusProduct
    MixedPlusMinusOperator

"""

from .struqture_py import ProductType, SystemType, NoiseType
from typing import Optional, List, Tuple, Set, Union
from .bosons import *
from .fermions import *
from .spins import *

class MixedProduct(ProductType):
    """
A mixed product of pauli products and boson products.

A `PauliProduct <struqture_py.spins.PauliProduct>` is a representation of products of pauli matrices acting on qubits. It is used in order to build the corresponding spin terms of a hamiltonian.

A `BosonProduct <struqture_py.bosons.BosonProduct>` is a product of bosonic creation and annihilation operators.
It is used as an index for non-hermitian, normal ordered bosonic operators.

A `FermionProduct <struqture_py.fermions.FermionProduct>` is a product of bosonic creation and annihilation operators.
It is used as an index for non-hermitian, normal ordered bosonic operators.

Note: For a physical system, the `bosons` (BosonProduct) are usually considered
in presence of a `system-spin` part (PauliProduct) and a `bath-spin` part (PauliProduct),
as shown in the example below.

Args:
    spins (List[PauliProduct]): Products of pauli operators acting on qubits.
    bosons (List[BosonProduct]): Products of bosonic creation and annihilation operators.
    fermions (List[FermionProduct]): Products of fermionic creation and annihilation operators.

Returns:
    MixedProduct: a new MixedProduct with the input of spins, bosons and fermions.

Raises:
    ValueError: MixedProduct can not be constructed from the input.

Examples
--------

.. code-block:: python

    from struqture_py.mixed_systems import MixedProduct
    from struqture_py.spins import PauliProduct
    from struqture_py.bosons import BosonProduct
    
    # For instance, to represent the terms $\sigma_0^{x} \sigma_2^{x}$
    # and $\sigma_1^{x} \sigma_2^{x}$
    mp_spins_system = PauliProduct().x(0).x(2)
    mp_spins_bath = PauliProduct().x(1).x(2)

    # For instance, to represent $a_1*a_1$
    mp_bosons = BosonProduct([1], [1])
    
    mp = MixedProduct([mp_spins_system, mp_spins_bath], [mp_bosons], [])
    npt.assert_equal(mp.spins(), [mp_spins_system, mp_spins_bath])
    npt.assert_equal(mp.bosons(), [mp_bosons])
    
"""

    def __init__(self, spins: List[PauliProduct], bosons: List[BosonProduct], fermions: List[FermionProduct]):
       return
