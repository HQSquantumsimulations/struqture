# This is an auto generated file containing only the documentation.
# You can find the full implementation on this page:
# https://github.com/HQSquantumsimulations/struqture

"""
Bosons module of struqture Python interface

Module for representing bosonic indices (BosonProduct and HermitianBosonProduct), bosonic systems (BosonOperator and BosonHamiltonian),
and Lindblad type bosonic open systems (BosonLindbladNoiseOperator, BosonLindbladOpenSystem).
Module for representing bosonic indices (BosonProduct and HermitianBosonProduct), bosonic systems (BosonOperator and BosonHamiltonian),
and Lindblad type bosonic open systems (BosonLindbladNoiseOperator, BosonLindbladOpenSystem).

.. autosummary::
    :toctree: generated/

    BosonProduct
    HermitianBosonProduct
    BosonOperator
    BosonHamiltonian
    BosonLindbladNoiseOperator
    BosonOperator
    BosonHamiltonian
    BosonLindbladNoiseOperator
    BosonLindbladOpenSystem

"""

from .struqture_py import ProductType, SystemType, NoiseType
from typing import Optional, List, Tuple, Set, Union

class BosonProduct(ProductType):
    """
A product of bosonic creation and annihilation operators.

The BosonProduct is used as an index for non-hermitian, normal ordered bosonic operators.
A bosonic operator can be written as a sum over normal ordered products of creation and annihilation operators.
The BosonProduct is used as an index when setting or adding new summands to a bosonic operator and when querrying the
weight of a product of operators in the sum.

Args:
    creators (List[int]): List of creator sub-indices.
    annihilators (List[int]): List of annihilator sub-indices.

Returns:
    self: The new (empty) BosonProduct.

Example:
--------

.. code-block:: python

    from struqture_py.bosons import BosonProduct
    import numpy.testing as npt
    # For instance, to represent $c_0a_0$
    b_product = BosonProduct([0], [0])
    npt.assert_equal(b_product.creators(), [0])
    npt.assert_equal(b_product.annihilators(), [0])
    
"""

    def __init__(self, creators: List[int], annihilators: List[int]):
       return

    def hermitian_conjugate(self): # type: ignore
        """
Return the hermitian conjugate of self and its prefactor.

Returns:
    (self, float): The hermitian conjugate of self and the potential sign it has picked up.
"""

    def is_natural_hermitian(self) -> bool: # type: ignore
        """
Return whether self is naturally hermitian.

For spin objects, this is true when applying the hermitian conjugation does not change the sign.
For bosonic and fermionic objects, this is true when creators == annihilators.
For mixed objects, this is true when all of the spin, bosonic and fermionic parts' `is_naturally_hermitian` functions evaluate to true.

        Returns:
            bool: Whether self is naturally hermitian or not."""
    def number_creators(self) -> int:  # type: ignore
        """
Get the number of creator indices of self.

        Returns:
            int: The number of creator indices in self."""
    def number_annihilators(self) -> int:  # type: ignore
        """
Get the number of annihilator indices of self.

        Returns:
            int: The number of annihilator indices in self."""
    def current_number_modes(self) -> int:  # type: ignore
        """
Returns the maximal number of modes self acts on.

Self acts on a state space of unknown dimension.
There is only a lower bound of the dimension or number of modes based on the
maximal mode the product of operators in the index acts on.
For example an index consisting of one creator acting on mode 0 would have
a current_number_modes of one. An index consisting of one annhihilator acting on 3
would have current_number_modes of four.

        Returns:
            int: The maximal number of modes self acts on."""
    def creators(self) -> List[int]:  # type: ignore
        """
Return list of creator indices.

        Returns:
            List[int]: A list of the corresponding creator indices."""
    def annihilators(self) -> List[int]:  # type: ignore
        """
Return list of annihilator indices.

        Returns:
            List[int]: A list of the corresponding annihilator indices."""
    def remap_modes(self):  # type: ignore
        """
Remap modes according to an input dictionary.

Args:
   reordering_dictionary (dict) - The dictionary specifying the remapping. It must represent a permutation.

Returns:
  (Self, CalculatorComplex) - The instance of Self with modes remapped, and the sign resulting from symmetry/antisymmetry.
"""

    def number_creators(self) -> int: # type: ignore
        """
Get the number of creator indices of self.

Returns:
    int: The number of creator indices in self.
"""

    def number_annihilators(self) -> int: # type: ignore
        """
Get the number of annihilator indices of self.

Returns:
    int: The number of annihilator indices in self.
"""

    def current_number_modes(self) -> int: # type: ignore
        """
Returns the maximal number of modes self acts on.

Self acts on a state space of unknown dimension.
There is only a lower bound of the dimension or number of modes based on the
maximal mode the product of operators in the index acts on.
For example an index consisting of one creator acting on mode 0 would have
a current_number_modes of one. An index consisting of one annhihilator acting on 3
would have current_number_modes of four.

Returns:
    int: The maximal number of modes self acts on.
"""

    def creators(self) -> List[int]: # type: ignore
        """
Return list of creator indices.

Returns:
    List[int]: A list of the corresponding creator indices.
"""

    def annihilators(self) -> List[int]: # type: ignore
        """
Return list of annihilator indices.

Returns:
    List[int]: A list of the corresponding annihilator indices.
"""

    def remap_modes(self): # type: ignore
        """
Remap modes according to an input dictionary.

Args:
   reordering_dictionary (dict) - The dictionary specifying the remapping. It must represent a permutation.

Returns:
  (Self, CalculatorComplex) - The instance of Self with modes remapped, and the sign resulting from symmetry/antisymmetry.

Raises:
   ValueError: Input reordering dictionary is not a permutation of the indices.
"""

    def create_valid_pair(self, creators: List[int], annihilators: List[int], value: Union[float, int, str, complex]): # type: ignore
        """
Create valid pair of index and value to be set in an operator.

The first item is the valid instance of self created from the input creators and annihilators.
The second term is the input CalculatorComplex transformed according to the valid order of creators and annihilators.

Args:
   creators (List[int]): The creator indices to have in the instance of self.
   annihilators (List[int]): The annihilators indices to have in the instance of self.
   value (CalculatorComplex): The CalculatorComplex to transform.
>>>>>>> c5cc867 (Struqture 2.0 (#147))

Returns:
   (self, CalculatorComplex): The valid instance of self and the corresponding transformed CalculatorComplex.

Raises:
    TypeError: Value is not CalculatorComplex.
    ValueError: Indices given in either creators or annihilators contain a double index specification (only applicable to fermionic objects).
"""

    def from_bincode(self, input: bytearray): # type: ignore
