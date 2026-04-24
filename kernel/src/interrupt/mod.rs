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

/// Returns the current stage label for the controlled exception path.
///
/// This label makes it explicit that the current milestone now uses a real
/// breakpoint handler path instead of a scaffolded post-trigger marker.
#[must_use]
pub const fn controlled_exception_stage_label(exception: ControlledException) -> &'static str {
    match exception {
        ControlledException::Breakpoint => crate::BREAKPOINT_HANDLER_ACTIVE_MESSAGE,
    }
}

/// Returns the success marker for the controlled exception path.
///
/// This marker is emitted only after the real breakpoint handler has run.
#[must_use]
pub const fn controlled_exception_success_marker(exception: ControlledException) -> &'static str {
    match exception {
        ControlledException::Breakpoint => crate::BREAKPOINT_HANDLER_REACHED_MESSAGE,
    }
}

/// Triggers the current controlled exception path.
///
/// For the current implementation, this uses the real architecture-specific
/// breakpoint path. Callers should only use this after the kernel has installed
/// the real breakpoint handler path; otherwise the resulting CPU exception path
/// is not guaranteed by this module.
pub fn trigger_controlled_exception(exception: ControlledException) {
    match exception {
        ControlledException::Breakpoint => {
            crate::arch::trigger_breakpoint();
        }
    }
}

/// Reports the controlled exception success marker.
///
/// This function emits output only when the real breakpoint handler has already
/// been reached. If the handler has not run yet, this function is intentionally
/// silent.
pub fn report_controlled_exception(exception: ControlledException) {
    match exception {
        ControlledException::Breakpoint => {
            if crate::arch::breakpoint_handler_reached() {
                crate::console::write_line(controlled_exception_success_marker(exception));
            }
        }
    }
}

/// Returns whether the kernel has installed the real breakpoint-handler path
/// used by the current controlled exception flow.
///
/// This does not imply that broader exception coverage is complete.
#[must_use]
pub fn has_real_exception_handlers() -> bool {
    crate::arch::has_real_exception_handlers()
}

/// Returns whether the current interrupt groundwork is ready enough for the
/// early runtime sequence to continue.
///
/// The current readiness rule is intentionally narrow:
/// - breakpoint groundwork must be ready
/// - double-fault groundwork must be ready
/// - timer interrupt groundwork must be ready
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
        controlled_exception_ready, controlled_exception_stage_label,
        controlled_exception_success_marker, exception_summary, init, interrupt_summary, is_ready,
        state_summary,
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
    fn controlled_exception_stage_label_matches_breakpoint() {
        assert_eq!(
            controlled_exception_stage_label(ControlledException::Breakpoint),
            crate::BREAKPOINT_HANDLER_ACTIVE_MESSAGE
        );
    }

    #[test]
    fn controlled_exception_success_marker_matches_breakpoint() {
        assert_eq!(
            controlled_exception_success_marker(ControlledException::Breakpoint),
            crate::BREAKPOINT_HANDLER_REACHED_MESSAGE
        );
    }

    #[test]
    fn real_exception_handler_status_matches_architecture_support() {
        #[cfg(target_arch = "x86_64")]
        assert!(!has_real_exception_handlers());

        #[cfg(not(target_arch = "x86_64"))]
        assert!(!super::has_real_exception_handlers());
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
