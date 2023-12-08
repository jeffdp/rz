#![allow(unused_imports)]
use super::intersection::*;
use super::material::*;
use super::matrix::*;
use super::ray::*;
use super::shape::*;
use super::tuple::*;
use std::f64::consts::PI;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    pub transform: Matrix<4>,
    pub material: Material,
}

impl From<Sphere> for Shape {
    fn from(sphere: Sphere) -> Self {
        Shape::Sphere(sphere)
    }
}

impl Sphere {
    pub fn new(transform: Matrix<4>, material: Material) -> Self {
        Self {
            transform,
            material,
        }
    }

    pub fn default() -> Self {
        Self {
            transform: Matrix::identity(),
            material: Material::default_material(),
        }
    }

    pub fn normal(&self, p: Tuple) -> Tuple {
        let local_point = self.transform.inverse() * p;
        let local_normal = local_point - point(0.0, 0.0, 0.0);

        local_normal
    }

    pub fn with_transform(&self, transform: Matrix<4>) -> Self {
        Self {
            transform,
            material: self.material,
        }
    }

    pub fn with_material(&self, material: Material) -> Self {
        Self {
            transform: self.transform,
            material,
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
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
                object: (*self).into(),
            },
            Intersection {
                t: t2,
                object: (*self).into(),
            },
        ]
    }
}

#[test]
fn normal_on_sphere_on_x_axis() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let obj: Shape = Sphere::default().into();
    let hits = obj.intersect(r);

    assert_eq!(hits.len(), 2);
    assert_eq!(hits[0].t, 4.0);
    assert_eq!(hits[1].t, 6.0);
}

#[test]
fn ray_originates_inside_a_sphere() {
    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let obj: Shape = Sphere::default().into();
    let hits = obj.intersect(r);

    assert_eq!(hits.len(), 2);
    assert_eq!(hits[0].t, -1.0);
    assert_eq!(hits[1].t, 1.0);
}

#[test]
fn sphere_is_behind_a_ray() {
    let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
    let obj: Shape = Sphere::default().into();
    let hits = obj.intersect(r);

    assert_eq!(hits.len(), 2);
    assert_eq!(hits[0].t, -6.0);
    assert_eq!(hits[1].t, -4.0);
}

#[test]
fn changing_sphere_transform() {
    let m = Matrix::translation(2.0, 3.0, 4.0);
    let obj: Shape = Sphere::default().with_transform(m).into();

    println!("obj: {:?}", obj);

    assert_eq!(obj.transform(), m);
}

#[test]
fn intersecting_scaled_sphere() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let obj: Shape = Sphere::default()
        .with_transform(Matrix::scaling(2.0, 2.0, 2.0))
        .into();
    let hits = obj.intersect(r);

    assert_eq!(hits.len(), 2);
    assert_eq!(hits[0].t, 3.0);
    assert_eq!(hits[1].t, 7.0);
}

#[test]
fn normal_on_sphere() {
    let obj: Shape = Sphere::default().into();
    let n = obj.normal(point(1.0, 0.0, 0.0));

    assert_eq!(n, vector(1.0, 0.0, 0.0));
}

#[test]
fn normal_on_translated_sphere() {
    let obj: Shape = Sphere::default()
        .with_transform(Matrix::translation(0.0, 1.0, 0.0))
        .into();

    let sq2 = (2 as f64).sqrt() / 2.0;
    let n = obj.normal(point(0.0, 1.0 + sq2, -sq2));
    assert_eq!(n, vector(0.0, sq2, -sq2));
}

#[test]
fn normal_on_transformed_sphere() {
    let scale = Matrix::scaling(1.0, 0.5, 1.0);
    let rotation = Matrix::rotation_z(PI / 5.0);
    let obj: Shape = Sphere::default().with_transform(scale * rotation).into();
    let sq2 = (2 as f64).sqrt() / 2.0;
    let n = obj.normal(point(0.0, sq2, -sq2));
    assert_eq!(n, vector(0.0, 0.9701425001453319, -0.24253562503633294));
}
