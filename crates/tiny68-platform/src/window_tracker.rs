use tiny68_core::geom::Rect;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;

/// Enumerate visible non-cloaked top-level application windows to find surfaces.
pub fn enumerate_usable_surfaces() -> Vec<Rect> {
    let mut surfaces = Vec::new();
    unsafe {
        let surfaces_ptr = &mut surfaces as *mut Vec<Rect>;

        unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
            if !IsWindowVisible(hwnd).as_bool() || IsIconic(hwnd).as_bool() {
                return TRUE;
            }

            let mut rc = RECT::default();
            if GetWindowRect(hwnd, &mut rc).is_ok() {
                let width = (rc.right - rc.left) as f32;
                let height = (rc.bottom - rc.top) as f32;

                // Only consider substantial windows, ignore 0-sized or tiny tooltips
                if width >= 100.0 && height >= 80.0 {
                    let surfaces = &mut *(lparam.0 as *mut Vec<Rect>);
                    surfaces.push(Rect::new(rc.left as f32, rc.top as f32, width, height));
                }
            }
            TRUE
        }

        let _ = EnumWindows(Some(enum_proc), LPARAM(surfaces_ptr as isize));
    }
    surfaces
}

/// Checks whether foreground window is a fullscreen exclusive / borderless game or app.
pub fn is_foreground_fullscreen() -> bool {
    unsafe {
        let fg = GetForegroundWindow();
        if fg.0.is_null() {
            return false;
        }

        let mut rc = RECT::default();
        if GetWindowRect(fg, &mut rc).is_err() {
            return false;
        }

        let scr_w = GetSystemMetrics(SM_CXSCREEN);
        let scr_h = GetSystemMetrics(SM_CYSCREEN);

        let w = rc.right - rc.left;
        let h = rc.bottom - rc.top;

        // Covers entire primary screen and is at 0,0
        if rc.left <= 0 && rc.top <= 0 && w >= scr_w && h >= scr_h {
            // Check if it's the desktop or Shell_TrayWnd
            let mut class_name = [0u16; 64];
            let len = GetClassNameW(fg, &mut class_name);
            if len > 0 {
                let name = String::from_utf16_lossy(&class_name[..len as usize]);
                if name == "Progman" || name == "WorkerW" || name == "Shell_TrayWnd" {
                    return false;
                }
            }
            return true;
        }
        false
    }
}
