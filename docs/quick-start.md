# Quick start

## The model

A [`Figure`](../src/figure/mod.rs) holds the elements of one problem and the
conditions that are known about them:

- **elements** — points, full lines, segments, rays and measurable objects (a
  length or an angle). Every element gets an `ID`, a `u32` index inside the
  figure.
- **conditions** — facts such as "these two lines are parallel" or "this sum of
  lengths equals that one", each with the proof that established it.
- **proofs** — every condition is either assumed (empty proof) or derived from
  earlier conditions by one transfer step. `prove::solve::solve` keeps applying
  the transfers of a rule until the target condition is proven.

Coordinates and measures are exact rational numbers, so nothing is lost to
floating point.

## Example

Proving `AB ∥ EF` from `AB ∥ CD` and `CD ∥ EF`:

```rust
use proverrr_core::figure::{Figure, PanLine, Point};
use proverrr_core::prove::condition::{Condition, ParallelCondition};
use proverrr_core::prove::solve::{Rule, solve};
use proverrr_core::ratio::Ratio;

fn main() {
    let mut figure = Figure::new();

    let a = figure.add_point(Point::new(Ratio::from(0), Ratio::from(0)));
    let b = figure.add_point(Point::new(Ratio::from(2), Ratio::from(0)));
    let c = figure.add_point(Point::new(Ratio::from(0), Ratio::from(1)));
    let d = figure.add_point(Point::new(Ratio::from(2), Ratio::from(1)));
    let e = figure.add_point(Point::new(Ratio::from(0), Ratio::from(2)));
    let f = figure.add_point(Point::new(Ratio::from(2), Ratio::from(2)));

    let ab = PanLine::segment(a, b);
    let cd = PanLine::segment(c, d);
    let ef = PanLine::segment(e, f);

    figure.add_condition(Condition::Parallel(ParallelCondition::new(ab, cd)));
    figure.add_condition(Condition::Parallel(ParallelCondition::new(cd, ef)));

    let target = Condition::Parallel(ParallelCondition::new(ab, ef));
    let proof = solve(&mut figure, &target, &Rule::new()).expect("target is provable");
    println!("{:?}", proof.describe);
}
```

This prints `[ParallelTransitive]`: the rule needed a single step, and the proof
of the target records it.

Next: [Figure API](figure.md) and [Conditions and proofs](proofs.md).
