#![no_std]
#![cfg_attr(target_os = "uefi", feature(abi_x86_interrupt))]

//! Shared kernel modules and small constants for `rustos`.
//!
//! This crate keeps the early kernel structure explicit and minimal.
//! The current module layout is intentionally small so future milestones
//! can grow without forcing a large refactor.

pub mod arch;
pub mod boot;
pub mod console;
pub mod interrupt;
pub mod memory;
pub mod paging;
pub mod panic;
pub mod syscall;

/// Boot and runtime log markers used across the early kernel path.
pub const BOOT_MESSAGE: &str = "rustos: boot start";
pub const BOOT_MODE_NORMAL: &str = "rustos: boot mode normal";
pub const BOOT_MODE_EXCEPTION_TEST: &str = "rustos: boot mode exception-test";
pub const RUNTIME_INIT_START_MESSAGE: &str = "rustos: runtime init start";
pub const RUNTIME_INIT_COMPLETE_MESSAGE: &str = "rustos: runtime init complete";
pub const ARCH_INIT_START_MESSAGE: &str = "rustos: arch init start";
pub const ARCH_INIT_COMPLETE_MESSAGE: &str = "rustos: arch init complete";

/// Exception and interrupt log markers.
pub const EXCEPTION_INIT_MESSAGE: &str = "rustos: exception init";
pub const EXCEPTION_INIT_PENDING_MESSAGE: &str = "rustos: exception init pending";
pub const EXCEPTION_INIT_COMPLETE_MESSAGE: &str = "rustos: exception groundwork modeled";
pub const INTERRUPT_GROUNDWORK_MODELED_MESSAGE: &str = "rustos: interrupt groundwork modeled";
pub const TIMER_GROUNDWORK_MODELED_MESSAGE: &str = "rustos: timer groundwork modeled";
pub const EXCEPTION_HANDLERS_INSTALLED_MESSAGE: &str = "rustos: exception handlers installed";
pub const BREAKPOINT_HANDLER_ACTIVE_MESSAGE: &str = "rustos: breakpoint handler active";
pub const EXCEPTION_TEST_START_MESSAGE: &str = "rustos: exception test start";
pub const BREAKPOINT_TEST_MESSAGE: &str = "rustos: breakpoint test";
pub const BREAKPOINT_HANDLER_REACHED_MESSAGE: &str = "rustos: breakpoint handler reached";
pub const EXCEPTION_TEST_COMPLETE_MESSAGE: &str = "rustos: exception test complete";
pub const INTERRUPT_INIT_MESSAGE: &str = "rustos: interrupt init";
pub const INTERRUPT_INIT_PENDING_MESSAGE: &str = "rustos: interrupt init pending";
pub const INTERRUPT_INIT_COMPLETE_MESSAGE: &str = INTERRUPT_GROUNDWORK_MODELED_MESSAGE;
pub const TIMER_INIT_MESSAGE: &str = "rustos: timer init";
pub const TIMER_INIT_PENDING_MESSAGE: &str = "rustos: timer init pending";
pub const TIMER_INIT_COMPLETE_MESSAGE: &str = TIMER_GROUNDWORK_MODELED_MESSAGE;

/// Memory and paging log markers.
pub const MEMORY_INIT_MESSAGE: &str = "rustos: memory init";
pub const MEMORY_INIT_PENDING_MESSAGE: &str = "rustos: memory init pending";
pub const MEMORY_INIT_COMPLETE_MESSAGE: &str = "rustos: memory init complete";
pub const MEMORY_INIT_DEFERRED_MESSAGE: &str = "rustos: memory init deferred";
pub const MEMORY_MAP_INIT_MESSAGE: &str = "rustos: memory map init";
pub const DISCOVERED_MEMORY_PENDING_MESSAGE: &str = "rustos: discovered memory pending";
pub const DISCOVERED_MEMORY_MAP_MESSAGE: &str = "rustos: discovered memory map";
pub const DISCOVERED_CONVENTIONAL_MEMORY_MESSAGE: &str = "rustos: discovered conventional memory";
pub const FIRST_CONVENTIONAL_RANGE_MESSAGE: &str = "rustos: first conventional range discovered";
pub const FRAME_ALLOCATOR_INIT_MESSAGE: &str = "rustos: frame allocator init";
pub const FRAME_ALLOCATOR_SEED_PENDING_MESSAGE: &str = "rustos: frame allocator seed pending";
pub const FRAME_ALLOCATOR_SEED_READY_MESSAGE: &str = "rustos: frame allocator seed ready";
pub const PAGING_INIT_MESSAGE: &str = "rustos: paging init";
pub const PAGING_INIT_DEFERRED_MESSAGE: &str = "rustos: paging deferred";
pub const PAGING_DIRECTION_DEFINED_MESSAGE: &str = "rustos: paging direction defined";
pub const PAGING_ARCH_PROBE_READY_MESSAGE: &str = "rustos: paging arch probe ready";
pub const HEAP_INIT_DEFERRED_MESSAGE: &str = "rustos: heap init deferred";

/// Syscall log markers.
pub const SYSCALL_INIT_MESSAGE: &str = "rustos: syscall init";
pub const SYSCALL_DIRECTION_DEFINED_MESSAGE: &str = "rustos: syscall direction defined";
pub const SYSCALL_BOUNDARY_READY_MESSAGE: &str = "rustos: syscall boundary ready";
pub const SYSCALL_INVALID_NUMBER_MESSAGE: &str = "rustos: syscall invalid number";
pub const SYSCALL_SUCCESS_MESSAGE: &str = "rustos: syscall success";
pub const SYSCALL_INVALID_ARGUMENT_MESSAGE: &str = "rustos: syscall invalid argument";
pub const SYSCALL_INVALID_HANDLE_MESSAGE: &str = "rustos: syscall invalid handle";

/// Final early-runtime log markers.
pub const IDLE_READY_MESSAGE: &str = "rustos: idle ready";
pub const HELLO_MESSAGE: &str = "rustos: hello from UEFI";

/// Returns the project name used by the kernel.
#[must_use]
pub const fn kernel_name() -> &'static str {
    "rustos"
}
