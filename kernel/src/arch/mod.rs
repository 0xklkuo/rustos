//! Architecture-specific code for `rustos`.
//!
//! This module keeps target-specific runtime groundwork separate from
//! shared kernel logic. The current implementation stays intentionally
//! small and only exposes the minimal placeholders needed by the early
//! boot flow.
#![cfg_attr(test, allow(clippy::bool_assert_comparison))]

/// Small summary of architecture runtime state during early boot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeState {
    interrupts_ready: bool,
    timer_ready: bool,
}

/// Returns the current architecture name used by the kernel.
///
/// This is a small placeholder so architecture-specific behavior can
/// grow behind a dedicated module boundary without changing callers.
#[must_use]
pub const fn name() -> &'static str {
    "x86_64"
}

impl RuntimeState {
    /// Creates a new runtime state with no initialized low-level services.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            interrupts_ready: false,
            timer_ready: false,
        }
    }

    /// Returns whether interrupt groundwork has been initialized.
    #[must_use]
    pub const fn is_interrupts_ready(self) -> bool {
        self.interrupts_ready
    }

    /// Returns whether timer groundwork has been initialized.
    #[must_use]
    pub const fn is_timer_ready(self) -> bool {
        self.timer_ready
    }
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self::new()
    }
}

/// Performs the minimal architecture-specific runtime setup for the
/// current milestone.
///
/// This function is intentionally small. It records the expected early
/// runtime state without introducing real interrupt or timer handling yet.
#[must_use]
pub const fn init() -> RuntimeState {
    RuntimeState {
        interrupts_ready: true,
        timer_ready: true,
    }
}

/// Returns a short plain-language description of the current runtime state.
#[must_use]
pub const fn runtime_summary(state: RuntimeState) -> &'static str {
    if state.is_interrupts_ready() && state.is_timer_ready() {
        "arch runtime ready"
    } else if state.is_interrupts_ready() {
        "interrupt groundwork ready"
    } else if state.is_timer_ready() {
        "timer groundwork ready"
    } else {
        "arch runtime not initialized"
    }
}

/// Idles the current CPU in the smallest possible way for this milestone.
///
/// A real halt or wait-for-interrupt path can be introduced later once the
/// interrupt groundwork is in place.
pub fn idle() {
    core::hint::spin_loop();
}

#[cfg(test)]
mod tests {
    use super::{RuntimeState, init, runtime_summary};

    #[test]
    fn new_runtime_state_starts_uninitialized() {
        let state = RuntimeState::new();

        assert_eq!(state.is_interrupts_ready(), false);
        assert_eq!(state.is_timer_ready(), false);
        assert_eq!(runtime_summary(state), "arch runtime not initialized");
    }

    #[test]
    fn init_marks_interrupts_and_timer_ready() {
        let state = init();

        assert_eq!(state.is_interrupts_ready(), true);
        assert_eq!(state.is_timer_ready(), true);
        assert_eq!(runtime_summary(state), "arch runtime ready");
    }

    #[test]
    fn default_runtime_state_matches_new() {
        let state = RuntimeState::default();

        assert_eq!(state, RuntimeState::new());
    }
}
