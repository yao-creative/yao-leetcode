# 1226. The Dining Philosophers (Rust)

This folder contains:

- `1226-the-dining-philosophers.ipynb`: a copy of the original notebook prompt.
- `src/lib.rs`: a Rust scaffold with the LeetCode-style `DiningPhilosophers` API.
- `cargo test`: a threaded harness that checks each philosopher completes the full pick/eat/put sequence without deadlock.

Usage:

```bash
cargo test
```

Workflow:

1. Implement the synchronization logic inside `DiningPhilosophers::wants_to_eat`.
2. Remove the `#[ignore]` attributes from the case tests.
3. Re-run `cargo test`.
