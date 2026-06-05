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
}

/// Small frame allocator skeleton for future memory work.
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
