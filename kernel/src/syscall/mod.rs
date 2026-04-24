//! Syscall subsystem entry points for `rustos`.
//!
//! This module keeps the kernel-side syscall boundary small and explicit.
//! Host-testable syscall logic lives in `nucleus`, while this module adds the
//! smallest kernel-facing boundary needed for the current U6 milestone.
//!
//! The current milestone does not implement a real syscall ABI, trap entry, or
//! user-mode transition. It only makes the syscall direction visible in code
//! and boot logs.

pub use nucleus::syscall::{Error, Number, Result, number_summary, result_summary};

/// Small kernel-side syscall initialization result.
///
/// This keeps the current milestone explicit:
/// - host-testable syscall logic still comes from `nucleus`
/// - the kernel exposes only a minimal syscall boundary
/// - real syscall entry and dispatch remain deferred
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitResult {
    boundary_ready: bool,
}

impl InitResult {
    /// Creates a new syscall initialization result.
    #[must_use]
    pub const fn new(boundary_ready: bool) -> Self {
        Self { boundary_ready }
    }

    /// Returns whether the minimal syscall boundary is ready.
    #[must_use]
    pub const fn is_boundary_ready(self) -> bool {
        self.boundary_ready
    }
}

/// Performs the current syscall initialization step.
///
/// This keeps the U6 milestone intentionally small:
/// - define the syscall direction explicitly
/// - expose a minimal kernel boundary
/// - defer real syscall ABI and dispatch work
#[must_use]
pub const fn init() -> InitResult {
    InitResult::new(true)
}

/// Returns a small plain-language summary of the current syscall direction.
#[must_use]
pub const fn init_summary(_result: InitResult) -> &'static str {
    crate::SYSCALL_DIRECTION_DEFINED_MESSAGE
}

/// Returns a small plain-language summary of the current syscall boundary.
#[must_use]
pub const fn boundary_summary(result: InitResult) -> &'static str {
    if result.is_boundary_ready() {
        crate::SYSCALL_BOUNDARY_READY_MESSAGE
    } else {
        crate::SYSCALL_DIRECTION_DEFINED_MESSAGE
    }
}

/// Returns a kernel-facing plain-language summary of the syscall number.
///
/// This keeps kernel boot and runtime messages aligned with the constants
/// defined in `kernel`.
#[must_use]
pub const fn kernel_number_summary(number: Number) -> &'static str {
    match number {
        Number::Write => "rustos: syscall write",
        Number::Exit => "rustos: syscall exit",
        Number::Unknown(_) => crate::SYSCALL_INVALID_NUMBER_MESSAGE,
    }
}

/// Returns a kernel-facing plain-language summary of the syscall result.
///
/// This keeps kernel boot and runtime messages aligned with the constants
/// defined in `kernel`.
#[must_use]
pub const fn kernel_result_summary(result: Result) -> &'static str {
    match result.error_kind() {
        None => crate::SYSCALL_SUCCESS_MESSAGE,
        Some(Error::InvalidNumber) => crate::SYSCALL_INVALID_NUMBER_MESSAGE,
        Some(Error::InvalidArgument) => crate::SYSCALL_INVALID_ARGUMENT_MESSAGE,
        Some(Error::InvalidHandle) => crate::SYSCALL_INVALID_HANDLE_MESSAGE,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        InitResult, boundary_summary, init, init_summary, kernel_number_summary,
        kernel_result_summary,
    };
    use nucleus::syscall::{Error, Number, Result};

    #[test]
    fn init_reports_boundary_ready() {
        let result = init();

        assert!(result.is_boundary_ready());
        assert_eq!(
            init_summary(result),
            crate::SYSCALL_DIRECTION_DEFINED_MESSAGE
        );
        assert_eq!(
            boundary_summary(result),
            crate::SYSCALL_BOUNDARY_READY_MESSAGE
        );
    }

    #[test]
    fn explicit_unready_boundary_reports_direction_only() {
        let result = InitResult::new(false);

        assert!(!result.is_boundary_ready());
        assert_eq!(
            init_summary(result),
            crate::SYSCALL_DIRECTION_DEFINED_MESSAGE
        );
        assert_eq!(
            boundary_summary(result),
            crate::SYSCALL_DIRECTION_DEFINED_MESSAGE
        );
    }

    #[test]
    fn kernel_syscall_summaries_match_expected_messages() {
        assert_eq!(
            kernel_number_summary(Number::Unknown(99)),
            crate::SYSCALL_INVALID_NUMBER_MESSAGE
        );
        assert_eq!(
            kernel_result_summary(Result::success(1)),
            crate::SYSCALL_SUCCESS_MESSAGE
        );
        assert_eq!(
            kernel_result_summary(Result::error(Error::InvalidArgument)),
            crate::SYSCALL_INVALID_ARGUMENT_MESSAGE
        );
        assert_eq!(
            kernel_result_summary(Result::error(Error::InvalidHandle)),
            crate::SYSCALL_INVALID_HANDLE_MESSAGE
        );
    }
}
