//! Minimal descriptor direction that is safe to test on the host.
//!
//! This module defines the smallest useful descriptor-like handle concept for
//! the current U6 milestone.

use crate::task::Id;

/// Small descriptor-like handle type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Handle(u32);

/// Small ownership sketch connecting one task to one descriptor-like handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ownership {
    task_id: Id,
    handle: Handle,
}

impl Handle {
    /// Creates a new descriptor-like handle.
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the raw handle value.
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.0
    }

    /// Returns whether the handle is valid.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.0 > 0
    }
}

impl Ownership {
    /// Creates a new task-to-handle ownership sketch.
    #[must_use]
    pub const fn new(task_id: Id, handle: Handle) -> Self {
        Self { task_id, handle }
    }

    /// Returns the owning task identifier.
    #[must_use]
    pub const fn task_id(self) -> Id {
        self.task_id
    }

    /// Returns the owned descriptor-like handle.
    #[must_use]
    pub const fn handle(self) -> Handle {
        self.handle
    }

    /// Returns whether the ownership sketch is valid.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.task_id.is_valid() && self.handle.is_valid()
    }
}

/// Returns a small plain-language summary of the handle state.
#[must_use]
pub const fn handle_summary(handle: Handle) -> &'static str {
    if handle.is_valid() {
        "rustos: descriptor handle valid"
    } else {
        "rustos: descriptor handle invalid"
    }
}

/// Returns a small plain-language summary of the ownership sketch.
#[must_use]
pub const fn ownership_summary(ownership: Ownership) -> &'static str {
    if ownership.is_valid() {
        "rustos: descriptor ownership valid"
    } else {
        "rustos: descriptor ownership invalid"
    }
}

#[cfg(test)]
mod tests {
    use super::{Handle, Ownership, handle_summary, ownership_summary};
    use crate::task::Id;

    #[test]
    fn non_zero_handle_is_valid() {
        let handle = Handle::new(1);

        assert_eq!(handle.as_u32(), 1);
        assert!(handle.is_valid());
        assert_eq!(handle_summary(handle), "rustos: descriptor handle valid");
    }

    #[test]
    fn zero_handle_is_invalid() {
        let handle = Handle::new(0);

        assert_eq!(handle.as_u32(), 0);
        assert!(!handle.is_valid());
        assert_eq!(handle_summary(handle), "rustos: descriptor handle invalid");
    }

    #[test]
    fn ownership_reports_expected_fields() {
        let ownership = Ownership::new(Id::new(1), Handle::new(2));

        assert_eq!(ownership.task_id(), Id::new(1));
        assert_eq!(ownership.handle(), Handle::new(2));
    }

    #[test]
    fn ownership_is_valid_when_task_and_handle_are_valid() {
        let ownership = Ownership::new(Id::new(1), Handle::new(2));

        assert!(ownership.is_valid());
        assert_eq!(
            ownership_summary(ownership),
            "rustos: descriptor ownership valid"
        );
    }

    #[test]
    fn ownership_is_invalid_when_task_is_invalid() {
        let ownership = Ownership::new(Id::new(0), Handle::new(2));

        assert!(!ownership.is_valid());
        assert_eq!(
            ownership_summary(ownership),
            "rustos: descriptor ownership invalid"
        );
    }

    #[test]
    fn ownership_is_invalid_when_handle_is_invalid() {
        let ownership = Ownership::new(Id::new(1), Handle::new(0));

        assert!(!ownership.is_valid());
        assert_eq!(
            ownership_summary(ownership),
            "rustos: descriptor ownership invalid"
        );
    }
}
