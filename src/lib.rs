//! A light-weight library for solving geometric problems with exact arithmetic.
//!
//! A [`figure::Figure`] stores the points and lines of a problem together with
//! the [`prove::condition::Condition`]s known to be true. Every condition comes
//! with a [`prove::proof::Proof`] recording the steps that derived it, and
//! [`prove::solve::solve`] keeps deriving conditions until the target is proven.

pub mod figure;
pub mod prove;
pub mod ratio;
