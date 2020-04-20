use crate::tuples::Color;

const PPM_IDENTIFIER: &str = "P3";
const MAX_COLOR_VALUE: f64 = 255.0;
const MAX_LINE_LENGTH: u32 = 70;

pub struct Canvas {
  pub width: u32,
  pub height: u32,
  pixel_data: Vec<Vec<Color>>
}

impl Canvas {
  pub fn new(width: u32, height: u32) -> Canvas {
    Canvas {
      width,
      height,
      pixel_data: vec![vec![Color { r: 0.0, g: 0.0, b: 0.0 }; width as usize]; height as usize]
    }
  }

  pub fn to_ppm(&self) -> String {
    let mut output = String::from("");
    let identifier = PPM_IDENTIFIER;
    let width = self.width.to_string();
    let height = self.height.to_string();
    let max_color_value = MAX_COLOR_VALUE.to_string();

    // Construct header
    output.push_str(identifier);
    output.push_str("\n");
    output.push_str(&width);
    output.push_str(" ");
    output.push_str(&height);
    output.push_str("\n");
    output.push_str(&max_color_value);
    output.push_str("\n");

    // Write pixel data
    for row in 0..self.height {
      let mut pixel = String::from("");
      let mut line_length: u32 = pixel.chars().count() as u32;

      for col in 0..self.width {
        let color = self.pixel_data[row as usize][col as usize];
        let r = &(color.r as i32).to_string();
        let g = &(color.g as i32).to_string();
        let b = &(color.b as i32).to_string();

        // Add an extra char count to line_length because of the additional whitespace.
        line_length += r.chars().count() as u32 + 1;
        trim_line(&mut pixel, &mut line_length);
        pixel.push_str(r);
        pixel.push_str(" ");

        line_length += g.chars().count() as u32 + 1;
        trim_line(&mut pixel, &mut line_length);
        pixel.push_str(g);
        pixel.push_str(" ");

        line_length += b.chars().count() as u32 + 1;
        trim_line(&mut pixel, &mut line_length);
        pixel.push_str(b);

        if col == self.width - 1 {
          pixel.push_str("\n");
        } else {
          pixel.push_str(" ");
        }
      }

      output.push_str(&pixel);
    }

    output
  }

  pub fn write_pixel(&mut self, column: u32, row: u32, color: Color) {
    let scaled_color = Color {
      r: (color.r * MAX_COLOR_VALUE as f64).min(255.0).max(0.0).ceil(),
      g: (color.g * MAX_COLOR_VALUE as f64).min(255.0).max(0.0).ceil(),
      b: (color.b * MAX_COLOR_VALUE as f64).min(255.0).max(0.0).ceil(),
    };

    self.pixel_data[row as usize][column as usize] = scaled_color;
  }
}

fn trim_line(pixel_data: &mut String, line_length: &mut u32){
  if *line_length > MAX_LINE_LENGTH {
    // Remove the trailing whitespace before inserting a newline.
    pixel_data.pop();
    pixel_data.push_str("\n");
    *line_length = 0;
  }
}

#[test]
fn constructing_ppm_header() {
  let c = Canvas::new(5, 3);
  let ppm = c.to_ppm();
  let mut lines = ppm.lines();

  assert_eq!(lines.next(), Some("P3"));
  assert_eq!(lines.next(), Some("5 3"));
  assert_eq!(lines.next(), Some("255"));
}

#[test]
fn constructing_ppm_pixel_data() {
  let mut c = Canvas::new(5, 3);
  c.write_pixel(0, 0, Color { r: 1.5, g: 0.0, b: 0.0 });
  c.write_pixel(2, 1, Color { r: 0.0, g: 0.5, b: 0.0 });
  c.write_pixel(4, 2, Color { r: -0.5, g: 0.0, b: 1.0 });
  let ppm = c.to_ppm();
  let mut lines = ppm.lines();

  // Skip the header lines.
  lines.next();
  lines.next();
  lines.next();

  assert_eq!(lines.next(), Some("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0"));
  assert_eq!(lines.next(), Some("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0"));
  assert_eq!(lines.next(), Some("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"));
}

#[test]
fn splitting_long_lines_ppm() {
  let mut c = Canvas::new(10, 2);

  for row in 0..c.height {
    for col in 0..c.width {
      c.write_pixel(col, row, Color { r: 1.0, g: 0.8, b: 0.6});
    }
  }

  let ppm = c.to_ppm();
  let mut lines = ppm.lines();

  // Skip the header lines.
  lines.next();
  lines.next();
  lines.next();

  assert_eq!(lines.next(), Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"));
  assert_eq!(lines.next(), Some("153 255 204 153 255 204 153 255 204 153 255 204 153"));
  assert_eq!(lines.next(), Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"));
  assert_eq!(lines.next(), Some("153 255 204 153 255 204 153 255 204 153 255 204 153"));
}

#[test]
fn ppm_file_terminated_by_newline() {
  let c = Canvas::new(5, 3);
  let ppm = c.to_ppm();

  assert_eq!(ppm.chars().last(), Some('\n'));
}
