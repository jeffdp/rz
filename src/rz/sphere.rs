#![allow(unused_imports)]
use super::intersection::*;
use super::material::*;
use super::matrix::*;
use super::ray::*;
use super::tuple::*;
use std::f64::consts::PI;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    pub transform: Matrix<4>,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transform: Matrix::identity(),
            material: Material::default_material(),
        }
    }

    pub fn normal(self, p: Tuple) -> Tuple {
        let local_point = self.transform.inverse() * p;
        let local_normal = local_point - point(0.0, 0.0, 0.0);
        let mut world_normal = self.transform.inverse().transposed() * local_normal;
        world_normal.w = 0.0;

        world_normal.normalized()
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let ray = ray.transform(self.transform.inverse());
        let sphere_to_ray = ray.origin - point(0.0, 0.0, 0.0);

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let det = b * b - 4.0 * a * c;
        let t1 = (-b - det.sqrt()) / (2.0 * a);
        let t2 = (-b + det.sqrt()) / (2.0 * a);

        if det < 0.0 {
            return vec![];
        }

        vec![
            Intersection {
                t: t1,
                object: Some(*self),
            },
            Intersection {
                t: t2,
                object: Some(*self),
            },
        ]
    }
}

#[test]
fn normal_on_sphere_on_x_axis() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let hits = s.intersect(r);

    assert_eq!(hits.len(), 2);
    assert_eq!(hits[0].t, 4.0);
    assert_eq!(hits[1].t, 6.0);
}

#[test]
fn ray_originates_inside_a_sphere() {
    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let hits = s.intersect(r);

    assert_eq!(hits.len(), 2);
    assert_eq!(hits[0].t, -1.0);
    assert_eq!(hits[1].t, 1.0);
}

#[test]
fn sphere_is_behind_a_ray() {
    let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let hits = s.intersect(r);

    assert_eq!(hits.len(), 2);
    assert_eq!(hits[0].t, -6.0);
    assert_eq!(hits[1].t, -4.0);
}

#[test]
fn changing_sphere_transform() {
    let mut s = Sphere::new();
    let m = Matrix::translation(2.0, 3.0, 4.0);
    s.transform = m;

    assert_eq!(s.transform, m);
}

#[test]
fn intersecting_scaled_sphere() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let mut s = Sphere::new();
    s.transform = Matrix::scaling(2.0, 2.0, 2.0);
    let hits = s.intersect(r);

    assert_eq!(hits.len(), 2);
    assert_eq!(hits[0].t, 3.0);
    assert_eq!(hits[1].t, 7.0);
}

#[test]
fn normal_on_sphere() {
    let s = Sphere::new();
    let n = s.normal(point(1.0, 0.0, 0.0));

    assert_eq!(n, vector(1.0, 0.0, 0.0));
}

#[test]
fn normal_on_translated_sphere() {
    let mut s = Sphere::new();
    s.transform = Matrix::translation(0.0, 1.0, 0.0);

    let sq2 = (2 as f64).sqrt() / 2.0;
    let n = s.normal(point(0.0, 1.0 + sq2, -sq2));
    assert_eq!(n, vector(0.0, sq2, -sq2));
}

#[test]
fn normal_on_transformed_sphere() {
    let mut s = Sphere::new();
    let scale = Matrix::scaling(1.0, 0.5, 1.0);
    let rotation = Matrix::rotation_z(PI / 5.0);
    s.transform = scale * rotation;
    let sq2 = (2 as f64).sqrt() / 2.0;
    let n = s.normal(point(0.0, sq2, -sq2));
    assert_eq!(n, vector(0.0, 0.9701425001453319, -0.24253562503633294));
}
