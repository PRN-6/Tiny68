//! Formal Renderer trait abstraction to decouple rendering backends.

use crate::buffer::PixelBuffer;

pub trait Renderer {
    fn name(&self) -> &'static str;
    fn clear(&mut self, buffer: &mut PixelBuffer) {
        buffer.clear();
    }
    fn present_buffer(&mut self, buffer: &[u32], width: u32, height: u32);
}

pub struct SoftwareLayeredRenderer {
    pub frame_count: u64,
}

impl SoftwareLayeredRenderer {
    pub fn new() -> Self {
        Self { frame_count: 0 }
    }
}

impl Default for SoftwareLayeredRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer for SoftwareLayeredRenderer {
    fn name(&self) -> &'static str {
        "Win32 Layered Software Premultiplied Blitter"
    }

    fn present_buffer(&mut self, _buffer: &[u32], _width: u32, _height: u32) {
        self.frame_count += 1;
    }
}
