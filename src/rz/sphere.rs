use super::ray::*;
use super::tuple::*;

use std::ops::Index;

pub struct Hit {
    pub intersections: Vec<f64>,
    pub count: i8,
    pub object: Option<Sphere>,
}

impl Index<usize> for Hit {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {}
    }

    pub fn intersect(&self, ray: Ray) -> Hit {
        let sphere_to_ray = ray.origin - point(0.0, 0.0, 0.0);

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let det = b * b - 4.0 * a * c;
        let t1 = (-b - det.sqrt()) / (2.0 * a);
        let t2 = (-b + det.sqrt()) / (2.0 * a);

        if det < 0.0 {
            return Hit {
                intersections: vec![],
                count: 0,
                object: None,
            };
        }

        Hit {
            intersections: vec![t1, t2],
            count: 2,
            object: Some(*self),
        }
    }
}

#[test]
fn normal_on_sphere_on_x_axis() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let hit = s.intersect(r);

    assert_eq!(hit.count, 2);
    assert_eq!(hit[0], 4.0);
    assert_eq!(hit[1], 6.0);
}

#[test]
fn ray_originates_inside_a_sphere() {
    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let hit = s.intersect(r);

    assert_eq!(hit.count, 2);
    assert_eq!(hit[0], -1.0);
    assert_eq!(hit[1], 1.0);
}

#[test]
fn sphere_is_behind_a_ray() {
    let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let hit = s.intersect(r);

    assert_eq!(hit.count, 2);
    assert_eq!(hit[0], -6.0);
    assert_eq!(hit[1], -4.0);
}
