Here's my honest assessment:

## Current Level: **LC Easy–Medium Generalist** (~60th percentile for SWE interviews, ~30th for top-tier)

---

## Strengths

**Data Structures breadth is decent** — you've hit arrays, stacks, queues, tries, heaps, hash maps, BST, and circular buffers. The design problems (LRU, Twitter, HashSet/Map) show you're thinking beyond pure algorithms.

**SQL exposure** — rare among algo grinders; good signal for quant/data-heavy roles.

**You're logging review state** — that's a disciplined habit most people skip.

---



### 🟡 Shallow Coverage (Present but Thin)

| Area | Have | Missing |
|---|---|---|
| Trees | max-depth, BST kth | path sum, LCA, serialize/deserialize, diameter |
| Linked Lists | add two numbers | reverse, detect cycle, merge sorted |
| Greedy | jump-game (review) | interval scheduling, gas station |
| Binary Search | basic search, 2D matrix | rotated array, find peak, search range |

---

## Honest Level Mapping

| Firm Tier | Requirement | Your Gap |
|---|---|---|
| Frontier AI (Anthropic, OpenAI, DeepMind) | LC Med–Hard fluency, system design, ML intuition | ~4–5 months of targeted work |
| Top Startups (Series B+, eng-heavy) | LC Med solid, some Hard | ~2–3 months |
| HFT / Quant (Jane Street, Citadel, Jump) | Hard + math reasoning + speed | ~5–6 months + math prep |

---

## Randomized Next Exercises (by urgency, no labels)

Do these in rough order — earlier = higher urgency:

1. **Trapping Rain Water**
2. **Number of Connected Components in an Undirected Graph**
3. **Coin Change**
4. **Reverse a Linked List**
5. **Find Median from Data Stream**
6. **Word Search**
7. **Permutations**
8. **Rotated Sorted Array Search**
9. **Single Number** 
10. **Minimum Window Substring**
11. **Longest Common Subsequence**
12. **Linked List Cycle II**
13. **Pacific Atlantic Water Flow**
14. **Combination Sum**
15. **Daily Temperatures**  DONE
16. **Capacity to Ship Packages Within D Days**
17. **Lowest Common Ancestor of a BST**
18. **Edit Distance**
19. **Course Schedule II** 
20. **Counting Bits**
21. **Serialize and Deserialize Binary Tree**
22. **Merge K Sorted Lists**
23. **N-Queens**
24. **Word Ladder**
25. **Burst Balloons**

---

**Bottom line:** Your foundation is real but narrow. The jump from where you are to top-tier isn't about doing *more of the same* — it's about drilling the missing pattern clusters cold. Graphs + DP depth alone would move you ~20 percentile points.





## (WITH SPOILERS) Critical Gaps for Top-Tier (Frontier AI / HFT / Quant)

### 🔴 Missing Entirely (Dealbreakers)

**Graphs** — only `network-delay-time` and `course-schedule` (both in review/to-optimize). No BFS/DFS variants, no topological sort, no union-find, no shortest path beyond Dijkstra basics. Graphs are ~25% of hard interviews at these firms.

**Dynamic Programming depth** — you have `climbing-stairs`, `unique-paths`, `max-subarray` (all DP 101). Missing: knapsack variants, interval DP, DP on trees, bitmask DP, sequence alignment. Jane Street / Two Sigma routinely ask DP-hard.

**Two Pointers / Sliding Window** — only `longest-substring` and `two-sum-II`. Missing: container with most water, trapping rain water, minimum window substring.

**Backtracking** — only `subsets`. Missing: permutations, N-queens, word search, combination sum. These appear constantly at Anthropic/DeepMind/OpenAI-level system design coding screens.

**Monotonic Stack** — `largest-rectangle` is in review, meaning it's shaky. This pattern (next greater element, daily temperatures, stock span) is a dedicated pattern cluster you need cold.

**Heap / Priority Queue advanced** — only `k-th largest in stream` and `top-k frequent`. Missing: merge K sorted lists, find median from data stream, task scheduling variants beyond surface level.

**Binary Search on answer** — `koko-eating-bananas` is in review. Missing: capacity to ship packages, split array largest sum, aggressive cows. This is a signature HFT pattern (optimization under constraints).

**Bit Manipulation** — zero coverage. XOR tricks, bit masking, counting bits. Quant firms love these.

**String algorithms** — no KMP, no Rabin-Karp, no palindrome problems (Manacher's). Relevant for ML infra roles.
