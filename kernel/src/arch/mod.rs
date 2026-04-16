//! Architecture-specific code for `rustos`.
//!
//! This module keeps target-specific runtime groundwork separate from
//! shared kernel logic. The current implementation stays intentionally
//! small and delegates pure runtime state logic to `kernel_core`.

pub use kernel_core::arch::{RuntimeState, init, runtime_summary};

/// Returns the current architecture name used by the kernel.
///
/// This is a small placeholder so architecture-specific behavior can
/// grow behind a dedicated module boundary without changing callers.
#[must_use]
pub const fn name() -> &'static str {
    "x86_64"
}

/// Idles the current CPU in the smallest possible way for this milestone.
///
/// A real halt or wait-for-interrupt path can be introduced later once the
/// interrupt groundwork is in place.
pub fn idle() {
    core::hint::spin_loop();
}
