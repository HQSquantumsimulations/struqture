from struqture_py import PauliHamiltonian, PauliProduct
from openfermion import QubitOperator


def struqture_to_openfermion(struqture_hamiltonian: PauliHamiltonian):
    """Transform a struqture Hamiltonian to an OpenFermion Hamiltonian (QubitOperator).

    Args:
        struqture_hamiltonian (PauliHamiltonian): struqture Hamiltonian to be transformed.

    Returns:
        QubitOperator: OpenFermion Hamiltonian equivalent to the struqture Hamiltonian.
    """
    openfermion_hamiltonian = QubitOperator()
    for term in struqture_hamiltonian.keys():
        value = struqture_hamiltonian.get(term)
        openfermion_term_str = ""
        is_first_spin = True
        for spin in term.keys():
            if not is_first_spin:
                openfermion_term_str += " "
            else:
                is_first_spin = False
            operator = term.get(spin)
            openfermion_term_str += f"{operator}{spin}"
        openfermion_hamiltonian += value.float() * QubitOperator(openfermion_term_str)
    return openfermion_hamiltonian


def openfermion_to_struqture(openfermion_hamiltonian: QubitOperator):
    """Transform an OpenFermion Hamiltonian (QubitOperator) to a struqture Hamiltonian.

    Args:
        openfermion_hamiltonian (QubitOperator): OpenFermion Hamiltonian to be transformed.

    Returns:
        PauliHamiltonian: Struqture Hamiltonian equivalent to the OpenFermion Hamiltonian.
    """
    struqture_hamiltonian = PauliHamiltonian()
    for term, coefficient in openfermion_hamiltonian.terms.items():
        term_product = PauliProduct()
        for spin, operator in term:
            term_product = term_product.set_pauli(spin, operator)
        struqture_hamiltonian.add_operator_product(term_product, coefficient)
    return struqture_hamiltonian
