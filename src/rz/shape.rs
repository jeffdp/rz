use super::intersection::*;
use super::material::*;
use super::matrix::*;
use super::ray::*;
use super::sphere::*;
use super::tuple::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
}

impl From<Sphere> for Shape {
    fn from(sphere: Sphere) -> Self {
        Shape::Sphere(sphere)
    }
}

pub trait Intersectable {
    fn material(&self) -> Material;
    fn transform(&self) -> Matrix<4>;
    fn normal(&self, p: Tuple) -> Tuple;
    fn intersect(&self, ray: Ray) -> Vec<Intersection>;
}

impl Intersectable for Shape {
    fn material(&self) -> Material {
        match *self {
            Shape::Sphere(sphere) => sphere.material,
        }
    }

    fn transform(&self) -> Matrix<4> {
        match *self {
            Shape::Sphere(sphere) => sphere.transform,
        }
    }

    fn normal(&self, p: Tuple) -> Tuple {
        match *self {
            Shape::Sphere(sphere) => sphere.normal(p),
        }
    }

    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        match *self {
            Shape::Sphere(sphere) => sphere.intersect(ray),
        }
    }
}
