//!fatrix, for four matrix (or fake matrix if you so chooes).
//!It is a struct for 4xN matricies.
use std::fmt;
use std::fmt::Debug;
use std::ops::{Index, IndexMut, Mul};

type Float = f32;

///A 4xN matrix. Pretty standard. It has that size limitation because I don't need a
///general matrix for anything. The same goes for why this hardcodes the type.
///It is stored transposed because that is easier.
struct Fatrix {
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

///May panic on index out of bounds.
impl Index<[usize; 2]> for Fatrix {
    type Output = Float;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.store[index[1]][index[0]]
    }
}

///May panic on index out of bounds.
impl IndexMut<[usize; 2]> for Fatrix {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Float {
        &mut self.store[index[1]][index[0]]
    }
}

impl Fatrix {
    ///Creates a Fatrix with a certain amount of columns
    pub fn with_size(columns: usize) -> Fatrix {
        Fatrix {
            store: vec![[Float::default(); 4]; columns],
        }
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }
}

///This is in actuality just a 4x4 matrix, but it is stored a bit differently and has the sole
///purpose of being used to modify a Fatrix. Separated from Fatrix as that has a different purpose,
///to store a representation of some space. This matrix is stored more traditionally.
struct Modtrix {
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

///May panic on index out of bounds.
impl Index<[usize; 2]> for Modtrix {
    type Output = Float;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.store[index[0]][index[1]]
    }
}

///May panic on index out of bounds.
impl IndexMut<[usize; 2]> for Modtrix {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Float {
        &mut self.store[index[0]][index[1]]
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
}

impl Mul<Fatrix> for Modtrix {
    type Output = Fatrix;

    ///Multiply a Modtrix to a Fatrix. Really this can be thought of as applying some modifier to
    ///the fatrix and getting that new fatrix back. Note that it does consume the previous Fatrix
    ///but that's ok, we didn't want to use that thing again anyway!
    //TODO: Make this just modify the original Fatrix and maybe move to that struct as just a
    //.mul() function or somehthing
    fn mul(self, rhs: Fatrix) -> Self::Output {
        //make sure the multiplication is actually defined, else panic
        //pretty ugly, but it should get the job done.
        Fatrix {
            store: rhs
                .store
                .into_iter()
                .map(|v| {
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
