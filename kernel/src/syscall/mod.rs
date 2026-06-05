//! Syscall subsystem entry points for `rustos`.
//!
//! This module keeps the kernel-side syscall boundary small and explicit.
//! Host-testable syscall logic lives in `nucleus`, while this module adds the
//! smallest kernel-facing boundary needed for the current stage.

pub use nucleus::syscall::{
    Error, Number, Request, Result, dispatch, number_summary, result_summary,
};

/// Small kernel-side syscall initialization result.
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
#[must_use]
pub const fn kernel_number_summary(number: Number) -> &'static str {
    match number {
        Number::Write => "rustos: syscall write",
        Number::Exit => "rustos: syscall exit",
        Number::Unknown(_) => crate::SYSCALL_INVALID_NUMBER_MESSAGE,
    }
}

/// Returns a kernel-facing plain-language summary of the syscall result.
#[must_use]
pub const fn kernel_result_summary(result: Result) -> &'static str {
    match result.error_kind() {
        None => crate::SYSCALL_SUCCESS_MESSAGE,
        Some(Error::InvalidNumber) => crate::SYSCALL_INVALID_NUMBER_MESSAGE,
        Some(Error::InvalidArgument) => crate::SYSCALL_INVALID_ARGUMENT_MESSAGE,
        Some(Error::InvalidHandle) => crate::SYSCALL_INVALID_HANDLE_MESSAGE,
    }
}

/// Returns a kernel-facing plain-language summary of the syscall request.
#[must_use]
pub const fn kernel_request_summary(request: Request) -> &'static str {
    kernel_number_summary(request.number())
}

/// Dispatches a syscall request through the current host-testable model and
/// returns a kernel-facing plain-language summary of the result.
#[must_use]
pub const fn dispatch_summary(request: Request) -> &'static str {
    kernel_result_summary(dispatch(request))
}
