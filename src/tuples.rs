use std::ops::{Add, Div, Mul, Neg, Sub};

const VECTOR_W: f64 = 0.0;
const POINT_W: f64 = 1.0;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Tuple {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub w: f64,
}

impl Tuple {
  fn create_vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: VECTOR_W }
  }

  fn create_point(x: f64, y: f64, z: f64) -> Tuple {
     Tuple { x, y, z, w: POINT_W }
  }
}

impl Add for Tuple {
  type Output = Self;

  fn add(self, other: Tuple) -> Tuple {
    Tuple {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
      w: self.w + other.w,
    }
  }
}

impl Div<f64> for Tuple {
  type Output = Self;

  fn div(self, scalar: f64) -> Self::Output {
    Tuple {
      x: self.x / scalar,
      y: self.y / scalar,
      z: self.z / scalar,
      w: self.w / scalar,
    }
  }
}

impl Mul<f64> for Tuple {
  type Output = Self;

  fn mul(self, scalar: f64) -> Self::Output {
    Tuple {
      x: self.x * scalar,
      y: self.y * scalar,
      z: self.z * scalar,
      w: self.w * scalar,
    }
  }
}

impl Neg for Tuple {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Tuple {
      x: 0.0 - self.x,
      y: 0.0 - self.y,
      z: 0.0 - self.z,
      w: 0.0 - self.w,
    }
  }
}

impl Sub for Tuple {
  type Output = Tuple;

  fn sub(self, other: Tuple) -> Tuple {
    Tuple {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
      w: self.w - other.w,
    }
  }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
  Tuple::create_point(x, y, z)
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
  Tuple::create_vector(x, y, z)
}

pub fn magnitude(v: &Tuple) -> f64 {
  (v.x.powi(2) + v.y.powi(2) + v.z.powi(2) + v.w.powi(2)).sqrt()
}

pub fn normalize(v: &Tuple) -> Tuple {
  let m = magnitude(v);
  Tuple {
    x: v.x / m,
    y: v.y / m,
    z: v.z / m,
    w: v.w / m,
  }
}

pub fn dot(a: &Tuple, b: &Tuple) -> f64 {
  (a.x * b.x) + (a.y * b.y) + (a.z * b.z) + (a.w * b.w)
}

pub fn cross(a: &Tuple, b: &Tuple) -> Tuple {
  Tuple {
    x: (a.y * b.z) - (a.z * b.y),
    y: (a.z * b.x) - (a.x * b.z),
    z: (a.x * b.y) - (a.y * b.x),
    w: 0.0
  }
}

#[test]
fn is_point_test() {
  let p = point(4.0, -4.0, 3.0);
  assert_eq!(p, Tuple { x: 4.0, y: -4.0, z: 3.0, w: 1.0 }, "point() creates tuple with w=1.0");
}

#[test]
fn is_vector_test() {
  let v = vector(4.0, -4.0, 3.0);
  assert_eq!(v, Tuple { x: 4.0, y: -4.0, z: 3.0, w: 0.0 }, "vector() creates tuple with w=0.0");
}

#[test]
fn adding_vector_to_point() {
  let p = point(3.0, -2.0, 5.0);
  let v = vector(-2.0, 3.0, 1.0);
  let p2 = p + v;
  assert_eq!(p2, point(1.0, 1.0, 6.0), "Adding a vector to a point results in a point.");
}

#[test]
fn subtracting_two_points() {
  let p1 = point(3.0, 2.0, 1.0);
  let p2 = point(5.0, 6.0, 7.0);
  let v = p1 - p2;
  assert_eq!(v, vector(-2.0, -4.0, -6.0), "Subtracting two points results in a vector.");
}

#[test]
fn subtracting_vector_from_point() {
  let p = point(3.0, 2.0, 1.0);
  let v = vector(5.0, 6.0, 7.0);
  let p2 = p - v;
  assert_eq!(p2, point(-2.0, -4.0, -6.0), "Subtracting vector from point results in a point");
}

#[test]
fn subtracting_two_vectors() {
  let v1 = vector(3.0, 2.0, 1.0);
  let v2 = vector(5.0, 6.0, 7.0);
  let v3 = v1 - v2;
  assert_eq!(v3, vector(-2.0, -4.0, -6.0), "Subtracting two vector results in a vector");
}

#[test]
fn subtracting_zero_vector() {
  let v1 = vector(1.0, -2.0, 3.0);
  let zero = vector(0.0 , 0.0, 0.0);
  let v2 = zero - v1;
  assert_eq!(v2, vector(-1.0, 2.0, -3.0),
            "Subtracting a vector from zero vector negates values.");
}

#[test]
fn negating_a_tuple() {
  let tuple = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
  assert_eq!(-tuple, Tuple { x: -1.0, y: 2.0, z: -3.0, w: 4.0 },
            "Negating a tuple should negate each value in it.");
}

#[test]
fn multiplying_tuple_by_scalar() {
  let tuple = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
  assert_eq!(tuple * 3.5, Tuple { x: 3.5, y: -7.0, z: 10.5, w: -14.0 },
            "Multiplying a tuple by a scalar.");
}

#[test]
fn multiplying_tuple_by_fraction() {
  let tuple = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
  assert_eq!(tuple * 0.5, Tuple { x: 0.5, y: -1.0, z: 1.5, w: -2.0 },
    "Multiplying a tuple by a fraction.");
}

#[test]
fn dividing_tuple_by_scalar() {
  let tuple = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
  assert_eq!(tuple / 2.0, Tuple { x: 0.5, y: -1.0, z: 1.5, w: -2.0 },
            "Dividing a tuple by a scalar.");
}

#[test]
fn compute_vector_magnitude_0 () {
  let v = vector(1.0, 0.0, 0.0);
  assert_eq!(magnitude(&v), 1.0_f64, "Magnitude is correct.");
}


#[test]
fn compute_vector_magnitude_1 () {
  let v = vector(0.0, 1.0, 0.0);
  assert_eq!(magnitude(&v), 1.0, "Magnitude is correct.");
}

#[test]
fn compute_vector_magnitude_2 () {
  let v = vector(0.0, 0.0, 1.0);
  assert_eq!(magnitude(&v), 1.0, "Magnitude is correct.");
}

#[test]
fn compute_vector_magnitude_3 () {
  let v = vector(1.0, 2.0, 3.0);
  assert_eq!(magnitude(&v), 14.0_f64.sqrt(), "Magnitude is correct.");
}

#[test]
fn compute_vector_magnitude_4 () {
  let v = vector(-1.0, -2.0, -3.0);
  assert_eq!(magnitude(&v), 14.0_f64.sqrt(), "Magnitude is correct.");
}

#[test]
fn normalizing_vector_0() {
  let v = vector(4.0, 0.0, 0.0);
  assert_eq!(normalize(&v), vector(1.0, 0.0, 0.0), "Normalizing a vector.");
}

#[test]
fn normalizing_vector_1() {
  let v = vector(1.0, 2.0, 3.0);
  let m = magnitude(&v);
  assert_eq!(normalize(&v), vector(1.0 / m, 2.0 / m, 3.0 / m), "Normalizing a vector.");
}

#[test]
fn magnitude_normalized_vector() {
  let v = vector(1.0, 2.0, 3.0);
  let norm = normalize(&v);
  assert_eq!(magnitude(&norm), 1.0, "Magnitude of normalized vector should be 1.0.");
}

#[test]
fn dot_product_of_two_tuples() {
  let a = vector(1.0, 2.0, 3.0);
  let b = vector(2.0, 3.0, 4.0);
  assert_eq!(dot(&a, &b), 20.0, "Dot product should be 20.");
}

#[test]
fn cross_product_of_two_vectors() {
  let a = vector(1.0, 2.0, 3.0);
  let b = vector(2.0, 3.0, 4.0);
  assert_eq!(cross(&a, &b), vector(-1.0, 2.0, -1.0), "Cross product should be correct.");
  assert_eq!(cross(&b, &a), vector(1.0, -2.0, 1.0), "Cross product should be correct.");
}
