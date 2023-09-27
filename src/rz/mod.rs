mod canvas;
mod color;
mod matrix;
mod tuple;

pub use canvas::*;
pub use color::*;
pub use matrix::*;
pub use tuple::*;

mod prelude {
    pub use crate::rz::{Canvas, Color, Matrix, Tuple};
}
