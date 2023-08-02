# Particle Mesh (Nannou)

A small program to draw a moving particle mesh onto the screen with [Nannou](https://nannou.cc/).

## Usage

The program offers some parameters to control the number of particles, their radius and behavior:

```text
particle-mesh-nannou.exe [OPTIONS]

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

## Example animations

### 10 particles with a radius of 2.5 wrapping around

![10 particles with a radius of 2.5 wrapping around](./docs/wrap_around_320x240.png "10 particles with a radius of 2.5 wrapping around")

### 10 particles with a radius of 2.5 bouncing off the borders

![10 particles with a radius of 2.5 bouncing off the borders](./docs/bounce_320x240.png "10 particles with a radius of 2.5 bouncing off the borders")
