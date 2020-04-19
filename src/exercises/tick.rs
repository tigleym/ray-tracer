// Exercise 1: Fire projectiles and see how far they go.
use crate::tuples::{ Tuple, point, vector, normalize };

pub struct Projectile {
  position: Tuple,
  velocity: Tuple,
}

pub struct Environment {
  gravity: Tuple,
  wind: Tuple,
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
  let position = proj.position + proj.velocity;
  let velocity = proj.velocity + env.gravity + env.wind;

  Projectile { position, velocity }
}

pub fn fire_projectiles() {
  let mut p = Projectile {
    position: point(0.0, 1.0, 0.0),
    velocity: normalize(&vector(2.0, 2.0, 0.0))
  };

  let e = Environment {
    gravity: vector(0.0, -0.1, 0.0),
    wind: vector(-0.01, 0.0, 0.0)
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
