#![no_main]
#![no_std]

mod bootinfo;

use core::time::Duration;
use log::info;

use uefi::prelude::*;
use uefi::boot;
use uefi::boot::MemoryType;
use uefi::system;
use crate::bootinfo::BootInfo;

#[entry]
fn main() -> Status {
    // disable watchdog temporarily for debugging
    let _ = boot::set_watchdog_timer(0, 0x10000, None);

    // enable helpers for logging
    uefi::helpers::init().unwrap();

    // print boot up message
    info!("Prototype 0: Booting up...");

    let boot_info = BootInfo {
        uefi_revision: system::uefi_revision(),
        memory_map: boot::memory_map(MemoryType::LOADER_DATA).unwrap(),
    };

    info!("UEFI Revision: {:?} Memory Map: {:?}", boot_info.uefi_revision, boot_info.memory_map);

    // Wait 10 seconds (for testing)
    boot::stall(Duration::from_secs(10));

    Status::SUCCESS
}