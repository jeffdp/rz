use super::intersection::*;
use super::material::*;
use super::matrix::*;
use super::plane::*;
use super::ray::*;
use super::sphere::*;
use super::tuple::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
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
            Shape::Plane(plane) => plane.material,
        }
    }

    fn transform(&self) -> Matrix<4> {
        match *self {
            Shape::Sphere(sphere) => sphere.transform,
            Shape::Plane(plane) => plane.transform,
        }
    }

    fn normal(&self, p: Tuple) -> Tuple {
        match *self {
            Shape::Sphere(sphere) => sphere.normal(p),
            Shape::Plane(plane) => plane.normal(p),
        }
    }

    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        match *self {
            Shape::Sphere(sphere) => sphere.intersect(ray),
            Shape::Plane(plane) => plane.intersect(ray),
        }
    }
}
