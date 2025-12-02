use uefi::mem::memory_map::MemoryMapOwned;
use uefi::table::Revision;

#[derive(Debug)]
pub struct BootInfo {
    pub uefi_revision: Revision,
    pub memory_map: MemoryMapOwned,
}