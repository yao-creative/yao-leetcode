These are actually two different concepts:

1. **Does a constructor (`new`) belong in the trait?**
2. **What do `Send` and `'static` mean?**

They solve completely different problems.

---

# 1. Should `new()` be in the trait?

**Short answer: No, unless the trait promises that every implementation can be constructed the same way.**

Suppose you have

```rust
trait FooBarTrait {
    fn foo(&self);
}
```

Then one implementation could be

```rust
struct FooBar1 {
    x: i32,
}
```

and another

```rust
struct FooBar2 {
    name: String,
    count: usize,
}
```

Each type has different initialization requirements.

So each type usually has its own constructor:

```rust
impl FooBar1 {
    pub fn new() -> Self {
        Self { x: 0 }
    }
}

impl FooBar2 {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            count: 0,
        }
    }
}
```

Notice these are **not** inside the trait.

---

## When should `new()` be inside the trait?

Only when construction is part of the abstraction.

Example

```rust
trait FooBarTrait {
    fn new() -> Self;
    fn foo(&self);
}
```

Now every implementation **must** provide `new()`.

```rust
impl FooBarTrait for FooBar1 {

    fn new() -> Self {
        Self { x: 0 }
    }

    fn foo(&self) {}
}
```

and

```rust
impl FooBarTrait for FooBar2 {

    fn new() -> Self {
        Self {
            name: String::new(),
            count: 0,
        }
    }

    fn foo(&self) {}
}
```

The trait is saying

> "Every object implementing me knows how to construct itself."

If that isn't part of your abstraction, don't put it in the trait.

---

## Design heuristic

Ask:

> "If I have only a `T: FooBarTrait`, do I need to be able to create one?"

If yes:

```rust
trait FooBarTrait {
    fn new() -> Self;
}
```

Otherwise

```rust
impl FooBar1 {
    fn new() -> Self { ... }
}
```

is enough.

---

# Category theory view

A trait is essentially a specification of morphisms.

Suppose

[
\mathcal{C}
]

is the category of Rust types.

A trait

```rust
trait FooBarTrait {
    fn foo(&self);
}
```

specifies a morphism

[
foo : T \rightarrow ()
]

Adding

```rust
fn new() -> Self
```

adds another required morphism

[
new : 1 \rightarrow T
]

where (1) is the **terminal object** (a type with exactly one value, analogous to Rust's unit type `()`).

So including `new()` in the trait means **every object in the trait's category must admit a canonical morphism from the terminal object**. Many abstractions do not naturally have such a canonical constructor, so `new()` is often left out of the trait.

---

# 2. What is `Send`?

`Send` is one of Rust's **marker traits**.

It has no methods.

```rust
trait Send {}
```

It simply means

> This value may be **moved** to another thread safely.

Notice the word **move**.

Suppose

```rust
let x = String::from("hello");
```

Thread A owns

```text
String
```

If you spawn

```rust
std::thread::spawn(move || {
    println!("{}", x);
});
```

Ownership moves

```text
Thread A

String

↓

move

↓

Thread B
```

After moving,

Thread A no longer owns it.

There is only one owner.

Safe.

---

## Category theory

Ownership is a resource.

Think of

[
Own(T)
]

as

> "currently owning a value of type (T)."

Moving between threads is

[
Own_A(T)
\rightarrow
Own_B(T)
]

`Send` certifies that this morphism preserves validity.

---

# What is `Sync`?

People often confuse these.

`Send`

means

```text
Move ownership
```

`Sync`

means

```text
Share references
```

Formally,

Rust defines

```text
T: Sync
```

iff

```text
&T: Send
```

In words:

> A shared reference to `T` can safely be sent to another thread.

Example

```rust
let x = Arc::new(5);
```

Many threads can read

```text
&x
```

simultaneously.

---

# 3. What is `'static`?

This is probably the most misunderstood lifetime.

Many people think

> "Lives forever."

Not exactly.

It means

> **Contains no borrowed references that could expire too early.**

---

Example

```rust
let s = String::from("hello");
```

The `String` owns its heap allocation.

It contains no borrowed references.

Therefore

```rust
String
```

is `'static`.

Even if you drop it one second later!

The lifetime bound is about **what the type is allowed to contain**, not how long the particular value actually exists.

---

Example

```rust
let s = String::from("hello");

let r = &s;
```

Now

```rust
&r
```

is **not** `'static`.

It points into

```text
s
```

If

```text
s
```

dies,

```text
r
```

would dangle.

---

# Why thread spawning requires `'static`

Suppose

```rust
fn foo() {

    let s = String::from("hello");

    std::thread::spawn(|| {

        println!("{}", s);

    });

}
```

Imagine

```text
foo()

↓

spawn thread

↓

foo returns

↓

s destroyed

↓

thread finally runs
```

The thread would access freed memory.

Bad.

So Rust says

```rust
F: 'static
```

meaning

> "The closure must not capture short-lived borrowed references."

---

# Category theory interpretation

Let

[
\mathcal{L}
]

be a category whose objects are lifetimes.

There is an ordering

[
'a \le 'b
]

meaning

> `'a` is contained within `'b`.

`'static` is the **greatest** lifetime:

[
\forall a,\quad a \le 'static
]

A bound

```rust
T: 'static
```

means that all references inside `T` are valid for the maximum possible lifetime—or, equivalently, that `T` owns its data rather than borrowing from shorter-lived scopes.

---

# Putting it together

When you see

```rust
fn foo<F>(callback: F)
where
    F: Fn() + Send + 'static,
```

the compiler is requiring three independent properties:

* `Fn()` — `F` is callable.
* `Send` — ownership of `F` may be moved to another thread.
* `'static` — `F` does not contain borrowed references that could become invalid before the thread finishes.

This combination is exactly what APIs like `std::thread::spawn` need: a callback that can be called, safely transferred to another thread, and remain valid even if the creating function has already returned.
