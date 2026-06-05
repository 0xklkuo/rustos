//! Paging subsystem entry points for `rustos`.
//!
//! This module is the kernel-facing paging boundary.
//! Host-testable paging state and helpers live in `nucleus`, while this module
//! adds the smallest architecture-facing probe needed by the kernel runtime.

pub use nucleus::paging::{
    HeapStrategy, PageRange, PhysicalAddress, State, VirtualAddress, align_down, align_up,
    heap_strategy_summary, init as init_state, init_arch_probe, is_page_aligned,
    page_count_for_bytes, page_range, state_summary,
};

/// Small kernel-side paging initialization result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitResult {
    state: State,
    arch_probe_ready: bool,
}

impl InitResult {
    /// Creates a new paging initialization result.
    #[must_use]
    pub const fn new(state: State, arch_probe_ready: bool) -> Self {
        Self {
            state,
            arch_probe_ready,
        }
    }

    /// Returns the current paging subsystem state.
    #[must_use]
    pub const fn state(self) -> State {
        self.state
    }

    /// Returns whether a small architecture-facing paging probe is ready.
    #[must_use]
    pub const fn is_arch_probe_ready(self) -> bool {
        self.arch_probe_ready
    }
}

/// Performs the current paging initialization step.
#[must_use]
pub fn init() -> InitResult {
    let arch_probe_ready = crate::arch::has_paging_probe();
    let state = if arch_probe_ready {
        init_arch_probe()
    } else {
        init_state()
    };

    InitResult::new(state, arch_probe_ready)
}

/// Returns a small plain-language summary of the current paging state.
#[must_use]
pub const fn init_summary(result: InitResult) -> &'static str {
    match result.state() {
        State::Deferred => crate::PAGING_INIT_DEFERRED_MESSAGE,
        State::DirectionDefined | State::ArchProbeReady => crate::PAGING_DIRECTION_DEFINED_MESSAGE,
    }
}

/// Returns a small plain-language summary of the architecture-facing paging
/// probe boundary.
#[must_use]
pub const fn arch_probe_summary(result: InitResult) -> &'static str {
    if result.is_arch_probe_ready() {
        crate::PAGING_ARCH_PROBE_READY_MESSAGE
    } else {
        crate::PAGING_DIRECTION_DEFINED_MESSAGE
    }
}
