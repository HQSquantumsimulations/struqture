# This is an auto generated file containing only the documentation.
# You can find the full implementation on this page:
# https://github.com/HQSquantumsimulations/struqture

"""
Spin module of struqture Python interface

Module for representing spin indices (PauliProduct and DecoherenceProduct), spin systems (SpinSystem and SpinHamiltonianSystem)
and Lindblad type spin open systems (SpinLindbladNoiseSystem and SpinLindbladOpenSystem).

.. autosummary::
    :toctree: generated/

    PauliProduct
    DecoherenceProduct
    SpinSystem
    SpinHamiltonianSystem
    SpinLindbladNoiseSystem
    SpinLindbladOpenSystem

"""

from .struqture_py import ProductType, SystemType, NoiseType
import numpy
from typing import Optional, List, Tuple, Dict, Union

class PauliProduct(ProductType):
    """
    PauliProducts are combinations of SingleSpinOperators on specific qubits.

    PauliProducts can be used in either noise-free or a noisy system.
    They are representations of products of pauli matrices acting on qubits,
    in order to build the terms of a hamiltonian.
    For instance, to represent the term :math:`\sigma_0^{x}` :math:`\sigma_2^{x}` :

    `PauliProduct().x(0).x(2)`.

    PauliProduct is  supposed to be used as input for the function `set_pauli_product`,
    for instance in the spin system classes SpinLindbladOpenSystem, SpinHamiltonianSystem or SpinSystem,
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
        return

    def jordan_wigner(self):  # type: ignore
        """
        Transform the given spin object into a fermionic object using
        the Jordan Wigner mapping.
        """

    def x(self, index: int) -> PauliProduct:  # type: ignore
        """
        Set a new entry for SingleSpinOperator X in the internal dictionary.

        Args:
            index (int): Index of set object.

        Returns:
            PauliProduct: The PauliProduct with the new entry.
        """

    def y(self, index: int) -> PauliProduct:  # type: ignore
        """
        Set a new entry for SingleSpinOperator Y in the internal dictionary.

        Args:
            index (int): Index of set object.

        Returns:
            PauliProduct: The PauliProduct with the new entry.
        """

    def z(self, index: int) -> PauliProduct:  # type: ignore
        """
        Set a new entry for SingleSpinOperator Z in the internal dictionary.

        Args:
            index (int): Index of set object.

        Returns:
            PauliProduct: The PauliProduct with the new entry.
        """

    def set_pauli(self, index: int, pauli: str) -> PauliProduct:  # type: ignore
        """
        Set a new entry in the internal_map. This function consumes self.

        Args:
            index (int): Index of set object.
            pauli (str): Value of set object.

        Returns:
            self: The entry was correctly set and the PauliProduct is returned.
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

    def get(self, index: int) -> Optional[str]:  # type: ignore
        """
        Get the pauli matrix corresponding to the index.

        Args:
            index (int): Index of get object.

        Returns:
            Optional[str]: The key's corresponding value (if it exists).
        """

    def keys(self) -> List[int]:  # type: ignore
        """
        Return a list of the unsorted keys in self.

        Returns:
            List[int]: The sequence of qubit index keys of self.
        """

    def current_number_spins(self) -> int:  # type: ignore
        """
        Return maximum index in self.

        Returns:
            int: Maximum index.
        """

    def is_empty(self) -> bool:  # type: ignore
        """
        Return whether self is empty or not.

        Returns:
            bool: Whether self is empty or not.
        """

    def remap_qubits(self, mapping: Dict[int, int]) -> PauliProduct:  # type: ignore
        """
        Remap the qubits in a new instance of self (returned).

        Args:
            mapping (Dict[int, int]): The map containing the {qubit: qubit} mapping to use.

        Returns:
            self: The new instance of self with the qubits remapped.
        """

    def concatenate(self, other: PauliProduct) -> List[int]:  # type: ignore
        """
        Return the concatenation of two objects of type `self` with no overlapping qubits.

        Args:
            other (self): The object to concatenate self with.

        Returns:
            List[int]: A list of the corresponding creator indices.

        Raises:
            ValueError: The two objects could not be concatenated.
        """

    def multiply(self, left: PauliProduct, right: PauliProduct):  # type: ignore
        """
        Multiplication function for a self-typed object by a self-typed object.

        Args:
            left (self): Left-hand self typed object to be multiplied.
            right (self): Right-hand self typed object to be multiplied.

        Returns:
            (self, complex):  The multiplied objects and the resulting prefactor.
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

    def from_string(self, input: str) -> PauliProduct:  # type: ignore
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

    def json_schema(self) -> str:  # type: ignore
        """
        Return the JsonSchema for the json serialisation of the class.

        Returns:
            str: The json schema serialized to json
        """

class DecoherenceProduct(ProductType):
    """
    These are combinations of SingleDecoherenceOperators on specific qubits.

    DecoherenceProducts act in a noisy system. They are representation of products of decoherence
    matrices acting on qubits in order to build the terms of a hamiltonian.
    For instance, to represent the term :math:`\sigma_0^{x}` :math:`\sigma_2^{z}`:

    `DecoherenceProduct().x(0).z(2)`.

    DecoherenceProduct is  supposed to be used as input for the function `add_noise`,
    for instance in the spin system classes SpinLindbladOpenSystem, SpinLindbladNoiseSystem or SpinLindbladNoiseOperator,
    or in the mixed systems as part of `MixedDecoherenceProduct <mixed_systems.MixedDecoherenceProduct>`.

    Returns:
        self: The new, empty DecoherenceProduct.

    Examples
    --------

    .. code-block:: python

        import numpy.testing as npt
        from struqture_py.spins import DecoherenceProduct
        dp = DecoherenceProduct().x(0).iy(1).z(2)
        dp = dp.set_pauli(3, "X")
        npt.assert_equal(dp.get(1), "iY")
        npt.assert_equal(dp.keys(), [0, 1, 2, 3])

    """

    def __init__(self):
        return

    def jordan_wigner(self):  # type: ignore
        """
        Transform the given spin object into a fermionic object using
        the Jordan Wigner mapping.
        """

    def x(self, index: int) -> DecoherenceProduct:  # type: ignore
        """
        Set a new entry for SingleDecoherenceOperator X in the internal dictionary.

        Args:
            index (int): Index of set object.

        Returns:
            DecoherenceProduct: The DecoherenceProduct with the new entry.
        """

    def iy(self, index: int) -> DecoherenceProduct:  # type: ignore
        """
        Set a new entry for SingleDecoherenceOperator iY in the internal dictionary.

        Args:
            index (int): Index of set object.

        Returns:
            DecoherenceProduct: The DecoherenceProduct with the new entry.
        """

    def z(self, index: int) -> DecoherenceProduct:  # type: ignore
        """
        Set a new entry for SingleDecoherenceOperator Z in the internal dictionary.

        Args:
            index (int): Index of set object.

        Returns:
            DecoherenceProduct: The DecoherenceProduct with the new entry.
        """

    def set_pauli(self, index: int, pauli: str) -> DecoherenceProduct:  # type: ignore
        """
        Set a new entry in the internal_map. This function consumes self.

        Args:
            index (int): Index of set object.
            pauli (str): Value of set object.

        Returns:
            self: The entry was correctly set and the DecoherenceProduct is returned.
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

    def get(self, index: int) -> Optional[str]:  # type: ignore
        """
        Get the pauli matrix corresponding to the index.

        Args:
            index (int): Index of get object.

        Returns:
            Optional[str]: The key's corresponding value (if it exists).
        """

    def keys(self) -> List[int]:  # type: ignore
        """
        Return a list of the unsorted keys in self.

        Returns:
            List[int]: The sequence of qubit index keys of self.
        """

    def current_number_spins(self) -> int:  # type: ignore
        """
        Return maximum index in self.

        Returns:
            int: Maximum index.
        """

    def is_empty(self) -> bool:  # type: ignore
        """
        Return whether self is empty or not.

        Returns:
            bool: Whether self is empty or not.
        """

    def remap_qubits(self, mapping: Dict[int, int]) -> DecoherenceProduct:  # type: ignore
        """
        Remap the qubits in a new instance of self (returned).

        Args:
            mapping (Dict[int, int]): The map containing the {qubit: qubit} mapping to use.

        Returns:
            self: The new instance of self with the qubits remapped.
        """

    def concatenate(self, other: DecoherenceProduct) -> List[int]:  # type: ignore
        """
        Return the concatenation of two objects of type `self` with no overlapping qubits.

        Args:
            other (self): The object to concatenate self with.

        Returns:
            List[int]: A list of the corresponding creator indices.

        Raises:
            ValueError: The two objects could not be concatenated.
        """

    def multiply(self, left: DecoherenceProduct, right: DecoherenceProduct):  # type: ignore
        """
        Multiplication function for a self-typed object by a self-typed object.

        Args:
            left (self): Left-hand self typed object to be multiplied.
            right (self): Right-hand self typed object to be multiplied.

        Returns:
            (self, complex):  The multiplied objects and the resulting prefactor.
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

    def from_string(self, input: str) -> DecoherenceProduct:  # type: ignore
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

    def json_schema(self) -> str:  # type: ignore
        """
        Return the JsonSchema for the json serialisation of the class.

        Returns:
            str: The json schema serialized to json
        """

class SpinSystem(SystemType):
    """
    These are representations of systems of spins.

    SpinSystems are characterized by a SpinOperator to represent the hamiltonian of the spin system
    and an optional number of spins.

     Args:
        number_spins (Optional[int]): The number of spins in the SpinSystem.

    Returns:
        self: The new SpinSystem with the input number of spins.

    Examples
    --------

    .. code-block:: python

        import numpy.testing as npt
        import scipy.sparse as sp
        from qoqo_calculator_pyo3 import CalculatorComplex
        from struqture_py.spins import SpinSystem, PauliProduct

        ssystem = SpinSystem(2)
        pp = PauliProduct().z(0)
        ssystem.add_operator_product(pp, 5.0)
        npt.assert_equal(ssystem.number_spins(), 2)
        npt.assert_equal(ssystem.get(pp), CalculatorComplex(5))
        npt.assert_equal(ssystem.keys(), [pp])
        dimension = 4**ssystem.number_spins()
        matrix = sp.coo_matrix(ssystem.sparse_matrix_superoperator_coo(), shape=(dimension, dimension))

    """

    def __init__(self, number_spins: Optional[int]):
        return

    def jordan_wigner(self):  # type: ignore
        """
        Transform the given spin object into a fermionic object using
        the Jordan Wigner mapping.
        """

    def keys(self) -> List[OperatorProduct]:  # type: ignore
        """
        Return a list of the unsorted keys in self.

        Returns:
            List[OperatorProduct]: The sequence of keys of the self.
        """

    def empty_clone(self, capacity: Optional[int]) -> SpinSystem:  # type: ignore
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

    def truncate(self, threshold: float) -> SpinSystem:  # type: ignore
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

    def hermitian_conjugate(self) -> SpinSystem:  # type: ignore
        """
        Return the hermitian conjugate of self.

        Returns:
            self: The hermitian conjugate of self.
        """

    def current_number_spins(self) -> int:  # type: ignore
        """
        Return maximum spin index in self.

        Returns:
            int: Maximum index.
        """

    def number_spins(self) -> int:  # type: ignore
        """
        Return the number_spins input of self.

        Returns:
            int: The number of spins in self.
        """

    def separate_into_n_terms(self, number_spins: int) -> Tuple[Self, Self]:  # type: ignore
        """
        Separate self into an operator with the terms of given number of spins and an operator with the remaining operations.

        Args:
            number_spins (int): Number of spins to filter for in the keys.

        Returns:
            Tuple[Self, Self]: Operator with the noise terms where the number of spins matches the number of spins the operator product acts on and Operator with all other contributions.

        Raises:
            ValueError: Error in adding terms to return values.
        """

    def sparse_matrix_coo(self, number_spins: Optional[int]) -> Tuple[numpy.ndarray, Tuple[numpy.ndarray, numpy.ndarray]]:  # type: ignore
        """
        Constructs the sparse matrix representation of self as a scipy COO matrix with a given number of spins.

        Args:
            number_spins (Optional[int]): The number of spins in self.

        Returns:
            Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]]: The matrix representation of self.

        Raises:
            ValueError: CalculatorError.
            RuntimeError: Could not convert to complex superoperator matrix.
        """

    def sparse_matrix_superoperator_coo(self, number_spins) -> Tuple[numpy.ndarray, Tuple[numpy.ndarray, numpy.ndarray]]:  # type: ignore
        """
        Construct the sparse matrix representation of the superoperator in COO representation.

        The superoperator for the operator O is defined as the Matrix S so that
        `flatten(-i [O, p]) = S flatten(p)` wher `[,]` is the commutator, `p` is a matrix
        and `flatten` flattens a matrix into a vector in row-major form.

        Args:
            number_spins: The number of spins to construct the matrix for.

        Returns:
            Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]]: The matrix representation of self.

        Raises:
            ValueError: CalculatorError.
            RuntimeError: Could not convert to complex superoperator matrix.
        """

    def unitary_sparse_matrix_coo(self) -> Tuple[numpy.ndarray, Tuple[numpy.ndarray, numpy.ndarray]]:  # type: ignore
        """
        Return the unitary part of the superoperator in the sparse COO format.

        Returns:
            Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]]: The matrix representation of the unitary part of self.

        Raises:
            ValueError: CalculatorError.
            RuntimeError: Could not convert to complex superoperator matrix.
        """

    def sparse_lindblad_entries(self) -> List[Tuple[Tuple[numpy.ndarray, Tuple[numpy.ndarray, numpy.ndarray]], Tuple[numpy.ndarray, Tuple[numpy.ndarray, numpy.ndarray]], complex]]:  # type: ignore
        """
        Output the Lindblad entries in the form (left, right, rate) where left/right are the left and right lindblad operators, and rate is the lindblad rate respectively.

        Returns:
            List[Tuple[Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]], Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]], complex]]: The matrix representation of the noise part of self.

        Raises:
            ValueError: CalculatorError.
            RuntimeError: Could not convert to complex superoperator matrix.
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

    def json_schema(self) -> str:  # type: ignore
        """
        Return the JsonSchema for the json serialisation of the class.

        Returns:
            str: The json schema serialized to json
        """

class SpinHamiltonianSystem(SystemType):
    """
    These are representations of systems of spins.

    SpinHamiltonianSystems are characterized by a SpinOperator to represent the hamiltonian of the spin system
    and an optional number of spins.

    Args:
        number_spins (Optional[int]): The number of spins in the SpinHamiltonianSystem.

    Returns:
        self: The new SpinHamiltonianSystem with the input number of spins.

    Examples
    --------

    .. code-block:: python

        import numpy.testing as npt
        import scipy.sparse as sp
        from qoqo_calculator_pyo3 import CalculatorComplex
        from struqture_py.spins import SpinHamiltonianSystem, PauliProduct

        ssystem = SpinHamiltonianSystem(2)
        pp = PauliProduct().z(0)
        ssystem.add_operator_product(pp, 5.0)
        npt.assert_equal(ssystem.number_spins(), 2)
        npt.assert_equal(ssystem.get(pp), CalculatorComplex(5))
        npt.assert_equal(ssystem.keys(), [pp])
        dimension = 4**ssystem.number_spins()
        matrix = sp.coo_matrix(ssystem.sparse_matrix_superoperator_coo(), shape=(dimension, dimension))

    """

    def __init__(self, number_spins: Optional[int]):
        return

    def jordan_wigner(self):  # type: ignore
        """
        Transform the given spin object into a fermionic object using
        the Jordan Wigner mapping.
        """

    def keys(self) -> List[OperatorProduct]:  # type: ignore
        """
        Return a list of the unsorted keys in self.

        Returns:
            List[OperatorProduct]: The sequence of keys of the self.
        """

    def empty_clone(self, capacity: Optional[int]) -> SpinHamiltonianSystem:  # type: ignore
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

    def truncate(self, threshold: float) -> SpinHamiltonianSystem:  # type: ignore
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

    def hermitian_conjugate(self) -> SpinHamiltonianSystem:  # type: ignore
        """
        Return the hermitian conjugate of self.

        Returns:
            self: The hermitian conjugate of self.
        """

    def current_number_spins(self) -> int:  # type: ignore
        """
        Return maximum spin index in self.

        Returns:
            int: Maximum index.
        """

    def number_spins(self) -> int:  # type: ignore
        """
        Return the number_spins input of self.

        Returns:
            int: The number of spins in self.
        """

    def separate_into_n_terms(self, number_spins: int) -> Tuple[Self, Self]:  # type: ignore
        """
        Separate self into an operator with the terms of given number of spins and an operator with the remaining operations.

        Args:
            number_spins (int): Number of spins to filter for in the keys.

        Returns:
            Tuple[Self, Self]: Operator with the noise terms where the number of spins matches the number of spins the operator product acts on and Operator with all other contributions.

        Raises:
            ValueError: Error in adding terms to return values.
        """

    def sparse_matrix_coo(self, number_spins: Optional[int]) -> Tuple[numpy.ndarray, Tuple[numpy.ndarray, numpy.ndarray]]:  # type: ignore
        """
        Constructs the sparse matrix representation of self as a scipy COO matrix with a given number of spins.

        Args:
            number_spins (Optional[int]): The number of spins in self.

        Returns:
            Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]]: The matrix representation of self.

        Raises:
            ValueError: CalculatorError.
            RuntimeError: Could not convert to complex superoperator matrix.
        """

    def sparse_matrix_superoperator_coo(self, number_spins) -> Tuple[numpy.ndarray, Tuple[numpy.ndarray, numpy.ndarray]]:  # type: ignore
        """
        Construct the sparse matrix representation of the superoperator in COO representation.

        The superoperator for the operator O is defined as the Matrix S so that
        `flatten(-i [O, p]) = S flatten(p)` wher `[,]` is the commutator, `p` is a matrix
        and `flatten` flattens a matrix into a vector in row-major form.

        Args:
            number_spins: The number of spins to construct the matrix for.

        Returns:
            Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]]: The matrix representation of self.

        Raises:
            ValueError: CalculatorError.
            RuntimeError: Could not convert to complex superoperator matrix.
        """

    def unitary_sparse_matrix_coo(self) -> Tuple[numpy.ndarray, Tuple[numpy.ndarray, numpy.ndarray]]:  # type: ignore
        """
        Return the unitary part of the superoperator in the sparse COO format.

        Returns:
            Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]]: The matrix representation of the unitary part of self.

        Raises:
            ValueError: CalculatorError.
            RuntimeError: Could not convert to complex superoperator matrix.
        """

    def sparse_lindblad_entries(self) -> List[Tuple[Tuple[numpy.ndarray, Tuple[numpy.ndarray, numpy.ndarray]], Tuple[numpy.ndarray, Tuple[numpy.ndarray, numpy.ndarray]], complex]]:  # type: ignore
        """
        Output the Lindblad entries in the form (left, right, rate) where left/right are the left and right lindblad operators, and rate is the lindblad rate respectively.

        Returns:
            List[Tuple[Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]], Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]], complex]]: The matrix representation of the noise part of self.

        Raises:
            ValueError: CalculatorError.
            RuntimeError: Could not convert to complex superoperator matrix.
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

    def json_schema(self) -> str:  # type: ignore
        """
        Return the JsonSchema for the json serialisation of the class.

        Returns:
            str: The json schema serialized to json
        """

class SpinLindbladNoiseSystem(SystemType):
    """
    These are representations of noisy systems of spins.

    In a SpinLindbladNoiseSystem is characterized by a SpinLindbladNoiseOperator to represent the hamiltonian of the spin system, and an optional number of spins.

    Args:
        number_spins (Optional[int]): The number of spins in the SpinLindbladNoiseSystem.

    Returns:
        self: The new SpinLindbladNoiseSystem with the input number of spins.

    Examples
    --------

    .. code-block:: python

        import numpy.testing as npt
        import scipy.sparse as sp
        from qoqo_calculator_pyo3 import CalculatorComplex
        from struqture_py.spins import SpinLindbladNoiseSystem, DecoherenceProduct

        slns = SpinLindbladNoiseSystem()
        dp = DecoherenceProduct().z(0).x(1)
        slns.add_operator_product((dp, dp), 2.0)
        npt.assert_equal(slns.current_number_spins(), 2)
        npt.assert_equal(slns.get((dp, dp)), CalculatorComplex(2))
        npt.assert_equal(slns.keys(), [(dp, dp)])
        dimension = 4**slns.number_spins()
        matrix = sp.coo_matrix(slns.sparse_matrix_superoperator_coo(), shape=(dimension, dimension))

    """

    def __init__(self, number_spins: Optional[int]):
        return

    def jordan_wigner(self):  # type: ignore
        """
        Transform the given spin object into a fermionic object using
        the Jordan Wigner mapping.
        """

    def separate_into_n_terms(self, number_spins_left: int, number_spins_right: int) -> Tuple[SpinLindbladNoiseSystem, SpinLindbladNoiseSystem]:  # type: ignore
        """
        Separate self into an operator with the terms of given number of spins and an operator with the remaining operations.

        Args:
            number_spins_left (int): Number of spins to filter for in the left term of the keys.
            number_spins_right (int): Number of spins to filter for in the right term of the keys.

        Returns:
            Tuple[SpinLindbladNoiseSystem, SpinLindbladNoiseSystem]: Operator with the noise terms where the number of spins matches the number of spins the operator product acts on and Operator with all other contributions.

        Raises:
            ValueError: Error in adding terms to return values.
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

    def empty_clone(self, capacity) -> SpinLindbladNoiseSystem:  # type: ignore
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

    def truncate(self, threshold) -> SpinLindbladNoiseSystem:  # type: ignore
        """
        Truncate self by returning a copy without entries under a threshold.

        Args:
            threshold: The threshold for inclusion.

        Returns:
            self: The truncated version of self.
        """

    def current_number_spins(self) -> int:  # type: ignore
        """
        Return maximum spin index in object.

        Returns:
            int: Maximum index.
        """

    def number_spins(self) -> int:  # type: ignore
        """
        Return the number_spins input of self.

        Returns:
            int: The number of spins in self.
        """

    def sparse_matrix_superoperator_coo(self, number_spins) -> Tuple[numpy.ndarray, Tuple[numpy.ndarray, numpy.ndarray]]:  # type: ignore
        """
        Construct the sparse matrix representation of the superoperator in COO representation.

        The superoperator for the operator O is defined as the Matrix S so that
        `flatten(-i [O, p]) = S flatten(p)` wher `[,]` is the commutator, `p` is a matrix
        and `flatten` flattens a matrix into a vector in row-major form.

        Args:
            number_spins: The number of spins in self.

        Returns:
            Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]]: The matrix representation of self.

        Raises:
            ValueError: CalculatorError.
            RuntimeError: Could not convert to complex superoperator matrix.
        """

    def unitary_sparse_matrix_coo(self) -> Tuple[numpy.ndarray, Tuple[numpy.ndarray, numpy.ndarray]]:  # type: ignore
        """
        Return the unitary part of the superoperator in the sparse COO format.

        Returns:
            Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]]: The matrix representation of the unitary part of self.

        Raises:
            ValueError: CalculatorError.
            RuntimeError: Could not convert to complex superoperator matrix.
        """

    def sparse_lindblad_entries(self) -> List[Tuple[Tuple[numpy.ndarray, Tuple[numpy.ndarray, numpy.ndarray]], Tuple[numpy.ndarray, Tuple[numpy.ndarray, numpy.ndarray]], complex]]:  # type: ignore
        """
        Output the Lindblad entries in the form (left, right, rate) where left/right are the left and right lindblad operators, and rate is the lindblad rate respectively.

        Returns:
            List[Tuple[Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]], Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]], complex]]: The matrix representation of the noise part of self.

        Raises:
            ValueError: CalculatorError.
            RuntimeError: Could not convert to complex superoperator matrix.
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

    def json_schema(self) -> str:  # type: ignore
        """
        Return the JsonSchema for the json serialisation of the class.

        Returns:
            str: The json schema serialized to json
        """

class SpinLindbladOpenSystem(SystemType):
    """
    These are representations of noisy systems of spins.

    In a SpinLindbladOpenSystem is characterized by a SpinLindbladOpenOperator to represent the hamiltonian of the system, and an optional number of spins.

    Args:
        number_spins (Optional[int]): The number of spins in the SpinLindbladOpenSystem.

    Returns:
        SpinLindbladOpenSystem: The new SpinLindbladOpenSystem with the input number of spins.

    Examples
    --------

    .. code-block:: python

        import numpy.testing as npt
        import scipy.sparse as sp
        from qoqo_calculator_pyo3 import CalculatorComplex, CalculatorFloat
        from struqture_py.spins import SpinLindbladOpenSystem, DecoherenceProduct

        slns = SpinLindbladOpenSystem()
        dp = DecoherenceProduct().z(0).x(1)
        slns.system_add_operator_product(dp, 2.0)
        npt.assert_equal(slns.current_number_spins(), 2)
        npt.assert_equal(slns.system().get(dp), CalculatorFloat(2))
        dimension = 4**slns.number_spins()
        matrix = sp.coo_matrix(slns.sparse_matrix_superoperator_coo(), shape=(dimension, dimension))

    """

    def __init__(self, number_spins: Optional[int]):
        return

    def jordan_wigner(self):  # type: ignore
        """
        Transform the given spin object into a fermionic object using
        the Jordan Wigner mapping.
        """

    def current_number_spins(self) -> int:  # type: ignore
        """
        Return maximum spin index in object.

        Returns:
            int: Maximum index.
        """

    def number_spins(self) -> int:  # type: ignore
        """
        Return the number_spins input of self.

        Returns:
            int: The number of spins in self.
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

    def group(self, system, noise) -> SpinLindbladOpenSystem:  # type: ignore
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

    def empty_clone(self) -> SpinLindbladOpenSystem:  # type: ignore
        """
        Return an instance of self that has no entries but clones all other properties, with the given capacity.

        Returns:
            self: An empty clone with the same properties as self, with the given capacity.
        """

    def truncate(self, threshold) -> SpinLindbladOpenSystem:  # type: ignore
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

    def sparse_matrix_superoperator_coo(self, number_spins) -> Tuple[numpy.ndarray, Tuple[numpy.ndarray, numpy.ndarray]]:  # type: ignore
        """
        Construct the sparse matrix representation of the superoperator in COO representation.

        The superoperator for the operator O is defined as the Matrix S so that
        `flatten(-i [O, p]) = S flatten(p)` wher `[,]` is the commutator, `p` is a matrix
        and `flatten` flattens a matrix into a vector in row-major form.

        Args:
            number_spins: The number of spins in self.

        Returns:
            Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]]: The matrix representation of self.

        Raises:
            ValueError: CalculatorError.
            RuntimeError: Could not convert to complex superoperator matrix.
        """

    def unitary_sparse_matrix_coo(self) -> Tuple[numpy.ndarray, Tuple[numpy.ndarray, numpy.ndarray]]:  # type: ignore
        """
        Return the unitary part of the superoperator in the sparse COO format.

        Returns:
            Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]]: The matrix representation of the unitary part of self.

        Raises:
            ValueError: CalculatorError.
            RuntimeError: Could not convert to complex superoperator matrix.
        """

    def sparse_lindblad_entries(self) -> List[Tuple[Tuple[numpy.ndarray, Tuple[numpy.ndarray, numpy.ndarray]], Tuple[numpy.ndarray, Tuple[numpy.ndarray, numpy.ndarray]], complex]]:  # type: ignore
        """
        Output the Lindblad entries in the form (left, right, rate) where left/right are the left and right lindblad operators, and rate is the lindblad rate respectively.

        Returns:
            List[Tuple[Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]], Tuple[np.ndarray, Tuple[np.ndarray, np.ndarray]], complex]]: The matrix representation of the noise part of self.

        Raises:
            ValueError: CalculatorError.
            RuntimeError: Could not convert to complex superoperator matrix.
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

    def json_schema(self) -> str:  # type: ignore
        """
        Return the JsonSchema for the json serialisation of the class.

        Returns:
            str: The json schema serialized to json
        """

class PlusMinusProduct(ProductType):
    """
    PlusMinusProducts are combinations of SinglePlusMinusOperators on specific qubits.

    PlusMinusProducts can be used in either noise-free or a noisy system.
    They are representations of products of pauli matrices acting on qubits,
    in order to build the terms of a hamiltonian.
    For instance, to represent the term :math:`\sigma_0^{+}` :math:`\sigma_2^{+}` :

    `PlusMinusProduct().plus(0).plus(2)`.

    Returns:
        self: The new, empty PlusMinusProduct.

    Examples
    --------

    .. code-block:: python

        import numpy.testing as npt
        from struqture_py.spins import PlusMinusProduct

        pp = PlusMinusProduct().plus(0).minus(1).z(2)
        pp = pp.set_pauli(3, "+")
        npt.assert_equal(pp.get(0), "+")
        npt.assert_equal(pp.keys(), [0, 1, 2, 3])

    """

    def __init__(self):
        return

    def jordan_wigner(self):  # type: ignore
        """
        Transform the given spin object into a fermionic object using
        the Jordan Wigner mapping.
        """

    def plus(self, index: int) -> PlusMinusProduct:  # type: ignore
        """
        Set a new entry for SinglePlusMinusOperator X in the internal dictionary.

        Args:
            index (int): Index of set object.

        Returns:
            PlusMinusProduct: The PlusMinusProduct with the new entry.
        """

    def minus(self, index: int) -> PlusMinusProduct:  # type: ignore
        """
        Set a new entry for SinglePlusMinusOperator Y in the internal dictionary.

        Args:
            index (int): Index of set object.

        Returns:
            PlusMinusProduct: The PlusMinusProduct with the new entry.
        """

    def z(self, index: int) -> PlusMinusProduct:  # type: ignore
        """
        Set a new entry for SinglePlusMinusOperator Z in the internal dictionary.

        Args:
            index (int): Index of set object.

        Returns:
            PlusMinusProduct: The PlusMinusProduct with the new entry.
        """

    def set_pauli(self, index: int, pauli: str) -> PlusMinusProduct:  # type: ignore
        """
        Set a new entry in the internal_map. This function consumes self.

        Args:
            index (int): Index of set object.
            pauli (str): Value of set object.

        Returns:
            self: The entry was correctly set and the PlusMinusProduct is returned.
        """

    def get(self, index: int) -> Optional[str]:  # type: ignore
        """
        Get the pauli matrix corresponding to the index.

        Args:
            index (int): Index of get object.

        Returns:
            Optional[str]: The key's corresponding value (if it exists).
        """

    def keys(self) -> List[int]:  # type: ignore
        """
        Return a list of the unsorted keys in self.

        Returns:
            List[int]: The sequence of qubit index keys of self.
        """

    def current_number_spins(self) -> int:  # type: ignore
        """
        Return maximum index in self.

        Returns:
            int: Maximum index.
        """

    def remap_qubits(self, mapping: Dict[int, int]) -> PlusMinusProduct:  # type: ignore
        """
        Remap the qubits in a new instance of self (returned).

        Args:
            mapping (Dict[int, int]): The map containing the {qubit: qubit} mapping to use.

        Returns:
            self: The new instance of self with the qubits remapped.
        """

    def concatenate(self, other: PlusMinusProduct) -> List[int]:  # type: ignore
        """
        Return the concatenation of two objects of type `self` with no overlapping qubits.

        Args:
            other (self): The object to concatenate self with.

        Returns:
            List[int]: A list of the corresponding creator indices.

        Raises:
            ValueError: The two objects could not be concatenated.
        """

    def from_product(self, value: PauliProduct or DecoherenceProduct) -> List[Tuple[(PlusMinusProduct, Union[float, int, str, complex])]]:  # type: ignore
        """
        Creates a list of corresponding (PlusMinusProduct, CalculatorComplex) tuples from the input PauliProduct or DecoherenceProduct.

        Args:
            value (PauliProduct or DecoherenceProduct): The input object to convert.

        Returns:
            List[Tuple[(PlusMinusProduct, CalculatorComplex)]]: The converted input.

        Raises:
            ValueError: Input is neither a PauliProduct nor a DecoherenceProduct.
        """

    def to_pauli_product(self) -> List[Tuple[(PauliProduct, Union[float, int, str, complex])]]:  # type: ignore
        """
        DEPRECATED: Convert `self` into a list of (PauliProduct, CalculatorComplex) tuples.

        This function is deprecated, please use `to_pauli_product_list`

        Returns:
            List[Tuple[(PauliProduct, CalculatorComplex)]]: A list of the terms `self` corresponds to.
        """

    def to_pauli_product_list(self) -> List[Tuple[(PauliProduct, Union[float, int, str, complex])]]:  # type: ignore
        """
        Convert `self` into a list of (PauliProduct, CalculatorComplex) tuples.

        Returns:
            List[Tuple[(PauliProduct, CalculatorComplex)]]: A list of the terms `self` corresponds to.
        """

    def to_decoherence_product(self) -> List[Tuple[(DecoherenceProduct, Union[float, int, str, complex])]]:  # type: ignore
        """
        DEPRECATED: Convert `self` into a list of (DecoherenceProduct, CalculatorComplex) tuples.

        This function is deprecated, please use `to_decoherence_product_list`

        Returns:
            List[Tuple[(DecoherenceProduct, CalculatorComplex)]]: A list of the terms `self` corresponds to.
        """

    def to_decoherence_product_list(self) -> List[Tuple[(DecoherenceProduct, Union[float, int, str, complex])]]:  # type: ignore
        """
        Convert `self` into a list of (DecoherenceProduct, CalculatorComplex) tuples.

        Returns:
            List[Tuple[(DecoherenceProduct, CalculatorComplex)]]: A list of the terms `self` corresponds to.
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

    def from_string(self, input: str) -> PlusMinusProduct:  # type: ignore
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

    def json_schema(self) -> str:  # type: ignore
        """
        Return the JsonSchema for the json serialisation of the class.

        Returns:
            str: The json schema serialized to json
        """

class PlusMinusOperator:
    """
    These are representations of systems of spins.

    PlusMinusOperators are characterized by a SpinOperator to represent the hamiltonian of the spin system
    and an optional number of spins.

    Returns:
        self: The new PlusMinusOperator with the input number of spins.

    Examples
    --------

    .. code-block:: python

        import numpy.testing as npt
        from qoqo_calculator_pyo3 import CalculatorComplex
        from struqture_py.spins import PlusMinusOperator, PlusMinusProduct

        ssystem = PlusMinusOperator()
        pp = PlusMinusProduct().z(0)
        ssystem.add_operator_product(pp, 5.0)
        npt.assert_equal(ssystem.get(pp), CalculatorComplex(5))
        npt.assert_equal(ssystem.keys(), [pp])

    """

    def __init__(self):
        return

    def jordan_wigner(self):  # type: ignore
        """
        Transform the given spin object into a fermionic object using
        the Jordan Wigner mapping.
        """

    def separate_into_n_terms(self):  # type: ignore
        """
        Separate self into an operator with the terms of given number of spins and an operator with the remaining operations

        Args
            number_spins (int): Number of spins to filter for in the keys.

        Returns
            (PlusMinusOperator, PlusMinusOperator): Operator with the terms where number_spins matches the number of spins the operator product acts on and Operator with all other contributions.

        Raises:
            ValueError: Error in adding terms to return values.
        """

    def from_spin_system(self, value: SpinSystem) -> PlusMinusOperator:  # type: ignore
        """
        Convert a SpinSystem into a PlusMinusOperator.

        Args:
            value (SpinSystem): The SpinSystem to create the PlusMinusOperator from.

        Returns:
            PlusMinusOperator: The operator created from the input SpinSystem.

        Raises:
            ValueError: Could not create SpinSystem from input.
        """

    def from_spin_hamiltonian_system(self, value: SpinHamiltonianSystem) -> PlusMinusOperator:  # type: ignore
        """
        Convert a SpinHamiltonianSystem into a PlusMinusOperator.

        Args:
            value (SpinHamiltonianSystem): The SpinHamiltonianSystem to create the PlusMinusOperator from.

        Returns:
            PlusMinusOperator: The operator created from the input SpinSystem.

        Raises:
            ValueError: Could not create SpinHamiltonianSystem from input.
        """

    def to_spin_system(self, number_spins: Optional[int]) -> SpinSystem:  # type: ignore
        """
        Convert a PlusMinusOperator into a SpinSystem.

        Args:
            number_spins (Optional[int]): The number of spins to initialize the SpinSystem with.

        Returns:
            SpinSystem: The operator created from the input PlusMinusOperator and optional number of spins.

        Raises:
            ValueError: Could not create SpinSystem from PlusMinusOperator.
        """

    def to_spin_hamiltonian_system(self, number_spins: Optional[int]) -> SpinHamiltonianSystem:  # type: ignore
        """
        Convert a PlusMinusOperator into a SpinHamiltonianSystem.

        Args:
            number_spins (Optional[int]): The number of spins to initialize the SpinHamiltonianSystem with.

        Returns:
            SpinHamiltonianSystem: The operator created from the input PlusMinusOperator and optional number of spins.

        Raises:
            ValueError: Could not create SpinHamiltonianSystem from PlusMinusOperator.
        """

    def keys(self) -> List[OperatorProduct]:  # type: ignore
        """
        Return a list of the unsorted keys in self.

        Returns:
            List[OperatorProduct]: The sequence of keys of the self.
        """

    def empty_clone(self, capacity: Optional[int]) -> PlusMinusOperator:  # type: ignore
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

    def truncate(self, threshold: float) -> PlusMinusOperator:  # type: ignore
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

    def hermitian_conjugate(self) -> PlusMinusOperator:  # type: ignore
        """
        Return the hermitian conjugate of self.

        Returns:
            self: The hermitian conjugate of self.
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

    def json_schema(self) -> str:  # type: ignore
        """
        Return the JsonSchema for the json serialisation of the class.

        Returns:
            str: The json schema serialized to json
        """

class PlusMinusLindbladNoiseOperator(NoiseType):
    """
    These are representations of noisy systems of spins.

    In a PlusMinusLindbladNoiseOperator is characterized by a SpinLindbladNoiseOperator to represent the hamiltonian of the spin system, and an optional number of spins.

    Returns:
        self: The new PlusMinusLindbladNoiseOperator with the input number of spins.

    Examples
    --------

    .. code-block:: python

        import numpy.testing as npt
        from qoqo_calculator_pyo3 import CalculatorComplex
        from struqture_py.spins import PlusMinusLindbladNoiseOperator, PlusMinusProduct

        slns = PlusMinusLindbladNoiseOperator()
        dp = PlusMinusProduct().z(0).plus(1)
        slns.add_operator_product((dp, dp), 2.0)
        npt.assert_equal(slns.get((dp, dp)), CalculatorComplex(2))
        npt.assert_equal(slns.keys(), [(dp, dp)])

    """

    def __init__(self):
        return

    def jordan_wigner(self):  # type: ignore
        """
        Transform the given spin object into a fermionic object using
        the Jordan Wigner mapping.
        """

    def separate_into_n_terms(self):  # type: ignore
        """
        Separate self into an operator with the terms of given number of spins (left and right) and an operator with the remaining operations.

        Args
            number_spins_left (int): Number of spin to filter for in the left key.
            number_spins_right (int): Number of spin to filter for in the right key.

        Returns
            Tuple[PlusMinusLindbladNoiseOperator, PlusMinusLindbladNoiseOperator]: Operator with the noise terms where number_spins (left and right) matches the number of spins the operator product acts on and Operator with all other contributions.

        Raises:
            ValueError: Error in adding terms to return values.
        """

    def from_spin_noise_system(self, value: SpinLindbladNoiseSystem) -> PlusMinusLindbladNoiseOperator:  # type: ignore
        """
        Convert a SpinLindbladNoiseSystem into a PlusMinusLindbladNoiseOperator.

        Args:
            value (SpinLindbladNoiseSystem): The SpinLindbladNoiseSystem to create the PlusMinusLindbladNoiseOperator from.

        Returns:
            PlusMinusLindbladNoiseOperator: The operator created from the input SpinLindbladNoiseSystem.

        Raises:
            ValueError: Could not create SpinLindbladNoiseSystem from input.
        """

    def to_spin_noise_system(self, number_spinss: Optional[int]) -> SpinLindbladNoiseSystem:  # type: ignore
        """
        Convert a PlusMinusLindbladNoiseOperator into a SpinLindbladNoiseSystem.

        Args:
            number_spinss (Optional[int]): The number of spins to initialize the SpinLindbladNoiseSystem with.

        Returns:
            SpinLindbladNoiseSystem: The operator created from the input PlusMinusLindbladNoiseOperator and optional number of spins.

        Raises:
            ValueError: Could not create SpinLindbladNoiseSystem from PlusMinusLindbladNoiseOperator.
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

    def empty_clone(self, capacity) -> PlusMinusLindbladNoiseOperator:  # type: ignore
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

    def truncate(self, threshold) -> PlusMinusLindbladNoiseOperator:  # type: ignore
        """
        Truncate self by returning a copy without entries under a threshold.

        Args:
            threshold: The threshold for inclusion.

        Returns:
            self: The truncated version of self.
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

    def json_schema(self) -> str:  # type: ignore
        """
        Return the JsonSchema for the json serialisation of the class.

        Returns:
            str: The json schema serialized to json
        """
