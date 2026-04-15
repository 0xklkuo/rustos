//! Panic support for the `rustos` kernel.
//!
//! This module keeps panic-related behavior in one place so the early boot
//! entry path stays small and easy to read.

use core::panic::PanicInfo;

/// Returns a short panic label used by early boot output.
#[must_use]
pub const fn panic_label() -> &'static str {
    "rustos: panic"
}

/// Returns a small plain-language panic description.
///
/// This helper stays intentionally minimal for the current milestone.
/// It avoids deeper panic message inspection so the panic path remains
/// simple and compatible with the current toolchain surface.
#[must_use]
pub fn panic_message(_info: &PanicInfo<'_>) -> &'static str {
    "rustos: panic"
}
