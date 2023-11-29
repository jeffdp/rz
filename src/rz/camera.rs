#![allow(unused_imports)]
use super::canvas::*;
use super::color::*;
use super::matrix::*;
use super::ray::*;
use super::tuple::*;
use super::world::*;
use std::f64::consts::PI;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    half_width: f64,
    half_height: f64,
    pub transform: Matrix<4>,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;
        let half_width: f64;
        let half_height: f64;
        if hsize > vsize {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        Camera {
            hsize,
            vsize,
            field_of_view,
            half_width,
            half_height,
            transform: Matrix::identity(),
        }
    }

    pub fn pixel_size(&self) -> f64 {
        self.half_width * 2.0 / self.hsize as f64
    }

    pub fn ray_for_pixel(self, px: &usize, py: &usize) -> Ray {
        // Calculate the offset from the edge of the canvas to the pixel's center.
        let x_offset = (*px as f64 + 0.5) * self.pixel_size();
        let y_offset = (*py as f64 + 0.5) * self.pixel_size();

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let pixel = self.transform.inverse() * point(world_x, world_y, -1.0);
        let origin = self.transform.inverse() * point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalized();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize - 1 {
            for x in 0..self.hsize - 1 {
                let ray = self.ray_for_pixel(&x, &y);
                let color = world.color(&ray);
                image.write(x, y, color);
            }
        }

        image
    }
}

#[test]
fn pixel_size_for_horizontal_canvas() {
    let c = Camera::new(125, 200, PI / 2.0);
    assert!((c.pixel_size() - 0.01).abs() < 0.00001);
}

#[test]
fn ray_through_center_of_canvas() {
    let c = Camera::new(201, 101, PI / 2.0);
    let ray = c.ray_for_pixel(&0, &0);

    assert_eq!(ray.origin, point(0.0, 0.0, 0.0));
    assert_eq!(ray.direction, vector(0.66519, 0.33259, -0.66851));
}

#[test]
fn ray_when_camera_is_transformed() {
    let mut c = Camera::new(201, 101, PI / 2.0);
    c.transform = Matrix::rotation_y(PI / 4.0) * Matrix::translation(0.0, -2.0, 5.0);
    let ray = c.ray_for_pixel(&100, &50);

    assert_eq!(ray.origin, point(0.0, 2.0, -5.0));
    assert_eq!(
        ray.direction,
        vector(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0)
    );
}

#[test]
fn render_world_with_camera() {
    let w = World::default();
    let mut c = Camera::new(11, 11, PI / 2.0);
    let from = point(0.0, 0.0, -5.0);
    let to = point(0.0, 0.0, 0.0);
    let up = vector(0.0, 1.0, 0.0);
    c.transform = Matrix::view(from, to, up);
    let image = c.render(&w);

    assert_eq!(image.pixel_at(5, 5), color(0.38066, 0.47583, 0.2855));
}
