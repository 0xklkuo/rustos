#![no_main]
#![no_std]

//! UEFI entrypoint for the `rustos` kernel binary.
//!
//! This binary keeps the firmware-facing entry path intentionally small.
//! The actual boot flow lives in `kernel::boot`, so this file only provides
//! the UEFI entry symbol and delegates immediately to the shared boot logic.

use kernel::boot;

/// UEFI entrypoint for the `rustos` kernel binary.
///
/// This function is called by firmware after the UEFI application is loaded.
/// It delegates directly to `kernel::boot::run()` so the boot sequence stays
/// centralized in the `kernel` crate.
#[uefi::entry]
fn efi_main() -> uefi::Status {
    boot::run()
}
