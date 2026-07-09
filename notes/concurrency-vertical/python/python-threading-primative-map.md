`notify` belongs to **condition variables**. It is not a separate synchronization primitive; it is an **operation on a condition variable**.

The compression:

| Operation      | Primitive          | Meaning                                                  |
| -------------- | ------------------ | -------------------------------------------------------- |
| `wait()`       | Condition Variable | "I cannot continue; suspend until predicate may be true" |
| `notify()`     | Condition Variable | "Wake one waiter; state may have changed"                |
| `notify_all()` | Condition Variable | "Wake all waiters; re-check your predicates"             |

---

## Condition variable state machine

A condition variable itself does not store the application state.

You have:

$$
\text{Shared State } S
$$

and a predicate:

$$
P(S)
$$

Example:

```python
queue = []
condition = threading.Condition()
```

Predicate:

$$
P(queue) = |queue| > 0
$$

Consumer:

```python
with condition:
    while len(queue) == 0:
        condition.wait()

    item = queue.pop()
```

Producer:

```python
with condition:
    queue.append(item)
    condition.notify()
```

The flow:

$$
\text{mutate } S
\rightarrow
\text{notify}
\rightarrow
\text{wake waiter}
\rightarrow
\text{recheck } P(S)
$$

---

## Why not just wake immediately?

Because `notify()` does **not mean the condition is true**.

It means:

> "Something changed. You should check."

Example:

Two consumers:

```
queue = []
```

Consumer A:

```
wait()
```

Consumer B:

```
wait()
```

Producer:

```
queue.append(item)
notify_all()
```

Both wake.

But only one gets:

```
queue.pop()
```

The other sees:

```
queue empty again
```

and waits.

Therefore:

```python
while not predicate:
    condition.wait()
```

not:

```python
if not predicate:
    condition.wait()
```

---

## `notify` vs `notify_all`

### notify()

Wake one waiter.

Complexity:

$$
O(1)
$$

Use when:

* one resource became available
* one worker can proceed

Example:

```
1 item added
→ wake 1 consumer
```

---

### notify_all()

Wake everyone.

Complexity:

$$
O(n)
$$

Use when:

* global state changed
* many threads may now proceed

Example:

```
shutdown=True

→ wake every worker
```

---

## Relationship to Event

They look similar:

Event:

```
set()
 |
 v
everyone wakes
```

Condition:

```
notify()
 |
 v
everyone checks predicate
```

Difference:

|                 | Event            | Condition                   |
| --------------- | ---------------- | --------------------------- |
| Memory          | internal boolean | external shared state       |
| Reset           | manual           | predicate naturally changes |
| Direction       | signal           | state coordination          |
| Multiple cycles | awkward          | natural                     |

Example:

### Event

```python
started.set()

worker.wait()
```

Meaning:

> startup happened once.

---

### Condition

```python
while jobs == 0:
    cond.wait()
```

Meaning:

> wait until jobs exist.

---

## Full threading primitive map including notify

```
                 Synchronization

                       |
        --------------------------------
        |              |               |
    Ownership       Signalling      Capacity
        |              |               |
     Mutex        Condition       Semaphore
        |              |
     Lock      wait / notify
                    |
              ----------------
              |              |
           notify()     notify_all()
```

So `notify` is not at the same abstraction level as Lock/Event/Semaphore. It is a **transition operation on a condition-variable-based state machine**.
