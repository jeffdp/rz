#![allow(dead_code)]

mod rz;
use rz::*;
use std::f64::consts::PI;

fn main() {
    let floor = Plane {
        transform: Matrix::identity(),
        material: Material {
            color: color(0.0, 0.0, 1.0),
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 20.0,
        },
    }
    .into();

    let left = Sphere {
        transform: Matrix::identity()
            .scale(0.33, 0.33, 0.33)
            .translate(-1.5, 0.33, -0.75),
        material: Material {
            color: color(1.0, 0.8, 0.1),
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 20.0,
        },
    }
    .into();

    let middle = Sphere {
        transform: Matrix::identity().translate(-0.5, 1.0, 0.5),
        material: Material {
            color: color(0.1, 1.0, 0.5),
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 20.0,
        },
    }
    .into();

    let right = Sphere {
        transform: Matrix::identity()
            .scale(0.5, 0.5, 0.5)
            .translate(1.5, 0.5, -0.5),
        material: Material {
            color: color(1.0, 0.3, 0.1),
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 5.0,
        },
    }
    .into();

    let world = World {
        light: PointLight {
            position: point(-10.0, 10.0, -10.0),
            intensity: Color::white(),
        },
        objects: vec![floor, left, middle, right],
    };

    let mut camera = Camera::new(800, 400, PI / 3.0);
    camera.transform = Matrix::view(
        point(0.0, 1.5, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0),
    );

    camera.render(&world).save("output/world.png")
}
