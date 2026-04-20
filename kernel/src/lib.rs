#![no_std]

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
pub mod panic;

/// Deterministic boot message printed during early startup.
pub const BOOT_MESSAGE: &str = "rustos: boot start";

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

/// Runtime log message printed after exception groundwork completes.
pub const EXCEPTION_INIT_COMPLETE_MESSAGE: &str = "rustos: exception init complete";

/// Runtime log message printed before a controlled exception test begins.
pub const EXCEPTION_TEST_START_MESSAGE: &str = "rustos: exception test start";

/// Runtime log message printed when a controlled breakpoint test is requested.
pub const BREAKPOINT_TEST_MESSAGE: &str = "rustos: breakpoint test";

/// Runtime log message printed after a controlled exception test completes.
pub const EXCEPTION_TEST_COMPLETE_MESSAGE: &str = "rustos: exception test complete";

/// Runtime log message printed before interrupt groundwork begins.
pub const INTERRUPT_INIT_MESSAGE: &str = "rustos: interrupt init";

/// Runtime log message printed while interrupt groundwork is still pending.
pub const INTERRUPT_INIT_PENDING_MESSAGE: &str = "rustos: interrupt init pending";

/// Runtime log message printed after interrupt groundwork completes.
pub const INTERRUPT_INIT_COMPLETE_MESSAGE: &str = "rustos: interrupt init complete";

/// Runtime log message printed before timer groundwork begins.
pub const TIMER_INIT_MESSAGE: &str = "rustos: timer init";

/// Runtime log message printed while timer groundwork is still pending.
pub const TIMER_INIT_PENDING_MESSAGE: &str = "rustos: timer init pending";

/// Runtime log message printed after timer groundwork completes.
pub const TIMER_INIT_COMPLETE_MESSAGE: &str = "rustos: timer init complete";

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

/// Runtime log message printed when frame allocator setup begins.
pub const FRAME_ALLOCATOR_INIT_MESSAGE: &str = "rustos: frame allocator init";

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
