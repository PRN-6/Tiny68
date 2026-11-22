# Software Requirements Specification (SRS) — Tiny68

## 1. Introduction

### 1.1 Purpose
Tiny68 is a lightweight Windows desktop companion application written in modern Rust. It renders small, charming, animated characters directly over the Windows desktop without disrupting underlying application workflows.

### 1.2 Tagline
*Your Desktop. Their World.*

### 1.3 Scope
Tiny68 operates unobtrusively in the background via the Windows System Tray. Characters traverse the screen, walk on open application windows, rest on the taskbar, react to mouse interaction, and coexist peacefully with Windows applications.

---

## 2. Overall Description

### 2.1 Product Perspective
Tiny68 is NOT a wallpaper engine, screensaver, web-based simulation, Electron app, or Explorer replacement. It is a native, per-pixel alpha layered Windows desktop companion requiring no administrator privileges.

### 2.2 User Characteristics
- Windows 10/11 desktop users who desire engaging, animated desktop companions.
- Developers, designers, and gamers requiring minimal system footprint and zero input disruption.

---

## 3. Specific Requirements

### 3.1 Functional Requirements
- **FR-01 Overlay Transparency:** Transparent regions of the character layer must pass mouse input directly to underlying applications (`WM_NCHITTEST` returning `HTTRANSPARENT`).
- **FR-02 Character Hit Testing:** Mouse clicks within character bounding boxes must be captured (`HTCLIENT`), permitting picking up, dragging, and tossing characters.
- **FR-03 Character Animation:** Support multi-frame looping and non-looping animations (idle, walk, sit, sleep, jump, fall, land, drag) at configurable frame rates.
- **FR-04 Autonomous Behavior:** Characters utilize a Hierarchical Finite State Machine influenced by personality profiles (Playful, Lazy, Curious, Energetic, Shy, Calm).
- **FR-05 Surface Interaction:** Characters detect and land on open application windows (`EnumWindows`, `GetWindowRect`) and taskbar boundaries.
- **FR-06 System Tray Management:** Full native system tray icon with context menu for pausing, adding characters, toggling performance modes, and exiting.
- **FR-07 Configuration Persistence:** JSON settings persisted atomically in `%LOCALAPPDATA%\Tiny68\config.json`.
- **FR-08 Fullscreen Detection:** Automatically suspend or pause simulation during fullscreen games/applications.

### 3.2 Non-Functional Requirements
- **NFR-01 CPU Efficiency:** ≤0.5% CPU when idle; ≤2.0% average CPU with 10 animated characters.
- **NFR-02 Memory Footprint:** ≤120 MB private working set with 10 active characters.
- **NFR-03 Frame Timing:** Consistent 60 FPS in Smooth Mode, 30 FPS in Balanced Mode, 15 FPS in Eco Mode.
- **NFR-04 Zero Telemetry:** Completely offline, no tracking, no external network requests.

