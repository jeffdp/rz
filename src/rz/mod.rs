mod canvas;
mod color;
mod tuple;

pub use canvas::*;
pub use color::*;
pub use tuple::*;

mod prelude {
    pub use crate::rz::{Canvas, Color, Tuple};
}
