This is about **atomic state mutation**, which is one of the cleanest low-level concurrency tools in Rust.

Let’s structure it from first principles → operations → patterns.

---

# 1. What an atomic variable actually is

An atomic (e.g. `AtomicBool`) is:

> a single memory cell with hardware-enforced indivisible operations

So instead of:

```text id="h2v9q1"
read → modify → write   (race condition possible)
```

you get:

```text id="k1q8zm"
atomic operation (single step at CPU level)
```

---

# 2. Basic type

```rust id="9kq3mv"
use std::sync::atomic::AtomicBool;

let flag = AtomicBool::new(false);
```

Wrapped for threads:

```rust id="q7m1lx"
use std::sync::Arc;

let flag = Arc::new(AtomicBool::new(false));
```

---

# 3. How to flip (toggle) a boolean

## IMPORTANT: there is no direct `toggle()`

You must use **compare-and-swap loops**.

---

## Correct way: `fetch_xor`

This is the cleanest atomic flip for booleans.

```rust id="v9k2pl"
use std::sync::atomic::{AtomicBool, Ordering};

let flag = AtomicBool::new(false);

// flip it
flag.fetch_xor(true, Ordering::SeqCst);
```

---

## Why this works

`XOR` truth table:

| A | A XOR 1 |
| - | ------- |
| 0 | 1       |
| 1 | 0       |

So:

[
flag \leftarrow flag \oplus 1
]

This is an **atomic read-modify-write instruction at CPU level**.

---

# 4. Alternative: compare-and-swap loop (more general)

If you want full control:

```rust id="x3m8kq"
use std::sync::atomic::{AtomicBool, Ordering};

let flag = AtomicBool::new(false);

let mut old = flag.load(Ordering::Relaxed);

loop {
    let new = !old;

    match flag.compare_exchange_weak(
        old,
        new,
        Ordering::SeqCst,
        Ordering::SeqCst,
    ) {
        Ok(_) => break,
        Err(x) => old = x,
    }
}
```

---

## What this is doing

You are expressing:

> “Only update if nobody changed it since I last saw it.”

This is a **fixed-point retry loop over state drift**.

---

# 5. Atomic operations cheat sheet

For `AtomicBool`:

```rust id="c1v7tq"
flag.store(true, Ordering::SeqCst);   // set
flag.store(false, Ordering::SeqCst);  // clear

flag.load(Ordering::SeqCst);         // read

flag.fetch_or(true, Ordering::SeqCst);   // set bits
flag.fetch_and(false, Ordering::SeqCst); // clear bits
flag.fetch_xor(true, Ordering::SeqCst);  // toggle
```

---

# 6. How other threads “see” changes

Example:

```rust id="m8v3qn"
let flag = Arc::new(AtomicBool::new(false));
```

Thread A:

```rust id="a9k2lm"
flag.store(true, Ordering::Release);
```

Thread B:

```rust id="b7x9qv"
if flag.load(Ordering::Acquire) {
    println!("seen!");
}
```

---

## Why ordering matters (important intuition)

Think of:

* `Release` = publish state to memory
* `Acquire` = observe fully initialized state

Without ordering → compiler/CPU may reorder operations.

---

# 7. Pattern: atomic flag for control flow

## Stop signal example

```rust id="p2q8mv"
use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

fn main() {
    let stop = Arc::new(AtomicBool::new(false));

    let worker_stop = stop.clone();

    let handle = thread::spawn(move || {
        while !worker_stop.load(Ordering::Acquire) {
            println!("working...");
            thread::sleep(Duration::from_millis(200));
        }
    });

    thread::sleep(Duration::from_secs(1));

    stop.store(true, Ordering::Release);

    handle.join().unwrap();
}
```

---

# 8. Pattern: atomic toggle shared state

```rust id="q8v2mx"
flag.fetch_xor(true, Ordering::SeqCst);
```

Used for:

* feature flags
* simple on/off switches
* debugging toggles

---

# 9. When NOT to use atomic

Do NOT use atomic if:

* you need multiple fields
* you need invariants across values
* you need blocking/waiting

Instead use:

```text id="z9q1kl"
Mutex<T>
Condvar
channel
```

---

# 10. Category-theoretic view (clean version)

Let:

[
S = {0,1}
]

An atomic variable is a system:

[
S \xrightarrow{\text{atomic op}} S
]

But with constraint:

> all transitions are **linearizable morphisms**

Meaning:

* every operation appears instantaneous
* all threads agree on a single global order

So:

```text id="v1m8qk"
Atomic = globally ordered endomorphisms on state space
```

Toggle:

[
f(s) = 1 - s
]

implemented as a **single morphism in the linearizable category of memory operations**.

---

# 11. Mental model summary

Think of `AtomicBool` as:

> a single shared bit with hardware-level “instant rewrite”

Operations:

* `load` → observe
* `store` → overwrite
* `fetch_xor` → flip atomically
* `compare_exchange` → conditional rewrite

---

If you want next step, I can connect this to:

* how CPU guarantees atomicity (CAS instructions)
* why Mutex internally often uses atomics
* or how Rust builds Mutex → Condvar → channel stack from atomic primitives

That’s where the full concurrency stack becomes mathematically clean.
