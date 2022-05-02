///A space where you can add lines and triangles and lights
///you write its stuff to a screen
use std::fmt::Debug;

use super::{Float, Modtrix, Point};
use crate::gmath;
use crate::screen::{Color, Screen};

#[derive(Clone, Debug)]
pub struct Space {
    lin_space: Vec<[Float; 4]>,
    tri_space: Vec<[Float; 4]>,
}

impl Space {
    ///creates a space with a certain starting capacity in both the line and trianlge space
    ///both of these starting capacities are the same
    pub fn with_capacity(columns: usize) -> Self {
        Self {
            lin_space: Vec::with_capacity(columns),
            tri_space: Vec::with_capacity(columns),
        }
    }

    ///creates a Space with zero starting capacity
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    ///adds an line to the Space
    pub fn add_line(&mut self, p: Point, q: Point) {
        self.lin_space.push([p.0, p.1, p.2, 1.0]);
        self.lin_space.push([q.0, q.1, q.2, 1.0]);
    }

    ///adds a triangle to the Space
    ///note that p, q, and r, should be put in counter clockwise order. If you are looking at a
    ///clock which is like a triangle the p would be at 9:00, the q at 7:00, and the r at 4:00
    pub fn add_tri(&mut self, p: Point, q: Point, r: Point) {
        self.tri_space.push([p.0, p.1, p.2, 1.0]);
        self.tri_space.push([q.0, q.1, q.2, 1.0]);
        self.tri_space.push([r.0, r.1, r.2, 1.0]);
    }

    ///apply tranformation stored in a Modtrix
    pub fn apply(&mut self, transform: &Modtrix) {
        let mult = |x: &Vec<[Float; 4]>| {
            x.iter()
                .map(|&v| {
                    [
                        v[0] * transform.store[0][0]
                            + v[1] * transform.store[0][1]
                            + v[2] * transform.store[0][2]
                            + v[3] * transform.store[0][3],
                        v[0] * transform.store[1][0]
                            + v[1] * transform.store[1][1]
                            + v[2] * transform.store[1][2]
                            + v[3] * transform.store[1][3],
                        v[0] * transform.store[2][0]
                            + v[1] * transform.store[2][1]
                            + v[2] * transform.store[2][2]
                            + v[3] * transform.store[2][3],
                        v[0] * transform.store[3][0]
                            + v[1] * transform.store[3][1]
                            + v[2] * transform.store[3][2]
                            + v[3] * transform.store[3][3],
                    ]
                })
                .collect()
        };
        self.lin_space = mult(&self.lin_space);
        self.tri_space = mult(&self.tri_space);
    }

    ///draws the lines currently in the space to a given screen
    pub fn draw_space<T: Color>(space: &Space, s: &mut Screen<T>) {
        space.lin_space.windows(2).step_by(2).for_each(|w| {
            let p1 = (w[0][0], w[0][1], w[0][2]);
            let p2 = (w[1][0], w[1][1], w[1][2]);
            s.draw_line(p1, p2, T::random_color());
        });
        space.tri_space.windows(3).step_by(3).for_each(|w| {
            let view = (0.0, 0.0, 1.0);
            let p1 = (w[0][0], w[0][1], w[0][2]);
            let p2 = (w[1][0], w[1][1], w[1][2]);
            let p3 = (w[2][0], w[2][1], w[2][2]);
            let snorm = gmath::norm(p1, p2, p3);

            if gmath::dot(snorm, view) > 0.0 {
                s.draw_tri(p1, p2, p3, T::random_color());
            }
        });
    }
}
