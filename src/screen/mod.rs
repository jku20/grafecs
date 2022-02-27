use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::ops::{Index, IndexMut};

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

///Can panic if the index is too large
///Note that indexing has 0,0 on bottom left.
///The higher the first index, the furthur right,
///the higher the second index, the furthur up.
impl<T: Color> Index<[usize; 2]> for Screen<T> {
    type Output = T;
    fn index(&self, index: [usize; 2]) -> &T {
        &self.grid[index[1]][index[0]]
    }
}

///Can panic if the index is too large
///Note that indexing has 0,0 on bottom left.
///The higher the first index, the furthur right,
///the higher the second index, the furthur up.
impl<T: Color> IndexMut<[usize; 2]> for Screen<T> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut T {
        &mut self.grid[index[1]][index[0]]
    }
}

impl<T: Color> Screen<T> {
    pub fn with_size(width: usize, height: usize) -> Screen<T> {
        Screen {
            grid: vec![vec![T::default(); width]; height],
            width,
            height,
        }
    }

    ///returns an exclusive width
    pub fn width(&self) -> usize {
        self.width
    }

    ///returns an exclusive height
    pub fn height(&self) -> usize {
        self.height
    }

    ///Write contents as ppm to specified file path.
    ///The header writes ascii colors for readability
    pub fn write_ascii_ppm(&self, file: &mut File) -> Result<(), io::Error> {
        let mut file = BufWriter::new(file);
        write!(
            file,
            "P3\n{} {}\n{}\n",
            self.width,
            self.height,
            T::max_val()
        )?;
        for v in self.grid.iter().rev() {
            for c in v {
                write!(file, "{} {} {} ", c.red(), c.green(), c.blue())?;
            }
            writeln!(file)?;
        }
        file.flush()?;
        Ok(())
    }

    ///Write contents as ppm to specified file path.
    ///The header writes in the binary format
    pub fn write_binary_ppm(&self, file: &mut File) -> Result<(), io::Error> {
        let mut file = BufWriter::new(file);
        let max_val = T::max_val();
        write!(
            file,
            "P6\n{} {}\n{}\n",
            self.width,
            self.height,
            T::max_val()
        )?;
        for v in self.grid.iter().rev() {
            for c in v {
                //panic on malformed max_val
                if max_val < c.red() || max_val < c.green() || max_val < c.blue() {
                    panic!("max_val less than red, green, or blue value");
                }
                //256 is the magic number for ppm files
                //this should also mean the below never panics
                if max_val < 256 {
                    file.write_all(&[c.red() as u8, c.green() as u8, c.blue() as u8])?;
                } else {
                    file.write_all(&c.red().to_le_bytes())?;
                    file.write_all(&c.green().to_le_bytes())?;
                    file.write_all(&c.blue().to_le_bytes())?;
                }
            }
        }
        file.flush()?;
        Ok(())
    }
}
