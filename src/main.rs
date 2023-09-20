#![allow(dead_code)]

mod rz;
mod prelude {
    pub use crate::rz::Tuple;
}
use rz::*;

fn main() {
    let origin = Tuple::point(0.0, 0.0, 0.0);

    print!("Origin: [{}, {}, {}]", origin.x, origin.y, origin.z);
}
