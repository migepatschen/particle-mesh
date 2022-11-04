//! Contains the struct to hold the data needed to draw the particle mesh onto the screen
use nannou::{
    color::named,
    prelude::{window, Srgb, WindowId},
};

use crate::{line::Line, particle::Particle};

/// The data needed to draw the particle mesh onto the screen
pub(crate) struct Model {
    /// Required by [Nannou](https://nannou.cc/), but ignored
    _window: window::Id,
    /// All [Particle]s to be drawn onto the screen. The number of particles does not change during runtime.
    pub(crate) particles: Vec<Particle>,
    /// All [Line]s to be drawn onto the screen. The number of lines does change during runtime.
    pub(crate) lines: Vec<Line>,
    /// Flag indicating if the [Particle]s are going to "screen wrap" or "bounce off" the window's border.
    pub(crate) wrap_around: bool,
    /// The background color used
    pub(crate) color_bg: Srgb<u8>,
    /// The foreground color used to draw the [Line]s and [Particle]s
    pub(crate) color_fg: Srgb<u8>,
}

impl Model {
    /// Creates the initial data to start drawing with
    pub(crate) fn new(window_id: WindowId, particles: Vec<Particle>, wrap_around: bool) -> Self {
        Model {
            _window: window_id,
            particles,
            lines: Vec::new(),
            wrap_around,
            color_bg: named::DARKORCHID,
            color_fg: named::ORCHID,
        }
    }
}
