use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::UI::Shell::*;
use windows::Win32::UI::WindowsAndMessaging::*;

pub const WM_TRAY_CALLBACK: u32 = WM_APP + 68;

pub const ID_TRAY_SETTINGS: usize = 1001;
pub const ID_TRAY_ADD_MOCHI: usize = 1002;
pub const ID_TRAY_ADD_KIKO: usize = 1003;
pub const ID_TRAY_ADD_SHINOBI: usize = 1020;
pub const ID_TRAY_ADD_SAMURAI: usize = 1021;
pub const ID_TRAY_ADD_SCHOOLGIRL: usize = 1022;
pub const ID_TRAY_ADD_FIGHTER: usize = 1023;
pub const ID_TRAY_PAUSE_RESUME: usize = 1004;
pub const ID_TRAY_HIDE_SHOW: usize = 1005;
pub const ID_TRAY_MODE_ECO: usize = 1006;
pub const ID_TRAY_MODE_BALANCED: usize = 1007;
pub const ID_TRAY_MODE_SMOOTH: usize = 1008;
pub const ID_TRAY_MODE_GAMING: usize = 1009;
pub const ID_TRAY_RESET_POS: usize = 1010;
pub const ID_TRAY_TOGGLE_DIAG: usize = 1011;
pub const ID_TRAY_EXIT: usize = 1012;

pub const ID_CHAR_TITLE: usize = 2001;
pub const ID_CHAR_FOLLOW: usize = 2002;
pub const ID_CHAR_SIT: usize = 2003;
pub const ID_CHAR_SLEEP_TOGGLE: usize = 2004;
pub const ID_CHAR_RESET: usize = 2005;
pub const ID_CHAR_REMOVE: usize = 2006;

pub const ID_CHAR_PER_PLAYFUL: usize = 2011;
pub const ID_CHAR_PER_LAZY: usize = 2012;
pub const ID_CHAR_PER_CURIOUS: usize = 2013;
pub const ID_CHAR_PER_ENERGETIC: usize = 2014;
pub const ID_CHAR_PER_SHY: usize = 2015;
pub const ID_CHAR_PER_CALM: usize = 2016;

pub const ID_CHAR_SCALE_075: usize = 2021;
pub const ID_CHAR_SCALE_100: usize = 2022;
pub const ID_CHAR_SCALE_125: usize = 2023;
pub const ID_CHAR_SCALE_150: usize = 2024;
pub const ID_CHAR_SCALE_200: usize = 2025;

use tiny68_core::types::PersonalityType;

#[derive(Debug, Clone, PartialEq)]
pub enum CharacterMenuAction {
    ToggleFollowCursor,
    SitHere,
    ToggleSleep,
    ChangePersonality(PersonalityType),
    ChangeScale(f32),
    ResetPosition,
    RemoveCharacter,
}

#[derive(Debug, Clone)]
pub enum TrayEvent {
    OpenSettings,
    AddMochi,
    AddKiko,
    AddShinobi,
    AddSamurai,
    AddSchoolgirl,
    AddFighter,
    TogglePause,
    ToggleHide,
    SetModeEco,
    SetModeBalanced,
    SetModeSmooth,
    SetModeGaming,
    ResetPositions,
    ToggleDiagnostics,
    Exit,
}

pub struct TrayIcon {
    nid: NOTIFYICONDATAW,
    hwnd: HWND,
}

impl TrayIcon {
    pub fn new(hwnd: HWND, tooltip: &str) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        unsafe {
            let mut nid = NOTIFYICONDATAW {
                cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
                hWnd: hwnd,
                uID: 1,
                uFlags: NIF_MESSAGE | NIF_ICON | NIF_TIP,
                uCallbackMessage: WM_TRAY_CALLBACK,
                hIcon: LoadIconW(None, IDI_APPLICATION)?,
                ..Default::default()
            };

            let chars: Vec<u16> = tooltip.encode_utf16().chain(std::iter::once(0)).collect();
            let len = chars.len().min(nid.szTip.len());
            nid.szTip[..len].copy_from_slice(&chars[..len]);

            let _ = Shell_NotifyIconW(NIM_ADD, &nid);

            Ok(Self { nid, hwnd })
        }
    }

    pub fn recreate(&mut self) {
        unsafe {
            let _ = Shell_NotifyIconW(NIM_ADD, &self.nid);
        }
    }

    pub fn show_context_menu(
        &self,
        pt_x: i32,
        pt_y: i32,
        is_paused: bool,
        is_hidden: bool,
        show_diagnostics: bool,
    ) -> Option<TrayEvent> {
        unsafe {
            let hmenu = CreatePopupMenu().ok()?;

            let _ = AppendMenuW(hmenu, MF_STRING, ID_TRAY_SETTINGS, w!("Settings..."));
            let _ = AppendMenuW(hmenu, MF_SEPARATOR, 0, PCWSTR::null());

            let add_submenu = CreatePopupMenu().ok()?;
            let _ = AppendMenuW(
                add_submenu,
                MF_STRING,
                ID_TRAY_ADD_MOCHI,
                w!("Add Mochi (Chibi)"),
            );
            let _ = AppendMenuW(
                add_submenu,
                MF_STRING,
                ID_TRAY_ADD_KIKO,
                w!("Add Kiko (Chibi)"),
            );
            let _ = AppendMenuW(
                add_submenu,
                MF_STRING,
                ID_TRAY_ADD_SHINOBI,
                w!("Add Shinobi (Ninja)"),
            );
            let _ = AppendMenuW(
                add_submenu,
                MF_STRING,
                ID_TRAY_ADD_SAMURAI,
                w!("Add Samurai"),
            );
            let _ = AppendMenuW(
                add_submenu,
                MF_STRING,
                ID_TRAY_ADD_SCHOOLGIRL,
                w!("Add Aoi (Schoolgirl)"),
            );
            let _ = AppendMenuW(
                add_submenu,
                MF_STRING,
                ID_TRAY_ADD_FIGHTER,
                w!("Add Fighter"),
            );
            let _ = AppendMenuW(hmenu, MF_POPUP, add_submenu.0 as usize, w!("Add Character"));

            let pause_text = if is_paused {
                w!("Resume Characters")
            } else {
                w!("Pause Characters")
            };
            let _ = AppendMenuW(hmenu, MF_STRING, ID_TRAY_PAUSE_RESUME, pause_text);

            let hide_text = if is_hidden {
                w!("Show Characters")
            } else {
                w!("Hide Characters")
            };
            let _ = AppendMenuW(hmenu, MF_STRING, ID_TRAY_HIDE_SHOW, hide_text);

            let mode_submenu = CreatePopupMenu().ok()?;
            let _ = AppendMenuW(mode_submenu, MF_STRING, ID_TRAY_MODE_ECO, w!("Eco Mode"));
            let _ = AppendMenuW(
                mode_submenu,
                MF_STRING,
                ID_TRAY_MODE_BALANCED,
                w!("Balanced Mode"),
            );
            let _ = AppendMenuW(
                mode_submenu,
                MF_STRING,
                ID_TRAY_MODE_SMOOTH,
                w!("Smooth Mode"),
            );
            let _ = AppendMenuW(
                mode_submenu,
                MF_STRING,
                ID_TRAY_MODE_GAMING,
                w!("Gaming Mode"),
            );
            let _ = AppendMenuW(
                hmenu,
                MF_POPUP,
                mode_submenu.0 as usize,
                w!("Performance Mode"),
            );

            let diag_text = if show_diagnostics {
                w!("Hide Diagnostics HUD")
            } else {
                w!("Show Diagnostics HUD")
            };
            let _ = AppendMenuW(hmenu, MF_STRING, ID_TRAY_TOGGLE_DIAG, diag_text);

            let _ = AppendMenuW(hmenu, MF_STRING, ID_TRAY_RESET_POS, w!("Reset Positions"));
            let _ = AppendMenuW(hmenu, MF_SEPARATOR, 0, PCWSTR::null());
            let _ = AppendMenuW(hmenu, MF_STRING, ID_TRAY_EXIT, w!("Exit Tiny68"));

            let _ = SetForegroundWindow(self.hwnd);
            let cmd = TrackPopupMenuEx(
                hmenu,
                (TPM_RETURNCMD | TPM_RIGHTBUTTON).0,
                pt_x,
                pt_y,
                self.hwnd,
                None,
            );
            let _ = DestroyMenu(hmenu);

            match cmd.0 as usize {
                ID_TRAY_SETTINGS => Some(TrayEvent::OpenSettings),
                ID_TRAY_ADD_MOCHI => Some(TrayEvent::AddMochi),
                ID_TRAY_ADD_KIKO => Some(TrayEvent::AddKiko),
                ID_TRAY_ADD_SHINOBI => Some(TrayEvent::AddShinobi),
                ID_TRAY_ADD_SAMURAI => Some(TrayEvent::AddSamurai),
                ID_TRAY_ADD_SCHOOLGIRL => Some(TrayEvent::AddSchoolgirl),
                ID_TRAY_ADD_FIGHTER => Some(TrayEvent::AddFighter),
                ID_TRAY_PAUSE_RESUME => Some(TrayEvent::TogglePause),
                ID_TRAY_HIDE_SHOW => Some(TrayEvent::ToggleHide),
                ID_TRAY_MODE_ECO => Some(TrayEvent::SetModeEco),
                ID_TRAY_MODE_BALANCED => Some(TrayEvent::SetModeBalanced),
                ID_TRAY_MODE_SMOOTH => Some(TrayEvent::SetModeSmooth),
                ID_TRAY_MODE_GAMING => Some(TrayEvent::SetModeGaming),
                ID_TRAY_TOGGLE_DIAG => Some(TrayEvent::ToggleDiagnostics),
                ID_TRAY_RESET_POS => Some(TrayEvent::ResetPositions),
                ID_TRAY_EXIT => Some(TrayEvent::Exit),
                _ => None,
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn show_character_context_menu(
        &self,
        pt_x: i32,
        pt_y: i32,
        display_name: &str,
        is_following: bool,
        is_sleeping: bool,
        current_personality: PersonalityType,
        _current_scale: f32,
    ) -> Option<CharacterMenuAction> {
        unsafe {
            let hmenu = CreatePopupMenu().ok()?;

            // Title: Character Name (Grayed header)
            let header_text: Vec<u16> = format!("{} (Companion)", display_name)
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();
            let _ = AppendMenuW(
                hmenu,
                MF_STRING | MF_GRAYED,
                ID_CHAR_TITLE,
                PCWSTR(header_text.as_ptr()),
            );
            let _ = AppendMenuW(hmenu, MF_SEPARATOR, 0, PCWSTR::null());

            // Follow Cursor
            let follow_flag = if is_following {
                MF_CHECKED
            } else {
                MF_UNCHECKED
            };
            let _ = AppendMenuW(
                hmenu,
                MF_STRING | follow_flag,
                ID_CHAR_FOLLOW,
                w!("Follow Cursor"),
            );

            // Sit Here
            let _ = AppendMenuW(hmenu, MF_STRING, ID_CHAR_SIT, w!("Sit Here"));

            // Sleep / Wake up
            let sleep_text = if is_sleeping {
                w!("Wake Up")
            } else {
                w!("Go to Sleep")
            };
            let _ = AppendMenuW(hmenu, MF_STRING, ID_CHAR_SLEEP_TOGGLE, sleep_text);

            let _ = AppendMenuW(hmenu, MF_SEPARATOR, 0, PCWSTR::null());

            // Personality submenu
            let per_menu = CreatePopupMenu().ok()?;
            let check_p = |pt: PersonalityType| {
                if pt == current_personality {
                    MF_CHECKED
                } else {
                    MF_UNCHECKED
                }
            };
            let _ = AppendMenuW(
                per_menu,
                MF_STRING | check_p(PersonalityType::Playful),
                ID_CHAR_PER_PLAYFUL,
                w!("Playful"),
            );
            let _ = AppendMenuW(
                per_menu,
                MF_STRING | check_p(PersonalityType::Lazy),
                ID_CHAR_PER_LAZY,
                w!("Lazy"),
            );
            let _ = AppendMenuW(
                per_menu,
                MF_STRING | check_p(PersonalityType::Curious),
                ID_CHAR_PER_CURIOUS,
                w!("Curious"),
            );
            let _ = AppendMenuW(
                per_menu,
                MF_STRING | check_p(PersonalityType::Energetic),
                ID_CHAR_PER_ENERGETIC,
                w!("Energetic"),
            );
            let _ = AppendMenuW(
                per_menu,
                MF_STRING | check_p(PersonalityType::Shy),
                ID_CHAR_PER_SHY,
                w!("Shy"),
            );
            let _ = AppendMenuW(
                per_menu,
                MF_STRING | check_p(PersonalityType::Calm),
                ID_CHAR_PER_CALM,
                w!("Calm"),
            );
            let _ = AppendMenuW(
                hmenu,
                MF_POPUP,
                per_menu.0 as usize,
                w!("Change Personality"),
            );

            // Scale submenu
            let scale_menu = CreatePopupMenu().ok()?;
            let _ = AppendMenuW(
                scale_menu,
                MF_STRING,
                ID_CHAR_SCALE_075,
                w!("0.75x (Small)"),
            );
            let _ = AppendMenuW(
                scale_menu,
                MF_STRING,
                ID_CHAR_SCALE_100,
                w!("1.0x (Normal)"),
            );
            let _ = AppendMenuW(
                scale_menu,
                MF_STRING,
                ID_CHAR_SCALE_125,
                w!("1.25x (Medium)"),
            );
            let _ = AppendMenuW(scale_menu, MF_STRING, ID_CHAR_SCALE_150, w!("1.5x (Large)"));
            let _ = AppendMenuW(scale_menu, MF_STRING, ID_CHAR_SCALE_200, w!("2.0x (Giant)"));
            let _ = AppendMenuW(hmenu, MF_POPUP, scale_menu.0 as usize, w!("Change Scale"));

            let _ = AppendMenuW(hmenu, MF_SEPARATOR, 0, PCWSTR::null());
            let _ = AppendMenuW(hmenu, MF_STRING, ID_CHAR_RESET, w!("Reset Position"));
            let _ = AppendMenuW(hmenu, MF_STRING, ID_CHAR_REMOVE, w!("Remove Character"));

            let _ = SetForegroundWindow(self.hwnd);
            let cmd = TrackPopupMenuEx(
                hmenu,
                (TPM_RETURNCMD | TPM_RIGHTBUTTON).0,
                pt_x,
                pt_y,
                self.hwnd,
                None,
            );
            let _ = DestroyMenu(hmenu);

            match cmd.0 as usize {
                ID_CHAR_FOLLOW => Some(CharacterMenuAction::ToggleFollowCursor),
                ID_CHAR_SIT => Some(CharacterMenuAction::SitHere),
                ID_CHAR_SLEEP_TOGGLE => Some(CharacterMenuAction::ToggleSleep),
                ID_CHAR_PER_PLAYFUL => Some(CharacterMenuAction::ChangePersonality(
                    PersonalityType::Playful,
                )),
                ID_CHAR_PER_LAZY => Some(CharacterMenuAction::ChangePersonality(
                    PersonalityType::Lazy,
                )),
                ID_CHAR_PER_CURIOUS => Some(CharacterMenuAction::ChangePersonality(
                    PersonalityType::Curious,
                )),
                ID_CHAR_PER_ENERGETIC => Some(CharacterMenuAction::ChangePersonality(
                    PersonalityType::Energetic,
                )),
                ID_CHAR_PER_SHY => {
                    Some(CharacterMenuAction::ChangePersonality(PersonalityType::Shy))
                }
                ID_CHAR_PER_CALM => Some(CharacterMenuAction::ChangePersonality(
                    PersonalityType::Calm,
                )),
                ID_CHAR_SCALE_075 => Some(CharacterMenuAction::ChangeScale(0.75)),
                ID_CHAR_SCALE_100 => Some(CharacterMenuAction::ChangeScale(1.0)),
                ID_CHAR_SCALE_125 => Some(CharacterMenuAction::ChangeScale(1.25)),
                ID_CHAR_SCALE_150 => Some(CharacterMenuAction::ChangeScale(1.5)),
                ID_CHAR_SCALE_200 => Some(CharacterMenuAction::ChangeScale(2.0)),
                ID_CHAR_RESET => Some(CharacterMenuAction::ResetPosition),
                ID_CHAR_REMOVE => Some(CharacterMenuAction::RemoveCharacter),
                _ => None,
            }
        }
    }
}

impl Drop for TrayIcon {
    fn drop(&mut self) {
        unsafe {
            let _ = Shell_NotifyIconW(NIM_DELETE, &self.nid);
        }
    }
}
