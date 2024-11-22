"Tests for the mappings python interface."

from struqture_py.spins import *
from struqture_py.fermions import *


def test_jordan_wigner_spin_to_fermion():
    pp = PauliProduct().x(0).z(1).y(2)
    assert type(pp.jordan_wigner()) == FermionSystem

    dp = DecoherenceProduct().x(0).iy(2)
    assert type(dp.jordan_wigner()) == FermionSystem

    pmp = PlusMinusProduct().plus(0).minus(1)
    assert type(pmp.jordan_wigner()) == FermionSystem

    pmo = PlusMinusOperator()
    pmo.add_operator_product(pmp, 1.0)
    assert type(pmo.jordan_wigner()) == FermionSystem

    pmns = PlusMinusLindbladNoiseOperator()
    pmns.add_operator_product((pmp, pmp), 2.0)
    assert type(pmns.jordan_wigner()) == FermionLindbladNoiseSystem

    ss = SpinSystem(4)
    ss.add_operator_product(pp, 5.0)
    assert type(ss.jordan_wigner()) == FermionSystem

    shs = SpinHamiltonianSystem(5)
    shs.add_operator_product(pp, 5.0)
    assert type(shs.jordan_wigner()) == FermionHamiltonianSystem

    slns = SpinLindbladNoiseSystem(4)
    slns.add_operator_product((dp, dp), 2.0)
    assert type(slns.jordan_wigner()) == FermionLindbladNoiseSystem

    slos = SpinLindbladOpenSystem()
    slos.system_add_operator_product(pp, 2.0)
    slos.noise_add_operator_product((dp, dp), 2.0)
    assert type(slos.jordan_wigner()) == FermionLindbladOpenSystem


def test_jordan_wigner_fermion_to_spin():
    fp = FermionProduct([0], [2, 3])
    assert type(fp.jordan_wigner()) == SpinSystem

    hfp = HermitianFermionProduct([0], [2, 3])
    assert type(hfp.jordan_wigner()) == SpinHamiltonianSystem

    fs = FermionSystem(4)
    fs.add_operator_product(fp, 1.0)
    assert type(fs.jordan_wigner()) == SpinSystem

    fh = FermionHamiltonianSystem(5)
    fh.add_operator_product(hfp, 1.0)
    assert type(fh.jordan_wigner()) == SpinHamiltonianSystem

    flns = FermionLindbladNoiseSystem()
    flns.add_operator_product((fp, fp), 1.0)
    assert type(flns.jordan_wigner()) == SpinLindbladNoiseSystem

    flos = FermionLindbladOpenSystem()
    flos.system_add_operator_product(fp, 2.0)
    flos.noise_add_operator_product((fp, fp), 2.0)
    assert type(flos.jordan_wigner()) == SpinLindbladOpenSystem
