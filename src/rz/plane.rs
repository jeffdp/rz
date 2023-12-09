#![allow(unused_imports)]
use super::intersection::*;
use super::material::*;
use super::matrix::*;
use super::ray::*;
use super::shape::*;
use super::tuple::*;
use std::f64::consts::PI;
const EPSILON: f64 = 0.00001;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Plane {
    pub transform: Matrix<4>,
    pub material: Material,
}

impl From<Plane> for Shape {
    fn from(plane: Plane) -> Self {
        Shape::Plane(plane)
    }
}

impl Plane {
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

    pub fn normal(&self, _p: Tuple) -> Tuple {
        vector(0.0, 1.0, 0.0)
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
        if ray.direction.y.abs() < EPSILON {
            return vec![];
        }

        let t = -ray.origin.y / ray.direction.y;

        vec![Intersection {
            t,
            object: (*self).into(),
        }]
    }
}

#[test]
fn normal_of_plane_is_constant() {
    let p = Plane::default();
    let n1 = p.normal(point(0.0, 0.0, 0.0));
    let n2 = p.normal(point(10.0, 0.0, -10.0));
    let n3 = p.normal(point(-5.0, 0.0, 150.0));

    assert_eq!(n1, vector(0.0, 1.0, 0.0));
    assert_eq!(n2, vector(0.0, 1.0, 0.0));
    assert_eq!(n3, vector(0.0, 1.0, 0.0));
}

#[test]
fn intersect_with_parallel_ray() {
    let obj: Shape = Plane::default().into();
    let ray = Ray::new(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));
    let hits = obj.intersect(ray);

    assert!(hits.is_empty());
}

#[test]
fn intersect_with_coplanar_ray() {
    let obj: Shape = Plane::default().into();
    let ray = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let hits = obj.intersect(ray);

    assert!(hits.is_empty());
}

#[test]
fn instersect_from_above() {
    let obj: Shape = Plane::default().into();
    let ray = Ray::new(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
    let hits = obj.intersect(ray);

    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].t, 1.0);
    assert_eq!(hits[0].object, obj);
}

#[test]
fn intersect_from_below() {
    let obj: Shape = Plane::default().into();
    let ray = Ray::new(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));
    let hits = obj.intersect(ray);

    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].t, 1.0);
    assert_eq!(hits[0].object, obj);
}
