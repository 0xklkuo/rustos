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
