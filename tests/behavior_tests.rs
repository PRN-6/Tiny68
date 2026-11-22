use tiny68_behavior::fsm::{BehaviorState, CharacterStateMachine};
use tiny68_behavior::personality::PersonalityProfile;
use tiny68_core::types::PersonalityType;

#[test]
fn test_state_machine_transition() {
    let mut fsm = CharacterStateMachine::new();
    assert_eq!(fsm.current_state, BehaviorState::Idle);

    fsm.state_timer = 5.0;
    fsm.transition_to(BehaviorState::Walk);

    assert_eq!(fsm.current_state, BehaviorState::Walk);
    assert_eq!(fsm.state_timer, 0.0, "Transition should reset state timer");
}

#[test]
fn test_personality_profiles() {
    let playful = PersonalityProfile::from_type(PersonalityType::Playful);
    let lazy = PersonalityProfile::from_type(PersonalityType::Lazy);

    assert!(playful.walk_speed > lazy.walk_speed, "Playful characters walk faster than lazy");
    assert!(lazy.sleep_chance > playful.sleep_chance, "Lazy characters sleep more often than playful");
}

