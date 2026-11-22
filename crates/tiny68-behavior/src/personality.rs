use tiny68_core::types::PersonalityType;

#[derive(Debug, Clone)]
pub struct PersonalityProfile {
    pub ptype: PersonalityType,
    pub walk_speed: f32,
    pub jump_power: f32,
    pub curiosity_chance: f32,
    pub sleep_chance: f32,
    pub climb_chance: f32,
    pub social_chance: f32,
    pub idle_duration_range: (f32, f32),
}

impl PersonalityProfile {
    pub fn from_type(ptype: PersonalityType) -> Self {
        match ptype {
            PersonalityType::Playful => Self {
                ptype,
                walk_speed: 60.0,
                jump_power: 280.0,
                curiosity_chance: 0.4,
                sleep_chance: 0.05,
                climb_chance: 0.35,
                social_chance: 0.6,
                idle_duration_range: (1.0, 3.0),
            },
            PersonalityType::Lazy => Self {
                ptype,
                walk_speed: 30.0,
                jump_power: 150.0,
                curiosity_chance: 0.1,
                sleep_chance: 0.5,
                climb_chance: 0.05,
                social_chance: 0.2,
                idle_duration_range: (4.0, 8.0),
            },
            PersonalityType::Curious => Self {
                ptype,
                walk_speed: 50.0,
                jump_power: 220.0,
                curiosity_chance: 0.6,
                sleep_chance: 0.1,
                climb_chance: 0.5,
                social_chance: 0.4,
                idle_duration_range: (1.5, 3.5),
            },
            PersonalityType::Energetic => Self {
                ptype,
                walk_speed: 90.0,
                jump_power: 320.0,
                curiosity_chance: 0.5,
                sleep_chance: 0.02,
                climb_chance: 0.6,
                social_chance: 0.5,
                idle_duration_range: (0.5, 1.5),
            },
            PersonalityType::Shy => Self {
                ptype,
                walk_speed: 40.0,
                jump_power: 180.0,
                curiosity_chance: 0.15,
                sleep_chance: 0.2,
                climb_chance: 0.1,
                social_chance: 0.1,
                idle_duration_range: (2.0, 5.0),
            },
            PersonalityType::Calm => Self {
                ptype,
                walk_speed: 45.0,
                jump_power: 200.0,
                curiosity_chance: 0.25,
                sleep_chance: 0.2,
                climb_chance: 0.2,
                social_chance: 0.3,
                idle_duration_range: (3.0, 6.0),
            },
        }
    }
}
