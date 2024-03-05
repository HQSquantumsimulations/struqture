"""Just a simple test"""
from struqture_py import (spins, bosons, fermions, mixed_systems)

def test_simple():
    # testing Mixed products
    a = bosons.BosonProduct([0], [0])
    print(a.__repr__())
    b = bosons.BosonProduct.from_string(a.__repr__())
    print(b.__repr__())

    a = spins.PauliProduct().x(0).y(1).z(2)
    print(a.__repr__())
    b = spins.PauliProduct.from_string(a.__repr__())
    print(b.__repr__())

    a = spins.DecoherenceProduct().x(0).iy(1).z(2)
    print(a.__repr__())
    b = spins.DecoherenceProduct.from_string(a.__repr__())
    print(b.__repr__())

    a = mixed_systems.HermitianMixedProduct(
        ["1X", spins.PauliProduct().z(1)],
        [bosons.BosonProduct([0], [0]), "a1"],
        [fermions.FermionProduct([0],[0])])
    print(a.__repr__())
    b = mixed_systems.HermitianMixedProduct.from_string(a.__repr__())
    print(b.__repr__())
    s = mixed_systems.MixedHamiltonian(2, 2, 1)
    print(s.__repr__())
    # This should not fail
    s.set(a, 2 + 2j)

    c = mixed_systems.HermitianMixedProduct(
        ["1X", spins.PauliProduct().z(1)],
        [bosons.BosonProduct([0], [0]), ""],
        [fermions.FermionProduct([0],[0])])
    # This should  fail
    # s.set_mixed_product(c, 2 + 2j)
    print(s.__repr__())
    c = s.get(a)
    print("from get", c)
    s.add_operator_product(a, 3)
    print(s.__repr__())

    a = mixed_systems.HermitianMixedProduct(
        ["1X",
        spins.PauliProduct().z(1)],
        [bosons.BosonProduct([0], [0]), "a1"],
        [fermions.FermionProduct([0],[0])])
    print(a.__repr__())
    b = mixed_systems.HermitianMixedProduct.from_string(a.__repr__())
    print(b.__repr__())

    # Testing LindbladOperator and decoherence products

    a = mixed_systems.MixedDecoherenceProduct(
        ["1X", spins.DecoherenceProduct().z(1).iy(0)],
        [bosons.BosonProduct([0], [0]), "a1"],
        [fermions.FermionProduct([0],[0])])

    s = mixed_systems.MixedLindbladNoiseOperator(2, 2, 1)
    print(s.__repr__())
    s.set((a, a.__repr__()), 2+2j)
    print(s.__repr__())
    c = s.get((a, a.__repr__()))
    print("from get",c)
    s.add_operator_product((a, a.__repr__()), 3)
    print(s.__repr__())
