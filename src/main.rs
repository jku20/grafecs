//!This is my graphics class project.
//!It's done in rust so maybe I will learn something by the end.
//!Not meant to be useful but hopefully interesting in at least some way.

#![warn(missing_docs,missing_debug_implementations,rust_2018_idioms)]

use std::fs::File;
use std::path::PathBuf;
use std::error::Error;
use std::process;

mod screen;

use screen::Screen;
use screen::color::RGB8Color;

const FILE_NAME: &str = "graphics_out";

fn g(x: usize) -> u8 {
    let mut x = x as f32;
    x /= 500.0;
    x *= 3.0;
    x -= 3.0;
    (7.0*(((5.0*x).cos() + (5.0*x).sin() + 3.0)*(0.2*x*x*x*x*x + x*x*x*x + x*x*x + 0.5*x*x + x + 2.0))) as u8
}

fn make_cool_screen() -> Screen<RGB8Color> {
    let mut scrn = Screen::<RGB8Color>::with_size(500, 500);
    for i in 0..500 {
        for j in 0..500-i {
            scrn[[i, i+j]] = RGB8Color::new(g(i), g(j), g(i));
        }
    }
    for i in 0..500 {
        for j in 0..500-i {
            scrn[[i+j, i]] = RGB8Color::new(g(i), g(j), g(i));
        }
    }
    scrn
}

fn run() -> Result<(), Box<dyn Error>> {
    let scrn = make_cool_screen();

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
