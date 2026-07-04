Python concurrency mechanisms can be organized by the **execution model**, **scheduling semantics**, and **memory sharing model**. The first decision is usually:

1. **Is the workload CPU-bound or I/O-bound?**
2. **Do tasks need shared mutable state?**
3. **Do tasks need low latency or high throughput?**
4. **Are failures isolated or propagated?**

---

# 1. Threads

## Model

Multiple OS threads inside one process sharing the same address space.

[
\text{Thread}_i : S \rightarrow S'
]

where all threads operate on shared state (S).

## Library

```python
import threading
```

## Example

```python
import threading
import time

def worker(name):
    time.sleep(1)
    print(name)

threads = []

for i in range(5):
    t = threading.Thread(target=worker, args=(i,))
    t.start()
    threads.append(t)

for t in threads:
    t.join()
```

## Synchronization primitives

| Primitive   | Purpose                     |
| ----------- | --------------------------- |
| `Lock`      | Mutual exclusion            |
| `RLock`     | Recursive lock              |
| `Condition` | Wait for predicate          |
| `Semaphore` | Bounded access              |
| `Event`     | Signal state changes        |
| `Barrier`   | Phase synchronization       |
| `Queue`     | Thread-safe message passing |

Example:

```python
lock = threading.Lock()

with lock:
    shared_counter += 1
```

---

## Advantages

* Shared memory
* Low communication overhead
* Natural for I/O workloads

## Problems

* Race conditions
* Deadlocks
* Lock contention
* Priority inversion

---

## The GIL

The Global Interpreter Lock allows only one thread to execute Python bytecode simultaneously.

Therefore:

* CPU workloads do **not** scale well.
* I/O workloads scale very well.

---

# 2. Multiprocessing

## Model

Multiple independent processes.

[
P_i \cap P_j = \varnothing
]

Each process owns separate memory.

## Library

```python
import multiprocessing
```

## Example

```python
from multiprocessing import Process

def worker():
    print("hello")

p = Process(target=worker)

p.start()
p.join()
```

---

## IPC mechanisms

| Primitive      | Description           |
| -------------- | --------------------- |
| `Queue`        | Message passing       |
| `Pipe`         | Bidirectional channel |
| `Manager`      | Shared proxy objects  |
| `SharedMemory` | Shared arrays/objects |
| `Value`        | Shared scalar         |
| `Array`        | Shared arrays         |

---

## Advantages

* True parallelism.
* Avoids GIL.
* Fault isolation.

## Costs

* Serialization overhead.
* Process creation cost.
* IPC complexity.

---

# 3. Futures

High-level abstraction over threads or processes.

Library:

```python
concurrent.futures
```

## Thread pool

```python
from concurrent.futures import ThreadPoolExecutor

with ThreadPoolExecutor() as executor:
    futures = [executor.submit(pow, 2, i) for i in range(10)]

    for future in futures:
        print(future.result())
```

## Process pool

```python
from concurrent.futures import ProcessPoolExecutor

with ProcessPoolExecutor() as executor:
    results = executor.map(pow, [2]*10, range(10))
```

---

## Formalization

A future is essentially:

[
Future[T]
]

which represents

[
T + \text{Pending}
]

or categorically:

[
Future : \mathcal{C} \rightarrow \mathcal{C}
]

a computational context similar to a monad.

---

# 4. Async IO

## Model

Single-threaded cooperative scheduling.

Tasks voluntarily yield execution.

[
Task_i \xrightarrow{await} Scheduler
]

## Library

```python
asyncio
```

## Example

```python
import asyncio

async def worker():
    await asyncio.sleep(1)
    print("done")

asyncio.run(worker())
```

---

## Scheduler

Event loop:

```text
Ready Queue
    ↓
Execute Task
    ↓
await socket.read()
    ↓
Suspend Task
    ↓
Resume when event arrives
```

---

## Advantages

* Massive scalability.
* Extremely low memory overhead.
* No locking for most applications.

---

## Disadvantages

* Blocking calls freeze the event loop.
* CPU tasks require offloading.

---

## Common libraries

| Purpose     | Library                                                                                                                  |
| ----------- | ------------------------------------------------------------------------------------------------------------------------ |
| HTTP client | [aiohttp](https://aiohttp.readthedocs.io?utm_source=chatgpt.com)                                                         |
| HTTP server | [FastAPI](https://fastapi.tiangolo.com?utm_source=chatgpt.com)                                                           |
| Database    | [asyncpg](https://magicstack.github.io/asyncpg/current/?utm_source=chatgpt.com)                                          |
| Redis       | [redis-py asyncio support](https://redis.readthedocs.io/en/stable/examples/asyncio_examples.html?utm_source=chatgpt.com) |

---

# 5. Green Threads / Coroutines

User-space scheduled lightweight threads.

Examples:

* [gevent](https://www.gevent.org?utm_source=chatgpt.com)
* [eventlet](https://eventlet.net?utm_source=chatgpt.com)

Example:

```python
import gevent

def task():
    gevent.sleep(1)

g = gevent.spawn(task)
g.join()
```

---

## Characteristics

| Property      | Value       |
| ------------- | ----------- |
| Kernel thread | No          |
| Shared memory | Yes         |
| Scheduling    | Cooperative |

---

# 6. Actor Model

State ownership through message passing.

[
Actor_i
\overset{message}{\longrightarrow}
Actor_j
]

No shared mutable state.

---

## Libraries

* [Pykka](https://www.pykka.org?utm_source=chatgpt.com)
* [Thespian](https://thespianpy.com/doc/?utm_source=chatgpt.com)

Example:

```python
actor.tell({"type": "increment"})
```

---

## Benefits

* Eliminates races.
* Naturally distributed.
* Failure isolation.

---

# 7. Reactive Streams

Asynchronous event pipelines.

Libraries:

* [RxPY](https://rxpy.readthedocs.io?utm_source=chatgpt.com)

Example:

```python
import rx

rx.from_iterable(range(10)) \
  .map(lambda x: x * 2) \
  .subscribe(print)
```

---

## Formalization

Observable behaves similarly to:

[
Observable[T]
]

which can be viewed as:

[
Time \rightarrow T
]

or more precisely:

[
Observable[T] : Stream(T)
]

---

# 8. Distributed Task Queues

## Libraries

* [Celery](https://docs.celeryq.dev?utm_source=chatgpt.com)
* [RQ](https://python-rq.org?utm_source=chatgpt.com)
* [Dramatiq](https://dramatiq.io?utm_source=chatgpt.com)

Architecture:

```text
Producer
    ↓
Broker
    ↓
Workers
```

---

# 9. Parallel Computing Libraries

| Library                                                                            | Purpose                        |
| ---------------------------------------------------------------------------------- | ------------------------------ |
| [Dask](https://www.dask.org?utm_source=chatgpt.com)                                | Parallel dataframe computation |
| [Ray](https://ray.io?utm_source=chatgpt.com)                                       | Distributed execution          |
| [Joblib](https://joblib.readthedocs.io?utm_source=chatgpt.com)                     | CPU parallel loops             |
| [PySpark](https://spark.apache.org/docs/latest/api/python/?utm_source=chatgpt.com) | Cluster computation            |

---

# 10. Lock-Free and Wait-Free Structures

Python has limited support because of the GIL.

Available tools:

* `queue.Queue`
* `collections.deque`
* `SimpleQueue`

Third-party:

* [janus](https://github.com/aio-libs/janus?utm_source=chatgpt.com)

---

# Decision Matrix

| Workload                | Recommended              |
| ----------------------- | ------------------------ |
| HTTP requests           | `asyncio`                |
| Web server              | `asyncio` + FastAPI      |
| CPU computation         | multiprocessing          |
| Numerical computing     | multiprocessing + NumPy  |
| Background jobs         | Celery                   |
| Shared mutable state    | threads                  |
| Millions of connections | asyncio                  |
| Distributed systems     | actors or message queues |
| Data science pipelines  | Dask or Ray              |

---

# Concurrency Design Patterns

| Pattern             | Python Primitive               |
| ------------------- | ------------------------------ |
| Producer-Consumer   | `Queue`                        |
| Active Object       | `Queue` + thread               |
| Reactor             | `asyncio` event loop           |
| Thread Pool         | `ThreadPoolExecutor`           |
| Scheduler           | `sched`, asyncio               |
| Monitor Object      | `Lock` + `Condition`           |
| Guarded Suspension  | `Condition.wait()`             |
| Readers-Writer Lock | third-party `readerwriterlock` |
| Future/Promise      | `Future`                       |
| Actor               | Pykka                          |
| CSP                 | `asyncio.Queue`, channels      |

---

# Rough Rule

```text
I/O bound?
│
├── yes
│   ├── <1000 concurrent tasks -> threads
│   └── >1000 concurrent tasks -> asyncio
│
└── CPU bound
    ├── single machine -> multiprocessing
    └── cluster -> Ray / Dask
```

Modern Python web stacks such as [FastAPI](https://fastapi.tiangolo.com?utm_source=chatgpt.com), [Starlette](https://www.starlette.io?utm_source=chatgpt.com), and [uvicorn](https://www.uvicorn.org?utm_source=chatgpt.com) are primarily built around the **reactor pattern**, **coroutines**, **futures**, and **message-passing event loops**, whereas traditional frameworks such as [Django](https://www.djangoproject.com?utm_source=chatgpt.com) historically relied on thread-per-request execution models before adding async support.
