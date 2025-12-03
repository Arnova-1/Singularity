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
    let mut framebuffer = gop_protocol.frame_buffer();

    FrameBufferInfo {
        fb_ptr: framebuffer.as_mut_ptr(),
        fb_size: framebuffer.size(),
        width: mode.resolution().0,
        height: mode.resolution().1,
        stride: mode.stride(),
        pixel_format: mode.pixel_format(),
    }
}

pub fn put_pixel(info: &FrameBufferInfo, x: usize, y: usize, color: (u8, u8, u8)) {
    if x >= info.width || y >= info.height {
        return;
    }

    let fb_slice = unsafe { core::slice::from_raw_parts_mut(info.fb_ptr, info.fb_size) };
    let offset = (y * info.stride + x) * 4;

    let pixel = match info.pixel_format {
        PixelFormat::Rgb => {
            ((color.0 as u32) << 16) | ((color.1 as u32) << 8) | (color.2 as u32) // R | G | B
        }
        PixelFormat::Bgr => {
            ((color.2 as u32) << 16) | ((color.1 as u32) << 8) | (color.0 as u32) // B | G | R
        }
        _ => return,
    };

    fb_slice[offset..offset + 4].copy_from_slice(&pixel.to_le_bytes());
}

pub fn clear_screen(info: &FrameBufferInfo, color: (u8, u8, u8)) {
    for y in 0..info.height {
        for x in 0..info.width {
            put_pixel(info, x, y, color);
        }
    }
}