These three locks represent increasing sophistication in synchronization algorithms.

| Lock          | Primitive      | Scalability | Fairness | Busy Wait | Mutual Exclusion | Hardware Support  |
| ------------- | -------------- | ----------- | -------- | --------- | ---------------- | ----------------- |
| LockOne       | Shared flags   | 2 threads   | No       | Yes       | Yes              | Atomic read/write |
| LockTwo       | Victim (turn)  | 2 threads   | No       | Yes       | No (alone)       | Atomic read/write |
| Peterson Lock | Flags + Victim | 2 threads   | Yes      | Yes       | Yes              | Atomic read/write |

---

# Formal model

A lock is a protocol over shared state.

Let

* Threads

$$
T = {0,1,\ldots,n-1}
$$

* Lock state

$$
S
$$

* Operations

$$
lock_i : S \rightarrow S
$$

$$
unlock_i : S \rightarrow S
$$

The desired invariant is

$$
|{i \mid i \in CriticalSection}| \le 1
$$

This is **mutual exclusion**.

---

# Core correctness properties

Synchronization algorithms are usually judged by three properties.

## 1. Mutual Exclusion

At most one thread enters the critical section.

Formally

$$
\forall t,;
|CS(t)|\le1
$$

---

## 2. Progress

If nobody is inside the critical section,

and one or more threads want to enter,

eventually one succeeds.

No deadlock.

---

## 3. Bounded Waiting

No starvation.

If thread (i) requests entry,

other threads can enter only a bounded number of times before (i).

---

# LockOne

Shared state

```text
flag[2]
```

Initially

```python
flag = [False, False]
```

Algorithm

Thread i

```python
flag[i] = True

while flag[j]:
    pass

# critical section

flag[i] = False
```

where

```python
j = 1 - i
```

---

## Intuition

"I'm interested."

If the other thread is interested,

wait.

---

### State

$$
(flag_0,flag_1)
$$

---

### Safety

Works.

Both cannot pass simultaneously once both flags are visible.

---

### Problem

If both set

```text
flag=True
```

at nearly the same time

```
flag=[True,True]
```

both wait forever.

Deadlock.

No progress.

---

Python example

```python
class LockOne:
    def __init__(self):
        self.flag = [False, False]

    def lock(self, i):
        other = 1 - i
        self.flag[i] = True
        while self.flag[other]:
            pass

    def unlock(self, i):
        self.flag[i] = False
```

---

# LockTwo

Instead of flags,

use one shared variable.

```python
victim = 0
```

Algorithm

```python
victim = i

while victim == i:
    pass
```

---

Meaning

"I yield priority to myself."

If the other thread changes

```python
victim
```

you proceed.

---

Problem

Suppose

Thread 0 executes

```python
victim = 0
```

Immediately enters.

Thread 1

never wants the lock.

Thread 0 now waits forever because

```python
victim == 0
```

No one changes it.

Progress fails.

Even worse,

if timing is unfortunate,

mutual exclusion can also fail because there is no indication that another thread actually intends to enter.

---

Python

```python
class LockTwo:
    def __init__(self):
        self.victim = 0

    def lock(self, i):
        self.victim = i
        while self.victim == i:
            pass

    def unlock(self, i):
        pass
```

---

# Peterson Lock

Observation

Need both

* intent
* tie breaker

Shared state

```python
flag = [False, False]
victim = 0
```

---

Algorithm

```python
flag[i] = True
victim = i

while flag[other] and victim == i:
    pass
```

Unlock

```python
flag[i] = False
```

---

Interpretation

Flag

```
I want in.
```

Victim

```
If both want in,
I'll politely wait.
```

---

State

$$
(flag_0,flag_1,victim)
$$

---

Waiting condition

$$
flag_{other}
\land
victim=i
$$

Need both.

---

Cases

## Case 1

Only one thread wants lock.

```
flag=[True,False]
```

Condition

```
False
```

Immediately enters.

---

## Case 2

Both want lock.

```
flag=[True,True]
```

Both set victim.

Last write wins.

Suppose

```
victim=1
```

Thread 1 waits

```
flag[0] and victim==1
```

True.

Thread 0 sees

```
victim==0
```

False.

Thread 0 enters.

---

When thread 0 exits

```python
flag[0]=False
```

Thread 1 proceeds.

No deadlock.

No starvation.

---

Python

```python
class PetersonLock:
    def __init__(self):
        self.flag = [False, False]
        self.victim = 0

    def lock(self, i):
        other = 1 - i

        self.flag[i] = True
        self.victim = i

        while self.flag[other] and self.victim == i:
            pass

    def unlock(self, i):
        self.flag[i] = False
```

---

# Why Peterson works

Think of the protocol as combining two predicates:

* **Interest predicate**:
  $$
  I_i = \text{flag}[i]
  $$
  which records a thread's intention to enter.

* **Priority predicate**:
  $$
  P_i = (\text{victim} = i)
  $$
  which records who should back off if there is contention.

A thread waits exactly when

$$
I_{\text{other}} \land P_i.
$$

This conjunction is essential:

* If only the interest predicate exists (LockOne), simultaneous interest can lead to deadlock.
* If only the priority predicate exists (LockTwo), there is no reliable indication of contention, so progress and safety are not guaranteed.
* Peterson's algorithm composes both predicates, ensuring:

  * **Safety:** two threads cannot satisfy the entry condition simultaneously.
  * **Progress:** if only one thread is interested, it enters immediately.
  * **Bounded waiting:** when both compete, the `victim` variable alternates priority so neither thread can be postponed indefinitely.

From a category-theoretic perspective, Peterson's lock can be viewed as the product of two state machines: one modeling **intent** (`flag`) and one modeling **tie-breaking** (`victim`). The lock protocol is their synchronized product, where transitions preserve the global invariant:

$$
|\text{CriticalSection}| \le 1.
$$

This illustrates a common design pattern in concurrent algorithms: compose simple protocols, each enforcing one aspect of correctness, to obtain a protocol satisfying a richer set of liveness and safety properties.
