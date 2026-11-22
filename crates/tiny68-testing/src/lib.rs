//! Headless test runner, simulation fixtures, and property verification.

use tiny68_animation::AnimationPlayer;
use tiny68_assets::manifest::CharacterManifest;
use tiny68_behavior::fsm::CharacterStateMachine;
use tiny68_behavior::personality::PersonalityProfile;
use tiny68_core::geom::Rect;
use tiny68_physics::PhysicsBody;

pub struct MockCharacter {
    pub id: u64,
    pub name: String,
    pub physics: PhysicsBody,
    pub fsm: CharacterStateMachine,
    pub anim: AnimationPlayer,
    pub personality: PersonalityProfile,
}

impl MockCharacter {
    pub fn new(
        id: u64,
        name: &str,
        x: f32,
        y: f32,
        ptype: tiny68_core::types::PersonalityType,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            physics: PhysicsBody::new(x, y, 48.0, 48.0),
            fsm: CharacterStateMachine::new(),
            anim: AnimationPlayer::new("idle"),
            personality: PersonalityProfile::from_type(ptype),
        }
    }

    pub fn tick(
        &mut self,
        dt: f32,
        surfaces: &[Rect],
        screen: &Rect,
        manifest: &CharacterManifest,
    ) {
        self.physics.integrate(dt, surfaces, screen);
        self.anim.update(dt, manifest);
    }
}
