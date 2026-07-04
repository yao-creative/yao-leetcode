Below is a **LeetCode-to-concurrency-pattern mapping** organized from a **goal → coordination constraint → synchronization primitive → problem archetype** perspective. This is useful because most concurrency problems reduce to *what must be coordinated* rather than *which pattern name is used*.

I’ll treat each pattern as:
**(1) coordination intent → (2) primitive → (3) closest LeetCode-style exercise(s)**

---

## 1. Active Object

**Intent:** decouple method invocation from execution via async queue

* Core mechanism: request queue + worker thread + future/promise
* LeetCode analogs:

  * LC 1188 – *Design Bounded Blocking Queue*
  * LC 1114 – *Print in Order*
  * LC 1242 – *Web Crawler Multithreaded*

**Why:** caller enqueues “commands”, worker executes asynchronously.

---

## 2. Balking Pattern

**Intent:** skip execution if system is not in valid state

* Primitive: lock + state guard + early return
* LeetCode analogs:

  * LC 1116 – *Print Zero Even Odd* (balk when not your turn)
  * LC 1226 – *The Dining Philosophers* (balking on unavailable fork acquisition variants)
  * LC 1246 – *Palindrome Removal* (conceptually state-dependent skip, though not threading)

**Key idea:** “if not ready → do nothing immediately”

---

## 3. Barrier

**Intent:** synchronize phases; all threads must arrive before proceeding

* Primitive: CountDownLatch / CyclicBarrier
* LeetCode analogs:

  * LC 1117 – *Building H2O*
  * LC 1115 – *Print FooBar Alternately*
  * LC 1226 – *Dining Philosophers* (phase coordination variant)

**Core structure:** phase = collect N arrivals → release all

---

## 4. Double-Checked Locking

**Intent:** minimize locking overhead for lazy initialization

* Primitive: volatile + synchronized block
* LeetCode analogs:

  * LC 1117 – *Building H2O* (initialization gating style variants)
  * LC 1242 – *Web Crawler* (visited set lazy init pattern)
  * LC 1246 – *Web Crawler Multithreaded*

**Core structure:**

```text
if (not initialized)
   lock
     if (still not initialized)
        initialize
```

---

## 5. Guarded Suspension

**Intent:** wait until condition becomes true

* Primitive: condition variable / wait-notify
* LeetCode analogs:

  * LC 1114 – *Print in Order*
  * LC 1115 – *FooBar Alternately*
  * LC 1188 – *Bounded Blocking Queue*

**Key structure:** `while (!condition) wait()`

---

## 6. Monitor Object

**Intent:** encapsulate shared state + synchronization in one object

* Primitive: intrinsic lock (synchronized methods)
* LeetCode analogs:

  * LC 1188 – *Bounded Blocking Queue*
  * LC 1226 – *Dining Philosophers*
  * LC 1114 – *Print in Order*

**Core idea:** object = state + lock + methods

---

## 7. “Nuclear Reaction”

This is not a standard concurrency pattern name. If interpreted literally, it maps best to:

**Intent (interpreted): cascading state-triggered execution**

* Primitive: event chain / propagation / fork-join explosion
* LeetCode analogs:

  * LC 1192 – *Critical Connections in a Network* (cascade propagation thinking)
  * LC 207/210 – *Course Schedule I/II* (dependency explosion)
  * LC 1242 – *Web Crawler Multithreaded*

If you meant a specific textbook pattern, clarify and I can re-map precisely.

---

## 8. Reactor Pattern

**Intent:** single-threaded event loop demultiplexes IO events

* Primitive: event loop + non-blocking IO + dispatcher
* LeetCode analogs:

  * LC 346 – *Moving Average from Data Stream* (stream processing flavor)
  * LC 359 – *Logger Rate Limiter*
  * LC 1622-ish conceptually (custom stream processors; no direct LC perfect match)

More realistic mapping:

* Any “design event system / stream processor” interview problem

**Core structure:**

```text
event loop → select ready events → dispatch handler
```

---

## 9. Readers–Writer Lock

**Intent:** many readers OR one writer

* Primitive: read-write lock
* LeetCode analogs:

  * LC 1226 – *Dining Philosophers* (resource sharing variant)
  * LC 1117 – *Building H2O* (role-based access constraints)
  * LC 1195 – *Fizz Buzz Multithreaded* (role separation pattern)

**Key constraint:** concurrent reads allowed, writes exclusive

---

## 10. Scheduler Pattern

**Intent:** decide execution order across competing tasks

* Primitive: priority queue + condition variables
* LeetCode analogs:

  * LC 621 – *Task Scheduler*
  * LC 1834 – *Single-Threaded CPU*
  * LC 1244 – *Design Leaderboard* (ranking scheduler flavor)

**Core structure:** choose next runnable task based on policy

---

## 11. Thread Pool Pattern

**Intent:** reuse fixed worker threads to process tasks

* Primitive: blocking queue + worker loop
* LeetCode analogs:

  * LC 1188 – *Bounded Blocking Queue*
  * LC 1242 – *Web Crawler Multithreaded*
  * LC 1117 – *Building H2O* (thread reuse roles)

**Core structure:**

```text
workers = fixed N threads
queue → dispatch → execute → loop
```

---

## 12. Thread-Local Storage

**Intent:** per-thread isolated state

* Primitive: ThreadLocal map
* LeetCode analogs:

  * LC 1117 – *Building H2O* (thread identity role tracking)
  * LC 1116 – *Print Zero Even Odd* (thread identity control logic)
  * LC 1226 – *Dining Philosophers* (per-thread state reasoning)

**Core idea:** no shared mutation; state indexed by thread identity

---

# Higher-level compression (useful mental model)

You can reduce all of these into 3 coordination primitives:

### A. Ordering constraints

* Barrier
* Guarded suspension
* Scheduler

### B. Resource constraints

* Readers–writer lock
* Monitor object
* Balking

### C. Execution decoupling

* Active object
* Thread pool
* Reactor

Everything else is just a *composition of these three axes*.

---

If you want, I can next:

* map these into **Rust/Java/Python implementations side-by-side**
* or build a **LeetCode progression path (easy → hard → system design)**
* or express each pattern as a **category-theory morphism (Kleisli / comonad style)**
