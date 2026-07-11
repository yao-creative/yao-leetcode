The Bakery algorithm is naturally modeled as **two interacting state machines**:

1. **Per-thread lifecycle** (what each thread does)
2. **Lock protocol** (the doorway and waiting semantics)

The first is usually the most useful.

```mermaid
stateDiagram-v2
    [*] --> Idle

    Idle --> Choosing : lock()

    Choosing : choosing[i] = true
    Choosing : number[i] = max(number)+1

    Choosing --> Waiting : choosing[i] = false

    Waiting --> Waiting : exists j\nchoosing[j]
    Waiting --> Waiting : exists j\n(number[j],j) < (number[i],i)

    Waiting --> Critical : no higher priority thread

    Critical : critical section

    Critical --> Unlock : unlock()

    Unlock : number[i] = 0

    Unlock --> Idle
```

---

## Expanded version with the doorway made explicit

```mermaid
stateDiagram-v2
    [*] --> Idle

    Idle --> RaiseChoosing

    RaiseChoosing : choosing[i]=true

    RaiseChoosing --> PickTicket

    PickTicket : number[i]=1+max(number)

    PickTicket --> LowerChoosing

    LowerChoosing : choosing[i]=false

    LowerChoosing --> WaitChoosing

    WaitChoosing --> WaitChoosing : choosing[j]==true
    WaitChoosing --> WaitPriority : choosing[j]==false

    WaitPriority --> WaitPriority : number[j]!=0 &&\n(number[j],j)<(number[i],i)

    WaitPriority --> Critical : all j checked

    Critical --> Release

    Release : number[i]=0

    Release --> Idle
```

---

## Protocol interaction between two threads

This illustrates why the `choosing` flag exists.

```mermaid
sequenceDiagram
    participant A
    participant B

    A->>A: choosing[A]=true
    A->>A: number[A]=5

    B->>A: read choosing[A]
    Note over B: waits

    A->>A: choosing[A]=false

    B->>A: read number[A]=5

    alt A has priority
        B->>B: spin
    else B has priority
        B->>B: continue
    end
```

The critical invariant is:

* `choosing = true` means **ticket is unstable**.
* `choosing = false` means **ticket is finalized**.
* `number = 0` means **not competing**.

---

## Formal state machine

For thread (i), the protocol can be expressed as a finite-state automaton:

[
Q =
{
\text{Idle},
\text{Choosing},
\text{Waiting},
\text{Critical}
}
]

with transitions

[
\begin{aligned}
\text{Idle}
&\xrightarrow{\text{lock}}
\text{Choosing} \
\text{Choosing}
&\xrightarrow{\text{ticket assigned}}
\text{Waiting} \
\text{Waiting}
&\xrightarrow{\forall j,\ (number[j],j)\ge(number[i],i)}
\text{Critical} \
\text{Critical}
&\xrightarrow{\text{unlock}}
\text{Idle}.
\end{aligned}
]

The important insight is that **`Choosing` is a transient "doorway" state, not part of lock ownership**. Ownership begins only upon entering **Critical**, while `number = 0` is what relinquishes ownership.
