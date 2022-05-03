//!misc math functions useful for graphics programming

use crate::space::{Float, Point};

pub fn dot(v1: Point, v2: Point) -> Float {
    v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
}

pub fn norm(p1: Point, p2: Point, p3: Point) -> Point {
    let a = (p1.0 - p2.0, p1.1 - p2.1, p1.2 - p2.2);
    let b = (p1.0 - p3.0, p1.1 - p3.1, p1.2 - p3.2);

    (
        a.1 * b.2 - a.2 * b.1,
        a.2 * b.0 - a.0 * b.2,
        a.0 * b.1 - a.1 * b.0,
    )
}

pub fn normalize(p: Point) -> Point {
    let rat = 1.0 / (p.0 * p.0 + p.1 * p.1 + p.2 * p.2).sqrt();
    (p.0 * rat, p.1 * rat, p.2 * rat)
}

///not true vector math but these are used enough that these utility functions are worth while
pub fn add(p1: Point, p2: Point) -> Point {
    (p1.0 + p2.0, p1.1 + p2.1, p1.2 + p2.2)
}

pub fn sub(p1: Point, p2: Point) -> Point {
    (p1.0 - p2.0, p1.1 - p2.1, p1.2 - p2.2)
}

pub fn scale(f: Float, p: Point) -> Point {
    (f * p.0, f * p.1, f * p.2)
}
