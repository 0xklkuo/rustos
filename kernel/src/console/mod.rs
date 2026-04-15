//! Early console support for `rustos`.
//!
//! This module keeps early output intentionally small and explicit.
//! For now it is a thin wrapper around the UEFI text console.

use uefi::Status;
use uefi::println;

/// Initializes early console support.
///
/// For the current milestone, this delegates to the UEFI helper setup so
/// printing and panic output work during early boot.
pub fn init() -> Result<(), Status> {
    uefi::helpers::init().map_err(|error| error.status())
}

/// Prints a single line to the early console.
pub fn write_line(message: &str) {
    println!("{message}");
}
