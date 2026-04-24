//! Paging subsystem entry points for `rustos`.
//!
//! This module is the kernel-facing paging boundary.
//! Host-testable paging state and helpers live in `nucleus`, while this module
//! adds the smallest architecture-facing probe needed by the kernel runtime.
//!
//! This module does not implement page-table management, mapping, or
//! heap-backed paging structures. It exposes:
//! - paging direction state
//! - small paging helpers re-exported from `nucleus`
//! - a narrow architecture-facing probe summary for boot-time reporting

pub use nucleus::paging::{
    HeapStrategy, PageRange, PhysicalAddress, State, VirtualAddress, align_down, align_up,
    heap_strategy_summary, init as init_state, init_arch_probe, is_page_aligned,
    page_count_for_bytes, page_range, state_summary,
};

/// Small kernel-side paging initialization result.
///
/// This type keeps the kernel-facing paging state explicit:
/// - `state` reports the current paging direction
/// - `arch_probe_ready` reports whether the architecture layer can observe a
///   minimal paging-facing runtime boundary
///
/// When `arch_probe_ready` is `true`, `state` is expected to be
/// `State::ArchProbeReady`.
/// When `arch_probe_ready` is `false`, `state` is expected to remain at the
/// direction-only level.
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
/// This function does not modify page tables or install mappings.
/// It only reports the current kernel-facing paging boundary:
/// - if an architecture-facing paging probe is available, return
///   `State::ArchProbeReady`
/// - otherwise return the direction-only paging state
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
///
/// This summary reports the subsystem direction first, even when the
/// architecture-facing probe is also ready. That keeps the boot log explicit:
/// - paging direction is defined
/// - architecture-facing probe readiness is reported separately
#[must_use]
pub const fn init_summary(result: InitResult) -> &'static str {
    match result.state() {
        State::Deferred => crate::PAGING_INIT_DEFERRED_MESSAGE,
        State::DirectionDefined | State::ArchProbeReady => crate::PAGING_DIRECTION_DEFINED_MESSAGE,
    }
}

/// Returns a small plain-language summary of the architecture-facing paging
/// probe boundary.
///
/// When the probe is ready, this returns the probe-ready message.
/// When the probe is not ready, this falls back to the paging-direction message
/// instead of a probe-specific deferred message.
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
        assert_eq!(
            init_summary(result),
            crate::PAGING_DIRECTION_DEFINED_MESSAGE
        );
        assert_eq!(
            arch_probe_summary(result),
            crate::PAGING_ARCH_PROBE_READY_MESSAGE
        );
    }
}
