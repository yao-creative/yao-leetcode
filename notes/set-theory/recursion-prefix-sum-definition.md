For prefix sum, the decomposition is exactly:

* **Time/index monoid**: ((\mathbb N,+,0))
* **State space**: accumulated values (S=\mathbb Z)
* **Transition**:

$$
G:(S\times E)\rightarrow S
$$

where:

$$
G(s,x)=s+x
$$

* **Recursive trajectory**:

$$
f(0)=0
$$

$$
f(n+1)=G(f(n),a(n))
$$

So the recursion is just repeatedly applying the transition algebra.

---

## Python

### Explicit decomposition

```python
from typing import Callable, Iterable, TypeVar

S = TypeVar("S")
E = TypeVar("E")


def recurse(
    events: Iterable[E],
    initial: S,
    G: Callable[[S, E], S]
):
    state = initial
    history = [state]

    for event in events:
        state = G(state, event)
        history.append(state)

    return history


numbers = [3, 5, 2, 7]


def prefix_sum_transition(state, event):
    return state + event


result = recurse(
    numbers,
    0,
    prefix_sum_transition
)

print(result)
```

Output:

```text
[0, 3, 8, 10, 17]
```

Mathematically this produced:

$$
f=
{
(0,0),
(1,3),
(2,8),
(3,10),
(4,17)
}
$$

---

## The transition (G) as a monoid action

The transition is:

```python
def G(state, x):
    return state + x
```

Composition:

```python
G(G(0,3),5)
```

means:

$$
G(G(0,3),5)
$$

# $$

G(3,5)
$$

$$
=8
$$

The accumulated state is the orbit:

$$
0
\rightarrow
3
\rightarrow
8
\rightarrow
10
\rightarrow
17
$$

---

# Rust

Rust version makes the state transition explicit.

```rust
fn recurse<S, E, F>(
    events: Vec<E>,
    initial: S,
    transition: F,
) -> Vec<S>
where
    S: Clone,
    F: Fn(S, E) -> S,
{
    let mut state = initial;
    let mut history = vec![state.clone()];

    for event in events {
        state = transition(state, event);
        history.push(state.clone());
    }

    history
}


fn main() {
    let numbers = vec![3, 5, 2, 7];

    let prefix = recurse(
        numbers,
        0,
        |state, x| state + x
    );

    println!("{:?}", prefix);
}
```

Output:

```text
[0, 3, 8, 10, 17]
```

---

# Making the algebra explicit in Rust

You can encode the "monoid" idea:

```rust
trait Monoid {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}
```

For integers:

```rust
impl Monoid for i32 {
    fn empty() -> Self {
        0
    }

    fn combine(self, other: Self) -> Self {
        self + other
    }
}
```

Then prefix sum is:

```rust
fn prefix_sum<T: Monoid + Clone>(xs: Vec<T>) -> Vec<T> {
    let mut acc = T::empty();
    let mut result = vec![acc.clone()];

    for x in xs {
        acc = acc.combine(x);
        result.push(acc.clone());
    }

    result
}
```

Now the same recursion works for:

* integers → addition
* strings → concatenation
* sets → union
* matrices → multiplication
* probability distributions → composition

because the only requirement is the algebra:

$$
(S,\otimes,e)
$$

---

The production connection is that systems like:

* MapReduce
* Spark `reduce`
* database aggregation
* distributed counters
* stream processors

all exploit this same decomposition:

$$
\boxed{
\text{sequence of events}
+
\text{monoid transition}
\rightarrow
\text{folded state}
}
$$

Prefix sum is the smallest example of the same architecture.
