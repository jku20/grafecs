//!fatrix, for four matrix (or fake matrix if you so chooes).
//!It is a struct for 4xN matricies.

use crate::screen::{color::Color, Screen};

use std::fmt::{self, Debug};
use std::ops::Mul;

//when Float is updated, make sure to update the below three lines as well
pub type Float = f32;

///The point is stored (x, y, z)
type Point = (Float, Float, Float);

///A 4xN matrix. Pretty standard. It has that size limitation because I don't need a
///general matrix for anything. The same goes for why this hardcodes the type.
///It is stored transposed because that is easier.
#[derive(Clone)]
pub struct Fatrix {
    store: Vec<[Float; 4]>,
}

impl Debug for Fatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..4 {
            for v in &self.store {
                write!(f, "{:?} ", v[i])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Default for Fatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl Fatrix {
    ///Creates a Fatrix with a certain amount of columns
    pub fn with_size(columns: usize) -> Fatrix {
        Fatrix {
            store: vec![[Float::default(); 4]; columns],
        }
    }

    ///Creates an empty Fatrix
    pub fn new() -> Fatrix {
        Self::with_size(0)
    }

    ///Reserves a certain amount of space for possibly better performance.
    pub fn reserve(&mut self, r: usize) {
        self.store.reserve(r);
    }

    fn add_point(&mut self, p: Point) {
        self.store.push([p.0, p.1, p.2, 1.0]);
    }

    pub fn add_edge(&mut self, p: Point, q: Point) {
        self.add_point(p);
        self.add_point(q);
    }

    pub fn apply(&mut self, transform: &Modtrix) {
        let new_mat = transform * self;
        self.store = new_mat.store;
    }

    pub fn screen<T: Color>(&self, color: T, width: usize, height: usize) -> Screen<T> {
        self.store.windows(2).step_by(2).fold(
            Screen::<T>::with_size(width, height),
            |mut acc, w| {
                let p1 = (w[0][0] as i32, w[0][1] as i32);
                let p2 = (w[1][0] as i32, w[1][1] as i32);
                acc.draw_line(p1, p2, color);
                acc
            },
        )
    }
}

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
    pub fn ident(&mut self) {
        self.store = Self::IDENT.store;
    }
    pub fn mult(lhs: &Modtrix, rhs: &mut Modtrix) {
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
        rhs.store = res;
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

impl Mul<&Fatrix> for &Modtrix {
    type Output = Fatrix;

    ///Multiply a Modtrix to a Fatrix. Really this can be thought of as applying some modifier to
    ///the fatrix and getting that new fatrix back. Note that it does consume the previous Fatrix
    ///but that's ok, we didn't want to use that thing again anyway!
    //TODO: Make this just modify the original Fatrix and maybe move to that struct as just a
    //.mul() function or somehthing
    fn mul(self, rhs: &Fatrix) -> Self::Output {
        //make sure the multiplication is actually defined, else panic
        //pretty ugly, but it should get the job done.
        Fatrix {
            store: rhs
                .store
                .iter()
                .map(|&v| {
                    [
                        v[0] * self.store[0][0]
                            + v[1] * self.store[0][1]
                            + v[2] * self.store[0][2]
                            + v[3] * self.store[0][3],
                        v[0] * self.store[1][0]
                            + v[1] * self.store[1][1]
                            + v[2] * self.store[1][2]
                            + v[3] * self.store[1][3],
                        v[0] * self.store[2][0]
                            + v[1] * self.store[2][1]
                            + v[2] * self.store[2][2]
                            + v[3] * self.store[2][3],
                        v[0] * self.store[3][0]
                            + v[1] * self.store[3][1]
                            + v[2] * self.store[3][2]
                            + v[3] * self.store[3][3],
                    ]
                })
                .collect(),
        }
    }
}
