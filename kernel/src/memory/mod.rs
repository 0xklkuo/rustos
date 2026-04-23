//! Memory subsystem entry points for `rustos`.
//!
//! This module keeps the kernel-side memory boundary small and explicit.
//! Host-testable state and bookkeeping stay in `nucleus`, while this module
//! adds the smallest UEFI-facing discovery step needed for the current
//! milestone.

pub use nucleus::memory::{
    DiscoveredMemory, FrameAllocator, FrameAllocatorSeed, HeapStrategy, State,
    discovered_memory_summary, frame_allocator, frame_allocator_seed, frame_allocator_seed_summary,
    init as init_state, is_initialized, state_summary,
};

#[cfg(target_os = "uefi")]
use uefi::boot;
#[cfg(target_os = "uefi")]
use uefi::mem::memory_map::MemoryMap;
#[cfg(target_os = "uefi")]
use uefi::mem::memory_map::MemoryType;

#[cfg(target_os = "uefi")]
const UEFI_PAGE_SIZE: u64 = 4096;

/// Small kernel-side memory initialization result.
///
/// This keeps the current milestone explicit:
/// - host-testable memory state still comes from `nucleus`
/// - discovered memory information comes from the real UEFI memory map
/// - heap support remains deferred
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitResult {
    state: State,
    discovered: DiscoveredMemory,
    frame_allocator_seed: FrameAllocatorSeed,
}

impl InitResult {
    /// Creates a new memory initialization result.
    #[must_use]
    pub const fn new(
        state: State,
        discovered: DiscoveredMemory,
        frame_allocator_seed: FrameAllocatorSeed,
    ) -> Self {
        Self {
            state,
            discovered,
            frame_allocator_seed,
        }
    }

    /// Returns the current memory subsystem state.
    #[must_use]
    pub const fn state(self) -> State {
        self.state
    }

    /// Returns the discovered memory summary.
    #[must_use]
    pub const fn discovered(self) -> DiscoveredMemory {
        self.discovered
    }

    /// Returns the minimal frame allocator seed derived from discovered memory.
    #[must_use]
    pub const fn frame_allocator_seed(self) -> FrameAllocatorSeed {
        self.frame_allocator_seed
    }
}

/// Performs the current memory initialization step.
///
/// On the UEFI target, this reads the real firmware memory map and records a
/// small discovered-memory summary. On non-UEFI builds, it falls back to the
/// host-testable state only.
#[must_use]
pub fn init() -> InitResult {
    let state = init_state();
    let discovered = discover_memory();
    let frame_allocator_seed = frame_allocator_seed(discovered);

    InitResult::new(state, discovered, frame_allocator_seed)
}

/// Returns a small plain-language summary of discovered memory information.
#[must_use]
pub const fn discovered_summary(result: InitResult) -> &'static str {
    discovered_memory_summary(result.discovered())
}

/// Returns a small plain-language summary of the current memory state.
#[must_use]
pub const fn init_summary(result: InitResult) -> &'static str {
    state_summary(result.state())
}

/// Returns the current discovered-memory summary without changing memory state.
#[must_use]
pub fn discover() -> DiscoveredMemory {
    discover_memory()
}

/// Returns a small plain-language summary of discovered memory information.
#[must_use]
pub const fn discovered_memory_counts(memory: DiscoveredMemory) -> &'static str {
    if memory.conventional_regions() > 0 {
        crate::DISCOVERED_CONVENTIONAL_MEMORY_MESSAGE
    } else if memory.descriptor_count() > 0 {
        crate::DISCOVERED_MEMORY_MAP_MESSAGE
    } else {
        crate::DISCOVERED_MEMORY_PENDING_MESSAGE
    }
}

/// Returns a small plain-language summary of the current frame allocator seed.
#[must_use]
pub const fn frame_allocator_seed_status(result: InitResult) -> &'static str {
    if result.frame_allocator_seed().is_empty() {
        crate::FRAME_ALLOCATOR_SEED_PENDING_MESSAGE
    } else {
        crate::FRAME_ALLOCATOR_SEED_READY_MESSAGE
    }
}

#[cfg(target_os = "uefi")]
fn discover_memory() -> DiscoveredMemory {
    let memory_map = match boot::memory_map(MemoryType::LOADER_DATA) {
        Ok(memory_map) => memory_map,
        Err(_) => return DiscoveredMemory::new(),
    };

    let mut discovered = DiscoveredMemory::new();

    for descriptor in memory_map.entries() {
        let bytes = descriptor.page_count * UEFI_PAGE_SIZE;

        if descriptor.ty == MemoryType::CONVENTIONAL {
            let start_frame = (descriptor.phys_start / UEFI_PAGE_SIZE) as usize;
            let frame_count = descriptor.page_count as usize;
            discovered = discovered.record_conventional_range(start_frame, frame_count, bytes);
        } else {
            discovered = discovered.record_descriptor();
        }
    }

    discovered
}

#[cfg(not(target_os = "uefi"))]
fn discover_memory() -> DiscoveredMemory {
    DiscoveredMemory::new()
}
