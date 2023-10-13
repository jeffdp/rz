#![allow(unused_imports)]
use super::matrix::*;
use super::tuple::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Ray {
            origin,
            direction: direction.normalized(),
        }
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }

    pub fn transform(&self, m: Matrix<4>) -> Ray {
        Ray {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }
}

#[test]
fn computing_a_point_from_a_distance() {
    let r = Ray::new(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));

    assert_eq!(r.position(0.0), point(2.0, 3.0, 4.0));
    assert_eq!(r.position(1.0), point(3.0, 3.0, 4.0));
    assert_eq!(r.position(-1.0), point(1.0, 3.0, 4.0));
    assert_eq!(r.position(2.5), point(4.5, 3.0, 4.0));
}

#[test]
fn translating_ray() {
    let r = Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
    let m = Matrix::translation(3.0, 4.0, 5.0);
    let r2 = r.transform(m);

    assert_eq!(r2.origin, point(4.0, 6.0, 8.0));
    assert_eq!(r2.direction, vector(0.0, 1.0, 0.0));
}

#[test]
fn scaling_ray() {
    let r = Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
    let m = Matrix::scaling(2.0, 3.0, 4.0);
    let r2 = r.transform(m);

    assert_eq!(r2.origin, point(2.0, 6.0, 12.0));
    assert_eq!(r2.direction, vector(0.0, 3.0, 0.0));
}
