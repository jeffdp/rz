#![allow(dead_code)]

mod rz;
use rz::*;

fn main() {
    let origin = point(0.0, 0.0, 0.0);
    let canvas = Canvas::new(32, 32);
    canvas.write(0, 0, color(1.0, 0.0, 0.0));

    print!("Origin: [{}, {}, {}]", origin.x, origin.y, origin.z);
}
