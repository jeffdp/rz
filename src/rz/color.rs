use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub fn color(r: f32, g: f32, b: f32) -> Color {
    Color::new(r, g, b)
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r: r, g: g, b: b }
    }

    pub fn black() -> Self {
        Color::new(0.0, 0.0, 0.0)
    }
}

impl Add<Self> for Color {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Sub<Self> for Color {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;
    fn mul(self, other: f32) -> Self::Output {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl Mul<Self> for Color {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Div<f32> for Color {
    type Output = Self;
    fn div(self, other: f32) -> Self::Output {
        Color {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other,
        }
    }
}

impl Neg for Color {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Color {
            r: -self.r,
            g: -self.g,
            b: -self.b,
        }
    }
}

#[test]
fn create_color() {
    let c = color(-0.5, 0.4, 1.7);
    assert_eq!(c.r, -0.5);
    assert_eq!(c.g, 0.4);
    assert_eq!(c.b, 1.7);
}

// #[test]
fn adding_colors() {
    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);
    assert_eq!(c1 + c2, color(1.6, 0.7, 1.0));
}

fn subracting_colors() {
    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);
    assert_eq!(c1 - c2, color(0.2, 0.5, 0.5));
}

fn multiplying_a_color_by_a_scalar() {
    let c = color(0.2, 0.3, 0.4);
    assert_eq!(c * 2.0, color(0.4, 0.6, 0.8));
}

fn multiplying_colors() {
    let c1 = color(1.0, 0.2, 0.4);
    let c2 = color(0.9, 1.0, 0.1);
    assert_eq!(c1 * c2, color(0.9, 0.2, 0.04));
}
