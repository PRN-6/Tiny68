use tiny68_core::geom::Rect;
use tiny68_renderer::buffer::PixelBuffer;

#[test]
fn test_pixel_buffer_clear_and_resize() {
    let mut buf = PixelBuffer::new(100, 100);
    assert_eq!(buf.pixels.len(), 10000);
    assert_eq!(buf.pixels[0], 0);

    buf.pixels[0] = 0xFF_FF_FF_FF;
    assert_eq!(buf.pixels[0], 0xFF_FF_FF_FF);

    buf.clear();
    assert_eq!(buf.pixels[0], 0);

    buf.resize(200, 100);
    assert_eq!(buf.pixels.len(), 20000);
}

#[test]
fn test_premultiplied_alpha_blitting() {
    let mut dest = PixelBuffer::new(10, 10);
    let src_pixels = vec![0xFF_00_00_FF; 4]; // 2x2 fully opaque red (BGRA premultiplied: A=0xFF, R=0xFF, G=0, B=0)
    let src_rect = Rect::new(0.0, 0.0, 2.0, 2.0);

    dest.blit(
        &src_pixels,
        2,
        &src_rect,
        0,
        0,
        false,
        1.0,
        1.0,
    );

    // Pixel at (0, 0) in dest should now have Alpha 0xFF and Red 0xFF
    let px = dest.pixels[0];
    let a = (px >> 24) & 0xFF;
    let r = (px >> 16) & 0xFF;
    assert_eq!(a, 0xFF, "Blitted pixel should be fully opaque");
    assert_eq!(r, 0xFF, "Blitted pixel should have full red channel");
}

