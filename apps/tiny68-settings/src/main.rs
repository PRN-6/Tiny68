//! Settings management and native GUI application for Tiny68.

#![windows_subsystem = "windows"]

use std::sync::atomic::{AtomicUsize, Ordering};
use tiny68_config::{AppConfig, CharacterConfig, ConfigManager};
use tiny68_core::types::{PerformanceProfile, PersonalityType};
use tiny68_platform::{is_autostart_enabled, set_autostart};

use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::UI::WindowsAndMessaging::*;

const ID_TAB_HOME: usize = 101;
const ID_TAB_CHARS: usize = 102;
const ID_TAB_PERF: usize = 103;
const ID_TAB_DESKTOP: usize = 104;
const ID_TAB_ABOUT: usize = 105;

const ID_BTN_SAVE: usize = 201;
const ID_BTN_RESET: usize = 202;

const ID_BTN_ADD_MOCHI: usize = 301;
const ID_BTN_ADD_KIKO: usize = 302;
const ID_BTN_REMOVE_LAST: usize = 303;

const ID_BTN_PERF_ECO: usize = 401;
const ID_BTN_PERF_BALANCED: usize = 402;
const ID_BTN_PERF_SMOOTH: usize = 403;
const ID_BTN_PERF_GAMING: usize = 404;

const ID_BTN_TOGGLE_AUTOSTART: usize = 501;
const ID_BTN_TOGGLE_FULLSCREEN: usize = 502;
const ID_BTN_TOGGLE_CLICKTHROUGH: usize = 503;

static CURRENT_TAB: AtomicUsize = AtomicUsize::new(0); // 0: Home, 1: Chars, 2: Perf, 3: Desktop, 4: About

struct SettingsState {
    config_mgr: ConfigManager,
    config: AppConfig,
    status_text: String,
    autostart_active: bool,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let config_mgr = ConfigManager::new("Tiny68");

    // Check for CLI reset flag
    if args.contains(&"--reset".to_string()) {
        let default_cfg = AppConfig::default();
        config_mgr.save(&default_cfg)?;
        return Ok(());
    }

    // Launch native GUI
    unsafe {
        let instance = windows::Win32::System::LibraryLoader::GetModuleHandleW(None)?;
        let class_name = w!("Tiny68_Settings_Window");

        let bg_brush = CreateSolidBrush(COLORREF(0x001E1B18)); // Dark modern background

        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(settings_wnd_proc),
            hInstance: instance.into(),
            lpszClassName: class_name,
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hbrBackground: bg_brush,
            ..Default::default()
        };

        let _ = RegisterClassExW(&wc);

        let scr_w = GetSystemMetrics(SM_CXSCREEN);
        let scr_h = GetSystemMetrics(SM_CYSCREEN);
        let win_w = 680;
        let win_h = 520;
        let win_x = (scr_w - win_w) / 2;
        let win_y = (scr_h - win_h) / 2;

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            class_name,
            w!("Tiny68 Settings — Your Desktop. Their World."),
            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX,
            win_x,
            win_y,
            win_w,
            win_h,
            None,
            None,
            instance,
            None,
        )?;

        let config = config_mgr.load();
        let autostart_active = is_autostart_enabled("Tiny68");

        let state = Box::into_raw(Box::new(SettingsState {
            config_mgr,
            config,
            status_text: "Configuration loaded. Make adjustments and click Save.".to_string(),
            autostart_active,
        }));

        SetWindowLongPtrW(hwnd, GWLP_USERDATA, state as isize);

        // Create Navigation Buttons (Left sidebar)
        create_btn(
            hwnd,
            "Overview",
            16,
            50,
            120,
            36,
            ID_TAB_HOME,
            instance.into(),
        );
        create_btn(
            hwnd,
            "Characters",
            16,
            92,
            120,
            36,
            ID_TAB_CHARS,
            instance.into(),
        );
        create_btn(
            hwnd,
            "Performance",
            16,
            134,
            120,
            36,
            ID_TAB_PERF,
            instance.into(),
        );
        create_btn(
            hwnd,
            "Desktop",
            16,
            176,
            120,
            36,
            ID_TAB_DESKTOP,
            instance.into(),
        );
        create_btn(
            hwnd,
            "About",
            16,
            218,
            120,
            36,
            ID_TAB_ABOUT,
            instance.into(),
        );

        // Create Global Action Buttons (Bottom)
        create_btn(
            hwnd,
            "Save Settings",
            390,
            430,
            130,
            36,
            ID_BTN_SAVE,
            instance.into(),
        );
        create_btn(
            hwnd,
            "Reset Defaults",
            530,
            430,
            120,
            36,
            ID_BTN_RESET,
            instance.into(),
        );

        // Create Tab-specific interactive controls
        // Characters Tab buttons
        create_btn(
            hwnd,
            "+ Add Mochi",
            160,
            370,
            110,
            32,
            ID_BTN_ADD_MOCHI,
            instance.into(),
        );
        create_btn(
            hwnd,
            "+ Add Kiko",
            280,
            370,
            110,
            32,
            ID_BTN_ADD_KIKO,
            instance.into(),
        );
        create_btn(
            hwnd,
            "- Remove Last",
            400,
            370,
            110,
            32,
            ID_BTN_REMOVE_LAST,
            instance.into(),
        );

        // Performance Tab buttons
        create_btn(
            hwnd,
            "Eco Mode",
            160,
            120,
            110,
            32,
            ID_BTN_PERF_ECO,
            instance.into(),
        );
        create_btn(
            hwnd,
            "Balanced",
            280,
            120,
            110,
            32,
            ID_BTN_PERF_BALANCED,
            instance.into(),
        );
        create_btn(
            hwnd,
            "Smooth",
            400,
            120,
            110,
            32,
            ID_BTN_PERF_SMOOTH,
            instance.into(),
        );
        create_btn(
            hwnd,
            "Gaming",
            520,
            120,
            110,
            32,
            ID_BTN_PERF_GAMING,
            instance.into(),
        );

        // Desktop Integration buttons
        create_btn(
            hwnd,
            "Toggle Startup",
            160,
            110,
            140,
            32,
            ID_BTN_TOGGLE_AUTOSTART,
            instance.into(),
        );
        create_btn(
            hwnd,
            "Toggle Fullscreen",
            160,
            170,
            140,
            32,
            ID_BTN_TOGGLE_FULLSCREEN,
            instance.into(),
        );
        create_btn(
            hwnd,
            "Toggle Click-thru",
            160,
            230,
            140,
            32,
            ID_BTN_TOGGLE_CLICKTHROUGH,
            instance.into(),
        );

        let _ = ShowWindow(hwnd, SW_SHOW);
        let _ = UpdateWindow(hwnd);

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        let _ = Box::from_raw(state);
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
unsafe fn create_btn(
    parent: HWND,
    label: &str,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    id: usize,
    instance: HINSTANCE,
) {
    let wide_label: Vec<u16> = label.encode_utf16().chain(std::iter::once(0)).collect();
    let _ = CreateWindowExW(
        WINDOW_EX_STYLE::default(),
        w!("BUTTON"),
        PCWSTR(wide_label.as_ptr()),
        WS_CHILD | WS_VISIBLE | WINDOW_STYLE(BS_PUSHBUTTON as u32),
        x,
        y,
        w,
        h,
        parent,
        HMENU(id as *mut _),
        instance,
        None,
    );
}

unsafe extern "system" fn settings_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let raw = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
    if raw == 0 {
        return DefWindowProcW(hwnd, msg, wparam, lparam);
    }
    let state = &mut *(raw as *mut SettingsState);

    match msg {
        WM_COMMAND => {
            let cmd_id = wparam.0 & 0xFFFF;
            match cmd_id {
                ID_TAB_HOME => CURRENT_TAB.store(0, Ordering::Relaxed),
                ID_TAB_CHARS => CURRENT_TAB.store(1, Ordering::Relaxed),
                ID_TAB_PERF => CURRENT_TAB.store(2, Ordering::Relaxed),
                ID_TAB_DESKTOP => CURRENT_TAB.store(3, Ordering::Relaxed),
                ID_TAB_ABOUT => CURRENT_TAB.store(4, Ordering::Relaxed),

                ID_BTN_SAVE => {
                    if let Err(e) = state.config_mgr.save(&state.config) {
                        state.status_text = format!("Error saving: {}", e);
                    } else {
                        state.status_text = "Settings successfully saved!".to_string();
                    }
                }
                ID_BTN_RESET => {
                    state.config = AppConfig::default();
                    state.status_text =
                        "Reset to factory defaults. Click Save to apply.".to_string();
                }

                ID_BTN_ADD_MOCHI => {
                    let next_id = (state.config.characters.len() + 1) as u64;
                    state.config.characters.push(CharacterConfig {
                        instance_id: next_id,
                        pack_id: "mochi".to_string(),
                        display_name: format!("Mochi {}", next_id),
                        personality: PersonalityType::Playful,
                        scale: 1.0,
                        initial_x: None,
                        initial_y: None,
                    });
                    state.status_text = format!("Added Mochi {}. Click Save to apply.", next_id);
                }
                ID_BTN_ADD_KIKO => {
                    let next_id = (state.config.characters.len() + 1) as u64;
                    state.config.characters.push(CharacterConfig {
                        instance_id: next_id,
                        pack_id: "kiko".to_string(),
                        display_name: format!("Kiko {}", next_id),
                        personality: PersonalityType::Curious,
                        scale: 1.0,
                        initial_x: None,
                        initial_y: None,
                    });
                    state.status_text = format!("Added Kiko {}. Click Save to apply.", next_id);
                }
                ID_BTN_REMOVE_LAST => {
                    if state.config.characters.len() > 1 {
                        let removed = state.config.characters.pop();
                        state.status_text = format!(
                            "Removed {}. Click Save to apply.",
                            removed.map(|c| c.display_name).unwrap_or_default()
                        );
                    } else {
                        state.status_text = "Cannot remove last remaining character.".to_string();
                    }
                }

                ID_BTN_PERF_ECO => {
                    state.config.performance_profile = PerformanceProfile::Eco;
                    state.status_text =
                        "Selected Eco Mode (15 FPS, max battery saving).".to_string();
                }
                ID_BTN_PERF_BALANCED => {
                    state.config.performance_profile = PerformanceProfile::Balanced;
                    state.status_text = "Selected Balanced Mode (30 FPS default).".to_string();
                }
                ID_BTN_PERF_SMOOTH => {
                    state.config.performance_profile = PerformanceProfile::Smooth;
                    state.status_text = "Selected Smooth Mode (60 FPS fluid motion).".to_string();
                }
                ID_BTN_PERF_GAMING => {
                    state.config.performance_profile = PerformanceProfile::Gaming;
                    state.status_text = "Selected Gaming Mode (Suspends during games).".to_string();
                }

                ID_BTN_TOGGLE_AUTOSTART => {
                    let new_val = !state.autostart_active;
                    let _ = set_autostart("Tiny68", new_val);
                    state.autostart_active = new_val;
                    state.config.auto_start_with_windows = new_val;
                    state.status_text = format!(
                        "Launch at Windows Startup: {}",
                        if new_val { "ENABLED" } else { "DISABLED" }
                    );
                }
                ID_BTN_TOGGLE_FULLSCREEN => {
                    state.config.pause_on_fullscreen_game = !state.config.pause_on_fullscreen_game;
                    state.status_text = format!(
                        "Pause on Fullscreen: {}",
                        if state.config.pause_on_fullscreen_game {
                            "ENABLED"
                        } else {
                            "DISABLED"
                        }
                    );
                }
                ID_BTN_TOGGLE_CLICKTHROUGH => {
                    state.config.click_through_enabled = !state.config.click_through_enabled;
                    state.status_text = format!(
                        "Click-through background: {}",
                        if state.config.click_through_enabled {
                            "ENABLED"
                        } else {
                            "DISABLED"
                        }
                    );
                }
                _ => {}
            }

            let _ = InvalidateRect(hwnd, None, TRUE);
            LRESULT(0)
        }
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);

            let tab = CURRENT_TAB.load(Ordering::Relaxed);

            // Double buffered painting
            let mut rc = RECT::default();
            let _ = GetClientRect(hwnd, &mut rc);
            let mem_dc = CreateCompatibleDC(hdc);
            let mem_bmp = CreateCompatibleBitmap(hdc, rc.right, rc.bottom);
            let old_bmp = SelectObject(mem_dc, mem_bmp);

            // Background fill
            let bg_brush = CreateSolidBrush(COLORREF(0x001E1B18));
            let _ = FillRect(mem_dc, &rc, bg_brush);
            let _ = DeleteObject(bg_brush);

            // Sidebar separator
            let side_brush = CreateSolidBrush(COLORREF(0x00282420));
            let side_rc = RECT {
                left: 0,
                top: 0,
                right: 150,
                bottom: rc.bottom,
            };
            let _ = FillRect(mem_dc, &side_rc, side_brush);
            let _ = DeleteObject(side_brush);

            // Content card background
            let card_brush = CreateSolidBrush(COLORREF(0x002D2824));
            let card_rc = RECT {
                left: 160,
                top: 16,
                right: rc.right - 16,
                bottom: rc.bottom - 60,
            };
            let _ = FillRect(mem_dc, &card_rc, card_brush);
            let _ = DeleteObject(card_brush);

            let _ = SetBkMode(mem_dc, TRANSPARENT);

            // Title and header
            let _ = SetTextColor(mem_dc, COLORREF(0x0099CC57)); // Mint green
            draw_str(mem_dc, 180, 32, "Tiny68 Desktop Companion");

            let _ = SetTextColor(mem_dc, COLORREF(0x00E0E0E0)); // Light gray
            match tab {
                0 => {
                    draw_str(mem_dc, 180, 60, "Overview & Status");
                    draw_str(mem_dc, 180, 100, "Application Status: Active & Running");
                    draw_str(
                        mem_dc,
                        180,
                        130,
                        &format!("Active Characters: {}", state.config.characters.len()),
                    );
                    draw_str(
                        mem_dc,
                        180,
                        160,
                        &format!(
                            "Current Performance Profile: {:?}",
                            state.config.performance_profile
                        ),
                    );
                    draw_str(
                        mem_dc,
                        180,
                        190,
                        &format!(
                            "Auto-Start with Windows: {}",
                            if state.autostart_active { "Yes" } else { "No" }
                        ),
                    );
                    draw_str(
                        mem_dc,
                        180,
                        220,
                        &format!(
                            "Click-Through Enabled: {}",
                            if state.config.click_through_enabled {
                                "Yes"
                            } else {
                                "No"
                            }
                        ),
                    );
                    draw_str(mem_dc, 180, 270, "Quick Tip:");
                    draw_str(
                        mem_dc,
                        180,
                        295,
                        "* Click and hold mouse button to pick up and drag any character.",
                    );
                    draw_str(
                        mem_dc,
                        180,
                        320,
                        "* Right-click any character on your desktop for quick context menu.",
                    );
                }
                1 => {
                    draw_str(mem_dc, 180, 60, "Character Management");
                    draw_str(mem_dc, 180, 90, "Active Companions on your screen:");
                    let mut y = 120;
                    for (i, c) in state.config.characters.iter().take(6).enumerate() {
                        draw_str(
                            mem_dc,
                            180,
                            y,
                            &format!(
                                "{}. {} (Pack: {}, Personality: {:?}, Scale: {:.2}x)",
                                i + 1,
                                c.display_name,
                                c.pack_id,
                                c.personality,
                                c.scale
                            ),
                        );
                        y += 26;
                    }
                }
                2 => {
                    draw_str(mem_dc, 180, 60, "Performance & Energy Profiles");
                    draw_str(mem_dc, 180, 90, "Select simulation and rendering mode:");
                    draw_str(
                        mem_dc,
                        180,
                        170,
                        &format!("Current Profile: {:?}", state.config.performance_profile),
                    );
                    draw_str(
                        mem_dc,
                        180,
                        200,
                        &format!(
                            "Target FPS: {} frames per second",
                            state.config.performance_profile.target_fps()
                        ),
                    );
                    draw_str(mem_dc, 180, 240, "Profile Descriptions:");
                    draw_str(
                        mem_dc,
                        180,
                        265,
                        "* Eco: 15 FPS, max battery saving on laptops.",
                    );
                    draw_str(
                        mem_dc,
                        180,
                        290,
                        "* Balanced: 30 FPS, smooth everyday motion with <2% CPU.",
                    );
                    draw_str(
                        mem_dc,
                        180,
                        315,
                        "* Smooth: 60 FPS, ultra-fluid motion for plugged-in PCs.",
                    );
                    draw_str(
                        mem_dc,
                        180,
                        340,
                        "* Gaming: Automatically suspends during fullscreen games.",
                    );
                }
                3 => {
                    draw_str(mem_dc, 180, 60, "Desktop Integration");
                    draw_str(
                        mem_dc,
                        180,
                        115,
                        &format!(
                            "Launch at Windows startup: {}",
                            if state.autostart_active { "ON" } else { "OFF" }
                        ),
                    );
                    draw_str(
                        mem_dc,
                        180,
                        175,
                        &format!(
                            "Pause on Fullscreen games: {}",
                            if state.config.pause_on_fullscreen_game {
                                "ON"
                            } else {
                                "OFF"
                            }
                        ),
                    );
                    draw_str(
                        mem_dc,
                        180,
                        235,
                        &format!(
                            "Click-Through transparent regions: {}",
                            if state.config.click_through_enabled {
                                "ON"
                            } else {
                                "OFF"
                            }
                        ),
                    );
                }
                4 => {
                    draw_str(mem_dc, 180, 60, "About Tiny68");
                    draw_str(mem_dc, 180, 100, "Tiny68 Desktop Companion Engine v0.1.0");
                    draw_str(mem_dc, 180, 130, "Tagline: Your Desktop. Their World.");
                    draw_str(mem_dc, 180, 170, "Architecture:");
                    draw_str(
                        mem_dc,
                        180,
                        195,
                        "- Modular Rust 2024 Workspace (14 modular crates)",
                    );
                    draw_str(
                        mem_dc,
                        180,
                        220,
                        "- Win32 Layered Window + Per-Pixel Alpha Blitter",
                    );
                    draw_str(
                        mem_dc,
                        180,
                        245,
                        "- Zero telemetry, 100% offline, privacy by design",
                    );
                    draw_str(mem_dc, 180, 285, "License: MIT OR Apache-2.0");
                }
                _ => {}
            }

            // Status bar at bottom
            let _ = SetTextColor(mem_dc, COLORREF(0x00A0A0A0));
            draw_str(mem_dc, 160, rc.bottom - 40, &state.status_text);

            let _ = BitBlt(hdc, 0, 0, rc.right, rc.bottom, mem_dc, 0, 0, SRCCOPY);
            let _ = SelectObject(mem_dc, old_bmp);
            let _ = DeleteObject(mem_bmp);
            let _ = DeleteDC(mem_dc);

            let _ = EndPaint(hwnd, &ps);
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe fn draw_str(hdc: HDC, x: i32, y: i32, text: &str) {
    let wide: Vec<u16> = text.encode_utf16().collect();
    let _ = TextOutW(hdc, x, y, &wide);
}
