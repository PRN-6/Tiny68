# Changelog — Tiny68

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-11-22

### Added
- Native Windows transparent overlay with `WS_EX_LAYERED`, `WS_EX_TOOLWINDOW`, `WS_EX_TOPMOST`, `WS_EX_NOACTIVATE`.
- Dynamic hit-testing routing via `WM_NCHITTEST` supporting seamless click-through and character picking/dragging.
- Continuous sweep 2D collision detection preventing tunneling on window top edges and taskbar.
- Hierarchical Finite State Machine supporting Idle, Walk, Sit, Sleep, Jump, Fall, Land, and Dragged states.
- Six distinct personality profiles: Playful, Lazy, Curious, Energetic, Shy, and Calm.
- 32-bit BGRA premultiplied alpha software blitter with horizontal mirroring and scaling.
- Original character packs: **Mochi** (pink pastel chibi) and **Kiko** (mint teal cat-eared chibi) with PNG atlases and manifests.
- System tray integration with context menus, pause/resume, character spawning, and performance profile toggles.
- Explorer restart recovery detecting `TaskbarCreated` message.
- Fullscreen application auto-pause preserving gaming performance.
- Atomic configuration persistence in `%LOCALAPPDATA%\Tiny68\config.json`.
- Performance benchmark runner executing Scenarios A through F.
- Complete Inno Setup installer script and GitHub Actions CI workflow.

