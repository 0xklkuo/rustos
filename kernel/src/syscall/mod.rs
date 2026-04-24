//! Syscall subsystem entry points for `rustos`.
//!
//! This module keeps the kernel-side syscall boundary small and explicit.
//! Host-testable syscall logic lives in `nucleus`, while this module adds the
//! smallest kernel-facing boundary needed for the current stage.
//!
//! This module does not implement a real syscall ABI, trap entry, or user-mode
//! transition. It defines a small kernel-facing syscall boundary and keeps
//! runtime-facing summaries aligned with the kernel's log messages.

pub use nucleus::syscall::{
    Error, Number, Request, Result, dispatch, number_summary, result_summary,
};

/// Small kernel-side syscall initialization result.
///
/// This type reports whether the minimal syscall boundary is available to the
/// rest of the kernel. It does not imply that a real syscall ABI or runtime
/// trap path exists yet.
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
/// The current implementation only marks the minimal syscall boundary as ready.
/// It does not install a real syscall entry path or runtime dispatcher.
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
/// This helper is intended for kernel logs and other plain-language status
/// output. It summarizes only the syscall kind, not any request arguments.
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
/// This helper is intended for kernel logs and other plain-language status
/// output. It summarizes only the result state, not the returned value.
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
///
/// This helper summarizes only the syscall number. It intentionally omits the
/// handle and value fields so the kernel-facing summary stays small and stable.
#[must_use]
pub const fn kernel_request_summary(request: Request) -> &'static str {
    kernel_number_summary(request.number())
}

/// Dispatches a syscall request through the current host-testable model and
/// returns a kernel-facing plain-language summary of the result.
///
/// This helper does not expose the dispatched `Result` directly. It exists for
/// kernel-facing status reporting while real syscall ABI wiring and runtime
/// trap handling remain deferred.
#[must_use]
pub const fn dispatch_summary(request: Request) -> &'static str {
    kernel_result_summary(dispatch(request))
}

#[cfg(test)]
mod tests {
    use super::{
        InitResult, boundary_summary, dispatch_summary, init, init_summary, kernel_number_summary,
        kernel_request_summary, kernel_result_summary,
    };
    use nucleus::syscall::{Error, Number, Request, Result};

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

    #[test]
    fn kernel_request_summary_matches_syscall_number() {
        assert_eq!(
            kernel_request_summary(Request::new(Number::Write, 1, 12)),
            "rustos: syscall write"
        );
        assert_eq!(
            kernel_request_summary(Request::new(Number::Exit, 0, 7)),
            "rustos: syscall exit"
        );
        assert_eq!(
            kernel_request_summary(Request::new(Number::Unknown(99), 0, 0)),
            crate::SYSCALL_INVALID_NUMBER_MESSAGE
        );
    }

    #[test]
    fn dispatch_summary_matches_dispatched_result() {
        assert_eq!(
            dispatch_summary(Request::new(Number::Write, 1, 12)),
            crate::SYSCALL_SUCCESS_MESSAGE
        );
        assert_eq!(
            dispatch_summary(Request::new(Number::Write, 0, 12)),
            crate::SYSCALL_INVALID_HANDLE_MESSAGE
        );
        assert_eq!(
            dispatch_summary(Request::new(Number::Write, 1, 0)),
            crate::SYSCALL_INVALID_ARGUMENT_MESSAGE
        );
        assert_eq!(
            dispatch_summary(Request::new(Number::Unknown(99), 0, 0)),
            crate::SYSCALL_INVALID_NUMBER_MESSAGE
        );
    }
}
