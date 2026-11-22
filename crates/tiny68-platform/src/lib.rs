//! Windows native platform integration: DPI, display enumeration, window geometry tracking,
//! fullscreen detection, and Explorer restart notifications.

pub mod autostart;
pub mod display;
pub mod window_tracker;

pub use autostart::{is_autostart_enabled, set_autostart};
pub use display::{get_virtual_screen_bounds, get_work_area_bounds, MonitorInfo};
pub use window_tracker::{enumerate_usable_surfaces, is_foreground_fullscreen};
