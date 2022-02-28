use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::ops::{Index, IndexMut};

pub mod color;

use color::Color;

type Point = (i32, i32);

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

    ///Plots a point p to the screen.
    ///Points which are off the screen will be ignored.
    ///Should never panic except for really weird cases I don't understand due to the order of the
    ///checks.
    pub fn plot(&mut self, p: Point, color: T) {
        if p.0 >= 0 && p.1 >= 0 && (p.0 as usize) < self.width() && (p.1 as usize) < self.height() {
            self[[p.0 as usize, p.1 as usize]] = color;
        }
    }

    ///Draws a line of pixels to the screen using Bresenham's line algorithm
    ///or a similar algorithm described [here](https://zingl.github.io/Bresenham.pdf).
    ///Pixels not visable on the screen (i.e. (-1, 4)) will just be ignored.
    ///The pixels are inclusive meaning both p1 and p2 may be drawn
    pub fn draw_line(&mut self, p1: Point, p2: Point, color: T) {
        //algorithm by Alois Zingl (https://zingl.github.io/Bresenham.pdf)
        //used because it is super clean
        let dx = (p2.0 - p1.0).abs();
        let dy = -(p2.1 - p1.1).abs();
        let sx = (p2.0 - p1.0).signum();
        let sy = (p2.1 - p1.1).signum();

        let mut e = dx + dy;
        let (mut x, mut y) = (p1.0, p1.1);
        loop {
            self.plot((x, y), color);
            if x == p2.0 && y == p2.1 {
                break;
            }
            let et = e * 2;
            if et >= dy {
                x += sx;
                e += dy;
            }
            if et <= dx {
                y += sy;
                e += dx;
            }
        }
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

    /*
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
    */
}
