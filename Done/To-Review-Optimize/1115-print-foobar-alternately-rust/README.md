# 1115. Print FooBar Alternately (Rust)

This folder contains:

- `1115-print-foobar-alternately.ipynb`: a copy of the original notebook prompt.
- `src/lib.rs`: a Rust scaffold with the LeetCode-style `FooBar` API.
- `cargo test`: a threaded harness that checks the alternating output contract.

Usage:

```bash
cargo test
```

Workflow:

1. Implement the synchronization logic inside `FooBar::foo` and `FooBar::bar`.
2. Remove the `#[ignore]` attributes from the case tests.
3. Re-run `cargo test`.
