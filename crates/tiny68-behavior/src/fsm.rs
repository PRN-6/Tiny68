use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BehaviorState {
    Idle,
    Walk,
    Run,
    Sit,
    Sleep,
    WakeUp,
    Jump,
    Fall,
    Land,
    Dragged,
    ClimbWindow,
    Curious,
    Wave,
    LookAround,
    Stretch,
    FollowCursor,
}

#[derive(Debug, Clone)]
pub struct CharacterStateMachine {
    pub current_state: BehaviorState,
    pub state_timer: f32,
    pub next_decision_in: f32,
}

impl CharacterStateMachine {
    pub fn new() -> Self {
        Self {
            current_state: BehaviorState::Idle,
            state_timer: 0.0,
            next_decision_in: 2.0,
        }
    }

    pub fn transition_to(&mut self, new_state: BehaviorState) {
        if self.current_state != new_state {
            self.current_state = new_state;
            self.state_timer = 0.0;
        }
    }
}

impl Default for CharacterStateMachine {
    fn default() -> Self {
        Self::new()
    }
}
