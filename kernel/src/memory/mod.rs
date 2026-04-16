//! Memory subsystem entry points for `rustos`.
//!
//! This module re-exports the host-testable memory foundation from
//! `kernel-core` so the kernel can share the same small, explicit logic
//! with normal unit tests.

pub use kernel_core::memory::{
    FrameAllocator, HeapStrategy, State, frame_allocator, init, is_initialized, state_summary,
};
