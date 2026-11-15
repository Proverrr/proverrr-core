# Proverrr Core
**Currently in Experimental Stage, Use It at Your Own Risk**\
A light-weight, Rust library for solving geometric problems.

## Documentation

See [docs/index.md](docs/index.md) for detail.

## Build and test

```sh
cargo build
cargo test
```

## FFI

The library also compiles as a C-ABI shared library (`cdylib`): `proverrr_core.dll`
(Windows), `libproverrr_core.so` (Linux) or `libproverrr_core.dylib` (macOS).

All functions are prefixed `proverrr_` and exported from the shared library:

| Function | Purpose |
|---|---|
| `proverrr_figure_new` / `proverrr_figure_free` | Create / free a figure |
| `proverrr_figure_add_point(fig)` → id | Add a point, returns its ID (`-1` on null) |
| `proverrr_condition_parallel(fig, x, y)` | `x ∥ y`, `x`/`y` are `LineSpec` values |
| `proverrr_condition_side(fig, t, a, b, same)` | `a`, `b` on the same/opposite side of `t` (`same != 0`) |
| `proverrr_condition_collinear(fig, a, b, c)` | `a`, `b`, `c` collinear |
| `proverrr_condition_congruent(fig, a1, b1, c1, a2, b2, c2)` | Triangles congruent |
| `proverrr_condition_angles_equal(fig, a, b, c, d, e, f)` | `angle(abc) == angle(def)` |
| `proverrr_condition_free` / `proverrr_figure_add_condition(fig, cond)` | Free a condition / assume it in a figure (consumes it) |
| `proverrr_proof_solve(fig, target)` → proof | Prove `target` (consumes it), `null` if unprovable |
| `proverrr_proof_step_count` / `proverrr_proof_step` / `proverrr_proof_free` | Read the proof steps |
| `proverrr_string_free` | Free a string returned by `proverrr_proof_step` |

`LineSpec` is `{ uint32_t kind; uint32_t a; uint32_t b; }` where `kind` is
`0` = full line, `1` = segment, `2` = ray, and `a`/`b` are point IDs from
`proverrr_figure_add_point`. Every function accepts null where a handle is
expected and returns a neutral value instead of crashing; conditions built
from point IDs the figure does not know are rejected with `null`.

C usage sketch:

```c
proverrr_line x = {1, 0, 1};          /* segment a-b */
proverrr_line y = {1, 2, 3};          /* segment c-d */
proverrr_condition *xy = proverrr_condition_parallel(fig, x, y);
proverrr_figure_add_condition(fig, xy);            /* consumes xy */
proverrr_condition *target = proverrr_condition_parallel(fig, x, (proverrr_line){1, 4, 5});
proverrr_proof *proof = proverrr_proof_solve(fig, target);  /* consumes target */
const char *step = proverrr_proof_step(proof, 0);  /* e.g. "ParallelTransitive" */
proverrr_string_free(step);
proverrr_proof_free(proof);
proverrr_figure_free(fig);
```