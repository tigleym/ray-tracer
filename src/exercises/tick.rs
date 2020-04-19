// Exercise 1: Fire projectiles and see how far they go.
use crate::tuples::{ Vector, Point };

pub struct Projectile {
  position: Point,
  velocity: Vector,
}

pub struct Environment {
  gravity: Vector,
  wind: Vector,
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
  let position = proj.position + proj.velocity;
  let velocity = proj.velocity + env.gravity + env.wind;

  Projectile { position, velocity }
}

pub fn fire_projectiles() {
  let mut p = Projectile {
    position: Point { x: 0.0, y: 1.0, z: 0.0 },
    velocity: Vector { x: 1.0, y: 1.0, z: 0.0 }.normalize()
  };

  let e = Environment {
    gravity: Vector { x: 0.0, y: -0.1, z: 0.0 },
    wind: Vector { x: -0.01, y: 0.0, z: 0.0 }
  };

  println!("Starting projectile position: {:?}", p.position);
  let mut ticks = 0;

  loop {
    if p.position.y <= 0.0 {
      break;
    }

    println!("Projectile position: {:?}", p.position);
    println!("Number of Ticks: {}", ticks);
    ticks += 1;

    p = tick(&e, &p);
  }
}
