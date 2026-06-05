//! Minimal syscall direction that is safe to test on the host.
//!
//! This module defines the smallest useful syscall concepts for the current
//! Unix-like foundation. It does not implement a real syscall ABI or kernel
//! entry path yet.

/// Small set of syscall numbers for the current milestone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Number {
    /// Write to a descriptor-like handle.
    Write,
    /// Exit the current task.
    Exit,
    /// Unknown or unsupported syscall number.
    Unknown(u64),
}

/// Small syscall error model for the current milestone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The syscall number is not recognized.
    InvalidNumber,
    /// One or more syscall arguments are invalid.
    InvalidArgument,
    /// The descriptor-like handle is invalid.
    InvalidHandle,
}

/// Small syscall result model for the current milestone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Result {
    value: usize,
    error: Option<Error>,
}

/// Small syscall request model for the current milestone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Request {
    number: Number,
    handle: u32,
    value: usize,
}

impl Number {
    /// Decodes a raw syscall number into the current minimal syscall model.
    #[must_use]
    pub const fn decode(raw: u64) -> Self {
        match raw {
            1 => Self::Write,
            2 => Self::Exit,
            other => Self::Unknown(other),
        }
    }

    /// Returns the raw syscall number.
    #[must_use]
    pub const fn raw(self) -> u64 {
        match self {
            Self::Write => 1,
            Self::Exit => 2,
            Self::Unknown(raw) => raw,
        }
    }

    /// Returns whether the syscall number is known.
    #[must_use]
    pub const fn is_known(self) -> bool {
        !matches!(self, Self::Unknown(_))
    }
}

impl Result {
    /// Creates a successful syscall result.
    #[must_use]
    pub const fn success(value: usize) -> Self {
        Self { value, error: None }
    }

    /// Creates a failed syscall result.
    #[must_use]
    pub const fn error(error: Error) -> Self {
        Self {
            value: 0,
            error: Some(error),
        }
    }

    /// Returns the successful value, if present.
    #[must_use]
    pub const fn value(self) -> usize {
        self.value
    }

    /// Returns the syscall error, if present.
    #[must_use]
    pub const fn error_kind(self) -> Option<Error> {
        self.error
    }

    /// Returns whether the syscall result is successful.
    #[must_use]
    pub const fn is_success(self) -> bool {
        self.error.is_none()
    }
}

impl Request {
    /// Creates a new syscall request.
    #[must_use]
    pub const fn new(number: Number, handle: u32, value: usize) -> Self {
        Self {
            number,
            handle,
            value,
        }
    }

    /// Returns the syscall number for this request.
    #[must_use]
    pub const fn number(self) -> Number {
        self.number
    }

    /// Returns the descriptor-like handle for this request.
    #[must_use]
    pub const fn handle(self) -> u32 {
        self.handle
    }

    /// Returns the small value field for this request.
    #[must_use]
    pub const fn value(self) -> usize {
        self.value
    }
}

/// Dispatches a minimal syscall request.
#[must_use]
pub const fn dispatch(request: Request) -> Result {
    match request.number() {
        Number::Write => {
            if request.handle() == 0 {
                Result::error(Error::InvalidHandle)
            } else if request.value() == 0 {
                Result::error(Error::InvalidArgument)
            } else {
                Result::success(request.value())
            }
        }
        Number::Exit => Result::success(request.value()),
        Number::Unknown(_) => Result::error(Error::InvalidNumber),
    }
}

/// Returns a small plain-language summary of the syscall number.
#[must_use]
pub const fn number_summary(number: Number) -> &'static str {
    match number {
        Number::Write => "rustos: syscall write",
        Number::Exit => "rustos: syscall exit",
        Number::Unknown(_) => "rustos: syscall invalid number",
    }
}

/// Returns a small plain-language summary of the syscall result.
#[must_use]
pub const fn result_summary(result: Result) -> &'static str {
    match result.error_kind() {
        None => "rustos: syscall success",
        Some(Error::InvalidNumber) => "rustos: syscall invalid number",
        Some(Error::InvalidArgument) => "rustos: syscall invalid argument",
        Some(Error::InvalidHandle) => "rustos: syscall invalid handle",
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, Number, Request, Result, dispatch, number_summary, result_summary};

    #[test]
    fn decode_known_syscall_numbers() {
        assert_eq!(Number::decode(1), Number::Write);
        assert_eq!(Number::decode(2), Number::Exit);
    }

    #[test]
    fn decode_unknown_syscall_number() {
        assert_eq!(Number::decode(99), Number::Unknown(99));
    }

    #[test]
    fn raw_syscall_numbers_match_expected_values() {
        assert_eq!(Number::Write.raw(), 1);
        assert_eq!(Number::Exit.raw(), 2);
        assert_eq!(Number::Unknown(99).raw(), 99);
    }

    #[test]
    fn known_syscall_numbers_report_known() {
        assert!(Number::Write.is_known());
        assert!(Number::Exit.is_known());
        assert!(!Number::Unknown(99).is_known());
    }

    #[test]
    fn successful_syscall_result_reports_success() {
        let result = Result::success(7);

        assert!(result.is_success());
        assert_eq!(result.value(), 7);
        assert_eq!(result.error_kind(), None);
        assert_eq!(result_summary(result), "rustos: syscall success");
    }

    #[test]
    fn failed_syscall_result_reports_error() {
        let result = Result::error(Error::InvalidArgument);

        assert!(!result.is_success());
        assert_eq!(result.value(), 0);
        assert_eq!(result.error_kind(), Some(Error::InvalidArgument));
        assert_eq!(result_summary(result), "rustos: syscall invalid argument");
    }

    #[test]
    fn syscall_number_summaries_match_expected_values() {
        assert_eq!(number_summary(Number::Write), "rustos: syscall write");
        assert_eq!(number_summary(Number::Exit), "rustos: syscall exit");
        assert_eq!(
            number_summary(Number::Unknown(99)),
            "rustos: syscall invalid number"
        );
    }

    #[test]
    fn request_reports_expected_fields() {
        let request = Request::new(Number::Write, 1, 12);

        assert_eq!(request.number(), Number::Write);
        assert_eq!(request.handle(), 1);
        assert_eq!(request.value(), 12);
    }

    #[test]
    fn dispatch_write_succeeds_for_valid_handle_and_non_zero_length() {
        let request = Request::new(Number::Write, 1, 12);
        let result = dispatch(request);

        assert!(result.is_success());
        assert_eq!(result.value(), 12);
        assert_eq!(result.error_kind(), None);
        assert_eq!(result_summary(result), "rustos: syscall success");
    }

    #[test]
    fn dispatch_write_rejects_invalid_handle() {
        let request = Request::new(Number::Write, 0, 12);
        let result = dispatch(request);

        assert!(!result.is_success());
        assert_eq!(result.value(), 0);
        assert_eq!(result.error_kind(), Some(Error::InvalidHandle));
        assert_eq!(result_summary(result), "rustos: syscall invalid handle");
    }

    #[test]
    fn dispatch_write_rejects_zero_length() {
        let request = Request::new(Number::Write, 1, 0);
        let result = dispatch(request);

        assert!(!result.is_success());
        assert_eq!(result.value(), 0);
        assert_eq!(result.error_kind(), Some(Error::InvalidArgument));
        assert_eq!(result_summary(result), "rustos: syscall invalid argument");
    }

    #[test]
    fn dispatch_exit_returns_exit_code_as_success_value() {
        let request = Request::new(Number::Exit, 0, 7);
        let result = dispatch(request);

        assert!(result.is_success());
        assert_eq!(result.value(), 7);
        assert_eq!(result.error_kind(), None);
        assert_eq!(result_summary(result), "rustos: syscall success");
    }

    #[test]
    fn dispatch_unknown_syscall_reports_invalid_number() {
        let request = Request::new(Number::Unknown(99), 0, 0);
        let result = dispatch(request);

        assert!(!result.is_success());
        assert_eq!(result.value(), 0);
        assert_eq!(result.error_kind(), Some(Error::InvalidNumber));
        assert_eq!(result_summary(result), "rustos: syscall invalid number");
    }
}
