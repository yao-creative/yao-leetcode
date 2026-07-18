Yes — your intuition is close, but there are **three different things that need to be separated**:

1. **The syntax of a λ-term**: what `λx.t` is.
2. **The free-variable function**: what `FV` computes.
3. **Substitution as dependency resolution**: where your DAG/Kahn's algorithm intuition becomes useful.

The most important correction is:

> `λx.t` is not "the parameter `x`" and it is not an evaluation equation. It is a **term constructor** that binds the variable `x` in the term `t`.

---

# 1. What is $\lambda x.t$?

Suppose:

$$
t = x+y.
$$

Then:

$$
\lambda x.(x+y)
$$

means:

> Construct a function whose input is called $x$, and whose output is $x+y$.

The variable $x$ is the **formal parameter**.

The entire expression:

$$
\lambda x.(x+y)
$$

is a term.

The body is:

$$
x+y.
$$

The binder is:

$$
x.
$$

The binding relation is:

$$
\lambda x
\quad\text{binds occurrences of }x\text{ in its body}.
$$

So:

$$
\lambda x.(x+y)
$$

has the structure:

$$
\boxed{
\text{binder}
+
\text{bound variable}
+
\text{body}
}
$$

More formally:

$$
\lambda x.t
$$

is an abstract syntax tree node with:

* operator: `λ`,
* binder: $x$,
* child: $t$.

It is analogous to a programming language AST node:

```text
Lambda(
    parameter = x,
    body = ...
)
```

The crucial point is that the binder changes the **scope** of $x$.

---

# 2. Why does $FV(\lambda x.t)=FV(t)\setminus{x}$?

`FV` is not evaluating the expression.

It is a **structural analysis function** on syntax.

You can define it recursively:

$$
FV(x)={x}
$$

$$
FV(tu)=FV(t)\cup FV(u)
$$

$$
FV(\lambda x.t)=FV(t)\setminus{x}.
$$

Why?

Because `FV` asks:

> Which variable names does this term depend on from outside itself?

Take:

$$
\lambda x.(x+y).
$$

First inspect the body:

$$
FV(x+y)={x,y}.
$$

Now the abstraction says:

> The $x$ inside this body is supplied by the λ-binder.

Therefore it is no longer an external dependency:

$$
FV(\lambda x.(x+y))=
{x,y}\setminus{x}{y}.
$$

The result is:

$$
\boxed{
FV(\lambda x.(x+y))={y}
}
$$

The function depends externally on $y$, but not on $x$.

For example:

$$
(\lambda x.(x+y)),5
$$

can evaluate to:

$$
5+y.
$$

The value of $x$ comes from the function application.

The value of $y$ must come from outside.

So:

$$
\lambda x.(x+y)
$$

is a function of $x$ but has a free dependency on $y$.

---

# 3. A better analogy: local variables versus external variables

Consider a programming function:

```python
y = 10

def f(x):
    return x + y
```

The variable `x` is local to `f`.

The variable `y` is external.

The λ-calculus representation is:

$$
\lambda x.(x+y).
$$

Then:

$$
FV(\lambda x.(x+y))={y}.
$$

So the equation:

$$
FV(\lambda x.t)=FV(t)\setminus{x}
$$

means:

> The body may mention $x$, but after wrapping the body in `λx`, those occurrences are now supplied internally by the function abstraction.

This is a **scope transformation**, not a value computation.

---

# 4. The difference between a variable occurrence and a variable name

This is subtle but important.

Consider:

$$
\lambda x.(x+x).
$$

The body has two occurrences of the name $x$:

$$
x+x.
$$

Both are bound by the same λ-binder.

You can represent the syntax more precisely as:

$$
\lambda x.
\big(
\operatorname{add}(x,x)
\big).
$$

The binder induces a relation:

$$
\operatorname{binder}(\lambda x)
\longrightarrow
{\text{both occurrences of }x}.
$$

The free-variable function then removes all occurrences governed by that binder from the external dependency set.

So:

$$
FV(\lambda x.(x+x))=\varnothing.
$$

---

# 5. What does $\Delta$ mean in the substitution lemma?

The substitution lemma is usually stated with contexts.

For example:

$$
\Gamma,x:A\vdash t:B
$$

means:

> Under the assumptions in $\Gamma$, and with an additional variable $x$ of type $A$, the term $t$ has type $B$.

For example:

$$
\Gamma = y:B,\ z:C
$$

and:

$$
y:B,\ z:C,\ x:A\vdash t:D.
$$

Now suppose we have:

$$
\Delta\vdash u:A.
$$

Then we can replace $x$ by $u$:

$$
\Gamma,\Delta\vdash t[x:=u]:B.
$$

So what is $\Delta$?

It is the context containing the external variables needed to construct the replacement term $u$.

For example:

$$
u = f(a,b).
$$

Then perhaps:

$$
\Delta=(a:P,b:Q,f:P\times Q\to A).
$$

So:

$$
\Delta\vdash f(a,b):A.
$$

Now suppose:

$$
\Gamma,x:A\vdash t:B.
$$

Substituting $f(a,b)$ for $x$ gives:

$$
t[x:=f(a,b)].
$$

The resulting term depends on both:

* the original external variables in $\Gamma$,
* the external variables needed to construct $u$, namely $\Delta$.

Hence:

$$
\Gamma,\Delta\vdash t[x:=u]:B.
$$

The key meaning is:

$$
\boxed{
\Delta
=

\text{the context required to construct the substitution term}
}
$$

---

# 6. Example of the substitution lemma

Let:

$$
\Gamma=(y:B).
$$

Suppose:

$$
y:B,x:A\vdash x+y:C.
$$

Now suppose:

$$
\Delta=(z:D)
$$

and:

$$
z:D\vdash u:A.
$$

Then:

$$
y:B,z:D
\vdash
u+y:C.
$$

This is exactly:

$$
(x+y)[x:=u]=u+y.
$$

The substitution lemma says:

$$
\boxed{
\text{a well-typed term remains well-typed after substituting a well-typed term}
}
$$

This is one of the foundations of type safety.

---

# 7. Your DAG intuition is very good

Now to your main question.

Suppose you have:

$$
x:=f(y,z)
$$

$$
y:=g(a)
$$

$$
z:=h(b,c)
$$

Then the dependency structure is:

$$
x
\leftarrow
{y,z}
$$

$$
y
\leftarrow
{a}
$$

$$
z
\leftarrow
{b,c}.
$$

The dependency graph is a DAG if there are no cyclic definitions.

Then you can resolve it in topological order:

$$
a,b,c
\longrightarrow
y,z
\longrightarrow
x.
$$

This is exactly the same fundamental structure as:

* spreadsheet evaluation,
* build systems,
* compiler dependency resolution,
* dataflow graphs,
* query plans,
* symbolic expression evaluation,
* SSA construction,
* lazy computation graphs.

But there is an important distinction:

> **β-substitution itself is not generally a monoid action on the graph.**

The closest precise structure is a **monoid/category of substitutions acting on terms**.

---

# 8. Substitutions form a composition structure

Let a simultaneous substitution be:

$$
\sigma=
[x_1:=u_1,\ldots,x_n:=u_n].
$$

It acts on a term:

$$
t[\sigma].
$$

For example:

$$
\sigma=[x:=f(y),y:=a].
$$

Then:

$$
x[\sigma]=f(y)
$$

and:

$$
y[\sigma]=a.
$$

Substitutions compose.

Suppose:

$$
\sigma=[x:=f(y)]
$$

and:

$$
\tau=[y:=a].
$$

Then:

$$
\sigma;\tau
=

[x:=f(a),y:=a].
$$

The key law is:

$$
\boxed{
t[\sigma][\tau]
=

t[\sigma;\tau]
}
$$

up to the precise convention for composition order.

This is an action-like law.

There is:

* a set of terms,
* a set of substitutions,
* substitution composition,
* an action of substitutions on terms.

But because substitutions change the context, the most precise structure is generally **not one single monoid**.

It is better represented by a **category**.

---

# 9. Why a category rather than a single monoid?

Suppose:

$$
\Gamma=(x:A,y:B)
$$

and:

$$
\Delta=(a:C,b:D).
$$

A substitution:

$$
\sigma:\Delta\to\Gamma
$$

is a tuple of terms:

$$
\sigma=
(x:=u(a,b),y:=v(a,b)).
$$

This means:

$$
\Delta
\longrightarrow
\Gamma.
$$

Another substitution:

$$
\tau:\Theta\to\Delta.
$$

Then they compose:

$$
\Theta
\overset{\tau}{\longrightarrow}
\Delta
\overset{\sigma}{\longrightarrow}
\Gamma.
$$

The result is:

$$
\sigma\circ\tau:\Theta\to\Gamma.
$$

So contexts are objects and substitutions are morphisms.

This is naturally a category:

$$
\boxed{
\text{Contexts}
\quad+\quad
\text{Substitutions}
\quad=\quad
\text{category of contexts}
}
$$

A monoid appears only when you restrict to substitutions:

$$
\Gamma\to\Gamma.
$$

Those are endomorphisms of one context:

$$
\operatorname{End}(\Gamma).
$$

Then:

$$
\operatorname{End}(\Gamma)
$$

is a monoid under substitution composition.

So the precise statement is:

$$
\boxed{
\text{all substitutions}
\to
\text{category}
}
$$

while:

$$
\boxed{
\text{substitutions from one context to itself}
\to
\text{monoid}
}
$$

---

# 10. Is β-reduction a monoid action?

There are two possible interpretations.

## Interpretation A: substitution action

Yes, structurally.

Substitutions act on terms:

$$
(t,\sigma)
\longmapsto
t[\sigma].
$$

Composition satisfies:

$$
t[\sigma][\tau]
=

t[\sigma\circ\tau].
$$

So the substitution monoid of an appropriate fixed context acts on terms.

This is genuinely an action-like structure.

## Interpretation B: β-reduction itself

Not exactly.

β-reduction is a rewrite relation:

$$
(\lambda x.t)u
\to_\beta
t[x:=u].
$$

It is not generally:

$$
m\cdot t
$$

for a fixed monoid $M$.

Why?

Because β-reduction is:

* context-sensitive,
* partially applicable,
* changing syntax shape,
* potentially nondeterministic because multiple redexes can exist.

For example:

$$
(\lambda x.x)((\lambda y.y)z)
$$

has multiple possible reduction positions.

You can reduce the outer redex:

$$
(\lambda x.x)((\lambda y.y)z)
\to
(\lambda y.y)z
\to z.
$$

Or the inner redex first:

$$
(\lambda x.x)((\lambda y.y)z)
\to
(\lambda x.x)z
\to z.
$$

The reduction relation is therefore better understood as a **rewrite system**.

---

# 11. But your Kahn's algorithm idea applies to a restricted class

Suppose the term is a closed acyclic let-expression:

$$
\operatorname{let}\ x=f(a)
\operatorname{\ in}
\operatorname{let}\ y=g(x)
\operatorname{\ in}
h(y).
$$

The dependency graph is:

$$
a\to x\to y\to h(y).
$$

Then you can resolve definitions in topological order.

This is essentially:

$$
\text{dependency analysis}
\to
\text{topological evaluation}.
$$

The β-reduction analogue is:

$$
(\lambda x.t)u
\to
t[x:=u].
$$

If $u$ is itself a DAG expression, substitution can be represented by graph sharing rather than textual copying.

For example:

$$
(\lambda x.x+x)(f(a,b))
$$

can be represented as:

$$
x
\longrightarrow
f(a,b)
$$

and then both occurrences of $x$ point to the same node:

$$
f(a,b)
\longrightarrow
\text{first use of }x
$$

and:

$$
f(a,b)
\longrightarrow
\text{second use of }x.
$$

This is why real compilers do not literally copy strings for substitution.

They use:

* ASTs,
* environments,
* closures,
* DAGs,
* SSA,
* graph rewriting.

---

# 12. The key distinction: substitution versus dependency resolution

Your intuition can be formalized as follows.

A substitution:

$$
\sigma:X\to T
$$

assigns terms to variables.

The dependency relation is:

$$
x\prec_\sigma y
\iff
x\in FV(\sigma(y)).
$$

For example:

$$
\sigma(x)=f(y,z)
$$

gives:

$$
y\prec_\sigma x
$$

and:

$$
z\prec_\sigma x.
$$

The transitive closure:

$$
\prec_\sigma^\ast
$$

captures indirect dependencies.

If:

$$
\prec_\sigma^\ast
$$

is irreflexive, the dependency graph is acyclic.

Then a topological order exists.

So:

$$
\boxed{
\text{acyclic substitution environment}
\Longrightarrow
\text{topological evaluation order}
}
$$

Kahn's algorithm is therefore a valid evaluation strategy for the **dependency graph induced by a substitution environment**.

---

# 13. But λ-calculus allows cycles

Consider:

$$
\lambda x.x,x.
$$

This is syntactically finite but self-application can generate recursive structures.

More dramatically:

$$
\Omega
=

(\lambda x.xx)(\lambda x.xx).
$$

Then:

$$
\Omega
\to_\beta
\Omega.
$$

This is a cycle in the reduction graph.

So the term's **syntax DAG** and its **reduction dependency graph** are different objects.

This is a crucial distinction:

$$
\boxed{
\text{syntax graph}
\neq
\text{reduction graph}
}
$$

A term can have finite acyclic syntax but infinite reduction behavior.

---

# 14. The most useful synthesis

I would frame the entire system like this:

## Level 1: Syntax

Terms form an inductively generated algebra:

$$
t::=
x
\mid
t,t
\mid
\lambda x.t.
$$

## Level 2: Binding

The λ-constructor creates a scope:

$$
\lambda x.t.
$$

The binder removes $x$ from the external dependency set:

$$
FV(\lambda x.t)=FV(t)\setminus{x}.
$$

## Level 3: Substitution

A substitution is an environment:

$$
\sigma:
X\to T.
$$

It acts on terms:

$$
t\mapsto t[\sigma].
$$

## Level 4: Composition

Substitutions compose:

$$
t[\sigma][\tau] = 

t[\sigma\circ\tau].
$$

This gives a category of contexts and substitutions.

## Level 5: β-reduction

λ-abstraction and application interact:

$$
(\lambda x.t)u
\to
t[x:=u].
$$

This is a rewrite relation induced by substitution.

## Level 6: DAG evaluation

If the induced variable dependency graph is acyclic, evaluation can be topologically ordered.

Thus:

$$
\boxed{
\text{binding}
\to
\text{free-variable dependency}
\to
\text{substitution}
\to
\text{dependency graph}
\to
\text{topological resolution}
}
$$

---

# The deepest correction to your original intuition

You said:

> "FV is like an eval equation."

I would refine that to:

$$
\boxed{
FV
\text{ is a static dependency-analysis homomorphism}
}
$$

It maps syntax to a set of external variable dependencies:

$$
FV:T\to\mathcal P(V).
$$

Its recursive equations are:

$$
FV(x)={x}
$$

$$
FV(tu)=FV(t)\cup FV(u)
$$

$$
FV(\lambda x.t)=FV(t)\setminus{x}.
$$

So $FV$ is structurally analogous to a compiler analysis such as:

* free-variable analysis,
* liveness analysis,
* dependency analysis,
* dataflow analysis.

It does **not** execute the term.

And your DAG intuition is then:

$$
\boxed{
FV
\text{ gives the dependency edges}
}
$$

while:

$$
\boxed{
\text{substitution}
\text{ propagates definitions along those edges}
}
$$

and:

$$
\boxed{
\text{Kahn's algorithm}
\text{ resolves them when the dependency relation is acyclic}.
}
$$

The caveat is that arbitrary β-reduction is a **graph-rewriting system**, not merely a topological sort, because λ-calculus can create new dependencies and cycles dynamically.
