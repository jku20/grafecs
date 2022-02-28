use std::fmt::Debug;

///A trait implemented by structs storing one pixel's color is stored.
///As this project is meant only to write to the ppm format, colors
///should be `u16`s as that is the maximum color value of that format.
pub trait Color: Debug + Default + Clone + Copy {
    fn red(&self) -> u16;
    fn blue(&self) -> u16;
    fn green(&self) -> u16;
    ///The maximum value for any element of the RGB triple.
    ///Could panic if malformed
    fn max_val() -> u16;
}

///Color implemented in the common 8 bit RGB triple format.
#[derive(Debug, Clone, Copy, Default)]
pub struct RGB8Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color for RGB8Color {
    fn red(&self) -> u16 {
        self.red.into()
    }
    fn green(&self) -> u16 {
        self.green.into()
    }
    fn blue(&self) -> u16 {
        self.blue.into()
    }
    fn max_val() -> u16 {
        u8::MAX.into()
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

///Color implemented in the common 16 bit RGB triple format.
#[derive(Debug, Clone, Copy, Default)]
pub struct RGB16Color {
    red: u16,
    green: u16,
    blue: u16,
}

impl Color for RGB16Color {
    fn red(&self) -> u16 {
        self.red
    }
    fn green(&self) -> u16 {
        self.green
    }
    fn blue(&self) -> u16 {
        self.blue
    }
    fn max_val() -> u16 {
        u16::MAX
    }
}

impl From<(u16, u16, u16)> for RGB16Color {
    fn from(c: (u16, u16, u16)) -> Self {
        Self {
            red: c.0,
            green: c.1,
            blue: c.2,
        }
    }
}
