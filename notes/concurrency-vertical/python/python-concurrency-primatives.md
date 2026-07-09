The underlying intent is **synchronization primitive taxonomy**: understanding what synchronization objects exist, what invariant each maintains, and when to compose them.

The biggest compression is to think of every threading primitive as controlling one of only four things:

| Primitive              | Controls          | Mathematical view           | Typical question          |
| ---------------------- | ----------------- | --------------------------- | ------------------------- |
| **Lock / Mutex**       | Ownership         | Resource algebra            | "Who may mutate?"         |
| **Condition Variable** | State transitions | Predicate over shared state | "When may I continue?"    |
| **Event**              | Notification      | Boolean latch               | "Has something happened?" |
| **Semaphore**          | Capacity          | Counting monoid             | "How many may enter?"     |

---

# 1. Lock / Mutex

Purpose:

* Mutual exclusion
* Protects invariants

Invariant

$$
|\text{owners}| \le 1
$$

Think

```
Resource
   ^
 Mutex
```

Without one

```
Thread A
Thread B
      |
      v
same memory
```

With one

```
Thread A
   |
 Mutex
   |
Resource
```

Python

```python
lock = threading.Lock()

with lock:
    balance += 1
```

Use whenever

* writing shared memory
* maintaining data structure invariants
* atomic updates

---

# 2. RLock (Recursive Lock)

Same as mutex except

Owner may acquire multiple times.

Useful when

```
A()
  calls
    B()

Both lock
```

instead of deadlocking.

---

# 3. Condition Variable

A condition variable **does not protect memory**.

Instead it lets threads sleep until

> some predicate becomes true.

Mathematically

Shared state

$$
S
$$

Predicate

$$
P(S)
$$

Thread waits until

$$
P(S)=\text{true}
$$

Python

```python
condition = threading.Condition()

with condition:
    while not ready:
        condition.wait()

    consume()
```

Producer

```python
with condition:
    ready = True
    condition.notify()
```

Notice

Condition variable always accompanies a mutex.

Reason

Need atomic

```
check predicate
sleep
wake
```

---

# 4. Event

Event is much simpler.

State

```
False
```

or

```
True
```

Once true

everyone waiting wakes.

Python

```python
event = threading.Event()

event.wait()

event.set()
```

Think

```
startup complete

download finished

shutdown requested
```

Not

```
queue length > 0
```

because queue length changes repeatedly.

---

Difference

Condition

```
Wait until

queue not empty
```

Event

```
Wait until

startup complete
```

---

# 5. Semaphore

Controls

Capacity

Instead of one owner

Allow

N owners.

Invariant

$$
count \ge 0
$$

Acquire

```
count--
```

Release

```
count++
```

Python

```python
sem = threading.Semaphore(4)

with sem:
    work()
```

Example

Only

```
4 downloads
```

at once.

---

# 6. BoundedSemaphore

Like semaphore

except

```
release()
```

cannot exceed initial capacity.

Useful for bug detection.

---

# 7. Barrier

Synchronize phases.

```
A
 \
  \
   Barrier
  /
 /
B
```

Nobody proceeds until everyone arrives.

Python

```python
barrier.wait()
```

Example

Parallel matrix multiplication

Phase 1

↓

Barrier

↓

Phase 2

---

# 8. Queue

A synchronized data structure.

Contains

* mutex
* conditions
* buffer

Already solved for you.

```python
q.put(x)

q.get()
```

Instead of manually writing

```
mutex

condition

list
```

---

# 9. Future

Represents

"A value that will exist."

State machine

```
Pending

Running

Finished

Failed

Cancelled
```

Thread

```
future.result()
```

blocks until complete.

---

# 10. Thread

Execution context

Contains

* stack
* registers
* instruction pointer

Not synchronization itself.

---

# 11. Thread Pool

Instead of

```
1000 threads
```

Use

```
8 workers
```

Tasks submitted.

Workers execute.

---

# 12. Atomic Variables

Python lacks true general atomic integers in the standard library (the GIL does not make compound operations atomic across threads). In lower-level languages such as C++ or Rust you have atomic types.

Example in Rust

```rust
AtomicBool

AtomicUsize
```

Operations

```
load

store

swap

compare_exchange

fetch_add
```

These avoid locking for simple shared state.

---

# 13. Local Storage

Each thread gets private memory.

Python

```python
threading.local()
```

Equivalent to

```
Thread A

counter

Thread B

counter
```

No synchronization needed.

---

# 14. Timer

Runs function later.

```python
threading.Timer(...)
```

Really just

```
sleep

then

spawn
```

---

# Compression by abstraction level

| Level       | Primitive    | Protects                 |
| ----------- | ------------ | ------------------------ |
| Memory      | Mutex        | Exclusive ownership      |
| Memory      | RLock        | Recursive ownership      |
| Memory      | Atomic       | Single variable          |
| State       | Condition    | Predicate satisfaction   |
| Signal      | Event        | Boolean notification     |
| Capacity    | Semaphore    | Available permits        |
| Phase       | Barrier      | Global synchronization   |
| Data        | Queue        | Ordered communication    |
| Computation | Future       | Deferred result          |
| Execution   | Thread       | Independent control flow |
| Scheduling  | ThreadPool   | Worker management        |
| Isolation   | Thread Local | Per-thread state         |

---

## Decision tree

```
Need exclusive mutation?
    Mutex

Need to wait for state?
    Condition

Need one-shot signal?
    Event

Need N concurrent users?
    Semaphore

Need everyone to meet?
    Barrier

Need producer-consumer?
    Queue

Need async result?
    Future

Need one atomic integer/bool?
    Atomic

Need thread-private state?
    Thread Local
```

---

## Category-theoretic compression

Treat each primitive as enforcing a different **algebra on concurrent state**:

* **Mutex**: endows a resource with an **exclusive ownership** discipline (a partial monoid where ownership cannot be duplicated).
* **Semaphore**: a **counting resource algebra** over natural numbers, with acquire/release as inverse updates within capacity constraints.
* **Condition Variable**: a morphism from a shared state space (S) to a predicate (P : S \to {\text{true}, \text{false}}), suspending computation until the predicate holds.
* **Event**: a monotonic state machine ( {\text{unset} \rightarrow \text{set}} ), acting as a one-way notification latch.
* **Barrier**: a synchronization morphism that composes multiple execution paths into a single phase transition.
* **Queue**: a communication channel (a morphism between producer and consumer processes) that encapsulates synchronization internally.
* **Future**: a deferred computation modeled as a state machine from `Pending` to a terminal state (`Completed`, `Failed`, or `Cancelled`), exposing a single eventual value.
