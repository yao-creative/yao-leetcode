# More Data Structures

## Existing Coverage

### Done
- Two Sum
- Add Two Numbers
- Two Integer Sum II
- Min Stack
- Evaluate Reverse Polish Notation
- Longest Substring Without Repeating Characters
- Kth Smallest Integer in BST
- Implement Trie (Prefix Tree)
- K-th Largest Element in a Stream
- Merge Intervals
- Binary Search
- Subsets
- Number of Islands
- Search a 2D Matrix
- Maximum Depth of Binary Tree
- Climbing Stairs
- Unique Paths

### Todo
- Reverse Integer
- Maximum Subarray
- Top K Frequent Elements
- Jump Game II

### random-practice/data-structures
- Valid Sudoku
- Longest Consecutive Sequence
- Daily Temperatures
- Kth Largest Element in an Array
- Time Based Key-Value Store
- Copy List with Random Pointer
- LRU Cache
- Merge k Sorted Lists
- Word Search II
- Find Median from Data Stream
- LFU Cache

## Ranked High-ROI Additions

1. `49-design-hashset.ipynb` — Design HashSet
   Why here: shortest warm-up for bucketed hashing and collision handling.
2. `50-design-hashmap.ipynb` — Design HashMap
   Why here: same family as HashSet, but with key-value updates and removal paths.
3. `51-implement-queue-using-stacks.ipynb` — Implement Queue using Stacks
   Why here: easy FIFO/LIFO inversion and amortized reasoning.
4. `52-implement-stack-using-queues.ipynb` — Implement Stack using Queues
   Why here: complementary inversion problem; quick reinforcement without much overlap.
5. `53-design-linked-list.ipynb` — Design Linked List
   Why here: direct pointer/index bookkeeping practice with sentinel-node edge cases.
6. `54-design-circular-queue.ipynb` — Design Circular Queue
   Why here: fixed-capacity queue, wraparound indexing, and full-vs-empty state handling.
7. `55-design-circular-deque.ipynb` — Design Circular Deque
   Why here: extends circular queue into both-end updates.
8. `56-insert-delete-getrandom-o1.ipynb` — Insert Delete GetRandom O(1)
   Why here: high-ROI hash map + dynamic array pattern.
9. `57-design-twitter.ipynb` — Design Twitter
   Why here: capstone design problem combining hash maps, sets, ordering, and heap-style feed merging.

## Ordering Principle

- Starts with easier hash/stack/queue fundamentals.
- Spreads practice across hashing, linked lists, ring buffers, randomized sets, and multi-structure system design.
- Avoids problems already present in `Done`, `Todo`, and `random-practice/data-structures`.

## What Each New Notebook Trains

- `49-design-hashset.ipynb`
  Trains: bucket indexing, collisions, membership checks, insert/remove invariants.
  Think with: hash buckets, direct addressing vs chaining, load distribution.
  Applications: dedup sets, fast access-control membership checks, seen-item filters.
- `50-design-hashmap.ipynb`
  Trains: key-value storage, overwrite semantics, deletion from collision chains.
  Think with: mapping identity to state, update-in-place behavior, collision resolution.
  Applications: caches, config maps, counters, object registries.
- `51-implement-queue-using-stacks.ipynb`
  Trains: amortized analysis and reversing access order.
  Think with: “one structure can simulate another if I delay work.”
  Applications: buffering, task pipelines, inbox/outbox queue patterns.
- `52-implement-stack-using-queues.ipynb`
  Trains: interface emulation and operation tradeoffs.
  Think with: “what operation do I want cheap, and what can I afford to pay for?”
  Applications: adapter layers, constrained APIs, wrapping limited primitives.
- `53-design-linked-list.ipynb`
  Trains: pointer rewiring, sentinel nodes, index traversal edge cases.
  Think with: local pointer updates instead of array shifting.
  Applications: eviction lists, undo chains, intrusive lists, playlist/editor cursors.
- `54-design-circular-queue.ipynb`
  Trains: ring buffers, modulo indexing, full-vs-empty state tracking.
  Think with: fixed-capacity streaming storage.
  Applications: telemetry buffers, producer-consumer queues, packet/audio buffers.
- `55-design-circular-deque.ipynb`
  Trains: constant-time insertion and removal at both ends.
  Think with: symmetric front/back operations under bounded capacity.
  Applications: schedulers, sliding windows, browser/history style structures.
- `56-insert-delete-getrandom-o1.ipynb`
  Trains: hash map + array composition, swap-with-last deletion.
  Think with: “I need both fast lookup and fast random access.”
  Applications: randomized load distribution, sampling active items, game/entity pools.
- `57-design-twitter.ipynb`
  Trains: combining maps, sets, timestamps, and top-k retrieval.
  Think with: merging many small sorted sources instead of globally sorting everything.
  Applications: social feeds, notification aggregation, event timelines.

## Notebook Order

1. `49-design-hashset.ipynb` — Design HashSet DONE
2. `50-design-hashmap.ipynb` — Design HashMap DONE
3. `51-implement-queue-using-stacks.ipynb` — Implement Queue using Stacks DONE
4. `52-implement-stack-using-queues.ipynb` — Implement Stack using Queues
5. `53-design-linked-list.ipynb` — Design Linked List
6. `54-design-circular-queue.ipynb` — Design Circular Queue
7. `55-design-circular-deque.ipynb` — Design Circular Deque
8. `56-insert-delete-getrandom-o1.ipynb` — Insert Delete GetRandom O(1) `[#neetcode-150]` DONE
9. `57-design-twitter.ipynb` — Design Twitter `[#neetcode-150]` DONE
10. `58-group-anagrams.ipynb` — Group Anagrams `[#neetcode-150]` DONE
11. `59-largest-rectangle-in-histogram.ipynb` — Largest Rectangle in Histogram `[#neetcode-150]`
12. `60-simplify-path.ipynb` — Simplify Path
13. `61-online-stock-span.ipynb` — Online Stock Span
14. `62-number-of-recent-calls.ipynb` — Number of Recent Calls
15. `63-reverse-nodes-in-k-group.ipynb` — Reverse Nodes in k-Group `[#neetcode-150]`
16. `64-flatten-a-multilevel-doubly-linked-list.ipynb` — Flatten a Multilevel Doubly Linked List
17. `65-design-add-and-search-words-data-structure.ipynb` — Design Add and Search Words Data Structure `[#neetcode-150]`
18. `66-replace-words.ipynb` — Replace Words
19. `67-map-sum-pairs.ipynb` — Map Sum Pairs
20. `68-task-scheduler.ipynb` — Task Scheduler `[#neetcode-150]`
21. `69-sort-characters-by-frequency.ipynb` — Sort Characters By Frequency
22. `70-maximum-gap.ipynb` — Maximum Gap
23. `71-contains-duplicate-iii.ipynb` — Contains Duplicate III
24. `72-reduce-array-size-to-the-half.ipynb` — Reduce Array Size to The Half
25. `75-task-scheduler-ii.ipynb` — Task Scheduler II

## Bucket Sort Focus (New)

- `68-task-scheduler.ipynb`
  Why schedule relevant: greedy counting and cooldown-slot reasoning to minimize total intervals.
- `69-sort-characters-by-frequency.ipynb`
  Why bucket-sort relevant: frequency buckets over characters, then rebuild string from high to low counts.
- `70-maximum-gap.ipynb`
  Why bucket-sort relevant: bucket range partitioning enables linear-time max-gap search without full sorting.
- `71-contains-duplicate-iii.ipynb`
  Why bucket-sort relevant: value buckets plus sliding window can check near-duplicates in linear time.
- `72-reduce-array-size-to-the-half.ipynb`
  Why bucket-sort relevant: bucket frequencies-of-frequencies to greedily remove largest groups first.
- `75-task-scheduler-ii.ipynb`
  Why schedule relevant: hash map of last execution day plus simulation-by-jump for cooldown gaps.

## How To Think About Picking The Next Exercise

- If you keep missing constant-time lookup opportunities, pick a hashing problem.
- If you keep rescanning left/right neighbors, pick a stack or deque problem.
- If edge-case bugs happen during insert/delete, pick a linked-list design problem.
- If you need top-k or “most recent / smallest / largest so far”, pick a heap problem.
- If the problem keeps asking for prefixes or word matching, pick a trie problem.
- If the problem mixes operations and state transitions, prefer design problems over pure algorithm drills.

## Real-System Pattern Map

- Hash maps / hash sets
  Shows up in caches, indexes, registries, dedup pipelines, and rate-limit state.
- Linked lists
  Shows up when order changes frequently and local rewiring is cheaper than shifting arrays.
- Queues / deques / circular buffers
  Shows up in streaming systems, schedulers, networking buffers, and rolling windows.
- Heaps
  Shows up in ranking, task prioritization, top-k analytics, and feed assembly.
- Tries
  Shows up in autocomplete, routing/prefix matching, dictionaries, and search suggestions.
- Composite structures
  Shows up in real interview design questions because production systems usually need two or more structures at once.
