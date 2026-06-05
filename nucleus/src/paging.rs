//! Minimal paging direction that is safe to test on the host.
//!
//! This module does not implement real page-table management yet. It only
//! defines the smallest useful paging concepts and helpers needed to make the
//! current paging boundary explicit and testable.

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
        (align_up(bytes) / PAGE_SIZE_4K) as usize
    }
}

/// Returns a minimal page range for the given byte count starting at the
/// address aligned down from `start`.
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
        HeapStrategy, PAGE_SIZE_4K, PageRange, PhysicalAddress, State, VirtualAddress, align_down,
        align_up, heap_strategy_summary, init, init_arch_probe, is_page_aligned,
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
