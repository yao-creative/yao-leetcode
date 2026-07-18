Your intent is **building an algebraic programming toolkit in Rust**: knowing which language abstractions correspond to mathematical structures like sequences, folds, monoids, morphisms, and state transformations.

For contiguous subarrays + monoid/fold algorithms, you do **not** need all of Rust. You need a specific subset.

---

# 1. Slices: representing subarrays

Mathematical:

[
A_{l,r}=A|_{{l,\dots,r}}
]

Rust:

```rust
let a = vec![1,2,3,4,5];

let sub = &a[1..4];
```

Learn:

### `&[T]`

A borrowed view into contiguous memory.

Methods:

```rust
.len()
.is_empty()
.first()
.last()
.get(i)
.iter()
```

Important:

```rust
&a[l..r]
```

means:

[
\text{restriction}
]

not copying.

---

# 2. Iterators: the sequence abstraction

Mathematical:

[
(x_1,x_2,\dots,x_n)
]

Rust:

```rust
xs.iter()
```

Core methods:

## `map`

Function application:

[
f(x_1),f(x_2),...
]

Rust:

```rust
xs.iter()
   .map(|x| x*x)
```

---

## `filter`

Predicate selection:

[
{x\mid P(x)}
]

Rust:

```rust
xs.iter()
   .filter(|x| **x > 0)
```

---

## `fold`

Monoid reduction:

[
(((e\otimes x_1)\otimes x_2)...)
]

Rust:

```rust
xs.iter()
   .fold(0, |acc,x| acc+x)
```

This is the most important one.

---

## `collect`

Materialize a structure:

[
\text{Iterator}\rightarrow\text{Container}
]

Rust:

```rust
let v:Vec<_> =
    xs.iter()
      .map(|x|x*x)
      .collect();
```

---

# 3. Closures: representing morphisms

Mathematics:

[
f:A\rightarrow B
]

Rust:

```rust
|x| x+1
```

Examples:

```rust
let f = |x:i32| x*x;

println!("{}", f(5));
```

Used everywhere:

```rust
.map(...)
.filter(...)
.fold(...)
.sort_by(...)
```

---

# 4. Ownership and borrowing

This is Rust's core.

For algorithms:

You usually want:

```
data owns memory
        |
        v
algorithm borrows view
```

Example:

```rust
fn sum(xs:&[i32])->i32 {
    xs.iter().sum()
}
```

Meaning:

"I don't own the array."

Learn:

* `&T`
* `&mut T`
* ownership move
* `clone()`
* lifetime basics

---

# 5. Generics: abstract over algebra

Mathematics:

[
\forall M
]

Rust:

```rust
fn fold<M>(...)
```

Example:

```rust
fn combine<T>(a:T,b:T)->T
```

You need:

* generic types
* trait bounds

Example:

```rust
fn f<T: Clone>(x:T)
```

means:

[
T\in \text{Types satisfying Clone}
]

---

# 6. Traits: algebraic structures

This is where Rust maps beautifully.

Mathematics:

[
(M,\otimes,e)
]

Rust:

```rust
trait Monoid {
    fn empty()->Self;

    fn combine(
        self,
        other:Self
    )->Self;
}
```

Traits you need:

* `Clone`
* `Copy`
* `Eq`
* `Ord`
* `Hash`
* `Default`
* `Iterator`
* `IntoIterator`

---

# 7. `Option`: partial functions

Mathematics:

[
f:A\rightarrow B?
]

Rust:

```rust
Option<T>
```

Example:

```rust
xs.first()
```

returns:

```rust
Option<&T>
```

because the first element may not exist.

Methods:

```rust
.unwrap()
.unwrap_or()
.map()
.and_then()
```

---

# 8. `Result`: computation with failure

Mathematics:

[
A\rightarrow B+\text{Error}
]

Rust:

```rust
Result<T,E>
```

Important:

```rust
?
```

operator:

```rust
let x = parse()?; 
```

means propagate failure.

---

# 9. Enums: algebraic data types

Mathematics:

[
A+B
]

(sum types)

Rust:

```rust
enum State {
    Empty,
    Running,
    Done
}
```

Useful for:

* state machines
* parsers
* algorithms

---

# 10. Pattern matching

Mathematics:

case analysis.

Rust:

```rust
match state {
    State::Empty => {},
    State::Running => {},
    State::Done => {}
}
```

Essential.

---

# 11. Collections

## Vec

Dynamic array:

[
[x_0,x_1,...]
]

Methods:

```rust
.push()
.pop()
.len()
.sort()
```

---

## HashMap

Finite mapping:

[
K\rightarrow V
]

Methods:

```rust
.insert()
.get()
.entry()
```

---

## BTreeMap

Ordered map:

[
\text{sorted finite function}
]

Useful for intervals.

---

# 12. The algorithmic hierarchy

For the ideas you are studying:

```
array
 |
 v
slice (&[T])
 |
 v
iterator
 |
 v
map/filter/fold
 |
 v
monoid abstraction
 |
 v
prefix sum / segment tree / DP
```

Rust concepts:

| Math idea        | Rust                  |
| ---------------- | --------------------- |
| finite sequence  | `Vec<T>`              |
| subarray         | `&[T]`                |
| function         | closure               |
| morphism         | trait method/function |
| monoid           | trait                 |
| fold             | `.fold()`             |
| composition      | iterator chains       |
| partial function | `Option`              |
| error algebra    | `Result`              |
| state machine    | `enum`                |

---

For your specific direction (competitive programming + systems design + category theory), the highest ROI order is:

1. `Vec`, slices `&[T]`
2. ownership/borrowing
3. iterators
4. closures
5. `fold`, `map`, `filter`
6. traits + generics
7. `Option` / `Result`
8. enums + pattern matching
9. collections (`HashMap`, `BTreeMap`)
10. async/concurrency later

Once these are solid, most "algebraic" Rust code becomes recognizable.
