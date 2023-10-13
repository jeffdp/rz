#![allow(dead_code)]

mod rz;
use rz::*;

fn main() {
    let mut canvas = Canvas::new(640, 640);
    let wall_size = 7.0;
    let pixel_size = wall_size / canvas.width as f64;
    let half = wall_size / 2.0;

    let color = Color::new(1.0, 0.0, 0.0);
    let sphere = Sphere::new();
    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;

    for y in 0..canvas.height - 1 {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas.width - 1 {
            let world_x = -half + pixel_size * x as f64;
            let position = point(world_x, world_y, wall_z);

            let r = Ray::new(ray_origin, (position - ray_origin).normalized());
            let hits = sphere.intersect(r);

            if hits.len() > 0 {
                canvas.write(x, y, color);
            }
        }
    }

    canvas.save("output/test.png")
}
