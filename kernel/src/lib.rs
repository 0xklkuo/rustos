#![no_std]

//! Minimal placeholder kernel crate for `rustos`.
//!
//! This crate intentionally stays small during Milestone 0.
//! Boot logic, platform initialization, and runtime setup will be
//! introduced in later milestones.

/// Returns the current kernel crate name.
///
/// This is a tiny placeholder API so the crate has at least one
/// documented public item during the initial scaffold phase.
#[must_use]
pub const fn kernel_name() -> &'static str {
    "rustos"
}
