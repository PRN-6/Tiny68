//! Main Tiny68 Desktop Companion Engine.

#![windows_subsystem = "windows"]

use std::sync::mpsc::channel;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tiny68_animation::AnimationPlayer;
use tiny68_assets::manifest::CharacterManifest;
use tiny68_behavior::fsm::{BehaviorState, CharacterStateMachine};
use tiny68_behavior::personality::PersonalityProfile;
use tiny68_config::{CharacterConfig, ConfigManager};
use tiny68_core::geom::{Point, Rect, Vec2};
use tiny68_core::types::{FacingDirection, PerformanceProfile, PersonalityType, SimulationState};
use tiny68_diagnostics::{init_logging, MetricsTracker};
use tiny68_overlay::tray::{CharacterMenuAction, TrayEvent, TrayIcon};
use tiny68_overlay::window::{OverlayWindow, WindowEvent};
use tiny68_physics::{PhysicsBody, WallContact};
use tiny68_platform::display::{get_virtual_screen_bounds, get_work_area_bounds};
use tiny68_platform::window_tracker::{enumerate_usable_surfaces, is_foreground_fullscreen};
use tiny68_renderer::atlas::{LoadedAtlas, TextureManager};
use tiny68_renderer::buffer::PixelBuffer;
use tiny68_renderer::compositor::SpriteCompositor;

use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::Threading::*;
use windows::Win32::UI::WindowsAndMessaging::*;

struct RuntimeCharacter {
    pub _id: u64,
    pub display_name: String,
    pack_id: String,
    physics: PhysicsBody,
    fsm: CharacterStateMachine,
    anim: AnimationPlayer,
    personality: PersonalityProfile,
    scale: f32,
    is_following_cursor: bool,
    social_cooldown: f32,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // 1. Single instance mutex check
    unsafe {
        let mutex_name = w!("Local\\Tiny68_SingleInstance_Mutex");
        let _mutex = CreateMutexW(None, TRUE, mutex_name);
        if GetLastError() == ERROR_ALREADY_EXISTS {
            tracing::warn!("Another instance of Tiny68 is already running. Exiting.");
            return Ok(());
        }
    }

    // 2. Initialize diagnostics
    init_logging();
    tracing::info!("Starting Tiny68 - Your Desktop. Their World.");

    // 3. Load or create user configuration
    let config_mgr = ConfigManager::new("Tiny68");
    let mut config = config_mgr.load();

    // 4. Virtual screen geometry
    let screen_bounds = get_virtual_screen_bounds();
    let work_area = get_work_area_bounds();
    tracing::info!(
        "Virtual screen: {:?}, Work area: {:?}",
        screen_bounds,
        work_area
    );

    // 5. Initialize texture manager and embed original character packs
    let mut textures = TextureManager::new();
    let mut manifests: std::collections::HashMap<String, CharacterManifest> =
        std::collections::HashMap::new();

    let mochi_manifest = create_default_mochi_manifest();
    let mochi_atlas = create_default_mochi_atlas();
    textures.insert("mochi", mochi_atlas);
    manifests.insert("mochi".to_string(), mochi_manifest.clone());

    let kiko_manifest = create_default_kiko_manifest();
    let kiko_atlas = create_default_kiko_atlas();
    textures.insert("kiko", kiko_atlas);
    manifests.insert("kiko".to_string(), kiko_manifest.clone());

    // Scan assets/characters directory for installed character packs on disk
    let pack_roots = [
        std::path::PathBuf::from("assets/characters"),
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.join("assets/characters")))
            .unwrap_or_else(|| std::path::PathBuf::from("assets/characters")),
    ];

    for root in &pack_roots {
        if let Ok(entries) = std::fs::read_dir(root) {
            for entry in entries.flatten() {
                let pack_dir = entry.path();
                if pack_dir.is_dir() {
                    let manifest_path = pack_dir.join("manifest.json");
                    if let Ok(content) = std::fs::read_to_string(&manifest_path) {
                        if let Ok(manifest) = serde_json::from_str::<CharacterManifest>(&content) {
                            let atlas_path = pack_dir.join(&manifest.atlas_file);
                            if let Ok(atlas) = LoadedAtlas::from_file(&atlas_path) {
                                tracing::info!(
                                    "Loaded character pack from disk: {} ({})",
                                    manifest.display_name,
                                    manifest.id
                                );
                                textures.insert(&manifest.id, atlas);
                                manifests.insert(manifest.id.clone(), manifest);
                            }
                        }
                    }
                }
            }
        }
    }

    // 6. Runtime characters state
    let mut characters = Vec::new();
    let spawn_y = (work_area.y + work_area.height - 100.0).max(100.0);
    let mut spawn_x = 100.0;

    for char_cfg in &config.characters {
        let x = char_cfg.initial_x.unwrap_or(spawn_x);
        let y = char_cfg.initial_y.unwrap_or(spawn_y);
        spawn_x += 80.0;

        let (sw, sh) = manifests
            .get(&char_cfg.pack_id)
            .map(|m| (m.sprite_width as f32, m.sprite_height as f32))
            .unwrap_or((48.0, 48.0));

        characters.push(RuntimeCharacter {
            _id: char_cfg.instance_id,
            display_name: char_cfg.display_name.clone(),
            pack_id: char_cfg.pack_id.clone(),
            physics: PhysicsBody::new(x, y, sw * char_cfg.scale, sh * char_cfg.scale),
            fsm: CharacterStateMachine::new(),
            anim: AnimationPlayer::new("idle"),
            personality: PersonalityProfile::from_type(char_cfg.personality),
            scale: char_cfg.scale.clamp(0.4, 3.0),
            is_following_cursor: false,
            social_cooldown: 0.0,
        });
    }

    if characters.is_empty() {
        // Spawn default friendly companions: Mochi, Shinobi, and Samurai!
        characters.push(RuntimeCharacter {
            _id: 1,
            display_name: "Mochi".to_string(),
            pack_id: "mochi".to_string(),
            physics: PhysicsBody::new(200.0, spawn_y + 30.0, 48.0, 48.0),
            fsm: CharacterStateMachine::new(),
            anim: AnimationPlayer::new("idle"),
            personality: PersonalityProfile::from_type(PersonalityType::Playful),
            scale: 1.0,
            is_following_cursor: false,
            social_cooldown: 0.0,
        });

        if manifests.contains_key("shinobi") {
            characters.push(RuntimeCharacter {
                _id: 2,
                display_name: "Shinobi".to_string(),
                pack_id: "shinobi".to_string(),
                physics: PhysicsBody::new(320.0, spawn_y, 128.0 * 0.65, 128.0 * 0.65),
                fsm: CharacterStateMachine::new(),
                anim: AnimationPlayer::new("idle"),
                personality: PersonalityProfile::from_type(PersonalityType::Energetic),
                scale: 0.65,
                is_following_cursor: false,
                social_cooldown: 0.0,
            });
        }
        if manifests.contains_key("samurai") {
            characters.push(RuntimeCharacter {
                _id: 3,
                display_name: "Samurai".to_string(),
                pack_id: "samurai".to_string(),
                physics: PhysicsBody::new(440.0, spawn_y, 128.0 * 0.65, 128.0 * 0.65),
                fsm: CharacterStateMachine::new(),
                anim: AnimationPlayer::new("idle"),
                personality: PersonalityProfile::from_type(PersonalityType::Calm),
                scale: 0.65,
                is_following_cursor: false,
                social_cooldown: 0.0,
            });
        }
    } else if characters.len() == 1 && characters[0].pack_id == "mochi" {
        // If config only has Mochi from an earlier run, automatically introduce the new companions!
        if manifests.contains_key("shinobi") {
            characters.push(RuntimeCharacter {
                _id: 2,
                display_name: "Shinobi".to_string(),
                pack_id: "shinobi".to_string(),
                physics: PhysicsBody::new(320.0, spawn_y, 128.0 * 0.65, 128.0 * 0.65),
                fsm: CharacterStateMachine::new(),
                anim: AnimationPlayer::new("idle"),
                personality: PersonalityProfile::from_type(PersonalityType::Energetic),
                scale: 0.65,
                is_following_cursor: false,
                social_cooldown: 0.0,
            });
        }
        if manifests.contains_key("samurai") {
            characters.push(RuntimeCharacter {
                _id: 3,
                display_name: "Samurai".to_string(),
                pack_id: "samurai".to_string(),
                physics: PhysicsBody::new(440.0, spawn_y, 128.0 * 0.65, 128.0 * 0.65),
                fsm: CharacterStateMachine::new(),
                anim: AnimationPlayer::new("idle"),
                personality: PersonalityProfile::from_type(PersonalityType::Calm),
                scale: 0.65,
                is_following_cursor: false,
                social_cooldown: 0.0,
            });
        }
    }

    // Shared state for hit testing
    let character_boxes: Arc<std::sync::RwLock<Vec<Rect>>> = Arc::new(std::sync::RwLock::new(
        characters
            .iter()
            .map(|c| c.physics.bounding_box())
            .collect(),
    ));

    let char_boxes_for_hit = character_boxes.clone();
    let hit_test_cb = Box::new(move |pt: Point| -> Option<usize> {
        let boxes = char_boxes_for_hit.read().ok()?;
        for (idx, b) in boxes.iter().enumerate().rev() {
            if b.contains_point(pt) {
                return Some(idx);
            }
        }
        None
    });

    let (event_tx, event_rx) = channel();

    // 7. Create overlay window & tray icon
    let mut overlay = OverlayWindow::new(
        "Tiny68 Desktop Overlay",
        screen_bounds,
        hit_test_cb,
        event_tx.clone(),
    )?;

    let mut tray = TrayIcon::new(overlay.hwnd, "Tiny68 - Desktop Companion")?;

    let mut pixel_buffer =
        PixelBuffer::new(screen_bounds.width as u32, screen_bounds.height as u32);
    let mut compositor = SpriteCompositor::new();
    let mut metrics_tracker = MetricsTracker::new(120);

    let mut sim_state = SimulationState::Running;
    let mut perf_profile = config.performance_profile;
    let mut show_diagnostics = false;
    let mut config_dirty = false;
    let mut last_config_save = Instant::now();
    let config_path = config_mgr.config_path().to_path_buf();
    let mut last_config_mtime = std::fs::metadata(&config_path)
        .and_then(|m| m.modified())
        .ok();
    let mut config_check_timer = Instant::now();

    let mut dragged_idx: Option<usize> = None;
    let mut drag_offset = Vec2::ZERO;

    let mut last_tick = Instant::now();
    let mut surface_cache_timer = Instant::now();
    let mut cached_surfaces = enumerate_usable_surfaces();

    tracing::info!("Tiny68 initialized and running successfully.");

    // 8. Main event & engine loop
    let mut running = true;
    while running {
        // Windows message pump
        unsafe {
            let mut msg = MSG::default();
            while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                if msg.message == WM_QUIT {
                    running = false;
                    break;
                }
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }

        // Query current mouse cursor coordinates on screen
        let mut win_cursor = POINT::default();
        let _ = unsafe { GetCursorPos(&mut win_cursor) };
        let mouse_pos = Point::new(win_cursor.x as f32, win_cursor.y as f32);

        // Handle internal events from window / tray
        while let Ok(evt) = event_rx.try_recv() {
            match evt {
                WindowEvent::MouseDown {
                    character_index,
                    pt,
                } => {
                    if let Some(idx) = character_index {
                        if idx < characters.len() {
                            dragged_idx = Some(idx);
                            let c = &mut characters[idx];
                            drag_offset =
                                Vec2::new(pt.x - c.physics.position.x, pt.y - c.physics.position.y);
                            c.physics.is_dragging = true;
                            c.is_following_cursor = false;
                            c.fsm.transition_to(BehaviorState::Dragged);
                            c.anim.play("drag", false);
                        }
                    }
                }
                WindowEvent::MouseDrag { pt } => {
                    if let Some(idx) = dragged_idx {
                        if idx < characters.len() {
                            let c = &mut characters[idx];
                            c.physics.position.x = pt.x - drag_offset.x;
                            c.physics.position.y = pt.y - drag_offset.y;
                            config_dirty = true;
                        }
                    }
                }
                WindowEvent::MouseUp { pt: _ } => {
                    if let Some(idx) = dragged_idx.take() {
                        if idx < characters.len() {
                            let c = &mut characters[idx];
                            c.physics.is_dragging = false;
                            c.fsm.transition_to(BehaviorState::Fall);
                            c.anim.play("fall", false);
                            config_dirty = true;
                        }
                    }
                }
                WindowEvent::RightClick {
                    character_index,
                    pt,
                } => {
                    if let Some(idx) = character_index {
                        if idx < characters.len() {
                            let (name, is_following, is_sleeping, ptype, scale) = {
                                let c = &characters[idx];
                                (
                                    c.display_name.clone(),
                                    c.is_following_cursor,
                                    c.fsm.current_state == BehaviorState::Sleep,
                                    c.personality.ptype,
                                    c.scale,
                                )
                            };

                            if let Some(action) = tray.show_character_context_menu(
                                pt.x as i32,
                                pt.y as i32,
                                &name,
                                is_following,
                                is_sleeping,
                                ptype,
                                scale,
                            ) {
                                match action {
                                    CharacterMenuAction::ToggleFollowCursor => {
                                        let c = &mut characters[idx];
                                        c.is_following_cursor = !c.is_following_cursor;
                                    }
                                    CharacterMenuAction::SitHere => {
                                        let c = &mut characters[idx];
                                        c.is_following_cursor = false;
                                        c.fsm.transition_to(BehaviorState::Sit);
                                        c.anim.play("sit", false);
                                        c.physics.velocity.x = 0.0;
                                    }
                                    CharacterMenuAction::ToggleSleep => {
                                        let c = &mut characters[idx];
                                        c.is_following_cursor = false;
                                        if c.fsm.current_state == BehaviorState::Sleep {
                                            c.fsm.transition_to(BehaviorState::Idle);
                                            c.anim.play("idle", false);
                                        } else {
                                            c.fsm.transition_to(BehaviorState::Sleep);
                                            c.anim.play("sleep", false);
                                            c.physics.velocity.x = 0.0;
                                        }
                                    }
                                    CharacterMenuAction::ChangePersonality(p) => {
                                        let c = &mut characters[idx];
                                        c.personality = PersonalityProfile::from_type(p);
                                        config_dirty = true;
                                    }
                                    CharacterMenuAction::ChangeScale(s) => {
                                        let c = &mut characters[idx];
                                        c.scale = s;
                                        let base_dim =
                                            if c.pack_id == "mochi" || c.pack_id == "kiko" {
                                                48.0
                                            } else {
                                                128.0
                                            };
                                        c.physics.width = base_dim * s;
                                        c.physics.height = base_dim * s;
                                        config_dirty = true;
                                    }
                                    CharacterMenuAction::ResetPosition => {
                                        let c = &mut characters[idx];
                                        c.physics.position.y =
                                            work_area.y + work_area.height - 60.0;
                                        c.physics.velocity = Vec2::ZERO;
                                        c.physics.is_grounded = true;
                                        config_dirty = true;
                                    }
                                    CharacterMenuAction::RemoveCharacter => {
                                        if characters.len() > 1 {
                                            characters.remove(idx);
                                            config_dirty = true;
                                        }
                                    }
                                }
                            }
                        }
                    } else if let Some(tray_evt) = tray.show_context_menu(
                        pt.x as i32,
                        pt.y as i32,
                        sim_state == SimulationState::Paused,
                        sim_state == SimulationState::Hidden,
                        show_diagnostics,
                    ) {
                        handle_tray_event(
                            tray_evt,
                            &mut sim_state,
                            &mut perf_profile,
                            &mut characters,
                            &work_area,
                            &mut show_diagnostics,
                            &mut config_dirty,
                            &mut running,
                        );
                    }
                }
                WindowEvent::TrayRightClick { pt } | WindowEvent::TrayLeftClick { pt } => {
                    if let Some(tray_evt) = tray.show_context_menu(
                        pt.x as i32,
                        pt.y as i32,
                        sim_state == SimulationState::Paused,
                        sim_state == SimulationState::Hidden,
                        show_diagnostics,
                    ) {
                        handle_tray_event(
                            tray_evt,
                            &mut sim_state,
                            &mut perf_profile,
                            &mut characters,
                            &work_area,
                            &mut show_diagnostics,
                            &mut config_dirty,
                            &mut running,
                        );
                    }
                }
                WindowEvent::DisplayChanged | WindowEvent::DpiChanged(_) => {
                    let new_bounds = get_virtual_screen_bounds();
                    overlay.resize(new_bounds);
                    pixel_buffer.resize(new_bounds.width as u32, new_bounds.height as u32);
                }
                WindowEvent::ExplorerRestarted => {
                    tray.recreate();
                }
                WindowEvent::CloseRequested => {
                    running = false;
                }
            }
        }

        if !running {
            break;
        }

        // Fullscreen app detection (Gaming mode / Auto-pause)
        let is_fullscreen = config.pause_on_fullscreen_game && is_foreground_fullscreen();
        let effective_state = if is_fullscreen {
            SimulationState::Paused
        } else {
            sim_state
        };

        // Fixed timestep tick
        let now = Instant::now();
        let dt = now.duration_since(last_tick).as_secs_f32().min(0.1);
        last_tick = now;

        // Cache window surfaces periodically (every 1 second)
        if surface_cache_timer.elapsed().as_secs_f32() > 1.0 {
            cached_surfaces = enumerate_usable_surfaces();
            surface_cache_timer = Instant::now();
        }

        if effective_state == SimulationState::Running {
            // 1. Separate characters horizontally and handle proximity
            let char_len = characters.len();
            for i in 0..char_len {
                for j in (i + 1)..char_len {
                    let (left, right) = characters.split_at_mut(j);
                    let c1 = &mut left[i];
                    let c2 = &mut right[0];

                    c1.physics.separate_from(&mut c2.physics, 44.0 * c1.scale);

                    // Social interaction check
                    let dist = (c1.physics.position.x - c2.physics.position.x).abs();
                    if dist < 65.0
                        && c1.physics.is_grounded
                        && c2.physics.is_grounded
                        && c1.social_cooldown <= 0.0
                        && c2.social_cooldown <= 0.0
                    {
                        c1.fsm.transition_to(BehaviorState::Wave);
                        c1.anim.play("sit", false);
                        c1.social_cooldown = 4.0;

                        c2.fsm.transition_to(BehaviorState::Wave);
                        c2.anim.play("sit", false);
                        c2.social_cooldown = 4.0;
                    }
                }
            }

            // 2. Update characters simulation & behavior
            for c in &mut characters {
                c.social_cooldown = (c.social_cooldown - dt).max(0.0);
                c.fsm.state_timer += dt;

                // Handle cursor following
                if c.is_following_cursor {
                    let dx = mouse_pos.x - (c.physics.position.x + c.physics.width * 0.5);
                    if dx.abs() > 30.0 {
                        c.fsm.transition_to(BehaviorState::FollowCursor);
                        c.anim.facing = if dx > 0.0 {
                            FacingDirection::Right
                        } else {
                            FacingDirection::Left
                        };
                        c.physics.velocity.x = if dx > 0.0 {
                            c.personality.walk_speed
                        } else {
                            -c.personality.walk_speed
                        };
                        c.anim.play("walk", false);
                    } else {
                        c.physics.velocity.x = 0.0;
                        c.anim.play("idle", false);
                    }
                } else if c.fsm.current_state != BehaviorState::Dragged {
                    // Check if falling
                    if !c.physics.is_grounded && !c.physics.is_climbing {
                        c.fsm.transition_to(BehaviorState::Fall);
                        c.anim.play("fall", false);
                    } else if c.fsm.current_state == BehaviorState::Fall {
                        c.fsm.transition_to(BehaviorState::Land);
                        c.anim.play("land", false);
                    } else if c.physics.is_grounded {
                        // Check wall contact for window climbing
                        if let Some((contact, _wall_rect)) =
                            c.physics.check_wall_contact(&cached_surfaces)
                        {
                            let rng = (c.fsm.state_timer * 1000.0) as u32;
                            let roll = (rng % 100) as f32 / 100.0;
                            if roll < c.personality.climb_chance {
                                c.physics.is_climbing = true;
                                c.fsm.transition_to(BehaviorState::ClimbWindow);
                                c.anim.play("walk", false);
                            } else {
                                // Turn around
                                c.anim.facing = match contact {
                                    WallContact::LeftEdge => FacingDirection::Left,
                                    WallContact::RightEdge => FacingDirection::Right,
                                    WallContact::None => c.anim.facing,
                                };
                                c.physics.velocity.x = -c.physics.velocity.x;
                            }
                        } else if c.fsm.state_timer >= c.fsm.next_decision_in {
                            make_character_decision(c, mouse_pos);
                        }
                    }
                }

                // Physics step (using work area bottom as floor so characters walk atop taskbar)
                let floor_bounds = Rect::new(
                    screen_bounds.x,
                    screen_bounds.y,
                    screen_bounds.width,
                    (work_area.y + work_area.height).min(screen_bounds.height),
                );
                c.physics.integrate(dt, &cached_surfaces, &floor_bounds);

                // Update animation frames
                let manifest = manifests.get(&c.pack_id).unwrap_or(&mochi_manifest);
                c.anim.update(dt, manifest);
            }
        }

        // Render characters whenever visible (even when simulation paused)
        if effective_state != SimulationState::Hidden {
            // Sync bounding boxes for hit testing
            if let Ok(mut boxes) = character_boxes.write() {
                boxes.clear();
                for c in &characters {
                    boxes.push(c.physics.bounding_box());
                }
            }

            // Render characters to pixel buffer
            pixel_buffer.clear();
            compositor.dirty_rects.clear();

            for c in &characters {
                let manifest = manifests.get(&c.pack_id).unwrap_or(&mochi_manifest);
                compositor.draw_character(
                    &mut pixel_buffer,
                    manifest,
                    &textures,
                    &c.anim,
                    c.physics.position.x,
                    c.physics.position.y,
                    c.scale,
                    1.0,
                );
            }

            // Render Diagnostics HUD if enabled
            if show_diagnostics {
                let (avg_t, p95_t, p99_t) = metrics_tracker.compute_percentiles();
                let active_chars = characters.len();
                let sleeping_chars = characters
                    .iter()
                    .filter(|c| c.fsm.current_state == BehaviorState::Sleep)
                    .count();

                let line1 = "TINY68 PERFORMANCE HUD";
                let line2 = format!(
                    "FPS: {:.1} (Target: {})",
                    metrics_tracker.current_fps,
                    perf_profile.target_fps()
                );
                let line3 = format!(
                    "Frame Time: Avg {:.2}ms | P95 {:.2}ms | P99 {:.2}ms",
                    avg_t, p95_t, p99_t
                );
                let line4 = format!(
                    "Companions: {} Active | {} Sleeping",
                    active_chars, sleeping_chars
                );
                let line5 = format!(
                    "Mode: {:?} | V-Screen: {}x{}",
                    perf_profile, screen_bounds.width as i32, screen_bounds.height as i32
                );

                let hud_lines = [
                    line1,
                    line2.as_str(),
                    line3.as_str(),
                    line4.as_str(),
                    line5.as_str(),
                ];
                pixel_buffer.draw_diagnostics_hud(24, 24, &hud_lines);
            }

            // Present to Windows layered window
            overlay.present(
                &pixel_buffer.pixels,
                pixel_buffer.width,
                pixel_buffer.height,
            );
        }

        // Periodic check for external configuration updates (e.g. from Settings GUI)
        if config_check_timer.elapsed() > Duration::from_millis(500) {
            config_check_timer = Instant::now();
            if let Ok(meta) = std::fs::metadata(&config_path) {
                if let Ok(mtime) = meta.modified() {
                    if Some(mtime) != last_config_mtime {
                        last_config_mtime = Some(mtime);
                        let fresh_cfg = config_mgr.load();
                        perf_profile = fresh_cfg.performance_profile;

                        let existing_ids: std::collections::HashSet<u64> =
                            characters.iter().map(|c| c._id).collect();

                        for c_cfg in &fresh_cfg.characters {
                            if !existing_ids.contains(&c_cfg.instance_id) {
                                let (sw, sh) = manifests
                                    .get(&c_cfg.pack_id)
                                    .map(|m| (m.sprite_width as f32, m.sprite_height as f32))
                                    .unwrap_or((48.0, 48.0));
                                let base_scale = if c_cfg.scale > 0.0 { c_cfg.scale } else { 0.65 };
                                characters.push(RuntimeCharacter {
                                    _id: c_cfg.instance_id,
                                    display_name: c_cfg.display_name.clone(),
                                    pack_id: c_cfg.pack_id.clone(),
                                    physics: PhysicsBody::new(
                                        c_cfg
                                            .initial_x
                                            .unwrap_or(150.0 + (c_cfg.instance_id as f32 * 50.0)),
                                        c_cfg
                                            .initial_y
                                            .unwrap_or(work_area.y + work_area.height - 100.0),
                                        sw * base_scale,
                                        sh * base_scale,
                                    ),
                                    fsm: CharacterStateMachine::new(),
                                    anim: AnimationPlayer::new("idle"),
                                    personality: PersonalityProfile::from_type(c_cfg.personality),
                                    scale: base_scale,
                                    is_following_cursor: false,
                                    social_cooldown: 0.0,
                                });
                            }
                        }

                        let fresh_ids: std::collections::HashSet<u64> =
                            fresh_cfg.characters.iter().map(|c| c.instance_id).collect();
                        if !fresh_ids.is_empty() {
                            characters.retain(|c| fresh_ids.contains(&c._id));
                        }
                    }
                }
            }
        }

        // Measure frame timing for diagnostics
        let frame_ms = now.elapsed().as_secs_f32() * 1000.0;
        metrics_tracker.record_frame(frame_ms);

        // Debounced configuration persistence
        if config_dirty && last_config_save.elapsed() > Duration::from_secs(2) {
            config.characters = characters
                .iter()
                .enumerate()
                .map(|(idx, c)| CharacterConfig {
                    instance_id: (idx + 1) as u64,
                    pack_id: c.pack_id.clone(),
                    display_name: c.display_name.clone(),
                    personality: c.personality.ptype,
                    scale: c.scale,
                    initial_x: Some(c.physics.position.x),
                    initial_y: Some(c.physics.position.y),
                })
                .collect();
            config.performance_profile = perf_profile;
            let _ = config_mgr.save(&config);
            config_dirty = false;
            last_config_save = Instant::now();
        }

        // Frame timing according to performance profile
        let target_fps = perf_profile.target_fps();
        let target_frame_time = if target_fps > 0 {
            Duration::from_secs_f32(1.0 / target_fps as f32)
        } else {
            Duration::from_millis(100)
        };

        let elapsed = now.elapsed();
        if elapsed < target_frame_time {
            std::thread::sleep(target_frame_time - elapsed);
        }
    }

    // Persist config on exit
    config.characters = characters
        .iter()
        .enumerate()
        .map(|(idx, c)| CharacterConfig {
            instance_id: (idx + 1) as u64,
            pack_id: c.pack_id.clone(),
            display_name: c.display_name.clone(),
            personality: c.personality.ptype,
            scale: c.scale,
            initial_x: Some(c.physics.position.x),
            initial_y: Some(c.physics.position.y),
        })
        .collect();
    config.performance_profile = perf_profile;
    let _ = config_mgr.save(&config);

    tracing::info!("Tiny68 exiting cleanly. Goodbye!");
    Ok(())
}

fn make_character_decision(c: &mut RuntimeCharacter, mouse_pos: Point) {
    let rng = (c.fsm.state_timer * 1000.0) as u32;
    let roll = (rng % 100) as f32 / 100.0;

    c.fsm.state_timer = 0.0;
    c.fsm.next_decision_in = match rng % 3 {
        0 => 2.0,
        1 => 3.5,
        _ => 5.0,
    };

    // Cursor hover reaction: if cursor is near, look at it
    let dist_to_mouse = ((c.physics.position.x - mouse_pos.x).powi(2)
        + (c.physics.position.y - mouse_pos.y).powi(2))
    .sqrt();
    if dist_to_mouse < 90.0 && roll < c.personality.curiosity_chance {
        c.fsm.transition_to(BehaviorState::Curious);
        c.anim.facing = if mouse_pos.x > c.physics.position.x {
            FacingDirection::Right
        } else {
            FacingDirection::Left
        };
        c.anim.play("sit", false);
        c.physics.velocity.x = 0.0;
        return;
    }

    if roll < c.personality.sleep_chance {
        c.fsm.transition_to(BehaviorState::Sleep);
        c.anim.play("sleep", false);
        c.physics.velocity.x = 0.0;
    } else if roll < 0.4 {
        c.fsm.transition_to(BehaviorState::Walk);
        c.anim.play("walk", false);
        let go_right = rng.is_multiple_of(2);
        c.anim.facing = if go_right {
            FacingDirection::Right
        } else {
            FacingDirection::Left
        };
        let speed = c.personality.walk_speed;
        c.physics.velocity.x = if go_right { speed } else { -speed };
    } else if roll < 0.55 && c.personality.jump_power > 0.0 {
        c.fsm.transition_to(BehaviorState::Jump);
        c.anim.play("jump", false);
        c.physics.velocity.y = -c.personality.jump_power;
        c.physics.is_grounded = false;
    } else if roll < 0.75 {
        c.fsm.transition_to(BehaviorState::Sit);
        c.anim.play("sit", false);
        c.physics.velocity.x = 0.0;
    } else {
        c.fsm.transition_to(BehaviorState::Idle);
        c.anim.play("idle", false);
        c.physics.velocity.x = 0.0;
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_tray_event(
    evt: TrayEvent,
    sim_state: &mut SimulationState,
    perf_profile: &mut PerformanceProfile,
    characters: &mut Vec<RuntimeCharacter>,
    work_area: &Rect,
    show_diagnostics: &mut bool,
    config_dirty: &mut bool,
    running: &mut bool,
) {
    match evt {
        TrayEvent::OpenSettings => {
            tracing::info!("Launching Tiny68 Settings GUI...");
            let exe_dir = std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|p| p.to_path_buf()))
                .unwrap_or_else(|| std::path::PathBuf::from("."));
            let settings_exe = exe_dir.join("tiny68-settings.exe");
            let _ = std::process::Command::new(settings_exe).spawn();
        }
        TrayEvent::AddMochi => {
            let next_id = (characters.len() + 1) as u64;
            characters.push(RuntimeCharacter {
                _id: next_id,
                display_name: format!("Mochi {}", next_id),
                pack_id: "mochi".to_string(),
                physics: PhysicsBody::new(
                    150.0 + (next_id as f32 * 40.0),
                    work_area.y + work_area.height - 60.0,
                    48.0,
                    48.0,
                ),
                fsm: CharacterStateMachine::new(),
                anim: AnimationPlayer::new("idle"),
                personality: PersonalityProfile::from_type(PersonalityType::Playful),
                scale: 1.0,
                is_following_cursor: false,
                social_cooldown: 0.0,
            });
            *config_dirty = true;
        }
        TrayEvent::AddKiko => {
            let next_id = (characters.len() + 1) as u64;
            characters.push(RuntimeCharacter {
                _id: next_id,
                display_name: format!("Kiko {}", next_id),
                pack_id: "kiko".to_string(),
                physics: PhysicsBody::new(
                    150.0 + (next_id as f32 * 40.0),
                    work_area.y + work_area.height - 60.0,
                    48.0,
                    48.0,
                ),
                fsm: CharacterStateMachine::new(),
                anim: AnimationPlayer::new("idle"),
                personality: PersonalityProfile::from_type(PersonalityType::Curious),
                scale: 1.0,
                is_following_cursor: false,
                social_cooldown: 0.0,
            });
            *config_dirty = true;
        }
        TrayEvent::AddShinobi => {
            let next_id = (characters.len() + 1) as u64;
            characters.push(RuntimeCharacter {
                _id: next_id,
                display_name: format!("Shinobi {}", next_id),
                pack_id: "shinobi".to_string(),
                physics: PhysicsBody::new(
                    150.0 + (next_id as f32 * 40.0),
                    work_area.y + work_area.height - 90.0,
                    128.0 * 0.65,
                    128.0 * 0.65,
                ),
                fsm: CharacterStateMachine::new(),
                anim: AnimationPlayer::new("idle"),
                personality: PersonalityProfile::from_type(PersonalityType::Energetic),
                scale: 0.65,
                is_following_cursor: false,
                social_cooldown: 0.0,
            });
            *config_dirty = true;
        }
        TrayEvent::AddSamurai => {
            let next_id = (characters.len() + 1) as u64;
            characters.push(RuntimeCharacter {
                _id: next_id,
                display_name: format!("Samurai {}", next_id),
                pack_id: "samurai".to_string(),
                physics: PhysicsBody::new(
                    150.0 + (next_id as f32 * 40.0),
                    work_area.y + work_area.height - 90.0,
                    128.0 * 0.65,
                    128.0 * 0.65,
                ),
                fsm: CharacterStateMachine::new(),
                anim: AnimationPlayer::new("idle"),
                personality: PersonalityProfile::from_type(PersonalityType::Calm),
                scale: 0.65,
                is_following_cursor: false,
                social_cooldown: 0.0,
            });
            *config_dirty = true;
        }
        TrayEvent::AddSchoolgirl => {
            let next_id = (characters.len() + 1) as u64;
            characters.push(RuntimeCharacter {
                _id: next_id,
                display_name: format!("Aoi {}", next_id),
                pack_id: "schoolgirl".to_string(),
                physics: PhysicsBody::new(
                    150.0 + (next_id as f32 * 40.0),
                    work_area.y + work_area.height - 90.0,
                    128.0 * 0.65,
                    128.0 * 0.65,
                ),
                fsm: CharacterStateMachine::new(),
                anim: AnimationPlayer::new("idle"),
                personality: PersonalityProfile::from_type(PersonalityType::Playful),
                scale: 0.65,
                is_following_cursor: false,
                social_cooldown: 0.0,
            });
            *config_dirty = true;
        }
        TrayEvent::AddFighter => {
            let next_id = (characters.len() + 1) as u64;
            characters.push(RuntimeCharacter {
                _id: next_id,
                display_name: format!("Fighter {}", next_id),
                pack_id: "fighter".to_string(),
                physics: PhysicsBody::new(
                    150.0 + (next_id as f32 * 40.0),
                    work_area.y + work_area.height - 90.0,
                    128.0 * 0.65,
                    128.0 * 0.65,
                ),
                fsm: CharacterStateMachine::new(),
                anim: AnimationPlayer::new("idle"),
                personality: PersonalityProfile::from_type(PersonalityType::Energetic),
                scale: 0.65,
                is_following_cursor: false,
                social_cooldown: 0.0,
            });
            *config_dirty = true;
        }
        TrayEvent::TogglePause => {
            *sim_state = if *sim_state == SimulationState::Paused {
                SimulationState::Running
            } else {
                SimulationState::Paused
            };
        }
        TrayEvent::ToggleHide => {
            *sim_state = if *sim_state == SimulationState::Hidden {
                SimulationState::Running
            } else {
                SimulationState::Hidden
            };
        }
        TrayEvent::SetModeEco => {
            *perf_profile = PerformanceProfile::Eco;
            *config_dirty = true;
        }
        TrayEvent::SetModeBalanced => {
            *perf_profile = PerformanceProfile::Balanced;
            *config_dirty = true;
        }
        TrayEvent::SetModeSmooth => {
            *perf_profile = PerformanceProfile::Smooth;
            *config_dirty = true;
        }
        TrayEvent::SetModeGaming => {
            *perf_profile = PerformanceProfile::Gaming;
            *config_dirty = true;
        }
        TrayEvent::ToggleDiagnostics => {
            *show_diagnostics = !*show_diagnostics;
        }
        TrayEvent::ResetPositions => {
            let mut x = 100.0;
            let y = work_area.y + work_area.height - 60.0;
            for c in characters.iter_mut() {
                c.physics.position = Point::new(x, y);
                c.physics.velocity = Vec2::ZERO;
                c.physics.is_grounded = true;
                c.physics.is_climbing = false;
                x += 80.0;
            }
            *config_dirty = true;
        }
        TrayEvent::Exit => {
            *running = false;
        }
    }
}

// Helper generators for builtin characters Mochi and Kiko
fn create_default_mochi_manifest() -> CharacterManifest {
    use std::collections::HashMap;
    use tiny68_assets::manifest::{AnimationDef, FrameDef};

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
                    duration_ms: 180,
                },
                FrameDef {
                    x: 144,
                    y: 0,
                    width: 48,
                    height: 48,
                    duration_ms: 180,
                },
                FrameDef {
                    x: 192,
                    y: 0,
                    width: 48,
                    height: 48,
                    duration_ms: 180,
                },
                FrameDef {
                    x: 240,
                    y: 0,
                    width: 48,
                    height: 48,
                    duration_ms: 180,
                },
            ],
        },
    );
    anims.insert(
        "sit".to_string(),
        AnimationDef {
            looping: true,
            frames: vec![FrameDef {
                x: 288,
                y: 0,
                width: 48,
                height: 48,
                duration_ms: 1000,
            }],
        },
    );
    anims.insert(
        "sleep".to_string(),
        AnimationDef {
            looping: true,
            frames: vec![
                FrameDef {
                    x: 336,
                    y: 0,
                    width: 48,
                    height: 48,
                    duration_ms: 800,
                },
                FrameDef {
                    x: 384,
                    y: 0,
                    width: 48,
                    height: 48,
                    duration_ms: 800,
                },
            ],
        },
    );
    anims.insert(
        "jump".to_string(),
        AnimationDef {
            looping: false,
            frames: vec![FrameDef {
                x: 432,
                y: 0,
                width: 48,
                height: 48,
                duration_ms: 300,
            }],
        },
    );
    anims.insert(
        "fall".to_string(),
        AnimationDef {
            looping: true,
            frames: vec![FrameDef {
                x: 480,
                y: 0,
                width: 48,
                height: 48,
                duration_ms: 200,
            }],
        },
    );
    anims.insert(
        "land".to_string(),
        AnimationDef {
            looping: false,
            frames: vec![FrameDef {
                x: 528,
                y: 0,
                width: 48,
                height: 48,
                duration_ms: 200,
            }],
        },
    );
    anims.insert(
        "drag".to_string(),
        AnimationDef {
            looping: true,
            frames: vec![FrameDef {
                x: 576,
                y: 0,
                width: 48,
                height: 48,
                duration_ms: 200,
            }],
        },
    );

    CharacterManifest {
        format_version: 1,
        id: "mochi".to_string(),
        display_name: "Mochi".to_string(),
        author: "Tiny68 Team".to_string(),
        license: "MIT / CC-BY-4.0".to_string(),
        atlas_file: "mochi_atlas.png".to_string(),
        sprite_width: 48,
        sprite_height: 48,
        anchor_x: 0.5,
        anchor_y: 1.0,
        animations: anims,
    }
}

fn create_default_mochi_atlas() -> LoadedAtlas {
    let total_frames = 14;
    let width = total_frames * 48;
    let height = 48;
    let mut pixels = vec![0u32; (width * height) as usize];

    for f in 0..total_frames {
        render_chibi_sprite_frame(&mut pixels, width, f, 0xFF_FFE5EC, 0xFF_FF8FA3);
    }

    LoadedAtlas {
        width,
        height,
        premultiplied_bgra: pixels,
    }
}

fn create_default_kiko_manifest() -> CharacterManifest {
    use std::collections::HashMap;
    use tiny68_assets::manifest::{AnimationDef, FrameDef};

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
                    duration_ms: 400,
                },
                FrameDef {
                    x: 48,
                    y: 0,
                    width: 48,
                    height: 48,
                    duration_ms: 400,
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
                FrameDef {
                    x: 192,
                    y: 0,
                    width: 48,
                    height: 48,
                    duration_ms: 150,
                },
                FrameDef {
                    x: 240,
                    y: 0,
                    width: 48,
                    height: 48,
                    duration_ms: 150,
                },
            ],
        },
    );
    anims.insert(
        "sit".to_string(),
        AnimationDef {
            looping: true,
            frames: vec![FrameDef {
                x: 288,
                y: 0,
                width: 48,
                height: 48,
                duration_ms: 1000,
            }],
        },
    );
    anims.insert(
        "sleep".to_string(),
        AnimationDef {
            looping: true,
            frames: vec![
                FrameDef {
                    x: 336,
                    y: 0,
                    width: 48,
                    height: 48,
                    duration_ms: 700,
                },
                FrameDef {
                    x: 384,
                    y: 0,
                    width: 48,
                    height: 48,
                    duration_ms: 700,
                },
            ],
        },
    );
    anims.insert(
        "jump".to_string(),
        AnimationDef {
            looping: false,
            frames: vec![FrameDef {
                x: 432,
                y: 0,
                width: 48,
                height: 48,
                duration_ms: 300,
            }],
        },
    );
    anims.insert(
        "fall".to_string(),
        AnimationDef {
            looping: true,
            frames: vec![FrameDef {
                x: 480,
                y: 0,
                width: 48,
                height: 48,
                duration_ms: 200,
            }],
        },
    );
    anims.insert(
        "land".to_string(),
        AnimationDef {
            looping: false,
            frames: vec![FrameDef {
                x: 528,
                y: 0,
                width: 48,
                height: 48,
                duration_ms: 200,
            }],
        },
    );
    anims.insert(
        "drag".to_string(),
        AnimationDef {
            looping: true,
            frames: vec![FrameDef {
                x: 576,
                y: 0,
                width: 48,
                height: 48,
                duration_ms: 200,
            }],
        },
    );

    CharacterManifest {
        format_version: 1,
        id: "kiko".to_string(),
        display_name: "Kiko".to_string(),
        author: "Tiny68 Team".to_string(),
        license: "MIT / CC-BY-4.0".to_string(),
        atlas_file: "kiko_atlas.png".to_string(),
        sprite_width: 48,
        sprite_height: 48,
        anchor_x: 0.5,
        anchor_y: 1.0,
        animations: anims,
    }
}

fn create_default_kiko_atlas() -> LoadedAtlas {
    let total_frames = 14;
    let width = total_frames * 48;
    let height = 48;
    let mut pixels = vec![0u32; (width * height) as usize];

    for f in 0..total_frames {
        render_chibi_sprite_frame(&mut pixels, width, f, 0xFF_D0F4DE, 0xFF_57CC99);
    }

    LoadedAtlas {
        width,
        height,
        premultiplied_bgra: pixels,
    }
}

/// Programmatic sprite generator for high-definition, anti-aliased, premultiplied chibi sprites
fn render_chibi_sprite_frame(
    pixels: &mut [u32],
    atlas_width: u32,
    frame_idx: u32,
    base_color_rgb: u32,
    accent_color_rgb: u32,
) {
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

    let base_r = ((base_color_rgb >> 16) & 0xFF) as f32;
    let base_g = ((base_color_rgb >> 8) & 0xFF) as f32;
    let base_b = (base_color_rgb & 0xFF) as f32;

    let acc_r = ((accent_color_rgb >> 16) & 0xFF) as f32;
    let acc_g = ((accent_color_rgb >> 8) & 0xFF) as f32;
    let acc_b = (accent_color_rgb & 0xFF) as f32;

    for y in 0..48 {
        for x in 0..48 {
            let px = (offset_x + x) as f32;
            let py = y as f32;

            let dx = (px - center_x) / squash_x;
            let dy = (py - center_y) / squash_y;
            let dist = (dx * dx + dy * dy).sqrt();

            // Anti-aliased body edge
            if dist < radius + 1.0 {
                let alpha = ((radius + 1.0 - dist).clamp(0.0, 1.0) * 255.0) as u32;
                if alpha > 0 {
                    let is_eye = (5.0..=8.0).contains(&dx.abs()) && (-2.0..=3.0).contains(&dy);
                    let is_blush = (8.0..=12.0).contains(&dx.abs()) && (2.0..=5.0).contains(&dy);
                    let is_ear = dy < -10.0 && (6.0..=13.0).contains(&dx.abs());

                    let (r, g, b) = if is_eye {
                        (40.0, 40.0, 48.0)
                    } else if is_blush || is_ear {
                        (acc_r, acc_g, acc_b)
                    } else {
                        let shade = 1.0 - (dy / radius * 0.15);
                        (base_r * shade, base_g * shade, base_b * shade)
                    };

                    let pr = ((r * alpha as f32 / 255.0).round() as u32).min(255);
                    let pg = ((g * alpha as f32 / 255.0).round() as u32).min(255);
                    let pb = ((b * alpha as f32 / 255.0).round() as u32).min(255);

                    let packed = (alpha << 24) | (pr << 16) | (pg << 8) | pb;
                    let idx = (y as usize) * (atlas_width as usize) + (offset_x + x) as usize;
                    pixels[idx] = packed;
                }
            }
        }
    }
}
