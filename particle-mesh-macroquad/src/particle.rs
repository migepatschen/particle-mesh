//! Contains everything related to the particles to be drawn onto the screen.
use macroquad::prelude::{vec2, Vec2};

/// The data needed to draw a particle
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Particle {
    /// The particle's center point
    pub(crate) center: Vec2,
    /// The radius of the particle
    pub(crate) radius: f32,
    /// The movement speed
    pub(crate) speed: f32,
    /// The direction to move to
    pub(crate) direction: Vec2,
}

impl Particle {
    /// Creates a new particle based on the given parameters
    pub(crate) fn new(
        x: f32,
        y: f32,
        radius: f32,
        speed: f32,
        direction_x: f32,
        direction_y: f32,
    ) -> Self {
        Particle {
            center: vec2(x, y),
            radius,
            speed,
            direction: vec2(direction_x, direction_y),
        }
    }

    /// Replaces the particle's center coordinates
    pub(crate) fn set_center(&mut self, x: f32, y: f32) {
        self.center.x = x;
        self.center.y = y;
    }
}
