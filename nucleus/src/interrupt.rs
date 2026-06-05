//! Host-testable exception and interrupt groundwork.

/// Small summary of exception handling state during early runtime setup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExceptionState {
    breakpoint_ready: bool,
    double_fault_ready: bool,
    controlled_breakpoint_ready: bool,
}

impl ExceptionState {
    /// Creates a new exception state with no configured handlers.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            breakpoint_ready: false,
            double_fault_ready: false,
            controlled_breakpoint_ready: false,
        }
    }

    /// Creates an exception state with the current minimal handlers prepared.
    #[must_use]
    pub const fn ready() -> Self {
        Self {
            breakpoint_ready: true,
            double_fault_ready: true,
            controlled_breakpoint_ready: false,
        }
    }

    /// Creates an exception state with the current minimal handlers prepared
    /// and a controlled breakpoint path enabled.
    #[must_use]
    pub const fn controlled_breakpoint_ready() -> Self {
        Self {
            breakpoint_ready: true,
            double_fault_ready: true,
            controlled_breakpoint_ready: true,
        }
    }

    /// Returns whether breakpoint handling groundwork is ready.
    #[must_use]
    pub const fn is_breakpoint_ready(self) -> bool {
        self.breakpoint_ready
    }

    /// Returns whether double-fault handling groundwork is ready.
    #[must_use]
    pub const fn is_double_fault_ready(self) -> bool {
        self.double_fault_ready
    }

    /// Returns whether the controlled breakpoint path is ready.
    #[must_use]
    pub const fn is_controlled_breakpoint_ready(self) -> bool {
        self.controlled_breakpoint_ready
    }
}

impl Default for ExceptionState {
    fn default() -> Self {
        Self::new()
    }
}

/// Small summary of hardware interrupt groundwork during early runtime setup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterruptState {
    timer_ready: bool,
    keyboard_ready: bool,
}

impl InterruptState {
    /// Creates a new interrupt state with no configured handlers.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            timer_ready: false,
            keyboard_ready: false,
        }
    }

    /// Creates an interrupt state with the current minimal handlers prepared.
    #[must_use]
    pub const fn ready() -> Self {
        Self {
            timer_ready: true,
            keyboard_ready: false,
        }
    }

    /// Returns whether timer interrupt groundwork is ready.
    #[must_use]
    pub const fn is_timer_ready(self) -> bool {
        self.timer_ready
    }

    /// Returns whether keyboard interrupt groundwork is ready.
    #[must_use]
    pub const fn is_keyboard_ready(self) -> bool {
        self.keyboard_ready
    }
}

impl Default for InterruptState {
    fn default() -> Self {
        Self::new()
    }
}

/// Small summary of the current interrupt subsystem state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    exceptions: ExceptionState,
    interrupts: InterruptState,
}

impl State {
    /// Creates a new interrupt subsystem state with no configured handlers.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            exceptions: ExceptionState::new(),
            interrupts: InterruptState::new(),
        }
    }

    /// Creates an interrupt subsystem state with the current minimal groundwork prepared.
    #[must_use]
    pub const fn ready() -> Self {
        Self {
            exceptions: ExceptionState::ready(),
            interrupts: InterruptState::ready(),
        }
    }

    /// Creates an interrupt subsystem state with the current minimal groundwork
    /// prepared and a controlled breakpoint path enabled.
    #[must_use]
    pub const fn controlled_breakpoint_ready() -> Self {
        Self {
            exceptions: ExceptionState::controlled_breakpoint_ready(),
            interrupts: InterruptState::ready(),
        }
    }

    /// Returns the current exception handling state.
    #[must_use]
    pub const fn exceptions(self) -> ExceptionState {
        self.exceptions
    }

    /// Returns the current hardware interrupt state.
    #[must_use]
    pub const fn interrupts(self) -> InterruptState {
        self.interrupts
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

/// Performs the current minimal exception and interrupt initialization step.
#[must_use]
pub const fn init() -> State {
    State::ready()
}

/// Performs the current minimal exception and interrupt initialization step
/// with a controlled breakpoint path enabled.
#[must_use]
pub const fn init_controlled_breakpoint() -> State {
    State::controlled_breakpoint_ready()
}

/// Returns a small plain-language summary of the current exception state.
#[must_use]
pub const fn exception_summary(state: ExceptionState) -> &'static str {
    if state.is_controlled_breakpoint_ready() {
        "controlled breakpoint path ready"
    } else if state.is_breakpoint_ready() && state.is_double_fault_ready() {
        "exception groundwork ready"
    } else if state.is_breakpoint_ready() {
        "breakpoint groundwork ready"
    } else if state.is_double_fault_ready() {
        "double fault groundwork ready"
    } else {
        "exception groundwork not initialized"
    }
}

/// Returns a small plain-language summary of the current hardware interrupt state.
#[must_use]
pub const fn interrupt_summary(state: InterruptState) -> &'static str {
    if state.is_timer_ready() && state.is_keyboard_ready() {
        "hardware interrupt groundwork ready"
    } else if state.is_timer_ready() {
        "timer interrupt groundwork ready"
    } else if state.is_keyboard_ready() {
        "keyboard interrupt groundwork ready"
    } else {
        "hardware interrupt groundwork not initialized"
    }
}

/// Returns a small plain-language summary of the current interrupt subsystem state.
#[must_use]
pub const fn state_summary(state: State) -> &'static str {
    if state.exceptions().is_controlled_breakpoint_ready()
        && state.exceptions().is_breakpoint_ready()
        && state.exceptions().is_double_fault_ready()
        && state.interrupts().is_timer_ready()
    {
        "interrupt foundation ready with controlled breakpoint"
    } else if state.exceptions().is_breakpoint_ready()
        && state.exceptions().is_double_fault_ready()
        && state.interrupts().is_timer_ready()
    {
        "interrupt foundation ready"
    } else if state.exceptions().is_breakpoint_ready()
        || state.exceptions().is_double_fault_ready()
        || state.interrupts().is_timer_ready()
        || state.interrupts().is_keyboard_ready()
    {
        "interrupt foundation partial"
    } else {
        "interrupt foundation deferred"
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ExceptionState, InterruptState, State, exception_summary, init, init_controlled_breakpoint,
        interrupt_summary, state_summary,
    };

    #[test]
    fn new_exception_state_starts_uninitialized() {
        let state = ExceptionState::new();

        assert!(!state.is_breakpoint_ready());
        assert!(!state.is_double_fault_ready());
        assert!(!state.is_controlled_breakpoint_ready());
        assert_eq!(
            exception_summary(state),
            "exception groundwork not initialized"
        );
    }

    #[test]
    fn ready_exception_state_reports_ready() {
        let state = ExceptionState::ready();

        assert!(state.is_breakpoint_ready());
        assert!(state.is_double_fault_ready());
        assert!(!state.is_controlled_breakpoint_ready());
        assert_eq!(exception_summary(state), "exception groundwork ready");
    }

    #[test]
    fn controlled_breakpoint_exception_state_reports_ready() {
        let state = ExceptionState::controlled_breakpoint_ready();

        assert!(state.is_breakpoint_ready());
        assert!(state.is_double_fault_ready());
        assert!(state.is_controlled_breakpoint_ready());
        assert_eq!(exception_summary(state), "controlled breakpoint path ready");
    }

    #[test]
    fn new_interrupt_state_starts_uninitialized() {
        let state = InterruptState::new();

        assert!(!state.is_timer_ready());
        assert!(!state.is_keyboard_ready());
        assert_eq!(
            interrupt_summary(state),
            "hardware interrupt groundwork not initialized"
        );
    }

    #[test]
    fn ready_interrupt_state_reports_timer_groundwork() {
        let state = InterruptState::ready();

        assert!(state.is_timer_ready());
        assert!(!state.is_keyboard_ready());
        assert_eq!(interrupt_summary(state), "timer interrupt groundwork ready");
    }

    #[test]
    fn init_returns_ready_interrupt_foundation() {
        let state = init();

        assert_eq!(state, State::ready());
        assert_eq!(state_summary(state), "interrupt foundation ready");
    }

    #[test]
    fn controlled_breakpoint_init_reports_ready_interrupt_foundation() {
        let state = init_controlled_breakpoint();

        assert_eq!(state, State::controlled_breakpoint_ready());
        assert_eq!(
            state_summary(state),
            "interrupt foundation ready with controlled breakpoint"
        );
    }

    #[test]
    fn default_interrupt_foundation_matches_new() {
        let state = State::default();

        assert_eq!(state, State::new());
        assert_eq!(state_summary(state), "interrupt foundation deferred");
    }
}
