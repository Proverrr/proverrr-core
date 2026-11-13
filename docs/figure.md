# Figure API

`Figure` ([source](../src/figure/mod.rs)) is the container for one problem: it
owns every element and every known condition, and hands out an `ID` for each of
them. Elements are added by value, so a line or a segment is built from the IDs
of its points.

## Add elements

| Method | Returns | Meaning |
| --- | --- | --- |
| `add_point(Point)` | `ID` | a point with exact coordinates |
| `add_line(Line)` | `ID` | a full line through two points |
| `add_segment(Segment)` | `ID` | the part of a line between two points |
| `add_ray(Ray)` | `ID` | an origin and the point it goes through |
| `add_object(Object)` | `ID` | a measurable length or angle |
| `add_condition(Condition)` | `ID` | assumes a condition, with an empty proof |

`Figure::new()` creates an empty figure.

## Read elements

| Method | Returns |
| --- | --- |
| `point(id)`, `line(id)`, `segment(id)`, `ray(id)`, `object(id)`, `condition(id)` | `Option<&T>`, `None` for an unknown ID |
| `points()`, `lines()`, `segments()`, `rays()`, `objects()`, `conditions()`, `proofs()` | `&[T]`, in insertion order |

## Reason about conditions

| Method | Returns |
| --- | --- |
| `index_of(&Condition)` | `Option<usize>`, the position among the known conditions |
| `proof_of(&Condition)` | `Option<&Proof>`, `None` if the figure does not know it |
| `add_derived(Condition, Vec<Condition>, TransferType)` | `bool`, `false` if the condition is already known |

`proofs()[i]` belongs to `conditions()[i]`.

`add_derived` merges the proofs of the fathers into the new proof, so the fathers
are expected to be known already; unknown ones are skipped.

## Elements

- `Point { x, y }` — exact rational coordinates; `Point::origin()` is `(0, 0)`.
- `Line { a, b }` — a full line through the points `a` and `b`.
- `Segment { a, b }` — the part of a line between `a` and `b`.
- `Ray { origin, through }` — starts at `origin` and goes through `through`.
- `Object { kind, points }` — a measurable quantity, either
  `Object::length(a, b)` or `Object::angle(a, b, c)` with `b` as the vertex.
- `PanLine` — a line, segment or ray; conditions accept all three.
  `PanLine::line(a, b)`, `PanLine::segment(a, b)` and `PanLine::ray(a, b)` build
  one from point IDs.

`ID`s are independent per kind: point `0` and segment `0` are different elements.
