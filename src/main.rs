mod canvas;
mod tuples;
mod exercises;

use exercises::fire_projectiles;
use canvas::Canvas;
use tuples::Color;

fn main() {
  let mut c = Canvas::new(10, 2);

  for row in 0..c.height {
    for col in 0..c.width {
      c.write_pixel(col, row, Color { r: 1.0, g: 0.8, b: 0.6});
    }
  }

  let ppm = c.to_ppm();

  println!("{}", ppm);
}
