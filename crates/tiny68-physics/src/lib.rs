//! Lightweight 2D kinematics, gravity, collisions, and screen boundaries.

use tiny68_core::geom::{Point, Rect, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallContact {
    None,
    LeftEdge,
    RightEdge,
}

#[derive(Debug, Clone)]
pub struct PhysicsBody {
    pub position: Point,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub gravity: f32,
    pub is_grounded: bool,
    pub width: f32,
    pub height: f32,
    pub is_dragging: bool,
    pub is_climbing: bool,
    pub climb_speed: f32,
}

impl PhysicsBody {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Point::new(x, y),
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            gravity: 980.0, // pixels/sec^2
            is_grounded: false,
            width,
            height,
            is_dragging: false,
            is_climbing: false,
            climb_speed: 60.0,
        }
    }

    pub fn bounding_box(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.width, self.height)
    }

    /// Separates two grounded or nearby bodies horizontally to prevent unnatural overlapping
    pub fn separate_from(&mut self, other: &mut PhysicsBody, min_dist: f32) {
        if self.is_dragging || other.is_dragging {
            return;
        }

        let center_self = self.position.x + self.width * 0.5;
        let center_other = other.position.x + other.width * 0.5;
        let delta_x = center_self - center_other;
        let dist = delta_x.abs();

        let y_overlap =
            (self.position.y - other.position.y).abs() < (self.height.max(other.height) * 0.75);

        if y_overlap && dist < min_dist {
            let overlap = min_dist - dist;
            let half = overlap * 0.5;
            if delta_x >= 0.0 {
                self.position.x += half;
                other.position.x -= half;
            } else {
                self.position.x -= half;
                other.position.x += half;
            }
        }
    }

    /// Checks if the body touches a vertical edge of an application window
    pub fn check_wall_contact(&self, surfaces: &[Rect]) -> Option<(WallContact, Rect)> {
        let left = self.position.x;
        let right = self.position.x + self.width;
        let top = self.position.y;
        let bottom = self.position.y + self.height;

        for s in surfaces {
            // Check if vertically aligned within window height
            if bottom > s.y + 10.0 && top < s.y + s.height {
                // Colliding with left edge of window (body moving right into it)
                if (right >= s.x - 4.0 && right <= s.x + 8.0) && self.velocity.x >= 0.0 {
                    return Some((WallContact::LeftEdge, *s));
                }
                // Colliding with right edge of window (body moving left into it)
                if (left <= s.x + s.width + 4.0 && left >= s.x + s.width - 8.0)
                    && self.velocity.x <= 0.0
                {
                    return Some((WallContact::RightEdge, *s));
                }
            }
        }
        None
    }

    pub fn integrate(&mut self, dt: f32, surfaces: &[Rect], screen_bounds: &Rect) {
        if self.is_dragging {
            self.velocity = Vec2::ZERO;
            self.is_grounded = false;
            self.is_climbing = false;
            return;
        }

        if self.is_climbing {
            self.velocity.x = 0.0;
            self.velocity.y = -self.climb_speed;
            self.position.y += self.velocity.y * dt;
            self.is_grounded = false;

            // Check if reached top of any surface
            for s in surfaces {
                if self.position.y <= s.y - self.height + 4.0
                    && (self.position.x + self.width >= s.x - 8.0
                        && self.position.x <= s.x + s.width + 8.0)
                {
                    self.position.y = s.y - self.height;
                    self.velocity.y = 0.0;
                    self.is_climbing = false;
                    self.is_grounded = true;
                    return;
                }
            }

            // Don't climb above screen top
            if self.position.y < screen_bounds.y {
                self.position.y = screen_bounds.y;
                self.is_climbing = false;
            }
            return;
        }

        let prev_foot_y = self.position.y + self.height;

        // Apply gravity if not grounded
        if !self.is_grounded {
            self.velocity.y += self.gravity * dt;
        }

        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;

        // Ground / surface collision checks
        let foot_y = self.position.y + self.height;
        let foot_x_min = self.position.x + 4.0;
        let foot_x_max = self.position.x + self.width - 4.0;

        let mut landed = false;

        // Check window top edges / surfaces with sweep test (prev_foot_y <= s.y <= foot_y)
        for s in surfaces {
            let crossed_surface = (prev_foot_y <= s.y + 2.0 && foot_y >= s.y)
                || (foot_y >= s.y && foot_y <= s.y + 24.0);

            if self.velocity.y >= 0.0
                && crossed_surface
                && foot_x_max >= s.x
                && foot_x_min <= s.x + s.width
            {
                self.position.y = s.y - self.height;
                self.velocity.y = 0.0;
                landed = true;
                break;
            }
        }

        // Screen bottom (taskbar boundary)
        let floor_y = screen_bounds.y + screen_bounds.height;
        if foot_y >= floor_y {
            self.position.y = floor_y - self.height;
            self.velocity.y = 0.0;
            landed = true;
        }

        self.is_grounded = landed;

        // Screen sides collision
        if self.position.x < screen_bounds.x {
            self.position.x = screen_bounds.x;
            self.velocity.x = 0.0;
        } else if self.position.x + self.width > screen_bounds.x + screen_bounds.width {
            self.position.x = screen_bounds.x + screen_bounds.width - self.width;
            self.velocity.x = 0.0;
        }
    }
}
