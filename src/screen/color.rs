use std::fmt::Debug;
use std::ops::{Add, AddAssign, Sub};

use crate::space::Float;

use rand::random;

///an integer type big enough to handle the color values of all the colors implemented
type Uint = u8;

///A trait implemented by structs storing one pixel's color is stored.
///As this project is meant only to write to the ppm format, colors
///should be `u16`s as that is the maximum color value of that format.
pub trait Color: Debug + Default + Clone + Copy + Add + AddAssign + Sub {
    fn red(&self) -> Uint;
    fn blue(&self) -> Uint;
    fn green(&self) -> Uint;
    fn random_color() -> Self;
    ///The maximum value for any element of the RGB triple.
    ///Could panic if malformed
    fn max_val() -> Uint;
    ///multiplies the color by a 3 tuple
    ///define it how you want but make it reasonable
    ///generally larger numbers in that tuple should mean a brighter color
    ///for rgb this means reds, greens, and blues, are brighter
    fn mult(&self, _: (Float, Float, Float)) -> Self;
}

///Color implemented in the common 8 bit RGB triple format.
#[derive(Debug, Clone, Copy, Default)]
pub struct RGB8Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color for RGB8Color {
    fn red(&self) -> Uint {
        self.red.into()
    }
    fn green(&self) -> Uint {
        self.green.into()
    }
    fn blue(&self) -> Uint {
        self.blue.into()
    }
    fn random_color() -> Self {
        Self {
            red: random(),
            green: random(),
            blue: random(),
        }
    }
    fn max_val() -> Uint {
        u8::MAX.into()
    }
    fn mult(&self, c: (Float, Float, Float)) -> Self {
        let (kr, kg, kb) = c;
        let nr = kr * self.red() as Float;
        let ng = kg * self.green() as Float;
        let nb = kb * self.blue() as Float;

        let red = nr.min(Self::max_val() as Float).max(0.0) as Uint;
        let green = ng.min(Self::max_val() as Float).max(0.0) as Uint;
        let blue = nb.min(Self::max_val() as Float).max(0.0) as Uint;

        Self {
            red,
            green,
            blue,
        }
    }
}

impl From<(u8, u8, u8)> for RGB8Color {
    fn from(tup: (u8, u8, u8)) -> Self {
        Self {
            red: tup.0,
            green: tup.1,
            blue: tup.2,
        }
    }
}

impl AddAssign<Self> for RGB8Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = rhs + *self;
    }
}

impl Add<Self> for RGB8Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let red = self.red().checked_add(rhs.red())
            .unwrap_or(Self::max_val())
            .min(Self::max_val())
            as u8;
        let green = self.green().checked_add(rhs.green())
            .unwrap_or(Self::max_val())
            .min(Self::max_val())
            as u8;
        let blue = self.blue().checked_add(rhs.blue())
            .unwrap_or(Self::max_val())
            .min(Self::max_val())
            as u8;

        Self {
            red,
            green,
            blue,
        }
    }
}

impl Sub<Self> for RGB8Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let red = self.red().checked_sub(rhs.red())
            .unwrap_or(0)
            as u8;
        let green = self.green().checked_add(rhs.green())
            .unwrap_or(0)
            as u8;
        let blue = self.blue().checked_add(rhs.blue())
            .unwrap_or(0)
            as u8;

        Self {
            red,
            green,
            blue,
        }
    }
}
