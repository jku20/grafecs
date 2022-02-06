///A trait implemented by structs storing one pixel's color is stored.
///As this project is meant only to write to the ppm format, colors
///should be `u16`s as that is the maximum color value of that format.
pub trait Color: Default + Clone {
    fn red(&self) -> u16;
    fn blue(&self) -> u16;
    fn green(&self) -> u16;
    ///The maximum value for any element of the RGB triple.
    fn max_val() -> u16;
}

///Color implemented in the common RGB triple format.
#[derive(Debug, Clone, Copy, Default)]
pub struct RGB8Color {
    red: u8,
    blue: u8,
    green: u8,
}

impl Color for RGB8Color {
    fn red(&self) -> u16 {
        self.red.into()
    }
    fn blue(&self) -> u16 {
        self.blue.into()
    }
    fn green(&self) -> u16 {
        self.green.into()
    }
    fn max_val() -> u16 {
        255
    }
}
