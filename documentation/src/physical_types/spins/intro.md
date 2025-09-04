# Spins

Struqture can be used to represent spin operators, hamiltonians and open systems, such as:

\\[
\hat{H} = \sum_{i, j=0}^N \alpha_{i, j} (\sigma^x_i \sigma^x_j + \sigma^y_i \sigma^y_j) + \sum_{i=0}^N \lambda_i \sigma^z_i .
\\] 

All spin objects in `struqture` are expressed based on products of either Pauli matrices {X, Y, Z} or operators which are better suited to express decoherence {X, iY, Z}. 

The Pauli matrices (coherent dynamics):
* I: identity matrix
\\[
\begin{pmatrix}
1 & 0\\\\
0 & 1
\end{pmatrix}
\\]

* X: \\( \sigma^x \\) matrix
\\[
\begin{pmatrix}
0 & 1\\\\
1 & 0
\end{pmatrix}
\\]

* Y: \\( \sigma^y \\) matrix
\\[
\begin{pmatrix}
0 & -i\\\\
i & 0
\end{pmatrix}
\\]

* Z: \\( \sigma^z \\) matrix
\\[
\begin{pmatrix}
1 & 0\\\\
0 & -1
\end{pmatrix}
\\]

Struqture also provides modified Pauli matrices for incoherent dynamics. These are used in order to avoid complex noise operators. The identity, X and Z matrices remain identical to the ones defined above, but the Y matrix is modified as follows:

* iY: \\( \mathrm{i} \sigma^y \\)
\\[
\begin{pmatrix}
0 & 1 \\\\
-1 & 0
\end{pmatrix}
\\]

The simplest way that the user can interact with these matrices is by using symbolic representation: `"0X1X"` represents a \\( \sigma^x_0 \sigma^x_1 \\) term. This is a very scalable approach, as indices not mentioned in this string representation are assumed to be acted on by the identity operator: `"7Y25Z"` represents a \\( \sigma^y_7 \sigma^z_{25} \\) term, where all other spins (0 to 6 and 8 to 24) are acted on by \\(I\\).

However, for more fine-grain control over the operators, we invite the user to look into the `PauliProducts` and `DecoherenceProducts` classes, in the [Building blocks](./products.md) section. Otherwise please proceed to the [coherent](./noisefree.md) or [decoherent](./noisy.md) dynamics section.

**NOTE**: There exists an alternative representation, the {+, -, Z} basis, detailed in the [alternative basis](./plus_minus.md) section.