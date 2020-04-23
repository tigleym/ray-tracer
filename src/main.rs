use std::io::Result;

mod canvas;
mod tuples;
mod exercises;

use exercises::fire_projectiles;

fn main() -> Result<()> {
  fire_projectiles()?;

  Ok(())
}
