use tiny68_core::geom::Rect;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;

#[derive(Debug, Clone)]
pub struct MonitorInfo {
    pub bounds: Rect,
    pub work_area: Rect,
    pub is_primary: bool,
}

pub fn get_virtual_screen_bounds() -> Rect {
    unsafe {
        let x = GetSystemMetrics(SM_XVIRTUALSCREEN);
        let y = GetSystemMetrics(SM_YVIRTUALSCREEN);
        let cx = GetSystemMetrics(SM_CXVIRTUALSCREEN);
        let cy = GetSystemMetrics(SM_CYVIRTUALSCREEN);

        if cx > 0 && cy > 0 {
            Rect::new(x as f32, y as f32, cx as f32, cy as f32)
        } else {
            // Fallback to primary screen
            let w = GetSystemMetrics(SM_CXSCREEN).max(800);
            let h = GetSystemMetrics(SM_CYSCREEN).max(600);
            Rect::new(0.0, 0.0, w as f32, h as f32)
        }
    }
}

pub fn get_work_area_bounds() -> Rect {
    unsafe {
        let mut rc = RECT::default();
        if SystemParametersInfoW(
            SPI_GETWORKAREA,
            0,
            Some(&mut rc as *mut _ as *mut _),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
        .is_ok()
        {
            Rect::new(
                rc.left as f32,
                rc.top as f32,
                (rc.right - rc.left) as f32,
                (rc.bottom - rc.top) as f32,
            )
        } else {
            get_virtual_screen_bounds()
        }
    }
}
