"Tests for the mappings python interface."

from struqture_py.spins import *; 
from struqture_py.fermions import *;

def test_jordan_wigner_spin_to_fermion():

    pp = PauliProduct().x(0).z(1).y(2)
    pp.jordan_wigner()

    dp = DecoherenceProduct().x(0).iy(2)
    print(dp.jordan_wigner())

    pmp = PlusMinusProduct().plus(0).minus(1)
    print(pmp.jordan_wigner())

    pmo  = PlusMinusOperator()
    pmo.add_operator_product(pmp, 1.0)
    print(pmo.jordan_wigner())

    pmns = PlusMinusLindbladNoiseOperator()
    pmns.add_operator_product((pmp, pmp), 2.0)
    print(pmns.jordan_wigner())

    ss = SpinSystem(4)
    ss.add_operator_product(pp, 5.0)

    shs = SpinHamiltonianSystem(5)
    shs.add_operator_product(pp, 5.0)
    print(shs.jordan_wigner())

    slns = SpinLindbladNoiseSystem(4)
    slns.add_operator_product((dp, dp), 2.0)
    print(slns.jordan_wigner())

    slos = SpinLindbladOpenSystem()
    slos.system_add_operator_product(pp, 2.0)
    slos.noise_add_operator_product((dp, dp), 2.0)
    print(slos.jordan_wigner())
