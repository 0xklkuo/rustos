#![allow(dead_code)]
#![cfg(target_arch = "x86_64")]

//! Minimal x86_64 exception support for `rustos`.
//!
//! This module introduces the smallest real breakpoint-handler path needed for
//! the current milestone:
//! - build a small IDT
//! - install a real breakpoint handler
//! - expose explicit handler state to the rest of the kernel
//!
//! The implementation stays intentionally narrow. It does not claim full
//! interrupt or exception subsystem completeness.

use core::sync::atomic::{AtomicBool, Ordering};

use lazy_static::lazy_static;
use x86_64::instructions::interrupts;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

/// Tracks whether the IDT has been loaded for the current boot.
static IDT_LOADED: AtomicBool = AtomicBool::new(false);

/// Tracks whether the real breakpoint handler has run.
static BREAKPOINT_HANDLER_REACHED: AtomicBool = AtomicBool::new(false);

/// Small summary of the current x86_64 exception handler state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandlerState {
    idt_loaded: bool,
    breakpoint_handler_installed: bool,
    breakpoint_handler_reached: bool,
}

impl HandlerState {
    /// Creates a new empty handler state.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            idt_loaded: false,
            breakpoint_handler_installed: false,
            breakpoint_handler_reached: false,
        }
    }

    /// Creates a handler state after the IDT has been loaded.
    #[must_use]
    pub const fn installed() -> Self {
        Self {
            idt_loaded: true,
            breakpoint_handler_installed: true,
            breakpoint_handler_reached: false,
        }
    }

    /// Creates a handler state after the breakpoint handler has run.
    #[must_use]
    pub const fn breakpoint_reached() -> Self {
        Self {
            idt_loaded: true,
            breakpoint_handler_installed: true,
            breakpoint_handler_reached: true,
        }
    }

    /// Returns whether the IDT is loaded.
    #[must_use]
    pub const fn is_idt_loaded(self) -> bool {
        self.idt_loaded
    }

    /// Returns whether the breakpoint handler is installed.
    #[must_use]
    pub const fn is_breakpoint_handler_installed(self) -> bool {
        self.breakpoint_handler_installed
    }

    /// Returns whether the breakpoint handler has been reached.
    #[must_use]
    pub const fn is_breakpoint_handler_reached(self) -> bool {
        self.breakpoint_handler_reached
    }
}

impl Default for HandlerState {
    fn default() -> Self {
        Self::new()
    }
}

/// Loads the minimal IDT for the current milestone.
///
/// Safety and invariants:
/// - the IDT is stored in a process-long static and is never moved
/// - only the breakpoint entry is configured at this stage
/// - repeated calls are harmless for the current milestone
pub fn init_idt() -> HandlerState {
    IDT.load();
    IDT_LOADED.store(true, Ordering::SeqCst);
    BREAKPOINT_HANDLER_REACHED.store(false, Ordering::SeqCst);
    HandlerState::installed()
}

/// Returns the current handler state observed by the kernel.
#[must_use]
pub fn handler_state() -> HandlerState {
    let idt_loaded = IDT_LOADED.load(Ordering::SeqCst);
    let breakpoint_handler_reached = BREAKPOINT_HANDLER_REACHED.load(Ordering::SeqCst);

    if idt_loaded && breakpoint_handler_reached {
        HandlerState::breakpoint_reached()
    } else if idt_loaded {
        HandlerState::installed()
    } else {
        HandlerState::new()
    }
}

/// Returns whether the real breakpoint handler path is available.
#[must_use]
pub fn has_real_breakpoint_handler() -> bool {
    handler_state().is_breakpoint_handler_installed()
}

/// Returns whether the real breakpoint handler has already run.
#[must_use]
pub fn breakpoint_handler_reached() -> bool {
    handler_state().is_breakpoint_handler_reached()
}

/// Triggers a real CPU breakpoint exception.
///
/// This should only be called after `init_idt`.
pub fn trigger_breakpoint() {
    interrupts::int3();
}

/// Returns a short plain-language summary of the current handler state.
#[must_use]
pub const fn handler_summary(state: HandlerState) -> &'static str {
    if state.is_breakpoint_handler_reached() {
        crate::BREAKPOINT_HANDLER_REACHED_MESSAGE
    } else if state.is_breakpoint_handler_installed() && state.is_idt_loaded() {
        crate::BREAKPOINT_HANDLER_ACTIVE_MESSAGE
    } else {
        crate::EXCEPTION_INIT_PENDING_MESSAGE
    }
}

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    BREAKPOINT_HANDLER_REACHED.store(true, Ordering::SeqCst);
}

#[cfg(test)]
mod tests {
    use super::{HandlerState, handler_summary};

    #[test]
    fn new_handler_state_starts_uninitialized() {
        let state = HandlerState::new();

        assert!(!state.is_idt_loaded());
        assert!(!state.is_breakpoint_handler_installed());
        assert!(!state.is_breakpoint_handler_reached());
        assert_eq!(
            handler_summary(state),
            crate::EXCEPTION_INIT_PENDING_MESSAGE
        );
    }

    #[test]
    fn installed_handler_state_reports_installed() {
        let state = HandlerState::installed();

        assert!(state.is_idt_loaded());
        assert!(state.is_breakpoint_handler_installed());
        assert!(!state.is_breakpoint_handler_reached());
        assert_eq!(
            handler_summary(state),
            crate::BREAKPOINT_HANDLER_ACTIVE_MESSAGE
        );
    }

    #[test]
    fn reached_handler_state_reports_reached() {
        let state = HandlerState::breakpoint_reached();

        assert!(state.is_idt_loaded());
        assert!(state.is_breakpoint_handler_installed());
        assert!(state.is_breakpoint_handler_reached());
        assert_eq!(
            handler_summary(state),
            crate::BREAKPOINT_HANDLER_REACHED_MESSAGE
        );
    }

    #[test]
    fn default_handler_state_matches_new() {
        assert_eq!(HandlerState::default(), HandlerState::new());
    }
}
