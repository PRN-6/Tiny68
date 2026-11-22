//! Animation playback system, frame stepping, transitions, and sprite state.

use tiny68_assets::manifest::{CharacterManifest, FrameDef};
use tiny68_core::types::FacingDirection;

#[derive(Debug, Clone)]
pub struct AnimationPlayer {
    pub current_anim: String,
    pub frame_index: usize,
    pub elapsed_in_frame_ms: f32,
    pub is_finished: bool,
    pub facing: FacingDirection,
    pub playback_speed: f32,
}

impl AnimationPlayer {
    pub fn new(initial_anim: impl Into<String>) -> Self {
        Self {
            current_anim: initial_anim.into(),
            frame_index: 0,
            elapsed_in_frame_ms: 0.0,
            is_finished: false,
            facing: FacingDirection::Right,
            playback_speed: 1.0,
        }
    }

    pub fn play(&mut self, anim_name: &str, restart_if_same: bool) {
        if self.current_anim == anim_name && !restart_if_same {
            return;
        }
        self.current_anim = anim_name.to_string();
        self.frame_index = 0;
        self.elapsed_in_frame_ms = 0.0;
        self.is_finished = false;
    }

    pub fn update(&mut self, delta_seconds: f32, manifest: &CharacterManifest) {
        if let Some(anim) = manifest.animations.get(&self.current_anim) {
            if anim.frames.is_empty() {
                return;
            }

            self.elapsed_in_frame_ms += delta_seconds * 1000.0 * self.playback_speed;
            let current_frame = &anim.frames[self.frame_index];

            if self.elapsed_in_frame_ms >= current_frame.duration_ms as f32 {
                self.elapsed_in_frame_ms -= current_frame.duration_ms as f32;
                if self.frame_index + 1 < anim.frames.len() {
                    self.frame_index += 1;
                } else if anim.looping {
                    self.frame_index = 0;
                } else {
                    self.is_finished = true;
                }
            }
        }
    }

    pub fn current_frame<'a>(&self, manifest: &'a CharacterManifest) -> Option<&'a FrameDef> {
        manifest
            .animations
            .get(&self.current_anim)
            .and_then(|a| a.frames.get(self.frame_index))
    }
}
