# Software Design Specification (SDS) — Tiny68

## 1. Architectural Overview

Tiny68 is structured as a modular Cargo workspace separating core simulation logic, graphics compositing, native Windows integration, behavior state machines, and configuration management.

```
+-------------------------------------------------------+
|                 apps/tiny68 (Main Application)        |
+-------------------------------------------------------+
       |                  |                  |
+--------------+   +--------------+   +---------------+
|tiny68-overlay|   |tiny68-render |   |tiny68-behavior|
+--------------+   +--------------+   +---------------+
       |                  |                  |
+--------------+   +--------------+   +---------------+
|tiny68-platfrm|   |tiny68-anim   |   |tiny68-physics |
+--------------+   +--------------+   +---------------+
       \                  |                  /
        +-----------------+-----------------+
                          |
                  +--------------+
                  | tiny68-core  |
                  +--------------+
```

---

## 2. Detailed Component Design

### 2.1 Overlay & Interaction (`tiny68-overlay`)
- Creates a top-level Win32 window with `WS_EX_LAYERED | WS_EX_TOOLWINDOW | WS_EX_TOPMOST | WS_EX_NOACTIVATE`.
- Intercepts `WM_NCHITTEST`:
  - When cursor hits a character: returns `HTCLIENT`.
  - When cursor is on empty desktop: returns `HTTRANSPARENT`.
- Initiates dragging via `SetCapture` on `WM_LBUTTONDOWN` and releases with `ReleaseCapture` on `WM_LBUTTONUP`.

### 2.2 Rendering Pipeline (`tiny68-renderer`)
- Maintains a 32-bit BGRA premultiplied software frame buffer.
- Performs Porter-Duff Over alpha blending:
  $$\text{Out} = \text{Src} + \text{Dst} \times (1 - \text{Src}_\alpha)$$
- Delivers buffer to DWM compositor via Win32 `UpdateLayeredWindow` with `AC_SRC_ALPHA` and `ULW_ALPHA`.

### 2.3 Physics & Kinematics (`tiny68-physics`)
- Integrates fixed-step 2D kinematics:
  $$v_{t+1} = v_t + a \cdot \Delta t$$
  $$p_{t+1} = p_t + v \cdot \Delta t$$
- Continuous collision sweep testing prevents ground/window tunneling during landing.
- Drag physics overrides velocity integration and releases characters into gravity drops.

### 2.4 Behavior State Machine (`tiny68-behavior`)
- Hierarchical Finite State Machine with state priority resolution:
  - Dragging > Falling > Autonomous states (Walk, Jump, Sit, Sleep, Idle).
- Decisions scheduled with randomized cooldown intervals weighted by Personality profiles.

