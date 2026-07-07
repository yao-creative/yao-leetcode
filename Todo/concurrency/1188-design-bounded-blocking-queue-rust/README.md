# 1188. Design Bounded Blocking Queue (Rust)

This folder contains:

- `1188-design-bounded-blocking-queue.ipynb`: a copy of the original notebook prompt.
- `src/lib.rs`: a Rust scaffold with the LeetCode-style `BoundedBlockingQueue` API.
- `cargo test`: sequential and threaded harnesses that check FIFO order, blocking dequeue, and backpressure.

Usage:

```bash
cargo test
```

Workflow:

1. Implement the synchronization logic inside `BoundedBlockingQueue::enqueue`, `dequeue`, and `size`.
2. Remove the `#[ignore]` attributes from the blocking case tests.
3. Re-run `cargo test`.
