use serde::{Deserialize, Serialize};

/// Unique runtime identifier for a character instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CharacterId(pub u64);

impl std::fmt::Display for CharacterId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "char-{}", self.0)
    }
}

/// Direction character is facing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FacingDirection {
    #[default]
    Right,
    Left,
}

impl FacingDirection {
    pub fn is_left(&self) -> bool {
        matches!(self, FacingDirection::Left)
    }
}

/// Personality profile driving autonomous behavior decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PersonalityType {
    #[default]
    Playful,
    Lazy,
    Curious,
    Energetic,
    Shy,
    Calm,
}

/// Performance profile for adaptive simulation and rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PerformanceProfile {
    Eco,
    #[default]
    Balanced,
    Smooth,
    Gaming,
}

impl PerformanceProfile {
    pub fn target_fps(&self) -> u32 {
        match self {
            Self::Eco => 15,
            Self::Balanced => 30,
            Self::Smooth => 60,
            Self::Gaming => 0, // suspended / dormant
        }
    }

    pub fn fixed_timestep_seconds(&self) -> f32 {
        match self {
            Self::Eco => 1.0 / 30.0,
            Self::Balanced => 1.0 / 60.0,
            Self::Smooth => 1.0 / 60.0,
            Self::Gaming => 1.0 / 10.0,
        }
    }
}

/// Overall engine simulation state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
    Hidden,
}
