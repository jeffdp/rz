mod canvas;
mod color;
mod intersection;
mod matrix;
mod ray;
mod sphere;
mod tuple;

pub use canvas::*;
pub use color::*;
pub use intersection::*;
pub use matrix::*;
pub use ray::*;
pub use sphere::*;
pub use tuple::*;

mod prelude {
    pub use crate::rz::{Canvas, Color, Matrix, Tuple};
}
