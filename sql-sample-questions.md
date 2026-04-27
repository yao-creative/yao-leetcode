Below is a **targeted set of ambiguous SQL interview questions** where each one is explicitly designed to test a specific concept you listed earlier (window functions, LAG, joins, CTEs vs subqueries, join failure modes, grain thinking).

For each:

* **Intent (what they’re really testing)**
* **Ambiguous question**
* **Design pattern label**
* **Sample solution (SQL pattern)**
* **Why this solution is correct (conceptual reason)**

---

# 1. Window function + LAG (time change detection)

## Intent

Test: **state transition reasoning + row-to-row comparison**

---

## Question

> “For each user, find days where their activity increased compared to the previous day.”

Assume:
`user_activity(user_id, activity_date, actions)`

---

## Design pattern label

**Temporal self-comparison via window offset (`LAG`)**

---

## Solution

```sql id="lag_1"
SELECT
  user_id,
  activity_date,
  actions,
  LAG(actions) OVER (
    PARTITION BY user_id
    ORDER BY activity_date
  ) AS prev_actions
FROM user_activity
WHERE actions > LAG(actions) OVER (
  PARTITION BY user_id
  ORDER BY activity_date
);
```

---

## Why this works

* `LAG` gives previous state in sequence
* `PARTITION BY user_id` isolates each timeline
* `ORDER BY date` defines temporal ordering
* Comparison turns static rows into **delta system**

Key insight:

> This converts a table into a **time-series difference model**

---

# 2. Window function ranking (top-N per group)

## Intent

Test: **partitioned ranking logic**

---

## Question

> “Find the top 3 most active users per day.”

`user_activity(user_id, activity_date, actions)`

---

## Design pattern label

**Top-N per partition with window ranking + outer filter**

---

## Solution

```sql id="rank_1"
WITH ranked AS (
  SELECT
    user_id,
    activity_date,
    actions,
    ROW_NUMBER() OVER (
      PARTITION BY activity_date
      ORDER BY actions DESC
    ) AS rnk
  FROM user_activity
)
SELECT *
FROM ranked
WHERE rnk <= 3;
```

---

## Why this works

* Partition by day → independent ranking per day
* Row_number avoids ties ambiguity
* CTE separates ranking logic from filtering

Key insight:

> This is **local ordering inside a slice of time**

---

# 3. Join failure mode (row explosion control)

## Intent

Test: **grain alignment + join safety**

---

## Question

> “Compute total actions per user including user profile data.”

Tables:

* `users(user_id, country)`
* `user_actions(user_id, action_count)`

---

## WRONG naive approach (intentional trap)

```sql
SELECT u.user_id, u.country, SUM(a.action_count)
FROM users u
JOIN user_actions a
ON u.user_id = a.user_id
GROUP BY u.user_id, u.country;
```

---

## Design pattern label

**Aggregate-then-join (grain alignment before dimensional join)**

---

## Correct solution (safe grain alignment)

```sql id="join_1"
WITH actions_agg AS (
  SELECT user_id, SUM(action_count) AS total_actions
  FROM user_actions
  GROUP BY user_id
)
SELECT
  u.user_id,
  u.country,
  a.total_actions
FROM users u
LEFT JOIN actions_agg a
ON u.user_id = a.user_id;
```

---

## Why this works

* Pre-aggregation removes duplication risk
* LEFT JOIN preserves full user population
* Grain is aligned: 1 row per user

Key insight:

> Always reduce many-to-one before joining to dimension tables

---

# 4. INNER vs LEFT JOIN (missing data reasoning)

## Intent

Test: **population preservation vs filtering logic**

---

## Question

> “Find all users and whether they have made a purchase.”

Tables:

* `users(user_id)`
* `orders(user_id, amount)`

---

## Design pattern label

**Population-preserving LEFT JOIN (coverage-first join strategy)**

---

## Solution

```sql id="join_2"
SELECT
  u.user_id,
  o.amount
FROM users u
LEFT JOIN orders o
ON u.user_id = o.user_id;
```

---

## Why this works

* We want ALL users → must preserve left table
* INNER JOIN would remove users with no orders (wrong intent)

Key insight:

> LEFT JOIN defines a **complete population view**, not filtered subset

---

# 5. CTE vs Subquery (multi-step reasoning)

## Intent

Test: **pipeline thinking vs inline computation**

---

## Question

> “Find users whose average daily activity is above overall average.”

---

## Design pattern label

**Stepwise analytic pipeline with chained CTEs**

---

## Solution (CTE)

```sql id="cte_1"
WITH user_daily AS (
  SELECT user_id, activity_date, SUM(actions) AS daily_actions
  FROM user_activity
  GROUP BY user_id, activity_date
),
user_avg AS (
  SELECT user_id, AVG(daily_actions) AS avg_actions
  FROM user_daily
  GROUP BY user_id
),
global_avg AS (
  SELECT AVG(daily_actions) AS overall_avg
  FROM user_daily
)
SELECT ua.user_id
FROM user_avg ua
CROSS JOIN global_avg ga
WHERE ua.avg_actions > ga.overall_avg;
```

---

## Why this works

* Breaks problem into **interpretable stages**
* Avoids nested logic confusion
* Makes debugging easy in pair programming
