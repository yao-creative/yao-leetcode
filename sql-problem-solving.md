# SQL Problem Solving Guide

Use this when solving any SQL interview problem under time pressure.

## 1. Start With Output Grain

Before writing SQL, define:
- What does **1 output row** represent?
- What are the exact output columns?
- What is sorting requirement?

If grain is wrong, the whole query is wrong.

## 2. Classify the Problem Type

Tag the question quickly:
- `JOIN`: combine tables
- `ANTI-JOIN`: find missing rows (`LEFT JOIN ... IS NULL` or `NOT EXISTS`)
- `GROUP BY`: aggregate metrics
- `WINDOW`: rank/compare without collapsing rows
- `CTE`: multi-step transformation
- `TIME-SERIES`: date buckets, rolling logic, cohort-like metrics

Pick the pattern first, then write SQL.

## 3. Build in This Order

1. `SELECT` final columns only  
2. `FROM` + `JOIN` keys  
3. `WHERE` row-level filters  
4. `GROUP BY` / aggregates (if needed)  
5. `HAVING` aggregate filters  
6. window columns (if needed)  
7. final `ORDER BY`

## 4. Core Templates

### A) Anti-join
```sql
SELECT a.id
FROM A a
LEFT JOIN B b ON b.a_id = a.id
WHERE b.a_id IS NULL;
```

### B) Top-N per group (window)
```sql
WITH ranked AS (
  SELECT
    t.*,
    ROW_NUMBER() OVER (PARTITION BY group_col ORDER BY metric DESC) AS rn
  FROM t
)
SELECT *
FROM ranked
WHERE rn <= 3;
```

### C) Aggregate metric
```sql
SELECT key_col, COUNT(*) AS cnt
FROM t
WHERE condition
GROUP BY key_col
HAVING COUNT(*) >= 2;
```

### D) Stepwise CTE approach
```sql
WITH step1 AS (...),
step2 AS (...),
final AS (...)
SELECT *
FROM final;
```

## 5. Sanity Check Before Submit (30 seconds)

- Join duplicates accidentally inflating counts?
- `WHERE` vs `HAVING` used correctly?
- Null handling needed (`COALESCE`, `IS NULL`)?
- Ranking function correct (`ROW_NUMBER` vs `RANK`)?
- Date boundary correct (`>=`, `<`, inclusive month)?

## 6. Common Mistakes

- Aggregating before fixing join multiplicity
- Using `COUNT(col)` when nulls should count/not count
- Filtering aggregate condition in `WHERE` instead of `HAVING`
- Choosing wrong partition/order in window functions
- Not clarifying tie behavior for ranking questions

## 7. Interview Narration Script

Use this out loud:

1. “Output grain is one row per ___.”  
2. “I’ll solve with ___ pattern (join/group/window/cte).”  
3. “I’ll build in steps: filter -> transform -> final select.”  
4. “I’ll quickly validate duplicates, nulls, and tie/date edge cases.”

This shows structured thinking even before final query is perfect.

## 8. Rush Mode Strategy

When cramming:
1. Do one problem from each pattern type first (breadth).
2. Keep an error log with pattern + mistake.
3. Redo only weak patterns (usually window + CTE + time-series).
4. Final pass: solve 5-8 mixed problems aloud without notes.

