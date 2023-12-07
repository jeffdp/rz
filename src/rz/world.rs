#![allow(unused_imports)]
use super::color::*;
use super::intersection::*;
use super::lights::*;
use super::material::*;
use super::matrix::*;
use super::ray::*;
use super::shape::*;
use super::sphere::*;
use super::tuple::*;

const EPSILON: f64 = 0.00001;

#[derive(Debug, PartialEq)]
pub struct World {
    pub light: PointLight,
    pub objects: Vec<Shape>,
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
        let material = Material {
            color: color(0.8, 1.0, 0.6),
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0,
        };
        let s1: Shape = Sphere::default().with_material(material).into();

        let material = Material {
            color: color(0.0, 0.0, 0.0),
            ambient: 0.0,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 0.0,
        };
        let s2: Shape = Sphere::default()
            .with_transform(Matrix::scaling(0.5, 0.5, 0.5))
            .with_material(material)
            .into();

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
        let in_shadow = self.is_shadowed(&comps.over_point);

        comps.object.material().lighting(
            self.light,
            comps.point,
            comps.eye,
            comps.normal,
            in_shadow,
        )
    }

    pub fn color(&self, ray: &Ray) -> Color {
        let hits = self.intersect(*ray);
        if hits.is_empty() {
            return Color::black();
        }

        let comps = IntersectionInfo::prepare_computations(hits[0], *ray);
        self.shade_hit(comps)
    }

    pub fn is_shadowed(&self, point: &Tuple) -> bool {
        let v = self.light.position - *point;
        let distance = v.magnitude();
        let direction = v.normalized();
        let ray = Ray::new(*point, direction);
        let intersections = self.intersect(ray);

        if let Some(hit) = hit(intersections) {
            hit.t < distance
        } else {
            false
        }
    }
}

pub struct IntersectionInfo {
    t: f64,
    object: Shape,
    point: Tuple,
    eye: Tuple,
    normal: Tuple,
    inside: bool,
    over_point: Tuple,
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
            over_point: point + normal * EPSILON,
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
    let s = Sphere::default().into();
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
    let shape = Sphere::default().into();
    let hit = Intersection::new(4.0, shape);
    let comps = IntersectionInfo::prepare_computations(hit, r);

    assert_eq!(comps.inside, false);
}

#[test]
fn intersection_on_the_inside() {
    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let shape = Sphere::default().into();
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
    let material = Material {
        color: color(0.8, 1.0, 0.6),
        ambient: 0.0,
        diffuse: 0.7,
        specular: 0.2,
        shininess: 0.0,
    };
    let s1: Shape = Sphere::default().with_material(material).into();

    let material = Material {
        color: color(0.0, 0.0, 0.0),
        ambient: 0.0,
        diffuse: 0.7,
        specular: 0.2,
        shininess: 0.0,
    };
    let s2: Shape = Sphere::default()
        .with_transform(Matrix::scaling(0.5, 0.5, 0.5))
        .with_material(material)
        .into();

    let world = World {
        light: PointLight {
            position: point(-10.0, 10.0, -10.0),
            intensity: Color::white(),
        },
        objects: vec![s1, s2],
    };

    let r = Ray::new(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
    let c = world.color(&r);

    assert_eq!(c, world.objects[1].material().color);
}

#[test]
fn no_shadow_when_nothing_is_collinear_with_point_and_light() {
    let world = World::default();
    let p = point(0.0, 10.0, 0.0);

    assert!(world.is_shadowed(&p) == false);
}

#[test]
fn shadow_when_object_between_point_and_light() {
    let world = World::default();
    let p = point(10.0, -10.0, 10.0);

    assert!(world.is_shadowed(&p) == true);
}

#[test]
fn no_shadow_when_object_behind_light() {
    let world = World::default();
    let p = point(-20.0, 20.0, -20.0);

    assert!(world.is_shadowed(&p) == false);
}

#[test]
fn no_shadow_when_object_behind_point() {
    let world = World::default();
    let p = point(-2.0, 2.0, -2.0);

    assert!(world.is_shadowed(&p) == false);
}

#[test]
fn intersection_in_shadow() {
    let mut world = World::new();
    world.light = PointLight {
        position: point(0.0, 0.0, -10.0),
        intensity: color(1.0, 1.0, 1.0),
    };

    let s1: Shape = Sphere {
        transform: Matrix::identity(),
        material: Material::default_material(),
    }
    .into();

    let s2: Shape = Sphere {
        transform: Matrix::translation(0.0, 0.0, 10.0),
        material: Material::default_material(),
    }
    .into();

    world.objects = vec![s1, s2];

    let ray = Ray {
        origin: point(0.0, 0.0, 5.0),
        direction: vector(0.0, 0.0, 1.0),
    };
    let intersection = Intersection { t: 4.0, object: s2 };
    let comps = IntersectionInfo::prepare_computations(intersection, ray);
    let c = world.shade_hit(comps);

    assert_eq!(c, color(0.1, 0.1, 0.1));
}

#[test]
fn hit_should_offset_point() {
    let ray = Ray {
        origin: point(0.0, 0.0, -5.0),
        direction: vector(0.0, 0.0, 1.0),
    };

    let shape: Shape = Sphere {
        transform: Matrix::translation(0.0, 0.0, 1.0),
        material: Material::default_material(),
    }
    .into();

    let intersection = Intersection {
        t: 5.0,
        object: shape,
    };

    let comps = IntersectionInfo::prepare_computations(intersection, ray);

    assert!(comps.over_point.z < -EPSILON / 2.0)
}
