use std::f64::EPSILON;
use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
  pub r: f64,
  pub g: f64,
  pub b: f64,
}

impl Add for Color {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Color {
      r: self.r + other.r,
      g: self.g + other.g,
      b: self.b + other.b,
    }
  }
}

impl Sub for Color {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Color {
      r: self.r - other.r,
      g: self.g - other.g,
      b: self.b - other.b,
    }
  }
}

impl Mul for Color {
  type Output = Self;

  fn mul(self, other: Self) -> Self::Output {
    Color {
      r: self.r * other.r,
      g: self.g * other.g,
      b: self.b * other.b,
    }
  }
}

impl Mul<f64> for Color {
  type Output = Self;

  fn mul(self, scalar: f64) -> Self::Output {
    Color {
      r: self.r * scalar,
      g: self.g * scalar,
      b: self.b * scalar,
    }
  }
}

fn compare_colors(c: Color, expected: Color) {
  assert!((c.r -  expected.r).abs() < EPSILON);
  assert!((c.g -  expected.g).abs() < EPSILON);
  assert!((c.b -  expected.b).abs() < EPSILON);
}

#[test]
fn create_color() {
  let c = Color { r: -0.5, g: 0.4, b: 1.7 };
  assert_eq!(c.r, -0.5);
  assert_eq!(c.g, 0.4);
  assert_eq!(c.b, 1.7);
}

#[test]
fn adding_colors() {
  let c1 = Color { r: 0.9, g: 0.6, b: 0.75 };
  let c2 = Color { r: 0.7, g: 0.1, b: 0.25 };
  compare_colors(c1 + c2, Color { r: 1.6, g: 0.7, b: 1.0 });
}

#[test]
fn subtracting_colors() {
  let c1 = Color { r: 0.9, g: 0.6, b: 0.75 };
  let c2 = Color { r: 0.7, g: 0.1, b: 0.25 };
  compare_colors(c1 - c2, Color { r: 0.2, g: 0.5, b: 0.5 });
}

#[test]
fn multiplying_color_by_scalar() {
  let c = Color { r: 0.2, g: 0.3, b: 0.4 };
  compare_colors(c * 2.0, Color { r: 0.4, g: 0.6, b: 0.8 });
}

#[test]
fn multiplying_color_by_color() {
  // hadamard product
  let c1 = Color { r: 1.0, g: 0.2, b: 0.4 };
  let c2 = Color { r: 0.9, g: 1.0, b: 0.1 };
  compare_colors(c1 * c2, Color { r: 0.9, g: 0.2, b: 0.04 });
}
