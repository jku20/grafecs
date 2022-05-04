use super::Float;
///This is in actuality just a 4x4 matrix, but it is stored a bit differently and has the purpose
///of being used to transform a space.
use std::fmt::{self, Debug};

#[derive(Clone)]
pub struct Modtrix {
    pub store: [[Float; 4]; 4],
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

pub use {move_matrix, rotx_matrix, roty_matrix, rotz_matrix, scale_matrix};
