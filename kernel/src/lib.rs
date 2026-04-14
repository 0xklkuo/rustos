#![no_std]

//! Shared constants and small helpers for the `rustos` kernel crate.

/// Deterministic boot message printed during early startup.
pub const BOOT_MESSAGE: &str = "rustos: boot start";

/// Deterministic greeting printed after basic UEFI initialization.
pub const HELLO_MESSAGE: &str = "rustos: hello from UEFI";

/// Returns the project name used by the kernel.
#[must_use]
pub const fn kernel_name() -> &'static str {
    "rustos"
}
