#![allow(unused_imports)]
use super::color::*;
use super::intersection::*;
use super::lights::*;
use super::material::*;
use super::matrix::*;
use super::ray::*;
use super::sphere::*;
use super::tuple::*;

#[derive(Debug, Clone, PartialEq)]
pub struct World {
    pub light: PointLight,
    pub objects: Vec<Sphere>,
}

impl World {
    pub fn new() -> World {
        World {
            light: PointLight {
                position: point(-10.0, 10.0, -10.0),
                intensity: Color::white(),
            },
            objects: vec![],
        }
    }

    pub fn default() -> World {
        let mut s1 = Sphere::default();
        s1.material = Material {
            color: color(0.8, 1.0, 0.6),
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0,
        };

        let mut s2 = Sphere::default();
        s2.transform = Matrix::scaling(0.5, 0.5, 0.5);
        s2.material = Material {
            color: color(0.0, 0.0, 0.0),
            ambient: 0.0,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 0.0,
        };

        World {
            light: PointLight {
                position: point(-10.0, 10.0, -10.0),
                intensity: Color::white(),
            },
            objects: vec![s1, s2],
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut hits: Vec<Intersection> = self
            .objects
            .iter()
            .flat_map(|obj| obj.intersect(ray))
            .collect();

        hits.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        hits
    }

    pub fn shade_hit(&self, comps: IntersectionInfo) -> Color {
        comps
            .object
            .material
            .lighting(self.light, comps.point, comps.eye, comps.normal)
    }

    pub fn color(&self, ray: &Ray) -> Color {
        let hits = self.intersect(*ray);
        if hits.is_empty() {
            return Color::black();
        }

        let comps = IntersectionInfo::prepare_computations(hits[0], *ray);
        self.shade_hit(comps)
    }
}

pub struct IntersectionInfo {
    t: f64,
    object: Sphere,
    point: Tuple,
    eye: Tuple,
    normal: Tuple,
    inside: bool,
}

impl IntersectionInfo {
    pub fn prepare_computations(hit: Intersection, ray: Ray) -> IntersectionInfo {
        let point = ray.position(hit.t);
        let eye = -ray.direction;
        let mut normal = hit.object.normal(point);
        let inside: bool;
        if normal.dot(eye) < 0.0 {
            inside = true;
            normal = -normal;
        } else {
            inside = false;
        }

        IntersectionInfo {
            t: hit.t,
            object: hit.object,
            point,
            eye,
            normal,
            inside,
        }
    }
}

#[test]
fn intersect_default_world() {
    let world = World::default();
    let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let hits = world.intersect(ray);

    assert_eq!(hits.len(), 4);
    assert_eq!(hits[0].t, 4.0);
    assert_eq!(hits[1].t, 4.5);
    assert_eq!(hits[2].t, 5.5);
    assert_eq!(hits[3].t, 6.0);
}

#[test]
fn precompute_intersections() {
    let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::default();
    let hit = Intersection::new(4.0, s);
    let comps = IntersectionInfo::prepare_computations(hit, ray);

    assert_eq!(comps.t, hit.t);
    assert_eq!(comps.object, hit.object);
    assert_eq!(comps.point, point(0.0, 0.0, -1.0));
    assert_eq!(comps.eye, vector(0.0, 0.0, -1.0));
    assert_eq!(comps.normal, vector(0.0, 0.0, -1.0));
}

#[test]
fn intersection_on_the_outside() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let shape = Sphere::default();
    let hit = Intersection::new(4.0, shape);
    let comps = IntersectionInfo::prepare_computations(hit, r);

    assert_eq!(comps.inside, false);
}

#[test]
fn intersection_on_the_inside() {
    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let shape = Sphere::default();
    let hit = Intersection::new(1.0, shape);
    let comps = IntersectionInfo::prepare_computations(hit, r);

    assert_eq!(comps.point, point(0.0, 0.0, 1.0));
    assert_eq!(comps.eye, vector(0.0, 0.0, -1.0));
    assert_eq!(comps.inside, true);
    assert_eq!(comps.normal, vector(0.0, 0.0, -1.0));
}

#[test]
fn color_of_ray_miss() {
    let world = World::default();
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
    let c = world.color(&r);

    assert_eq!(c, color(0.0, 0.0, 0.0));
}

#[test]
fn color_of_ray_hit() {
    let world = World::default();
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let c = world.color(&r);

    assert_eq!(c, color(0.38066, 0.47583, 0.2855));
}

#[test]
fn color_of_hit_behind_ray() {
    let mut s1 = Sphere::default();
    s1.material = Material {
        color: color(0.8, 1.0, 0.6),
        ambient: 0.0,
        diffuse: 0.7,
        specular: 0.2,
        shininess: 0.0,
    };

    let mut s2 = Sphere::default();
    s2.transform = Matrix::scaling(0.5, 0.5, 0.5);
    s2.material = Material {
        color: color(0.0, 0.0, 0.0),
        ambient: 0.0,
        diffuse: 0.7,
        specular: 0.2,
        shininess: 0.0,
    };

    let world = World {
        light: PointLight {
            position: point(-10.0, 10.0, -10.0),
            intensity: Color::white(),
        },
        objects: vec![s1, s2],
    };

    let r = Ray::new(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
    let c = world.color(&r);

    assert_eq!(c, world.objects[1].material.color);
}
