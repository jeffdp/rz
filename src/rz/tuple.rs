use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

pub type F = f64;

pub fn point(x: F, y: F, z: F) -> Tuple {
    Tuple::point(x, y, z)
}

pub fn vector(x: F, y: F, z: F) -> Tuple {
    Tuple::vector(x, y, z)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tuple {
    pub x: F,
    pub y: F,
    pub z: F,
    pub w: F,
}

impl Tuple {
    pub fn point(x: F, y: F, z: F) -> Self {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: 1.0,
        }
    }

    pub fn vector(x: F, y: F, z: F) -> Self {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: 0.0,
        }
    }

    pub fn new(x: F, y: F, z: F, w: F) -> Self {
        Self { x, y, z, w }
    }

    pub fn is_point(&self) -> bool {
        (self.w - 1.0).abs() < std::f64::EPSILON
    }

    pub fn is_vector(&self) -> bool {
        (self.w - 0.0).abs() < std::f64::EPSILON
    }

    pub fn magnitude(&self) -> F {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalized(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn dot(&self, other: Self) -> F {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: Self) -> Self {
        vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Add<Self> for Tuple {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub<Self> for Tuple {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Mul<F> for Tuple {
    type Output = Self;
    fn mul(self, other: F) -> Self::Output {
        Tuple {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Div<F> for Tuple {
    type Output = Self;
    fn div(self, other: F) -> Self::Output {
        Tuple {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Index<usize> for Tuple {
    type Output = F;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &self.w,
        }
    }
}

impl IndexMut<usize> for Tuple {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => &mut self.w,
        }
    }
}

#[test]
fn create_tuples() {
    let pt = Tuple::point(1.0, 2.0, 3.0);
    assert_eq!(pt.x, 1.0);
    assert_eq!(pt.y, 2.0);
    assert_eq!(pt.z, 3.0);
    assert!(pt.is_point());

    let vec = Tuple::vector(1.0, 2.0, 3.0);
    assert_eq!(vec.x, 1.0);
    assert_eq!(vec.y, 2.0);
    assert_eq!(vec.z, 3.0);
    assert!(vec.is_vector());
}

#[test]
fn adding_two_vectors() {
    let a = Tuple::vector(1.0, 2.0, 3.0);
    let b = Tuple::vector(0.0, 1.0, 0.0);
    let sum = Tuple::vector(1.0, 3.0, 3.0);
    assert_eq!(a + b, sum);
    assert!(sum.is_vector());

    let pt = Tuple::point(1.0, 2.0, 3.0);
    let vec = Tuple::vector(0.0, 1.0, 0.0);
    let pt2 = Tuple::point(1.0, 3.0, 3.0);
    assert_eq!(pt + vec, pt2);
    assert!(pt2.is_point());
}

#[test]
fn subtracting_two_points() {
    let p1 = Tuple::point(3.0, 2.0, 1.0);
    let p2 = Tuple::point(5.0, 6.0, 7.0);
    assert_eq!(p1 - p2, Tuple::vector(-2.0, -4.0, -6.0));
}

#[test]
fn subtracting_a_vector_from_a_point() {
    let p = point(3.0, 2.0, 1.0);
    let v = Tuple::vector(5.0, 6.0, 7.0);
    assert_eq!(p - v, Tuple::point(-2.0, -4.0, -6.0));
}

#[test]
fn subtracting_two_vectors() {
    let v1 = vector(3.0, 2.0, 1.0);
    let v2 = vector(5.0, 6.0, 7.0);
    assert_eq!(v1 - v2, vector(-2.0, -4.0, -6.0));
}

#[test]
fn negating_a_tuple() {
    let a = vector(1.0, -2.0, 3.0);
    assert_eq!(-a, vector(-1.0, 2.0, -3.0));
}

#[test]
fn multiplying_a_tuple_by_a_scalar() {
    let a = vector(1.0, -2.0, 3.0);
    assert_eq!(a * 3.5, vector(3.5, -7.0, 10.5));
}

#[test]
fn dividing_a_tuple_by_a_scalar() {
    let a = vector(1.0, -2.0, 3.0);
    assert_eq!(a / 2.0, vector(0.5, -1.0, 1.5));
}

#[test]
fn computing_magnitudes() {
    let v1 = vector(1.0, 0.0, 0.0);
    assert_eq!(v1.magnitude(), 1.0);

    let v2 = vector(0.0, 1.0, 0.0);
    assert_eq!(v2.magnitude(), 1.0);

    let v3 = vector(0.0, 0.0, 1.0);
    assert_eq!(v3.magnitude(), 1.0);

    let v4 = vector(1.0, 2.0, 3.0);
    assert_eq!(v4.magnitude(), 14f64.sqrt());

    let v5 = vector(-1.0, -2.0, -3.0);
    assert_eq!(v5.magnitude(), 14f64.sqrt());
}

#[test]
fn normalizing_a_vector() {
    let v1 = vector(4.0, 0.0, 0.0);
    assert_eq!(v1.normalized(), vector(1.0, 0.0, 0.0));

    let v2 = vector(1.0, 2.0, 3.0);
    let v2n = v2.normalized();
    assert_eq!(v2n.magnitude(), 1.0);
}

#[test]
fn the_dot_product_of_two_tuples() {
    let a = vector(1.0, 2.0, 3.0);
    let b = vector(2.0, 3.0, 4.0);
    assert_eq!(a.dot(b), 20.0);
}

#[test]
fn the_cross_product_of_two_vectors() {
    let a = vector(1.0, 2.0, 3.0);
    let b = vector(2.0, 3.0, 4.0);
    assert_eq!(a.cross(b), vector(-1.0, 2.0, -1.0));
    assert_eq!(b.cross(a), vector(1.0, -2.0, 1.0));
}
