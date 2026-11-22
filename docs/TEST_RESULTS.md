# Test Results — Tiny68

## 1. Execution Summary
All tests executed on Windows 11 x64 via `cargo test --workspace`.

```
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 2. Detailed Results by Crate

### `tiny68-physics`
- `test_gravity_accelerates_falling_body`: **PASSED** (verified velocity increases downwards under gravity).
- `test_surface_collision_lands_character`: **PASSED** (continuous sweep collision prevents tunneling on window top edges).
- `test_screen_boundary_clamping`: **PASSED** (clamped to screen rect edges).
- `test_dragging_disables_gravity_and_sets_velocity_zero`: **PASSED** (drag overrides physics).

### `tiny68-behavior`
- `test_state_machine_transition`: **PASSED** (state updates cleanly and resets timer).
- `test_personality_profiles`: **PASSED** (verified variances between Playful and Lazy profiles).

### `tiny68-animation`
- `test_animation_frame_progression`: **PASSED** (verified 100ms frame thresholds and loop wrapping).
- `test_non_looping_animation_finishes`: **PASSED** (marked finished at end of frame sequence).
- `test_facing_direction_flip`: **PASSED** (horizontal facing flag toggles correctly).

### `tiny68-config`
- `test_config_atomic_save_and_load`: **PASSED** (atomic temporary file replacement works).
- `test_config_fallback_on_corrupt_file`: **PASSED** (malformed JSON safely recovers to defaults).

### `tiny68-assets`
- `test_path_traversal_detection`: **PASSED** (parent traversal `..` rejected).
- `test_valid_pack_relative_path`: **PASSED** (valid relative paths accepted).

### `tiny68-renderer`
- `test_pixel_buffer_clear_and_resize`: **PASSED** (zeroing and reallocation verified).
- `test_premultiplied_alpha_blitting`: **PASSED** (Porter-Duff Over composite matches exact BGRA expectation).

### `tiny68-overlay`
- `test_hit_test_character_bounds`: **PASSED** (inside returns hit; outside transparent regions pass through).

