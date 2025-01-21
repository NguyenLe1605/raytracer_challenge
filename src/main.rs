use std::{thread::sleep, time::Duration};

use raytracer_challenge::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Projectile {
    pub position: Tuple, // a point
    pub velocity: Tuple, // a vector
}

#[derive(Debug, Copy, Clone)]
pub struct Environment {
    pub gravity: Tuple, // a vector
    pub wind: Tuple,    // a vector
}

fn tick(env: Environment, proj: Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile { position, velocity }
}

fn main() {
    let mut p = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.0, 0.0).normalize(),
    };

    let e = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    let mut t = 0;

    while p.position.y > 0.0 {
        p = tick(e, p);
        t += 1;
        println!(
            "Projectile position: {} - {} after {} ticks",
            p.position.x, p.position.y, t
        );
        sleep(Duration::from_millis(500));
    }
    println!("Number of ticks: {}", t);
}
