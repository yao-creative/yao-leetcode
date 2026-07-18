Yes. The thing you are asking for is the **logical specification first**, before introducing the optimization operator (\max). Then the recursive state can be defined as a **set-theoretic object** (a derived set/function), not as an informal programming variable.

Let's do it carefully.

---

## 1. Original problem as pure quantifiers

Let:

[
a:{1,\dots,n}\rightarrow\mathbb R
]

The answer is a value (m\in\mathbb R) such that:

[
\boxed{
\exists i^*,j^*
\left[
1\le i^*\le j^*\le n
\land
\forall i,j
\left(
(1\le i\le j\le n)
\Rightarrow
\sum_{k=i}^{j}a(k)
\le
\sum_{k=i^*}^{j^*}a(k)
\right)
\right]
}
]

Meaning:

> there exists an interval ((i^*,j^*)) such that every valid interval has sum no larger.

---

## 2. Expand the interval universe

Define:

[
I={1,\dots,n}
]

and:

[
S=
{(i,j)\in I\times I\mid i\le j}.
]

The specification becomes:

[
\boxed{
\exists s^*\in S:
\forall s\in S:
F(s)\le F(s^*)
}
]

where:

[
F(i,j)=\sum_{k=i}^{j}a(k)
]

---

# Now derive the recursive state

The key move is **not introducing max**. We introduce an equivalence class of candidates.

---

## 3. Partition candidates by endpoint

Define:

[
S_j=
{(i,j)\in S\mid i\le j}
]

This is:

> all contiguous subsequences ending at (j).

So:

[
S=
\bigcup_{j\in I}S_j
]

and:

[
\exists(i,j)\in S
]

becomes:

[
\exists j\in I,\exists(i,j)\in S_j
]

---

## 4. Define the state as a set of optimal candidates

Instead of:

[
B_j=\text{number}
]

define:

[
\boxed{
C_j=
\left{
(i,j)\in S_j
\mid
\forall(r,j)\in S_j:
\sum_{k=r}^{j}a(k)
\le
\sum_{k=i}^{j}a(k)
\right}
}
]

Read:

[
C_j=
\text{the set of all best intervals ending at }j.
]

This is pure set-builder notation.

No max.

---

## 5. Derive the recursive relationship

Take:

[
(i,j)\in C_j
]

There are two cases.

### Case 1

[
i=j
]

The interval is:

[
(j,j)
]

with sum:

[
a(j).
]

---

### Case 2

[
i<j
]

Then:

[
(i,j-1)\in S_{j-1}
]

and:

[
\sum_{k=i}^{j}a(k)
==================

\sum_{k=i}^{j-1}a(k)+a(j).
]

The best predecessor must come from:

[
C_{j-1}.
]

Therefore the transition is:

[
C_j=
\begin{cases}
{(j,j)}
&
\text{if starting is better}
\
{(i,j)\mid(i,j-1)\in C_{j-1}}
&
\text{if extending is better}
\end{cases}
]

More formally, the next state is a function:

[
T_j:\mathcal P(S_{j-1})\rightarrow\mathcal P(S_j)
]

where:

[
C_j=T_j(C_{j-1}).
]

---

## 6. The scalar Kadane state is a quotient

The normal recurrence:

[
b_j=\max(a(j),b_{j-1}+a(j))
]

is obtained by applying a projection:

[
\phi(C_j)=
\sum_{k=i}^{j}a(k)
]

for any:

[
(i,j)\in C_j.
]

Because all members of (C_j) have equal value.

So:

[
\boxed{
b_j=\phi(C_j)
}
]

The "variable" (b_j) is actually a **quotient representation** of the larger set state.

---

The hierarchy is:

[
\text{all intervals}
]

[
\downarrow
]

[
S_j={\text{intervals ending at }j}
]

[
\downarrow
]

[
C_j={\text{optimal intervals in }S_j}
]

[
\downarrow
]

[
b_j=\text{equivalence class representative value}
]

So yes: the recursive state absolutely can be defined in set theory. The usual dynamic-programming notation hides the fact that it is a **derived object obtained by restricting a quantified search space and proving closure under a transition function**.
