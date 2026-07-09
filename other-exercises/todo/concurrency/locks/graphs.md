Yes. In fact, these algorithms are best viewed as **examples of synthesizing a lock protocol from minimal atomic shared state**. Historically, they answer the question:

> **"What is the minimum shared state needed to derive mutual exclusion?"**

Each algorithm introduces exactly one new piece of state because the previous one is missing a correctness property.

---

# LockOne

State:

```text
flag[2] : bool
```

Meaning

* `flag[i] = True` means thread `i` intends to enter.

The protocol is

```mermaid
stateDiagram-v2
    [*] --> Idle

    Idle --> Interested : flag[i]=True
    Interested --> Waiting : flag[other]==True
    Interested --> Critical : flag[other]==False

    Waiting --> Critical : flag[other]==False

    Critical --> Exit
    Exit --> Idle : flag[i]=False
```

The communication graph is

```mermaid
flowchart LR
    T0["Thread 0"]
    F0["flag[0]"]

    T1["Thread 1"]
    F1["flag[1]"]

    T0 --> F0
    T1 --> F1

    F0 --> T1
    F1 --> T0
```

Each thread only advertises:

> "I'm interested."

There is **no arbitration**.

---

## Failure

Both threads execute

```text
flag=True
```

Result

```text
flag=[True,True]
```

Both observe the other's flag.

```mermaid
sequenceDiagram
    participant T0
    participant T1

    T0->>T0: flag0=True
    T1->>T1: flag1=True

    T0->>T0: sees flag1=True
    T1->>T1: sees flag0=True

    Note over T0,T1: Both wait forever
```

---

# LockTwo

State

```text
victim : int
```

Communication

```mermaid
flowchart LR
    T0 --> V[victim]
    T1 --> V

    V --> T0
    V --> T1
```

Protocol

```mermaid
stateDiagram-v2
    [*] --> Idle

    Idle --> Waiting : victim=i

    Waiting --> Critical : victim!=i

    Critical --> Idle
```

Meaning

Each thread says

> "I'll wait if I am the victim."

Notice something strange.

Nobody ever says

> "I actually want the lock."

There is arbitration

but no intent.

---

Failure

```mermaid
sequenceDiagram
    participant T0
    participant T1

    T0->>T0: victim=0
    T1-->>T1: never competes

    T0->>T0: victim==0

    Note over T0: Waits forever
```

---

# Peterson

Peterson composes both ideas.

State

```text
flag[2]
victim
```

Communication graph

```mermaid
flowchart LR

    subgraph Intent
        T0 --> F0["flag0"]
        T1 --> F1["flag1"]

        F0 --> T1
        F1 --> T0
    end

    subgraph Arbitration
        T0 --> V[victim]
        T1 --> V

        V --> T0
        V --> T1
    end
```

State machine

```mermaid
stateDiagram-v2

    [*] --> Idle

    Idle --> Interested : flag=True

    Interested --> Waiting : victim=i

    Waiting --> Critical : !flag[other] OR victim!=i

    Critical --> Exit

    Exit --> Idle : flag=False
```

---

Successful contention

```mermaid
sequenceDiagram

    participant T0
    participant T1

    T0->>T0: flag0=True
    T1->>T1: flag1=True

    T0->>T0: victim=0
    T1->>T1: victim=1

    Note over T0,T1: last write wins

    T0->>T0: victim!=0
    T0->>T0: enters

    T1->>T1: waits

    T0->>T0: flag0=False

    T1->>T1: enters
```

---

# The derivation process

You can think of these algorithms as an incremental synthesis:

```mermaid
flowchart LR

A["No shared state"]
--> B["Add<br/>flag[]"]

B -->|"Mutual exclusion ✓<br/>Progress ✗"| C["Need arbitration"]

C --> D["Add victim"]

D -->|"Peterson"| E["Mutual exclusion ✓<br/>Progress ✓<br/>Bounded waiting ✓"]
```

Each additional shared variable repairs a missing property.

---

# A more abstract view

Viewed algebraically, a lock protocol is a collection of shared state variables together with transition rules.

$$
\text{Lock} = (\text{State},\ \text{Transition},\ \text{Invariant})
$$

For these three algorithms:

| Algorithm | Shared state      | Information represented                               |
| --------- | ----------------- | ----------------------------------------------------- |
| LockOne   | `flag[]`          | **Intent** ("I want the lock")                        |
| LockTwo   | `victim`          | **Arbitration** ("Who yields if there's contention?") |
| Peterson  | `flag[] × victim` | Intent **and** arbitration                            |

In category-theoretic terms, Peterson's lock can be viewed as the **product** of two simpler protocols:

* an **intent protocol** over the Boolean state `flag[]`, and
* an **arbitration protocol** over the `victim` state.

Neither protocol alone satisfies the full specification of mutual exclusion with progress, but their composition does. This illustrates a general engineering pattern: **derive a protocol by composing orthogonal state machines, each responsible for one correctness property**.

This same pattern recurs throughout systems engineering. Modern mutexes, distributed consensus algorithms, transactional databases, and cache-coherence protocols are all built by composing simple coordination mechanisms (intent, ownership, ordering, arbitration, acknowledgment) into a protocol whose global invariants can be proved. Peterson's algorithm is the smallest nontrivial example of this design philosophy.
