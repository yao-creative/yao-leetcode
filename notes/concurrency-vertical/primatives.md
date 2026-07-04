Concurrency primitives are the **minimal synchronization and coordination mechanisms** from which higher-level concurrency patterns are built.

A useful classification is by the coordination problem they solve:

| Problem                   | Primitive          |
| ------------------------- | ------------------ |
| Mutual exclusion          | Lock, Mutex        |
| Shared read access        | Reader-Writer Lock |
| Bounded resource access   | Semaphore          |
| State change notification | Condition Variable |
| One-shot signaling        | Event              |
| Phase synchronization     | Barrier            |
| Ownership transfer        | Queue, Channel     |
| Deferred results          | Future, Promise    |
| Atomic update             | Atomic Variables   |
| Cancellation              | Cancellation Token |
| Time coordination         | Timer              |
| Task execution            | Executor           |
| Structured lifetime       | Task Group         |
| Distributed coordination  | Distributed Lock   |

---

# 1. Mutex / Lock

Guarantees:

$$
|\text{critical section occupants}| \leq 1
$$

Python:

```python
lock = threading.Lock()

with lock:
    critical_section()
```

Use for:

* Shared dictionaries
* Shared counters
* Cache mutation

---

# 2. Recursive Lock (RLock)

Allows same thread to reacquire lock.

```python
lock = threading.RLock()
```

Internal state:

$$
(owner, count)
$$

Useful for:

* Recursive algorithms
* Nested monitor methods

---

# 3. Semaphore

Generalized mutex.

$$
0 \leq holders \leq N
$$

```python
sem = threading.Semaphore(5)

with sem:
    use_resource()
```

Examples:

* Database connection pools
* Rate limiting
* API quotas

---

# 4. Bounded Semaphore

Detects over-release bugs.

```python
sem = threading.BoundedSemaphore(5)
```

Invariant:

$$
count \leq initial_count
$$

---

# 5. Reader-Writer Lock

Allows:

* many readers
* one writer

State:

$$
(readers, writer)
$$

Example libraries:

* `readerwriterlock`
* `fasteners`

Useful for:

* Configuration stores
* In-memory caches

---

# 6. Condition Variable

Wait for predicate.

Formalization:

$$
wait(P)
$$

where

$$
P(S)=true
$$

```python
with condition:
    while not ready:
        condition.wait()
```

Producer:

```python
with condition:
    ready = True
    condition.notify()
```

---

# 7. Event

Boolean synchronization primitive.

State:

$$
event \in {0,1}
$$

```python
event = threading.Event()

event.wait()
event.set()
```

Common uses:

* shutdown signals
* startup coordination

---

# 8. Barrier

Wait until all participants arrive.

$$
count = N
$$

Release only when:

$$
arrived = N
$$

```python
barrier = threading.Barrier(8)

barrier.wait()
```

Useful for:

* simulations
* parallel algorithms

---

# 9. Queue

Message passing primitive.

```python
queue = Queue()

queue.put(item)
item = queue.get()
```

Provides:

* synchronization
* buffering
* ownership transfer

Mathematically:

$$
Producer \to Queue \to Consumer
$$

---

# 10. Priority Queue

Ordered queue.

```python
from queue import PriorityQueue

pq.put((priority, task))
```

Useful for:

* schedulers
* task dispatchers

---

# 11. LIFO Queue

Stack semantics.

```python
from queue import LifoQueue
```

Useful for:

* work stealing
* DFS scheduling

---

# 12. SimpleQueue

Lower overhead FIFO.

```python
from queue import SimpleQueue
```

---

# 13. Pipe

Point-to-point communication.

```python
parent, child = multiprocessing.Pipe()
```

Topology:

$$
A \leftrightarrow B
$$

---

# 14. Channel

Generalized queue abstraction.

Examples:

* `asyncio.Queue`
* `janus`
* `trio.MemorySendChannel`

CSP model:

$$
send(x)
$$

$$
receive() \rightarrow x
$$

---

# 15. Future

Represents a result that may exist later.

State machine:

```text
PENDING
 ↓
RUNNING
 ↓
FINISHED
```

```python
future.result()
```

---

# 16. Promise

Writable side of future.

```text
Promise ---> Future
```

Producer owns promise.

Consumer owns future.

Languages like Rust and Java expose this more explicitly than Python.

---

# 17. Executor

Task scheduling abstraction.

```python
executor.submit(fn)
```

Types:

* ThreadPoolExecutor
* ProcessPoolExecutor

---

# 18. Thread Pool

Pool of worker threads.

Topology:

```text
tasks
  ↓
queue
  ↓
workers
```

---

# 19. Process Pool

Equivalent abstraction for processes.

Useful for CPU workloads.

---

# 20. Atomic Variables

Operations are indivisible.

Examples:

```text
atomic_increment()
compare_exchange()
fetch_add()
swap()
```

Python lacks a standard atomic module.

Libraries:

* `atomics`
* `shared_memory`

---

# 21. Compare-And-Swap (CAS)

Fundamental lock-free primitive.

$$
CAS(address, old, new)
$$

Algorithm:

```text
if *address == old:
    *address = new
```

Used in:

* lock-free queues
* wait-free algorithms

---

# 22. Fetch Add

Atomic increment.

$$
x := x + 1
$$

without race conditions.

---

# 23. Spinlock

Busy waits instead of sleeping.

```text
while locked:
    pass
```

Good for:

* extremely short critical sections

Bad for Python due to GIL.

---

# 24. Monitor

Combination:

$$
Monitor = Lock + State + Condition
$$

Classic object-oriented synchronization model.

---

# 25. Latch

One-time barrier.

```text
countdown = N

wait until countdown = 0
```

Java provides `CountDownLatch`.

Python usually emulates with `Event`.

---

# 26. Countdown Event

Equivalent of latch.

```python
remaining -= 1

if remaining == 0:
    done.set()
```

---

# 27. Cancellation Token

Cooperative cancellation.

```python
cancel_event = Event()

while not cancel_event.is_set():
    work()
```

`asyncio` uses task cancellation exceptions.

---

# 28. Timer

Delayed execution primitive.

```python
timer = threading.Timer(5, fn)
timer.start()
```

---

# 29. Task Group

Structured concurrency primitive.

Examples:

```python
asyncio.TaskGroup()
```

Guarantees:

* child completion
* exception propagation
* cleanup

---

# 30. Context Variable

Task-local storage.

```python
import contextvars

request_id = contextvars.ContextVar("request_id")
```

Equivalent to:

$$
Task \to State
$$

---

# 31. Thread Local Storage

Per-thread state.

```python
local = threading.local()
```

Useful for:

* database sessions
* request context

---

# 32. Shared Memory

```python
from multiprocessing.shared_memory import SharedMemory
```

Provides:

$$
P_i \leftrightarrow M \leftrightarrow P_j
$$

---

# 33. Distributed Lock

Cross-machine mutex.

Libraries:

* [Redis distributed locks documentation](https://redis.io/docs/latest/develop/use/patterns/distributed-locks/?utm_source=chatgpt.com)
* [etcd locks](https://etcd.io/docs/v3.5/dev-guide/api_concurrency_reference_v3/?utm_source=chatgpt.com)

---

# 34. Rate Limiter

Concurrency-control primitive over time.

Examples:

* token bucket
* leaky bucket
* sliding window

Constraint:

$$
requests(t,t+\Delta t) \leq N
$$

---

# 35. Backpressure

Consumer controls producer speed.

Mechanisms:

* bounded queues
* TCP windowing
* reactive streams

---

# 36. Work Stealing Queue

Workers steal work from each other.

Used by:

* Rayon
* ForkJoinPool
* Dask scheduler

---

# 37. Epoch Reclamation

Lock-free memory reclamation strategy.

Used by:

* lock-free hash maps
* concurrent trees

---

# 38. Hazard Pointers

Safe reclamation mechanism for lock-free structures.

---

# 39. STM (Software Transactional Memory)

Transactions instead of locks.

$$
transaction : S \rightarrow S'
$$

Examples outside Python:

* Haskell STM
* Clojure refs

---

# Core Primitive Hierarchy

```text
Atomic Operations
│
├── CAS
├── FetchAdd
└── Atomic Load/Store

Synchronization
│
├── Lock
├── RLock
├── Semaphore
├── RWLock
├── Spinlock
└── Barrier

Coordination
│
├── Condition
├── Event
├── Latch
├── Cancellation Token
└── Timer

Communication
│
├── Queue
├── Channel
├── Pipe
└── Shared Memory

Scheduling
│
├── Future
├── Promise
├── Executor
├── Thread Pool
├── Process Pool
└── Task Group
```

Most real-world systems such as $$FastAPI$$(https://fastapi.tiangolo.com?utm_source=chatgpt.com), $$Celery$$(https://docs.celeryq.dev?utm_source=chatgpt.com), and $$Ray$$(https://ray.io?utm_source=chatgpt.com) are ultimately compositions of only a handful of these primitives:

$$
\boxed{
Lock,\ Queue,\ Event,\ Semaphore,\ Future
}
$$

with nearly every higher-level concurrency abstraction reducible to combinations of those five.
