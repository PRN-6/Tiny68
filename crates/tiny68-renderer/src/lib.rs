//! High-performance sprite rendering, 32-bit BGRA premultiplied alpha composition,
//! texture atlas caching, and dirty rectangle tracking.

pub mod atlas;
pub mod buffer;
pub mod compositor;
pub mod hud;
pub mod traits;

pub use atlas::{LoadedAtlas, TextureManager};
pub use buffer::PixelBuffer;
pub use compositor::SpriteCompositor;
pub use traits::{Renderer, SoftwareLayeredRenderer};
