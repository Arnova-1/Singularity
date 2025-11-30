#![no_main]
#![no_std]

use log::info;

use uefi::prelude::*;

#[entry]
fn main() -> Status {
    let _ = uefi::boot::set_watchdog_timer(0, 0x10000, None);

    // print
    info!("Prototype 0: Booting up...");

    Status::SUCCESS
}