//! Contains the struct to hold the data needed to draw the particle mesh onto the screen
use macroquad::{color::Color, color_u8, prelude::Rect};

use crate::{line::Line, particle::Particle};

/// The data needed to draw the particle mesh onto the screen
pub(crate) struct Model {
    /// All [Particle]s to be drawn onto the screen. The number of particles does not change during runtime.
    pub(crate) particles: Vec<Particle>,
    /// All [Line]s to be drawn onto the screen. The number of lines does change during runtime.
    pub(crate) lines: Vec<Line>,
    /// Flag indicating if the [Particle]s are going to "screen wrap" or "bounce off" the window's border.
    pub(crate) wrap_around: bool,
    /// The background color used
    pub(crate) color_bg: Color,
    /// The foreground color used to draw the [Line]s and [Particle]s
    pub(crate) color_fg: Color,
    /// The window dimensions as boundary
    pub(crate) boundary: Rect,
}

impl Model {
    /// Creates the initial data to start drawing with
    pub(crate) fn new(boundary: Rect, particles: Vec<Particle>, wrap_around: bool) -> Self {
        Model {
            boundary,
            particles,
            lines: Vec::new(),
            wrap_around,
            color_bg: color_u8!(153, 50, 204, 255), // DARKORCHID
            color_fg: color_u8!(218, 112, 214, 255), // ORCHID
        }
    }
}
