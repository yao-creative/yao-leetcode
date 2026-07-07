This is a question about the **memory model** of concurrent computation. More precisely, `Ordering` specifies **constraints on the visibility and ordering of memory operations across threads**, not the atomic operation itself. The atomicity (e.g., `compare_exchange`) guarantees indivisibility; the `Ordering` determines what other reads and writes are allowed to move around it.

A useful way to think about it is that there are two partial orders:

* **Program order**: the order written in your source code.
* **Visibility (happens-before) order**: when writes performed by one thread become observable to another.

The memory ordering determines how much of the program order must also be respected in the visibility order.

---

# The formal model

Let

* (M) be the set of memory operations.
* (P \subseteq M \times M) be program order.
* (H \subseteq M \times M) be the happens-before relation.

Memory orderings add constraints such as

$$
P \subseteq H
$$

for some operations.

A stronger ordering preserves more of (P); a weaker ordering allows more reordering.

---

# Why CPUs reorder

Suppose you write

```rust
x = 1;
flag.store(true);
```

A compiler or CPU may legally execute

```text
flag = true
x = 1
```

if nothing forbids it.

This is called **instruction reordering**.

The different `Ordering` variants tell the compiler and CPU which reorderings are forbidden.

---

# Relaxed

```rust
Ordering::Relaxed
```

Guarantees:

* the operation is atomic

Does **not** guarantee:

* visibility
* ordering
* synchronization

Think:

> "Only don't tear the value apart."

Example

```rust
counter.fetch_add(1, Ordering::Relaxed);
```

Perfect for statistics.

You don't care exactly when another thread sees the increment.

---

Formally

Let

$$
m \in M
$$

be the atomic operation.

Relaxed only guarantees

$$
m
$$

is indivisible.

It adds almost no ordering edges to (H).

---

# Acquire

```rust
Ordering::Acquire
```

Acquire is used on

* loads
* successful CAS

Meaning

> "After I read this value, everything after this cannot move before it."

Example

```rust
flag.load(Ordering::Acquire);

read(data);
```

Compiler may NOT transform into

```text
read(data)

flag.load()
```

because that would read data before synchronization occurred.

---

Formally

Acquire establishes

$$
Acquire
\rightarrow
\text{future reads/writes}
$$

All later operations remain after the acquire.

---

# Release

(Not asked, but needed.)

```rust
Ordering::Release
```

Used on

```rust
store()
```

Meaning

> Everything before the store must become visible before the store itself.

Example

```rust
data = 42;

flag.store(true, Ordering::Release);
```

Cannot become

```text
flag = true

data = 42
```

---

Formally

Release enforces

$$
\text{previous writes}
\rightarrow
Release
$$

---

# Acquire + Release

```rust
Ordering::AcqRel
```

Used for operations that both

* read
* write

Example

```rust
compare_exchange()
fetch_add()
swap()
```

The operation simultaneously

* acquires previous writes
* releases future writes

Think

```text
before
    ↓
 AcqRel operation
    ↓
after
```

Nothing crosses it.

---

Formally

If

$$
m
$$

is the atomic operation,

then

$$
\text{Before}(m)
<
m
<
\text{After}(m)
$$

No reordering across (m).

---

# Sequential Consistency

```rust
Ordering::SeqCst
```

Strongest ordering.

It says

> Pretend every atomic operation happened in one single global timeline.

Imagine four threads.

Without SeqCst

```text
CPU 1
A B C

CPU 2
B A C
```

Different processors can observe different orders.

With SeqCst

Everyone agrees

```text
A

↓

B

↓

C
```

There exists one global ordering.

---

Formally

Let

$$
S
$$

be all SeqCst operations.

Then

$$
S
$$

must admit one total order

$$
<
$$

such that every thread observes exactly that order.

This is much stronger than Acquire/Release.

---

# Compare them

| Ordering | Atomic | Prevents reordering | Synchronizes threads | Global order |
| -------- | ------ | ------------------- | -------------------- | ------------ |
| Relaxed  | ✅      | almost none         | ❌                    | ❌            |
| Acquire  | ✅      | later operations    | ✅ (with Release)     | ❌            |
| Release  | ✅      | earlier operations  | ✅ (with Acquire)     | ❌            |
| AcqRel   | ✅      | both directions     | ✅                    | ❌            |
| SeqCst   | ✅      | strongest           | ✅                    | ✅            |

---

# In terms of graph theory

Consider a directed graph

$$
G=(M,E)
$$

where vertices are memory operations.

Edges encode ordering constraints.

Relaxed

$$
E
=

\emptyset
$$

except atomicity.

Acquire

Adds edges

$$
Acquire
\rightarrow
\text{future operations}
$$

Release

Adds

$$
\text{previous operations}
\rightarrow
Release
$$

AcqRel

Adds both.

SeqCst

Adds enough edges that all SeqCst operations form a **total order**, rather than merely a partial order.

---

# Category-theoretic interpretation

Let the objects be machine states and morphisms be memory transitions.

A memory ordering specifies which morphisms are allowed to commute.

* **Relaxed**: many commuting squares are permitted; the compiler and CPU may rearrange independent transitions.
* **Acquire**: future morphisms cannot be moved before the acquire morphism.
* **Release**: prior morphisms cannot be moved after the release morphism.
* **AcqRel**: acts as a barrier in both directions, preserving composition across the synchronization point.
* **SeqCst**: embeds all sequentially consistent atomic transitions into a single totally ordered chain, so every thread factors its observations through the same linear execution order.

From a refinement perspective, these orderings form a lattice of increasing constraints:

```text
Relaxed
    │
Acquire     Release
     ╲     ╱
      AcqRel
         │
      SeqCst
```

Moving upward in this lattice preserves more ordering information and permits fewer compiler and hardware optimizations, but makes reasoning about concurrent programs progressively simpler.
