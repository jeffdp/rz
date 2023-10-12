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
fn translating_a_ray() {
    let r = Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
}
