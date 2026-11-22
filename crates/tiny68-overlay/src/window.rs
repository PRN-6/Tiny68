use std::sync::atomic::{AtomicU32, Ordering};
use tiny68_core::geom::{Point, Rect};
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;

static WM_TASKBAR_CREATED: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Clone)]
pub enum WindowEvent {
    MouseDown {
        character_index: Option<usize>,
        pt: Point,
    },
    MouseDrag {
        pt: Point,
    },
    MouseUp {
        pt: Point,
    },
    RightClick {
        character_index: Option<usize>,
        pt: Point,
    },
    DisplayChanged,
    DpiChanged(u32),
    ExplorerRestarted,
    CloseRequested,
}

pub struct HitTestResult {
    pub hit_character_index: Option<usize>,
}

pub type HitTestCallback = Box<dyn Fn(Point) -> Option<usize> + 'static>;

pub struct OverlayWindow {
    pub hwnd: HWND,
    pub width: u32,
    pub height: u32,
    pub mem_dc: HDC,
    pub dib_bitmap: HBITMAP,
    pub dib_bits: *mut u32,
    pub is_dragging: bool,
}

impl OverlayWindow {
    pub fn new(
        title: &str,
        bounds: Rect,
        hit_test_cb: HitTestCallback,
        event_sender: std::sync::mpsc::Sender<WindowEvent>,
    ) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        unsafe {
            let instance = windows::Win32::System::LibraryLoader::GetModuleHandleW(None)?;
            let class_name = w!("Tiny68_Overlay_Class");

            let wnd_class = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(overlay_wnd_proc),
                hInstance: instance.into(),
                lpszClassName: class_name,
                hCursor: LoadCursorW(None, IDC_ARROW)?,
                ..Default::default()
            };

            let _ = RegisterClassExW(&wnd_class);

            let taskbar_msg = RegisterWindowMessageW(w!("TaskbarCreated"));
            WM_TASKBAR_CREATED.store(taskbar_msg, Ordering::SeqCst);

            let ex_style = WS_EX_LAYERED | WS_EX_TOOLWINDOW | WS_EX_TOPMOST | WS_EX_NOACTIVATE;

            let hwnd = CreateWindowExW(
                ex_style,
                class_name,
                PCWSTR(windows::core::HSTRING::from(title).as_ptr()),
                WS_POPUP,
                bounds.x as i32,
                bounds.y as i32,
                bounds.width as i32,
                bounds.height as i32,
                None,
                None,
                instance,
                None,
            )?;

            // Allocate DIB section for 32-bit premultiplied alpha UpdateLayeredWindow
            let hdc_screen = GetDC(None);
            let mem_dc = CreateCompatibleDC(hdc_screen);

            let bmi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: bounds.width as i32,
                    biHeight: -(bounds.height as i32), // Top-down DIB
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB.0,
                    ..Default::default()
                },
                ..Default::default()
            };

            let mut dib_bits_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
            let dib_bitmap =
                CreateDIBSection(mem_dc, &bmi, DIB_RGB_COLORS, &mut dib_bits_ptr, None, 0)?;

            SelectObject(mem_dc, dib_bitmap);
            ReleaseDC(None, hdc_screen);

            let wnd_state = Box::new(WindowState {
                hit_test_cb,
                event_sender,
                is_dragging: false,
            });

            SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(wnd_state) as isize);

            let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE);

            Ok(Self {
                hwnd,
                width: bounds.width as u32,
                height: bounds.height as u32,
                mem_dc,
                dib_bitmap,
                dib_bits: dib_bits_ptr as *mut u32,
                is_dragging: false,
            })
        }
    }

    pub fn resize(&mut self, bounds: Rect) {
        unsafe {
            self.width = bounds.width as u32;
            self.height = bounds.height as u32;

            let _ = SetWindowPos(
                self.hwnd,
                HWND_TOPMOST,
                bounds.x as i32,
                bounds.y as i32,
                bounds.width as i32,
                bounds.height as i32,
                SWP_NOACTIVATE | SWP_SHOWWINDOW,
            );

            // Recreate DIB section for new dimensions
            let hdc_screen = GetDC(None);
            let bmi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: bounds.width as i32,
                    biHeight: -(bounds.height as i32),
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB.0,
                    ..Default::default()
                },
                ..Default::default()
            };

            let mut dib_bits_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
            if let Ok(new_bitmap) = CreateDIBSection(
                self.mem_dc,
                &bmi,
                DIB_RGB_COLORS,
                &mut dib_bits_ptr,
                None,
                0,
            ) {
                let old = SelectObject(self.mem_dc, new_bitmap);
                let _ = DeleteObject(old);
                self.dib_bitmap = new_bitmap;
                self.dib_bits = dib_bits_ptr as *mut u32;
            }
            ReleaseDC(None, hdc_screen);
        }
    }

    /// Present the 32-bit premultiplied pixel buffer to Windows compositor.
    pub fn present(&self, buffer: &[u32], width: u32, height: u32) {
        if self.dib_bits.is_null() {
            return;
        }

        unsafe {
            let copy_w = width.min(self.width) as usize;
            let copy_h = height.min(self.height) as usize;

            for y in 0..copy_h {
                let src_offset = y * (width as usize);
                let dst_offset = y * (self.width as usize);
                std::ptr::copy_nonoverlapping(
                    buffer.as_ptr().add(src_offset),
                    self.dib_bits.add(dst_offset),
                    copy_w,
                );
            }

            let pt_dst = POINT { x: 0, y: 0 };
            let size = SIZE {
                cx: self.width as i32,
                cy: self.height as i32,
            };
            let pt_src = POINT { x: 0, y: 0 };

            let blend = BLENDFUNCTION {
                BlendOp: AC_SRC_OVER as u8,
                BlendFlags: 0,
                SourceConstantAlpha: 255,
                AlphaFormat: AC_SRC_ALPHA as u8,
            };

            let hdc_screen = GetDC(None);
            let _ = UpdateLayeredWindow(
                self.hwnd,
                hdc_screen,
                Some(&pt_dst),
                Some(&size),
                self.mem_dc,
                Some(&pt_src),
                COLORREF(0),
                Some(&blend),
                ULW_ALPHA,
            );
            ReleaseDC(None, hdc_screen);
        }
    }
}

impl Drop for OverlayWindow {
    fn drop(&mut self) {
        unsafe {
            if !self.dib_bitmap.is_invalid() {
                let _ = DeleteObject(self.dib_bitmap);
            }
            if !self.mem_dc.is_invalid() {
                let _ = DeleteDC(self.mem_dc);
            }
            if !self.hwnd.is_invalid() {
                let _ = DestroyWindow(self.hwnd);
            }
        }
    }
}

struct WindowState {
    hit_test_cb: HitTestCallback,
    event_sender: std::sync::mpsc::Sender<WindowEvent>,
    is_dragging: bool,
}

unsafe extern "system" fn overlay_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let raw = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
    if raw == 0 {
        return DefWindowProcW(hwnd, msg, wparam, lparam);
    }
    let state = &mut *(raw as *mut WindowState);

    let taskbar_created = WM_TASKBAR_CREATED.load(Ordering::Relaxed);
    if taskbar_created != 0 && msg == taskbar_created {
        let _ = state.event_sender.send(WindowEvent::ExplorerRestarted);
        return LRESULT(0);
    }

    match msg {
        WM_NCHITTEST => {
            if state.is_dragging {
                return LRESULT(HTCLIENT as isize);
            }

            let x = (lparam.0 as i16) as f32;
            let y = ((lparam.0 >> 16) as i16) as f32;

            if (state.hit_test_cb)(Point::new(x, y)).is_some() {
                LRESULT(HTCLIENT as isize)
            } else {
                LRESULT(HTTRANSPARENT as isize)
            }
        }
        WM_LBUTTONDOWN => {
            let x = (lparam.0 as i16) as f32;
            let y = ((lparam.0 >> 16) as i16) as f32;
            let pt = Point::new(x, y);

            let hit = (state.hit_test_cb)(pt);
            if hit.is_some() {
                state.is_dragging = true;
                SetCapture(hwnd);
                let _ = state.event_sender.send(WindowEvent::MouseDown {
                    character_index: hit,
                    pt,
                });
            }
            LRESULT(0)
        }
        WM_MOUSEMOVE => {
            if state.is_dragging {
                let x = (lparam.0 as i16) as f32;
                let y = ((lparam.0 >> 16) as i16) as f32;
                let _ = state.event_sender.send(WindowEvent::MouseDrag {
                    pt: Point::new(x, y),
                });
            }
            LRESULT(0)
        }
        WM_LBUTTONUP => {
            if state.is_dragging {
                state.is_dragging = false;
                let _ = ReleaseCapture();
                let x = (lparam.0 as i16) as f32;
                let y = ((lparam.0 >> 16) as i16) as f32;
                let _ = state.event_sender.send(WindowEvent::MouseUp {
                    pt: Point::new(x, y),
                });
            }
            LRESULT(0)
        }
        WM_RBUTTONUP => {
            let x = (lparam.0 as i16) as f32;
            let y = ((lparam.0 >> 16) as i16) as f32;
            let pt = Point::new(x, y);
            let hit = (state.hit_test_cb)(pt);
            let _ = state.event_sender.send(WindowEvent::RightClick {
                character_index: hit,
                pt,
            });
            LRESULT(0)
        }
        WM_DISPLAYCHANGE => {
            let _ = state.event_sender.send(WindowEvent::DisplayChanged);
            LRESULT(0)
        }
        WM_DPICHANGED => {
            let dpi = (wparam.0 & 0xFFFF) as u32;
            let _ = state.event_sender.send(WindowEvent::DpiChanged(dpi));
            LRESULT(0)
        }
        WM_CLOSE => {
            let _ = state.event_sender.send(WindowEvent::CloseRequested);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
