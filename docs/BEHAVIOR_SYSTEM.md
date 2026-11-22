# Behavior Engine — Tiny68

## 1. Hierarchical Finite State Machine (HFSM)

Characters operate through discrete states:
- **Idle:** Standing still, breathing animation.
- **Walk:** Horizontal locomotion across screen or window boundaries.
- **Sit:** Resting on surfaces.
- **Sleep:** Low-energy restorative state.
- **Jump:** Upward impulse.
- **Fall:** Unsupported downward acceleration under gravity.
- **Land:** Ground impact recovery.
- **Dragged:** Interactive user manipulation via cursor.

---

## 2. Personality Profiles

Personalities modify autonomous decision distributions:

| Personality | Walk Speed | Jump Power | Curiosity | Sleep Chance | Idle Duration |
|---|---|---|---|---|---|
| **Playful** | 60 px/s | 280 px/s | 40% | 5% | 1.0–3.0 s |
| **Lazy** | 30 px/s | 150 px/s | 10% | 50% | 4.0–8.0 s |
| **Curious** | 50 px/s | 220 px/s | 60% | 10% | 1.5–3.5 s |
| **Energetic** | 90 px/s | 320 px/s | 50% | 2% | 0.5–1.5 s |
| **Shy** | 40 px/s | 180 px/s | 15% | 20% | 2.0–5.0 s |
| **Calm** | 45 px/s | 200 px/s | 25% | 20% | 3.0–6.0 s |

