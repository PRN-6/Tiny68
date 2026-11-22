# Development Log — Tiny68

## Milestone 1: Repository Foundation & Architecture
- Initialized Git repository and configured Cargo workspace with 14 modular packages.
- Established core geometry, identifiers, and configuration structures.
- Setup `tracing` diagnostics and percentile metrics tracker.

## Milestone 2: Transparent Overlay & Input Routing
- Evaluated Win32 layered window architecture.
- Implemented `OverlayWindow` with `WS_EX_LAYERED` and `UpdateLayeredWindow`.
- Intercepted `WM_NCHITTEST` returning `HTCLIENT` over active characters and `HTTRANSPARENT` over empty screen space.
- Verified click-through and mouse capture dragging with `SetCapture` and `ReleaseCapture`.

## Milestone 3: Original Character Art & Sprite Engine
- Created original chibi character packs: **Mochi** and **Kiko**.
- Implemented programmatic anti-aliased sprite generator and standalone pack generator.
- Added texture atlas management and caching.

## Milestone 4: Physics & Window Surface Integration
- Implemented 2D kinematics with gravity, acceleration, and jumping.
- Added sweep collision detection to reliably land on window top edges.
- Integrated `EnumWindows` and `GetWindowRect` to track usable application window surfaces.
- Implemented fullscreen foreground detection to automatically pause characters during games.

## Milestone 5: System Tray & User Settings
- Implemented `TrayIcon` with `Shell_NotifyIconW`.
- Added context popup menu for toggling simulation states, performance modes, and adding characters.
- Built `tiny68-settings` standalone CLI for configuration inspection and factory reset.

## Milestone 6: Benchmarks, Hardening & Packaging
- Implemented benchmark runner for Scenarios A through F, confirming sub-6ms step times for 50 characters.
- Wrote unit and integration test suite across all crates.
- Verified 100% test pass rate and strict Clippy compliance (`-D warnings`).
- Created Inno Setup installer script and GitHub Actions CI workflow.

