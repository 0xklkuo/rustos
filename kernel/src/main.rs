#![no_main]
#![no_std]

use uefi::prelude::*;
use uefi::println;

#[entry]
fn efi_main() -> Status {
    if let Err(error) = uefi::helpers::init() {
        return error.status();
    }

    println!("rustos: boot start");
    println!("rustos: hello from UEFI");

    Status::SUCCESS
}
