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

    /// Small host-testable summary of discovered memory information.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DiscoveredMemory {
        descriptor_count: usize,
        conventional_regions: usize,
        conventional_bytes: u64,
        first_conventional_start_frame: usize,
        first_conventional_frame_count: usize,
    }

    /// Small host-testable seed for the future frame allocator direction.
    ///
    /// This type does not allocate frames yet. It only records the smallest
    /// useful information derived from discovered conventional memory.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FrameAllocatorSeed {
        start_frame: usize,
        frame_count: usize,
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

    impl DiscoveredMemory {
        /// Creates a new empty discovered-memory summary.
        #[must_use]
        pub const fn new() -> Self {
            Self {
                descriptor_count: 0,
                conventional_regions: 0,
                conventional_bytes: 0,
                first_conventional_start_frame: 0,
                first_conventional_frame_count: 0,
            }
        }

        /// Returns a discovered-memory summary with explicit values.
        #[must_use]
        pub const fn from_counts(
            descriptor_count: usize,
            conventional_regions: usize,
            conventional_bytes: u64,
        ) -> Self {
            Self {
                descriptor_count,
                conventional_regions,
                conventional_bytes,
                first_conventional_start_frame: 0,
                first_conventional_frame_count: 0,
            }
        }

        /// Returns the number of memory descriptors observed.
        #[must_use]
        pub const fn descriptor_count(self) -> usize {
            self.descriptor_count
        }

        /// Returns the number of conventional memory regions observed.
        #[must_use]
        pub const fn conventional_regions(self) -> usize {
            self.conventional_regions
        }

        /// Returns the total conventional memory bytes observed.
        #[must_use]
        pub const fn conventional_bytes(self) -> u64 {
            self.conventional_bytes
        }

        /// Returns a new summary after recording one descriptor.
        #[must_use]
        pub const fn record_descriptor(self) -> Self {
            Self {
                descriptor_count: self.descriptor_count + 1,
                conventional_regions: self.conventional_regions,
                conventional_bytes: self.conventional_bytes,
                first_conventional_start_frame: self.first_conventional_start_frame,
                first_conventional_frame_count: self.first_conventional_frame_count,
            }
        }

        /// Returns the first conventional memory start frame, if known.
        #[must_use]
        pub const fn first_conventional_start_frame(self) -> usize {
            self.first_conventional_start_frame
        }

        /// Returns the first conventional memory frame count, if known.
        #[must_use]
        pub const fn first_conventional_frame_count(self) -> usize {
            self.first_conventional_frame_count
        }

        /// Returns whether the first conventional memory range is known.
        #[must_use]
        pub const fn has_first_conventional_range(self) -> bool {
            self.first_conventional_frame_count > 0
        }

        /// Returns a new summary after recording one conventional region.
        #[must_use]
        pub const fn record_conventional_region(self, bytes: u64) -> Self {
            Self {
                descriptor_count: self.descriptor_count + 1,
                conventional_regions: self.conventional_regions + 1,
                conventional_bytes: self.conventional_bytes + bytes,
                first_conventional_start_frame: self.first_conventional_start_frame,
                first_conventional_frame_count: self.first_conventional_frame_count,
            }
        }

        /// Returns a new summary after recording one conventional region with an explicit frame range.
        #[must_use]
        pub const fn record_conventional_range(
            self,
            start_frame: usize,
            frame_count: usize,
            bytes: u64,
        ) -> Self {
            let has_first_range = self.has_first_conventional_range();

            Self {
                descriptor_count: self.descriptor_count + 1,
                conventional_regions: self.conventional_regions + 1,
                conventional_bytes: self.conventional_bytes + bytes,
                first_conventional_start_frame: if has_first_range {
                    self.first_conventional_start_frame
                } else {
                    start_frame
                },
                first_conventional_frame_count: if has_first_range {
                    self.first_conventional_frame_count
                } else {
                    frame_count
                },
            }
        }
    }

    impl Default for DiscoveredMemory {
        fn default() -> Self {
            Self::new()
        }
    }

    impl FrameAllocatorSeed {
        /// Creates an empty frame allocator seed.
        #[must_use]
        pub const fn new() -> Self {
            Self {
                start_frame: 0,
                frame_count: 0,
            }
        }

        /// Creates a frame allocator seed with explicit values.
        #[must_use]
        pub const fn from_range(start_frame: usize, frame_count: usize) -> Self {
            Self {
                start_frame,
                frame_count,
            }
        }

        /// Returns the first frame index in the seed.
        #[must_use]
        pub const fn start_frame(self) -> usize {
            self.start_frame
        }

        /// Returns the number of frames in the seed.
        #[must_use]
        pub const fn frame_count(self) -> usize {
            self.frame_count
        }

        /// Returns whether the seed contains any frames.
        #[must_use]
        pub const fn is_empty(self) -> bool {
            self.frame_count == 0
        }
    }

    impl Default for FrameAllocatorSeed {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Returns a small plain-language summary of discovered memory information.
    #[must_use]
    pub const fn discovered_memory_summary(memory: DiscoveredMemory) -> &'static str {
        if memory.conventional_regions() > 0 {
            "rustos: discovered conventional memory"
        } else if memory.descriptor_count() > 0 {
            "rustos: discovered memory map"
        } else {
            "rustos: discovered memory pending"
        }
    }

    /// Returns a minimal frame allocator seed derived from discovered memory.
    ///
    /// The current rule is intentionally small:
    /// - if no conventional memory is known, return an empty seed
    /// - otherwise start at frame 0
    /// - derive the frame count from the discovered conventional bytes
    #[must_use]
    pub const fn frame_allocator_seed(memory: DiscoveredMemory) -> FrameAllocatorSeed {
        if memory.has_first_conventional_range() {
            FrameAllocatorSeed::from_range(
                memory.first_conventional_start_frame(),
                memory.first_conventional_frame_count(),
            )
        } else {
            FrameAllocatorSeed::new()
        }
    }

    /// Returns a small plain-language summary of the current frame allocator seed.
    #[must_use]
    pub const fn frame_allocator_seed_summary(seed: FrameAllocatorSeed) -> &'static str {
        if seed.is_empty() {
            "rustos: frame allocator seed pending"
        } else {
            "rustos: frame allocator seed ready"
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
            DiscoveredMemory, FrameAllocator, FrameAllocatorSeed, HeapStrategy, State,
            discovered_memory_summary, frame_allocator, frame_allocator_seed,
            frame_allocator_seed_summary, init, is_initialized, state_summary,
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

        #[test]
        fn discovered_memory_starts_empty() {
            let memory = DiscoveredMemory::new();

            assert_eq!(memory.descriptor_count(), 0);
            assert_eq!(memory.conventional_regions(), 0);
            assert_eq!(memory.conventional_bytes(), 0);
            assert_eq!(
                discovered_memory_summary(memory),
                "rustos: discovered memory pending"
            );
        }

        #[test]
        fn discovered_memory_records_descriptor_and_region_counts() {
            let memory = DiscoveredMemory::new();
            let memory = memory.record_descriptor();
            let memory = memory.record_conventional_region(4096);

            assert_eq!(memory.descriptor_count(), 2);
            assert_eq!(memory.conventional_regions(), 1);
            assert_eq!(memory.conventional_bytes(), 4096);
            assert_eq!(memory.first_conventional_start_frame(), 0);
            assert_eq!(memory.first_conventional_frame_count(), 0);
            assert_eq!(
                discovered_memory_summary(memory),
                "rustos: discovered conventional memory"
            );
        }

        #[test]
        fn discovered_memory_from_counts_preserves_values() {
            let memory = DiscoveredMemory::from_counts(4, 2, 8192);

            assert_eq!(memory.descriptor_count(), 4);
            assert_eq!(memory.conventional_regions(), 2);
            assert_eq!(memory.conventional_bytes(), 8192);
            assert_eq!(memory.first_conventional_start_frame(), 0);
            assert_eq!(memory.first_conventional_frame_count(), 0);
            assert_eq!(
                discovered_memory_summary(memory),
                "rustos: discovered conventional memory"
            );
        }

        #[test]
        fn discovered_memory_summary_reports_map_without_conventional_regions() {
            let memory = DiscoveredMemory::from_counts(3, 0, 0);

            assert_eq!(
                discovered_memory_summary(memory),
                "rustos: discovered memory map"
            );
        }

        #[test]
        fn frame_allocator_seed_starts_empty() {
            let seed = FrameAllocatorSeed::new();

            assert_eq!(seed.start_frame(), 0);
            assert_eq!(seed.frame_count(), 0);
            assert!(seed.is_empty());
            assert_eq!(
                frame_allocator_seed_summary(seed),
                "rustos: frame allocator seed pending"
            );
        }

        #[test]
        fn frame_allocator_seed_derives_frame_count_from_conventional_memory() {
            let memory = DiscoveredMemory::new().record_conventional_range(16, 2, 8192);
            let seed = frame_allocator_seed(memory);

            assert_eq!(seed.start_frame(), 16);
            assert_eq!(seed.frame_count(), 2);
            assert!(!seed.is_empty());
            assert_eq!(
                frame_allocator_seed_summary(seed),
                "rustos: frame allocator seed ready"
            );
        }

        #[test]
        fn frame_allocator_seed_stays_empty_without_full_frame() {
            let memory = DiscoveredMemory::from_counts(1, 1, 2048);
            let seed = frame_allocator_seed(memory);

            assert_eq!(seed, FrameAllocatorSeed::new());
            assert_eq!(
                frame_allocator_seed_summary(seed),
                "rustos: frame allocator seed pending"
            );
        }

        #[test]
        fn discovered_memory_records_first_conventional_range_once() {
            let memory = DiscoveredMemory::new();
            let memory = memory.record_conventional_range(8, 4, 16384);
            let memory = memory.record_conventional_range(32, 2, 8192);

            assert!(memory.has_first_conventional_range());
            assert_eq!(memory.first_conventional_start_frame(), 8);
            assert_eq!(memory.first_conventional_frame_count(), 4);
            assert_eq!(memory.conventional_regions(), 2);
            assert_eq!(memory.conventional_bytes(), 24576);
        }
    }
}

pub mod paging {
    //! Minimal paging direction that is safe to test on the host.
    //!
    //! This module does not implement real page-table management yet.
    //! It only defines the smallest useful paging concepts and helpers needed
    //! to make the U5 milestone explicit and testable.

    /// The smallest supported page size for the current milestone.
    pub const PAGE_SIZE_4K: u64 = 4096;

    /// Small summary of the current paging subsystem state.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum State {
        /// Paging work is still deferred.
        Deferred,
        /// Paging direction is defined, but real mappings are not managed yet.
        DirectionDefined,
        /// A small architecture-facing paging probe boundary is ready.
        ArchProbeReady,
    }

    /// Minimal heap strategy decision for paging-related work.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum HeapStrategy {
        /// Heap support remains intentionally deferred.
        Deferred,
    }

    /// Small virtual address wrapper for host-testable paging helpers.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VirtualAddress(u64);

    /// Small physical address wrapper for host-testable paging helpers.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PhysicalAddress(u64);

    /// Small page range summary used for future paging direction.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PageRange {
        start: u64,
        page_count: usize,
    }

    impl VirtualAddress {
        /// Creates a new virtual address.
        #[must_use]
        pub const fn new(value: u64) -> Self {
            Self(value)
        }

        /// Returns the raw address value.
        #[must_use]
        pub const fn as_u64(self) -> u64 {
            self.0
        }

        /// Returns whether the address is 4 KiB page aligned.
        #[must_use]
        pub const fn is_page_aligned(self) -> bool {
            is_page_aligned(self.0)
        }

        /// Returns the address aligned down to the nearest 4 KiB page boundary.
        #[must_use]
        pub const fn align_down(self) -> Self {
            Self(align_down(self.0))
        }

        /// Returns the address aligned up to the nearest 4 KiB page boundary.
        #[must_use]
        pub const fn align_up(self) -> Self {
            Self(align_up(self.0))
        }
    }

    impl PhysicalAddress {
        /// Creates a new physical address.
        #[must_use]
        pub const fn new(value: u64) -> Self {
            Self(value)
        }

        /// Returns the raw address value.
        #[must_use]
        pub const fn as_u64(self) -> u64 {
            self.0
        }

        /// Returns whether the address is 4 KiB page aligned.
        #[must_use]
        pub const fn is_page_aligned(self) -> bool {
            is_page_aligned(self.0)
        }

        /// Returns the address aligned down to the nearest 4 KiB page boundary.
        #[must_use]
        pub const fn align_down(self) -> Self {
            Self(align_down(self.0))
        }

        /// Returns the address aligned up to the nearest 4 KiB page boundary.
        #[must_use]
        pub const fn align_up(self) -> Self {
            Self(align_up(self.0))
        }
    }

    impl PageRange {
        /// Creates a new page range from an aligned start address and page count.
        #[must_use]
        pub const fn new(start: u64, page_count: usize) -> Self {
            Self { start, page_count }
        }

        /// Returns the aligned start address of the range.
        #[must_use]
        pub const fn start(self) -> u64 {
            self.start
        }

        /// Returns the number of pages in the range.
        #[must_use]
        pub const fn page_count(self) -> usize {
            self.page_count
        }

        /// Returns whether the range is empty.
        #[must_use]
        pub const fn is_empty(self) -> bool {
            self.page_count == 0
        }
    }

    /// Returns the current minimal paging state.
    #[must_use]
    pub const fn init() -> State {
        State::DirectionDefined
    }

    /// Returns the current paging state after a small architecture-facing probe boundary is ready.
    #[must_use]
    pub const fn init_arch_probe() -> State {
        State::ArchProbeReady
    }

    /// Returns whether the given address is aligned to a 4 KiB page boundary.
    #[must_use]
    pub const fn is_page_aligned(address: u64) -> bool {
        address.is_multiple_of(PAGE_SIZE_4K)
    }

    /// Returns the given address aligned down to a 4 KiB page boundary.
    #[must_use]
    pub const fn align_down(address: u64) -> u64 {
        address - (address % PAGE_SIZE_4K)
    }

    /// Returns the given address aligned up to a 4 KiB page boundary.
    #[must_use]
    pub const fn align_up(address: u64) -> u64 {
        if is_page_aligned(address) {
            address
        } else {
            align_down(address) + PAGE_SIZE_4K
        }
    }

    /// Returns the number of 4 KiB pages needed to cover the given byte count.
    #[must_use]
    pub const fn page_count_for_bytes(bytes: u64) -> usize {
        if bytes == 0 {
            0
        } else {
            ((align_up(bytes)) / PAGE_SIZE_4K) as usize
        }
    }

    /// Returns a minimal page range covering the given byte span.
    #[must_use]
    pub const fn page_range(start: u64, bytes: u64) -> PageRange {
        PageRange::new(align_down(start), page_count_for_bytes(bytes))
    }

    /// Returns a small plain-language summary of the current paging state.
    #[must_use]
    pub const fn state_summary(state: State) -> &'static str {
        match state {
            State::Deferred => "rustos: paging deferred",
            State::DirectionDefined => "rustos: paging direction defined",
            State::ArchProbeReady => "rustos: paging arch probe ready",
        }
    }

    /// Returns a small plain-language summary of the current heap strategy.
    #[must_use]
    pub const fn heap_strategy_summary(strategy: HeapStrategy) -> &'static str {
        match strategy {
            HeapStrategy::Deferred => "rustos: heap init deferred",
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{
            HeapStrategy, PAGE_SIZE_4K, PageRange, PhysicalAddress, State, VirtualAddress,
            align_down, align_up, heap_strategy_summary, init, init_arch_probe, is_page_aligned,
            page_count_for_bytes, page_range, state_summary,
        };

        #[test]
        fn page_size_constant_matches_expected_value() {
            assert_eq!(PAGE_SIZE_4K, 4096);
        }

        #[test]
        fn paging_init_reports_direction_defined() {
            assert_eq!(init(), State::DirectionDefined);
            assert_eq!(state_summary(init()), "rustos: paging direction defined");
        }

        #[test]
        fn paging_arch_probe_reports_ready() {
            assert_eq!(init_arch_probe(), State::ArchProbeReady);
            assert_eq!(
                state_summary(init_arch_probe()),
                "rustos: paging arch probe ready"
            );
        }

        #[test]
        fn deferred_paging_state_reports_deferred() {
            assert_eq!(state_summary(State::Deferred), "rustos: paging deferred");
        }

        #[test]
        fn deferred_heap_strategy_reports_deferred() {
            assert_eq!(
                heap_strategy_summary(HeapStrategy::Deferred),
                "rustos: heap init deferred"
            );
        }

        #[test]
        fn page_alignment_helpers_work_for_aligned_addresses() {
            assert!(is_page_aligned(PAGE_SIZE_4K));
            assert_eq!(align_down(PAGE_SIZE_4K), PAGE_SIZE_4K);
            assert_eq!(align_up(PAGE_SIZE_4K), PAGE_SIZE_4K);
        }

        #[test]
        fn page_alignment_helpers_work_for_unaligned_addresses() {
            assert!(!is_page_aligned(4097));
            assert_eq!(align_down(4097), 4096);
            assert_eq!(align_up(4097), 8192);
        }

        #[test]
        fn page_count_for_bytes_rounds_up() {
            assert_eq!(page_count_for_bytes(0), 0);
            assert_eq!(page_count_for_bytes(1), 1);
            assert_eq!(page_count_for_bytes(4096), 1);
            assert_eq!(page_count_for_bytes(4097), 2);
        }

        #[test]
        fn page_range_uses_aligned_start_and_rounded_page_count() {
            let range = page_range(4097, 5000);

            assert_eq!(range.start(), 4096);
            assert_eq!(range.page_count(), 2);
            assert!(!range.is_empty());
        }

        #[test]
        fn empty_page_range_reports_empty() {
            let range = PageRange::new(0, 0);

            assert!(range.is_empty());
        }

        #[test]
        fn virtual_address_helpers_wrap_alignment_logic() {
            let address = VirtualAddress::new(4097);

            assert_eq!(address.as_u64(), 4097);
            assert!(!address.is_page_aligned());
            assert_eq!(address.align_down().as_u64(), 4096);
            assert_eq!(address.align_up().as_u64(), 8192);
        }

        #[test]
        fn physical_address_helpers_wrap_alignment_logic() {
            let address = PhysicalAddress::new(12289);

            assert_eq!(address.as_u64(), 12289);
            assert!(!address.is_page_aligned());
            assert_eq!(address.align_down().as_u64(), 12288);
            assert_eq!(address.align_up().as_u64(), 16384);
        }
    }
}
