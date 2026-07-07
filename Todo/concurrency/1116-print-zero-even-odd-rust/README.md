# 1116. Print Zero Even Odd (Rust)

This folder contains:

- `1116-print-zero-even-odd.ipynb`: a copy of the original notebook prompt.
- `src/lib.rs`: a Rust scaffold with the LeetCode-style `ZeroEvenOdd` API.
- `cargo test`: a threaded harness that checks the required `0 x 0 y ...` ordering.

Usage:

```bash
cargo test
```

Workflow:

1. Implement the synchronization logic inside `ZeroEvenOdd::zero`, `ZeroEvenOdd::even`, and `ZeroEvenOdd::odd`.
2. Remove the `#[ignore]` attributes from the case tests.
3. Re-run `cargo test`.
