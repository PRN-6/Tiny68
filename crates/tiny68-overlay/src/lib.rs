//! Transparent desktop overlay window, hit testing, click-through, dragging,
//! and Windows system tray integration.

pub mod tray;
pub mod window;

pub use tray::{CharacterMenuAction, TrayEvent, TrayIcon};
pub use window::{HitTestResult, OverlayWindow, WindowEvent};
