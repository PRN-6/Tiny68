# Windows Integration Guide — Tiny68

## 1. Win32 Layered Window Architecture

Tiny68 employs Windows Layered Windows (`WS_EX_LAYERED`) paired with `UpdateLayeredWindow`.

### 1.1 Extended Window Styles
- `WS_EX_LAYERED`: Enables 32-bit BGRA per-pixel alpha composition directly with the Desktop Window Manager (DWM).
- `WS_EX_TOOLWINDOW`: Prevents the overlay window from cluttering the Alt+Tab application switcher.
- `WS_EX_TOPMOST`: Ensures characters remain visible above standard desktop windows.
- `WS_EX_NOACTIVATE`: Prevents clicking or interacting with characters from stealing foreground focus away from active applications.

---

## 2. Input Routing & Click-Through

One of the most critical aspects of Tiny68 is transparent click-through:
- If a window has `WS_EX_TRANSPARENT`, mouse events pass through unconditionally, making characters impossible to click or drag.
- If `WS_EX_TRANSPARENT` is omitted without dynamic hit-testing, the transparent fullscreen window blocks all clicks from reaching underlying desktop apps.

### 2.1 The WM_NCHITTEST Solution
Tiny68 intercepts `WM_NCHITTEST` in the window procedure:
```rust
WM_NCHITTEST => {
    if state.is_dragging {
        return LRESULT(HTCLIENT as isize);
    }
    let pt = Point::new(screen_x, screen_y);
    if (state.hit_test_cb)(pt).is_some() {
        LRESULT(HTCLIENT as isize)
    } else {
        LRESULT(HTTRANSPARENT as isize)
    }
}
```
When `WM_NCHITTEST` returns `HTTRANSPARENT`, the Windows input subsystem forwards the mouse message to the window directly beneath in the Z-order. When over a character, returning `HTCLIENT` routes the input to Tiny68 for dragging, clicking, and context menus.

---

## 3. Window Surface Detection

Tiny68 discovers top-level windows using `EnumWindows`:
- Filters out minimized (`IsIconic`), hidden (`!IsWindowVisible`), or small utility windows.
- Obtains window bounding rectangles via `GetWindowRect`.
- Feeds upper horizontal window edges to the physics system as walkable surfaces.

---

## 4. Fullscreen Application Detection

To preserve gaming performance, Tiny68 checks whether the foreground window occupies the entire display:
- Reads foreground window coordinates via `GetForegroundWindow` and `GetWindowRect`.
- Compares against primary screen metrics.
- Excludes shell windows (`Progman`, `WorkerW`, `Shell_TrayWnd`).
- When fullscreen gameplay is detected, Tiny68 automatically transitions to `SimulationState::Paused`.

