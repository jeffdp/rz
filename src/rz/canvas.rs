use crate::rz::color::*;
use image::{ImageBuffer, Rgb, RgbImage};

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
            pixels: vec![Color::black(); width * height as usize],
        }
    }

    pub fn write(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[x + y * self.width] = color;
    }

    pub fn save(&self, file: &str) {
        let mut image: RgbImage = ImageBuffer::new(self.width as u32, self.height as u32);

        for y in 0..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;
                let color = self.pixels[index];
                let pixel = Rgb([
                    (color.r * 255.0) as u8,
                    (color.g * 255.0) as u8,
                    (color.b * 255.0) as u8,
                ]);
                image.put_pixel(x as u32, y as u32, pixel)
            }
        }

        image.save(file).unwrap();
    }
}
