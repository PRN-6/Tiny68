use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameDef {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub duration_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationDef {
    pub looping: bool,
    pub frames: Vec<FrameDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterManifest {
    pub format_version: u32,
    pub id: String,
    pub display_name: String,
    pub author: String,
    pub license: String,
    pub atlas_file: String,
    pub sprite_width: u32,
    pub sprite_height: u32,
    pub anchor_x: f32,
    pub anchor_y: f32,
    pub animations: HashMap<String, AnimationDef>,
}
