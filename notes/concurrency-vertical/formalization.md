Below is a **formalized concurrency-pattern layer**, written in a way that makes the *intent → state space → transition semantics → correctness condition* explicit. I'll keep the structure consistent so you can compare patterns as morphisms over shared state machines.

I'll use a common template:

* **System state:** $S$
* **Threads / agents:** $T$
* **Events / requests:** $e \in E$
* **Transition function:** $\delta$
* **Invariant / safety condition:** $I(S)$
* **Progress condition:** $P$

You can read each pattern as a different constraint on $\delta$ and $I$.

---

# 1. Active Object

### Intent

Asynchronous invocation → decouple caller from execution

### Formal model

State:

$$
S = (Q, W)
$$

* $Q$: request queue
* $W$: worker state

Transitions:

* enqueue:

$$
\delta_{enqueue}: (Q, W, e) \mapsto (Q \cup \{e\}, W)
$$

* execute:

$$
\delta_{exec}: (Q, W) \mapsto (Q', W')
\quad \text{where } e = \mathrm{head}(Q)
$$

### Invariant

$$
I(S): \text{Q is FIFO-consistent, no request executed twice}
$$

### Key property

A morphism:

$$
\text{call} \rightarrow \text{enqueue} \rightarrow \text{eventual execution}
$$

---

# 2. Balking Pattern

### Intent

Reject execution if precondition fails

State:

$$
S = (s, \mathit{flag})
$$

Transition:

$$
\delta(e):
\begin{cases}
\text{execute}(e) & \text{if } \mathit{flag} = \text{true} \\
\bot & \text{otherwise}
\end{cases}
$$

### Invariant

$$
I(S): \neg \mathit{flag} \Rightarrow \text{no state mutation from event}
$$

### Key property

Partial function:

$$
\delta: E \rightharpoonup S
$$

---

# 3. Barrier

### Intent

All threads synchronize at phase boundary

State:

$$
S = (k, N)
$$

* $k$: arrivals
* $N$: total threads

Transition:

$$
\delta_{arrive}: k \mapsto k+1
$$

Release condition:

$$
k = N \Rightarrow \text{release all}
$$

### Invariant

$$
k \le N
$$

### Liveness condition

$$
\forall i,\; \text{thread}_i \text{ eventually proceeds iff } k = N
$$

---

# 4. Double-Checked Locking

### Intent

Avoid synchronization after initialization

State:

$$
S = (\mathit{init}, \mathit{lock})
$$

Transition:

$$
\delta(e):
\begin{cases}
\text{lock} \rightarrow \text{init} & \text{if } \neg \mathit{init} \\
\text{skip lock} & \text{if } \mathit{init}
\end{cases}
$$

### Invariant

$$
\mathit{init} \Rightarrow \text{unique construction of resource}
$$

### Key property

Optimization of guarded critical section:

$$
O(n) \rightarrow O(1) \text{ after initialization}
$$

---

# 5. Guarded Suspension

### Intent

Block until condition becomes true

State:

$$
S = (c, W)
$$

Transition:

$$
\delta(e):
\begin{cases}
\text{wait} & c = \text{false} \\
\text{execute} & c = \text{true}
\end{cases}
$$

### Invariant

$$
\neg c \Rightarrow \text{thread is suspended, no progress}
$$

### Progress condition

$$
c \rightarrow \text{eventual wakeup}
$$

---

# 6. Monitor Object

### Intent

Encapsulate state + mutual exclusion

State:

$$
S = (x, L)
$$

* $x$: shared state
* $L$: lock

Transition:

$$
\delta(e): x' = f(x)
\quad \text{only if } L = \text{acquired}
$$

### Invariant

$$
\forall t_i, t_j:\; t_i \neq t_j \Rightarrow \neg (\mathit{critical}_i \land \mathit{critical}_j)
$$

### Key property

Serialized access:

$$
\text{mutual exclusion} = \text{single-writer invariant}
$$

---

# 7. "Nuclear Reaction" (Cascade Propagation Model)

### Intent

One event triggers chain reactions

State:

$$
S = (G, A)
$$

* $G$: graph dependency structure
* $A$: active nodes

Transition:

$$
\delta(v):\; \forall u \in \mathrm{adj}(v),\; \mathrm{activate}(u)
$$

### Invariant

$$
\text{No node activates before predecessor condition}
$$

### Key property

Fixpoint computation:

$$
A_{t+1} = F(A_t)
$$

---

# 8. Reactor Pattern

### Intent

Event loop demultiplexing IO

State:

$$
S = (E, H)
$$

* $E$: event queue
* $H$: handlers

Transition:

$$
\delta:\; e \mapsto H(e.\mathit{type})(e)
$$

### Invariant

$$
\forall e:\; \text{exactly one handler dispatch}
$$

### Key property

Single-threaded event homomorphism:

$$
E \rightarrow \mathrm{Handler}(E)
$$

---

# 9. Readers–Writer Lock

### Intent

Concurrent reads, exclusive writes

State:

$$
S = (r, w)
$$

* $r$: reader count
* $w \in \{0, 1\}$: writer flag

Transitions:

* read:

$$
w = 0 \Rightarrow r \mapsto r + 1
$$

* write:

$$
r = 0 \land w = 0 \Rightarrow w = 1
$$

### Invariant

$$
w = 1 \Rightarrow r = 0
$$

---

# 10. Scheduler Pattern

### Intent

Choose next runnable task

State:

$$
S = (P, \mathit{policy})
$$

* $P$: task set

Transition:

$$
\delta = \arg\max_{p \in P} \mathrm{priority}(p)
$$

### Invariant

$$
\text{fairness constraint (optional)}:\; \forall p,\; \text{starvation bounded}
$$

---

# 11. Thread Pool Pattern

### Intent

Bounded worker reuse

State:

$$
S = (Q, W_1 \ldots W_n)
$$

Transition:

$$
Q \rightarrow W_i \rightarrow Q'
$$

Invariant:

$$
|W| = n = \text{constant}
$$

### Key property

Throughput bounded system:

$$
\text{concurrency cap} = n
$$

---

# 12. Thread-Local Storage

### Intent

Isolated per-thread state

State:

$$
S = \{ s_t \mid t \in T \}
$$

Transition:

$$
\delta_t(e): s_t \mapsto f(s_t, e)
$$

### Invariant

$$
\forall t_i \neq t_j:\; s_{t_i} \cap s_{t_j} = \emptyset
$$

### Key property

No shared mutable state:

$$
\text{global state} = \bigsqcup s_t
$$

---

# Cross-pattern unification (important structural insight)

You can classify all of them as constraints on the same base machine:

### 1. Scheduling constraint

* Scheduler
* Reactor
* Thread pool

### 2. State gating constraint

* Guarded suspension
* Balking
* Readers–writer lock

### 3. Synchronization boundary constraint

* Barrier
* Active object (indirectly via queue flush)
* Nuclear reaction (cascade fixpoint)

---

If you want, the next step up in abstraction is:

* express all of these as a **monad stack (State + IO + Cont)**
* or convert them into **Petri nets (places/transitions/guards)**
* or map them into **actor model morphisms vs CSP processes**

Those give you a single unifying algebra for all concurrency patterns.
