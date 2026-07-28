This is exactly the kind of systems problem that forces you to connect **universal algebra**, **compiler construction**, and **lattice theory**. Here's a specification in the style of a research or systems interview rather than a pure mathematics exercise.

---

# Problem: Build a Symbolic Lattice Algebra Evaluator

## Background

You are designing the symbolic core of a lattice algebra engine similar in spirit to an optimizing compiler or computer algebra system.

The engine manipulates finite lattices symbolically before constructing concrete Hasse diagrams.

The system must support symbolic reasoning, normalization, memoization, and efficient evaluation.

---

# Input Language

The input language consists of atomic lattices

[
\mathcal A={
\mathbf 0,
\mathbf 1,
C_n,
F_n,
B_n,
M_n,
N_n,
P_1,P_2,\ldots
}
]

where

* (C_n) chain
* (F_n) fence
* (B_n) Boolean lattice
* (M_n) diamond
* (N_n) pentagon

and operators

[
\Omega=
{
\oplus,
\times,
\sqcup,
\operatorname{Dual},
\operatorname{Ideal},
\operatorname{Downset},
\operatorname{Filter}
}.
]

Example expression

[
\operatorname{Ideal}
\left(
(F_5\oplus C_2)
\times
(M_3\sqcup C_4)
\right).
]

---

# Required Tasks

## Part A — Formal Syntax

Define

1. the signature

[
\Omega=(\Omega_0,\Omega_1,\Omega_2)
]

2. the free term algebra

[
T(\Omega)
]

3. a grammar generating all valid expressions.

---

## Part B — Parser

Design a parser which

maps

[
\Sigma^*
\longrightarrow
T(\Omega).
]

Your parser must

* detect malformed expressions
* support prefix or infix notation
* prove uniqueness of the parse

Analyze complexity.

---

## Part C — Internal Representation

Choose one representation.

Options include

* AST
* DAG
* hash-consed DAG
* e-graph

Justify your choice.

Questions

* When should identical subexpressions share nodes?
* Which representation minimizes repeated evaluation?

---

## Part D — Algebraic Laws

Implement rewrite rules.

Examples

Associativity

[
(P\oplus Q)\oplus R
\rightarrow
P\oplus Q\oplus R
]

Double dual

[
Dual(Dual(P))
\rightarrow
P
]

Identity

[
P\times\mathbf1
\rightarrow
P
]

Absorption

(if applicable)

Canonical ordering

[
Q\oplus P
\rightarrow
P\oplus Q
]

if the operator is commutative.

Explain how rewrite termination and confluence affect correctness.

---

## Part E — Evaluation

Define an algebra homomorphism

[
\operatorname{eval}
:
T(\Omega)
\rightarrow
\mathbf{FinLat}.
]

The evaluator must construct

* elements
* order relation
* cover relation
* Hasse diagram

for every expression.

---

## Part F — Optimization

Repeated subexpressions should only be evaluated once.

Example

[
(P\oplus Q)
\times
(P\oplus Q)
]

should compute

[
P\oplus Q
]

once.

Design a memoization strategy.

Analyze asymptotic improvement.

---

## Part G — Serialization

Support

* infix
* Polish
* Reverse Polish
* JSON

Prove that serialization and parsing satisfy

[
decode(encode(t))
=================

t.
]

---

## Part H — Algorithms

Implement algorithms for

* height
* width
* maximal antichains
* ideals
* filters
* join irreducibles
* meet irreducibles
* linear extensions

State complexity.

---

## Part I — Category Theory

Interpret your implementation categorically.

Identify

* the free algebra

[
T(\Omega)
]

* the semantic algebra

[
\mathbf{FinLat}
]

* the evaluation homomorphism

[
eval:T(\Omega)\rightarrow\mathbf{FinLat}.
]

Explain why parsing constructs the initial algebra while evaluation is its unique homomorphism.

---

# Stretch Goal

Suppose users repeatedly issue queries such as

[
Ideal((P\oplus Q)\times R)
]

where only (P,Q,R) change.

Design a compiler that transforms the symbolic expression into an optimized instruction sequence or bytecode supporting:

* constant folding,
* common subexpression elimination,
* incremental recomputation when only one leaf changes,
* parallel evaluation of independent subexpressions.

Discuss whether your instruction language should be:

* stack-based (RPN-like),
* register/SSA-based,
* graph-based,
  or another representation, and justify the trade-offs in terms of evaluation speed, optimization opportunities, and memory usage.

---

I like this formulation because it naturally mirrors the architecture of a modern compiler:

1. **Syntax** (free term algebra),
2. **Parsing** (strings → AST),
3. **Normalization** (term rewriting),
4. **Optimization** (DAG/e-graph, memoization),
5. **Semantics** (homomorphism into finite lattices),
6. **Execution** (constructing concrete lattice objects).

It also forces you to think in the same layered way that production symbolic systems, theorem provers, and optimizing compilers are designed.
