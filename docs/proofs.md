# Conditions and proofs

## Conditions

- `Condition::Parallel(ParallelCondition)` — two `PanLine`s are parallel. The
  pair is stored in a fixed order, so `a ∥ b` and `b ∥ a` are the same condition.
- `Condition::Sum(SumCondition)` — `SumCondition` states that the sum of the
  objects in its `a` field equals the sum of `b_objects` plus `b_const`. Lengths
  and angles are both `Object`s, so one condition covers both.
- `Condition::Congruent(CongruentCondition)` — two triangles are congruent. Each
  triangle is a `Triangle` of three points, and the vertices correspond by
  position (`a.a`↔`b.a`, `a.b`↔`b.b`, `a.c`↔`b.c`). The two triangles are stored
  in a fixed order, so the condition is symmetric.
- `Condition::Side(SideCondition)` — two points lie on the same side (`same =
  true`) or on opposite sides (`same = false`) of a line. It supplies the spatial
  configuration that abstract points alone cannot express.
- `Condition::Collinear(CollinearCondition)` — three points lie on one line,
  stored in a fixed order.

### Congruence premises

The congruence transfers read their premises from `Sum` conditions, using angles
in degrees:

- **equal lengths / angles** — `Sum([x], [y], 0)`, e.g.
  `SumCondition::new(vec![Object::length(a, b)], vec![Object::length(c, d)], Ratio::from(0))`.
- **right angle** — `Sum([angle], [], 90)`, e.g.
  `SumCondition::new(vec![Object::angle(a, b, c)], vec![], Ratio::from(90))`.

A side or angle shared by both triangles counts as equal to itself. Two right
angles count as equal even without a direct equality condition. Equalities are
read as given; transitivity of `Sum` is not derived yet.

### Parallel angle premises

The parallel angle transfers reason about two parallel lines cut by a
transversal. Points are abstract, so the configuration is asserted explicitly:

- `Parallel(l1, l2)` — the two parallel lines, each a `PanLine` through two points.
- `Side(t, x, y, same)` — the transversal `t` passes through the intersection
  points `p` (on `l1`) and `q` (on `l2`); `x` and `y` are the *other* points of
  `l1` and `l2`. `same` says whether `x` and `y` lie on the same side of `t`.
- For corresponding angles, an exterior point `z` on the transversal is given by
  `Collinear(p, q, z)` together with `Side(l2, z, p, false)` (beyond `q`).

The interior angles are `angle(x, p, q)` at `p` and `angle(y, q, p)` at `q`. From
the premises the transfers derive, in degrees:

- **alternate interior angles equal** — `x`, `y` on opposite sides:
  `Sum([angle(x,p,q)], [angle(y,q,p)], 0)`.
- **same-side interior angles supplementary** — `x`, `y` on the same
  side: `Sum([angle(x,p,q), angle(y,q,p)], [], 180)`.
- **corresponding angles equal** — an interior and an exterior angle on
  the same side: `Sum([angle(x,p,q)], [angle(y,q,z)], 0)`.

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
| `parallel_corresponding` | `ParallelAngleTransfer` | corresponding angles are equal |
| `parallel_alternate_interior` | `ParallelAngleTransfer` | alternate interior angles are equal |
| `parallel_co_interior` | `ParallelAngleTransfer` | same-side interior angles are supplementary |
| `congruent_sss` | `CongruentTransfer` | three pairs of equal sides give congruent triangles |
| `congruent_sas` | `CongruentTransfer` | two sides and the included angle |
| `congruent_asa` | `CongruentTransfer` | two angles and the included side |
| `congruent_aas` | `CongruentTransfer` | two angles and a non-included side |
| `congruent_hl` | `CongruentTransfer` | right triangles: equal hypotenuse and one leg |

The congruence transfers enumerate the triangles formed by the points that appear
in the premises, try every vertex correspondence, and record the matching parts
as the fathers of the new condition.

## Status

Parallel transitivity, the parallel angle properties (corresponding, alternate
interior, same-side interior) and the five triangle congruence criteria (SSS,
SAS, ASA, AAS, HL) are implemented. Transitivity of `Sum` conditions and other
angle relations are not derived yet.
