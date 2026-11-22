# Character Pack Format — Tiny68

## 1. Directory Structure

Each character pack is stored in a dedicated folder or archive containing:
```
characters/<pack_id>/
├── manifest.json
├── <atlas_file>.png
├── preview.png
└── LICENSE.txt
```

---

## 2. Manifest Schema (`manifest.json`)

```json
{
  "format_version": 1,
  "id": "mochi",
  "display_name": "Mochi",
  "author": "Tiny68 Team",
  "license": "CC-BY-4.0",
  "atlas_file": "mochi_atlas.png",
  "sprite_width": 48,
  "sprite_height": 48,
  "anchor_x": 0.5,
  "anchor_y": 1.0,
  "animations": {
    "idle": {
      "looping": true,
      "frames": [
        { "x": 0, "y": 0, "width": 48, "height": 48, "duration_ms": 500 },
        { "x": 48, "y": 0, "width": 48, "height": 48, "duration_ms": 500 }
      ]
    }
  }
}
```

---

## 3. Security Limits

- Path traversal attempts (`..`, absolute paths) are strictly rejected.
- Maximum atlas image dimensions: 4096×4096 px.
- Maximum uncompressed asset size: 50 MB.
- No executable code or scripts allowed in character packs.

