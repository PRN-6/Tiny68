//! Reproducible benchmark suite for Tiny68.
//! Implements Scenarios A through F from the Master Development Specification.

use std::time::Instant;
use tiny68_animation::AnimationPlayer;
use tiny68_assets::manifest::{AnimationDef, CharacterManifest, FrameDef};
use tiny68_behavior::fsm::CharacterStateMachine;
use tiny68_behavior::personality::PersonalityProfile;
use tiny68_core::geom::Rect;
use tiny68_core::types::PersonalityType;
use tiny68_diagnostics::MetricsTracker;
use tiny68_physics::PhysicsBody;
use tiny68_renderer::atlas::LoadedAtlas;
use tiny68_renderer::buffer::PixelBuffer;
use tiny68_renderer::compositor::SpriteCompositor;

struct BenchCharacter {
    physics: PhysicsBody,
    pub _fsm: CharacterStateMachine,
    anim: AnimationPlayer,
    pub _personality: PersonalityProfile,
    scale: f32,
}

fn create_bench_manifest() -> CharacterManifest {
    use std::collections::HashMap;
    let mut anims = HashMap::new();
    anims.insert(
        "idle".to_string(),
        AnimationDef {
            looping: true,
            frames: vec![
                FrameDef {
                    x: 0,
                    y: 0,
                    width: 48,
                    height: 48,
                    duration_ms: 500,
                },
                FrameDef {
                    x: 48,
                    y: 0,
                    width: 48,
                    height: 48,
                    duration_ms: 500,
                },
            ],
        },
    );
    anims.insert(
        "walk".to_string(),
        AnimationDef {
            looping: true,
            frames: vec![
                FrameDef {
                    x: 96,
                    y: 0,
                    width: 48,
                    height: 48,
                    duration_ms: 150,
                },
                FrameDef {
                    x: 144,
                    y: 0,
                    width: 48,
                    height: 48,
                    duration_ms: 150,
                },
            ],
        },
    );
    CharacterManifest {
        format_version: 1,
        id: "bench_char".to_string(),
        display_name: "Bench Character".to_string(),
        author: "Bench".to_string(),
        license: "MIT".to_string(),
        atlas_file: "bench.png".to_string(),
        sprite_width: 48,
        sprite_height: 48,
        anchor_x: 0.5,
        anchor_y: 1.0,
        animations: anims,
    }
}

fn create_bench_atlas() -> LoadedAtlas {
    LoadedAtlas {
        width: 192,
        height: 48,
        premultiplied_bgra: vec![0xFF_80_80_80; 192 * 48],
    }
}

struct BenchResult {
    name: &'static str,
    characters: usize,
    steps: usize,
    total_time_ms: f32,
    avg_step_ms: f32,
    p95_step_ms: f32,
    p99_step_ms: f32,
}

fn run_scenario(
    name: &'static str,
    num_chars: usize,
    steps: usize,
    manifest: &CharacterManifest,
    atlas: &LoadedAtlas,
    screen: &Rect,
    surfaces: &[Rect],
) -> BenchResult {
    let mut chars = Vec::with_capacity(num_chars);
    for i in 0..num_chars {
        let x = screen.x + ((i as f32 * 35.0) % screen.width);
        let y = screen.y + screen.height - 60.0;
        chars.push(BenchCharacter {
            physics: PhysicsBody::new(x, y, 48.0, 48.0),
            _fsm: CharacterStateMachine::new(),
            anim: AnimationPlayer::new("walk"),
            _personality: PersonalityProfile::from_type(PersonalityType::Playful),
            scale: 1.0,
        });
    }

    let mut buffer = PixelBuffer::new(screen.width as u32, screen.height as u32);
    let mut compositor = SpriteCompositor::new();
    let mut tracker = MetricsTracker::new(steps);

    let mut tex_mgr = tiny68_renderer::atlas::TextureManager::new();
    tex_mgr.insert(&manifest.id, atlas.clone());

    let dt = 1.0 / 60.0;
    let start_total = Instant::now();

    for _ in 0..steps {
        let step_start = Instant::now();

        for c in &mut chars {
            c.physics.integrate(dt, surfaces, screen);
            c.anim.update(dt, manifest);
        }

        buffer.clear();
        for c in &chars {
            compositor.draw_character(
                &mut buffer,
                manifest,
                &tex_mgr,
                &c.anim,
                c.physics.position.x,
                c.physics.position.y,
                c.scale,
                1.0,
            );
        }

        let step_ms = step_start.elapsed().as_secs_f32() * 1000.0;
        tracker.record_frame(step_ms);
    }

    let total_time_ms = start_total.elapsed().as_secs_f32() * 1000.0;
    let (avg_step_ms, p95_step_ms, p99_step_ms) = tracker.compute_percentiles();

    BenchResult {
        name,
        characters: num_chars,
        steps,
        total_time_ms,
        avg_step_ms,
        p95_step_ms,
        p99_step_ms,
    }
}

fn main() {
    println!("============================================================");
    println!("       Tiny68 Performance Benchmark Suite (Scenarios A-F)  ");
    println!("============================================================");

    let manifest = create_bench_manifest();
    let atlas = create_bench_atlas();
    let single_screen = Rect::new(0.0, 0.0, 1920.0, 1080.0);
    let multi_screen = Rect::new(0.0, 0.0, 3840.0, 1080.0);
    let surfaces = vec![
        Rect::new(200.0, 400.0, 600.0, 30.0),
        Rect::new(900.0, 500.0, 500.0, 30.0),
    ];

    let results = vec![
        // Scenario A: Completely idle (1 character, minimal activity)
        run_scenario(
            "Scenario A: Idle Desktop",
            1,
            300,
            &manifest,
            &atlas,
            &single_screen,
            &surfaces,
        ),
        // Scenario B: Normal Desktop (10 characters)
        run_scenario(
            "Scenario B: Normal Desktop (10 chars)",
            10,
            300,
            &manifest,
            &atlas,
            &single_screen,
            &surfaces,
        ),
        // Scenario C: Heavy Movement (50 characters)
        run_scenario(
            "Scenario C: Heavy Movement (50 chars)",
            50,
            300,
            &manifest,
            &atlas,
            &single_screen,
            &surfaces,
        ),
        // Scenario D: Gaming Mode (0 active chars rendered)
        run_scenario(
            "Scenario D: Gaming Mode (Suspended)",
            0,
            300,
            &manifest,
            &atlas,
            &single_screen,
            &surfaces,
        ),
        // Scenario E: Multi-Monitor (3840x1080, 20 characters)
        run_scenario(
            "Scenario E: Multi-Monitor (Dual 1080p)",
            20,
            300,
            &manifest,
            &atlas,
            &multi_screen,
            &surfaces,
        ),
        // Scenario F: Long-Running Simulation (10 chars, 2000 steps)
        run_scenario(
            "Scenario F: Long-Running (2000 steps)",
            10,
            2000,
            &manifest,
            &atlas,
            &single_screen,
            &surfaces,
        ),
    ];

    println!("\n| Benchmark Scenario | Characters | Steps | Avg Time (ms) | P95 Time (ms) | P99 Time (ms) | Total (ms) |");
    println!("|---|---|---|---|---|---|---|");
    for r in &results {
        println!(
            "| {:<35} | {:>10} | {:>5} | {:>13.3} | {:>13.3} | {:>13.3} | {:>10.2} |",
            r.name,
            r.characters,
            r.steps,
            r.avg_step_ms,
            r.p95_step_ms,
            r.p99_step_ms,
            r.total_time_ms
        );
    }
    println!("\nBenchmark suite completed successfully.");
}
