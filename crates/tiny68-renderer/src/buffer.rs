use tiny68_core::geom::Rect;

/// 32-bit BGRA premultiplied alpha pixel buffer.
/// Compatible with Win32 UpdateLayeredWindow with AC_SRC_ALPHA.
#[derive(Debug, Clone)]
pub struct PixelBuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u32>, // 0xAARRGGBB (in memory on LE: B, G, R, A)
}

impl PixelBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width as usize) * (height as usize);
        Self {
            width,
            height,
            pixels: vec![0u32; size],
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.pixels.resize((width as usize) * (height as usize), 0);
            self.clear();
        }
    }

    pub fn clear(&mut self) {
        self.pixels.fill(0);
    }

    pub fn clear_rect(&mut self, rect: &Rect) {
        let x0 = (rect.x.floor() as i32).clamp(0, self.width as i32) as u32;
        let y0 = (rect.y.floor() as i32).clamp(0, self.height as i32) as u32;
        let x1 = ((rect.x + rect.width).ceil() as i32).clamp(0, self.width as i32) as u32;
        let y1 = ((rect.y + rect.height).ceil() as i32).clamp(0, self.height as i32) as u32;

        for y in y0..y1 {
            let row_start = (y as usize) * (self.width as usize) + (x0 as usize);
            let row_end = (y as usize) * (self.width as usize) + (x1 as usize);
            self.pixels[row_start..row_end].fill(0);
        }
    }

    /// Blit source premultiplied BGRA sprite onto target buffer with Porter-Duff 'Over'.
    #[allow(clippy::too_many_arguments)]
    pub fn blit(
        &mut self,
        src_pixels: &[u32],
        src_atlas_width: u32,
        src_rect: &Rect,
        dest_x: i32,
        dest_y: i32,
        flip_h: bool,
        scale: f32,
        opacity: f32,
    ) {
        let sx0 = src_rect.x as u32;
        let sy0 = src_rect.y as u32;
        let sw = src_rect.width as u32;
        let sh = src_rect.height as u32;

        let dw = (sw as f32 * scale).round() as i32;
        let dh = (sh as f32 * scale).round() as i32;

        let alpha_mult = (opacity.clamp(0.0, 1.0) * 255.0).round() as u32;

        for dy in 0..dh {
            let target_y = dest_y + dy;
            if target_y < 0 || target_y >= self.height as i32 {
                continue;
            }

            let sy = sy0 + ((dy as f32 / scale) as u32).min(sh - 1);
            let dest_row = (target_y as usize) * (self.width as usize);

            for dx in 0..dw {
                let target_x = dest_x + dx;
                if target_x < 0 || target_x >= self.width as i32 {
                    continue;
                }

                let sx_rel = ((dx as f32 / scale) as u32).min(sw - 1);
                let sx = if flip_h {
                    sx0 + (sw - 1 - sx_rel)
                } else {
                    sx0 + sx_rel
                };

                let src_idx = (sy as usize) * (src_atlas_width as usize) + (sx as usize);
                let src_px = src_pixels[src_idx];

                let sa_orig = (src_px >> 24) & 0xFF;
                if sa_orig == 0 {
                    continue;
                }

                // Apply opacity
                let (sb, sg, sr, sa) = if alpha_mult < 255 {
                    let sa = (sa_orig * alpha_mult) / 255;
                    let sb = ((src_px & 0xFF) * alpha_mult) / 255;
                    let sg = (((src_px >> 8) & 0xFF) * alpha_mult) / 255;
                    let sr = (((src_px >> 16) & 0xFF) * alpha_mult) / 255;
                    (sb, sg, sr, sa)
                } else {
                    (
                        src_px & 0xFF,
                        (src_px >> 8) & 0xFF,
                        (src_px >> 16) & 0xFF,
                        sa_orig,
                    )
                };

                let dest_idx = dest_row + (target_x as usize);
                let dst_px = self.pixels[dest_idx];
                let da = (dst_px >> 24) & 0xFF;

                if da == 0 {
                    self.pixels[dest_idx] = (sa << 24) | (sr << 16) | (sg << 8) | sb;
                } else {
                    // Porter-Duff Over: out = src + dst * (1 - sa)
                    let inv_sa = 255 - sa;
                    let db = dst_px & 0xFF;
                    let dg = (dst_px >> 8) & 0xFF;
                    let dr = (dst_px >> 16) & 0xFF;

                    let out_b = (sb + ((db * inv_sa + 127) / 255)).min(255);
                    let out_g = (sg + ((dg * inv_sa + 127) / 255)).min(255);
                    let out_r = (sr + ((dr * inv_sa + 127) / 255)).min(255);
                    let out_a = (sa + ((da * inv_sa + 127) / 255)).min(255);

                    self.pixels[dest_idx] = (out_a << 24) | (out_r << 16) | (out_g << 8) | out_b;
                }
            }
        }
    }
}
