You are asking for the **formal model of Two-Phase Locking (2PL)**: a protocol that constrains *when locks may be acquired and released* to guarantee serializability.

The core idea:

A transaction's lock operations must have a phase structure:

$$
\textbf{Growing phase} \rightarrow \textbf{Shrinking phase}
$$

where:

* Growing: acquire locks, never release
* Shrinking: release locks, never acquire

---

## 1. Transaction model

Let a transaction be a sequence of operations:

$$
T_i =
(o_1,o_2,\dots,o_n)
$$

where operations are:

* reads:

$$
r_i(X)
$$

* writes:

$$
w_i(X)
$$

* lock:

$$
L_i(X)
$$

* unlock:

$$
U_i(X)
$$

Example:

$$
T_1 =
L_1(A),
r_1(A),
w_1(A),
U_1(A)
$$

---

# 2. Lock state

For each resource (X):

$$
Lock(X) \in {Free, Shared, Exclusive}
$$

or:

$$
Lock(X): Resource \rightarrow Owner
$$

Examples:

Shared:

$$
S(X)={T_1,T_2,T_3}
$$

Exclusive:

$$
X(X)=T_1
$$

---

# 3. Compatibility relation

Locks form a compatibility algebra:

$$
C : Mode \times Mode \rightarrow {true,false}
$$

Table:

|   | S | X |
| - | - | - |
| S | ✓ | ✗ |
| X | ✗ | ✗ |

Meaning:

$$
C(S,S)=true
$$

but:

$$
C(S,X)=false
$$

---

# 4. Two phases

Define:

$$
phase(T_i,t)
$$

where:

$$
phase \in {Growing,Shrinking}
$$

A transaction begins:

$$
phase(T_i,0)=Growing
$$

---

## Growing phase invariant

During growing:

$$
L_i(X) \text{ allowed}
$$

but:

$$
U_i(X) \text{ forbidden}
$$

Formally:

For every operation sequence:

$$
L_i(X)
$$

can increase the lock set:

$$
Locks(T_i,t+1)
==============

Locks(T_i,t)\cup X
$$

but:

$$
|Locks(T_i,t+1)| \geq |Locks(T_i,t)|
$$

---

## Shrinking phase invariant

Once the first unlock happens:

$$
\exists t: U_i(X)
$$

the transaction enters shrinking:

$$
phase(T_i,t)=Shrinking
$$

Then:

$$
U_i(X)
$$

allowed.

But:

$$
L_i(X)
$$

is forbidden.

The lock set monotonically decreases:

$$
Locks(T_i,t+1)
\subseteq Locks(T_i,t)
$$

---

# 5. Lock point

The key concept:

## Lock point

The moment a transaction acquires its final lock.

Define:

$$
LP(T_i)=max(t | L_i(X))
$$

The ordering of lock points defines serialization order.

Example:

```
T1:

lock A
lock B   <-- lock point
read/write
unlock A
unlock B


T2:

lock C
lock D   <-- lock point
read/write
unlock
```

If:

$$
LP(T_1)<LP(T_2)
$$

then:

$$
T_1 \prec T_2
$$

in the equivalent serial execution.

---

# 6. Why does 2PL work?

The problem:

Concurrent transactions can create cycles.

Example:

```
T1:
lock A
        wait B


T2:
lock B
        wait A
```

Dependency graph:

$$
T_1 \rightarrow T_2
$$

and:

$$
T_2 \rightarrow T_1
$$

Cycle:

$$
T_1 \rightarrow T_2 \rightarrow T_1
$$

2PL prevents this by ensuring every transaction has one monotonic lock acquisition frontier.

---

# 7. Serialization graph view

Transactions form a directed graph:

$$
G=(V,E)
$$

Vertices:

$$
V={T_1,T_2,...,T_n}
$$

Edges:

$$
T_i\rightarrow T_j
$$

if:

* (T_i) writes/reads a resource
* (T_j) conflicts later

2PL guarantees:

$$
G \text{ is acyclic}
$$

therefore:

$$
\exists \text{ topological ordering}
$$

therefore:

$$
schedule \equiv serial\ execution
$$

---

# 8. Strict 2PL

Databases usually use **Strict Two-Phase Locking**.

Additional invariant:

Hold all exclusive locks until commit.

Instead of:

$$
Unlock(X)
$$

during shrinking:

wait:

$$
commit(T_i)
\rightarrow
unlock(X)
$$

This prevents:

* dirty reads
* cascading aborts

---

# 9. Connection to concurrency primitives

The analogy:

| Database          | Threading                 |
| ----------------- | ------------------------- |
| Transaction       | Thread/task               |
| Row/page          | Shared resource           |
| Lock manager      | Mutex system              |
| Deadlock detector | Thread deadlock detection |
| Commit            | Successful completion     |
| Rollback          | Undo state                |

Dining philosophers:

* Fork = database row
* Philosopher = transaction
* Taking forks = acquiring locks
* Eating = critical section
* Putting forks down = release

The philosopher problem is basically a tiny distributed lock manager problem.

---

# 10. Category-theoretic compression

A transaction can be viewed as a morphism:

$$
T:S \rightarrow S'
$$

where (S) is system state.

Locks constrain composition:

Without locking:

$$
T_1 \circ T_2
$$

may not equal:

$$
T_2 \circ T_1
$$

2PL imposes a partial order:

$$
T_1 \prec T_2
$$

such that all valid compositions are equivalent to a serial composition:

$$
T_n\circ...\circ T_2\circ T_1
$$

So the essence of 2PL is:

> restrict the allowed interleavings of state-transforming morphisms so that concurrent execution collapses into an equivalent ordered composition.
