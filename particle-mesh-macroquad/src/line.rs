//! Contains everything related to the lines to be drawn onto the screen.
use macroquad::prelude::Vec2;

/// The data needed to draw a line between [particles](crate::particle::Particle)
#[derive(Debug, Clone)]
pub(crate) struct Line {
    /// first point
    pub(crate) start: Vec2,
    /// second point
    pub(crate) end: Vec2,
    /// alpha value
    pub(crate) a: f32,
    /// line width
    pub(crate) w: f32,
}

impl Line {
    /// Creates a new line between the given points.
    ///
    /// Returns [`None`] if:
    /// - the calculated *distance* between both points is >= than [MAX_LINE_LENGTH]
    /// - the calculated *alpha value* (based on the calculated distance) of the line to draw is `0`
    /// - the calculated *line weight* (based on the calculated distance) is `0`
    pub(crate) fn from_vec2s(start: Vec2, end: Vec2) -> Option<Self> {
        if start != end {
            let distance = start.distance(end);
            if distance < MAX_LINE_LENGTH {
                // calculate the alpha value
                let a = map_range(distance, 0.0, MAX_LINE_LENGTH, 255.0, 0.0);
                if a <= 0.0 {
                    // not visible
                    return None;
                }
                // calculate the width value
                let w: f32 = map_range(distance, 0.0, MAX_LINE_LENGTH, LINE_WEIGHT, 0.1);
                if w == 0.0 {
                    // not visible
                    return None;
                }
                Some(Line::new(start, end, a, w))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Creates a new line
    fn new(start: Vec2, end: Vec2, a: f32, w: f32) -> Self {
        Line { start, end, a, w }
    }
}

/// The weight of the [Line]s to be drawn
const LINE_WEIGHT: f32 = 1.5;
/// If two [Particle](crate::particle::Particle)s are as far or farther away from each other than this value, they will not be connected with a [Line]
const MAX_LINE_LENGTH: f32 = 200.0;

/// Maps a value from an input range to an output range.
fn map_range(val: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (val - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
}
