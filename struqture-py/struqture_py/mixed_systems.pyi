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

.. autosummary::
    :toctree: generated/

    MixedProduct
    HermitianMixedProduct
    MixedDecoherenceProduct
    MixedOperator
    MixedHamiltonian
    MixedLindbladNoiseOperator
    MixedLindbladOpenSystem
    MixedPlusMinusProduct
    MixedPlusMinusOperator

"""

from .struqture_py import ProductType, SystemType, NoiseType
from typing import Optional, List, Tuple, Set, Union, Any
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

    def __init__(
        self,
        spins: List[PauliProduct],
        bosons: List[BosonProduct],
        fermions: List[FermionProduct],
    ):
        return

    def create_valid_pair(self, creators, annihilators, value):  # type: ignore
        """
        Create a pair (MixedProduct, CalculatorComplex).

        The first item is the valid MixedProduct created from the input creators and annihilators.
        The second term is the input CalculatorComplex transformed according to the valid order of creators and annihilators.

        Args:
            creators: The creator indices to have in the MixedProduct.
            annihilators: The annihilators indices to have in the MixedProduct.
            value: The CalculatorComplex to transform.

        Returns:
            Tuple[MixedProduct, CalculatorComplex] - The valid MixedProduct and the corresponding transformed CalculatorComplex.

        Raises:
            ValueError: Valid pair could not be constructed, pauli spins couldn't be converted from string.
            ValueError: Valid pair could not be constructed, bosons couldn't be converted from string.
            ValueError: Valid pair could not be constructed, fermions couldn't be converted from string.
            TypeError: Value cannot be converted to CalculatorComplex.
            ValueError: Valid pair could not be constructed.
        """

    def hermitian_conjugate(self):  # type: ignore
        """
        Return the hermitian conjugate of self and its prefactor.

        Returns:
            (self, float): The hermitian conjugate of self and the potential sign it has picked up.
        """

    def is_natural_hermitian(self) -> bool:  # type: ignore
        """
        Return whether self is naturally hermitian.

        For spin objects, this is true when applying the hermitian conjugation does not change the sign.
        For bosonic and fermionic objects, this is true when creators == annihilators.
        For mixed objects, this is true when all of the spin, bosonic and fermionic parts' `is_naturally_hermitian` functions evaluate to true.

        Returns:
            bool: Whether self is naturally hermitian or not.
        """

    def spins(self) -> List[str]:  # type: ignore
        """
        Get the spin products of self.

        Returns:
            List[str]: The spin products of self.
        """

    def bosons(self) -> List[str]:  # type: ignore
        """
        Get the boson products of self.

        Returns:
            List[str]: The boson products of self.
        """

    def fermions(self) -> List[str]:  # type: ignore
        """
        Get the fermion products of self.

        Returns:
            List[str]: The fermion products of self.
        """

    def current_number_spins(self) -> List[int]:  # type: ignore
        """
        Return the current number of spins each subsystem acts upon.

        Returns:
            List[int]: Number of spins in each spin sub-system.
        """

    def current_number_bosonic_modes(self) -> List[int]:  # type: ignore
        """
        Return the current number of bosonic modes each subsystem acts upon.

        Returns:
            List[int]: Number of bosonic modes in each spin sub-system.
        """

    def current_number_fermionic_modes(self) -> List[int]:  # type: ignore
        """
        Return the current number of fermionic modes each subsystem acts upon.

        Returns:
            List[int]: Number of fermionic modes in each spin sub-system.
        """

    def from_json_struqture_1(self, input: Any) -> Any:  # type: ignore
        """
        Convert a json corresponding to a struqture 1 object to the equivalent object in struqture 2.

        Args:
            input (Any): the json of the struqture 1 object to convert.

        Returns:
            Any: the input object in struqture 2 form.

        Raises:
            ValueError: Input could not be deserialised form json.
            ValueError: Struqture 1 object could not be converted to struqture 2.
        """

    def from_bincode(self, input: bytearray):  # type: ignore
        """
        Convert the bincode representation of the object to an instance using the [bincode] crate.

        Args:
            input (bytearray): The serialized object (in [bincode] form).

        Returns:
           The deserialized Spin System.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized.
        """

    def to_bincode(self) -> bytearray:  # type: ignore
        """
        Return the bincode representation of the object using the [bincode] crate.

        Returns:
            bytearray: The serialized object (in [bincode] form).

        Raises:
            ValueError: Cannot serialize object to bytes.
        """

    def to_json(self) -> str:  # type: ignore
        """
        Return the json representation of the object.

        Returns:
            str: The serialized form of the object.

        Raises:
            ValueError: Cannot serialize object to json.
        """

    def from_json(self, input: str):  # type: ignore
        """
        Convert the json representation of the object to an instance.

        Args:
            input (str): The serialized object in json form.

        Returns:
            The deserialized object.

        Raises:
            ValueError: Input cannot be deserialized.
        """

    def from_string(self, input: str) -> MixedProduct:  # type: ignore
        """
        Convert a string representation of the object to an instance.

        Args:
            input (str): The serialized index in str representation.

        Returns:
            self: The converted object.

        Raises:
            ValueError: Input cannot be converted from str.
        """

    def current_version(self) -> str:  # type: ignore
        """
        Returns the current version of the struqture library .

        Returns:
            str: The current version of the library.
        """

    def min_supported_version(self) -> str:  # type: ignore
        """
        Return the minimum version of struqture that supports this object.

        Returns:
            str: The minimum version of the struqture library to deserialize this object.
        """

    def _get_serialisation_meta(self):  # type: ignore
        """
        Returns the StruqtureSerialisationMeta of the object.
        """

    def json_schema(self) -> str:  # type: ignore
        """
        Return the JsonSchema for the json serialisation of the class.

        Returns:
            str: The json schema serialized to json
        """

class HermitianMixedProduct(ProductType):
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
        HermitianMixedProduct: a new HermitianMixedProduct with the input of spins,  bosons and fermions.

    Raises:
        ValueError: if HermitianMixedProduct can not be constructed from the input.

    Examples
    --------

    .. code-block:: python

        from struqture_py.mixed_systems import HermitianMixedProduct
        from struqture_py.spins import PauliProduct
        from struqture_py.bosons import BosonProduct

        # For instance, to represent the terms $\sigma_0^{x} \sigma_2^{x}$
        # and $\sigma_1^{x} \sigma_2^{x}$
        mp_spins_system = PauliProduct().x(0).x(2)
        mp_spins_bath = PauliProduct().x(1).x(2)

        # For instance, to represent $a_1*a_1$
        mp_bosons = BosonProduct([1], [1])

        mp = HermitianMixedProduct([mp_spins_system, mp_spins_bath], [mp_bosons], [])
        npt.assert_equal(mp.spins(), [mp_spins_system, mp_spins_bath])
        npt.assert_equal(mp.bosons(), [mp_bosons])

    """

    def __init__(
        self,
        spins: List[PauliProduct],
        bosons: List[BosonProduct],
        fermions: List[FermionProduct],
    ):
        return

    def create_valid_pair(self, creators, annihilators, value):  # type: ignore
        """
        Create a pair (HermitianMixedProduct, CalculatorComplex).

        The first item is the valid HermitianMixedProduct created from the input creators and annihilators.
        The second term is the input CalculatorComplex transformed according to the valid order of creators and annihilators.

        Args:
            creators: The creator indices to have in the HermitianMixedProduct.
            annihilators: The annihilators indices to have in the HermitianMixedProduct.
            value: The CalculatorComplex to transform.

        Returns:
            Tuple[self, CalculatorComplex] - The valid HermitianMixedProduct and the corresponding transformed CalculatorComplex.

        Raises:
            ValueError: Valid pair could not be constructed, pauli spins couldn't be converted from string.
            ValueError: Valid pair could not be constructed, bosons couldn't be converted from string.
            ValueError: Valid pair could not be constructed, fermions couldn't be converted from string.
            TypeError: Value cannot be converted to CalculatorComplex.
            ValueError: Valid pair could not be constructed.
        """

    def hermitian_conjugate(self):  # type: ignore
        """
        Return the hermitian conjugate of self and its prefactor.

        Returns:
            (self, float): The hermitian conjugate of self and the potential sign it has picked up.
        """

    def is_natural_hermitian(self) -> bool:  # type: ignore
        """
        Return whether self is naturally hermitian.

        For spin objects, this is true when applying the hermitian conjugation does not change the sign.
        For bosonic and fermionic objects, this is true when creators == annihilators.
        For mixed objects, this is true when all of the spin, bosonic and fermionic parts' `is_naturally_hermitian` functions evaluate to true.

        Returns:
            bool: Whether self is naturally hermitian or not.
        """

    def spins(self) -> List[str]:  # type: ignore
        """
        Get the spin products of self.

        Returns:
            List[str]: The spin products of self.
        """

    def bosons(self) -> List[str]:  # type: ignore
        """
        Get the boson products of self.

        Returns:
            List[str]: The boson products of self.
        """

    def fermions(self) -> List[str]:  # type: ignore
        """
        Get the fermion products of self.

        Returns:
            List[str]: The fermion products of self.
        """

    def current_number_spins(self) -> List[int]:  # type: ignore
        """
        Return the current number of spins each subsystem acts upon.

        Returns:
            List[int]: Number of spins in each spin sub-system.
        """

    def current_number_bosonic_modes(self) -> List[int]:  # type: ignore
        """
        Return the current number of bosonic modes each subsystem acts upon.

        Returns:
            List[int]: Number of bosonic modes in each spin sub-system.
        """

    def current_number_fermionic_modes(self) -> List[int]:  # type: ignore
        """
        Return the current number of fermionic modes each subsystem acts upon.

        Returns:
            List[int]: Number of fermionic modes in each spin sub-system.
        """

    def from_json_struqture_1(self, input: Any) -> Any:  # type: ignore
        """
        Convert a json corresponding to a struqture 1 object to the equivalent object in struqture 2.

        Args:
            input (Any): the json of the struqture 1 object to convert.

        Returns:
            Any: the input object in struqture 2 form.

        Raises:
            ValueError: Input could not be deserialised form json.
            ValueError: Struqture 1 object could not be converted to struqture 2.
        """

    def from_bincode(self, input: bytearray):  # type: ignore
        """
        Convert the bincode representation of the object to an instance using the [bincode] crate.

        Args:
            input (bytearray): The serialized object (in [bincode] form).

        Returns:
           The deserialized Spin System.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized.
        """

    def to_bincode(self) -> bytearray:  # type: ignore
        """
        Return the bincode representation of the object using the [bincode] crate.

        Returns:
            bytearray: The serialized object (in [bincode] form).

        Raises:
            ValueError: Cannot serialize object to bytes.
        """

    def to_json(self) -> str:  # type: ignore
        """
        Return the json representation of the object.

        Returns:
            str: The serialized form of the object.

        Raises:
            ValueError: Cannot serialize object to json.
        """

    def from_json(self, input: str):  # type: ignore
        """
        Convert the json representation of the object to an instance.

        Args:
            input (str): The serialized object in json form.

        Returns:
            The deserialized object.

        Raises:
            ValueError: Input cannot be deserialized.
        """

    def from_string(self, input: str) -> HermitianMixedProduct:  # type: ignore
        """
        Convert a string representation of the object to an instance.

        Args:
            input (str): The serialized index in str representation.

        Returns:
            self: The converted object.

        Raises:
            ValueError: Input cannot be converted from str.
        """

    def current_version(self) -> str:  # type: ignore
        """
        Returns the current version of the struqture library .

        Returns:
            str: The current version of the library.
        """

    def min_supported_version(self) -> str:  # type: ignore
        """
        Return the minimum version of struqture that supports this object.

        Returns:
            str: The minimum version of the struqture library to deserialize this object.
        """

    def _get_serialisation_meta(self):  # type: ignore
        """
        Returns the StruqtureSerialisationMeta of the object.
        """

    def json_schema(self) -> str:  # type: ignore
        """
        Return the JsonSchema for the json serialisation of the class.

        Returns:
            str: The json schema serialized to json
        """

class MixedDecoherenceProduct(ProductType):
    """
    A mixed product of pauli products and boson products.

    A `DecoherenceProduct <struqture_py.spins.DecoherenceProduct>` is a representation of products of pauli matrices acting on qubits. It is used in order to build the corresponding spin terms of a hamiltonian.

    A `BosonProduct <struqture_py.bosons.BosonProduct>` is a product of bosonic creation and annihilation operators.
    It is used as an index for non-hermitian, normal ordered bosonic operators.

    A `FermionProduct <struqture_py.fermions.FermionProduct>` is a product of bosonic creation and annihilation operators.
    It is used as an index for non-hermitian, normal ordered bosonic operators.

    Note: For a physical system, the `bosons` (BosonProduct) are usually considered
    in presence of a `system-spin` part (DecoherenceProduct) and a `bath-spin` part (DecoherenceProduct),
    as shown in the example below.

    Args:
        spins (List[DecoherenceProduct]): products of pauli matrices acting on qubits.
        bosons (List[BosonProduct]): products of bosonic creation and annihilation operators.
        fermions (List[FermionProduct]): products of fermionic creation and annihilation operators.

    Returns:
        MixedDecoherenceProduct: a new MixedDecoherenceProduct with the input of spins, bosons and fermions.

    Raises:
        ValueError: if MixedDecoherenceProduct can not be constructed from the input.

    Examples
    --------

    .. code-block:: python

        from struqture_py.mixed_systems import MixedDecoherenceProduct
        from struqture_py.spins import DecoherenceProduct
        from struqture_py.bosons import BosonProduct

        # For instance, to represent the terms $\sigma_0^{x} \sigma_2^{x}$
        # and $\sigma_1^{x} \sigma_2^{x}$
        mp_spins_system = DecoherenceProduct().x(0).x(2)
        mp_spins_bath = DecoherenceProduct().x(1).x(2)

        # For instance, to represent $a_1*a_1$
        mp_bosons = BosonProduct([1], [1])

        mp = MixedDecoherenceProduct([mp_spins_system, mp_spins_bath], [mp_bosons], [])
        npt.assert_equal(mp.spins(), [mp_spins_system, mp_spins_bath])
        npt.assert_equal(mp.bosons(), [mp_bosons])

    """

    def __init__(
        self,
        spins: List[DecoherenceProduct],
        bosons: List[BosonProduct],
        fermions: List[FermionProduct],
    ):
        return

    def create_valid_pair(self, creators, annihilators, value):  # type: ignore
        """
        Create a pair (MixedDecoherenceProduct, CalculatorComplex).

        The first item is the valid MixedDecoherenceProduct created from the input creators and annihilators.
        The second term is the input CalculatorComplex transformed according to the valid order of creators and annihilators.

        Args:
            creators: The creator indices to have in the MixedDecoherenceProduct.
            annihilators: The annihilators indices to have in the MixedDecoherenceProduct.
            value: The CalculatorComplex to transform.

        Returns:
            Tuple[self, CalculatorComplex] - The valid MixedDecoherenceProduct and the corresponding transformed CalculatorComplex.

        Raises:
            ValueError: Valid pair could not be constructed, spins couldn't be converted from string.
            ValueError: Valid pair could not be constructed, bosons couldn't be converted from string.
            ValueError: Valid pair could not be constructed, fermions couldn't be converted from string.
            TypeError: Value cannot be converted to CalculatorComplex.
            ValueError: Valid pair could not be constructed.
        """

    def hermitian_conjugate(self):  # type: ignore
        """
        Return the hermitian conjugate of self and its prefactor.

        Returns:
            (self, float): The hermitian conjugate of self and the potential sign it has picked up.
        """

    def is_natural_hermitian(self) -> bool:  # type: ignore
        """
        Return whether self is naturally hermitian.

        For spin objects, this is true when applying the hermitian conjugation does not change the sign.
        For bosonic and fermionic objects, this is true when creators == annihilators.
        For mixed objects, this is true when all of the spin, bosonic and fermionic parts' `is_naturally_hermitian` functions evaluate to true.

        Returns:
            bool: Whether self is naturally hermitian or not.
        """

    def spins(self) -> List[str]:  # type: ignore
        """
        Get the spin products of self.

        Returns:
            List[str]: The spin products of self.
        """

    def bosons(self) -> List[str]:  # type: ignore
        """
        Get the boson products of self.

        Returns:
            List[str]: The boson products of self.
        """

    def fermions(self) -> List[str]:  # type: ignore
        """
        Get the fermion products of self.

        Returns:
            List[str]: The fermion products of self.
        """

    def current_number_spins(self) -> List[int]:  # type: ignore
        """
        Return the current number of spins each subsystem acts upon.

        Returns:
            List[int]: Number of spins in each spin sub-system.
        """

    def current_number_bosonic_modes(self) -> List[int]:  # type: ignore
        """
        Return the current number of bosonic modes each subsystem acts upon.

        Returns:
            List[int]: Number of bosonic modes in each spin sub-system.
        """

    def current_number_fermionic_modes(self) -> List[int]:  # type: ignore
        """
        Return the current number of fermionic modes each subsystem acts upon.

        Returns:
            List[int]: Number of fermionic modes in each spin sub-system.
        """

    def from_json_struqture_1(self, input: Any) -> Any:  # type: ignore
        """
        Convert a json corresponding to a struqture 1 object to the equivalent object in struqture 2.

        Args:
            input (Any): the json of the struqture 1 object to convert.

        Returns:
            Any: the input object in struqture 2 form.

        Raises:
            ValueError: Input could not be deserialised form json.
            ValueError: Struqture 1 object could not be converted to struqture 2.
        """

    def from_bincode(self, input: bytearray):  # type: ignore
        """
        Convert the bincode representation of the object to an instance using the [bincode] crate.

        Args:
            input (bytearray): The serialized object (in [bincode] form).

        Returns:
           The deserialized Spin System.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized.
        """

    def to_bincode(self) -> bytearray:  # type: ignore
        """
        Return the bincode representation of the object using the [bincode] crate.

        Returns:
            bytearray: The serialized object (in [bincode] form).

        Raises:
            ValueError: Cannot serialize object to bytes.
        """

    def to_json(self) -> str:  # type: ignore
        """
        Return the json representation of the object.

        Returns:
            str: The serialized form of the object.

        Raises:
            ValueError: Cannot serialize object to json.
        """

    def from_json(self, input: str):  # type: ignore
        """
        Convert the json representation of the object to an instance.

        Args:
            input (str): The serialized object in json form.

        Returns:
            The deserialized object.

        Raises:
            ValueError: Input cannot be deserialized.
        """

    def from_string(self, input: str) -> MixedDecoherenceProduct:  # type: ignore
        """
        Convert a string representation of the object to an instance.

        Args:
            input (str): The serialized index in str representation.

        Returns:
            self: The converted object.

        Raises:
            ValueError: Input cannot be converted from str.
        """

    def current_version(self) -> str:  # type: ignore
        """
        Returns the current version of the struqture library .

        Returns:
            str: The current version of the library.
        """

    def min_supported_version(self) -> str:  # type: ignore
        """
        Return the minimum version of struqture that supports this object.

        Returns:
            str: The minimum version of the struqture library to deserialize this object.
        """

    def _get_serialisation_meta(self):  # type: ignore
        """
        Returns the StruqtureSerialisationMeta of the object.
        """

    def json_schema(self) -> str:  # type: ignore
        """
        Return the JsonSchema for the json serialisation of the class.

        Returns:
            str: The json schema serialized to json
        """

class MixedOperator:
    """
    These are representations of systems of mixed_systems.

    MixedOperators are characterized by a MixedOperator to represent the hamiltonian of the spin system
    and an optional number of mixed_systems.

    Args:
        number_spins (int): The number of spin subsystems in the MixedOperator.
        number_bosons (int): The number of boson subsystems in the MixedOperator.
        number_fermions (int): The number of fermion subsystems in the MixedOperator.

    Returns:
        self: The new (empty) MixedOperator.

    Examples
    --------

    .. code-block:: python

        import numpy.testing as npt
        import scipy.sparse as sp
        from qoqo_calculator_pyo3 import CalculatorComplex
        from struqture_py.mixed_systems import MixedOperator, MixedProduct
        from struqture_py.spins import PauliProduct
        from struqture_py.bosons import BosonProduct
        from struqture_py.fermions import FermionProduct

        system = MixedOperator(1, 1, 1)
        pp = MixedProduct([PauliProduct().z(0)], [BosonProduct([0], [1])], [FermionProduct([0], [0])])
        system.add_operator_product(pp, 5.0)
        npt.assert_equal(system.current_number_spins(), [2])
        npt.assert_equal(system.get(pp), CalculatorComplex(5))

    """

    def __init__(self, number_spins: int, number_bosons: int, number_fermions: int):
        return

    def keys(self) -> List[OperatorProduct]:  # type: ignore
        """
        Return a list of the unsorted keys in self.

        Returns:
            List[OperatorProduct]: The sequence of keys of the self.
        """

    def empty_clone(self, capacity: Optional[int]) -> MixedOperator:  # type: ignore
        """
        Return an instance of self that has no entries but clones all other properties, with the given capacity.

        Args:
            capacity (Optional[int]): The capacity of the new instance to create.

        Returns:
            self: An empty clone with the same properties as self, with the given capacity.
        """

    def is_empty(self) -> bool:  # type: ignore
        """
        Return true if self contains no values.

        Returns:
            bool: Whether self is empty or not.
        """

    def truncate(self, threshold: float) -> MixedOperator:  # type: ignore
        """
        Truncate self by returning a copy without entries under a threshold.

        Args:
            threshold (float): The threshold for inclusion.

        Returns:
            self: The truncated version of self.
        """

    def get(self, key) -> Union[float, int, str, complex]:  # type: ignore
        """
        Get the coefficient corresponding to the key.

        Args:
            key: Product to get the value of.

        Returns:
            CalculatorComplex: Value at key (or 0.0).

        Raises:
            ValueError: Product could not be constructed from key.
        """

    def remove(self, key: ProductType) -> Optional[Union[Union[float, int, str, complex], Union[float, int, str]]]:  # type: ignore
        """
        Remove the value of the input key.

        Args:
            key (Product type): The key of the value to remove.

         Returns:
            Optional[Union[CalculatorComplex, CalculatorFloat]]: Key existed if this is not None, and this is the value it had before it was removed.

        Raises:
            ValueError: Product could not be constructed.
        """

    def set(self, key: ProductType, value: Union[Union[float, int, str, complex], Union[float, int, str]]) -> Optional[Union[Union[float, int, str, complex], Union[float, int, str]]]:  # type: ignore
        """
        Overwrite an existing entry or set a new entry in self.

        Args:
            key (Product type): The key to set.
            value (Union[CalculatorComplex, CalculatorFloat]): The value to set.

        Returns:
            Optional[Union[CalculatorComplex, CalculatorFloat]]: Key existed if this is not None, and this is the value it had before it was overwritten.

        Raises:
            ValueError: Product could not be constructed.
        """

    def add_operator_product(self, key: ProductType):  # type: ignore
        """
        Add a new (key object, value Union[CalculatorComplex, CalculatorFloat]) pair to existing entries.

        Args:
            key (Product type): The key object

        Raises:
            TypeError: Value is not CalculatorComplex or CalculatorFloat.
            ValueError: Product could not be constructed.
            ValueError: Error in add_operator_product function of self.
        """

    def values(self) -> List[Union[Union[float, int, str, complex], Union[float, int, str]]]:  # type: ignore
        """
        Return unsorted values in self.

        Returns:
            List[Union[CalculatorComplex, CalculatorFloat]]: The sequence of values of self.
        """

    def hermitian_conjugate(self) -> MixedOperator:  # type: ignore
        """
        Return the hermitian conjugate of self.

        Returns:
            self: The hermitian conjugate of self.
        """

    def current_number_spins(self) -> int:  # type: ignore
        """
        Return the current_number_spins input of each spin subsystem of self.

        Returns:
            int: The number of spins in each spin subsystem of self.
        """

    def current_number_bosonic_modes(self) -> List[int]:  # type: ignore
        """
        Return the number of bosonic modes in each bosonic subsystem of self.

        Returns:
            list[int]: The number of bosonic modes in each bosonic subsystem of self.
        """

    def current_number_fermionic_modes(self) -> List[int]:  # type: ignore
        """
        Return the number of fermionic modes in each fermionic subsystem of self.

        Returns:
            list[int]: The number of fermionic modes in each fermionic subsystem of self.
        """

    def from_json_struqture_1(self, input: Any) -> Any:  # type: ignore
        """
        Convert a json corresponding to a struqture 1 object to the equivalent object in struqture 2.

        Args:
            input (Any): the json of the struqture 1 object to convert.

        Returns:
            Any: the input object in struqture 2 form.

        Raises:
            ValueError: Input could not be deserialised form json.
            ValueError: Struqture 1 object could not be converted to struqture 2.
        """

    def from_bincode(self, input: bytearray):  # type: ignore
        """
        Convert the bincode representation of self to an instance using the [bincode] crate.

        Args:
            input (bytearray): The serialized object (in [bincode] form).

        Returns:
           The deserialized object.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized.
        """

    def to_bincode(self) -> bytearray:  # type: ignore
        """
        Return the bincode representation of self using the [bincode] crate.

        Returns:
            bytearray: The serialized object (in [bincode] form).

        Raises:
            ValueError: Cannot serialize object to bytes.
        """

    def to_json(self) -> str:  # type: ignore
        """
        Return the json representation of self.

        Returns:
            str: The serialized form of self.

        Raises:
            ValueError: Cannot serialize object to json.
        """

    def from_json(self, input: str):  # type: ignore
        """
        Convert the json representation of self to an instance.

        Args:
            input (str): The serialized object in json form.

        Returns:
            The deserialized object.

        Raises:
            ValueError: Input cannot be deserialized.
        """

    def current_version(self) -> str:  # type: ignore
        """
        Returns the current version of the struqture library.

        Returns:
            str: The current version of the library.
        """

    def min_supported_version(self) -> str:  # type: ignore
        """
        Return the minimum version of struqture that supports this object.

        Returns:
            str: The minimum version of the struqture library to deserialize this object.
        """

    def _get_serialisation_meta(self):  # type: ignore
        """
        Returns the StruqtureSerialisationMeta of the object.
        """

    def json_schema(self) -> str:  # type: ignore
        """
        Return the JsonSchema for the json serialisation of the class.

        Returns:
            str: The json schema serialized to json
        """

class MixedHamiltonian:
    """
    These are representations of systems of mixed_systems.

    MixedHamiltonians are characterized by a MixedOperator to represent the hamiltonian of the spin system
    and an optional number of mixed_systems.

    Args:
        number_spins (int): The number of spin subsystems in the MixedHamiltonian.
        number_bosons (int): The number of boson subsystems in the MixedHamiltonian.
        number_fermions (int): The number of fermion subsystems in the MixedHamiltonian.

    Returns:
        self: The new (empty) MixedHamiltonian.

    Examples
    --------

    .. code-block:: python

        import numpy.testing as npt
        import scipy.sparse as sp
        from qoqo_calculator_pyo3 import CalculatorComplex
        from struqture_py.mixed_systems import MixedHamiltonian, HermitianMixedProduct
        from struqture_py.spins import PauliProduct
        from struqture_py.bosons import BosonProduct
        from struqture_py.fermions import FermionProduct

        system = MixedHamiltonian(1, 1, 1)
        pp = HermitianMixedProduct([PauliProduct().z(0)], [BosonProduct([0], [1])], [FermionProduct([0], [0])])
        system.add_operator_product(pp, 5.0)
        npt.assert_equal(system.current_number_spins(), [2])
        npt.assert_equal(system.get(pp), CalculatorComplex(5))

    """

    def __init__(self, number_spins: int, number_bosons: int, number_fermions: int):
        return

    def keys(self) -> List[OperatorProduct]:  # type: ignore
        """
        Return a list of the unsorted keys in self.

        Returns:
            List[OperatorProduct]: The sequence of keys of the self.
        """

    def empty_clone(self, capacity: Optional[int]) -> MixedHamiltonian:  # type: ignore
        """
        Return an instance of self that has no entries but clones all other properties, with the given capacity.

        Args:
            capacity (Optional[int]): The capacity of the new instance to create.

        Returns:
            self: An empty clone with the same properties as self, with the given capacity.
        """

    def is_empty(self) -> bool:  # type: ignore
        """
        Return true if self contains no values.

        Returns:
            bool: Whether self is empty or not.
        """

    def truncate(self, threshold: float) -> MixedHamiltonian:  # type: ignore
        """
        Truncate self by returning a copy without entries under a threshold.

        Args:
            threshold (float): The threshold for inclusion.

        Returns:
            self: The truncated version of self.
        """

    def get(self, key) -> Union[float, int, str, complex]:  # type: ignore
        """
        Get the coefficient corresponding to the key.

        Args:
            key: Product to get the value of.

        Returns:
            CalculatorComplex: Value at key (or 0.0).

        Raises:
            ValueError: Product could not be constructed from key.
        """

    def remove(self, key: ProductType) -> Optional[Union[Union[float, int, str, complex], Union[float, int, str]]]:  # type: ignore
        """
        Remove the value of the input key.

        Args:
            key (Product type): The key of the value to remove.

         Returns:
            Optional[Union[CalculatorComplex, CalculatorFloat]]: Key existed if this is not None, and this is the value it had before it was removed.

        Raises:
            ValueError: Product could not be constructed.
        """

    def set(self, key: ProductType, value: Union[Union[float, int, str, complex], Union[float, int, str]]) -> Optional[Union[Union[float, int, str, complex], Union[float, int, str]]]:  # type: ignore
        """
        Overwrite an existing entry or set a new entry in self.

        Args:
            key (Product type): The key to set.
            value (Union[CalculatorComplex, CalculatorFloat]): The value to set.

        Returns:
            Optional[Union[CalculatorComplex, CalculatorFloat]]: Key existed if this is not None, and this is the value it had before it was overwritten.

        Raises:
            ValueError: Product could not be constructed.
        """

    def add_operator_product(self, key: ProductType):  # type: ignore
        """
        Add a new (key object, value Union[CalculatorComplex, CalculatorFloat]) pair to existing entries.

        Args:
            key (Product type): The key object

        Raises:
            TypeError: Value is not CalculatorComplex or CalculatorFloat.
            ValueError: Product could not be constructed.
            ValueError: Error in add_operator_product function of self.
        """

    def values(self) -> List[Union[Union[float, int, str, complex], Union[float, int, str]]]:  # type: ignore
        """
        Return unsorted values in self.

        Returns:
            List[Union[CalculatorComplex, CalculatorFloat]]: The sequence of values of self.
        """

    def hermitian_conjugate(self) -> MixedHamiltonian:  # type: ignore
        """
        Return the hermitian conjugate of self.

        Returns:
            self: The hermitian conjugate of self.
        """

    def current_number_spins(self) -> int:  # type: ignore
        """
        Return the current_number_spins input of each spin subsystem of self.

        Returns:
            int: The number of spins in each spin subsystem of self.
        """

    def current_number_bosonic_modes(self) -> List[int]:  # type: ignore
        """
        Return the number of bosonic modes in each bosonic subsystem of self.

        Returns:
            list[int]: The number of bosonic modes in each bosonic subsystem of self.
        """

    def current_number_fermionic_modes(self) -> List[int]:  # type: ignore
        """
        Return the number of fermionic modes in each fermionic subsystem of self.

        Returns:
            list[int]: The number of fermionic modes in each fermionic subsystem of self.
        """

    def from_json_struqture_1(self, input: Any) -> Any:  # type: ignore
        """
        Convert a json corresponding to a struqture 1 object to the equivalent object in struqture 2.

        Args:
            input (Any): the json of the struqture 1 object to convert.

        Returns:
            Any: the input object in struqture 2 form.

        Raises:
            ValueError: Input could not be deserialised form json.
            ValueError: Struqture 1 object could not be converted to struqture 2.
        """

    def from_bincode(self, input: bytearray):  # type: ignore
        """
        Convert the bincode representation of self to an instance using the [bincode] crate.

        Args:
            input (bytearray): The serialized object (in [bincode] form).

        Returns:
           The deserialized object.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized.
        """

    def to_bincode(self) -> bytearray:  # type: ignore
        """
        Return the bincode representation of self using the [bincode] crate.

        Returns:
            bytearray: The serialized object (in [bincode] form).

        Raises:
            ValueError: Cannot serialize object to bytes.
        """

    def to_json(self) -> str:  # type: ignore
        """
        Return the json representation of self.

        Returns:
            str: The serialized form of self.

        Raises:
            ValueError: Cannot serialize object to json.
        """

    def from_json(self, input: str):  # type: ignore
        """
        Convert the json representation of self to an instance.

        Args:
            input (str): The serialized object in json form.

        Returns:
            The deserialized object.

        Raises:
            ValueError: Input cannot be deserialized.
        """

    def current_version(self) -> str:  # type: ignore
        """
        Returns the current version of the struqture library.

        Returns:
            str: The current version of the library.
        """

    def min_supported_version(self) -> str:  # type: ignore
        """
        Return the minimum version of struqture that supports this object.

        Returns:
            str: The minimum version of the struqture library to deserialize this object.
        """

    def _get_serialisation_meta(self):  # type: ignore
        """
        Returns the StruqtureSerialisationMeta of the object.
        """

    def json_schema(self) -> str:  # type: ignore
        """
        Return the JsonSchema for the json serialisation of the class.

        Returns:
            str: The json schema serialized to json
        """

class MixedLindbladNoiseOperator(NoiseType):
    """
    These are representations of noisy systems of mixed_systems.

    In a MixedLindbladNoiseOperator is characterized by a MixedLindbladNoiseOperator to represent the hamiltonian of the system, and an optional number of mixed_systems.

    Examples
    --------

    .. code-block:: python

        import numpy.testing as npt
        import scipy.sparse as sp
        from qoqo_calculator_pyo3 import CalculatorComplex, CalculatorFloat
        from struqture_py.mixed_systems import MixedLindbladNoiseOperator, MixedDecoherenceProduct
        from struqture_py.spins import DecoherenceProduct
        from struqture_py.bosons import BosonProduct
        from struqture_py.fermions import FermionProduct

        slns = MixedLindbladNoiseOperator(1, 1, 1)
        dp = MixedDecoherenceProduct([DecoherenceProduct().z(0)], [BosonProduct([0], [1])], [FermionProduct([0], [0])])
        slns.add_operator_product((dp, dp), 2.0)
        npt.assert_equal(slns.current_number_spins(), [1])
        npt.assert_equal(slns.get((dp, dp)), CalculatorFloat(2))

    """

    def __init__(self):
        return

    def get(self, key: Tuple[ProductType, ProductType]) -> Union[float, int, str, complex]:  # type: ignore
        """
        Get the coefficient corresponding to the key.

        Args:
            key (Tuple[Product type, Product type]): Product to get the value of.

        Returns:
            CalculatorComplex: Value at key (or 0.0).

        Raises:
            ValueError: Left-hand product could not be constructed from key.
            ValueError: Right-hand product could not be constructed from key.
        """

    def remove(self, key: Tuple[ProductType, ProductType]) -> Optional[Union[float, int, str, complex]]:  # type: ignore
        """
        Remove the value of the input object key.

        Args:
            key (Tuple[Product type, Product type]): The key of the value to remove.

        Returns:
            Optional[CalculatorComplex]: Key existed if this is not None, and this is the value it had before it was removed.

        Raises:
            ValueError: Left-hand Product could not be constructed.
            ValueError: Right-hand Product could not be constructed.
        """

    def set(self, key: Tuple[ProductType, ProductType], value: Union[float, int, str, complex]) -> Optional[Union[float, int, str, complex]]:  # type: ignore
        """
        Overwrite an existing entry or set a new entry in self.

        Args:
            key (Tuple[Product type, Product type]): The key of the value to set.
            value (CalculatorComplex): The value to set.

        Returns:
            Optional[CalculatorComplex]: Key existed if this is not None, and this is the value it had before it was overwritten.

        Raises:
            ValueError: Left-hand Product could not be constructed.
            ValueError: Right-hand Product could not be constructed.
        """

    def add_operator_product(self, key: Tuple[ProductType, ProductType], value: Union[float, int, str, complex]):  # type: ignore
        """
        Adds a new (key object, CalculatorComplex) pair to existing entries.

        Args:
            key (Tuple[Product type, Product type]): The key of the value to add.
            value (CalculatorComplex): The value to add.

        Raises:
            TypeError: Value is not CalculatorComplex or CalculatorFloat.
            ValueError: Left-hand product could not be constructed.
            ValueError: Right-hand product could not be constructed.
            ValueError: Error in add_operator_product function of self.
        """

    def keys(self) -> List[(OperatorProduct, OperatorProduct)]:  # type: ignore
        """
        Return unsorted keys in self.

        Returns:
            List[(OperatorProduct, OperatorProduct)]: The sequence of keys of self.
        """

    def values(self) -> List[Union[float, int, str, complex]]:  # type: ignore
        """
        Return unsorted values in self.

        Returns:
            List[CalculatorComplex]: The sequence of values of self.
        """

    def empty_clone(self, capacity) -> MixedLindbladNoiseOperator:  # type: ignore
        """
        Return an instance of self that has no entries but clones all other properties, with the given capacity.

        Args:
            capacity: The capacity of the object to create.

        Returns:
            self: An empty clone with the same properties as self, with the given capacity.
        """

    def is_empty(self) -> bool:  # type: ignore
        """
        Return true if object contains no values.

        Returns:
            bool: Whether self is empty or not.
        """

    def truncate(self, threshold) -> MixedLindbladNoiseOperator:  # type: ignore
        """
        Truncate self by returning a copy without entries under a threshold.

        Args:
            threshold: The threshold for inclusion.

        Returns:
            self: The truncated version of self.
        """

    def current_number_spins(self) -> int:  # type: ignore
        """
        Return the current_number_spins input of each spin subsystem of self.

        Returns:
            int: The number of spins in each spin subsystem of self.
        """

    def current_number_bosonic_modes(self) -> List[int]:  # type: ignore
        """
        Return the number of bosonic modes in each bosonic subsystem of self.

        Returns:
            list[int]: The number of bosonic modes in each bosonic subsystem of self.
        """

    def current_number_fermionic_modes(self) -> List[int]:  # type: ignore
        """
        Return the number of fermionic modes in each fermionic subsystem of self.

        Returns:
            list[int]: The number of fermionic modes in each fermionic subsystem of self.
        """

    def from_json_struqture_1(self, input: Any) -> Any:  # type: ignore
        """
        Convert a json corresponding to a struqture 1 object to the equivalent object in struqture 2.

        Args:
            input (Any): the json of the struqture 1 object to convert.

        Returns:
            Any: the input object in struqture 2 form.

        Raises:
            ValueError: Input could not be deserialised form json.
            ValueError: Struqture 1 object could not be converted to struqture 2.
        """

    def from_bincode(self, input: bytearray):  # type: ignore
        """
        Convert the bincode representation of the object to an instance using the [bincode] crate.

        Args:
            input (bytearray): The serialized object (in [bincode] form).

        Returns:
           The deserialized object.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized.
        """

    def to_bincode(self) -> bytearray:  # type: ignore
        """
        Return the bincode representation of the object using the [bincode] crate.

        Returns:
            bytearray: The serialized object (in [bincode] form).

        Raises:
            ValueError: Cannot serialize object to bytes.
        """

    def to_json(self) -> str:  # type: ignore
        """
        Return the json representation of the object.

        Returns:
            str: The serialized form of the object.

        Raises:
            ValueError: Cannot serialize object to json.
        """

    def from_json(self, input: str):  # type: ignore
        """
        Convert the json representation of the object to an instance.

        Args:
            input (str): The serialized object in json form.

        Returns:
            The deserialized object.

        Raises:
            ValueError: Input cannot be deserialized.
        """

    def current_version(self) -> str:  # type: ignore
        """
        Returns the current version of the struqture library .

        Returns:
            str: The current version of the library.
        """

    def min_supported_version(self) -> str:  # type: ignore
        """
        Return the minimum version of struqture that supports this object.

        Returns:
            str: The minimum version of the struqture library to deserialize this object.
        """

    def _get_serialisation_meta(self):  # type: ignore
        """
        Returns the StruqtureSerialisationMeta of the object.
        """

    def json_schema(self) -> str:  # type: ignore
        """
        Return the JsonSchema for the json serialisation of the class.

        Returns:
            str: The json schema serialized to json
        """

class MixedLindbladOpenSystem(SystemType):
    """
    These are representations of noisy systems of mixed_systems.

    In a MixedLindbladOpenSystem is characterized by a MixedLindbladOpenOperator to represent the hamiltonian of the system, and an optional number of mixed_systems.

    Args:
        number_spins (int): The number of spin subsystems in the MixedLindbladOpenSystem.
        number_bosons (int): The number of boson subsystems in the MixedLindbladOpenSystem.
        number_fermions (int): The number of fermion subsystems in the MixedLindbladOpenSystem.

    Returns:
        self: The new MixedLindbladOpenSystem.

    Examples
    --------

    .. code-block:: python
        import numpy.testing as npt
        import scipy.sparse as sp
        from qoqo_calculator_pyo3 import CalculatorComplex, CalculatorFloat
        from struqture_py.mixed_systems import MixedLindbladOpenSystem
        from struqture_py.spins import DecoherenceProduct
        from struqture_py.bosons import BosonProduct
        from struqture_py.fermions import FermionProduct

        slns = MixedLindbladOpenSystem()
        dp = MixedDecoherenceProduct([DecoherenceProduct().z(0)], [BosonProduct([0], [1])], [FermionProduct([0], [0])])
        slns.noise_add_operator_product((dp, dp), 2.0)
        npt.assert_equal(slns.current_number_spins(), [1])
        npt.assert_equal(slns.noise().get((dp, dp)), CalculatorFloat(2))

    """

    def __init__(self, number_spins: int, number_bosons: int, number_fermions: int):
        return

    def system(self) -> SystemType:  # type: ignore
        """
        Return the system part of self.

        Returns:
            System type: The system of self.
        """

    def noise(self) -> NoiseType:  # type: ignore
        """
        Return the noise part of self.

        Returns:
            Noise type: The noise of self.
        """

    def ungroup(self):  # type: ignore
        """
        Return a tuple of the system and the noise of self.

        Returns:
            (System, Noise): The system and noise of self.
        """

    def group(self, system, noise) -> MixedLindbladOpenSystem:  # type: ignore
        """
        Take a tuple of a system term and a noise term and combines them to be a OpenSystem.

        Args:
            system: The system to have in the new instance.
            noise: The noise to have in the new instance.

        Returns:
            self: The OpenSystem with input system and noise terms.

        Raises:
            ValueError: System could not be constructed.
            ValueError: Noise could not be constructed.
            ValueError: Grouping could not be constructed.
        """

    def empty_clone(self) -> MixedLindbladOpenSystem:  # type: ignore
        """
        Return an instance of self that has no entries but clones all other properties, with the given capacity.

        Returns:
            self: An empty clone with the same properties as self, with the given capacity.
        """

    def truncate(self, threshold) -> MixedLindbladOpenSystem:  # type: ignore
        """
        Truncate self by returning a copy without entries under a threshold.

        Args:
            threshold: The threshold for inclusion.

        Returns:
            self: The truncated version of self.
        """

    def system_set(self, key: ProductType, value: Union[Union[float, int, str, complex], Union[float, int, str]]) -> OpenSystem:  # type: ignore
        """
        Set a new entry in the system of the open system.

        Args:
            key (Product type): Product key of set object.
            value (Union[CalculatorComplex, CalculatorFloat]): Value of set object.

        Returns:
            OpenSystem: The OpenSystem with the new entry.

        Raises:
            ValueError: key element cannot be converted to product.
            TypeError: Value cannot be converted to Union[CalculatorComplex, CalculatorFloat].
        """

    def noise_set(self, key: Tuple[ProductType, ProductType], value: Union[float, int, str, complex]) -> OpenSystem:  # type: ignore
        """
        Set a new entry in the noise of the open system.

        Args:
            key (Tuple[Product type, Product type]): Tuple of Products of set object.
            value (CalculatorComplex): CalculatorComplex value of set object.

        Returns:
            OpenSystem: The OpenSystem with the new entry.

        Raises:
            ValueError: Left key element cannot be converted to product.
            ValueError: Right key element cannot be converted to product.
            TypeError: Value cannot be converted to Union[CalculatorComplex, CalculatorFloat].
        """

    def system_get(self, key: ProductType) -> Union[float, int, str, complex] or Union[float, int, str]:  # type: ignore
        """
        Get the CalculatorComplex or CalculatorFloat coefficient corresponding to the key.

        Args:
            key (Product type): Product key of set object.

        Returns:
            CalculatorComplex or CalculatorFloat: Value at key (or 0.0).

        Raises:
            ValueError: key element cannot be converted to product.
        """

    def noise_get(self, key: Tuple[ProductType, ProductType]) -> Union[float, int, str, complex]:  # type: ignore
        """
        Get the CalculatorComplex coefficient corresponding to the key.

        Args:
            key (Tuple[Product type, Product type]): Tuple of Products of set object.

        Returns:
            CalculatorComplex: Value at key (or 0.0).

        Raises:
            ValueError: Left key element cannot be converted to product.
            ValueError: Right key element cannot be converted to product.
        """

    def system_add_operator_product(self, key: ProductType, value: Union[Union[float, int, str, complex], Union[float, int, str]]) -> OpenSystem:  # type: ignore
        """
        Add a new entry to the system of the open system.

        Args:
            key (Product type): Product key of set object.
            value (Union[CalculatorComplex, CalculatorFloat]): Value of set object.

        Returns:
            OpenSystem: The OpenSystem with the new entry.

        Raises:
            ValueError: key element cannot be converted to product.
            TypeError: Value cannot be converted to Union[CalculatorComplex, CalculatorFloat].
        """

    def noise_add_operator_product(self, key: Tuple[ProductType, ProductType], value: Union[float, int, str, complex]) -> OpenSystem:  # type: ignore
        """
        Add a new entry to the system of the open system.

        Args:
            key (Tuple[Product type, Product type]): Tuple of Products of set object.
            value (CalculatorComplex): Value of set object.

        Returns:
            OpenSystem: The OpenSystem with the new entry.

        Raises:
            ValueError: Left key element cannot be converted to product.
            ValueError: Right key element cannot be converted to product.
            TypeError: Value cannot be converted to CalculatorComplex.
        """

    def current_number_spins(self) -> int:  # type: ignore
        """
        Return the current_number_spins input of each spin subsystem of self.

        Returns:
            int: The number of spins in each spin subsystem of self.
        """

    def current_number_bosonic_modes(self) -> List[int]:  # type: ignore
        """
        Return the number of bosonic modes in each bosonic subsystem of self.

        Returns:
            list[int]: The number of bosonic modes in each bosonic subsystem of self.
        """

    def current_number_fermionic_modes(self) -> List[int]:  # type: ignore
        """
        Return the number of fermionic modes in each fermionic subsystem of self.

        Returns:
            list[int]: The number of fermionic modes in each fermionic subsystem of self.
        """

    def from_json_struqture_1(self, input: Any) -> Any:  # type: ignore
        """
        Convert a json corresponding to a struqture 1 object to the equivalent object in struqture 2.

        Args:
            input (Any): the json of the struqture 1 object to convert.

        Returns:
            Any: the input object in struqture 2 form.

        Raises:
            ValueError: Input could not be deserialised form json.
            ValueError: Struqture 1 object could not be converted to struqture 2.
        """

    def from_bincode(self, input: bytearray):  # type: ignore
        """
        Convert the bincode representation of the object to an instance using the [bincode] crate.

        Args:
            input (bytearray): The serialized object (in [bincode] form).

        Returns:
           The deserialized object.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized.
        """

    def to_bincode(self) -> bytearray:  # type: ignore
        """
        Return the bincode representation of the object using the [bincode] crate.

        Returns:
            bytearray: The serialized object (in [bincode] form).

        Raises:
            ValueError: Cannot serialize object to bytes.
        """

    def to_json(self) -> str:  # type: ignore
        """
        Return the json representation of the object.

        Returns:
            str: The serialized form of the object.

        Raises:
            ValueError: Cannot serialize object to json.
        """

    def from_json(self, input: str):  # type: ignore
        """
        Convert the json representation of the object to an instance.

        Args:
            input (str): The serialized object in json form.

        Returns:
            The deserialized object.

        Raises:
            ValueError: Input cannot be deserialized.
        """

    def current_version(self) -> str:  # type: ignore
        """
        Returns the current version of the struqture library .

        Returns:
            str: The current version of the library.
        """

    def min_supported_version(self) -> str:  # type: ignore
        """
        Return the minimum version of struqture that supports this object.

        Returns:
            str: The minimum version of the struqture library to deserialize this object.
        """

    def _get_serialisation_meta(self):  # type: ignore
        """
        Returns the StruqtureSerialisationMeta of the object.
        """

    def json_schema(self) -> str:  # type: ignore
        """
        Return the JsonSchema for the json serialisation of the class.

        Returns:
            str: The json schema serialized to json
        """

class MixedPlusMinusProduct(ProductType):
    """
    A mixed product of pauli products and boson products.

    A `PlusMinusProduct <struqture_py.spins.PlusMinusProduct>` is a representation of products of pauli matrices acting on qubits. It is used in order to build the corresponding spin terms of a hamiltonian.

    A `BosonProduct <struqture_py.bosons.BosonProduct>` is a product of bosonic creation and annihilation operators.
    It is used as an index for non-hermitian, normal ordered bosonic operators.

    A `FermionProduct <struqture_py.fermions.FermionProduct>` is a product of bosonic creation and annihilation operators.
    It is used as an index for non-hermitian, normal ordered bosonic operators.

    Note: For a physical system, the `bosons` (BosonProduct) are usually considered
    in presence of a `system-spin` part (PlusMinusProduct) and a `bath-spin` part (PlusMinusProduct),
    as shown in the example below.

    Args:
        spins (List[PlusMinusProduct]): Products of pauli operators acting on qubits.
        bosons (List[BosonProduct]): Products of bosonic creation and annihilation operators.
        fermions (List[FermionProduct]): Products of fermionic creation and annihilation operators.

    Returns:
        MixedPlusMinusProduct: a new MixedPlusMinusProduct with the input of spins, bosons and fermions.

    Raises:
        ValueError: MixedPlusMinusProduct can not be constructed from the input.

    Examples
    --------

    .. code-block:: python

        from struqture_py.mixed_systems import MixedPlusMinusProduct
        from struqture_py.spins import PlusMinusProduct
        from struqture_py.bosons import BosonProduct

        # For instance, to represent the terms $\sigma_0^{x} \sigma_2^{x}$
        # and $\sigma_1^{x} \sigma_2^{x}$
        mp_spins_system = PlusMinusProduct().x(0).x(2)
        mp_spins_bath = PlusMinusProduct().x(1).x(2)

        # For instance, to represent $a_1*a_1$
        mp_bosons = BosonProduct([1], [1])

        mp = MixedPlusMinusProduct([mp_spins_system, mp_spins_bath], [mp_bosons], [])
        npt.assert_equal(mp.spins(), [mp_spins_system, mp_spins_bath])
        npt.assert_equal(mp.bosons(), [mp_bosons])

    """

    def __init__(
        self,
        spins: List[PlusMinusProduct],
        bosons: List[BosonProduct],
        fermions: List[FermionProduct],
    ):
        return

    def from_mixed_product(self, value: MixedProduct) -> List[Tuple[(MixedPlusMinusProduct, Union[float, int, str, complex])]]:  # type: ignore
        """
        Creates a list of corresponding (MixedPlusMinusProduct, CalculatorComplex) tuples from the input MixedProduct.

        Args:
            value (MixedProduct): The MixedProduct object to convert.

        Returns:
            List[Tuple[(MixedPlusMinusProduct, CalculatorComplex)]]: The converted input.

        Raises:
            ValueError: Input is not a MixedProduct.
        """

    def to_mixed_product_list(self) -> List[Tuple[(MixedProduct, Union[float, int, str, complex])]]:  # type: ignore
        """
        Convert the `self` instance to the corresponding list of (MixedProduct, CalculatorComplex) instances.

        Returns:
            List[Tuple[(MixedProduct, CalculatorComplex)]]: The converted MixedPlusMinusProduct.

        Raises:
            ValueError: The conversion was not successful.
        """

    def hermitian_conjugate(self):  # type: ignore
        """
        Return the hermitian conjugate of self and its prefactor.

        Returns:
            (self, float): The hermitian conjugate of self and the potential sign it has picked up.
        """

    def is_natural_hermitian(self) -> bool:  # type: ignore
        """
        Return whether self is naturally hermitian.

        For spin objects, this is true when applying the hermitian conjugation does not change the sign.
        For bosonic and fermionic objects, this is true when creators == annihilators.
        For mixed objects, this is true when all of the spin, bosonic and fermionic parts' `is_naturally_hermitian` functions evaluate to true.

        Returns:
            bool: Whether self is naturally hermitian or not.
        """

    def spins(self) -> List[str]:  # type: ignore
        """
        Get the spin products of self.

        Returns:
            List[str]: The spin products of self.
        """

    def bosons(self) -> List[str]:  # type: ignore
        """
        Get the boson products of self.

        Returns:
            List[str]: The boson products of self.
        """

    def fermions(self) -> List[str]:  # type: ignore
        """
        Get the fermion products of self.

        Returns:
            List[str]: The fermion products of self.
        """

    def current_number_spins(self) -> List[int]:  # type: ignore
        """
        Return the current number of spins each subsystem acts upon.

        Returns:
            List[int]: Number of spins in each spin sub-system.
        """

    def current_number_bosonic_modes(self) -> List[int]:  # type: ignore
        """
        Return the current number of bosonic modes each subsystem acts upon.

        Returns:
            List[int]: Number of bosonic modes in each spin sub-system.
        """

    def current_number_fermionic_modes(self) -> List[int]:  # type: ignore
        """
        Return the current number of fermionic modes each subsystem acts upon.

        Returns:
            List[int]: Number of fermionic modes in each spin sub-system.
        """

    def from_json_struqture_1(self, input: Any) -> Any:  # type: ignore
        """
        Convert a json corresponding to a struqture 1 object to the equivalent object in struqture 2.

        Args:
            input (Any): the json of the struqture 1 object to convert.

        Returns:
            Any: the input object in struqture 2 form.

        Raises:
            ValueError: Input could not be deserialised form json.
            ValueError: Struqture 1 object could not be converted to struqture 2.
        """

    def from_bincode(self, input: bytearray):  # type: ignore
        """
        Convert the bincode representation of the object to an instance using the [bincode] crate.

        Args:
            input (bytearray): The serialized object (in [bincode] form).

        Returns:
           The deserialized Spin System.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized.
        """

    def to_bincode(self) -> bytearray:  # type: ignore
        """
        Return the bincode representation of the object using the [bincode] crate.

        Returns:
            bytearray: The serialized object (in [bincode] form).

        Raises:
            ValueError: Cannot serialize object to bytes.
        """

    def to_json(self) -> str:  # type: ignore
        """
        Return the json representation of the object.

        Returns:
            str: The serialized form of the object.

        Raises:
            ValueError: Cannot serialize object to json.
        """

    def from_json(self, input: str):  # type: ignore
        """
        Convert the json representation of the object to an instance.

        Args:
            input (str): The serialized object in json form.

        Returns:
            The deserialized object.

        Raises:
            ValueError: Input cannot be deserialized.
        """

    def from_string(self, input: str) -> MixedPlusMinusProduct:  # type: ignore
        """
        Convert a string representation of the object to an instance.

        Args:
            input (str): The serialized index in str representation.

        Returns:
            self: The converted object.

        Raises:
            ValueError: Input cannot be converted from str.
        """

    def current_version(self) -> str:  # type: ignore
        """
        Returns the current version of the struqture library .

        Returns:
            str: The current version of the library.
        """

    def min_supported_version(self) -> str:  # type: ignore
        """
        Return the minimum version of struqture that supports this object.

        Returns:
            str: The minimum version of the struqture library to deserialize this object.
        """

    def _get_serialisation_meta(self):  # type: ignore
        """
        Returns the StruqtureSerialisationMeta of the object.
        """

    def json_schema(self) -> str:  # type: ignore
        """
        Return the JsonSchema for the json serialisation of the class.

        Returns:
            str: The json schema serialized to json
        """

class MixedPlusMinusOperator:
    """
    These are representations of systems of mixed_systems.

    MixedPlusMinusOperators are characterized by a MixedOperator to represent the hamiltonian of the spin system
    and an optional number of mixed_systems.

    Args:
        number_spins (int): The number of spin subsystems in the MixedPlusMinusOperator.
        number_bosons (int): The number of boson subsystems in the MixedPlusMinusOperator.
        number_fermions (int): The number of fermion subsystems in the MixedPlusMinusOperator.

    Returns:
        self: The new (empty) MixedPlusMinusOperator.

    Examples
    --------

    .. code-block:: python

        import numpy.testing as npt
        import scipy.sparse as sp
        from qoqo_calculator_pyo3 import CalculatorComplex
        from struqture_py.mixed_systems import MixedPlusMinusOperator, MixedPlusMinusProduct
        from struqture_py.spins import PauliProduct
        from struqture_py.bosons import BosonProduct
        from struqture_py.fermions import FermionProduct

        system = MixedPlusMinusOperator(1, 1, 1)
        pp = MixedPlusMinusProduct([PauliProduct().z(0)], [BosonProduct([0], [1])], [FermionProduct([0], [0])])
        system.add_operator_product(pp, 5.0)
        npt.assert_equal(system.current_number_spins(), [2])
        npt.assert_equal(system.get(pp), CalculatorComplex(5))

    """

    def __init__(self, number_spins: int, number_bosons: int, number_fermions: int):
        return

    def from_mixed_operator(self, value: MixedOperator) -> MixedPlusMinusOperator:  # type: ignore
        """
        Convert a MixedOperator into a MixedPlusMinusOperator.

        Args:
            value (MixedOperator): The MixedOperator to create the MixedPlusMinusOperator from.

        Returns:
            MixedPlusMinusOperator: The operator created from the input MixedOperator.

        Raises:
            ValueError: Could not create MixedOperator from input.
        """

    def to_mixed_operator(self) -> MixedOperator:  # type: ignore
        """
        Convert a MixedPlusMinusOperator into a MixedOperator.

        Returns:
            MixedOperator: The operator created from the input MixedPlusMinusOperator and optional number of spins.

        Raises:
            ValueError: Could not create MixedOperator from MixedPlusMinusOperator.
            ValueError: Could not create MixedOperator from MixedOperator.
        """

    def keys(self) -> List[OperatorProduct]:  # type: ignore
        """
        Return a list of the unsorted keys in self.

        Returns:
            List[OperatorProduct]: The sequence of keys of the self.
        """

    def empty_clone(self, capacity: Optional[int]) -> MixedPlusMinusOperator:  # type: ignore
        """
        Return an instance of self that has no entries but clones all other properties, with the given capacity.

        Args:
            capacity (Optional[int]): The capacity of the new instance to create.

        Returns:
            self: An empty clone with the same properties as self, with the given capacity.
        """

    def is_empty(self) -> bool:  # type: ignore
        """
        Return true if self contains no values.

        Returns:
            bool: Whether self is empty or not.
        """

    def truncate(self, threshold: float) -> MixedPlusMinusOperator:  # type: ignore
        """
        Truncate self by returning a copy without entries under a threshold.

        Args:
            threshold (float): The threshold for inclusion.

        Returns:
            self: The truncated version of self.
        """

    def get(self, key) -> Union[float, int, str, complex]:  # type: ignore
        """
        Get the coefficient corresponding to the key.

        Args:
            key: Product to get the value of.

        Returns:
            CalculatorComplex: Value at key (or 0.0).

        Raises:
            ValueError: Product could not be constructed from key.
        """

    def remove(self, key: ProductType) -> Optional[Union[Union[float, int, str, complex], Union[float, int, str]]]:  # type: ignore
        """
        Remove the value of the input key.

        Args:
            key (Product type): The key of the value to remove.

         Returns:
            Optional[Union[CalculatorComplex, CalculatorFloat]]: Key existed if this is not None, and this is the value it had before it was removed.

        Raises:
            ValueError: Product could not be constructed.
        """

    def set(self, key: ProductType, value: Union[Union[float, int, str, complex], Union[float, int, str]]) -> Optional[Union[Union[float, int, str, complex], Union[float, int, str]]]:  # type: ignore
        """
        Overwrite an existing entry or set a new entry in self.

        Args:
            key (Product type): The key to set.
            value (Union[CalculatorComplex, CalculatorFloat]): The value to set.

        Returns:
            Optional[Union[CalculatorComplex, CalculatorFloat]]: Key existed if this is not None, and this is the value it had before it was overwritten.

        Raises:
            ValueError: Product could not be constructed.
        """

    def add_operator_product(self, key: ProductType):  # type: ignore
        """
        Add a new (key object, value Union[CalculatorComplex, CalculatorFloat]) pair to existing entries.

        Args:
            key (Product type): The key object

        Raises:
            TypeError: Value is not CalculatorComplex or CalculatorFloat.
            ValueError: Product could not be constructed.
            ValueError: Error in add_operator_product function of self.
        """

    def values(self) -> List[Union[Union[float, int, str, complex], Union[float, int, str]]]:  # type: ignore
        """
        Return unsorted values in self.

        Returns:
            List[Union[CalculatorComplex, CalculatorFloat]]: The sequence of values of self.
        """

    def hermitian_conjugate(self) -> MixedPlusMinusOperator:  # type: ignore
        """
        Return the hermitian conjugate of self.

        Returns:
            self: The hermitian conjugate of self.
        """

    def current_number_spins(self) -> int:  # type: ignore
        """
        Return the current_number_spins input of each spin subsystem of self.

        Returns:
            int: The number of spins in each spin subsystem of self.
        """

    def current_number_bosonic_modes(self) -> List[int]:  # type: ignore
        """
        Return the number of bosonic modes in each bosonic subsystem of self.

        Returns:
            list[int]: The number of bosonic modes in each bosonic subsystem of self.
        """

    def current_number_fermionic_modes(self) -> List[int]:  # type: ignore
        """
        Return the number of fermionic modes in each fermionic subsystem of self.

        Returns:
            list[int]: The number of fermionic modes in each fermionic subsystem of self.
        """

    def from_json_struqture_1(self, input: Any) -> Any:  # type: ignore
        """
        Convert a json corresponding to a struqture 1 object to the equivalent object in struqture 2.

        Args:
            input (Any): the json of the struqture 1 object to convert.

        Returns:
            Any: the input object in struqture 2 form.

        Raises:
            ValueError: Input could not be deserialised form json.
            ValueError: Struqture 1 object could not be converted to struqture 2.
        """

    def from_bincode(self, input: bytearray):  # type: ignore
        """
        Convert the bincode representation of self to an instance using the [bincode] crate.

        Args:
            input (bytearray): The serialized object (in [bincode] form).

        Returns:
           The deserialized object.

        Raises:
            TypeError: Input cannot be converted to byte array.
            ValueError: Input cannot be deserialized.
        """

    def to_bincode(self) -> bytearray:  # type: ignore
        """
        Return the bincode representation of self using the [bincode] crate.

        Returns:
            bytearray: The serialized object (in [bincode] form).

        Raises:
            ValueError: Cannot serialize object to bytes.
        """

    def to_json(self) -> str:  # type: ignore
        """
        Return the json representation of self.

        Returns:
            str: The serialized form of self.

        Raises:
            ValueError: Cannot serialize object to json.
        """

    def from_json(self, input: str):  # type: ignore
        """
        Convert the json representation of self to an instance.

        Args:
            input (str): The serialized object in json form.

        Returns:
            The deserialized object.

        Raises:
            ValueError: Input cannot be deserialized.
        """

    def current_version(self) -> str:  # type: ignore
        """
        Returns the current version of the struqture library.

        Returns:
            str: The current version of the library.
        """

    def min_supported_version(self) -> str:  # type: ignore
        """
        Return the minimum version of struqture that supports this object.

        Returns:
            str: The minimum version of the struqture library to deserialize this object.
        """

    def _get_serialisation_meta(self):  # type: ignore
        """
        Returns the StruqtureSerialisationMeta of the object.
        """

    def json_schema(self) -> str:  # type: ignore
        """
        Return the JsonSchema for the json serialisation of the class.

        Returns:
            str: The json schema serialized to json
        """
