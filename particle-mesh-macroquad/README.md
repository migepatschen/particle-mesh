# Particle Mesh (Macroquad)

A small program to draw a moving particle mesh onto the screen with [Macroquad](https://macroquad.rs/).

## Usage

The program offers some parameters to control the number of particles, their radius and behavior:

```text
particle-mesh-macroquad.exe [OPTIONS]

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
