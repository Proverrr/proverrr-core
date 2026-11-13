# Conditions and proofs

## Conditions

- `Condition::Parallel(ParallelCondition)` — two `PanLine`s are parallel. The
  pair is stored in a fixed order, so `a ∥ b` and `b ∥ a` are the same condition.
- `Condition::Sum(SumCondition)` — `SumCondition` states that the sum of the
  objects in its `a` field equals the sum of `b_objects` plus `b_const`. Lengths
  and angles are both `Object`s, so one condition covers both.

## Proofs

`Proof` keeps three parallel vectors: `conditions[i]` is proven from
`fathers[i]` by the step `describe[i]`.

- A condition added with `add_condition` is assumed, so its proof is empty.
- A condition added with `add_derived` gets the proofs of its fathers merged in,
  followed by one step for itself, so the proof carries the whole derivation.

## Solving

`solve(&mut figure, &target, &rule)` applies the rule until `target` is known or
nothing new appears. It returns the proof of `target`, or `None`.

A `Rule` says which transfers to run; `Rule::new()` enables every one of them:

| Field | Transfer | Step it records |
| --- | --- | --- |
| `parallel_transitive` | `ParallelTransfer` | `a ∥ b` and `b ∥ c` give `a ∥ c` |

## Status

Only parallel transitivity is implemented so far: `Rule` has a single switch,
`parallel_transitive`, driven by `ParallelTransfer`. Sum conditions, angle
relations and congruence rules are not derived yet.
