#![cfg_attr(test, allow(clippy::bool_assert_comparison))]

//! Early console support for `rustos`.
//!
//! This module keeps early output intentionally small and explicit.
//! For now it is a thin wrapper around the UEFI text console.

pub use nucleus::console::{State, state_summary};

use uefi::Status;
use uefi::println;

/// Initializes early console support.
///
/// For the current milestone, this delegates to the UEFI helper setup so
/// printing and panic output work during early boot.
pub fn init() -> Result<State, Status> {
    uefi::helpers::init()
        .map(|_| State::initialized())
        .map_err(|error| error.status())
}

/// Prints a single line to the early UEFI console.
///
/// This function appends a trailing newline through the underlying console
/// helper and does not report failures at this layer.
///
/// Callers should use this only after `init()` has completed successfully.
pub fn write_line(message: &str) {
    println!("{message}");
}

#[cfg(test)]
mod tests {
    use super::{State, state_summary};

    #[test]
    fn new_console_state_starts_uninitialized() {
        let state = State::new();

        assert_eq!(state.is_initialized(), false);
    }

    #[test]
    fn initialized_console_state_reports_initialized() {
        let state = State::initialized();

        assert_eq!(state.is_initialized(), true);
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
