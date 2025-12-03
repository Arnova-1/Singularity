use uefi::boot::{self, OpenProtocolAttributes, OpenProtocolParams, SearchType};
use uefi::Identify;
use uefi::proto::console::gop::{GraphicsOutput, ModeInfo, PixelFormat};

#[derive(Debug)]
pub struct FrameBufferInfo {
    pub fb_ptr: *mut u8,
    pub fb_size: usize,
    pub width: usize,
    pub height: usize,
    pub stride: usize,
    pub pixel_format: PixelFormat
}

pub fn init_gop() -> FrameBufferInfo {
    let gop_handle = *boot::locate_handle_buffer(
        SearchType::ByProtocol(&GraphicsOutput::GUID)).unwrap().first().expect("GraphicsOutput is missing");

    let mut gop_protocol = unsafe { boot::open_protocol::<GraphicsOutput>(
        OpenProtocolParams {
            handle: gop_handle,
            agent: gop_handle,
            controller: None,
        },
        OpenProtocolAttributes::GetProtocol,
    ).expect("failed to open protocol") };

    let mode = gop_protocol.current_mode_info();
    let mut fb = gop_protocol.frame_buffer();

    FrameBufferInfo {
        fb_ptr: fb.as_mut_ptr(),
        fb_size: fb.size(),
        width: mode.resolution().0,
        height: mode.resolution().1,
        stride: mode.stride(),
        pixel_format: mode.pixel_format(),
    }
}