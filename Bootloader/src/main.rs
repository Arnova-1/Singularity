#![no_main]
#![no_std]

use core::time::Duration;
use log::info;
use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    info!("Hello!");

    boot::stall(Duration::from_secs(10));

    Status::SUCCESS
}