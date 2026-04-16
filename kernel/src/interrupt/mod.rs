//! Exception and interrupt groundwork for `rustos`.
//!
//! This module keeps the firmware-facing kernel crate small by re-exporting
//! host-testable interrupt state from `nucleus` and providing only the
//! runtime-facing pieces that belong in the kernel crate.

pub use nucleus::interrupt::{
    ExceptionState, InterruptState, State, exception_summary, init, interrupt_summary,
    state_summary,
};

/// Returns whether the current interrupt groundwork is ready enough for the
/// early runtime sequence to continue.
///
/// This is intentionally small for the current milestone. It gives the boot
/// path a single explicit question to ask without exposing more policy than
/// needed.
#[must_use]
pub const fn is_ready(state: State) -> bool {
    state.exceptions().is_breakpoint_ready()
        && state.exceptions().is_double_fault_ready()
        && state.interrupts().is_timer_ready()
}

#[cfg(test)]
mod tests {
    use super::{State, exception_summary, init, interrupt_summary, is_ready, state_summary};

    #[test]
    fn initialized_interrupt_state_is_ready() {
        let state = init();

        assert!(is_ready(state));
    }

    #[test]
    fn default_interrupt_state_is_not_ready() {
        let state = State::new();

        assert!(!is_ready(state));
    }

    #[test]
    fn summaries_match_expected_ready_state() {
        let state = init();

        assert_eq!(
            exception_summary(state.exceptions()),
            "exception groundwork ready"
        );
        assert_eq!(
            interrupt_summary(state.interrupts()),
            "timer interrupt groundwork ready"
        );
        assert_eq!(state_summary(state), "interrupt foundation ready");
    }
}
