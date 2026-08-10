That's a good direction. Since you asked for **no spoilers**, I'll keep it focused on the iterator machinery rather than how to solve the exercise.

For an iterator over all natural numbers, Rust already provides one natively.

The simplest starting point is:

```rust
0..
```

Its type is essentially:

```rust
std::ops::RangeFrom<usize>
```

Conceptually, this represents the function

$$
\eta : \mathbb{N} \to \mathbb{N},\qquad \eta(n)=n,
$$

except it is implemented lazily. No collection of infinitely many numbers is ever allocated.

You can think of it as a coalgebra:

$$
s \mapsto (s, s+1)
$$

where the state is simply the current natural number.

---

### Building custom infinite iterators

Rust has several native patterns for creating infinite iterators:

```rust
std::iter::repeat(x)
```

produces

$$
x,x,x,\ldots
$$

---

```rust
std::iter::repeat_with(f)
```

calls `f` each time a value is requested.

---

```rust
std::iter::successors(seed, next)
```

starts from one value and repeatedly computes the next from the previous.

Its mathematical shape is

$$
x_0,;
g(x_0),;
g(g(x_0)),;
\ldots
$$

This is a very general way to describe recursively generated sequences.

---

### Iterator methods

One of the nicest parts of Rust is that almost all iterator functionality comes from the `Iterator` trait itself.

A useful way to organize the methods is by their type signatures.

#### Structure-preserving (Functor-like)

These transform each element independently:

```rust
map
inspect
enumerate
cloned
copied
```

They have the general shape

$$
I(X)\to I(Y).
$$

---

#### Filtering

These may remove elements:

```rust
filter
filter_map
skip
take
```

They still produce iterators.

---

#### Combining iterators

```rust
chain
zip
interleave (itertools)
```

These combine multiple traversals.

---

#### Consumers

These end the lazy computation and produce a result:

```rust
collect
fold
count
sum
product
all
any
find
position
```

Their shape is generally

$$
I(X)\to Y.
$$

---

### For your exercise

Without giving away any algorithmic hints, I'd recommend becoming comfortable with just these methods:

* `next`
* `map`
* `filter`
* `take`
* `skip`
* `enumerate`
* `zip`
* `peekable`
* `fold`

They cover a large fraction of idiomatic iterator usage.

Finally, since you mentioned you're thinking about **FIFO**, ask yourself one design question:

> Is the FIFO queue **the iterator itself**, or is it **the internal state that the iterator traverses**?

Those are mathematically different objects, and distinguishing them is often the key insight in iterator design. I won't say which applies to your exercise, but it's a useful question to keep in mind.
