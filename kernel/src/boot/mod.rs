//! Boot entry flow for `rustos`.
//!
//! This module keeps the early UEFI boot path small and explicit.
//! The current implementation is intentionally minimal and only
//! coordinates the first visible boot steps.

use uefi::Status;

/// Runs the early boot flow.
///
/// The current flow:
/// - initializes console output
/// - prints deterministic boot messages
/// - returns success to UEFI
pub fn run() -> Status {
    if let Err(status) = crate::console::init() {
        return status;
    }

    crate::console::write_line(crate::BOOT_MESSAGE);
    crate::console::write_line(crate::HELLO_MESSAGE);

    Status::SUCCESS
}
