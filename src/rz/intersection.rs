#![allow(unused_imports)]
use super::ray::*;
use super::shape::*;
use super::sphere::*;
use super::tuple::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub object: Shape,
}

impl Intersection {
    pub fn new(t: f64, s: Shape) -> Intersection {
        Intersection { t, object: s }
    }
}

pub fn hit(hits: Vec<Intersection>) -> Option<Intersection> {
    // hits.reduce(|a, b| if a.t < b.t { a } else { b })
    hits.iter()
        .filter(|a| a.t >= 0.0)
        .fold(None, |acc, &b| match acc {
            None => Some(b),
            Some(a) if b.t < a.t => Some(b),
            _ => acc,
        })
}

#[test]
fn all_positive_t() {
    let s = Sphere::default().into();
    let i1 = Intersection::new(1.0, s);
    let i2 = Intersection::new(2.0, s);
    let hit = hit(vec![i1, i2]);

    assert_eq!(hit, Some(i1));
}

#[test]
fn some_negative_t() {
    let s = Sphere::default().into();
    let i1 = Intersection::new(-1.0, s);
    let i2 = Intersection::new(1.0, s);
    let hit = hit(vec![i1, i2]);

    assert_eq!(hit, Some(i2));
}

#[test]
fn all_negative_t() {
    let s = Sphere::default().into();
    let i1 = Intersection::new(-2.0, s);
    let i2 = Intersection::new(-1.0, s);
    let hit = hit(vec![i1, i2]);

    assert_eq!(hit, None);
}

#[test]
fn randome_order_t() {
    let s = Sphere::default().into();
    let i1 = Intersection::new(5.0, s);
    let i2 = Intersection::new(7.0, s);
    let i3 = Intersection::new(-3.0, s);
    let i4 = Intersection::new(2.0, s);
    let hit = hit(vec![i1, i2, i3, i4]);

    assert_eq!(hit, Some(i4));
}
