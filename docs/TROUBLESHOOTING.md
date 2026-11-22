# Troubleshooting — Tiny68

## 1. Characters Do Not Appear on Screen
- Verify that Tiny68 is running in Task Manager (`tiny68.exe`).
- Check the System Tray: if characters were hidden, click the tray icon and select "Show Characters".
- If characters fell off an unplugged monitor, select "Reset Positions" in the tray menu to bring them back to the primary display.

---

## 2. Mouse Clicks Are Passing Through Characters
- Ensure that the character layer is not paused or disabled.
- Characters accept mouse clicks inside their visible bounding box. If you click outside the sprite boundary, the click passes through to the underlying application by design.

---

## 3. High CPU or Battery Usage on Laptops
- Open the System Tray menu -> **Performance Mode** -> select **Eco Mode**.
- Eco Mode drops simulation to 15 FPS and extends character sleep periods, reducing CPU usage to near 0%.

---

## 4. Running With Fullscreen Games
- By default, Tiny68 detects fullscreen games (`is_foreground_fullscreen()`) and automatically suspends rendering.
- If you want characters visible alongside borderless games, toggle the setting in `config.json` (`"pause_on_fullscreen_game": false`).

