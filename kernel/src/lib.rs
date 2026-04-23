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

/// Deterministic boot message printed during early startup.
pub const BOOT_MESSAGE: &str = "rustos: boot start";

/// Boot mode marker used for the normal boot flow.
pub const BOOT_MODE_NORMAL: &str = "rustos: boot mode normal";

/// Boot mode marker used for the controlled exception test flow.
pub const BOOT_MODE_EXCEPTION_TEST: &str = "rustos: boot mode exception-test";

/// Runtime log message printed when runtime initialization begins.
pub const RUNTIME_INIT_START_MESSAGE: &str = "rustos: runtime init start";

/// Runtime log message printed when runtime initialization completes.
pub const RUNTIME_INIT_COMPLETE_MESSAGE: &str = "rustos: runtime init complete";

/// Runtime log message printed before architecture-specific setup begins.
pub const ARCH_INIT_START_MESSAGE: &str = "rustos: arch init start";

/// Runtime log message printed after architecture-specific setup completes.
pub const ARCH_INIT_COMPLETE_MESSAGE: &str = "rustos: arch init complete";

/// Runtime log message printed before exception groundwork begins.
pub const EXCEPTION_INIT_MESSAGE: &str = "rustos: exception init";

/// Runtime log message printed while exception groundwork is still pending.
pub const EXCEPTION_INIT_PENDING_MESSAGE: &str = "rustos: exception init pending";

/// Runtime log message printed after modeled exception groundwork completes.
pub const EXCEPTION_INIT_COMPLETE_MESSAGE: &str = "rustos: exception groundwork modeled";

/// Runtime log message printed after modeled interrupt groundwork completes.
pub const INTERRUPT_GROUNDWORK_MODELED_MESSAGE: &str = "rustos: interrupt groundwork modeled";

/// Runtime log message printed after modeled timer groundwork completes.
pub const TIMER_GROUNDWORK_MODELED_MESSAGE: &str = "rustos: timer groundwork modeled";

/// Runtime log message printed when real exception handlers are installed.
pub const EXCEPTION_HANDLERS_INSTALLED_MESSAGE: &str = "rustos: exception handlers installed";

/// Runtime log message printed when the first real breakpoint handler path is active.
pub const BREAKPOINT_HANDLER_ACTIVE_MESSAGE: &str = "rustos: breakpoint handler active";

/// Runtime log message printed before a controlled exception test begins.
pub const EXCEPTION_TEST_START_MESSAGE: &str = "rustos: exception test start";

/// Runtime log message printed when a controlled breakpoint test is requested.
pub const BREAKPOINT_TEST_MESSAGE: &str = "rustos: breakpoint test";

/// Runtime log message printed when the real breakpoint handler is reached.
pub const BREAKPOINT_HANDLER_REACHED_MESSAGE: &str = "rustos: breakpoint handler reached";

/// Runtime log message printed after a controlled exception test completes.
pub const EXCEPTION_TEST_COMPLETE_MESSAGE: &str = "rustos: exception test complete";

/// Runtime log message printed before interrupt groundwork begins.
pub const INTERRUPT_INIT_MESSAGE: &str = "rustos: interrupt init";

/// Runtime log message printed while interrupt groundwork is still pending.
pub const INTERRUPT_INIT_PENDING_MESSAGE: &str = "rustos: interrupt init pending";

/// Runtime log message printed after modeled interrupt groundwork completes.
pub const INTERRUPT_INIT_COMPLETE_MESSAGE: &str = INTERRUPT_GROUNDWORK_MODELED_MESSAGE;

/// Runtime log message printed before timer groundwork begins.
pub const TIMER_INIT_MESSAGE: &str = "rustos: timer init";

/// Runtime log message printed while timer groundwork is still pending.
pub const TIMER_INIT_PENDING_MESSAGE: &str = "rustos: timer init pending";

/// Runtime log message printed after modeled timer groundwork completes.
pub const TIMER_INIT_COMPLETE_MESSAGE: &str = TIMER_GROUNDWORK_MODELED_MESSAGE;

/// Runtime log message printed before memory groundwork begins.
pub const MEMORY_INIT_MESSAGE: &str = "rustos: memory init";

/// Runtime log message printed while memory groundwork is still pending.
pub const MEMORY_INIT_PENDING_MESSAGE: &str = "rustos: memory init pending";

/// Runtime log message printed after memory groundwork completes.
pub const MEMORY_INIT_COMPLETE_MESSAGE: &str = "rustos: memory init complete";

/// Runtime log message printed when memory groundwork is deferred.
pub const MEMORY_INIT_DEFERRED_MESSAGE: &str = "rustos: memory init deferred";

/// Runtime log message printed when memory map discovery begins.
pub const MEMORY_MAP_INIT_MESSAGE: &str = "rustos: memory map init";

/// Runtime log message printed when discovered memory information is still pending.
pub const DISCOVERED_MEMORY_PENDING_MESSAGE: &str = "rustos: discovered memory pending";

/// Runtime log message printed when a memory map has been discovered.
pub const DISCOVERED_MEMORY_MAP_MESSAGE: &str = "rustos: discovered memory map";

/// Runtime log message printed when conventional memory has been discovered.
pub const DISCOVERED_CONVENTIONAL_MEMORY_MESSAGE: &str = "rustos: discovered conventional memory";

/// Runtime log message printed when the first conventional memory range is known.
pub const FIRST_CONVENTIONAL_RANGE_MESSAGE: &str = "rustos: first conventional range discovered";

/// Runtime log message printed when frame allocator setup begins.
pub const FRAME_ALLOCATOR_INIT_MESSAGE: &str = "rustos: frame allocator init";

/// Runtime log message printed when the frame allocator seed is still pending.
pub const FRAME_ALLOCATOR_SEED_PENDING_MESSAGE: &str = "rustos: frame allocator seed pending";

/// Runtime log message printed when the frame allocator seed is ready.
pub const FRAME_ALLOCATOR_SEED_READY_MESSAGE: &str = "rustos: frame allocator seed ready";

/// Runtime log message printed before paging groundwork begins.
pub const PAGING_INIT_MESSAGE: &str = "rustos: paging init";

/// Runtime log message printed when paging work is still deferred.
pub const PAGING_INIT_DEFERRED_MESSAGE: &str = "rustos: paging deferred";

/// Runtime log message printed when paging direction is defined.
pub const PAGING_DIRECTION_DEFINED_MESSAGE: &str = "rustos: paging direction defined";

/// Runtime log message printed when a small architecture-facing paging probe is ready.
pub const PAGING_ARCH_PROBE_READY_MESSAGE: &str = "rustos: paging arch probe ready";

/// Runtime log message printed when heap setup is still deferred.
pub const HEAP_INIT_DEFERRED_MESSAGE: &str = "rustos: heap init deferred";

/// Runtime log message printed before entering the idle path.
pub const IDLE_READY_MESSAGE: &str = "rustos: idle ready";

/// Deterministic greeting printed after basic UEFI initialization.
pub const HELLO_MESSAGE: &str = "rustos: hello from UEFI";

/// Returns the project name used by the kernel.
#[must_use]
pub const fn kernel_name() -> &'static str {
    "rustos"
}
