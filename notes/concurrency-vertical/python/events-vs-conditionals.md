The distinction is **what is the source of truth**.

* Use **Event** when the truth is a **single boolean fact**.
* Use **Condition** when the truth is a **predicate over shared state**.

Formally:

### Event

State space:

$$
S = {0,1}
$$

Predicate:

$$
P(S) = S = 1
$$

It represents:

> "Has this thing happened?"

Examples:

* server started
* model loaded
* shutdown requested
* configuration refreshed
* training completed

---

### Condition

State space:

$$
S = \text{your application state}
$$

Predicate:

$$
P : S \rightarrow {true,false}
$$

It represents:

> "Is the system currently in a state where I can proceed?"

Examples:

* queue has items
* buffer has space
* balance is sufficient
* state reached version 10
* worker pool has capacity

---

## Example 1: Startup signal → Event

You have workers:

```python
ready = threading.Event()

def worker():
    ready.wait()
    run_job()
```

Controller:

```python
initialize_model()

ready.set()
```

Meaning:

```
False  ---> True
```

Once the model is ready, it stays ready.

Event fits.

---

## Example 2: Producer-consumer queue → Condition

Bad abstraction:

```python
new_item = Event()
```

Why?

Because the event cannot represent:

```
queue length = 0
queue length = 1
queue length = 10
queue length = 0 again
```

You need:

```python
condition = threading.Condition()

while len(queue) == 0:
    condition.wait()
```

Producer:

```python
queue.append(item)
condition.notify()
```

The predicate is:

$$
|queue| > 0
$$

Condition fits.

---

## Key difference: persistence

### Event is usually monotonic

```
unset
  |
  v
set
```

Although Python allows:

```python
clear()
```

the common pattern is a lifecycle signal.

Examples:

```
application_ready
shutdown_requested
connection_established
```

---

### Condition tracks changing state

```
empty
 |
 v
available
 |
 v
empty
 |
 v
available
```

Examples:

```
queue length
buffer space
workers available
```

---

## Decision table

| Question                                     | Use              |
| -------------------------------------------- | ---------------- |
| "Did something happen?"                      | Event            |
| "Should everyone know something happened?"   | Event            |
| "Should I wake when a state becomes true?"   | Condition        |
| "Will the state repeatedly change?"          | Condition        |
| "Do I need to protect shared mutable state?" | Condition + Lock |
| "Do I only need a flag?"                     | Event            |

---

## A useful mental model

Think of:

### Event = notification channel

```
Producer
   |
   v
"ready!"
   |
   v
Everyone continues
```

### Condition = state coordination protocol

```
Thread A:
    "I need state X"

        |
        v

Shared State

        ^
        |

Thread B:
    "I changed state"
```

---

## In real Python code

Prefer:

```python
threading.Event()
```

for:

* lifecycle management
* cancellation
* startup/shutdown
* one-time readiness

Prefer:

```python
threading.Condition()
```

for:

* queues
* resource pools
* bounded buffers
* producer-consumer
* complex state machines

A good compression:

$$
\boxed{
Event = Condition + Boolean invariant
}
$$

$$
\boxed{
Condition = Wait/Notify mechanism for arbitrary invariants
}
$$

`Event` is the specialized tool; `Condition` is the general mechanism.
