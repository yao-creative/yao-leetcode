The intent of your question is **formal verification modeling**: you want the mathematical language used to express properties like safety, progress, starvation freedom, and bounded waiting.

Temporal logic is the extension of ordinary logic with **time-dependent statements**.

Normal logic talks about truth:

$$
P
$$

meaning:

> "P is true."

Temporal logic talks about how truth changes across executions:

$$
P \text{ is true now, eventually, always, or next.}
$$

It is the logic used heavily in:

* operating systems verification
* concurrent algorithms
* distributed systems
* hardware verification
* model checking

---

# 1. The basic model

A concurrent system is modeled as a transition system:

$$
M=(S,\rightarrow,L)
$$

where:

* (S) = set of states
* (\rightarrow) = possible transitions
* (L) = labeling function telling which propositions are true in each state

Example:

```text
State:

flag0 = true
flag1 = false
victim = 0

Thread0 = waiting
Thread1 = idle
```

A computation is a path:

$$
s_0 \rightarrow s_1 \rightarrow s_2 \rightarrow ...
$$

Temporal logic describes properties of these paths.

---

# 2. The two main temporal operators

## Next

$$
X P
$$

means:

> P is true in the next state.

Example:

$$
X(flag_0=false)
$$

means:

> On the next transition, thread 0 clears its flag.

---

## Eventually

$$
\Diamond P
$$

(read "eventually P")

means:

> At some future point, P becomes true.

Example:

$$
\Diamond CS_0
$$

means:

> Thread 0 eventually enters the critical section.

---

## Always

$$
\Box P
$$

means:

> P is true forever.

Example:

$$
\Box \neg(CS_0 \land CS_1)
$$

means:

> It is always true that both threads are not simultaneously in the critical section.

This is mutual exclusion.

---

## Until

$$
P\ U\ Q
$$

means:

> P remains true until Q becomes true.

Example:

$$
waiting_0\ U\ CS_0
$$

means:

> Thread 0 keeps waiting until it enters.

---

# 3. Safety properties

Safety means:

> "Something bad never happens."

Form:

$$
\Box \neg Bad
$$

Example: mutual exclusion.

Bad state:

$$
Bad=CS_0\land CS_1
$$

Therefore:

$$
\boxed{
\Box \neg(CS_0\land CS_1)
}
$$

Meaning:

> Always avoid the state where both threads are inside.

---

Other examples:

Memory safety:

$$
\Box(\neg InvalidAccess)
$$

Queue invariant:

$$
\Box(size\ge0)
$$

Lock ownership:

$$
\Box(owner\in Threads\cup{None})
$$

---

# 4. Liveness properties

Liveness means:

> "Something good eventually happens."

Form:

$$
\Diamond Good
$$

Example:

A thread eventually enters:

$$
\Diamond CS_i
$$

---

Progress:

"If somebody wants the lock, somebody eventually gets it."

$$
\Box(
Trying
\Rightarrow
\Diamond CS
)
$$

---

# 5. Fairness

Fairness prevents the scheduler from cheating.

Example:

A thread is ready forever:

```text
T0: ready
T1: running forever
```

Without fairness, the system can say:

> "T0 never ran, but that's allowed."

Fairness assumptions remove these unrealistic executions.

---

## Weak fairness

If an action is continuously enabled, it eventually happens.

Formal:

$$
\Box\Diamond Enabled(A)
\Rightarrow
\Box\Diamond Execute(A)
$$

Meaning:

> If you are always able to run, you eventually run.

---

## Strong fairness

If an action becomes enabled infinitely often, it eventually happens.

$$
\Box\Diamond Enabled(A)
\Rightarrow
\Diamond Execute(A)
$$

Stronger.

---

# 6. Safety vs liveness in Peterson

Peterson mutex:

Safety:

$$
\Box \neg(CS_0\land CS_1)
$$

Progress:

$$
\Box(
Trying_0\lor Trying_1
\Rightarrow
\Diamond(CS_0\lor CS_1)
)
$$

Starvation freedom:

$$
\Box(
Trying_i
\Rightarrow
\Diamond CS_i
)
$$

Bounded waiting:

Temporal logic alone usually cannot express the exact numeric bound easily.

You often need:

* counters
* ranking functions
* extended temporal logics

Example:

$$
\Box(
Request_i
\Rightarrow
CS_i \text{ within } N \text{ steps}
)
$$

---

# 7. LTL vs CTL

Two major families.

---

## Linear Temporal Logic (LTL)

Looks at one execution path.

Question:

> "Does every possible execution trace satisfy this?"

Example:

$$
\Box(request_i \Rightarrow \Diamond grant_i)
$$

Used for:

* protocols
* concurrency
* distributed systems

---

## Computation Tree Logic (CTL)

Looks at branching futures.

Operators:

$$
A
$$

= all paths

$$
E
$$

= some path

Example:

All paths eventually:

$$
AF\ p
$$

Some path eventually:

$$
EF\ p
$$

Used in:

* hardware model checking
* state exploration

---

# 8. Connection to your lock proofs

Your Peterson proof is basically:

Define states:

$$
S=(flag_0,flag_1,victim,pc_0,pc_1)
$$

Define invariant:

$$
I=\neg(CS_0\land CS_1)
$$

Prove:

Initial:

$$
I(s_0)
$$

Preservation:

$$
I(s)\land(s\rightarrow s')
\Rightarrow I(s')
$$

Therefore:

$$
\boxed{\Box I}
$$

The temporal logic statement is the final property:

> Along every execution, the invariant always holds.

---

The mental model:

$$
\textbf{State logic}
\rightarrow
\textbf{Transition system}
\rightarrow
\textbf{Temporal logic}
$$

* Ordinary logic: "What is true in this state?"
* Automata: "How can states evolve?"
* Temporal logic: "What must be true across all evolutions?"

Locks, databases, networks, and distributed systems are essentially all exercises in proving temporal properties over state-transition systems.
