#![no_main]
#![no_std]

use core::time::Duration;
use log::info;

use uefi::prelude::*;
use uefi::boot;
use uefi::proto::loaded_image::LoadedImage;

#[entry]
fn main() -> Status {
    // disable watchdog temporarily for debugging
    let _ = boot::set_watchdog_timer(0, 0x10000, None);

    // enable helpers for logging
    uefi::helpers::init().unwrap();

    // print boot up message
    info!("Prototype 0: Booting up...");

    // Get device handle
    let loaded = boot::open_protocol_exclusive::<LoadedImage>(boot::image_handle());
    let device = loaded.unwrap().device();

    // Print loaded device handle (for testing)
    info!("{:?}", device);

    // Wait 10 seconds (for testing)
    boot::stall(Duration::from_secs(10));

    Status::SUCCESS
}