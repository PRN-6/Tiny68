use image::{ImageBuffer, Rgba};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating original character sprite atlases and previews...");

    let mochi_dir = Path::new("assets/characters/mochi");
    let kiko_dir = Path::new("assets/characters/kiko");

    std::fs::create_dir_all(mochi_dir)?;
    std::fs::create_dir_all(kiko_dir)?;

    // Mochi: Pink pastel rounded chibi creature
    generate_atlas(
        mochi_dir.join("mochi_atlas.png"),
        [0xFF, 0xE5, 0xEC],
        [0xFF, 0x8F, 0xA3],
    )?;
    generate_preview(
        mochi_dir.join("preview.png"),
        [0xFF, 0xE5, 0xEC],
        [0xFF, 0x8F, 0xA3],
    )?;

    // Kiko: Mint/teal cute cat-eared chibi creature
    generate_atlas(
        kiko_dir.join("kiko_atlas.png"),
        [0xD0, 0xF4, 0xDE],
        [0x57, 0xCC, 0x99],
    )?;
    generate_preview(
        kiko_dir.join("preview.png"),
        [0xD0, 0xF4, 0xDE],
        [0x57, 0xCC, 0x99],
    )?;

    println!("Character packs generated successfully.");
    Ok(())
}

fn generate_atlas(
    path: impl AsRef<Path>,
    base_color: [u8; 3],
    acc_color: [u8; 3],
) -> Result<(), Box<dyn std::error::Error>> {
    let total_frames = 14;
    let width = total_frames * 48;
    let height = 48;
    let mut img = ImageBuffer::<Rgba<u8>, _>::new(width, height);

    for f in 0..total_frames {
        render_frame(&mut img, f, base_color, acc_color);
    }

    img.save(path)?;
    Ok(())
}

fn generate_preview(
    path: impl AsRef<Path>,
    base_color: [u8; 3],
    acc_color: [u8; 3],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = ImageBuffer::<Rgba<u8>, _>::new(48, 48);
    render_frame(&mut img, 0, base_color, acc_color);
    img.save(path)?;
    Ok(())
}

fn render_frame(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    frame_idx: u32,
    base_color: [u8; 3],
    acc_color: [u8; 3],
) {
    let [base_r, base_g, base_b] = base_color;
    let [acc_r, acc_g, acc_b] = acc_color;
    let offset_x = frame_idx * 48;
    let center_x = (offset_x + 24) as f32;
    let center_y = 26.0f32;
    let radius = 16.0f32;

    let (squash_x, squash_y) = match frame_idx {
        2 | 4 => (1.1, 0.9), // walk steps
        6 => (1.2, 0.8),     // sit / land
        7..=8 => (1.3, 0.7), // sleep
        9 => (0.85, 1.15),   // jump
        10 => (0.9, 1.1),    // fall
        11 => (1.25, 0.75),  // land
        12 => (0.95, 1.1),   // drag
        _ => (1.0, 1.0),
    };

    for y in 0..48 {
        for x in 0..48 {
            let px = (offset_x + x) as f32;
            let py = y as f32;

            let dx = (px - center_x) / squash_x;
            let dy = (py - center_y) / squash_y;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < radius + 1.0 {
                let alpha = ((radius + 1.0 - dist).clamp(0.0, 1.0) * 255.0) as u8;
                if alpha > 0 {
                    let is_eye = (5.0..=8.0).contains(&dx.abs()) && (-2.0..=3.0).contains(&dy);
                    let is_blush = (8.0..=12.0).contains(&dx.abs()) && (2.0..=5.0).contains(&dy);
                    let is_ear = dy < -10.0 && (6.0..=13.0).contains(&dx.abs());

                    let (r, g, b) = if is_eye {
                        (40, 40, 48)
                    } else if is_blush || is_ear {
                        (acc_r, acc_g, acc_b)
                    } else {
                        let shade = 1.0 - (dy / radius * 0.15);
                        (
                            ((base_r as f32) * shade).round() as u8,
                            ((base_g as f32) * shade).round() as u8,
                            ((base_b as f32) * shade).round() as u8,
                        )
                    };

                    img.put_pixel(offset_x + x, y, Rgba([r, g, b, alpha]));
                }
            }
        }
    }
}
