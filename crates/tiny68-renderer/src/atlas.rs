use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AtlasError {
    #[error("Failed to load image: {0}")]
    ImageLoad(#[from] image::ImageError),
    #[error("Atlas exceeds maximum supported dimensions ({0}x{1})")]
    DimensionsTooLarge(u32, u32),
}

#[derive(Debug, Clone)]
pub struct LoadedAtlas {
    pub width: u32,
    pub height: u32,
    pub premultiplied_bgra: Vec<u32>,
}

impl LoadedAtlas {
    pub fn from_image_bytes(bytes: &[u8]) -> Result<Self, AtlasError> {
        let img = image::load_from_memory(bytes)?.to_rgba8();
        let (width, height) = img.dimensions();

        if width > 4096 || height > 4096 {
            return Err(AtlasError::DimensionsTooLarge(width, height));
        }

        let mut pixels = Vec::with_capacity((width * height) as usize);
        for pixel in img.pixels() {
            let r = pixel[0] as u32;
            let g = pixel[1] as u32;
            let b = pixel[2] as u32;
            let a = pixel[3] as u32;

            // Premultiply RGB by Alpha for Win32 AC_SRC_ALPHA
            let pr = (r * a + 127) / 255;
            let pg = (g * a + 127) / 255;
            let pb = (b * a + 127) / 255;

            // Pack as 0xAARRGGBB (in memory LE: B, G, R, A)
            let packed = (a << 24) | (pr << 16) | (pg << 8) | pb;
            pixels.push(packed);
        }

        Ok(Self {
            width,
            height,
            premultiplied_bgra: pixels,
        })
    }

    pub fn from_file(path: &Path) -> Result<Self, AtlasError> {
        let bytes = std::fs::read(path).map_err(image::ImageError::IoError)?;
        Self::from_image_bytes(&bytes)
    }
}

pub struct TextureManager {
    atlases: HashMap<String, LoadedAtlas>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            atlases: HashMap::new(),
        }
    }

    pub fn load_or_get(&mut self, pack_id: &str, path: &Path) -> Result<&LoadedAtlas, AtlasError> {
        if !self.atlases.contains_key(pack_id) {
            let atlas = LoadedAtlas::from_file(path)?;
            self.atlases.insert(pack_id.to_string(), atlas);
        }
        Ok(self.atlases.get(pack_id).unwrap())
    }

    pub fn insert(&mut self, pack_id: impl Into<String>, atlas: LoadedAtlas) {
        self.atlases.insert(pack_id.into(), atlas);
    }

    pub fn get(&self, pack_id: &str) -> Option<&LoadedAtlas> {
        self.atlases.get(pack_id)
    }
}

impl Default for TextureManager {
    fn default() -> Self {
        Self::new()
    }
}
