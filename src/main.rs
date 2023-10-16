#![allow(dead_code)]

mod rz;
use rz::*;

fn main() {
    let world = World {
        light: PointLight {
            position: point(-10.0, 10.0, -10.0),
            intensity: Color::white(),
        },
        objects: vec![
            Sphere {
                transform: Matrix::identity()
                    .scale(0.5, 0.5, 0.5)
                    .translate(-0.5, 0.0, 1.0),
                material: Material {
                    color: color(0.8, 0.2, -0.5),
                    ambient: 0.1,
                    diffuse: 0.7,
                    specular: 0.2,
                    shininess: 20.0,
                },
            },
            Sphere {
                transform: Matrix::identity()
                    .scale(0.5, 0.5, 0.5)
                    .translate(0.3, 0.0, 0.2),
                material: Material {
                    color: color(0.2, 0.5, 0.8),
                    ambient: 0.1,
                    diffuse: 0.7,
                    specular: 0.2,
                    shininess: 5.0,
                },
            },
        ],
    };

    let mut canvas = Canvas::new(640, 640);
    let wall_size = 7.0;
    let pixel_size = wall_size / canvas.width as f64;
    let half = wall_size / 2.0;

    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;

    for y in 0..canvas.height - 1 {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas.width - 1 {
            let world_x = -half + pixel_size * x as f64;
            let position = point(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, (position - ray_origin).normalized());
            let color = world.color(ray);
            canvas.write(x, y, color);
        }
    }

    canvas.save("output/test.png")
}
