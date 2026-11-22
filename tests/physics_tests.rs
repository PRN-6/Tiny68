use tiny68_core::geom::{Point, Rect, Vec2};
use tiny68_physics::PhysicsBody;

#[test]
fn test_gravity_accelerates_falling_body() {
    let mut body = PhysicsBody::new(100.0, 100.0, 48.0, 48.0);
    let screen = Rect::new(0.0, 0.0, 1920.0, 1080.0);
    let surfaces = vec![];

    assert_eq!(body.velocity.y, 0.0);
    assert!(!body.is_grounded);

    body.integrate(0.1, &surfaces, &screen);

    assert!(body.velocity.y > 0.0, "Velocity Y should increase due to gravity");
    assert!(body.position.y > 100.0, "Position Y should move downward");
}

#[test]
fn test_surface_collision_lands_character() {
    let mut body = PhysicsBody::new(100.0, 390.0, 48.0, 48.0);
    let screen = Rect::new(0.0, 0.0, 1920.0, 1080.0);
    let window_surface = Rect::new(50.0, 440.0, 300.0, 20.0);
    let surfaces = vec![window_surface];

    body.velocity.y = 100.0;
    body.integrate(0.1, &surfaces, &screen);

    assert!(body.is_grounded, "Body should land on top edge of window");
    assert_eq!(body.position.y, 440.0 - 48.0);
    assert_eq!(body.velocity.y, 0.0);
}

#[test]
fn test_screen_boundary_clamping() {
    let mut body = PhysicsBody::new(-50.0, 2000.0, 48.0, 48.0);
    let screen = Rect::new(0.0, 0.0, 1920.0, 1080.0);
    let surfaces = vec![];

    body.integrate(0.01, &surfaces, &screen);

    assert!(body.position.x >= 0.0, "Body should be clamped within left screen edge");
    assert!(body.position.y <= 1080.0 - 48.0, "Body should not fall below screen bottom");
}

#[test]
fn test_dragging_disables_gravity_and_sets_velocity_zero() {
    let mut body = PhysicsBody::new(100.0, 100.0, 48.0, 48.0);
    let screen = Rect::new(0.0, 0.0, 1920.0, 1080.0);
    let surfaces = vec![];

    body.is_dragging = true;
    body.integrate(0.1, &surfaces, &screen);

    assert_eq!(body.velocity, Vec2::ZERO);
    assert_eq!(body.position, Point::new(100.0, 100.0));
}

#[test]
fn test_character_separation() {
    let mut b1 = PhysicsBody::new(100.0, 500.0, 48.0, 48.0);
    let mut b2 = PhysicsBody::new(110.0, 500.0, 48.0, 48.0);

    b1.separate_from(&mut b2, 48.0);

    let dist = (b2.position.x - b1.position.x).abs();
    assert!(
        dist >= 48.0,
        "Bodies should push apart to at least minimum distance"
    );
}

#[test]
fn test_window_climbing() {
    let mut body = PhysicsBody::new(150.0, 400.0, 48.0, 48.0);
    let screen = Rect::new(0.0, 0.0, 1920.0, 1080.0);
    let window_surface = Rect::new(150.0, 300.0, 200.0, 200.0);
    let surfaces = vec![window_surface];

    body.is_climbing = true;
    body.climb_speed = 200.0;

    for _ in 0..10 {
        body.integrate(0.1, &surfaces, &screen);
    }

    assert!(body.is_grounded, "Body should mount window top upon reaching surface");
    assert_eq!(body.position.y, 300.0 - 48.0);
    assert!(!body.is_climbing);
}

