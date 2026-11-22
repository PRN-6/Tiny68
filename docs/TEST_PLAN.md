# Test Plan — Tiny68

## 1. Overview
The Tiny68 testing strategy encompasses automated unit tests, headless integration tests, deterministic physics simulation fixtures, security validation, and Windows native integration verification.

---

## 2. Test Suites

### 2.1 Unit Tests
- **Kinematics & Physics:** Gravity integration, window landing detection, screen clamping, dragging override.
- **Behavior & HFSM:** State transitions, state timer resets, personality profile variances.
- **Animation System:** Progression of frames, looping rollover, non-looping finish, horizontal flipping.
- **Configuration:** Atomic temporary file write, persistence, fallback on corrupted JSON.
- **Assets & Security:** Rejection of path traversal (`..`), safe relative path resolution, size limits.
- **Rendering:** Memory buffer resizing, clear operations, premultiplied alpha blitting accuracy.

### 2.2 Benchmarks
- Automated execution of Scenarios A through F measuring frame time percentiles (Average, P95, P99).

### 2.3 Windows Native Verification Matrix
- Layered transparent window creation and click-through.
- System tray icon recreation on Explorer restart (`TaskbarCreated`).
- Window drag with `SetCapture` / `ReleaseCapture`.
- Single instance mutex enforcement.

