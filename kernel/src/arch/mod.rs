//! Architecture-specific code for `rustos`.
//!
//! This module is intentionally small in Milestone 3.
//! It exists to create a clear boundary between shared kernel logic
//! and target-specific implementation details.

/// Returns the current architecture name used by the kernel.
///
/// This is a small placeholder so architecture-specific behavior can
/// grow behind a dedicated module boundary without changing callers.
#[must_use]
pub const fn name() -> &'static str {
    "x86_64"
}
