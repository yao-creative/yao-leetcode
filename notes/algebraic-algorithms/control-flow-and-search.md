Your question is about **formalizing algorithmic control flow** in set theory. A useful way to think about it is:

* **Data** = sets.
* **State** = tuples of variables.
* **Transition** = a function on the state.
* **Condition** = a predicate on the state.
* **Loop** = repeated application of the transition until a predicate changes.

---

# 1. State space

Suppose an algorithm has variables

$$
i,\quad x,\quad S.
$$

The state space is

$$
X = \mathbb N \times A \times \mathcal P(U).
$$

A state is

$$
s=(i,x,S)\in X.
$$

---

# 2. Condition

A condition is simply a predicate

$$
C:X\to\{\top,\bot\}.
$$

or equivalently

$$
C\subseteq X.
$$

Example

$$
C(i,x,S)\iff i<n.
$$

---

# 3. Loop body

The loop body is a transition function

$$
T:X\to X.
$$

Example

```python
i += 1
sum += a[i]
```

becomes

$$
T(i,s)=(i+1,\ s+a_i).
$$

---

# 4. While loop

```python
while C(state):
    state = T(state)
```

is

$$
s_{k+1}=
\begin{cases}
T(s_k),&C(s_k)\\
s_k,&\neg C(s_k).
\end{cases}
$$

or

$$
F(s)=
\begin{cases}
T(s),&C(s)\\
s,&\text{otherwise}.
\end{cases}
$$

The algorithm computes

$$
F^n(s_0)
$$

until

$$
F(s_n)=s_n.
$$

Notice the endpoint is a **fixed point**.

---

# 5. For loop

A for loop

```python
for i=0..n-1:
    body
```

is bounded iteration.

Let

$$
T_i:X\to X
$$

be the body at iteration $i$.

Then

$$
s_n=(T_{n-1}\circ\cdots\circ T_1\circ T_0)(s_0).
$$

If the body is identical,

$$
T_i=T,
$$

then

$$
s_n=T^n(s_0).
$$

---

# 6. Conditional

```python
if P:
    A
else:
    B
```

is

$$
F(x)=
\begin{cases}
A(x),&P(x)\\
B(x),&\neg P(x).
\end{cases}
$$

So branching is simply choosing between functions.

---

# 7. Recursive traversal

DFS, recursion, tree traversal all repeatedly apply an expansion operator.

Let

$$
N:V\to\mathcal P(V)
$$

return neighboring nodes.

The recursive search builds

$$
R_0=\{v_0\},
$$

and

$$
R_{k+1}=R_k\cup\bigcup_{v\in R_k}N(v).
$$

Eventually

$$
R_{k+1}=R_k.
$$

This is exactly the reachable set.

---

# 8. Iterative traversal

BFS uses a frontier.

State

$$
(F,V)
$$

where

* $F$ = frontier
* $V$ = visited

Transition

$$
\begin{aligned}
F'
&=
\left(
\bigcup_{v\in F}N(v)
\right)
\setminus V,\\
V'
&=
V\cup F.
\end{aligned}
$$

Repeat until

$$
F=\varnothing.
$$

The frontier monotonically shrinks toward the fixed point.

---

# 9. Converging search on a set space

This is the most general viewpoint.

Suppose

$$
\Phi:\mathcal P(X)\to\mathcal P(X).
$$

You repeatedly compute

$$
S_{k+1}=\Phi(S_k).
$$

If

$$
S_{k+1}=S_k,
$$

the search has converged.

Examples:

* graph reachability
* dataflow analysis
* Datalog evaluation
* transitive closure
* abstract interpretation
* fixed-point algorithms

This is Kleene iteration.

---

# 10. Search as narrowing

Many optimization algorithms produce a decreasing sequence

$$
X_0 \supseteq X_1 \supseteq X_2 \supseteq \cdots
$$

where

$$
X_{k+1}=X_k\cap P_k
$$

for predicates $P_k$.

Examples include:

* branch-and-bound,
* SAT solving,
* CSP propagation,
* interval narrowing.

The search space becomes progressively smaller.

---

# 11. Search as expansion

Other searches grow information.

$$
S_0 \subseteq S_1 \subseteq S_2 \subseteq \cdots
$$

with

$$
S_{k+1}=S_k\cup E(S_k),
$$

where $E$ is an expansion operator.

Examples:

* BFS,
* DFS (reachable set),
* flood fill,
* closure algorithms,
* fixed-point inference.

---

# 12. Unified viewpoint

Every iterative algorithm can be modeled by the quadruple

$$
(X, T, C, I)
$$

where:

* $X$ is the **state space**.
* $I\in X$ is the **initial state**.
* $T:X\to X$ is the **transition** (loop body).
* $C:X\to\{\top,\bot\}$ is the **continuation predicate**.

The execution is the sequence

$$
s_0=I,\qquad s_{k+1}=T(s_k)
$$

while $C(s_k)$ holds, terminating at the least $n$ such that

$$
\neg C(s_n).
$$

Equivalently, define the guarded transition

$$
F(s)=
\begin{cases}
T(s), & C(s),\\
s, & \neg C(s),
\end{cases}
$$

and compute iterates $F^k(I)$. Termination occurs when a **fixed point** is reached:

$$
F(s^*)=s^*.
$$

From a category-theoretic perspective, $T$ is an endomorphism $T:X\to X$, the loop computes iterated composition $T^n$, and many recursive or traversal algorithms are instances of finding an initial algebra or least fixed point of a monotone endofunction on a poset or complete lattice. This explains why graph reachability, BFS/DFS, transitive closure, dataflow analysis, and Datalog evaluation all share the same underlying mathematical structure: they compute the least fixed point of an operator on a state or search space.
