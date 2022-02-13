//!This is my graphics class project.
//!It's done in rust so maybe I will learn something by the end.
//!Not meant to be useful but hopefully interesting in at least some way.

#![warn(missing_docs,missing_debug_implementations,rust_2018_idioms)]

use std::fs::File;
use std::path::PathBuf;
use std::error::Error;
use std::process;

mod screen;
mod draw;

use screen::Screen;
use screen::color::RGB8Color;
use draw::draw_line;

const FILE_NAME: &str = "graphics_out";

fn dw_test() -> Screen<RGB8Color> {
    let xres = 500;
    let yres = 500;
    //obviously fine conversion
    let xres_size = xres as usize;
    let yres_size = yres as usize;

    let mut scrn = Screen::<RGB8Color>::with_size(xres_size, yres_size);

    //octants 1 and 5
    let c = (0, 255, 0).into();
    draw_line((0, 0), (xres-1, yres-1), c, &mut scrn);
    draw_line((0, 0), (xres-1, yres/2), c, &mut scrn);
    draw_line((xres-1, yres-1), (0, yres/2), c, &mut scrn);

    //octants 8 and 4
    let c = (0, 255, 255).into();
    draw_line((0, yres-1), (xres-1, 0), c, &mut scrn);
    draw_line((0, yres-1), (xres-1, yres/2), c, &mut scrn);
    draw_line((xres-1, 0), (0, yres/2), c, &mut scrn);

    //octants 2 and 6
    let c = (255, 0, 0).into();
    draw_line((0, 0), (xres/2, yres-1), c, &mut scrn);
    draw_line((xres-1, yres-1), (xres/2, 0), c, &mut scrn);

    //octants 7 and 3
    let c = (255, 0, 255).into();
    draw_line((0, yres-1), (xres/2, 0), c, &mut scrn);
    draw_line((xres-1, 0), (xres/2, yres-1), c, &mut scrn);

    //horizontal and vertical
    let c = (255, 255, 0).into();
    draw_line((0, yres/2), (xres-1, yres/2), c, &mut scrn);
    draw_line((xres/2, 0), (xres/2, yres-1), c, &mut scrn);

    scrn
}

fn run() -> Result<(), Box<dyn Error>> {
    let scrn = dw_test();

    let file_ppm = format!("{}.ppm", FILE_NAME);
    let path = PathBuf::from(&file_ppm);
    //the program just puts the file wherever it was run from because why not...
    //clean it up yourself, I'm too lazy
    let mut file = File::create(&path)?;
    scrn.write_binary_ppm(&mut file)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    match run() {
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
        Ok(_) => {
            process::exit(0);
        }
    }
}
