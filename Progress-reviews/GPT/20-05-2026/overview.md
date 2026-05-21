Your current exercise distribution signals:

- **Strong early-intermediate algorithmic fluency**
- Good exposure to **core data structures**
- Some exposure to **systems-flavored design questions**
- Weakness in:
  - graph depth
  - advanced DP
  - concurrency/distributed reasoning
  - implementation-heavy problems
  - probabilistic/data-intensive systems thinking
  - low-level performance reasoning
  - mathematical modeling
  - production-grade abstractions

For frontier AI infra/trading/system firms, the gap is not “more LeetCode quantity.”
It is mostly:

1. **harder abstraction layers**
2. **implementation depth**
3. **performance modeling**
4. **systems constraints**
5. **graph/state/concurrency reasoning**
6. **mathematical maturity under uncertainty**

---

# Current Capability Estimate

## Relative ladder


| Level                          | Signal                                                  |
| ------------------------------ | ------------------------------------------------------- |
| Beginner                       | syntax + brute force                                    |
| Early intermediate             | standard DS&A patterns                                  |
| Intermediate                   | can solve medium unseen problems consistently           |
| Strong intermediate            | recognizes reductions/pattern transfer                  |
| Advanced                       | derives solutions under ambiguity                       |
| Frontier engineering candidate | combines algorithms + systems + modeling + optimization |


You are around:

# **Strong early-intermediate → intermediate**

More precisely:


| Area                         | Level              |
| ---------------------------- | ------------------ |
| Arrays/hashmaps              | good               |
| Stack/queue                  | good               |
| Sliding window               | decent             |
| Binary search                | decent             |
| Trees/BST                    | basic-intermediate |
| Trie                         | decent             |
| Heap/top-k                   | decent             |
| Graph traversal              | basic              |
| DAG reasoning                | weak               |
| Dynamic programming          | weak-basic         |
| Greedy proofs                | weak               |
| Backtracking                 | basic              |
| Design questions             | basic-intermediate |
| SQL                          | basic              |
| Advanced graph algorithms    | missing            |
| String algorithms            | mostly missing     |
| Geometry/math                | missing            |
| Bit manipulation             | missing            |
| Concurrency                  | missing            |
| Distributed systems modeling | missing            |
| Cache-aware performance      | missing            |
| Probabilistic algorithms     | missing            |
| Streaming/dataflow           | missing            |


---

# What Frontier AI/Trading Firms Actually Screen For

## AI Infra / Frontier Startups

Examples:

- [OpenAI](https://openai.com?utm_source=chatgpt.com)
- [Anthropic](https://www.anthropic.com?utm_source=chatgpt.com)
- [Thinking Machines Lab](https://thinkingmachines.ai?utm_source=chatgpt.com)
- [Perplexity](https://www.perplexity.ai?utm_source=chatgpt.com)
- [Modal](https://modal.com?utm_source=chatgpt.com)
- [Weights & Biases](https://wandb.ai?utm_source=chatgpt.com)

They increasingly care about:


| Category                         | Importance     |
| -------------------------------- | -------------- |
| async/distributed systems        | extremely high |
| memory/cache/network bottlenecks | high           |
| event-driven architectures       | high           |
| graph/state reasoning            | high           |
| implementation speed             | high           |
| debugging ambiguity              | extremely high |
| ML systems intuition             | high           |
| concurrency correctness          | high           |
| algorithm puzzles                | medium         |
| competitive programming tricks   | low-medium     |


---

## Trading Firms

Examples:

- [Jane Street](https://www.janestreet.com?utm_source=chatgpt.com)
- [Hudson River Trading](https://www.hudsonrivertrading.com?utm_source=chatgpt.com)
- [Citadel Securities](https://www.citadelsecurities.com?utm_source=chatgpt.com)
- [IMC Trading](https://www.imc.com?utm_source=chatgpt.com)

They care much more about:


| Category                  | Importance |
| ------------------------- | ---------- |
| probability/combinatorics | extreme    |
| invariants                | extreme    |
| optimization reasoning    | extreme    |
| graph/search              | high       |
| mental simulation         | high       |
| low latency thinking      | high       |
| advanced DS&A             | high       |
| mathematical proofs       | high       |
| systems programming       | high       |


Your current set is still far from this profile.

---

# Your Current Pattern Bias

You are currently over-indexed toward:

- canonical interview mediums
- CRUD-style DS&A
- isolated data structures

Under-indexed toward:

- state-space search
- formal invariants
- dynamic graph reasoning
- optimization
- amortized analysis
- scheduler/resource modeling
- online algorithms
- adversarial inputs
- concurrency/state machines
- heap+graph hybrids
- streaming systems

---

# Biggest Missing Topic Clusters

## Tier 1 Missing (urgent)

These are the highest leverage.


| Topic                           | Why it matters                          |
| ------------------------------- | --------------------------------------- |
| Topological sorting mastery     | workflow engines, build systems, agents |
| Union Find                      | distributed grouping/connectivity       |
| Advanced graph shortest path    | routing/schedulers/networking           |
| Monotonic structures            | allocators/schedulers/stream processing |
| Advanced heap patterns          | infra + trading                         |
| Interval/event sweep            | schedulers/log systems                  |
| State machine modeling          | async systems/agents                    |
| Advanced recursion/backtracking | search/planning                         |
| Memoized DP                     | optimization reasoning                  |
| Trie + DFS hybrids              | search engines/LLM retrieval            |


---

## Tier 2 Missing


| Topic                 | Why                             |
| --------------------- | ------------------------------- |
| Segment trees/Fenwick | streaming analytics             |
| Bitmask DP            | search-space optimization       |
| String algorithms     | retrieval/search/compiler infra |
| Rolling hashes        | dedup/search                    |
| KMP/Z                 | parsing/indexing                |
| Prefix sum variants   | analytics engines               |
| Line sweep            | simulations                     |
| Priority scheduling   | OS/task orchestration           |


---

## Tier 3 Missing


| Topic                           | Why                         |
| ------------------------------- | --------------------------- |
| Concurrency primitives          | frontier infra              |
| Lock-free reasoning             | high-performance systems    |
| NUMA/cache locality             | low latency                 |
| SIMD/vectorization              | inference infra             |
| Memory allocators               | systems depth               |
| Distributed consensus intuition | infra startups              |
| CRDT/event sourcing             | collaborative/agent systems |


---

# What Your Existing Problems Reveal

## Strong signals

### LRU Cache

Good sign because it combines:

- hashmap
- DLL
- invariants

### Design Twitter

Good systems abstraction starter.

### Task Scheduler

Good because it introduces:

- greedy
- heap scheduling
- cooldown constraints

### Course Schedule

Very important. DAG reasoning matters a lot.

### Network Delay Time

Good transition into weighted graphs.

### Largest Rectangle in Histogram

Very important pattern. Monotonic stack problems are disproportionately useful.

---

# Missing Difficulty Gradient

You have many isolated mediums, but not enough:


| Type                       | Missing |
| -------------------------- | ------- |
| hard graph problems        | yes     |
| hard DP                    | yes     |
| implementation-heavy hards | yes     |
| simulation problems        | yes     |
| state-machine problems     | yes     |
| heap+graph combos          | yes     |
| online algorithms          | yes     |
| interval scheduling        | yes     |
| streaming problems         | yes     |


---

# What Would Raise You Fastest

The fastest path is NOT random LeetCode.

It is:

## Phase 1 — Structural Algorithms

Master:

- graphs
- heaps
- monotonic structures
- intervals
- DP foundations

## Phase 2 — Systems Algorithms

Then:

- schedulers
- async/state machines
- caches
- queues
- event systems
- rate limiting
- consistent hashing
- streaming windows

## Phase 3 — Performance Thinking

Then:

- memory locality
- concurrency
- lock contention
- batching
- sharding
- vectorization
- amortization

That combination maps much closer to frontier engineering than “500 random mediums.”

---

# Randomized Next Exercises (Urgency Weighted, No Labels)

Ordered to maximize capability compounding.

1. Alien Dictionary
2. Word Ladder
3. Reconstruct Itinerary
4. Serialize and Deserialize Binary Tree
5. Accounts Merge
6. Daily Temperatures
7. Sliding Window Maximum
8. Trapping Rain Water
9. Median Finder
10. Cheapest Flights Within K Stops
11. Pacific Atlantic Water Flow
12. Number of Connected Components in an Undirected Graph
13. Rotting Oranges
14. Course Schedule II
15. Redundant Connection
16. Path With Minimum Effort
17. Clone Graph
18. Decode Ways
19. Coin Change
20. Partition Equal Subset Sum
21. Target Sum
22. Longest Increasing Subsequence
23. Edit Distance
24. Burst Balloons
25. N-Queens
26. Sudoku Solver
27. Word Search II
28. LFU Cache
29. Design Hit Counter
30. Time Based Key Value Store
31. My Calendar I
32. Employee Free Time
33. Meeting Rooms II
34. Car Fleet
35. Merge Triplets to Form Target Triplet
36. Swim in Rising Water
37. Find Median from Data Stream
38. Minimum Window Substring
39. All O(One) Data Structure
40. Minimum Cost to Connect Points
41. Network Delay Time (optimize again from scratch)
42. Task Scheduler (derive mathematically, no memorization)
43. Design Snake Game
44. Web Crawler Multithreaded
45. Dining Philosophers
46. Consistent Hashing implementation
47. Mini Redis
48. Build a Rate Limiter
49. Build an Async Task Queue
50. Implement a Persistent Trie

---

# Frontier-Level Meta Gap

Your largest gap is probably not raw algorithms.

It is:

# translating abstract system constraints into data structure invariants

Examples:


| Real system         | Core abstraction           |
| ------------------- | -------------------------- |
| GPU scheduler       | priority queues + fairness |
| Retrieval engine    | trie + ranking heap        |
| Agent orchestration | DAG + state machine        |
| Streaming analytics | sliding windows + sketches |
| Cache infra         | hashmap + eviction policy  |
| Distributed queue   | ordering + leases          |
| Event sourcing      | append-only logs + replay  |
| Rate limiter        | token bucket/leaky bucket  |
| Vector DB           | graph + ANN indexing       |


That translation skill is what separates:

- “good interview prep”
from
- “frontier systems engineer.”

# Algorithms / CS Topics Progression by Company Type


| Topic Cluster            | Big Tech SWE      | Frontier AI Infra | Trading/HFT  | Distributed Systems Infra | Research/ML Systems |
| ------------------------ | ----------------- | ----------------- | ------------ | ------------------------- | ------------------- |
| Arrays/Hashmaps          | foundational      | foundational      | foundational | foundational              | foundational        |
| Stack/Queue              | foundational      | foundational      | foundational | foundational              | foundational        |
| Sliding Window           | medium importance | medium            | high         | medium                    | medium              |
| Binary Search            | medium            | medium            | very high    | medium                    | medium              |
| Trees/BST                | medium            | medium            | high         | medium                    | medium              |
| Trie/Search Indexing     | low-medium        | very high         | medium       | high                      | very high           |
| Heap/Priority Queue      | high              | very high         | extreme      | very high                 | high                |
| Graph Traversal          | high              | extreme           | very high    | extreme                   | high                |
| DAG/Topological Systems  | medium            | extreme           | high         | extreme                   | high                |
| Shortest Path Algorithms | medium            | high              | extreme      | very high                 | medium              |
| Union Find               | medium            | high              | high         | high                      | medium              |
| Monotonic Structures     | medium            | high              | extreme      | high                      | medium              |
| Interval/Sweep Line      | medium            | high              | extreme      | medium                    | low                 |
| Backtracking/Search      | medium            | medium            | very high    | low                       | high                |
| Dynamic Programming      | medium            | medium            | extreme      | low-medium                | high                |
| Greedy Proofs            | medium            | high              | extreme      | medium                    | medium              |
| Bit Manipulation         | low-medium        | medium            | extreme      | low                       | medium              |
| String Algorithms        | medium            | high              | high         | high                      | very high           |
| Streaming Algorithms     | low               | very high         | high         | extreme                   | very high           |
| Concurrency              | medium            | extreme           | very high    | extreme                   | high                |
| Lock-free Structures     | low               | very high         | extreme      | high                      | medium              |
| Cache Locality/NUMA      | low               | high              | extreme      | medium                    | high                |
| SIMD/Vectorization       | low               | high              | extreme      | low                       | extreme             |
| Distributed Consensus    | low               | high              | low          | extreme                   | medium              |
| State Machines           | medium            | extreme           | high         | extreme                   | high                |
| Event Sourcing           | low               | high              | medium       | very high                 | medium              |
| Probabilistic Structures | low               | high              | extreme      | high                      | extreme             |
| Memory Allocators        | low               | high              | extreme      | high                      | medium              |
| GPU/Parallel Computing   | low               | extreme           | medium       | medium                    | extreme             |


---

# Question Progression by Company Type

## Big Tech SWE Progression


| Stage      | Question Types                                                    |
| ---------- | ----------------------------------------------------------------- |
| Entry      | Two Sum, Valid Anagram, Binary Search                             |
| Mid        | LRU Cache, Group Anagrams, K Closest Points                       |
| Strong Mid | Course Schedule, Clone Graph, Merge Intervals                     |
| Senior     | Design Twitter, LFU Cache, Word Ladder                            |
| Staff+     | Distributed rate limiter, streaming aggregation, scheduler design |


---

## Frontier AI Infra Progression

Examples:

- [OpenAI](https://openai.com?utm_source=chatgpt.com)
- [Anthropic](https://www.anthropic.com?utm_source=chatgpt.com)
- [Modal](https://modal.com?utm_source=chatgpt.com)


| Stage      | Question Types                                                                      |
| ---------- | ----------------------------------------------------------------------------------- |
| Entry      | Heap scheduling, DAG traversal, trie problems                                       |
| Mid        | Sliding Window Maximum, Task Scheduler, Network Delay Time                          |
| Strong Mid | Async queues, distributed caching, workflow orchestration                           |
| Senior     | Multi-agent DAG execution, GPU scheduling, streaming systems                        |
| Staff+     | Distributed inference routing, fault-tolerant orchestration, vector retrieval infra |


---

## Trading / HFT Progression

Examples:

- [Jane Street](https://www.janestreet.com?utm_source=chatgpt.com)
- [Hudson River Trading](https://www.hudsonrivertrading.com?utm_source=chatgpt.com)
- [Citadel Securities](https://www.citadelsecurities.com?utm_source=chatgpt.com)


| Stage      | Question Types                                                           |
| ---------- | ------------------------------------------------------------------------ |
| Entry      | Binary search, heaps, probability puzzles                                |
| Mid        | Sliding windows, interval merging, monotonic stacks                      |
| Strong Mid | DP optimization, graph shortest paths, combinatorics                     |
| Senior     | Lock contention modeling, low-latency queues, memory layout optimization |
| Staff+     | Exchange simulation, matching engines, NUMA-aware schedulers             |


---

## Distributed Systems / Infra Companies

Examples:

- [Cloudflare](https://www.cloudflare.com?utm_source=chatgpt.com)
- [Datadog](https://www.datadoghq.com?utm_source=chatgpt.com)
- [Confluent](https://www.confluent.io?utm_source=chatgpt.com)


| Stage      | Question Types                                                |
| ---------- | ------------------------------------------------------------- |
| Entry      | BFS/DFS, queues, caching                                      |
| Mid        | Consistent hashing, retry queues, task schedulers             |
| Strong Mid | Distributed locks, pub/sub, event sourcing                    |
| Senior     | Consensus intuition, replication models, backpressure systems |
| Staff+     | Multi-region failover, distributed logs, adaptive schedulers  |


---

# Systems Design Progression by Company Type

## Big Tech SWE Systems Design


| Level     | Typical Design Questions      |
| --------- | ----------------------------- |
| Mid       | URL shortener                 |
| Senior    | Twitter feed                  |
| Staff     | YouTube/Dropbox               |
| Principal | Global recommendation systems |


---

## Frontier AI Infra Systems Design


| Level     | Typical Design Questions                             |
| --------- | ---------------------------------------------------- |
| Mid       | Embedding search service                             |
| Senior    | Vector database                                      |
| Senior+   | LLM inference scheduler                              |
| Staff     | Multi-agent orchestration platform                   |
| Principal | Global distributed training/inference infrastructure |


---

## Trading / HFT Systems Design


| Level     | Typical Design Questions        |
| --------- | ------------------------------- |
| Mid       | Real-time market feed processor |
| Senior    | Matching engine                 |
| Senior+   | Low-latency order router        |
| Staff     | Risk management engine          |
| Principal | Exchange-scale trading platform |


---

## Distributed Systems Infra Design


| Level     | Typical Design Questions            |
| --------- | ----------------------------------- |
| Mid       | Distributed cache                   |
| Senior    | Kafka-like event bus                |
| Senior+   | Distributed cron/task scheduler     |
| Staff     | Multi-region replicated datastore   |
| Principal | Planet-scale observability pipeline |


---

## Research / ML Systems Design


| Level     | Typical Design Questions                   |
| --------- | ------------------------------------------ |
| Mid       | ML training pipeline                       |
| Senior    | Feature store                              |
| Senior+   | Distributed inference serving              |
| Staff     | Multi-tenant GPU cluster                   |
| Principal | Autonomous retraining/evaluation ecosystem |


---

# Fastest Capability Compounding Order


| Priority | Cluster                                 |
| -------- | --------------------------------------- |
| 1        | Graphs + heaps + DAGs                   |
| 2        | Scheduling/resource allocation          |
| 3        | Monotonic + interval structures         |
| 4        | State machines + async                  |
| 5        | Streaming/dataflow                      |
| 6        | Concurrency + lock contention           |
| 7        | Distributed systems primitives          |
| 8        | Performance engineering                 |
| 9        | Probabilistic/data-intensive algorithms |
| 10       | GPU/distributed ML infrastructure       |


