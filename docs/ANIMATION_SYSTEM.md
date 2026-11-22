# Animation System — Tiny68

## 1. Overview

Tiny68 features a frame-accurate sprite animation system driven by `AnimationPlayer`.

## 2. Data Structures

- `FrameDef`: Coordinates `(x, y, width, height)` in the texture atlas and duration in milliseconds.
- `AnimationDef`: Collection of frames with looping behavior flag.
- `AnimationPlayer`: Runtime controller maintaining current animation name, frame index, accumulated elapsed time, facing direction, and playback speed multiplier.

## 3. Transition System

Animations transition smoothly between behavior states:
- `idle` -> `walk`: Directional facing updated based on velocity.
- `jump` -> `fall`: Triggered when upward velocity reverses ($v_y \ge 0$).
- `fall` -> `land`: Triggered on ground/surface contact.
- `drag`: Activated immediately upon mouse down on character bounds.

