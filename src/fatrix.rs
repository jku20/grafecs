//!a collection of structs dealing with spaces to put shapes in
//!a lot of the stuff in here are just wrappers for 4xN matricies.

use crate::gmath;
use crate::screen::{color::Color, Screen};

use std::fmt::{self, Debug};

//when Float is updated, make sure to update the below three lines as well
pub type Float = f32;

///The point is stored (x, y, z)
pub type Point = (Float, Float, Float);

///This is in actuality just a 4x4 matrix, but it is stored a bit differently and has the sole
///purpose of being used to modify a Fatrix. Separated from Fatrix as that has a different purpose,
///to store a representation of some space. This matrix is stored more traditionally.
#[derive(Clone)]
pub struct Modtrix {
    store: [[Float; 4]; 4],
}

impl Debug for Modtrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..4 {
            for j in 0..4 {
                write!(f, "{:?} ", self.store[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Modtrix {
    pub const IDENT: Self = Modtrix {
        store: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };
    
    pub fn multr(lhs: &mut Modtrix, rhs: &Modtrix) {
        //make sure the multiplication is actually defined, else panic
        //Note that the modtrix length is 4
        //FIXME: currently doesn't work
        let mut res = [[0.0; 4]; 4];
        for (i, row) in res.iter_mut().enumerate() {
            for (j, num) in row.iter_mut().enumerate() {
                for k in 0..4 {
                    *num += lhs.store[i][k] * rhs.store[k][j];
                }
            }
        }
        lhs.store = res;
    }
}

impl From<[[Float; 4]; 4]> for Modtrix {
    fn from(store: [[Float; 4]; 4]) -> Self {
        Self { store }
    }
}

///creates translation matrix with given x, y, z values to transform by
#[macro_export]
macro_rules! move_matrix {
    ( $x:expr, $y:expr, $z:expr ) => {
        Modtrix::from([
            [1.0, 0.0, 0.0, $x],
            [0.0, 1.0, 0.0, $y],
            [0.0, 0.0, 1.0, $z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    };
}

///creates dilation (scale) matrix given how much each to scale on the x, y, or z, axis
#[macro_export]
macro_rules! scale_matrix {
    ( $x:expr, $y:expr, $z:expr ) => {
        Modtrix::from([
            [$x, 0.0, 0.0, 0.0],
            [0.0, $y, 0.0, 0.0],
            [0.0, 0.0, $z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    };
}

///creates rotation matrix around the z axis given given an angle of rotation in degrees rotating
///counter clockwise
#[macro_export]
macro_rules! rotz_matrix {
    ( $t:expr ) => {{
        let d = $t.to_radians();
        let sd = d.sin();
        let cd = d.cos();
        Modtrix::from([
            [cd, -sd, 0.0, 0.0],
            [sd, cd, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }};
}

///creates rotation matrix around the x axis given given an angle of rotation in degrees rotating
///counter clockwise
#[macro_export]
macro_rules! rotx_matrix {
    ( $t:expr ) => {{
        let d = $t.to_radians();
        let sd = d.sin();
        let cd = d.cos();
        Modtrix::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cd, -sd, 0.0],
            [0.0, sd, cd, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }};
}

///creates rotation matrix around the y axis given given an angle of rotation in degrees rotating
///counter clockwise
#[macro_export]
macro_rules! roty_matrix {
    ( $t:expr ) => {{
        let d = $t.to_radians();
        let sd = d.sin();
        let cd = d.cos();
        Modtrix::from([
            [cd, 0.0, sd, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sd, 0.0, cd, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }};
}

///A space where you can add lines and triangles
///you write its stuff to a screen
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
    pub fn draw_space<T: Color>(space: &Space, color: T, s: &mut Screen<T>) {
        space.lin_space.windows(2).step_by(2).for_each(|w| {
            let p1 = (w[0][0] as i32, w[0][1] as i32);
            let p2 = (w[1][0] as i32, w[1][1] as i32);
            s.draw_line(p1, p2, color);
        });
        space.tri_space.windows(3).step_by(3).for_each(|w| {
            let view = (0.0, 0.0, 1.0);
            let snorm = gmath::norm(
                (w[0][0], w[0][1], w[0][2]),
                (w[1][0], w[1][1], w[1][2]),
                (w[2][0], w[2][1], w[2][2]),
            );

            if gmath::dot(snorm, view) > 0.0 {
                let p1 = (w[0][0] as i32, w[0][1] as i32);
                let p2 = (w[1][0] as i32, w[1][1] as i32);
                let p3 = (w[2][0] as i32, w[2][1] as i32);

                s.draw_line(p1, p2, color);
                s.draw_line(p2, p3, color);
                s.draw_line(p3, p1, color);
            }
        });
    }
}
