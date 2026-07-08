What you’re describing is a **shared control signal** across threads:

> “One thread updates a boolean flag; other threads react to it safely.”

In Rust terms, this is a **shared mutable state problem under concurrency**, and there are a few correct designs depending on whether you need *polling*, *blocking*, or *event-driven coordination*.

I’ll structure this from first principles → best practice options.

---

# 1. Core modeling: what you are really building

You want:

* A shared state ( S \in {0,1} )
* Multiple threads reading ( S )
* One thread writing ( S )
* No data race

So the system is:

[
S : \text{shared mutable resource}
]

with transitions:

[
S \xrightarrow{\text{writer}} S'
\quad,\quad
S \xrightarrow{\text{reader}} S
]

The question is: how do we synchronize visibility and mutation?

---

# 2. Option space (Rust concurrency design lattice)

You have 3 main correct primitives:

```text
AtomicBool     → simplest signal
Mutex<bool>    → general shared state
Condvar        → blocking notification system
```

There is also:

```text
channel (mpsc) → message passing (often best)
```

We’ll go from simplest → most powerful.

---

# 3. Best for a boolean flag: `AtomicBool` (recommended first choice)

## Why this is best

A boolean flag is a **single memory location**, so you don’t need a mutex.

You want:

* lock-free
* fast
* shared visibility

---

## Code

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let flag = Arc::new(AtomicBool::new(false));

    let flag_writer = flag.clone();
    let flag_reader = flag.clone();

    // writer thread
    thread::spawn(move || {
        flag_writer.store(true, Ordering::Release);
    });

    // reader thread
    thread::spawn(move || {
        while !flag_reader.load(Ordering::Acquire) {
            // spin or sleep
        }
        println!("flag is ON");
    });
}
```

---

## Category interpretation

* `AtomicBool` = a **single global state morphism**
* `load/store` = synchronized read/write functors
* `Acquire/Release` = ordering constraints (causal edges in execution graph)

---

## When NOT to use this

Avoid if:

* you need multiple fields
* you need blocking (no spin loops)
* you need structured coordination

---

# 4. `Mutex<bool>` (only when state grows beyond 1 bit)

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let flag = Arc::new(Mutex::new(false));

    let flag1 = flag.clone();
    let flag2 = flag.clone();

    thread::spawn(move || {
        let mut f = flag1.lock().unwrap();
        *f = true;
    });

    thread::spawn(move || {
        loop {
            let f = flag2.lock().unwrap();
            if *f {
                break;
            }
        }
        println!("done");
    });
}
```

---

## Problem

This is **worse than atomic** because:

* locks every access
* reader may block writer
* can cause contention

---

## Category view

* `Mutex<T>` = **linear resource wrapper**
* `lock()` = acquire exclusive morphism
* `MutexGuard<T>` = proof of exclusive access

---

# 5. Best practice for “signal + wait”: `Condvar`

This is the *correct design* when:

> one thread signals, others sleep efficiently

---

## Code

```rust
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    let pair_writer = pair.clone();
    let pair_reader = pair.clone();

    // writer
    thread::spawn(move || {
        let (lock, cvar) = &*pair_writer;
        let mut flag = lock.lock().unwrap();
        *flag = true;
        cvar.notify_one();
    });

    // reader
    thread::spawn(move || {
        let (lock, cvar) = &*pair_reader;
        let mut flag = lock.lock().unwrap();

        while !*flag {
            flag = cvar.wait(flag).unwrap();
        }

        println!("flag is true");
    });
}
```

---

## Why this is better

Instead of:

```text
poll → poll → poll
```

you get:

```text
sleep → wake → continue
```

---

## Category theory view

* `Condvar` = **event functor**
* transforms state into **observable transitions**
* `wait` = suspend morphism until external co-morphism occurs

---

# 6. BEST PRACTICE (real-world Rust rule)

## Decision tree

### Case 1: simple flag (stop/start/run)

➡️ use `AtomicBool`

---

### Case 2: multiple fields of state

➡️ use `Mutex<State>`

---

### Case 3: thread coordination (producer/consumer)

➡️ use `channel`

---

### Case 4: blocking synchronization

➡️ use `Condvar + Mutex`

---

# 7. Most idiomatic modern Rust (important)

In real Rust systems:

### Prefer message passing over shared state

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("ON").unwrap();
    });

    println!("{}", rx.recv().unwrap());
}
```

---

## Why this is preferred

Instead of:

```text
shared state mutation
```

you get:

```text
ownership transfer of events
```

Category theory:

* Mutex model → shared object with constraints
* Channel model → morphism between states (no sharing)

Channels avoid:

* locks
* race conditions
* memory visibility complexity

---

# 8. Summary

If your goal is:

### “boolean flag across threads”

Best to worst:

1. ✅ `Arc<AtomicBool>` (best)
2. ⚠️ `Arc<Mutex<bool>>` (ok, heavier)
3. ⚠️ `Condvar` (only if waiting needed)
4. ❌ busy loop with shared variable (bad)

---

# 9. Clean mental model

You are choosing between:

| Model   | Interpretation                    |
| ------- | --------------------------------- |
| Atomic  | single global state cell          |
| Mutex   | exclusive access to shared object |
| Condvar | state + event suspension system   |
| Channel | no shared state, only messages    |

---

If you want, I can next map this into a **formal concurrency algebra (linear logic + category theory)** showing why:

* `AtomicBool ≈ comonoid`
* `Mutex ≈ linear modality`
* `Channel ≈ Kleisli category of IO`

That’s where Rust concurrency becomes very clean mathematically.
