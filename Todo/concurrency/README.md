# Concurrency Traversal Order

Use this sequence to build the core synchronization ideas from simplest ordering guarantees to broader coordination patterns.

| Order | Problem | Concept |
| --- | --- | --- |
| 1 | 1114 | Happens-before |
| 2 | 1115 | Coordination protocol |
| 3 | 1116 | Finite-state synchronization |
| 4 | 1117 | Barrier synchronization |
| 5 | 1188 | Producer-consumer |
| 6 | 1226 | Deadlock avoidance |
| 7 | 1242 | Worker pools |

Notes:
- `1114` teaches the smallest useful ordering guarantee.
- `1115` adds alternating turn ownership between peers.
- `1116` turns that idea into a multi-state protocol.
- `1117` introduces batch release and rendezvous constraints.
- `1188` applies blocking and backpressure to a shared queue.
- `1226` focuses on safe shared-resource acquisition.
- `1242` expands the model to parallel task execution across workers.
