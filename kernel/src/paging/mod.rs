//! Paging subsystem entry points for `rustos`.
//!
//! This module keeps the kernel-side paging boundary small and explicit.
//! Host-testable paging state and helpers live in `nucleus`, while this module
//! adds the smallest architecture-facing probe needed for the current U5
//! milestone.
//!
//! The current milestone does not implement page-table management, mapping, or
//! heap-backed paging structures. It only makes the paging direction visible in
//! code and boot logs.

pub use nucleus::paging::{
    HeapStrategy, PageRange, PhysicalAddress, State, VirtualAddress, align_down, align_up,
    heap_strategy_summary, init as init_state, init_arch_probe, is_page_aligned,
    page_count_for_bytes, page_range, state_summary,
};

/// Small kernel-side paging initialization result.
///
/// This keeps the current milestone explicit:
/// - host-testable paging state still comes from `nucleus`
/// - the kernel adds only a minimal architecture-facing probe
/// - real paging management remains deferred
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
///
/// This keeps the U5 milestone intentionally small:
/// - if an architecture-facing paging probe is available, report that boundary
/// - otherwise report that paging direction is defined but deeper work is still deferred
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
    state_summary(result.state())
}

/// Returns a small plain-language summary of the current architecture-facing
/// paging probe boundary.
#[must_use]
pub const fn arch_probe_summary(result: InitResult) -> &'static str {
    if result.is_arch_probe_ready() {
        crate::PAGING_ARCH_PROBE_READY_MESSAGE
    } else {
        crate::PAGING_DIRECTION_DEFINED_MESSAGE
    }
}

#[cfg(test)]
mod tests {
    use super::{InitResult, arch_probe_summary, init_summary};
    use nucleus::paging::State;

    #[test]
    fn init_result_reports_direction_defined_without_arch_probe() {
        let result = InitResult::new(State::DirectionDefined, false);

        assert_eq!(result.state(), State::DirectionDefined);
        assert!(!result.is_arch_probe_ready());
        assert_eq!(
            init_summary(result),
            crate::PAGING_DIRECTION_DEFINED_MESSAGE
        );
        assert_eq!(
            arch_probe_summary(result),
            crate::PAGING_DIRECTION_DEFINED_MESSAGE
        );
    }

    #[test]
    fn init_result_reports_arch_probe_ready() {
        let result = InitResult::new(State::ArchProbeReady, true);

        assert_eq!(result.state(), State::ArchProbeReady);
        assert!(result.is_arch_probe_ready());
        assert_eq!(init_summary(result), crate::PAGING_ARCH_PROBE_READY_MESSAGE);
        assert_eq!(
            arch_probe_summary(result),
            crate::PAGING_ARCH_PROBE_READY_MESSAGE
        );
    }
}
