# Competitive Programming Framing

To get good at this, you have to build a mental "translation dictionary." When competitive programmers read a problem, their eyes gloss over the story. They scan for trigger words that map to algorithmic concepts.

Here is the cheat sheet for how to properly phrase and frame these problems, along with the specific tricks used to crack LeetCode Mediums and Hards.

---

## 1. Story-to-Frame Translation Dictionary

When you read the **story**, your brain needs to instantly translate it into the **frame**.

- **Story:** "Find the number of continuous subarrays that sum to `K`."
  - **Frame:** **Prefix sums with a hash map.** (You are looking for `PrefixSum[i] - K = PrefixSum[j]`.)
- **Story:** "Find the longest substring with at most `K` distinct characters."
  - **Frame:** **Sliding window with a frequency map.** (Expand right pointer to add, shrink left pointer when invalid.)
- **Story:** "You have a list of tasks, and some tasks must be done before others. Can you finish them all?"
  - **Frame:** **Cycle detection in a directed graph.** (Use topological sort / Kahn's algorithm.)
- **Story:** "Divide an array into `M` subarrays such that the maximum sum among them is minimized."
  - **Frame:** **Binary search on answer.** (Guess the max sum, then greedily check if you can split the array into `<= M` parts using that guess.)
- **Story:** "You have a bunch of overlapping meetings/intervals, find the minimum number of rooms needed."
  - **Frame:** **Line sweep / difference array.** (Sort all start times as `+1` and end times as `-1`, then find the peak running sum.)
- **Story:** "Find the next warmest day for each day in a list of temperatures."
  - **Frame:** **Monotonic stack.** (Keep a stack of indices that strictly decreases in temperature; pop and resolve when you find a warmer day.)

---

## 2. Core Tricks (Meta-Patterns)

These are the underlying mechanisms that separate Easy solutions from Hard solutions. They are about bypassing redundant work.

### Trick 1: The Contribution Technique (Math/Combinatorics)

- **The Trap:** A problem asks for the sum of the minimums of *all possible subarrays*. Brute-forcing this is `O(N^3)` or `O(N^2)`.
- **The Trick:** Instead of finding every subarray and calculating its minimum, ask: *"For a specific element `A[i]`, in how many subarrays is it the absolute minimum?"* If element `X` is the minimum in `Y` different subarrays, its contribution to the final total is simply `X * Y`. You just turned an `O(N^3)` problem into an `O(N)` problem.

### Trick 2: Binary Search on Answer

- **The Trap:** You are asked to find the exact optimal value (for example, "Find the minimum time to deliver all packages"), but finding it directly requires complex DP.
- **The Trick:** Is the answer space monotonic? (Meaning, if `10` hours is enough, `11` hours is definitely enough, but `9` hours might not be.) If yes, **guess the answer**. It is often easier to write an `isValid(guess)` function than a `findExactAnswer()` function. If `isValid(middle)` is true, search the lower half.

### Trick 3: Monotonic Stack / Queue

- **The Trap:** You need to repeatedly find the "next greater element" or maintain a sliding window maximum, which usually requires nested loops.
- **The Trick:** Maintain a stack that only goes in one direction (always increasing or always decreasing). Whenever a new element breaks the rule, you pop elements off the stack until the rule is restored. The act of popping an element solves the problem for that element in `O(1)` amortized time.

### Trick 4: State Representation (Dynamic Programming)

- **The Trap:** DP problems are confusing because the story has too many moving parts (for example, "You can buy a stock, sell it, but there's a 1-day cooldown, and a transaction fee").
- **The Trick:** Frame the problem as a state machine. Define exactly what "state" you can be in on any given day `i`.
  - State 0: Holding a stock.
  - State 1: Not holding, but in cooldown.
  - State 2: Not holding, ready to buy.
  - Once you frame the states, the DP transitions are just mapping how you move from one state to another (for example, `Hold[i] = max(Hold[i-1], Ready[i-1] - price)`).

### Trick 5: Union-Find (Disjoint Set)

- **The Trap:** You are given pairs of connected items ("A is friends with B", "B is friends with C") and need to constantly answer "Is A connected to X?" or "How many distinct friend groups are there?" DFS/BFS is too slow if the graph is constantly updating.
- **The Trick:** Use a Union-Find data structure. It groups items into sets with a "parent" representative. Checking if two items are in the same group, or merging two groups, becomes `O(1)` (using path compression).

## How to Apply This

Next time you open a LeetCode Hard, do not look at the input array immediately. Look at the prompt and play Mad Libs:

> "Okay, I need to find the **[maximum/minimum]** of **[contiguous chunks of data]** where the **[order matters/doesn't matter]**."

If order matters and it's contiguous chunks -> sliding window or prefix sums.
If it's maximum/minimum of an impossible-to-calculate value -> binary search on answer.
If order doesn't matter -> sort it immediately and look for a greedy pattern.

# Other LeetCode Tricks

Transitioning from LeetCode Easy/Medium to hard Mediums and Hards is one of the most frustrating plateaus in programming. You are moving from *implementing known algorithms* to *discovering hidden patterns*.

At this level, problems are deliberately obfuscated. The story is a distraction; your goal is to strip it away to reveal the raw data structure or mathematical model underneath.

Here is a structured framework for framing and breaking down competitive programming problems.

---

## Phase 1: Framing the Problem (De-obfuscation)

Framing is the art of translating the problem's "story" into pure computer science terminology.

### 1. Strip the Narrative

Immediately translate nouns and verbs into data structures and operations.

- *Story:* "You have a network of computers and some cables connecting them. What's the minimum cost to make sure every computer can talk to each other?"
- *Frame:* "Given an undirected, weighted graph, find the minimum spanning tree (Kruskal's or Prim's)."
- *Story:* "Find the longest substring without repeating characters."
- *Frame:* "Maintain a dynamically resizing window over an array while enforcing a uniqueness constraint." (Sliding window.)

### 2. Let Constraints Dictate the Algorithm

In competitive programming, the constraints (`N`) are essentially a cheat code that tells you the required time complexity, which drastically narrows down the possible algorithms.

- **`N <= 20`:** `O(2^N)` or `O(N!)` -> Backtracking, bitmask DP.
- **`N <= 100`:** `O(N^3)` -> Floyd-Warshall, 3D DP.
- **`N <= 10^3`:** `O(N^2)` -> Nested loops, 2D DP, dense graphs.
- **`N <= 10^5`:** `O(N log N)` or `O(N)` -> Sorting, binary search, sliding window, two pointers, segment trees, prefix sums.
- **`N >= 10^9`:** `O(log N)` or `O(1)` -> Math, binary search on answer.

### 3. Identify What Is *Actually* Being Asked

Are you looking for an absolute value (min/max)? A count (how many ways)? A boolean (is it possible)?

- *Min/Max:* Often greedy, dynamic programming, or binary search on answer.
- *How many ways:* Almost always dynamic programming or combinatorics.
- *Is it possible:* Graph traversal (BFS/DFS), Union-Find, or greedy.

---

## Phase 2: Breaking It Down (Deconstruction)

Once you've framed the problem, you need to break it down into solvable subproblems. Do not start coding yet.

### 1. Solve It Manually on Paper

Create a small, non-trivial test case and solve it like a human. Observe *how* your brain solved it. Did you naturally sort the items first? Did you discard obvious bad choices immediately? Your brain naturally applies optimizations; your job is to figure out what rules your brain followed and formalize them into code.

### 2. Find the Bottleneck ("If I Only Knew X...")

Ask yourself: "What is the single piece of missing information that makes this problem hard?"

- *Example:* In "Trapping Rain Water" (Hard), the bottleneck is knowing the highest bar to the left and the highest bar to the right of any given index.
- *Breakdown:* "If I pre-calculate the max height to the left for every element (an `O(N)` pass), and the max height to the right (another `O(N)` pass), calculating the water at index `i` becomes an `O(1)` math equation."

### 3. Relax a Constraint

If the problem is too hard, remove a rule to make it easy. Solve the easy version, then add the rule back.

- *Problem:* Find the maximum subarray sum of a circular array.
- *Relaxation:* How do I find it for a *normal* (non-circular) array? (Kadane's algorithm.)
- *Re-application:* Now, how does a circular array change things? It means the subarray could wrap around. A wrap-around max sum is exactly the total array sum minus the *minimum* non-wrapping subarray sum. You can use Kadane's again.

---

## Phase 3: The Stuck Protocol

When you stare at a Medium/Hard and have no idea what to do, cycle through these mental checks:

1. **Can I sort it?** Sorting destroys the original order. Does the order matter? If not, sort it. It often reveals greedy solutions or enables two pointers.
2. **Can I binary search the answer?** If the problem asks for a minimum or maximum threshold, guess the answer. Is it easy to check if a guessed answer `K` is valid? If checking `K` is `O(N)`, and the range of possible answers is `M`, you can solve it in `O(N log M)` by binary searching the answer space.
3. **Does solving a sub-prefix help?** If knowing the optimal answer for array elements `0` to `i-1` helps you quickly find the answer for element `i`, you are looking at dynamic programming.
4. **Are things dynamically changing?** If you need to repeatedly find the min/max as data enters and leaves, you need a heap (priority queue).

## How to Practice This Effectively

- **The 30-Minute Rule:** Give a problem 30 minutes of deep, focused effort on paper. If you do not have a solid mathematical approach by then, read the solution.
- **Do not just read code; read concepts:** When looking at a solution, do not look at the Python/C++ code first. Read the *idea*. Once you understand the idea, try to code it yourself.
- **The Why Journal:** Keep a spreadsheet. When you fail a problem, write down exactly *why* you failed. "I did not realize the constraints allowed for `O(N log N)`, so I did not think to sort." Pattern recognition builds here, not in mindless grinding.

What specific topic (for example, dynamic programming, graphs, binary search) usually gives you the most trouble when you try to figure out the underlying pattern?
