//! Exception and interrupt groundwork for `rustos`.
//!
//! This module keeps the firmware-facing kernel crate small by re-exporting
//! host-testable interrupt state from `nucleus` and providing only the
//! runtime-facing pieces that belong in the kernel crate.

pub use nucleus::interrupt::{
    ExceptionState, InterruptState, State, exception_summary, init, interrupt_summary,
    state_summary,
};

/// Small summary of the current controlled exception path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledException {
    /// Trigger a breakpoint exception path first.
    Breakpoint,
}

/// Returns the current controlled exception used for narrow runtime validation.
#[must_use]
pub const fn controlled_exception() -> ControlledException {
    ControlledException::Breakpoint
}

/// Returns a short plain-language label for the current controlled exception.
#[must_use]
pub const fn controlled_exception_label(exception: ControlledException) -> &'static str {
    match exception {
        ControlledException::Breakpoint => crate::BREAKPOINT_TEST_MESSAGE,
    }
}

/// Returns whether the current controlled exception is ready to run.
#[must_use]
pub const fn controlled_exception_ready(state: State, exception: ControlledException) -> bool {
    match exception {
        ControlledException::Breakpoint => state.exceptions().is_controlled_breakpoint_ready(),
    }
}

/// Returns the success marker expected after the controlled exception path runs.
#[must_use]
pub const fn controlled_exception_success_marker(exception: ControlledException) -> &'static str {
    match exception {
        ControlledException::Breakpoint => "rustos: breakpoint handler reached",
    }
}

/// Triggers the current controlled exception path.
///
/// For the current milestone, this uses a real CPU breakpoint instruction so
/// the exception path can be exercised explicitly and observed in bounded QEMU
/// output.
pub fn trigger_controlled_exception(exception: ControlledException) {
    match exception {
        ControlledException::Breakpoint => {
            #[cfg(target_arch = "x86_64")]
            unsafe {
                core::arch::asm!("int3", options(nomem, nostack));
            }

            #[cfg(not(target_arch = "x86_64"))]
            {
                let _ = exception;
            }
        }
    }
}

/// Reports the controlled exception handler marker for the current milestone.
///
/// This keeps the first exception path explicit and easy to validate before a
/// fuller exception handler framework exists.
pub fn report_controlled_exception(exception: ControlledException) {
    match exception {
        ControlledException::Breakpoint => {
            crate::console::write_line(controlled_exception_success_marker(exception));
        }
    }
}

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
    use super::{
        ControlledException, State, controlled_exception, controlled_exception_label,
        controlled_exception_ready, controlled_exception_success_marker, exception_summary, init,
        interrupt_summary, is_ready, state_summary,
    };

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
    fn controlled_exception_defaults_to_breakpoint() {
        assert_eq!(controlled_exception(), ControlledException::Breakpoint);
    }

    #[test]
    fn controlled_exception_label_matches_breakpoint() {
        assert_eq!(
            controlled_exception_label(ControlledException::Breakpoint),
            crate::BREAKPOINT_TEST_MESSAGE
        );
    }

    #[test]
    fn controlled_exception_success_marker_matches_breakpoint() {
        assert_eq!(
            controlled_exception_success_marker(ControlledException::Breakpoint),
            "rustos: breakpoint handler reached"
        );
    }

    #[test]
    fn controlled_exception_ready_tracks_breakpoint_state() {
        let ready_state = nucleus::interrupt::init_controlled_breakpoint();
        let deferred_state = State::new();

        assert!(controlled_exception_ready(
            ready_state,
            ControlledException::Breakpoint
        ));
        assert!(!controlled_exception_ready(
            deferred_state,
            ControlledException::Breakpoint
        ));
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
