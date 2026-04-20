#![no_std]

//! Host-testable pure logic shared by the `rustos` kernel.
//!
//! This crate contains small, explicit logic that does not require UEFI,
//! firmware services, or direct hardware interaction. The goal is to keep
//! pure state and summary logic easy to test on the host while the main
//! `kernel` crate stays focused on runtime and boot integration.

pub mod arch {
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

    /// Performs the minimal architecture-specific runtime setup for the
    /// current milestone.
    ///
    /// This function is intentionally small. It records the expected early
    /// runtime state without introducing real interrupt or timer handling yet.
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
}

pub mod console {
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
}

pub mod interrupt {
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
            ExceptionState, InterruptState, State, exception_summary, init,
            init_controlled_breakpoint, interrupt_summary, state_summary,
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
}

pub mod memory {
    //! Minimal memory foundation that is safe to test on the host.

    /// Small summary of the current memory subsystem state.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct State {
        initialized: bool,
        frame_allocator_ready: bool,
        heap_strategy: HeapStrategy,
    }

    impl State {
        /// Creates a new uninitialized memory state.
        #[must_use]
        pub const fn new() -> Self {
            Self {
                initialized: false,
                frame_allocator_ready: false,
                heap_strategy: HeapStrategy::Deferred,
            }
        }

        /// Returns whether the memory subsystem has been initialized.
        #[must_use]
        pub const fn is_initialized(self) -> bool {
            self.initialized
        }

        /// Returns whether the frame allocator skeleton is ready.
        #[must_use]
        pub const fn is_frame_allocator_ready(self) -> bool {
            self.frame_allocator_ready
        }

        /// Returns the current heap strategy decision.
        #[must_use]
        pub const fn heap_strategy(self) -> HeapStrategy {
            self.heap_strategy
        }
    }

    impl Default for State {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Minimal heap strategy decision for the current milestone.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum HeapStrategy {
        /// Heap support is intentionally deferred.
        Deferred,
        /// A fixed-size heap may be introduced later.
        FixedSize,
    }

    /// Small frame allocator skeleton for future memory work.
    ///
    /// This type does not allocate real frames yet. It exists to make the
    /// intended subsystem boundary explicit.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FrameAllocator {
        next_frame: usize,
    }

    impl FrameAllocator {
        /// Creates a new empty frame allocator skeleton.
        #[must_use]
        pub const fn new() -> Self {
            Self { next_frame: 0 }
        }

        /// Returns the next frame index that would be handed out.
        #[must_use]
        pub const fn next_frame(self) -> usize {
            self.next_frame
        }

        /// Returns a new allocator state after reserving one frame.
        ///
        /// This is only a placeholder for the current milestone and does not
        /// correspond to real physical memory management yet.
        #[must_use]
        pub const fn reserve(self) -> Self {
            Self {
                next_frame: self.next_frame + 1,
            }
        }
    }

    impl Default for FrameAllocator {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Performs the current minimal memory initialization step.
    ///
    /// This keeps the boot flow explicit while making the future memory
    /// subsystem shape visible.
    #[must_use]
    pub const fn init() -> State {
        State {
            initialized: true,
            frame_allocator_ready: true,
            heap_strategy: HeapStrategy::Deferred,
        }
    }

    /// Returns whether the memory subsystem has real initialization logic yet.
    #[must_use]
    pub const fn is_initialized() -> bool {
        init().is_initialized()
    }

    /// Returns a small plain-language summary of the current memory state.
    #[must_use]
    pub const fn state_summary(state: State) -> &'static str {
        if state.is_initialized() && state.is_frame_allocator_ready() {
            "rustos: memory foundation ready"
        } else if state.is_initialized() {
            "rustos: memory init complete"
        } else {
            "rustos: memory init deferred"
        }
    }

    /// Returns the current frame allocator skeleton.
    #[must_use]
    pub const fn frame_allocator() -> FrameAllocator {
        FrameAllocator::new()
    }

    #[cfg(test)]
    mod tests {
        use super::{
            FrameAllocator, HeapStrategy, State, frame_allocator, init, is_initialized,
            state_summary,
        };

        #[test]
        fn new_state_starts_uninitialized() {
            let state = State::new();

            assert!(!state.is_initialized());
            assert!(!state.is_frame_allocator_ready());
            assert_eq!(state.heap_strategy(), HeapStrategy::Deferred);
        }

        #[test]
        fn init_returns_ready_memory_foundation() {
            let state = init();

            assert!(state.is_initialized());
            assert!(state.is_frame_allocator_ready());
            assert_eq!(state.heap_strategy(), HeapStrategy::Deferred);
            assert_eq!(state_summary(state), "rustos: memory foundation ready");
        }

        #[test]
        fn module_initialized_helper_matches_init_state() {
            assert!(is_initialized());
        }

        #[test]
        fn frame_allocator_starts_at_zero() {
            let allocator = FrameAllocator::new();

            assert_eq!(allocator.next_frame(), 0);
            assert_eq!(frame_allocator().next_frame(), 0);
        }

        #[test]
        fn frame_allocator_reserve_advances_next_frame() {
            let allocator = FrameAllocator::new();
            let allocator = allocator.reserve();
            let allocator = allocator.reserve();

            assert_eq!(allocator.next_frame(), 2);
        }
    }
}
