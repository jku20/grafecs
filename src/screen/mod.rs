//!stuff in here deals with the actual raster image which will be exported, acting as kind of an
//!intermediate between the final image file and the Space with all the shapes in it
mod color;
mod screen;

pub use color::{Color, RGB8Color};
pub use screen::Screen;
