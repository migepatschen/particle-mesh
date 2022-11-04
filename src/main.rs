//! Program to draw a moving particle mesh onto the screen

mod line;
mod model;
mod particle;

use crate::line::Line;
use crate::model::Model;
use crate::particle::Particle;
use clap::Parser;
use nannou::prelude::{random_range, rgba, App, Frame, Update};
use rayon::prelude::*;

/// Draws a moving particle mesh onto the screen
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// Specifies the number of particles to be drawn.
    #[clap(short, long, value_parser, default_value_t = 150)]
    number_of_particles: usize,
    /// The radius of a particle.
    #[clap(short, long, value_parser, default_value_t = 2.5)]
    radius: f32,
    /// Flag indicating if the particles should wrap around the screen or bounce off [default: wrap around]
    #[clap(short, long, action)]
    bounce: bool,
}

/// Creates the [Nannou](https://nannou.cc/) application and runs it.
fn main() {
    nannou::app(model).update(update).run();
}

/// Called by [Nannou](https://nannou.cc/) to create the model used for the application. Uses the command line parameters (or default values)
fn model(app: &App) -> Model {
    // parse the command line arguments
    let args = Args::parse();
    let radius = args.radius;

    // we create our application window and store the ID of the window for alter use
    let window_id = app.new_window().fullscreen().view(view).build().unwrap();

    // we create the initial positions of the particles
    // because we don't want them to be outside the application's window, we use the window dimensions as boundary
    let boundary = app.window_rect();
    let mut particles: Vec<Particle> = Vec::with_capacity(args.number_of_particles);
    for _ in 0..args.number_of_particles {
        // center of the particle
        let x = random_range(boundary.left(), boundary.right());
        let y = random_range(boundary.bottom(), boundary.top());

        // particle speed
        let speed = random_range(0.2, 2.0);

        // direction to move
        let dir_x = random_range(-1.0, 1.0);
        let dir_y = random_range(-1.0, 1.0);

        particles.push(Particle::new(x, y, radius, speed, dir_x, dir_y));
    }
    Model::new(window_id, particles, !args.bounce)
}

/// Called by [Nannou](https://nannou.cc/) to update the model within the application loop.
fn update(app: &App, model: &mut Model, _update: Update) {
    // calculate the particle centers
    let wrap_around = model.wrap_around;
    let boundary = app.window_rect();

    model.particles.par_iter_mut().for_each(|particle| {
        let mut x = particle.center.x + particle.direction.x * particle.speed;
        let mut y = particle.center.y + particle.direction.y * particle.speed;

        if wrap_around {
            // to get the "screen wrap" effect, we need to change
            // the particle's position to the other side of the window
            if x < boundary.left() {
                x = boundary.right() - 1.0;
            } else if x > boundary.right() {
                x = boundary.left() + 1.0;
            }

            if y < boundary.bottom() {
                y = boundary.top() - 1.0;
            } else if y > boundary.top() {
                y = boundary.bottom() + 1.0;
            }
        } else {
            // to get the "bouncing particles" effect, we need to change
            // the particle's direction without messing up the angle
            if x < boundary.left() {
                x = boundary.left() + 1.0;
                particle.direction.x *= -1.0
            } else if x > boundary.right() {
                x = boundary.right() - 1.0;
                particle.direction.x *= -1.0
            }

            if y < boundary.bottom() {
                y = boundary.bottom() + 1.0;
                particle.direction.y *= -1.0
            } else if y > boundary.top() {
                y = boundary.top() - 1.0;
                particle.direction.y *= -1.0
            }
        }

        particle.set_center(x, y);
    });

    // calculate the lines
    model.lines.clear();
    for (i1, p1) in model.particles.iter().enumerate() {
        for (i2, p2) in model.particles.iter().enumerate() {
            if i2 <= i1 {
                // we don't want to draw lines twice
                continue;
            }
            // only visible lines are of interest to us
            match Line::from_vec2s(p1.center, p2.center) {
                Some(line) => model.lines.push(line),
                None => continue,
            }
        }
    }
}

/// Called by [Nannou](https://nannou.cc/) to present the [Model] to the surface of a window on your display.
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(model.color_bg);

    let r = model.color_fg.red;
    let g = model.color_fg.green;
    let b = model.color_fg.blue;

    for line in &model.lines {
        draw.line()
            .weight(line.w)
            .color(rgba(r, g, b, line.a))
            .points(line.start, line.end);
    }

    for particle in &model.particles {
        draw.ellipse()
            .radius(particle.radius)
            .color(model.color_fg)
            .xy(particle.center);
    }

    draw.to_frame(app, &frame).unwrap();
}
