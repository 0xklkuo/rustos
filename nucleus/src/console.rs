//! Console state that is safe to test on the host.

/// Small summary of early console state during boot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    initialized: bool,
}

impl State {
    /// Creates a new uninitialized console state.
    #[must_use]
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    /// Creates an initialized console state.
    #[must_use]
    pub const fn initialized() -> Self {
        Self { initialized: true }
    }

    /// Returns whether the console has been initialized.
    #[must_use]
    pub const fn is_initialized(self) -> bool {
        self.initialized
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns a short plain-language summary of the current console state.
#[must_use]
pub const fn state_summary(state: State) -> &'static str {
    if state.is_initialized() {
        "rustos: console init complete"
    } else {
        "rustos: console init deferred"
    }
}

#[cfg(test)]
mod tests {
    use super::{State, state_summary};

    #[test]
    fn new_console_state_starts_uninitialized() {
        let state = State::new();

        assert!(!state.is_initialized());
    }

    #[test]
    fn initialized_console_state_reports_initialized() {
        let state = State::initialized();

        assert!(state.is_initialized());
    }

    #[test]
    fn default_console_state_matches_new_state() {
        let state = State::default();

        assert_eq!(state, State::new());
    }

    #[test]
    fn state_summary_reports_deferred_for_uninitialized_console() {
        let state = State::new();

        assert_eq!(state_summary(state), "rustos: console init deferred");
    }

    #[test]
    fn state_summary_reports_complete_for_initialized_console() {
        let state = State::initialized();

        assert_eq!(state_summary(state), "rustos: console init complete");
    }
}
