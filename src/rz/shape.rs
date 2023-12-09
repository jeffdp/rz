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
        let local_point = self.transform().inverse() * p;

        let local_normal = match *self {
            Shape::Sphere(sphere) => sphere.normal(local_point),
            Shape::Plane(plane) => plane.normal(local_point),
        };

        let mut world_normal = self.transform().inverse().transposed() * local_normal;
        world_normal.w = 0.0;

        world_normal.normalized()
    }

    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(self.transform().inverse());

        match *self {
            Shape::Sphere(sphere) => sphere.intersect(local_ray),
            Shape::Plane(plane) => plane.intersect(local_ray),
        }
    }
}
