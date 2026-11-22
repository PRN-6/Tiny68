//! Core models, geometric primitives, and orchestration types for Tiny68.

pub mod geom;
pub mod types;

pub use geom::{Point, Rect, Vec2};
pub use types::{
    CharacterId, FacingDirection, PerformanceProfile, PersonalityType, SimulationState,
};
