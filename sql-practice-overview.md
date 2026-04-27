Here is a **high-signal LeetCode set tailored exactly to what your interview is testing (SQL + product reasoning + live discussion)**.

I’ll structure this as a **skill funnel** so you don’t randomly grind problems.

---

# 1. Core SQL foundation (must be automatic)

These are “syntax reflex” problems — you should be able to solve them without thinking.

### JOIN + GROUP BY basics

- Combine Two Tables
- Customers Who Never Order
- Employees Earning More Than Their Managers
- Duplicate Emails
- Rising Temperature

Why these matter:
→ they test whether your mental model of **relational joins + filtering order** is stable

---

# 2. Aggregation + business metrics (very likely in your interview)

These map directly to “product feature task” style questions.

- Department Highest Salary
- Department Top Three Salaries
- Not Boring Movies
- Average Time of Process per Machine
- Game Play Analysis I & II

What you're practicing here:

> turning raw event tables → business metrics

Key mental skill:

- define grain
- aggregate correctly
- avoid duplication bugs

---

# 3. Window functions (HIGH PRIORITY — this is the separator)

This is the most important cluster for your interview type.

- Rank Scores
- Nth Highest Salary
- Department Top Three Salaries (again, but window version)
- Consecutive Numbers
- Human Traffic of Stadium
- Game Play Analysis IV

Core abstraction you’re building:

> comparing rows within partitions (not collapsing data)

If you only master one thing, it’s this:

- `ROW_NUMBER()`
- `RANK()`
- `LAG() / LEAD()`
- rolling sums

This is what interviewers use to test “senior-ish SQL thinking” ([Shadecoder](https://www.shadecoder.com/blogs/sql-interview-questions?utm_source=chatgpt.com))

---

# 4. CTE + multi-step reasoning (pair programming focus)

These simulate real interview discussion loops.

- Department Top Three Salaries (CTE version)
- Trips and Users
- Find Median Given Frequency of Numbers
- Human Traffic of Stadium
- Consecutive Numbers

What this tests:

> can you decompose a messy problem into staged transformations?

This is exactly what your interview description means by:
“discuss while coding”

---

# 5. Time-series / real-world analytics (very likely for product task)

This is closest to “feature implementation task”.

- Monthly Transaction I / II
- Restaurant Growth
- Game Play Analysis III / IV
- Immediate Food Delivery II
- Product Price at a Given Date

Core concept:

> turning event logs into time-aware product metrics

This is where they implicitly test:

- cohort thinking
- rolling windows
- business interpretation

---

# 6. “Tricky reasoning” problems (for discussion quality)

These are less about SQL syntax, more about clarity under ambiguity.

- Exchange Seats
- Friend Requests II
- Tree Node
- Managers with at least 5 direct reports
- Find the Team Size

Why they matter:

> they force you to justify modeling decisions out loud

---

# 7. Minimal “target set” (if time is tight)

If you only do ~12–15 problems, do this:

### Must do:

- Customers Who Never Order
- Employees Earning More Than Their Managers
- Rank Scores
- Department Top Three Salaries
- Consecutive Numbers
- Game Play Analysis IV
- Nth Highest Salary
- Human Traffic of Stadium
- Monthly Transaction I

This subset covers ~80% of interview signal space.

---

# 8. How to practice (important for your interview type)

Don’t solve like LeetCode grinding.

For EACH problem, follow this loop:

### Step 1: schema inference

- what is a row?
- what is event vs state?

### Step 2: define grain

> “1 row = user/day? user/event? user/session?”

### Step 3: choose transformation type

- join?
- aggregation?
- window function?
- multi-step CTE?

### Step 4: narrate tradeoffs

> “I’ll use window functions here because it avoids collapsing granularity”

This last step is what they explicitly care about.

---

# 9. Hidden pattern across all problems

All of them collapse into 4 abstractions:

1. **Relational linking (JOINs)**
2. **Compression (GROUP BY)**
3. **Context-aware row comparison (WINDOWS)**
4. **Multi-stage decomposition (CTEs)**

Everything else is syntax.

---

If you want, I can simulate a **real Solve-style SQL interview prompt** where:

- I give a messy product requirement
- you design schema
- then we iteratively build SQL while I interrupt and change requirements (like real pair programming)

That’s the closest thing to your actual final round.



