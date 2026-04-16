//! Early console support for `rustos`.
//!
//! This module keeps early output intentionally small and explicit.
//! For now it is a thin wrapper around the UEFI text console.

use uefi::Status;
use uefi::println;

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

/// Initializes early console support.
///
/// For the current milestone, this delegates to the UEFI helper setup so
/// printing and panic output work during early boot.
pub fn init() -> Result<State, Status> {
    uefi::helpers::init()
        .map(|_| State { initialized: true })
        .map_err(|error| error.status())
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

/// Prints a single line to the early console.
pub fn write_line(message: &str) {
    println!("{message}");
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
        let state = State { initialized: true };

        assert_eq!(state_summary(state), "rustos: console init complete");
    }
}
