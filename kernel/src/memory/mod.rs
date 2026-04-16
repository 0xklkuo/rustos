//! Minimal memory foundation for `rustos`.
//!
//! This module introduces a small, explicit memory subsystem shape for the
//! current milestone. It does not implement real memory discovery, paging,
//! or heap allocation yet, but it does provide clear placeholders for:
//! - memory initialization state
//! - frame allocator direction
//! - heap strategy decisions
//!
//! The goal is to make future memory work easier to grow without forcing a
//! large refactor.

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
        FrameAllocator, HeapStrategy, State, frame_allocator, init, is_initialized, state_summary,
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
