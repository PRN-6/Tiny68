# Performance Strategy & Engineering — Tiny68

## 1. Guiding Philosophy
*Tiny68 brings life to Windows without slowing Windows down.*

Every frame rendered and every decision evaluated must justify its CPU, GPU, memory, and battery consumption.

---

## 2. Key Optimization Techniques

### 2.1 Fixed-Timestep Simulation
The simulation integrates at fixed delta-time intervals ($60\text{ Hz}$ or adaptive $15\text{--}30\text{ Hz}$ in Eco mode). This decouples physics and animations from high-refresh-rate displays (144Hz/240Hz), avoiding excessive CPU iterations.

### 2.2 Texture Atlasing & Single Allocations
Instead of loading distinct PNG images per frame or per character, characters share a single decoded 32-bit premultiplied BGRA texture atlas. Multiple instances of Mochi and Kiko draw from the same atlas memory.

### 2.3 Event-Driven Behavior Scheduling
Characters do not make costly decisions every frame. Idle, sitting, and sleeping characters sleep with timers scheduled 2.0 to 5.0 seconds into the future.

### 2.4 Cached Window Surface Geometry
Desktop window boundaries are enumerated and cached every 1.0 second rather than polling `EnumWindows` at 60 FPS.

### 2.5 Adaptive Performance Profiles
- **Eco Mode:** 15 FPS target, reduced simulation ticks, max battery savings.
- **Balanced Mode (Default):** 30 FPS target, smooth everyday motion.
- **Smooth Mode:** 60 FPS target for plugged-in desktop systems.
- **Gaming Mode:** Zero continuous frame submissions while a fullscreen game is active.

