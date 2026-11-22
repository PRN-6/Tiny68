# Security & Privacy Model — Tiny68

## 1. Zero Network & Zero Telemetry
Tiny68 operates completely offline:
- Zero background network connections.
- Zero analytics, telemetry, or crash-reporting servers.
- No user screen recording or window content scraping.
- Only window bounding rectangles (`GetWindowRect`) are queried for physics surface landing.

---

## 2. Character Pack Sandbox
External or user-provided character packs are treated as untrusted data:
1. **Path Traversal Protection:** All asset paths are checked against directory traversal (`..`) attacks.
2. **Dimension Clamping:** Images larger than 4096×4096 pixels are rejected to prevent decompression bombs.
3. **File Size Limits:** Uncompressed asset payloads are capped at 50 MB.
4. **Declarative Art Only:** No script execution, bytecode, or native DLL loading is permitted from character packs.

---

## 3. Windows Native Safety
- No global keyboard hooks or low-level keyboard sniffing.
- No DLL injection into external processes.
- No administrator privilege requirement for ordinary execution.
- Single-instance mutex prevents duplicate background processes.

