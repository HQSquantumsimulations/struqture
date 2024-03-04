"Tests for the mappings python interface."

from struqture_py.spins import *
from struqture_py.fermions import *


def test_jordan_wigner_spin_to_fermion():
    pp = PauliProduct().x(0).z(1).y(2)
    assert type(pp.jordan_wigner()) == FermionOperator

    dp = DecoherenceProduct().x(0).iy(2)
    assert type(dp.jordan_wigner()) == FermionOperator

    pmp = PlusMinusProduct().plus(0).minus(1)
    assert type(pmp.jordan_wigner()) == FermionOperator

    pmo = PlusMinusOperator()
    pmo.add_operator_product(pmp, 1.0)
    assert type(pmo.jordan_wigner()) == FermionOperator

    pmns = PlusMinusLindbladNoiseOperator()
    pmns.add_operator_product((pmp, pmp), 2.0)
    assert type(pmns.jordan_wigner()) == FermionLindbladNoiseOperator

    ss = SpinOperator()
    ss.add_operator_product(pp, 5.0)
    assert type(ss.jordan_wigner()) == FermionOperator

    shs = SpinHamiltonian()
    shs.add_operator_product(pp, 5.0)
    assert type(shs.jordan_wigner()) == FermionHamiltonian

    slns = SpinLindbladNoiseOperator()
    slns.add_operator_product((dp, dp), 2.0)
    assert type(slns.jordan_wigner()) == FermionLindbladNoiseOperator

    slos = SpinLindbladOpenSystem()
    slos.system_add_operator_product(pp, 2.0)
    slos.noise_add_operator_product((dp, dp), 2.0)
    assert type(slos.jordan_wigner()) == FermionLindbladOpenSystem


def test_jordan_wigner_fermion_to_spin():
    fp = FermionProduct([0], [2, 3])
    assert type(fp.jordan_wigner()) == SpinOperator

    hfp = HermitianFermionProduct([0], [2, 3])
    assert type(hfp.jordan_wigner()) == SpinHamiltonian

    fs = FermionOperator()
    fs.add_operator_product(fp, 1.0)
    assert type(fs.jordan_wigner()) == SpinOperator

    fh = FermionHamiltonian()
    fh.add_operator_product(hfp, 1.0)
    assert type(fh.jordan_wigner()) == SpinHamiltonian

    flns = FermionLindbladNoiseOperator()
    flns.add_operator_product((fp, fp), 1.0)
    assert type(flns.jordan_wigner()) == SpinLindbladNoiseOperator

    flos = FermionLindbladOpenSystem()
    flos.system_add_operator_product(fp, 2.0)
    flos.noise_add_operator_product((fp, fp), 2.0)
    assert type(flos.jordan_wigner()) == SpinLindbladOpenSystem
