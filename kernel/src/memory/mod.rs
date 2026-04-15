//! Memory module placeholder for `rustos`.
//!
//! This module intentionally stays minimal during Milestone 3.
//! Real memory discovery, frame allocation, paging, and heap setup
//! will be introduced in later milestones.

/// Returns whether the memory subsystem has real initialization logic yet.
#[must_use]
pub const fn is_initialized() -> bool {
    false
}
