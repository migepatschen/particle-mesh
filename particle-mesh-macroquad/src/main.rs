mod line;
mod model;
mod particle;

use clap::Parser;
use macroquad::{
    prelude::{is_key_down, Color, KeyCode, Rect},
    rand::gen_range,
    shapes::{draw_circle, draw_line},
    window::{clear_background, next_frame, screen_height, screen_width, Conf},
};

use line::Line;
use model::Model;
use particle::Particle;
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};

/// The arguments to be parsed by [clap]
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

/// Configures the window
fn window_conf() -> Conf {
    Conf {
        window_title: "particle-mesh".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

/// Creates the model, and loops the [macroquad] logic
#[macroquad::main(window_conf)]
async fn main() {
    let mut model = model();

    // preparation is done, now let's use macroquad to draw something
    loop {
        if is_key_down(KeyCode::Escape) {
            // exits the application
            return;
        }

        view(&model);
        update(&mut model);

        next_frame().await
    }
}

/// Called to create the model used for the application. Uses the command line parameters (or default values)
fn model() -> Model {
    // parse the command line arguments
    let args = Args::parse();
    let radius = args.radius;

    // we create the initial positions of the particles
    // because we don't want them to be outside the application's window, we use the window dimensions as boundary
    let boundary = Rect::new(0.0, 0.0, screen_width(), screen_height());
    let mut particles: Vec<Particle> = Vec::with_capacity(args.number_of_particles);
    for _ in 0..args.number_of_particles {
        // center of the particle
        let x = gen_range(boundary.left(), boundary.right());
        let y = gen_range(boundary.bottom(), boundary.top());

        // particle speed
        let speed = gen_range(0.2, 2.0);

        // direction to move
        let dir_x = gen_range(-1.0, 1.0);
        let dir_y = gen_range(-1.0, 1.0);

        particles.push(Particle::new(x, y, radius, speed, dir_x, dir_y));
    }

    Model::new(boundary, particles, !args.bounce)
}

/// Called to update the model within the application loop.
fn update(model: &mut Model) {
    // calculate the particle centers
    let wrap_around = model.wrap_around;
    let boundary = model.boundary;

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

            if y < boundary.top() {
                y = boundary.bottom() - 1.0;
            } else if y > boundary.bottom() {
                y = boundary.top() + 1.0;
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

            if y < boundary.top() {
                y = boundary.top() + 1.0;
                particle.direction.y *= -1.0
            } else if y > boundary.bottom() {
                y = boundary.bottom() - 1.0;
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

/// Called to present the [Model] to the surface of a window on your display.
fn view(model: &Model) {
    clear_background(model.color_bg);

    let r = model.color_fg.r;
    let g = model.color_fg.g;
    let b = model.color_fg.b;

    for line in &model.lines {
        draw_line(
            line.start.x,
            line.start.y,
            line.end.x,
            line.end.y,
            line.w,
            Color::new(r, g, b, line.a),
        );
    }

    for particle in &model.particles {
        draw_circle(
            particle.center.x,
            particle.center.y,
            particle.radius,
            model.color_fg,
        );
    }
}
