//! Boot entry flow for `rustos`.
//!
//! This module keeps the early UEFI boot path small and explicit.
//! The current implementation introduces a minimal runtime
//! initialization sequence without changing the overall boot behavior.

use uefi::Status;

/// Boot mode for the current kernel run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Normal boot flow.
    Normal,
    /// Controlled exception test flow.
    ExceptionTest,
}

/// Runs the early boot flow.
///
/// The current flow:
/// - initializes console output
/// - prints deterministic boot messages
/// - selects a boot mode from a dedicated startup-script marker file when present
/// - falls back to the normal boot mode when no explicit marker is present
/// - runs a small runtime initialization sequence
/// - optionally runs the completed breakpoint exception test path
/// - returns success to UEFI
pub fn run() -> Status {
    run_with_mode(selected_mode())
}

/// Runs the early boot flow with an explicit boot mode.
pub fn run_with_mode(mode: Mode) -> Status {
    let console_state = match crate::console::init() {
        Ok(state) => state,
        Err(status) => return status,
    };

    crate::console::write_line(crate::BOOT_MESSAGE);
    crate::console::write_line(crate::HELLO_MESSAGE);
    crate::console::write_line(boot_mode_label(mode));

    initialize_runtime(console_state);

    if mode == Mode::ExceptionTest {
        run_exception_test();
    }

    Status::SUCCESS
}

/// Runs the minimal runtime initialization sequence.
///
/// This keeps the initialization order visible in boot logs while the
/// broader interrupt, timer, and memory subsystems remain intentionally small.
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

    if crate::interrupt::has_real_exception_handlers() {
        crate::console::write_line(crate::EXCEPTION_HANDLERS_INSTALLED_MESSAGE);
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

/// Returns the selected boot mode for the current boot.
///
/// This keeps boot-mode selection minimal:
/// - use a dedicated startup-script marker file when it is present
/// - otherwise fall back to the normal boot mode
/// - avoid introducing a larger boot argument parser for one narrow test path
fn selected_mode() -> Mode {
    #[cfg(target_os = "uefi")]
    {
        if exception_test_marker_present() {
            return Mode::ExceptionTest;
        }
    }

    Mode::Normal
}

/// Returns whether the dedicated exception-test marker file is present.
#[cfg(target_os = "uefi")]
fn exception_test_marker_present() -> bool {
    use uefi::boot;
    use uefi::cstr16;
    use uefi::proto::loaded_image::LoadedImage;
    use uefi::proto::media::file::{File, FileAttribute, FileMode};
    use uefi::proto::media::fs::SimpleFileSystem;

    let handle = boot::image_handle();
    let loaded_image = match boot::open_protocol_exclusive::<LoadedImage>(handle) {
        Ok(loaded_image) => loaded_image,
        Err(_) => return false,
    };

    let device = match loaded_image.device() {
        Some(device) => device,
        None => return false,
    };

    let mut file_system = match boot::open_protocol_exclusive::<SimpleFileSystem>(device) {
        Ok(file_system) => file_system,
        Err(_) => return false,
    };

    let mut root = match file_system.open_volume() {
        Ok(root) => root,
        Err(_) => return false,
    };

    root.open(
        cstr16!(r"\rustos-exception-test"),
        FileMode::Read,
        FileAttribute::empty(),
    )
    .is_ok()
}

/// Returns the plain-language label for the selected boot mode.
const fn boot_mode_label(mode: Mode) -> &'static str {
    match mode {
        Mode::Normal => crate::BOOT_MODE_NORMAL,
        Mode::ExceptionTest => crate::BOOT_MODE_EXCEPTION_TEST,
    }
}

/// Runs the controlled exception test flow.
///
/// This function stays intentionally small for the completed breakpoint
/// milestone. It reports the real controlled exception path explicitly so
/// contributors can see when the breakpoint handler test starts and completes.
fn run_exception_test() {
    crate::console::write_line(crate::EXCEPTION_TEST_START_MESSAGE);
    let exception = crate::interrupt::controlled_exception();
    crate::console::write_line(crate::interrupt::controlled_exception_label(exception));
    crate::console::write_line(crate::interrupt::controlled_exception_stage_label(
        exception,
    ));

    if !crate::interrupt::has_real_exception_handlers() {
        crate::console::write_line(crate::EXCEPTION_INIT_PENDING_MESSAGE);
        crate::console::write_line(crate::EXCEPTION_TEST_COMPLETE_MESSAGE);
        return;
    }

    crate::interrupt::trigger_controlled_exception(exception);
    crate::interrupt::report_controlled_exception(exception);
    crate::console::write_line(crate::EXCEPTION_TEST_COMPLETE_MESSAGE);
}
