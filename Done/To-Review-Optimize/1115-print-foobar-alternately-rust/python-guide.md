Yes. If by **single item** you mean "I have one boolean flag / one piece of state and I want to wait until it changes without polling", then use a **condition variable**. This is the standard non-busy-waiting pattern.

For a single boolean:

```python
import threading

class AtomicFlag:
    def __init__(self, value=False):
        self.value = value
        self.cond = threading.Condition()

    def set(self, value):
        with self.cond:
            self.value = value
            self.cond.notify_all()

    def wait_until(self, target):
        with self.cond:
            self.cond.wait_for(lambda: self.value == target)

    def get(self):
        with self.cond:
            return self.value


flag = AtomicFlag(True)

def worker():
    print("waiting for false...")
    flag.wait_until(False)
    print("flag is false now")

threading.Thread(target=worker).start()

# later...
flag.set(False)
```

The worker thread **sleeps**. It is not repeatedly checking.

---

## Why not just use `Event`?

`Event` is optimized for a one-directional synchronization pattern:

$$
False \xrightarrow{set} True
$$

Example:

* "configuration loaded"
* "shutdown requested"
* "worker finished"

A boolean flag with arbitrary transitions is a different abstraction:

$$
False \leftrightarrow True
$$

You need a condition variable because the wait condition is a predicate:

$$
wait(P(state))
$$

For your case:

$$
P(state)=\neg state
$$

---

## If it is literally one item in a queue

If by "single item" you mean "wait until an item exists or is removed", then `queue.Queue(maxsize=1)` is even better:

```python
from queue import Queue

q = Queue(maxsize=1)

# producer
q.put("item")   # blocks if full

# consumer
item = q.get()  # blocks if empty
```

This is also non-busy-waiting.

---

### Rule of thumb

* **One-way signal** → `threading.Event`
* **One mutable state variable with arbitrary conditions** → `threading.Condition`
* **One item / producer-consumer handoff** → `queue.Queue(maxsize=1)`

Conceptually:

$$
Event \subset Condition \subset General\ Synchronization
$$

An `Event` is basically a specialized condition variable where the predicate is fixed to:

$$
P(s)=s=True
$$

while `Condition` lets you define:

$$
P:S\rightarrow Boolean
$$

for any state predicate.
