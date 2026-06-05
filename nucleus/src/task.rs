//! Minimal task direction that is safe to test on the host.
//!
//! This module defines the smallest useful task concepts for the current U6
//! milestone. It does not implement scheduling or context switching.

/// Small kernel-visible task identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id(usize);

/// Small task state model for the current milestone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// The task exists but is not ready to run yet.
    Created,
    /// The task is ready to run.
    Ready,
    /// The task is currently running.
    Running,
    /// The task has exited.
    Exited,
}

impl Id {
    /// Creates a new task identifier.
    #[must_use]
    pub const fn new(value: usize) -> Self {
        Self(value)
    }

    /// Returns the raw task identifier value.
    #[must_use]
    pub const fn as_usize(self) -> usize {
        self.0
    }

    /// Returns whether the task identifier is valid.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.0 > 0
    }
}

/// Returns a small plain-language summary of the task state.
#[must_use]
pub const fn state_summary(state: State) -> &'static str {
    match state {
        State::Created => "rustos: task created",
        State::Ready => "rustos: task ready",
        State::Running => "rustos: task running",
        State::Exited => "rustos: task exited",
    }
}

#[cfg(test)]
mod tests {
    use super::{Id, State, state_summary};

    #[test]
    fn task_id_reports_raw_value() {
        let id = Id::new(1);

        assert_eq!(id.as_usize(), 1);
        assert!(id.is_valid());
    }

    #[test]
    fn zero_task_id_is_invalid() {
        let id = Id::new(0);

        assert_eq!(id.as_usize(), 0);
        assert!(!id.is_valid());
    }

    #[test]
    fn task_state_summaries_match_expected_values() {
        assert_eq!(state_summary(State::Created), "rustos: task created");
        assert_eq!(state_summary(State::Ready), "rustos: task ready");
        assert_eq!(state_summary(State::Running), "rustos: task running");
        assert_eq!(state_summary(State::Exited), "rustos: task exited");
    }
}
