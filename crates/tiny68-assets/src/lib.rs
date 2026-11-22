//! Character pack assets, atlas loader, security checks, and decoding.

pub mod manifest;
pub mod security;

pub use manifest::{AnimationDef, CharacterManifest, FrameDef};
pub use security::validate_pack_path;
