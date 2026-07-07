This question is really asking for the **semantic model** behind Rust. The synchronization primitives (`Arc`, `Mutex`, `Condvar`) are not arbitrary libraries—they each relax or enforce different structural constraints on the ownership graph.

Let's build it from first principles.

---

# Level 0: The ownership graph

A Rust program at any instant can be modeled as a directed graph

[
G=(V,E)
]

where

* (V) = runtime objects
* (E) = references (ownership or borrowing)

There are different edge types.

## Ownership edge

```text
Stack Variable
      │ owns
      ▼
   Heap Object
```

Graphically

```text
x ─────────▶ String
```

There is exactly **one ownership edge** into an object.

Formally,

[
\forall o,\quad |\text{Owners}(o)| = 1
]

This invariant prevents double frees.

---

# Borrow edges

Immutable borrow

```rust
let r = &x;
```

creates

```text
x ─────▶ Object
│
├────▶ r
```

The borrow edge points to the owned object without transferring ownership.

Multiple immutable borrows are allowed.

[
\deg^{-}_{borrow}(o) \ge 0
]

---

Mutable borrow

```rust
let r = &mut x;
```

creates

```text
x ─────▶ Object
│
└────▶ mutable borrow
```

The important invariant becomes

[
|\text{MutableBorrows}(o)| \le 1
]

---

# No aliasing

The famous Rust rule

> many readers OR one writer

is simply

[
\text{Readers}(o)\times\text{Writers}(o)=0
]

Equivalently

[
\neg
\left(
|\text{Readers}|>0
\land
|\text{Writers}|>0
\right)
]

This is sometimes called the **aliasing invariant**.

---

# Category-theoretic interpretation

Treat program states as objects.

[
S_0,S_1,S_2,\dots
]

A borrow is a morphism

[
b:S_i\rightarrow S_{i+1}
]

whose codomain contains additional references.

Ownership transfer (`move`)

```rust
let y = x;
```

is another morphism

[
m:S_i\rightarrow S_{i+1}
]

that removes one ownership edge and creates another.

The invariant is

[
\text{Owner}(o)
]

is preserved up to isomorphism.

Ownership is never duplicated.

---

# Linear logic interpretation

Rust's ownership system is extremely close to **linear logic**.

A value behaves like a linear resource.

```
consume once
```

You cannot duplicate it.

```
copy
```

requires explicit permission (`Copy`).

Borrowing corresponds to temporarily weakening the ownership restriction.

This is why people say Rust has an **affine type system**.

---

# When do you use `Arc<T>`?

`Arc` means

> multiple owners of the same object.

Normally

```text
A owns Object
```

After `Arc`

```text
Thread A ─┐
           │
Thread B ──┼────▶ Arc Control Block ───▶ Object
           │
Thread C ──┘
```

Notice something subtle.

The object still has one actual allocation.

The owners are really owning the **Arc control block**.

That block stores

* pointer
* atomic reference count

When count reaches zero

```
drop object
```

---

Formally

Without `Arc`

Owner relation

[
Owner:Object\rightarrow Variable
]

is a function.

With `Arc`

it becomes

[
Owners(Object)
==============

{v_1,v_2,\dots,v_n}
]

implemented safely through atomic reference counting.

---

## When do you need it?

Whenever ownership crosses thread boundaries.

Example

```rust
thread::spawn(move || ...)
```

consumes ownership.

If another thread still needs the object,

```
Arc
```

creates another owner.

---

# When do you use `Mutex<T>`?

`Arc` solves

> Who owns the object?

It does **not** solve

> Who may mutate it?

Suppose

```text
Thread A
Thread B
```

both own

```
Arc<Vec<_>>
```

Now both may try

```
push()
```

simultaneously.

Race condition.

A mutex introduces an exclusive-access node.

Graphically

```text
Thread A
        \
         \
          Mutex
         /
        /
Thread B
```

Only one path through the mutex exists at a time.

Formally

The mutex maintains

[
|\text{CurrentWriter}| \le 1
]

---

Category theoretically

Instead of

```
Thread → Object
```

we have

```
Thread → Mutex → Object
```

Composition only exists after acquiring the lock.

---

# When is Mutex needed?

Whenever

* shared mutable state
* multiple threads
* mutation

Example

```rust
Arc<Mutex<Vec<i32>>>
```

Everyone owns the vector.

Only one thread edits it.

---

# When do you use `Condvar`?

A mutex protects data.

A condition variable protects **state transitions**.

Suppose

```
queue empty
```

Worker shouldn't spin

```rust
loop {
    if queue.is_empty() {}
}
```

Instead

```
sleep
```

until another thread changes the predicate.

Graph

```text
Producer
     │
     ▼
 Queue State
     ▲
     │
Consumer waits
```

Producer performs

```
notify()
```

Consumer resumes.

---

Mathematically

There is a predicate

[
P(s)
]

on system state.

Consumer waits until

[
P(s)=\text{true}
]

The condition variable is not the predicate.

It merely notifies

> "The predicate may have changed."

Hence Rust always writes

```rust
while !condition {
    condition = condvar.wait(condition).unwrap();
}
```

because waking up does **not** imply

[
P(s)=\text{true}
]

only that it should be checked again.

---

# Putting them together

Suppose

```rust
Arc<Mutex<Vec<Job>>>
```

Graph

```text
          Arc
       /   |   \
 Thread Thread Thread
        \   |   /
         Mutex
           │
           ▼
        Job Queue
```

Ownership

```
Arc
```

Mutual exclusion

```
Mutex
```

Blocking until work exists

```
Condvar
```

Graph

```text
          Arc
       /   |   \
      T1  T2  T3
        \  |  /
        Mutex
          │
      Job Queue
          ▲
          │
      Condition
```

---

# Decision rule

When designing concurrent Rust code, ask three questions:

1. **Does exactly one scope own this value?**

   * Yes → own `T`.
   * No → use `Arc<T>` (or `Rc<T>` for single-threaded code).

2. **Can multiple threads modify it?**

   * No → `Arc<T>` is enough.
   * Yes → wrap it in `Mutex<T>` (or another synchronization primitive), giving `Arc<Mutex<T>>`.

3. **Do threads need to wait for a state change rather than repeatedly checking?**

   * Yes → pair the protected state with a `Condvar`, typically `Arc<(Mutex<State>, Condvar)>`.

This progression mirrors the structural constraints:

* **Ownership** controls *who can keep the object alive*.
* **Borrowing** controls *who can access it*.
* **Mutex** controls *who can mutate it right now*.
* **Condvar** controls *when execution may proceed based on a state predicate*.
