Here’s a **first-principles “problem-solving toolkit”** tuned for exactly this interview type (SQL + product feature + live reasoning). The goal is not tricks in isolation, but a **stable decomposition system you can reuse under ambiguity**.

I’ll frame everything as a **decision pipeline**:

> **Intent → model → transform → validate → adapt**

---

# 1. The universal decomposition trick (MOST IMPORTANT)

Any prompt is secretly:

> “Define a metric over a relational event system under ambiguity.”

So you always force 4 questions:

### (1) What is the atomic event?

* click?
* purchase?
* login?
* view?

### (2) What is the grain?

* 1 row = user?
* user-event?
* user-day?
* session?

### (3) What transformation is needed?

* filter (WHERE)
* collapse (GROUP BY)
* compare across time/entities (WINDOW)

### (4) What is the output meaning?

* ranking?
* rate?
* count?
* trend?

This is the **core funnel filter** for all problems.

---

# 2. Schema-first thinking (hidden separator skill)

Most candidates jump into SQL too early. Strong candidates do:

> “I cannot reason about SQL until I define the table grain.”

### Trick:

Before coding, always say:

> “Let me define the minimal schema that makes this query possible.”

Then define:

* entity tables (users, products)
* event tables (actions over time)
* relationship tables (joins)

---

# 3. Grain alignment trick (prevents 80% of bugs)

### Rule:

> All joins must preserve or intentionally change grain.

### Mental check:

Before joining, ask:

* “Will this multiply rows?”
* “Do both tables represent same unit?”

### Example failure:

user_events × user_profiles → OK
user_events × user_events → danger (explosion)

### Safe framing:

> “I’ll aggregate to the same grain before joining to avoid duplication.”

---

# 4. “Collapse vs Expand” decision framework

Every SQL operation is one of:

### Collapse (reduce rows)

* GROUP BY
* aggregation
* DISTINCT

### Expand (add context)

* JOIN
* window functions
* subqueries

### Trick:

> Always decide whether your step should collapse or expand BEFORE writing SQL.

This prevents structural confusion.

---

# 5. Window function intuition (the biggest separator)

Instead of memorizing syntax, use this model:

> “Each row gets a local universe of related rows.”

### When to use:

* ranking → “who is best within group?”
* time comparison → “what changed before/after?”
* rolling stats → “trend over time”

### Mental mapping:

| Need               | Tool       |
| ------------------ | ---------- |
| order within group | ROW_NUMBER |
| relative rank      | RANK       |
| previous value     | LAG        |
| running metric     | SUM OVER   |

### Framing line:

> “I’ll switch to a window function because I need row-level context without collapsing the dataset.”

---

# 6. Ambiguity handling trick (this is interview gold)

When spec is vague:

### Never guess silently.

Instead:

> “There are two plausible interpretations. I’ll pick one and structure the solution so we can extend it.”

Then explicitly branch:

* Version A: strict interpretation
* Version B: relaxed interpretation

This shows **product thinking, not just SQL skill**

---

# 7. Iterative construction trick (pair programming core)

Don’t build final query in one shot.

Use:

### Step ladder:

1. base table
2. filter
3. join
4. aggregate
5. window layer

Say:

> “I’ll first build the base dataset, then incrementally add logic.”

This aligns with how real systems are built.

---

# 8. Debugging framework (when output is wrong)

Instead of random fixes, always check:

### 1. Grain mismatch

> “Did I accidentally duplicate rows via join?”

### 2. Missing filter scope

> “Am I aggregating before filtering or after?”

### 3. Time window error

> “Did I define time boundaries correctly?”

### 4. Aggregation level error

> “Am I mixing per-user and per-event logic?”

---

# 9. Product-feature translation trick (VERY IMPORTANT)

When given vague feature like:

> “Find trending users”

Convert into:

### Step 1: define signal

* likes? views? posts?

### Step 2: define metric

* sum(activity)
* growth rate
* velocity

### Step 3: define window

* last 24h? 7d?

### Step 4: define comparison

* relative ranking? threshold?

### Final framing:

> “Trending = rate of change of engagement over a fixed time window.”

This is what they are actually testing.

---

# 10. “Minimal SQL plan” trick (fast execution)

Before writing SQL, always produce:

```
1. base table
2. filter conditions
3. grouping key
4. aggregation metric
5. optional ranking/window
```

This avoids chaotic coding.

---

# 11. Mental model for correctness vs readability tradeoff

In interviews:

* correctness > elegance
* clarity > optimization

### Safe statement:

> “I’ll prioritize correctness and clarity first, then optimize structure if needed.”

---

# 12. What interviewers are *actually scoring*

Under the hood:

### They are measuring:

1. **Can you define the system correctly?**
2. **Can you maintain grain consistency?**
3. **Can you reason under ambiguity?**
4. **Can you collaborate under change?**
5. **Can you explain simply while coding?**

SQL is just the medium.

---

# 13. One unified “gold script” you can reuse

When stuck, say:

> “Let me first define the event grain and expected output.
> Then I’ll build the base dataset, and incrementally transform it using either aggregation or window functions depending on whether we need to collapse or preserve row-level context.”

This single framing keeps you aligned across almost all tasks.

---

If you want, I can next:

* run a **full simulated Solve final interview (SQL + interruptions + ambiguity changes)**
* or give you a **“cheat sheet of 20 canonical SQL patterns with templates you can reuse instantly in live coding”**
