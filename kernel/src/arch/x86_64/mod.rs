#![cfg(target_arch = "x86_64")]

//! Minimal x86_64 exception support for `rustos`.
//!
//! This module introduces the smallest real breakpoint-handler path needed for
//! the current milestone:
//! - build a small IDT
//! - install a real breakpoint handler
//! - expose explicit handler state to the rest of the kernel

use core::sync::atomic::{AtomicBool, Ordering};

use lazy_static::lazy_static;
use x86_64::instructions::interrupts;
use x86_64::registers::control::Cr3;
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

/// Small summary of the current x86_64 paging probe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PagingProbeState {
    paging_active: bool,
    level_4_table_frame: u64,
}

impl PagingProbeState {
    /// Creates an empty paging probe state.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            paging_active: false,
            level_4_table_frame: 0,
        }
    }

    /// Creates a paging probe state with an observed level-4 table frame.
    #[must_use]
    pub const fn active(level_4_table_frame: u64) -> Self {
        Self {
            paging_active: true,
            level_4_table_frame,
        }
    }

    /// Returns whether paging is active for the current runtime.
    #[must_use]
    pub const fn is_paging_active(self) -> bool {
        self.paging_active
    }

    /// Returns the observed level-4 page-table frame start address.
    #[must_use]
    pub const fn level_4_table_frame(self) -> u64 {
        self.level_4_table_frame
    }
}

impl Default for PagingProbeState {
    fn default() -> Self {
        Self::new()
    }
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
pub fn trigger_breakpoint() {
    interrupts::int3();
}

/// Returns whether a minimal x86_64 paging probe is available.
#[must_use]
pub fn has_paging_probe() -> bool {
    paging_probe_state().is_paging_active()
}

/// Returns a minimal x86_64 paging probe state.
#[must_use]
pub fn paging_probe_state() -> PagingProbeState {
    let (frame, _) = Cr3::read();
    PagingProbeState::active(frame.start_address().as_u64())
}

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    BREAKPOINT_HANDLER_REACHED.store(true, Ordering::SeqCst);
}
