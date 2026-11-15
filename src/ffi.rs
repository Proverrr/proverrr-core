//! C ABI exports for the compiled library (`.dll`, `.so`, `.dylib`).
//!
//! Every handle returned by these functions is a boxed Rust value that the
//! caller must release with the matching `proverrr_*_free` function. Handles
//! may not be used after being freed, and each handle belongs to exactly one
//! owner.
//!
//! Ownership rules:
//! - `proverrr_figure_add_condition` consumes the condition on success (`0`);
//!   on failure (`-1`) the caller still owns it and must free it.
//! - `proverrr_proof_solve` consumes the target condition whenever both
//!   arguments are non-null, whether or not a proof is returned. With a null
//!   figure the caller keeps ownership of the target.
//!
//! Every function accepts null where a handle is expected and returns a
//! neutral value (`null`, `-1` or `0`) instead of crashing; conditions built
//! from point IDs that the figure does not know are rejected with `null`.
//!
//! A [`LineSpec`] names one of the three pan-line variants: `kind` is `0` for
//! a full line, `1` for a segment and `2` for a ray.

use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

use crate::figure::{Figure, Object, PanLine, Point, Triangle};
use crate::prove::condition::{
    CollinearCondition, Condition, CongruentCondition, ParallelCondition, SideCondition,
    SumCondition,
};
use crate::prove::proof::Proof;
use crate::prove::solve::{Rule, solve};
use crate::ratio::Ratio;

/// A pan-line argument for the C ABI.
#[repr(C)]
pub struct LineSpec {
    /// `0` = full line, `1` = segment, `2` = ray.
    pub kind: u32,
    /// ID of the first point.
    pub a: u32,
    /// ID of the second point.
    pub b: u32,
}

/// Resolves `spec` against `fig`, rejecting unknown points and unknown kinds.
fn pan_line(fig: &Figure, spec: LineSpec) -> Option<PanLine> {
    if fig.point(spec.a).is_none() || fig.point(spec.b).is_none() {
        return None;
    }
    let (a, b) = (Point::new(spec.a), Point::new(spec.b));
    match spec.kind {
        0 => Some(PanLine::line(a, b)),
        1 => Some(PanLine::segment(a, b)),
        2 => Some(PanLine::ray(a, b)),
        _ => None,
    }
}

/// Creates an empty figure. Free it with `proverrr_figure_free`.
///
/// # Safety
///
/// Always safe; free the returned handle exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn proverrr_figure_new() -> *mut Figure {
    Box::into_raw(Box::new(Figure::new()))
}

/// Frees a figure created by `proverrr_figure_new`.
///
/// # Safety
///
/// `fig` must be null or a handle from `proverrr_figure_new` that is still
/// live; double free or use after free is the caller's error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn proverrr_figure_free(fig: *mut Figure) {
    if !fig.is_null() {
        drop(unsafe { Box::from_raw(fig) });
    }
}

/// Adds a point to `fig` and returns its ID, or `-1` if `fig` is null.
///
/// # Safety
///
/// `fig` must be null or a live figure handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn proverrr_figure_add_point(fig: *mut Figure) -> i64 {
    match unsafe { fig.as_mut() } {
        Some(fig) => fig.add_point().id() as i64,
        None => -1,
    }
}

/// Builds `line(x.a, x.b) ∥ line(y.a, y.b)`; `null` on unknown points or kinds.
///
/// # Safety
///
/// `fig` must be null or a live figure handle; the condition is freed by
/// `proverrr_condition_free` or consumed by `proverrr_figure_add_condition`
/// / `proverrr_proof_solve`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn proverrr_condition_parallel(
    fig: *const Figure,
    x: LineSpec,
    y: LineSpec,
) -> *mut Condition {
    let Some(fig) = (unsafe { fig.as_ref() }) else {
        return ptr::null_mut();
    };
    match (pan_line(fig, x), pan_line(fig, y)) {
        (Some(x), Some(y)) => {
            Box::into_raw(Box::new(Condition::Parallel(ParallelCondition::new(x, y))))
        }
        _ => ptr::null_mut(),
    }
}

/// Builds `a` and `b` on the same (`same != 0`) or opposite side of the line
/// `t`; `null` on unknown points or kinds.
///
/// # Safety
///
/// Same handle rules as `proverrr_condition_parallel`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn proverrr_condition_side(
    fig: *const Figure,
    t: LineSpec,
    a: u32,
    b: u32,
    same: u8,
) -> *mut Condition {
    let Some(fig) = (unsafe { fig.as_ref() }) else {
        return ptr::null_mut();
    };
    if fig.point(a).is_none() || fig.point(b).is_none() {
        return ptr::null_mut();
    }
    match pan_line(fig, t) {
        Some(t) => Box::into_raw(Box::new(Condition::Side(SideCondition::new(
            t,
            Point::new(a),
            Point::new(b),
            same != 0,
        )))),
        None => ptr::null_mut(),
    }
}

/// Builds `a`, `b`, `c` being collinear; `null` on unknown points or null `fig`.
///
/// # Safety
///
/// Same handle rules as `proverrr_condition_parallel`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn proverrr_condition_collinear(
    fig: *const Figure,
    a: u32,
    b: u32,
    c: u32,
) -> *mut Condition {
    let Some(fig) = (unsafe { fig.as_ref() }) else {
        return ptr::null_mut();
    };
    if fig.point(a).is_none() || fig.point(b).is_none() || fig.point(c).is_none() {
        return ptr::null_mut();
    }
    Box::into_raw(Box::new(Condition::Collinear(CollinearCondition::new(
        Point::new(a),
        Point::new(b),
        Point::new(c),
    ))))
}

/// Builds triangle `a1b1c1` being congruent to triangle `a2b2c2`; `null` on
/// unknown points or null `fig`.
///
/// # Safety
///
/// Same handle rules as `proverrr_condition_parallel`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn proverrr_condition_congruent(
    fig: *const Figure,
    a1: u32,
    b1: u32,
    c1: u32,
    a2: u32,
    b2: u32,
    c2: u32,
) -> *mut Condition {
    let Some(fig) = (unsafe { fig.as_ref() }) else {
        return ptr::null_mut();
    };
    let points = [a1, b1, c1, a2, b2, c2];
    if points.iter().any(|id| fig.point(*id).is_none()) {
        return ptr::null_mut();
    }
    Box::into_raw(Box::new(Condition::Congruent(CongruentCondition::new(
        Triangle::new(Point::new(a1), Point::new(b1), Point::new(c1)),
        Triangle::new(Point::new(a2), Point::new(b2), Point::new(c2)),
    ))))
}

/// Builds `angle(a, b, c) == angle(d, e, f)`; `null` on unknown points or null
/// `fig`.
///
/// # Safety
///
/// Same handle rules as `proverrr_condition_parallel`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn proverrr_condition_angles_equal(
    fig: *const Figure,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
    f: u32,
) -> *mut Condition {
    let Some(fig) = (unsafe { fig.as_ref() }) else {
        return ptr::null_mut();
    };
    let points = [a, b, c, d, e, f];
    if points.iter().any(|id| fig.point(*id).is_none()) {
        return ptr::null_mut();
    }
    Box::into_raw(Box::new(Condition::Sum(SumCondition::new(
        vec![Object::angle(Point::new(a), Point::new(b), Point::new(c))],
        vec![Object::angle(Point::new(d), Point::new(e), Point::new(f))],
        Ratio::from(0),
    ))))
}

/// Frees a condition built by a `proverrr_condition_*` function.
///
/// # Safety
///
/// `cond` must be null or a live condition handle that no other call has
/// consumed; double free is the caller's error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn proverrr_condition_free(cond: *mut Condition) {
    if !cond.is_null() {
        drop(unsafe { Box::from_raw(cond) });
    }
}

/// Assumes `cond` in `fig`, consuming it; `0` on success, `-1` on null handles
/// (then the caller still owns `cond`).
///
/// # Safety
///
/// `fig` must be null or a live figure handle; `cond` null or a live condition
/// handle that this call consumes on success.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn proverrr_figure_add_condition(
    fig: *mut Figure,
    cond: *mut Condition,
) -> i32 {
    if cond.is_null() {
        return -1;
    }
    let Some(fig) = (unsafe { fig.as_mut() }) else {
        return -1;
    };
    let cond = unsafe { Box::from_raw(cond) };
    fig.add_condition(*cond);
    0
}

/// Tries to prove `target` in `fig` with all rules enabled, consuming `target`
/// when both handles are non-null. Returns a proof that must be freed with
/// `proverrr_proof_free`, or `null` when the target is not (yet) provable.
///
/// # Safety
///
/// `fig` must be null or a live figure handle; `target` null or a live
/// condition handle that this call consumes when both are non-null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn proverrr_proof_solve(
    fig: *mut Figure,
    target: *mut Condition,
) -> *mut Proof {
    let (Some(fig), Some(_)) = (unsafe { fig.as_mut() }, unsafe { target.as_ref() }) else {
        return ptr::null_mut();
    };
    let target = unsafe { Box::from_raw(target) };
    match solve(fig, &target, &Rule::new()) {
        Some(proof) => Box::into_raw(Box::new(proof)),
        None => ptr::null_mut(),
    }
}

/// Frees a proof returned by `proverrr_proof_solve`.
///
/// # Safety
///
/// `proof` must be null or a live proof handle; double free is the caller's
/// error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn proverrr_proof_free(proof: *mut Proof) {
    if !proof.is_null() {
        drop(unsafe { Box::from_raw(proof) });
    }
}

/// Returns the number of steps in `proof`; `null` also yields `0`.
///
/// # Safety
///
/// `proof` must be null or a live proof handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn proverrr_proof_step_count(proof: *const Proof) -> u32 {
    match unsafe { proof.as_ref() } {
        Some(proof) => proof.describe.len() as u32,
        None => 0,
    }
}

/// Returns the name of step `index` as a string that must be freed with
/// `proverrr_string_free`; `null` when the index is out of range.
///
/// # Safety
///
/// `proof` must be null or a live proof handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn proverrr_proof_step(proof: *const Proof, index: u32) -> *mut c_char {
    let Some(proof) = (unsafe { proof.as_ref() }) else {
        return ptr::null_mut();
    };
    match proof.describe.get(index as usize) {
        Some(step) => match CString::new(format!("{step:?}")) {
            Ok(name) => name.into_raw(),
            Err(_) => ptr::null_mut(),
        },
        None => ptr::null_mut(),
    }
}

/// Frees a string returned by `proverrr_proof_step`.
///
/// # Safety
///
/// `s` must be null or a pointer returned by `proverrr_proof_step` that is
/// still live; double free is the caller's error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn proverrr_string_free(s: *mut c_char) {
    if !s.is_null() {
        drop(unsafe { CString::from_raw(s) });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    fn seg(a: u32, b: u32) -> LineSpec {
        LineSpec { kind: 1, a, b }
    }

    fn line(a: u32, b: u32) -> LineSpec {
        LineSpec { kind: 0, a, b }
    }

    #[test]
    fn solves_transitive_parallel_via_abi() {
        unsafe {
            let fig = proverrr_figure_new();
            for _ in 0..6 {
                proverrr_figure_add_point(fig);
            }
            let xy = proverrr_condition_parallel(fig, seg(0, 1), seg(2, 3));
            let yz = proverrr_condition_parallel(fig, seg(2, 3), seg(4, 5));
            assert!(!xy.is_null());
            assert!(!yz.is_null());
            assert_eq!(proverrr_figure_add_condition(fig, xy), 0);
            assert_eq!(proverrr_figure_add_condition(fig, yz), 0);

            let target = proverrr_condition_parallel(fig, seg(0, 1), seg(4, 5));
            let proof = proverrr_proof_solve(fig, target);
            assert!(!proof.is_null());
            assert_eq!(proverrr_proof_step_count(proof), 1);
            let step = proverrr_proof_step(proof, 0);
            assert_eq!(CStr::from_ptr(step).to_bytes(), b"ParallelTransitive");
            proverrr_string_free(step);
            proverrr_proof_free(proof);
            proverrr_figure_free(fig);
        }
    }

    #[test]
    fn solves_alternate_interior_angles_via_abi() {
        unsafe {
            let fig = proverrr_figure_new();
            for _ in 0..4 {
                proverrr_figure_add_point(fig);
            }
            let l1 = proverrr_condition_parallel(fig, line(0, 1), line(2, 3));
            let side = proverrr_condition_side(fig, line(1, 2), 0, 3, 0);
            assert_eq!(proverrr_figure_add_condition(fig, l1), 0);
            assert_eq!(proverrr_figure_add_condition(fig, side), 0);

            let target = proverrr_condition_angles_equal(fig, 0, 1, 2, 3, 2, 1);
            let proof = proverrr_proof_solve(fig, target);
            assert!(!proof.is_null());
            let step = proverrr_proof_step(proof, 0);
            assert_eq!(
                CStr::from_ptr(step).to_bytes(),
                b"ParallelAlternateInterior"
            );
            proverrr_string_free(step);
            proverrr_proof_free(proof);
            proverrr_figure_free(fig);
        }
    }

    #[test]
    fn rejects_invalid_input() {
        unsafe {
            let fig = proverrr_figure_new();
            for _ in 0..6 {
                proverrr_figure_add_point(fig);
            }
            // Unknown points.
            assert!(proverrr_condition_parallel(fig, seg(0, 99), seg(1, 2)).is_null());
            assert!(proverrr_condition_side(fig, line(0, 1), 0, 99, 0).is_null());
            assert!(proverrr_condition_collinear(fig, 0, 1, 99).is_null());
            assert!(proverrr_condition_congruent(fig, 0, 1, 2, 3, 4, 99).is_null());
            assert!(proverrr_condition_angles_equal(fig, 0, 1, 2, 3, 4, 99).is_null());
            // Unknown kind.
            assert!(
                proverrr_condition_parallel(
                    fig,
                    LineSpec {
                        kind: 9,
                        a: 0,
                        b: 0
                    },
                    seg(0, 0)
                )
                .is_null()
            );
            // Null condition.
            assert_eq!(proverrr_figure_add_condition(fig, ptr::null_mut()), -1);
            // Unprovable target: only x ∥ y is known, so x ∥ z fails.
            let xy = proverrr_condition_parallel(fig, seg(0, 1), seg(2, 3));
            assert_eq!(proverrr_figure_add_condition(fig, xy), 0);
            let target = proverrr_condition_parallel(fig, seg(0, 1), seg(4, 5));
            assert!(proverrr_proof_solve(fig, target).is_null());
            // Null handles.
            assert_eq!(proverrr_figure_add_point(ptr::null_mut()), -1);
            assert_eq!(proverrr_proof_step_count(ptr::null()), 0);
            assert!(proverrr_proof_step(ptr::null(), 0).is_null());
            proverrr_figure_free(fig);
            // Freeing null is a no-op.
            proverrr_figure_free(ptr::null_mut());
            proverrr_condition_free(ptr::null_mut());
            proverrr_proof_free(ptr::null_mut());
            proverrr_string_free(ptr::null_mut());
        }
    }

    #[test]
    fn step_index_out_of_range_is_null() {
        unsafe {
            let fig = proverrr_figure_new();
            for _ in 0..6 {
                proverrr_figure_add_point(fig);
            }
            let xy = proverrr_condition_parallel(fig, seg(0, 1), seg(2, 3));
            let yz = proverrr_condition_parallel(fig, seg(2, 3), seg(4, 5));
            proverrr_figure_add_condition(fig, xy);
            proverrr_figure_add_condition(fig, yz);
            let target = proverrr_condition_parallel(fig, seg(0, 1), seg(4, 5));
            let proof = proverrr_proof_solve(fig, target);
            assert!(proverrr_proof_step(proof, 1).is_null());
            proverrr_proof_free(proof);
            proverrr_figure_free(fig);
        }
    }
}
