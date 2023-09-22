use crate::rz::color::*;

pub struct Canvas {
    pub width: usize,
    pub height: usize,

    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::black(); width * height],
        }
    }

    pub fn write(&self, x: usize, y: usize, color: Color) {}
}
