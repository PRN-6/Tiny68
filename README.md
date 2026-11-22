# Tiny68 — Desktop Companion Engine

<div align="center">

**Your Desktop. Their World.**

[![Build & Test](https://github.com/tiny68/tiny68/actions/workflows/ci.yml/badge.svg)](https://github.com/tiny68/tiny68/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE.txt)
[![Target: Windows 10/11 x64](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011%20x64-0078D6.svg)](https://www.microsoft.com/windows)
[![Rust: 2024 Stable](https://img.shields.io/badge/Rust-2024%20Stable-DEA584.svg)](https://www.rust-lang.org)

</div>

---

## 1. Project Vision

**Tiny68** is a lightweight, ultra-efficient Windows desktop companion application written in modern Rust. Small, adorable, animated characters live directly on your screen. They move around independently, react to the cursor, climb application window edges, rest on the taskbar, sleep, play, and interact with each other.

Characters appear as though they physically inhabit your computer screen.

### Tiny68 is NOT:
- A wallpaper or animated wallpaper.
- A screensaver.
- A game running inside a conventional window.
- An Electron or Chromium browser app.
- A shell hook or Explorer replacement.
- An application requiring Administrator privileges.

Tiny68 preserves your existing wallpaper, desktop icons, taskbar, opened programs, and workflow. Clicking through empty areas interacts directly with underlying applications, while clicking characters lets you pick them up, toss them, and interact with them.

---

## 2. Architecture & Design

Tiny68 is structured around an always-running, lightweight Rust core with decoupled on-demand settings and diagnostics:

```
+-------------------------------------------------------------------------+
|                  apps/tiny68 (Main Application & Tray)                  |
+-------------------------------------------------------------------------+
       |                           |                           |
+--------------+            +--------------+            +---------------+
|tiny68-overlay|            |tiny68-render |            |tiny68-behavior|
+--------------+            +--------------+            +---------------+
       |                           |                           |
+--------------+            +--------------+            +---------------+
|tiny68-platfrm|            |tiny68-anim   |            |tiny68-physics |
+--------------+            +--------------+            +---------------+
       \                           |                           /
        +--------------------------+--------------------------+
                                   |
                             +--------------+
                             | tiny68-core  |
                             +--------------+
```

### Modular Workspace Organization

| Crate / Application | Responsibility |
|---|---|
| `apps/tiny68` | Main Windows tray application, message loop, and simulation coordinator. |
| `apps/tiny68-settings` | Native Win32 settings GUI with dark theme, launched on demand. |
| `crates/tiny68-core` | Shared geometry primitives (`Point`, `Rect`, `Vec2`), identifiers, and enums. |
| `crates/tiny68-platform` | Windows DPI scaling, display bounds, surface tracking, and autostart registry. |
| `crates/tiny68-overlay` | Layered transparent window, `WM_NCHITTEST` dynamic routing, drag capture, tray. |
| `crates/tiny68-renderer` | 32-bit premultiplied BGRA blitter, texture caching, dirty rects, diagnostics HUD. |
| `crates/tiny68-animation` | Sprite animation playback, transitions, and horizontal directional flipping. |
| `crates/tiny68-behavior` | Hierarchical Finite State Machine (HFSM), personality profiles, autonomous scheduling. |
| `crates/tiny68-physics` | 2D kinematics, gravity, sweep surface landing, separation, window climbing. |
| `crates/tiny68-assets` | Character pack manifest validation, atlas loading, security path auditing. |
| `crates/tiny68-config` | Atomic JSON configuration persistence in `%LOCALAPPDATA%\Tiny68\config.json`. |
| `crates/tiny68-diagnostics`| Logging via `tracing`, real-time FPS, frame time, and latency percentiles. |
| `crates/tiny68-testing` | Headless test harnesses and simulation fixtures. |
| `benchmarks` | Standalone reproducible benchmark suite executing Scenarios A through F. |

---

## 3. Key Features

- **Seamless Click-Through & Direct Interaction:** Transparent screen regions pass mouse events unconditionally to underlying applications (`WM_NCHITTEST` returning `HTTRANSPARENT`). Clicking a character captures mouse input (`HTCLIENT`), allowing you to pick up, drag, and toss characters anywhere.
- **Original Character Packs:** Ships with two legally distributable, original characters:
  - **Mochi:** Sweet marshmallow chibi creature with soft pastel pink hues and playful antics.
  - **Kiko:** Mint/teal cat-eared desktop companion with a curious and energetic personality.
- **Surface Awareness & Window Climbing:** Characters detect open application window boundaries (`EnumWindows`, `GetWindowRect`) and taskbar edges. When walking into a window edge, characters climb vertical walls or turn around naturally.
- **Multi-Character Social Coexistence:** Characters avoid overlapping through horizontal separation impulses and react to each other with friendly gestures (waving, sitting together, playing).
- **Interactive Character Context Menu:** Right-click any character on your screen to toggle cursor following, force sit or sleep, adjust scale (0.75x to 2.0x), switch personality, or reset position.
- **Adaptive Performance Profiles:**
  - **Eco Mode:** 15 FPS target, reduced simulation ticks, maximum laptop battery life.
  - **Balanced Mode (Default):** 30 FPS target, smooth everyday motion using <0.5% CPU when idle and ≤2% CPU active.
  - **Smooth Mode:** 60 FPS target for ultra-fluid motion on high-refresh-rate displays.
  - **Gaming Mode:** Automatically pauses simulation and rendering during fullscreen games.
- **In-Engine Diagnostics HUD:** Optional semi-transparent HUD displaying real-time FPS, frame latency percentiles (Avg, P95, P99), active character counts, and screen bounds.
- **Zero Telemetry & 100% Offline:** No internet connectivity, no analytics SDKs, no screen recording, no administrator privileges required.

---

## 4. Performance Benchmarks

Measured on Windows 11 x64 using `cargo run --bin benchmark_runner`:

| Benchmark Scenario | Characters | Steps | Avg Time (ms) | P95 Time (ms) | P99 Time (ms) | Total Time (ms) | Budget (16.6ms @ 60 FPS) |
|---|---|---|---|---|---|---|---|
| **Scenario A: Idle Desktop** | 1 | 300 | 2.988 | 3.126 | 3.588 | 896.44 | **PASSED** (18% of budget) |
| **Scenario B: Normal Desktop** | 10 | 300 | 3.571 | 4.054 | 4.575 | 1071.63 | **PASSED** (21% of budget) |
| **Scenario C: Heavy Movement** | 50 | 300 | 5.867 | 6.260 | 7.020 | 1760.44 | **PASSED** (35% of budget) |
| **Scenario D: Gaming Mode** | 0 | 300 | 2.894 | 2.958 | 3.124 | 868.35 | **PASSED** (Suspended) |
| **Scenario E: Multi-Monitor (3840×1080)** | 20 | 300 | 6.918 | 7.109 | 8.157 | 2075.55 | **PASSED** (41% of budget) |
| **Scenario F: Long-Running Endurance** | 10 | 2000 | 3.471 | 3.575 | 4.026 | 6943.60 | **PASSED** (0 memory leak) |

---

## 5. Getting Started & Controls

### User Controls
- **Pick Up & Drag:** Click and hold your left mouse button on any character. Move to drag.
- **Drop / Toss:** Release the mouse button. The character falls under gravity and lands on application windows or the taskbar.
- **Character Context Menu:** Right-click directly on any character to:
  - Toggle **Follow Cursor**
  - **Sit Here**
  - **Sleep / Wake Up**
  - **Change Personality** (Playful, Lazy, Curious, Energetic, Shy, Calm)
  - **Change Scale** (0.75x Small, 1.0x Normal, 1.25x Medium, 1.5x Large, 2.0x Giant)
  - **Reset Position**
  - **Remove Character**
- **System Tray Icon:** Right-click the Tiny68 icon near the Windows taskbar clock to:
  - Open Settings UI
  - Add Mochi / Add Kiko
  - Pause / Resume Characters
  - Hide / Show Characters
  - Toggle Performance Modes (Eco, Balanced, Smooth, Gaming)
  - Toggle Diagnostics HUD
  - Exit Tiny68

---

## 6. Development & Build

### Prerequisites
- Windows 10 / 11 x64
- Stable Rust toolchain (Rust 1.80+)
- Git

### Build from Source
```powershell
# Clone the repository
git clone https://github.com/tiny68/tiny68.git
cd tiny68

# Build debug workspace
cargo build --workspace

# Build optimized release binaries
cargo build --workspace --release
```

Compiled binaries will be located in `target\release\`:
- `tiny68.exe` — Main companion engine
- `tiny68-settings.exe` — Native settings UI

### Quality Checks & Testing
```powershell
# Run code formatter check
cargo fmt --all -- --check

# Run linter
cargo clippy --workspace --all-targets -- -D warnings

# Run all workspace unit and integration tests
cargo test --workspace

# Run performance benchmark suite
cargo run --bin benchmark_runner
```

### Packaging & Installer
Create the production Windows setup executable using Inno Setup:
```powershell
iscc installer\tiny68_setup.iss
```
The resulting standalone installer will be output to `dist\Tiny68_Setup_v0.1.0.exe`.

---

## 7. Character Pack Format

Each character pack is declarative and safe:
```
assets/characters/<pack_id>/
├── manifest.json       # Versioned JSON metadata and frame timings
├── <atlas_file>.png    # 32-bit RGBA sprite atlas
├── preview.png         # Character preview thumbnail
└── LICENSE.txt         # Licensing and author attribution
```

External packs are validated against path traversal (`..`), image dimension limits (≤4096×4096 px), and payload size limits (≤50 MB). Executable scripts are strictly rejected.

---

## 8. License

Dual-licensed under either:
- **MIT License** ([LICENSE-MIT](LICENSE.txt))
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE.txt))

at your option. Original character artwork for Mochi and Kiko is licensed under **Creative Commons Attribution 4.0 International (CC-BY-4.0)**.

