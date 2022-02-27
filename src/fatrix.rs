//!fatrix, for four matrix (or fake matrix if you so chooes).
//!It is a struct for 4xN matricies.
use std::fmt;
use std::ops::{Index, IndexMut, Mul};
use std::fmt::Debug;

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

impl Index<[usize; 2]> for Fatrix {
    type Output = Float;
    fn index(&self, index: [usize; 2]) -> &Float {
        &self.store[index[1]][index[0]]
    }
}

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
///to store a representation of some space.
struct Modtrix {
    store: [[4; Float]; Float],
}


/*
impl Mul for Modtrix {
    type Output = Fatrix;

    ///Note this is only defined for proper fatrix lengths, as with normal matrix multiplication.
    fn mul(self, rhs: Fatrix) {
        //make sure the multiplication is actually defined, else panic
        assert_eq!(4 ,self.len());

        let res = Fatrix::with_size(rhs.len());
        let res = self.iter()

    }
}
*/


