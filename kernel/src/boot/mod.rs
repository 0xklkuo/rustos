//! Boot entry flow for `rustos`.
//!
//! This module keeps the early UEFI boot path small and explicit.
//! The current implementation introduces a minimal runtime
//! initialization sequence without changing the overall boot behavior.

use uefi::Status;

/// Runs the early boot flow.
///
/// The current flow:
/// - initializes console output
/// - prints deterministic boot messages
/// - runs a small runtime initialization sequence
/// - returns success to UEFI
pub fn run() -> Status {
    let console_state = match crate::console::init() {
        Ok(state) => state,
        Err(status) => return status,
    };

    crate::console::write_line(crate::BOOT_MESSAGE);
    crate::console::write_line(crate::HELLO_MESSAGE);

    initialize_runtime(console_state);

    Status::SUCCESS
}

/// Runs the minimal runtime initialization sequence.
///
/// This keeps the initialization order visible in boot logs while the
/// underlying subsystems are still placeholders.
fn initialize_runtime(console_state: crate::console::State) {
    crate::console::write_line(crate::RUNTIME_INIT_START_MESSAGE);

    crate::console::write_line(crate::console::state_summary(console_state));

    crate::console::write_line(crate::ARCH_INIT_START_MESSAGE);
    crate::console::write_line(crate::arch::name());
    let arch_state = crate::arch::init();
    crate::console::write_line(crate::arch::runtime_summary(arch_state));
    crate::console::write_line(crate::ARCH_INIT_COMPLETE_MESSAGE);

    let interrupt_state = crate::interrupt::init();

    crate::console::write_line(crate::EXCEPTION_INIT_MESSAGE);
    crate::console::write_line(crate::interrupt::exception_summary(
        interrupt_state.exceptions(),
    ));
    if interrupt_state.exceptions().is_breakpoint_ready()
        && interrupt_state.exceptions().is_double_fault_ready()
    {
        crate::console::write_line(crate::EXCEPTION_INIT_COMPLETE_MESSAGE);
    } else {
        crate::console::write_line(crate::EXCEPTION_INIT_PENDING_MESSAGE);
    }

    crate::console::write_line(crate::INTERRUPT_INIT_MESSAGE);
    crate::console::write_line(crate::interrupt::interrupt_summary(
        interrupt_state.interrupts(),
    ));
    if interrupt_state.interrupts().is_timer_ready() {
        crate::console::write_line(crate::INTERRUPT_INIT_COMPLETE_MESSAGE);
    } else {
        crate::console::write_line(crate::INTERRUPT_INIT_PENDING_MESSAGE);
    }

    crate::console::write_line(crate::TIMER_INIT_MESSAGE);
    if interrupt_state.interrupts().is_timer_ready() {
        crate::console::write_line(crate::TIMER_INIT_COMPLETE_MESSAGE);
    } else {
        crate::console::write_line(crate::TIMER_INIT_PENDING_MESSAGE);
    }

    crate::console::write_line(crate::MEMORY_INIT_MESSAGE);
    crate::console::write_line(crate::MEMORY_MAP_INIT_MESSAGE);
    let memory_state = crate::memory::init();
    crate::console::write_line(crate::FRAME_ALLOCATOR_INIT_MESSAGE);
    crate::console::write_line(crate::memory::state_summary(memory_state));
    if memory_state.heap_strategy() == crate::memory::HeapStrategy::Deferred {
        crate::console::write_line(crate::HEAP_INIT_DEFERRED_MESSAGE);
    }

    crate::console::write_line(crate::panic::init());
    crate::console::write_line(crate::IDLE_READY_MESSAGE);
    crate::console::write_line(crate::RUNTIME_INIT_COMPLETE_MESSAGE);
}
