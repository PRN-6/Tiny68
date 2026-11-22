# Rendering Pipeline — Tiny68

## 1. Pixel Format & Color Model

Tiny68 utilizes 32-bit BGRA Premultiplied Alpha format:
- Memory representation on Little-Endian x86_64: `[B, G, R, A]`.
- As a `u32`: `0xAARRGGBB`.

### Premultiplication Rule:
$$\text{Color}_{\text{premul}} = \frac{\text{Color} \times \alpha + 127}{255}$$

Premultiplication ensures seamless anti-aliased character boundaries without dark halos or white fringe artifacts when composed against arbitrary desktop backgrounds.

---

## 2. Blitting & Compositing

Tiny68 implements an optimized Porter-Duff 'Over' operator:
```rust
let inv_sa = 255 - sa;
let out_b = (sb + ((db * inv_sa + 127) / 255)).min(255);
let out_g = (sg + ((dg * inv_sa + 127) / 255)).min(255);
let out_r = (sr + ((dr * inv_sa + 127) / 255)).min(255);
let out_a = (sa + ((da * inv_sa + 127) / 255)).min(255);
```

### Features:
- Horizontal sprite mirroring for bidirectional locomotion.
- Arbitrary floating-point sprite scaling (0.5x to 3.0x).
- Sprite alpha modulation for fade transitions and visibility toggling.
- Texture atlasing sharing single texture allocations across multiple character instances.

