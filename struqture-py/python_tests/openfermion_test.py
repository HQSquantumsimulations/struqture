from struqture_py.spins import PauliHamiltonian
from struqture_py.openfermion_interface import openfermion_to_struqture, struqture_to_openfermion
from openfermion import QubitOperator


def test_openfermion_to_struqture():
    struqture_hamiltonian = PauliHamiltonian()
    struqture_hamiltonian.add_operator_product("0X1Y", 0.68)
    struqture_hamiltonian.add_operator_product("0X1X", -0.5)
    struqture_hamiltonian.add_operator_product("0Y1Y", 0.1)

    openfermion_hamiltonian = (
        0.68 * QubitOperator("X0 Y1") + 0.1 * QubitOperator("Y0 Y1") - 0.5 * QubitOperator("X0 X1")
    )

    assert struqture_hamiltonian == openfermion_to_struqture(openfermion_hamiltonian)


def test_openfermion_to_struqture_to_openfermion():
    openfermion_hamiltonian = (
        0.68 * QubitOperator("X0 Y1") + 0.1 * QubitOperator("Y0 Y1") - 0.5 * QubitOperator("X0 X1")
    )
    struqture_hamiltonian = openfermion_to_struqture(openfermion_hamiltonian)
    assert openfermion_hamiltonian == struqture_to_openfermion(struqture_hamiltonian)


def test_struqture_to_openfermion():
    struqture_hamiltonian = PauliHamiltonian()
    struqture_hamiltonian.add_operator_product("0X1Y", 0.68)
    struqture_hamiltonian.add_operator_product("0X1X", -0.5)
    struqture_hamiltonian.add_operator_product("0Y1Y", 0.1)

    openfermion_hamiltonian = (
        0.68 * QubitOperator("X0 Y1") + 0.1 * QubitOperator("Y0 Y1") - 0.5 * QubitOperator("X0 X1")
    )

    assert openfermion_hamiltonian == struqture_to_openfermion(struqture_hamiltonian)


def test_struqture_to_openfermion_to_struqture():
    struqture_hamiltonian = PauliHamiltonian()
    struqture_hamiltonian.add_operator_product("0X1Y", 0.68)
    struqture_hamiltonian.add_operator_product("0X1X", -0.5)
    struqture_hamiltonian.add_operator_product("0Y1Y", 0.1)

    openfermion_hamiltonian = struqture_to_openfermion(struqture_hamiltonian)

    assert struqture_hamiltonian == openfermion_to_struqture(openfermion_hamiltonian)
