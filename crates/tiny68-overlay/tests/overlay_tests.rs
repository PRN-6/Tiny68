use tiny68_core::geom::{Point, Rect};

#[test]
fn test_hit_test_character_bounds() {
    let char_rect = Rect::new(100.0, 200.0, 48.0, 48.0);

    // Inside bounds: should hit
    assert!(char_rect.contains_point(Point::new(100.0, 200.0)));
    assert!(char_rect.contains_point(Point::new(124.0, 224.0)));
    assert!(char_rect.contains_point(Point::new(148.0, 248.0)));

    // Outside bounds: click-through transparent desktop
    assert!(!char_rect.contains_point(Point::new(99.0, 200.0)));
    assert!(!char_rect.contains_point(Point::new(149.0, 200.0)));
    assert!(!char_rect.contains_point(Point::new(124.0, 199.0)));
    assert!(!char_rect.contains_point(Point::new(124.0, 249.0)));
}

#[test]
fn test_character_menu_actions() {
    use tiny68_core::types::PersonalityType;
    use tiny68_overlay::tray::CharacterMenuAction;

    let act1 = CharacterMenuAction::ToggleFollowCursor;
    let act2 = CharacterMenuAction::ChangePersonality(PersonalityType::Energetic);
    let act3 = CharacterMenuAction::ChangeScale(1.5);

    assert_eq!(act1, CharacterMenuAction::ToggleFollowCursor);
    assert_ne!(act1, act2);
    assert_eq!(act3, CharacterMenuAction::ChangeScale(1.5));
}
