#![allow(dead_code)]

mod rz;
use rz::*;

fn main() {
    let origin = point(0.0, 0.0, 0.0);
    let canvas = Canvas::new(640, 640);

    print!("Origin: [{}, {}, {}]", origin.x, origin.y, origin.z);

    canvas.save("output/test.png")
}
