#![no_std]

//! Host-testable pure logic shared by the `rustos` kernel.
//!
//! Keep firmware-free state models, summaries, and validation helpers here.
//! Runtime-facing integration stays in `kernel/`.

pub mod arch;
pub mod console;
pub mod descriptor;
pub mod interrupt;
pub mod memory;
pub mod paging;
pub mod syscall;
pub mod task;
pub mod vfs;
