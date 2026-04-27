I’ll frame this as a **system of relational transformation tools**, because that’s what they actually test: not definitions, but *when your mental model switches between tools*.

---

# 1. Window functions (core concept)

## 🔷 First-principles idea

A window function lets you compute something:

> **across a “related set of rows” while still keeping each row intact**

So unlike `GROUP BY` (which collapses rows), window functions:

* **preserve row-level detail**
* but add **context from other rows**

---

## 🔷 Mental model

Think:

> Each row gets a *view into its neighborhood* (its “window”).

Defined by:

* `PARTITION BY` → which group
* `ORDER BY` → ordering inside group

---

## 🔷 Generic form

```sql
function(...) OVER (
  PARTITION BY ...
  ORDER BY ...
)
```

---

## 🔷 What it replaces

Without window functions you’d do:

* subqueries
* self-joins
* repeated aggregations

Window functions simplify all of that.

---

# 2. LAG / LEAD (when and why)

## 🔷 Core idea

They let a row “look backward or forward” in time/order.

* `LAG()` → previous row
* `LEAD()` → next row

---

## 🔷 When you use LAG

Use it when you need:

### 1. Change over time

Example:

> “Did a user increase or decrease activity?”

### 2. Difference between events

Example:

> “Time between logins”

### 3. Sequence analysis

Example:

> “Was this event repeated consecutively?”

---

## 🔷 Example

### Table: user_activity

| user | day | actions |
| ---- | --- | ------- |
| A    | 1   | 10      |
| A    | 2   | 15      |
| A    | 3   | 12      |

### Query:

```sql
SELECT
  user,
  day,
  actions,
  LAG(actions) OVER (PARTITION BY user ORDER BY day) AS prev_actions,
  actions - LAG(actions) OVER (PARTITION BY user ORDER BY day) AS delta
FROM user_activity;
```

### Output logic:

| day | actions | prev | delta |
| --- | ------- | ---- | ----- |
| 1   | 10      | NULL | NULL  |
| 2   | 15      | 10   | +5    |
| 3   | 12      | 15   | -3    |

---

## 🔷 Key intuition

> LAG converts a dataset from “independent rows” → “state transition system”

---

# 3. Join failure modes (VERY IMPORTANT in interviews)

This is where most SQL bugs come from.

---

## 🔴 Failure mode 1: row explosion

### Cause:

Many-to-many join without controlling grain.

### Example:

### users

| user |
| ---- |
| A    |

### events

| user | event |
| ---- | ----- |
| A    | click |
| A    | view  |

### BAD JOIN:

```sql
SELECT *
FROM users u
JOIN events e
ON u.user = e.user;
```

### Result:

* OK here (1→many)
* but becomes dangerous if both sides are many-to-many

---

## 🔴 Failure mode 2: accidental duplication

If you join:

* users × orders
* then join × payments

You can multiply rows exponentially.

### Symptom:

* inflated counts
* incorrect aggregates

### Fix:

> pre-aggregate before joining

---

## 🔴 Failure mode 3: missing rows

Caused by wrong join type (INNER JOIN instead of LEFT JOIN)

---

# 4. INNER JOIN vs LEFT JOIN (core distinction)

---

## 🔷 INNER JOIN

> Returns ONLY matching rows

### Think:

> “intersection of sets”

### Use when:

* you only care about valid matches
* filtering irrelevant data

---

## 🔷 LEFT JOIN

> Keeps ALL rows from left table, even if no match

### Think:

> “preserve primary dataset, enrich if possible”

---

## 🔷 Example

### users

| id |
| -- |
| A  |
| B  |

### orders

| id |
| -- |
| A  |

---

### INNER JOIN:

```sql
A only
```

### LEFT JOIN:

```sql
A, B (B has NULL order)
```

---

## 🔷 Interview rule:

> Use LEFT JOIN when the left table defines the population you care about.

Example:

* “all users and their activity” → LEFT JOIN
* “only users with orders” → INNER JOIN

---

# 5. Subquery vs CTE

These are both ways to structure multi-step SQL, but they differ in **intent and readability**.

---

## 🔷 Subquery

### Definition:

A query inside another query.

### Example:

```sql
SELECT *
FROM (
  SELECT user, COUNT(*) AS cnt
  FROM events
  GROUP BY user
) t
WHERE cnt > 10;
```

### Use when:

* small logic
* one-off computation
* simple nesting

---

## 🔷 CTE (Common Table Expression)

### Definition:

Named intermediate step using `WITH`

### Example:

```sql
WITH user_counts AS (
  SELECT user, COUNT(*) AS cnt
  FROM events
  GROUP BY user
)
SELECT *
FROM user_counts
WHERE cnt > 10;
```

---

## 🔷 Why CTE is preferred in interviews

Because it:

### 1. Makes reasoning explicit

You can say:

> “Step 1 does aggregation, step 2 filters result”

### 2. Supports multi-stage logic

Useful for:

* event pipelines
* feature computation
* window layering

---

## 🔷 Mental model difference

| Type     | Mental model      |
| -------- | ----------------- |
| Subquery | inline expression |
| CTE      | pipeline stage    |

---

## 🔷 Interview rule:

> Use CTE when you want to explain your thinking step-by-step.

---

# 6. Unified mental map (how all of this connects)

You can think of SQL reasoning as:

```
Raw events
   ↓
(CTE stage 1: define grain)
   ↓
(join enrichment)
   ↓
(aggregation OR window function)
   ↓
(final metric)
```

And tool usage:

| Need           | Tool            |
| -------------- | --------------- |
| compare rows   | window function |
| previous/next  | LAG / LEAD      |
| reduce data    | GROUP BY        |
| enrich data    | JOIN            |
| stepwise logic | CTE             |
| inline logic   | subquery        |

---

# 7. One-line intuition summary

* **Window functions** → “context without collapsing rows”
* **LAG** → “compare with previous state”
* **INNER JOIN** → “only matches”
* **LEFT JOIN** → “preserve base set”
* **CTE** → “structured thinking pipeline”
* **Subquery** → “inline computation shortcut”

---

If you want next step, I can:

* give you **10 real interview SQL patterns using window + joins (with answers)**
* or run a **mock interview where I deliberately trigger join bugs and test your debugging reasoning live**
