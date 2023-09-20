#![allow(dead_code)]

mod rz;
mod prelude {
    pub use crate::rz::{point, vector, Tuple};
}
use rz::*;

fn main() {
    let origin = point(0.0, 0.0, 0.0);

    print!("Origin: [{}, {}, {}]", origin.x, origin.y, origin.z);
}
