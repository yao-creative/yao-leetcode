# 1279. Traffic Light Controlled Intersection (Rust)

This folder contains:

- `1279-traffic-light-controlled-intersection.ipynb`: a copy of the original notebook prompt.
- `src/lib.rs`: a Rust scaffold with the LeetCode-style `TrafficLight` API.
- `cargo test`: a threaded harness that checks green-light switching and crossing serialization.

Usage:

```bash
cargo test
```

Workflow:

1. Implement the synchronization logic inside `TrafficLight::car_arrived`.
2. Remove the `#[ignore]` attributes from the case tests.
3. Re-run `cargo test`.
