//!misc math functions useful for graphics programming

use crate::fatrix::{Float, Point};

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
