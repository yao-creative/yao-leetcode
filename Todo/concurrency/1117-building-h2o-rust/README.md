# 1117. Building H2O (Rust)

This folder contains:

- `1117-building-h2o.ipynb`: a copy of the original notebook prompt.
- `src/lib.rs`: a Rust scaffold with the LeetCode-style `H2O` API.
- `cargo test`: a threaded harness that validates each released batch contains two hydrogens and one oxygen.

Usage:

```bash
cargo test
```

Workflow:

1. Implement the synchronization logic inside `H2O::hydrogen` and `H2O::oxygen`.
2. Remove the `#[ignore]` attributes from the case tests.
3. Re-run `cargo test`.
