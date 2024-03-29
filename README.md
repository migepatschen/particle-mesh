# Particle Mesh

A set of small programs to draw a moving particle mesh onto the screen.
It started with a version using [Nannou](https://nannou.cc/) but now also has one using [Macroquad](https://macroquad.rs/).

## Usage

The programs offer some parameters to control the number of particles, their radius and behavior:

```text
particle-mesh-XXX [OPTIONS]

OPTIONS:
    -b, --bounce
            Flag indicating if the particles should wrap around the screen or bounce off [default: wrap around]

    -h, --help
            Print help information

    -n, --number-of-particles <NUMBER_OF_PARTICLES>
            Specifies the number of particles to be drawn [default: 150]

    -r, --radius <RADIUS>
            The radius of a particle [default: 2.5]

    -V, --version
            Print version information
```