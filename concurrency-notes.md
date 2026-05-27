# Concurrency Design Patterns — Compression Table

| Pattern                   | Core Idea                          | Mental Model                | Solves                     | Tradeoff                    | Common Tools         | Real Systems        |
| ------------------------- | ---------------------------------- | --------------------------- | -------------------------- | --------------------------- | -------------------- | ------------------- |
| Producer–Consumer         | Generate then process work         | Work frontier               | Decoupling stages          | Queue contention            | Queue/channel        | Crawlers, Kafka     |
| Worker Pool               | Reuse bounded workers              | Shared executor             | Resource control           | Queue bottleneck            | ThreadPoolExecutor   | Web servers         |
| Fan-Out / Fan-In          | Split then aggregate               | Parallel branches           | Parallelizable subtasks    | Aggregation sync            | Futures/gather       | MapReduce           |
| Pipeline                  | Multi-stage transformation         | Assembly line               | Throughput scaling         | Backpressure complexity     | Queues/channels      | ETL, inference      |
| Event Loop                | Cooperative task scheduler         | Single-thread reactor       | Massive IO concurrency     | Blocking dangerous          | asyncio              | Node.js             |
| Reactor                   | Events dispatch handlers           | IO multiplexing             | Socket scalability         | Callback complexity         | epoll/select         | Nginx               |
| Proactor                  | Completion-driven async            | OS finishes work            | Async IO abstraction       | OS dependence               | IOCP                 | Windows async IO    |
| Actor Model               | Isolated message entities          | Mailboxes                   | Shared-state avoidance     | Serialization overhead      | Ray/Akka             | Distributed systems |
| CSP / Channels            | Communicate via channels           | Synchronizing streams       | Structured coordination    | Channel topology complexity | Go/trio              | Go runtime          |
| Future / Promise          | Handle eventual result             | Deferred completion         | Async coordination         | Dependency chains           | Future/Task          | Async APIs          |
| Pub-Sub                   | Broadcast events                   | Event dissemination         | Decoupled notifications    | Event ordering              | Brokers              | Kafka               |
| Observer                  | Reactive subscriptions             | State listeners             | UI/event propagation       | Cascading updates           | callbacks/signals    | GUIs                |
| Fork–Join                 | Recursive split/merge              | Divide-and-conquer          | Parallel recursion         | Join overhead               | work stealing        | Parallel sorting    |
| Work Stealing             | Idle workers steal tasks           | Dynamic balancing           | Irregular workloads        | Scheduler complexity        | deque schedulers     | ForkJoinPool        |
| Leader–Follower           | One active coordinator             | Rotating leadership         | Reduced contention         | Failover logic              | condition vars       | Servers             |
| Thread-per-Request        | Dedicated execution flow           | Isolated request handling   | Simplicity                 | Poor scalability            | threads              | Old web servers     |
| Thread-per-Core           | Pin execution to cores             | CPU locality                | Cache efficiency           | Less flexible               | affinity             | Low-latency systems |
| Bulk Synchronous Parallel | Parallel then barrier              | Supersteps                  | Deterministic phases       | Barrier stalls              | barriers             | Pregel              |
| MapReduce                 | Parallel map then reduce           | Distributed aggregation     | Data parallelism           | Shuffle overhead            | distributed workers  | Hadoop              |
| Scatter–Gather            | Broadcast then collect             | Distributed fan-out         | Parallel querying          | Tail latency                | futures              | Search engines      |
| Blackboard                | Shared knowledge space             | Opportunistic collaboration | Emergent coordination      | Shared-state contention     | shared datastore     | AI systems          |
| Scheduler / Dispatcher    | Central work allocator             | Global orchestrator         | Fairness/priorities        | Bottleneck risk             | queues               | OS schedulers       |
| DAG Execution             | Dependency-driven tasks            | Partial order execution     | Structured parallelism     | Scheduling complexity       | DAG schedulers       | Airflow/Ray         |
| Barrier Synchronization   | Wait-for-all checkpoint            | Phase alignment             | Deterministic phases       | Slowest-worker bound        | Barrier              | HPC                 |
| Double Buffering          | Swap read/write buffers            | State snapshotting          | Avoid inconsistent reads   | Memory overhead             | buffer swap          | Rendering           |
| Read–Copy–Update (RCU)    | Immutable snapshots                | Versioned state             | Read-heavy concurrency     | Deferred cleanup            | atomic pointers      | Kernels             |
| Copy-on-Write             | Clone before mutation              | Immutable sharing           | Isolation                  | Memory amplification        | snapshots            | Databases           |
| Immutable Dataflow        | No mutation                        | Functional propagation      | Race elimination           | Allocation overhead         | immutable structures | Functional runtimes |
| Backpressure              | Slow downstream throttles upstream | Flow control                | Overload prevention        | Reduced throughput          | bounded queues       | Streaming systems   |
| Circuit Breaker           | Stop cascading failures            | Fail-fast gate              | Fault containment          | Recovery tuning             | counters/timers      | Microservices       |
| Retry with Idempotency    | Re-execute safely                  | Eventually succeeds         | Transient failures         | Duplicate traffic           | retries              | Distributed RPC     |
| Saga Pattern              | Distributed compensating actions   | Eventual consistency        | Multi-service transactions | Compensation complexity     | orchestrators        | Microservices       |
| Lease / Heartbeat         | Temporary ownership                | Soft lock                   | Failure recovery           | Clock sensitivity           | TTLs                 | Distributed locks   |

---

# Patterns By Concurrency Model

| Model           | Typical Patterns                         |
| --------------- | ---------------------------------------- |
| Threads         | worker pool, producer-consumer, locks    |
| Async           | event loop, reactor, futures             |
| Actors          | mailbox, pub-sub, supervision            |
| CSP             | channels, pipelines                      |
| Distributed DAG | scatter-gather, mapreduce, DAG execution |
| GPU/SIMD        | bulk synchronous parallel                |

---

# Shared-State vs Message-Passing Patterns

| Shared-State Patterns | Message-Passing Patterns |
| --------------------- | ------------------------ |
| mutexes               | actors                   |
| reader-writer locks   | channels                 |
| semaphores            | pub-sub                  |
| RCU                   | mailboxes                |
| copy-on-write         | event streams            |

Compression:

```text id="8kqjlwm"
Shared-state concurrency manages mutation.
Message-passing concurrency manages coordination.
```

---

# Web Crawler Pattern Stack

Your multithreaded crawler is basically:

| Layer         | Pattern                 |
| ------------- | ----------------------- |
| Traversal     | Parallel BFS            |
| Coordination  | Producer-consumer       |
| Scheduling    | Worker pool             |
| Deduplication | Shared synchronized set |
| Completion    | Fan-in / join           |
| Filtering     | Constraint propagation  |
| Scalability   | Backpressure            |

Compression:

```text id="h4i65l"
Concurrent frontier expansion
with synchronized deduplication.
```

---

# Important Pattern Families

## 1. Work Distribution Patterns

| Pattern        | Goal                            |
| -------------- | ------------------------------- |
| Worker pool    | Bound concurrency               |
| Work stealing  | Dynamic balancing               |
| Scatter-gather | Parallel querying               |
| MapReduce      | Massive distributed aggregation |

---

## 2. Coordination Patterns

| Pattern          | Goal                     |
| ---------------- | ------------------------ |
| Futures/promises | Deferred synchronization |
| Barrier          | Phase alignment          |
| Pub-sub          | Event dissemination      |
| Observer         | Reactive propagation     |

---

## 3. State Management Patterns

| Pattern         | Goal                 |
| --------------- | -------------------- |
| Immutable data  | Eliminate races      |
| RCU             | Cheap reads          |
| Copy-on-write   | Isolation            |
| Actor isolation | Ownership boundaries |

---

## 4. Fault-Tolerance Patterns

| Pattern           | Goal                      |
| ----------------- | ------------------------- |
| Retry/idempotency | Survive transient failure |
| Circuit breaker   | Prevent cascades          |
| Heartbeat         | Detect failure            |
| Saga              | Distributed rollback      |

---

# Most Important Real-World Compression

Modern systems usually combine:

| Layer        | Pattern            |
| ------------ | ------------------ |
| Ingress      | Event loop/reactor |
| Execution    | Worker pool        |
| Coordination | Futures/channels   |
| Dataflow     | DAG execution      |
| Scaling      | Backpressure       |
| Reliability  | Retry/idempotency  |
| Isolation    | Actors/processes   |

---

# High-Signal Concepts To Internalize

| Concept          | Why Important            |
| ---------------- | ------------------------ |
| Ownership        | Who controls mutation    |
| Backpressure     | Prevent collapse         |
| Partial orders   | Dependency structure     |
| Idempotency      | Safe retries             |
| Task granularity | Coordination overhead    |
| Contention       | Scalability bottleneck   |
| Quiescence       | Global completion        |
| Locality         | Cache/network efficiency |

---

# Ultimate Compression

Concurrency patterns are fundamentally techniques for:

```text id="7b51nc"
managing dependency-constrained work
while balancing:
- correctness,
- throughput,
- latency,
- isolation,
- and resource efficiency
```

Most systems reduce to:

```text id="8i3e0m"
dynamic task graph execution
with coordination and failure semantics
```
