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
    // 0xAARRGGBB in hex: Alpha=0xFF, Red=0xFF, Green=0, Blue=0
    let src_pixels = vec![0xFF_FF_00_00; 4];
    let src_rect = Rect::new(0.0, 0.0, 2.0, 2.0);

    dest.blit(&src_pixels, 2, &src_rect, 0, 0, false, 1.0, 1.0);

    let px = dest.pixels[0];
    let a = (px >> 24) & 0xFF;
    let r = (px >> 16) & 0xFF;
    assert_eq!(a, 0xFF, "Blitted pixel should be fully opaque");
    assert_eq!(r, 0xFF, "Blitted pixel should have full red channel");
}

#[test]
fn test_hud_rendering() {
    let mut dest = PixelBuffer::new(300, 200);
    let lines = ["TINY68 TEST HUD", "FPS: 60.0", "Companions: 2 Active"];
    dest.draw_diagnostics_hud(10, 10, &lines);

    // Verify non-zero pixels exist where HUD was rendered
    let hud_sample_px = dest.pixels[10 * 300 + 10];
    assert_ne!(
        hud_sample_px, 0,
        "HUD background or accent should be painted"
    );
}

#[test]
fn test_renderer_trait_abstraction() {
    use tiny68_renderer::traits::{Renderer, SoftwareLayeredRenderer};

    let mut renderer = SoftwareLayeredRenderer::new();
    assert_eq!(renderer.frame_count, 0);
    assert!(renderer.name().contains("Layered"));

    let buffer = vec![0u32; 100];
    renderer.present_buffer(&buffer, 10, 10);
    assert_eq!(renderer.frame_count, 1);
}
