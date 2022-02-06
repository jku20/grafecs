use std::fs::File;
use std::io;
use std::io::Write;

pub mod color;

use color::Color;

///Screen containting a grid of colors.
///This is the final destination before the grid is written to a file.
///Kind of just a wrapper for a color vector.
#[derive(Debug, Clone)]
pub struct Screen<T: Color> {
    grid: Vec<Vec<T>>,

    ///Height of screen, bottom of grid is 0.
    width: usize,
    ///Width of screen, left of grid is 0.
    height: usize,
}

impl<T: Color> Screen<T> {
    pub fn with_size(width: usize, height: usize) -> Screen<T> {
        Screen {
            grid: vec![vec![T::default(); width]; height],
            width,
            height,
        }
    }
    ///Write contents as ppm to specified file path.
    ///The header writes ascii colors (P3) for readability
    pub fn write_ppm(&self, file: &mut File) -> Result<(), io::Error> {
        write!(file, "P3\n{} {}\n{}\n", self.width, self.height, T::max_val())?;
        for v in &self.grid {
            for c in v {
                write!(file, "{} {} {} ", c.red(), c.blue(), c.green())?;
            }
            writeln!(file, "")?;
        }
        Ok(())
    }
}
