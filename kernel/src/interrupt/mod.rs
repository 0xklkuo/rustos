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

/// Returns the current stage label for the controlled exception path.
#[must_use]
pub const fn controlled_exception_stage_label(exception: ControlledException) -> &'static str {
    match exception {
        ControlledException::Breakpoint => crate::BREAKPOINT_HANDLER_ACTIVE_MESSAGE,
    }
}

/// Returns the success marker for the controlled exception path.
#[must_use]
pub const fn controlled_exception_success_marker(exception: ControlledException) -> &'static str {
    match exception {
        ControlledException::Breakpoint => crate::BREAKPOINT_HANDLER_REACHED_MESSAGE,
    }
}

/// Triggers the current controlled exception path.
pub fn trigger_controlled_exception(exception: ControlledException) {
    match exception {
        ControlledException::Breakpoint => {
            crate::arch::trigger_breakpoint();
        }
    }
}

/// Reports the controlled exception success marker.
pub fn report_controlled_exception(exception: ControlledException) {
    match exception {
        ControlledException::Breakpoint => {
            if crate::arch::breakpoint_handler_reached() {
                crate::console::write_line(controlled_exception_success_marker(exception));
            }
        }
    }
}

/// Returns whether the kernel has installed the real breakpoint-handler path.
#[must_use]
pub fn has_real_exception_handlers() -> bool {
    crate::arch::has_real_exception_handlers()
}

/// Returns whether the current interrupt groundwork is ready enough for the
/// early runtime sequence to continue.
#[must_use]
pub const fn is_ready(state: State) -> bool {
    state.exceptions().is_breakpoint_ready()
        && state.exceptions().is_double_fault_ready()
        && state.interrupts().is_timer_ready()
}
