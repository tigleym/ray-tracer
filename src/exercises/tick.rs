use std::fs;
use std::io::Result;

// Exercise 1 & 2: Fire projectiles and see how far they go.
use crate::tuples::{ Vector, Point, Color };
use crate::canvas::Canvas;

pub struct Projectile {
  position: Point,
  velocity: Vector,
}

pub struct Environment {
  gravity: Vector,
  wind: Vector,
}

fn tick(env: &Environment, proj: &Projectile, canvas: &mut Canvas) -> Projectile {
  let position = proj.position + proj.velocity;
  let velocity = proj.velocity + env.gravity + env.wind;

  let pixel_x = position.x.round() as u32;
  let pixel_y = position.y.round() as u32;
  let within_bounds = pixel_x < canvas.width && pixel_y < canvas.height;

  if within_bounds {
    // canvas's y-coordinate is upside down to world coordinates. Convert the projectile's
    // coordinates to canvas coordinates by subtracting the projectile's y from the canvas
    // height.
    canvas.write_pixel(pixel_x, canvas.height - pixel_y, Color { r: 1.0, g: 0.5, b: 0.0 });
  }

  Projectile { position, velocity }
}

pub fn fire_projectiles() -> Result<()> {
  let mut p = Projectile {
    position: Point { x: 0.0, y: 1.0, z: 0.0 },
    velocity: Vector { x: 1.0, y: 1.8, z: 0.0 }.normalize() * 12.5
  };

  let e = Environment {
    gravity: Vector { x: 0.0, y: -0.1, z: 0.0 },
    wind: Vector { x: -0.01, y: 0.0, z: 0.0 }
  };

  let mut c = Canvas::new(1200, 700);

  let mut ticks = 0;

  loop {
    println!("Projectile position: {:?}", p.position);
    println!("Number of Ticks: {}", ticks);
    ticks += 1;

    p = tick(&e, &p, &mut c);

    if p.position.y <= 0.0 {
      break;
    }
  }

  fs::write("projectile.ppm", c.to_ppm())?;

  Ok(())
}
