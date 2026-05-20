![Table 3 - System design questions per tier](../../../assets/Screenshot%202026-05-20%20at%209.14.17%E2%80%AFPM.png)

A few things worth calling out:

**Table 3** is where most people are caught off-guard. Frontier AI firms increasingly ask ML-adjacent system design (inference serving, vector search, feature stores) - not just the classic URL shortener. HFT firms want you to know *why* a `std::unordered_map` is unacceptable on a hot path and what you'd use instead.

Your biggest leverage right now: **graphs + DP depth + system design foundations**. Those three unlock the startup tier quickly and start opening the frontier AI door.
