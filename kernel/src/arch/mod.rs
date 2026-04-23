//! Architecture-specific code for `rustos`.
//!
//! This module keeps target-specific runtime groundwork separate from
//! shared kernel logic. The current implementation stays intentionally
//! small and delegates pure runtime state logic to `nucleus`.
//!
//! It also exposes the smallest architecture-facing paging probe boundary
//! needed for the current U5 milestone without claiming real paging
//! management yet.

#[cfg(target_os = "uefi")]
pub mod x86_64;

pub use nucleus::arch::{RuntimeState, init, runtime_summary};

/// Returns the current architecture name used by the kernel.
///
/// This is a small placeholder so architecture-specific behavior can
/// grow behind a dedicated module boundary without changing callers.
#[must_use]
pub const fn name() -> &'static str {
    "x86_64"
}

/// Initializes architecture-specific interrupt support for the current target.
///
/// This keeps the low-level interrupt setup behind the architecture boundary
/// while the rest of the kernel continues to use a small shared interface.
#[cfg(target_os = "uefi")]
pub fn init_interrupts() {
    x86_64::init_idt();
}

/// Initializes architecture-specific interrupt support for non-UEFI builds.
///
/// Host-side checks should compile cleanly without requiring target-only
/// interrupt support.
#[cfg(not(target_os = "uefi"))]
pub fn init_interrupts() {}

/// Returns whether a small architecture-facing paging probe is available for the current target.
///
/// This keeps the paging milestone narrow:
/// - x86_64 UEFI builds can expose a real architecture hook
/// - host-side builds still compile cleanly
/// - full paging management remains deferred
#[must_use]
#[cfg(target_os = "uefi")]
pub fn has_paging_probe() -> bool {
    x86_64::has_paging_probe()
}

/// Returns whether a small architecture-facing paging probe is available on non-UEFI builds.
#[must_use]
#[cfg(not(target_os = "uefi"))]
pub fn has_paging_probe() -> bool {
    false
}

/// Returns whether real exception handlers are installed for the current target.
#[must_use]
#[cfg(target_os = "uefi")]
pub fn has_real_exception_handlers() -> bool {
    x86_64::has_real_breakpoint_handler()
}

/// Returns whether real exception handlers are installed for non-UEFI builds.
#[must_use]
#[cfg(not(target_os = "uefi"))]
pub fn has_real_exception_handlers() -> bool {
    false
}

/// Returns whether the real breakpoint handler has already run.
#[must_use]
#[cfg(target_os = "uefi")]
pub fn breakpoint_handler_reached() -> bool {
    x86_64::breakpoint_handler_reached()
}

/// Returns whether the real breakpoint handler has already run on non-UEFI builds.
#[must_use]
#[cfg(not(target_os = "uefi"))]
pub fn breakpoint_handler_reached() -> bool {
    false
}

/// Triggers the current architecture's real breakpoint path.
#[cfg(target_os = "uefi")]
pub fn trigger_breakpoint() {
    x86_64::trigger_breakpoint();
}

/// Triggers the current architecture's real breakpoint path on non-UEFI builds.
///
/// This is a no-op so host-side checks can compile cleanly.
#[cfg(not(target_os = "uefi"))]
pub fn trigger_breakpoint() {}

/// Idles the current CPU in the smallest possible way for this milestone.
///
/// A real halt or wait-for-interrupt path can be introduced later once the
/// interrupt groundwork is in place.
pub fn idle() {
    core::hint::spin_loop();
}
