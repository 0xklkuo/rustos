#![no_std]

//! Shared kernel modules and small constants for `rustos`.
//!
//! This crate keeps the early kernel structure explicit and minimal.
//! The current module layout is intentionally small so future milestones
//! can grow without forcing a large refactor.

pub mod arch;
pub mod boot;
pub mod console;
pub mod memory;
pub mod panic;

/// Deterministic boot message printed during early startup.
pub const BOOT_MESSAGE: &str = "rustos: boot start";

/// Deterministic greeting printed after basic UEFI initialization.
pub const HELLO_MESSAGE: &str = "rustos: hello from UEFI";

/// Returns the project name used by the kernel.
#[must_use]
pub const fn kernel_name() -> &'static str {
    "rustos"
}
