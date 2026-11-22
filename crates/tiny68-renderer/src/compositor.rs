use crate::atlas::TextureManager;
use crate::buffer::PixelBuffer;
use tiny68_animation::AnimationPlayer;
use tiny68_assets::manifest::CharacterManifest;
use tiny68_core::geom::Rect;

pub struct SpriteCompositor {
    pub dirty_rects: Vec<Rect>,
}

impl SpriteCompositor {
    pub fn new() -> Self {
        Self {
            dirty_rects: Vec::new(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw_character(
        &mut self,
        buffer: &mut PixelBuffer,
        manifest: &CharacterManifest,
        textures: &TextureManager,
        anim_player: &AnimationPlayer,
        x: f32,
        y: f32,
        scale: f32,
        opacity: f32,
    ) {
        if let Some(atlas) = textures.get(&manifest.id) {
            if let Some(frame) = anim_player.current_frame(manifest) {
                let src_rect = Rect::new(
                    frame.x as f32,
                    frame.y as f32,
                    frame.width as f32,
                    frame.height as f32,
                );

                let flip_h = anim_player.facing.is_left();

                buffer.blit(
                    &atlas.premultiplied_bgra,
                    atlas.width,
                    &src_rect,
                    x.round() as i32,
                    y.round() as i32,
                    flip_h,
                    scale,
                    opacity,
                );

                let drawn_rect = Rect::new(
                    x.round(),
                    y.round(),
                    (frame.width as f32 * scale).round(),
                    (frame.height as f32 * scale).round(),
                );
                self.dirty_rects.push(drawn_rect);
            }
        }
    }
}

impl Default for SpriteCompositor {
    fn default() -> Self {
        Self::new()
    }
}
