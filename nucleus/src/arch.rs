//! Architecture-adjacent runtime state that is safe to test on the host.

/// Small summary of architecture runtime state during early boot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeState {
    interrupts_ready: bool,
    timer_ready: bool,
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

    /// Creates a runtime state with initialized interrupt and timer groundwork.
    #[must_use]
    pub const fn ready() -> Self {
        Self {
            interrupts_ready: true,
            timer_ready: true,
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

/// Performs the minimal architecture-specific runtime setup for the current
/// milestone.
#[must_use]
pub const fn init() -> RuntimeState {
    RuntimeState::ready()
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

#[cfg(test)]
mod tests {
    use super::{RuntimeState, init, runtime_summary};

    #[test]
    fn new_runtime_state_starts_uninitialized() {
        let state = RuntimeState::new();

        assert!(!state.is_interrupts_ready());
        assert!(!state.is_timer_ready());
        assert_eq!(runtime_summary(state), "arch runtime not initialized");
    }

    #[test]
    fn init_marks_interrupts_and_timer_ready() {
        let state = init();

        assert!(state.is_interrupts_ready());
        assert!(state.is_timer_ready());
        assert_eq!(runtime_summary(state), "arch runtime ready");
    }

    #[test]
    fn default_runtime_state_matches_new() {
        let state = RuntimeState::default();

        assert_eq!(state, RuntimeState::new());
    }
}
