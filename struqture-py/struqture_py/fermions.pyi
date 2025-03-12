# This is an auto generated file containing only the documentation.
# You can find the full implementation on this page:
# https://github.com/HQSquantumsimulations/struqture

"""
Fermions module of struqture Python interface

Module for representing fermionic indices (FermionProduct and HermitianFermionProduct), fermionic systems (FermionOperator and FermionHamiltonian),
and Lindblad type fermionic open systems (FermionLindbladNoiseOperator, FermionLindbladOpenSystem).

.. autosummary::
    :toctree: generated/

    FermionProduct
    HermitianFermionProduct
    FermionOperator
    FermionHamiltonian
    FermionLindbladNoiseOperator
    FermionLindbladOpenSystem

"""

from .struqture_py import ProductType, SystemType, NoiseType
from typing import Optional, List, Tuple, Set, Union, Any

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
        return

    def jordan_wigner(self):  # type: ignore
        """
        Transform the given fermionic object into a spin object using
        the Jordan Wigner mapping.
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

    def number_creators(self) -> int:  # type: ignore
        """
        Get the number of creator indices of self.

        Returns:
            int: The number of creator indices in self.
        """

    def number_annihilators(self) -> int:  # type: ignore
        """
        Get the number of annihilator indices of self.

        Returns:
            int: The number of annihilator indices in self.
        """

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
            int: The maximal number of modes self acts on.
        """

    def creators(self) -> List[int]:  # type: ignore
        """
        Return list of creator indices.

        Returns:
            List[int]: A list of the corresponding creator indices.
        """

    def annihilators(self) -> List[int]:  # type: ignore
        """
        Return list of annihilator indices.

        Returns:
            List[int]: A list of the corresponding annihilator indices.
        """

    def remap_modes(self):  # type: ignore
        """
        Remap modes according to an input dictionary.

        Args:
           reordering_dictionary (dict) - The dictionary specifying the remapping. It must represent a permutation.

        Returns:
          (Self, CalculatorComplex) - The instance of Self with modes remapped, and the sign resulting from symmetry/antisymmetry.

        Raises:
           ValueError: Input reordering dictionary is not a permutation of the indices.
        """

    def create_valid_pair(self, creators: List[int], annihilators: List[int], value: Union[float, int, str, complex]):  # type: ignore
        """
        Create valid pair of index and value to be set in an operator.

        The first item is the valid instance of self created from the input creators and annihilators.
        The second term is the input CalculatorComplex transformed according to the valid order of creators and annihilators.

        Args:
           creators (List[int]): The creator indices to have in the instance of self.
           annihilators (List[int]): The annihilators indices to have in the instance of self.
           value (CalculatorComplex): The CalculatorComplex to transform.

        Returns:
           (self, CalculatorComplex): The valid instance of self and the corresponding transformed CalculatorComplex.

        Raises:
            TypeError: Value is not CalculatorComplex.
            ValueError: Indices given in either creators or annihilators contain a double index specification (only applicable to fermionic objects).
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

    def from_string(self, input: str) -> FermionProduct:  # type: ignore
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

class HermitianFermionProduct(ProductType):
    """
    A product of fermionic creation and annihilation operators.

    The HermitianFermionProduct is used as an index for non-hermitian, normal ordered fermionic operators.
    A fermionic operator can be written as a sum over normal ordered products of creation and annihilation operators.
    The HermitianFermionProduct is used as an index when setting or adding new summands to a fermionic operator and when querrying the
    weight of a product of operators in the sum.

    Args:
        creators (List[int]): List of creator sub-indices.
        annihilators (List[int]): List of annihilator sub-indices.

    Returns:
        self: The new (empty) HermitianFermionProduct.

    Examples
    --------

    .. code-block:: python

        from struqture_py.fermions import HermitianFermionProduct
        import numpy.testing as npt
        # For instance, to represent $c_0a_1$
        fp = HermitianFermionProduct([0], [0])
        npt.assert_equal(fp.creators(), [0])
        npt.assert_equal(fp.annihilators(), [0])

    """

    def __init__(self, creators: List[int], annihilators: List[int]):
        return

    def jordan_wigner(self):  # type: ignore
        """
        Transform the given fermionic object into a spin object using
        the Jordan Wigner mapping.
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

    def number_creators(self) -> int:  # type: ignore
        """
        Get the number of creator indices of self.

        Returns:
            int: The number of creator indices in self.
        """

    def number_annihilators(self) -> int:  # type: ignore
        """
        Get the number of annihilator indices of self.

        Returns:
            int: The number of annihilator indices in self.
        """

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
            int: The maximal number of modes self acts on.
        """

    def creators(self) -> List[int]:  # type: ignore
        """
        Return list of creator indices.

        Returns:
            List[int]: A list of the corresponding creator indices.
        """

    def annihilators(self) -> List[int]:  # type: ignore
        """
        Return list of annihilator indices.

        Returns:
            List[int]: A list of the corresponding annihilator indices.
        """

    def remap_modes(self):  # type: ignore
        """
        Remap modes according to an input dictionary.

        Args:
           reordering_dictionary (dict) - The dictionary specifying the remapping. It must represent a permutation.

        Returns:
          (Self, CalculatorComplex) - The instance of Self with modes remapped, and the sign resulting from symmetry/antisymmetry.

        Raises:
           ValueError: Input reordering dictionary is not a permutation of the indices.
        """

    def create_valid_pair(self, creators: List[int], annihilators: List[int], value: Union[float, int, str, complex]):  # type: ignore
        """
        Create valid pair of index and value to be set in an operator.

        The first item is the valid instance of self created from the input creators and annihilators.
        The second term is the input CalculatorComplex transformed according to the valid order of creators and annihilators.

        Args:
           creators (List[int]): The creator indices to have in the instance of self.
           annihilators (List[int]): The annihilators indices to have in the instance of self.
           value (CalculatorComplex): The CalculatorComplex to transform.

        Returns:
           (self, CalculatorComplex): The valid instance of self and the corresponding transformed CalculatorComplex.

        Raises:
            TypeError: Value is not CalculatorComplex.
            ValueError: Indices given in either creators or annihilators contain a double index specification (only applicable to fermionic objects).
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

    def from_string(self, input: str) -> HermitianFermionProduct:  # type: ignore
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

class FermionOperator:
    """
    These are representations of systems of fermions.

    FermionOperators are characterized by a FermionOperator to represent the hamiltonian of the spin system
    and an optional number of fermions.

    Returns:
        self: The new FermionSystem with the input number of fermions.

    Examples
    --------

    .. code-block:: python

        import numpy.testing as npt
        import scipy.sparse as sp
        from qoqo_calculator_pyo3 import CalculatorComplex
        from struqture_py.fermions import FermionOperator, FermionProduct

        system = FermionOperator()
        pp = FermionProduct([0], [0])
        system.add_operator_product(pp, 5.0)
        npt.assert_equal(system.current_number_modes(), 2)
        npt.assert_equal(system.get(pp), CalculatorComplex(5))
        npt.assert_equal(system.keys(), [pp])

    """

    def __init__(self):
        return

    def jordan_wigner(self):  # type: ignore
        """
        Transform the given fermionic object into a spin object using
        the Jordan Wigner mapping.
        """

    def keys(self) -> List[OperatorProduct]:  # type: ignore
        """
        Return a list of the unsorted keys in self.

        Returns:
            List[OperatorProduct]: The sequence of keys of the self.
        """

    def empty_clone(self, capacity: Optional[int]) -> FermionOperator:  # type: ignore
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

    def truncate(self, threshold: float) -> FermionOperator:  # type: ignore
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

    def hermitian_conjugate(self) -> FermionOperator:  # type: ignore
        """
        Return the hermitian conjugate of self.

        Returns:
            self: The hermitian conjugate of self.
        """

    def current_number_modes(self) -> int:  # type: ignore
        """
        Return the current_number_modes input of self.

        Returns:
            int: The number of modes in self.
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

class FermionHamiltonian:
    """
    These are representations of systems of fermions.

    FermionHamiltonians are characterized by a FermionOperator to represent the hamiltonian of the spin system
    and an optional number of fermions.

    Returns:
        self: The new FermionHamiltonianSystem with the input number of fermions.

    Examples
    --------

    .. code-block:: python

        import numpy.testing as npt
        import scipy.sparse as sp
        from qoqo_calculator_pyo3 import CalculatorComplex
        from struqture_py.fermions import FermionHamiltonian, HermitianFermionProduct

        system = FermionHamiltonian()
        pp = HermitianFermionProduct([0], [0])
        system.add_operator_product(pp, 5.0)
        npt.assert_equal(system.current_number_modes(), 2)
        npt.assert_equal(system.get(pp), CalculatorComplex(5))
        npt.assert_equal(system.keys(), [pp])

    """

    def __init__(self):
        return

    def jordan_wigner(self):  # type: ignore
        """
        Transform the given fermionic object into a spin object using
        the Jordan Wigner mapping.
        """

    def separate_into_n_terms(self, number_creators_annihilators: Tuple[int, int]) -> Tuple[FermionHamiltonianWrapper, FermionHamiltonianWrapper]:  # type: ignore
        """
        Separate self into an operator with the terms of given number of creation and annihilation operators and an operator with the remaining operations.

        Args:
            number_creators_annihilators (Tuple[int, int]): Number of modes to filter for in the keys.

        Returns:
            Tuple[FermionHamiltonianWrapper, FermionHamiltonianWrapper]: Operator with the noise terms where the number of creation and annihilation operators matches the number of spins the operator product acts on and Operator with all other contributions.

        Raises:
            ValueError: Error in adding terms to return values.
        """

    def keys(self) -> List[OperatorProduct]:  # type: ignore
        """
        Return a list of the unsorted keys in self.

        Returns:
            List[OperatorProduct]: The sequence of keys of the self.
        """

    def empty_clone(self, capacity: Optional[int]) -> FermionHamiltonian:  # type: ignore
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

    def truncate(self, threshold: float) -> FermionHamiltonian:  # type: ignore
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

    def hermitian_conjugate(self) -> FermionHamiltonian:  # type: ignore
        """
        Return the hermitian conjugate of self.

        Returns:
            self: The hermitian conjugate of self.
        """

    def current_number_modes(self) -> int:  # type: ignore
        """
        Return the current_number_modes input of self.

        Returns:
            int: The number of modes in self.
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

class FermionLindbladNoiseOperator(NoiseType):
    """
    These are representations of noisy systems of fermions.

    In a FermionLindbladNoiseOperator is characterized by a FermionLindbladNoiseOperator to represent the hamiltonian of the system, and an optional number of fermions.

    Examples
    --------

    .. code-block:: python

        import numpy.testing as npt
        import scipy.sparse as sp
        from qoqo_calculator_pyo3 import CalculatorComplex
        from struqture_py.fermions import FermionLindbladNoiseOperator, FermionProduct

        slns = FermionLindbladNoiseOperator()
        dp = FermionProduct([0], [1])
        slns.add_operator_product((dp, dp), 2.0)
        npt.assert_equal(slns.current_number_modes(), 2)
        npt.assert_equal(slns.get((dp, dp)), CalculatorComplex(2))

    """

    def __init__(self):
        return

    def jordan_wigner(self):  # type: ignore
        """
        Transform the given fermionic object into a spin object using
        the Jordan Wigner mapping.
        """

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

    def empty_clone(self, capacity) -> FermionLindbladNoiseOperator:  # type: ignore
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

    def truncate(self, threshold) -> FermionLindbladNoiseOperator:  # type: ignore
        """
        Truncate self by returning a copy without entries under a threshold.

        Args:
            threshold: The threshold for inclusion.

        Returns:
            self: The truncated version of self.
        """

    def current_number_modes(self) -> int:  # type: ignore
        """
        Return the current_number_modes input of self.

        Returns:
            int: The number of modes in self.
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

class FermionLindbladOpenSystem(SystemType):
    """
    These are representations of noisy systems of fermions.

    In a FermionLindbladOpenSystem is characterized by a FermionLindbladOpenOperator to represent the hamiltonian of the system, and an optional number of fermions.

    Args:
        number_fermions (Optional[int]): The number of fermions in the FermionLindbladOpenSystem.

    Returns:
        self: The new FermionLindbladOpenSystem with the input number of fermions.

    Examples
    --------

    .. code-block:: python

        import numpy.testing as npt
        import scipy.sparse as sp
        from qoqo_calculator_pyo3 import CalculatorComplex
        from struqture_py.fermions import FermionLindbladOpenSystem, FermionProduct

        slns = FermionLindbladOpenSystem()
        dp = FermionProduct([0], [1])
        slns.system_add_operator_product(dp, 2.0)
        npt.assert_equal(slns.current_number_modes(), 2)
        npt.assert_equal(slns.system().get(dp), CalculatorComplex(2))

    """

    def __init__(self, number_fermions: Optional[int]):
        return

    def jordan_wigner(self):  # type: ignore
        """
        Transform the given fermionic object into a spin object using
        the Jordan Wigner mapping.
        """

    def current_number_modes(self) -> int:  # type: ignore
        """
        Return the current_number_modes input of self.

        Returns:
            int: The number of modes in self.
        """

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

    def group(self, system, noise) -> FermionLindbladOpenSystem:  # type: ignore
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

    def empty_clone(self) -> FermionLindbladOpenSystem:  # type: ignore
        """
        Return an instance of self that has no entries but clones all other properties, with the given capacity.

        Returns:
            self: An empty clone with the same properties as self, with the given capacity.
        """

    def truncate(self, threshold) -> FermionLindbladOpenSystem:  # type: ignore
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
