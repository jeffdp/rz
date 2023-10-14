#![allow(dead_code)]

mod rz;
use rz::*;

fn main() {
    let mut canvas = Canvas::new(640, 640);
    let wall_size = 7.0;
    let pixel_size = wall_size / canvas.width as f64;
    let half = wall_size / 2.0;

    let mut sphere = Sphere::new();
    sphere.material = Material {
        color: color(1.0, 0.2, 1.0),
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0,
    };

    // sphere.transform = Matrix::translation(0.5, 0.0, 0.0);
    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;

    let light = PointLight {
        position: point(-10.0, 10.0, -10.0),
        intensity: color(1.0, 1.0, 1.0),
    };

    for y in 0..canvas.height - 1 {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas.width - 1 {
            let world_x = -half + pixel_size * x as f64;
            let position = point(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, (position - ray_origin).normalized());
            let hits = sphere.intersect(ray);

            if hits.len() > 0 {
                if let Some(object) = hits[0].object {
                    let point = ray.position(hits[0].t);
                    let normal = object.normal(point);
                    let eye = -ray.direction;
                    let color = sphere.material.lighting(light, position, eye, normal);
                    canvas.write(x, y, color);
                }
            }
        }
    }

    canvas.save("output/test.png")
}
