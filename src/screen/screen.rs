use super::Color;
///the z axis is always quanitized for a computer
///this constatnt represents how many units one quantum of space is
///the larger the number the more inpercise, but also possibly nicer looking (becaues maybe less
///pixels with similar z values fighting)
use std::fs::File;
use std::io::{self, BufWriter, Write};

use crate::space::{Float, Point};

const Z_RESOLUTION: Float = 0.00;

///Screen containting a grid of colors.
///This is the final destination before the grid is written to a file.
///Kind of just a wrapper for a color vector.
#[derive(Debug, Clone)]
pub struct Screen<T: Color> {
    //remember, the grids are stored transposed
    grid: Vec<Vec<T>>,
    zbuffer: Vec<Vec<Float>>,

    ///Height of screen, bottom of grid is 0.
    width: usize,
    ///Width of screen, left of grid is 0.
    height: usize,
}

impl<T: Color> Screen<T> {
    pub fn with_size(width: usize, height: usize) -> Screen<T> {
        Screen {
            grid: vec![vec![T::default(); width]; height],
            zbuffer: vec![vec![Float::NEG_INFINITY; width]; height],
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
    ///clears current screen to default color
    pub fn clear(&mut self) {
        self.grid = vec![vec![T::default(); self.width()]; self.height()];
        self.zbuffer = vec![vec![Float::NEG_INFINITY; self.width()]; self.height()];
    }

    fn plot(&mut self, x: i32, y: i32, z: Float, color: T) {
        if x >= 0 && y >= 0 {
            let cx = x as usize;
            let cy = y as usize;
            if cx < self.width() && cy < self.height() && self.zbuffer[cy][cx] - z < Z_RESOLUTION {
                self.zbuffer[cy][cx] = z;
                self.grid[cy][cx] = color;
            }
        }
    }

    ///Draws a line of pixels to the screen using Bresenham's line algorithm
    ///or a similar algorithm described [here](https://zingl.github.io/Bresenham.pdf).
    ///Pixels not visable on the screen (i.e. (-1, 4)) will just be ignored.
    ///The pixels are inclusive meaning both p1 and p2 may be drawn
    pub fn draw_line(&mut self, p1: Point, p2: Point, color: T) {
        //algorithm by Alois Zingl (https://zingl.github.io/Bresenham.pdf)
        //used because it is super clean

        //the x and y coords are stuffed into integers because stuff is more accuate and looks better
        let x1 = p1.0 as i32;
        let x2 = p2.0 as i32;
        let y1 = p1.1 as i32;
        let y2 = p2.1 as i32;

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let dz = (p2.2 - p1.2).abs();
        let sx = (x2 - x1).signum();
        let sy = (y2 - y1).signum();
        let sz = (p2.2 - p1.2).signum();

        let dm = dx.max(dy).max(dz as i32);
        let dmf = ((p2.0 - p1.0).abs()).max((p2.1 - p1.1).abs()).max(dz);

        let (mut x, mut y, mut z) = (x1, y1, p1.2);
        let (mut ex, mut ey, mut ez) = (dm / 2, dm / 2, dmf / 2.0);
        for _ in 0..=dm {
            self.plot(x, y, z, color);

            ex -= dx;
            ey -= dy;
            ez -= dz;
            if ex < 0 {
                ex += dm;
                x += sx;
            }
            if ey < 0 {
                ey += dm;
                y += sy;
            }
            if ez < 0.0 {
                ez += dmf;
                z += sz;
            }
        }
    }

    ///Draws a triangle of pixels to the screen
    pub fn draw_tri(&mut self, p1: Point, p2: Point, p3: Point, color: T) {
        //the scanline conversion algo used is a accuarate version by Yusuf
        //the implementation is by me
        let (mut tt, mut tm, mut tb) = (p1, p2, p3);
        if tm.1 > tt.1 {
            (tt, tm) = (tm, tt);
        }
        if tb.1 > tm.1 {
            (tm, tb) = (tb, tm);
        }
        if tm.1 > tt.1 {
            (tt, tm) = (tm, tt);
        }

        let dxbm = (tm.0 - tb.0) / (tm.1 - tb.1);
        let dzbm = (tm.2 - tb.2) / (tm.1 - tb.1);
        let dxmt = (tt.0 - tm.0) / (tt.1 - tm.1);
        let dzmt = (tt.2 - tm.2) / (tt.1 - tm.1);
        let dxbt = (tt.0 - tb.0) / (tt.1 - tb.1);
        let dzbt = (tt.2 - tb.2) / (tt.1 - tb.1);

        let left_to_right = dxbm < dxbt;

        let mut x0 = tb.0 + dxbm * (tb.1.ceil() - tb.1);
        let mut z0 = tb.2 + dzbm * (tb.1.ceil() - tb.1);

        let mut x1 = tb.0 + dxbt * (tb.1.ceil() - tb.1);
        let mut z1 = tb.2 + dzbt * (tb.1.ceil() - tb.1);

        for y in tb.1.ceil() as i32..=tm.1.floor() as i32 {
            if left_to_right {
                let dz = (z1 - z0) / (x1.floor() - x0.ceil() + 1.0);
                let mut z = z0;
                for x in x0.ceil() as i32..=x1.floor() as i32 {
                    self.plot(x, y, z, color);
                    z += dz;
                }
            } else {
                let dz = (z0 - z1) / (x0.floor() - x1.ceil() + 1.0);
                let mut z = z1;
                for x in x1.ceil() as i32..=x0.floor() as i32 {
                    self.plot(x, y, z, color);
                    z += dz;
                }
            }
            x0 += dxbm;
            z0 += dzbm;
            x1 += dxbt;
            z1 += dzbt;
        }

        x0 = tm.0 + dxmt * (tm.1.ceil() - tm.1);
        z0 = tm.2 + dzmt * (tm.1.ceil() - tm.1);

        for y in tm.1.ceil() as i32..=tt.1.floor() as i32 {
            if left_to_right {
                let dz = (z1 - z0) / (x1.floor() - x0.ceil() + 1.0);
                let mut z = z0;
                for x in x0.ceil() as i32..=x1.floor() as i32 {
                    self.plot(x, y, z, color);
                    z += dz;
                }
            } else {
                let dz = (z0 - z1) / (x0.floor() - x1.ceil() + 1.0);
                let mut z = z1;
                for x in x1.ceil() as i32..=x0.floor() as i32 {
                    self.plot(x, y, z, color);
                    z += dz;
                }
            }
            x0 += dxmt;
            z0 += dzmt;
            x1 += dxbt;
            z1 += dzbt;
        }
    }

    pub fn byte_vec(&self) -> Vec<u8> {
        let mut out = Vec::new();
        let max_val = T::max_val();
        out.extend_from_slice(
            format!("P6\n{} {}\n{}\n", self.width, self.height, T::max_val()).as_bytes(),
        );
        for v in self.grid.iter().rev() {
            for c in v {
                //panic on malformed max_val
                if max_val < c.red() || max_val < c.green() || max_val < c.blue() {
                    panic!("max_val less than red, green, or blue value");
                }
                //256 is the magic number for ppm files
                //this should also mean the below never panics
                if max_val <= 255 {
                    out.extend_from_slice(&[c.red() as u8, c.green() as u8, c.blue() as u8]);
                } else {
                    out.extend_from_slice(&c.red().to_le_bytes());
                    out.extend_from_slice(&c.green().to_le_bytes());
                    out.extend_from_slice(&c.blue().to_le_bytes());
                }
            }
        }
        out
    }

    ///Write contents as ppm to specified file path.
    ///The header writes in the binary format
    pub fn write_binary_ppm(&self, file: &mut File) -> Result<(), io::Error> {
        let mut file = BufWriter::new(file);
        file.write_all(&self.byte_vec())?;
        Ok(())
    }
}
