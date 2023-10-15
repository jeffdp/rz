#![allow(unused_imports)]

use super::color::*;
use super::lights::*;
use super::tuple::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new(
        color: Color,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
    ) -> Material {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn lighting(self, light: PointLight, point: Tuple, eye: Tuple, normal: Tuple) -> Color {
        let effective_color = self.color * light.intensity;

        // Find the direction to the light source
        let light_vector = (light.position - point).normalized();

        // Compute the ambient contribution
        let ambient = effective_color * self.ambient;

        let diffuse: Color;
        let specular: Color;
        let light_dot_normal = light_vector.dot(normal);
        // Cosine of the angle between light and normal
        if light_dot_normal < 0.0 {
            // Light is on the other side of the surface
            diffuse = Color::black();
            specular = Color::black();
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflect = -light_vector.reflected(normal);
            let reflect_dot_eye = reflect.dot(eye);

            if reflect_dot_eye <= 0.0 {
                specular = Color::black();
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }

    pub fn default_material() -> Material {
        Material::new(Color::new(1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0)
    }
}

#[test]
fn sphere_with_default_material() {
    let eye = vector(0.0, 0.0, -1.0);
    let normal = vector(0.0, 0.0, -1.0);
    let light = PointLight {
        position: point(0.0, 0.0, -10.0),
        intensity: color(1.0, 1.0, 1.0),
    };
    let result = Material::default_material().lighting(light, point(0.0, 0.0, 0.0), eye, normal);

    assert_eq!(result, Color::new(1.9, 1.9, 1.9));
}

#[test]
fn lighting_with_eye_between() {
    let s22 = 2.0_f64.sqrt() / 2.0;
    let eye = vector(0.0, s22, -s22);
    let normal = vector(0.0, 0.0, -1.0);
    let light = PointLight {
        position: point(0.0, 0.0, -10.0),
        intensity: color(1.0, 1.0, 1.0),
    };
    let result = Material::default_material().lighting(light, point(0.0, 0.0, 0.0), eye, normal);

    assert_eq!(result, Color::new(1.0, 1.0, 1.0));
}

#[test]
fn lighting_with_eye_opposite() {
    let eye = vector(0.0, 0.0, -1.0);
    let normal = vector(0.0, 0.0, -1.0);
    let light = PointLight {
        position: point(0.0, 10.0, -10.0),
        intensity: color(1.0, 1.0, 1.0),
    };
    let result = Material::default_material().lighting(light, point(0.0, 0.0, 0.0), eye, normal);

    assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
}

#[test]
fn lighting_with_eye_in_path() {
    let s22 = 2.0_f64.sqrt() / 2.0;
    let eye = vector(0.0, -s22, -s22);
    let normal = vector(0.0, 0.0, -1.0);
    let light = PointLight {
        position: point(0.0, 10.0, -10.0),
        intensity: color(1.0, 1.0, 1.0),
    };
    let result = Material::default_material().lighting(light, point(0.0, 0.0, 0.0), eye, normal);

    assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
}

#[test]
fn lighting_with_light_behind() {
    let eye = vector(0.0, 0.0, -1.0);
    let normal = vector(0.0, 0.0, -1.0);
    let light = PointLight {
        position: point(0.0, 0.0, 10.0),
        intensity: color(1.0, 1.0, 1.0),
    };
    let result = Material::default_material().lighting(light, point(0.0, 0.0, 0.0), eye, normal);

    assert_eq!(result, Color::new(0.1, 0.1, 0.1));
}
