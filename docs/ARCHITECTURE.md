# Architecture — Tiny68

## 1. Workspace Organization

| Crate / Path | Responsibility |
|---|---|
| `apps/tiny68` | Main Windows entrypoint, message loop, tray lifecycle, game loop. |
| `apps/tiny68-settings` | Standalone settings viewer and configuration editor CLI. |
| `crates/tiny68-core` | Common geometry (`Point`, `Rect`, `Vec2`), identifiers, and enums. |
| `crates/tiny68-platform` | Native Win32 display enumeration, window surface tracking, fullscreen check. |
| `crates/tiny68-overlay` | Layered transparent window, `WM_NCHITTEST` routing, mouse drag, tray icon. |
| `crates/tiny68-renderer` | 32-bit BGRA premultiplied alpha buffer, texture manager, blitter, dirty rects. |
| `crates/tiny68-animation` | Sprite animation playback, frame progression, directional flipping. |
| `crates/tiny68-behavior` | Hierarchical finite state machines, personality profiles, autonomous schedules. |
| `crates/tiny68-physics` | 2D kinematics, gravity, surface landing collision, screen boundary clamping. |
| `crates/tiny68-assets` | Character pack manifest parsing, security checks, image limits. |
| `crates/tiny68-config` | JSON configuration loading, atomic disk writing, corruption recovery. |
| `crates/tiny68-diagnostics`| Logging setup via `tracing`, real-time FPS and percentile metrics. |
| `crates/tiny68-testing` | Headless simulation harness and test fixtures. |
| `benchmarks` | Standalone reproducible benchmark suite executing Scenarios A-F. |

---

## 2. Design Invariants

1. **Safety:** All unsafe Win32 FFI operations are encapsulated behind safe Rust interfaces with RAII handle disposal.
2. **Deterministic Simulation:** Kinematic equations use delta-time scaling to ensure character speed is identical across 60Hz, 144Hz, and 240Hz monitors.
3. **Zero Polling in Rendering:** Static scenes sleep according to the active performance profile target frame rate.
4. **Single-Instance Enforcement:** Global named mutex `Local\Tiny68_SingleInstance_Mutex` prevents accidental duplicate processes.

