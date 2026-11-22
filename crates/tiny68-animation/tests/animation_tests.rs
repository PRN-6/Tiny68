use std::collections::HashMap;
use tiny68_animation::AnimationPlayer;
use tiny68_assets::manifest::{AnimationDef, CharacterManifest, FrameDef};
use tiny68_core::types::FacingDirection;

fn create_test_manifest() -> CharacterManifest {
    let mut anims = HashMap::new();
    anims.insert(
        "idle".to_string(),
        AnimationDef {
            looping: true,
            frames: vec![
                FrameDef {
                    x: 0,
                    y: 0,
                    width: 32,
                    height: 32,
                    duration_ms: 100,
                },
                FrameDef {
                    x: 32,
                    y: 0,
                    width: 32,
                    height: 32,
                    duration_ms: 100,
                },
            ],
        },
    );
    anims.insert(
        "jump".to_string(),
        AnimationDef {
            looping: false,
            frames: vec![FrameDef {
                x: 64,
                y: 0,
                width: 32,
                height: 32,
                duration_ms: 50,
            }],
        },
    );

    CharacterManifest {
        format_version: 1,
        id: "test".to_string(),
        display_name: "Test Character".to_string(),
        author: "Test".to_string(),
        license: "MIT".to_string(),
        atlas_file: "test.png".to_string(),
        sprite_width: 32,
        sprite_height: 32,
        anchor_x: 0.5,
        anchor_y: 1.0,
        animations: anims,
    }
}

#[test]
fn test_animation_frame_progression() {
    let manifest = create_test_manifest();
    let mut player = AnimationPlayer::new("idle");

    assert_eq!(player.frame_index, 0);

    // 50ms elapsed: should still be frame 0 (duration is 100ms)
    player.update(0.05, &manifest);
    assert_eq!(player.frame_index, 0);

    // Another 60ms elapsed (total 110ms): should advance to frame 1
    player.update(0.06, &manifest);
    assert_eq!(player.frame_index, 1);

    // Another 100ms: looping animation wraps back to frame 0
    player.update(0.10, &manifest);
    assert_eq!(player.frame_index, 0);
}

#[test]
fn test_non_looping_animation_finishes() {
    let manifest = create_test_manifest();
    let mut player = AnimationPlayer::new("jump");

    assert!(!player.is_finished);
    player.update(0.06, &manifest);
    assert!(
        player.is_finished,
        "Non-looping single frame animation should finish"
    );
}

#[test]
fn test_facing_direction_flip() {
    let mut player = AnimationPlayer::new("idle");
    assert!(!player.facing.is_left());

    player.facing = FacingDirection::Left;
    assert!(player.facing.is_left());
}
