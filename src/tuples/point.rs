use std::ops::{Add, Neg, Sub};
use crate::tuples::Vector;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Add<Vector> for Point {
  type Output = Self;

  fn add(self, other: Vector) -> Self::Output {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl Neg for Point {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Point {
      x: 0.0 - self.x,
      y: 0.0 - self.y,
      z: 0.0 - self.z,
    }
  }
}

impl Sub<Point> for Point {
  type Output = Vector;

  fn sub(self, other: Point) -> Vector {
    Vector {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

impl Sub<Vector> for Point {
  type Output = Point;

  fn sub(self, other: Vector) -> Self::Output {
    Point {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

#[test]
fn adding_vector_to_point() {
  let p = Point { x: 3.0, y: -2.0, z: 5.0 };
  let v = Vector { x: -2.0, y: 3.0, z: 1.0 };
  let p2 = p + v;
  assert_eq!(p2, Point { x: 1.0, y: 1.0, z: 6.0 });
}

#[test]
fn subtracting_two_points() {
  let p1 = Point { x: 3.0, y: 2.0, z: 1.0 };
  let p2 = Point { x: 5.0, y: 6.0, z: 7.0 };
  let v = p1 - p2;
  assert_eq!(v, Vector { x: -2.0, y: -4.0, z: -6.0 });
}

#[test]
fn subtracting_vector_from_point() {
  let p = Point { x: 3.0, y: 2.0, z: 1.0 };
  let v = Vector { x: 5.0, y: 6.0, z: 7.0 };
  let p2 = p - v;
  assert_eq!(p2, Point { x: -2.0, y: -4.0, z: -6.0 });
}

#[test]
fn negating_a_point() {
  let v = Point { x: 1.0, y: -2.0, z: 3.0 };
  assert_eq!(-v, Point { x: -1.0, y: 2.0, z: -3.0 });
}
