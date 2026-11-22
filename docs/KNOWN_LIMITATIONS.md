# Known Limitations — Tiny68

## 1. Elevated (Administrator) Windows
In Windows, User Account Control (UAC) integrity levels prevent standard user applications from sending input messages to elevated (Administrator) windows. Tiny68 runs with standard user privileges, so character interactions occurring over elevated windows are constrained by Windows security boundaries.

## 2. Exclusive Fullscreen Direct3D / Vulkan Applications
Exclusive fullscreen games take complete hardware control of the display surface, preventing any desktop window overlay from appearing above them. Tiny68 detects this state and suspends rendering to preserve gaming performance and battery life. Borderless windowed games are supported.

## 3. Complex Non-Rectangular Window Shapes
Windows with non-standard custom shapes (translucent rounded corners or custom regional cutouts) are approximated by their outer bounding rectangles for character landing surfaces.

