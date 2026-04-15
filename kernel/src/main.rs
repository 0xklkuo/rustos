#![no_main]
#![no_std]

use kernel::boot;

#[uefi::entry]
fn efi_main() -> uefi::Status {
    boot::run()
}
