The key idea is that **addition is usually not an operation on the subarrays themselves**. Instead, you first define a function from subarrays into an algebraic object (typically a monoid or group), then perform algebra there.

Let's formalize this.

Let

$$
A:I\to R,\qquad I=\{0,\dots,n-1\},
$$

where $R$ is typically $(\mathbb Z,+,0)$.

A contiguous subarray is

$$
A_{l,r}=A\!\restriction_{I_{l,r}}
$$

where

$$
I_{l,r}=\{i\in I:l\le i\le r\}.
$$

---

# 1. Sum is a fold

Define

$$
\Sigma:\mathcal C(A)\to R
$$

by

$$
\boxed{\Sigma(A_{l,r})=\sum_{i=l}^{r}A(i)}
$$

This is a morphism from the collection of contiguous subarrays into the additive monoid.

Notice

* domain = subarrays
* codomain = numbers

Addition happens **after** applying $\Sigma$.

---

# 2. Union of adjacent intervals

Suppose

$$
r+1=s.
$$

Then

$$
I_{l,t}=I_{l,r}\cup I_{s,t}.
$$

These intervals are disjoint.

Therefore

$$
\boxed{\Sigma(A_{l,t})=\Sigma(A_{l,r})+\Sigma(A_{s,t})}
$$

This is the algebra behind prefix sums.

---

# 3. Difference

Suppose

$$
I_{m,r}\subseteq I_{l,r}.
$$

Then

$$
I_{l,m-1}=I_{l,r}\setminus I_{m,r}.
$$

Hence

$$
\boxed{\Sigma(I_{l,m-1})=\Sigma(I_{l,r})-\Sigma(I_{m,r})}
$$

This is why subtraction appears in prefix sums.

---

# 4. Prefix sums

Define

$$
P(k)=\sum_{i=0}^{k-1}A(i).
$$

Then

$$
\boxed{\Sigma(A_{l,r})=P(r+1)-P(l).}
$$

The set algebra is

$$
I_{0,r}=I_{0,l-1}\;\cup\;I_{l,r}
$$

(disjoint union)

which becomes

$$
P(r+1)=P(l)+\Sigma(A_{l,r}).
$$

---

# 5. Indicator-function viewpoint

Represent every interval by its characteristic function

$$
\chi_{l,r}:I\to\{0,1\}.
$$

Then

$$
\Sigma(A_{l,r})=\sum_{i\in I}A(i)\chi_{l,r}(i).
$$

Now interval algebra becomes ordinary algebra.

For example,

$$
\chi_{l,t}=\chi_{l,r}+\chi_{r+1,t}.
$$

This is extremely useful because many range algorithms are just algebra on indicator functions.

---

# 6. Measure-theoretic viewpoint

The sum is an integral over a finite counting measure:

$$
\boxed{\Sigma(A_{l,r})=\int_{I_{l,r}}A\,d\mu}
$$

where

$$
\mu(S)=|S|.
$$

Then every theorem about finite sums becomes a theorem about measures.

---

# 7. Category-theoretic viewpoint

Let

* $\mathbf{Int}(I)$ be the category (or poset) of intervals ordered by inclusion.
* $R=(R,+,0)$ be a commutative monoid.

Then

$$
\Sigma:\mathbf{Int}(I)\to R
$$

is a valuation satisfying

$$
\boxed{\Sigma(U\cup V)=\Sigma(U)+\Sigma(V)}
$$

whenever

* $U\cap V=\varnothing$,
* $U\cup V$ is again an interval.

This "valuation" perspective generalizes directly to segment trees, Fenwick trees, interval DP, and measures.

---

## High-ROI abstraction for algorithms

Almost every range algorithm can be expressed as the composition

$$
\boxed{\text{Interval}\xrightarrow{\chi}\text{Indicator Function}\xrightarrow{\text{fold}}(M,\otimes,e)}
$$

where:

* interval $\mapsto$ indicator function (select the indices),
* indicator function $\mapsto$ fold into a monoid.

Different algorithms simply choose different monoids:

* Sum: $(\mathbb{Z},+,0)$
* Product: $(R,\cdot,1)$
* XOR: $(\{0,1\}^k,\oplus,0)$
* Minimum: $(\mathbb{R}\cup\{\infty\},\min,\infty)$
* Maximum: $(\mathbb{R}\cup\{-\infty\},\max,-\infty)$
* GCD: $(\mathbb{N},\gcd,0)$

This fold-over-an-interval pattern is the unifying algebraic structure behind prefix sums, sparse tables, Fenwick trees, segment trees, sliding windows (when invertibility is available), and many dynamic programming recurrences.
