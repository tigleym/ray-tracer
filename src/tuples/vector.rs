use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Add for Vector {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Vector {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl Div<f64> for Vector {
  type Output = Self;

  fn div(self, scalar: f64) -> Self::Output {
    Vector {
      x: self.x / scalar,
      y: self.y / scalar,
      z: self.z / scalar,
    }
  }
}

impl Mul<f64> for Vector {
  type Output = Self;

  fn mul(self, scalar: f64) -> Self::Output {
    Vector {
      x: self.x * scalar,
      y: self.y * scalar,
      z: self.z * scalar,
    }
  }
}

impl Neg for Vector {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Vector {
      x: 0.0 - self.x,
      y: 0.0 - self.y,
      z: 0.0 - self.z,
    }
  }
}

impl Sub for Vector {
  type Output = Vector;

  fn sub(self, other: Vector) -> Self::Output {
    Vector {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

impl Vector {
  pub fn new() -> Vector {
    Vector { x: 0.0, y: 0.0, z: 0.0 }
  }

  pub fn magnitude(&self) -> f64 {
    (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
  }

  pub fn normalize(&self) -> Vector {
    let m = Self::magnitude(self);
    Vector {
      x: self.x / m,
      y: self.y / m,
      z: self.z / m,
    }
  }
}

pub fn dot(a: &Vector, b: &Vector) -> f64 {
  (a.x * b.x) + (a.y * b.y) + (a.z * b.z)
}

pub fn cross(a: &Vector, b: &Vector) -> Vector {
  Vector {
    x: (a.y * b.z) - (a.z * b.y),
    y: (a.z * b.x) - (a.x * b.z),
    z: (a.x * b.y) - (a.y * b.x),
  }
}

#[test]
fn adding_two_vectors() {
  let v1 = Vector { x: 3.0, y: 2.0, z: 1.0 };
  let v2 = Vector { x: 5.0, y: 6.0, z: 7.0 };
  let v3 = v1 + v2;
  assert_eq!(v3, Vector { x: 8.0, y: 8.0, z: 8.0 });
}

#[test]
fn subtracting_two_vectors() {
  let v1 = Vector { x: 3.0, y: 2.0, z: 1.0 };
  let v2 = Vector { x: 5.0, y: 6.0, z: 7.0 };
  let v3 = v1 - v2;
  assert_eq!(v3, Vector { x: -2.0, y: -4.0, z: -6.0});
}

#[test]
fn subtracting_zero_vector() {
  let v1 = Vector { x: 1.0, y: -2.0, z: 3.0 };
  let zero = Vector::new();
  let v2 = zero - v1;
  assert_eq!(v2, Vector { x: -1.0, y: 2.0, z: -3.0 });
}

#[test]
fn negating_a_vector() {
  let v = Vector { x: 1.0, y: -2.0, z: 3.0 };
  assert_eq!(-v, Vector { x: -1.0, y: 2.0, z: -3.0 });
}

#[test]
fn multiplying_vector_by_scalar() {
  let v = Vector { x: 1.0, y: -2.0, z: 3.0 };
  assert_eq!(v * 3.5, Vector { x: 3.5, y: -7.0, z: 10.5 });
}

#[test]
fn multiplying_vector_by_fraction() {
  let v = Vector { x: 1.0, y: -2.0, z: 3.0 };
  assert_eq!(v * 0.5, Vector { x: 0.5, y: -1.0, z: 1.5 });
}

#[test]
fn dividing_vector_by_scalar() {
  let Vector = Vector { x: 1.0, y: -2.0, z: 3.0 };
  assert_eq!(Vector / 2.0, Vector { x: 0.5, y: -1.0, z: 1.5 });
}

#[test]
fn compute_vector_magnitude_0 () {
  let v = Vector { x: 1.0, y: 0.0, z: 0.0 };
  assert_eq!(v.magnitude(), 1.0_f64);
}


#[test]
fn compute_vector_magnitude_1 () {
  let v = Vector { x: 0.0, y: 1.0, z: 0.0 };
  assert_eq!(v.magnitude(), 1.0);
}

#[test]
fn compute_vector_magnitude_2 () {
  let v = Vector { x: 0.0, y: 0.0, z: 1.0 };
  assert_eq!(v.magnitude(), 1.0);
}

#[test]
fn compute_vector_magnitude_3 () {
  let v = Vector { x: 1.0, y: 2.0, z: 3.0 };
  assert_eq!(v.magnitude(), 14.0_f64.sqrt());
}

#[test]
fn compute_vector_magnitude_4 () {
  let v = Vector { x: -1.0, y: -2.0, z: -3.0 };
  assert_eq!(v.magnitude(), 14.0_f64.sqrt());
}

#[test]
fn normalizing_vector_0() {
  let v = Vector { x: 4.0, y: 0.0, z: 0.0 };
  assert_eq!(v.normalize(), Vector { x: 1.0, y: 0.0, z: 0.0 });
}

#[test]
fn normalizing_vector_1() {
  let v = Vector { x: 1.0, y: 2.0, z: 3.0 };
  let m = v.magnitude();
  assert_eq!(v.normalize(), Vector { x: 1.0 / m, y: 2.0 / m, z: 3.0 / m });
}

#[test]
fn magnitude_normalized_vector() {
  let v = Vector { x: 1.0, y: 2.0, z: 3.0 };
  let norm = v.normalize();
  assert_eq!(norm.magnitude(), 1.0);
}

#[test]
fn dot_product_of_two_vectors() {
  let a = Vector { x: 1.0, y: 2.0, z: 3.0 };
  let b = Vector { x: 2.0, y: 3.0, z: 4.0 };
  assert_eq!(dot(&a, &b), 20.0);
}

#[test]
fn cross_product_of_two_vectors() {
  let a = Vector { x: 1.0, y: 2.0, z: 3.0 };
  let b = Vector { x: 2.0, y: 3.0, z: 4.0 };
  assert_eq!(cross(&a, &b), Vector { x: -1.0, y: 2.0, z: -1.0 });
  assert_eq!(cross(&b, &a), Vector { x: 1.0, y: -2.0, z: 1.0 });
}
