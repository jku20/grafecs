//!This is my graphics class project.
//!It's done in rust so maybe I will learn something by the end.
//!Not meant to be useful but hopefully interesting in at least some way.
use std::process::Command;
use std::fs::{self, File};
use std::path::PathBuf;
use std::error::Error;
use std::process;

mod screen;

use screen::Screen;
use screen::color;

const FILE_NAME: &str = "graphics_out";

fn run() -> Result<(), Box<dyn Error>> {
    let scrn = Screen::<color::RGB8Color>::with_size(500, 500);

    let file_ppm = format!("{}.ppm", FILE_NAME);
    let file_png = format!("{}.png", FILE_NAME);
    let path = PathBuf::from(&file_ppm);
    //the program just puts the file wherever it was run from because why not...
    //clean it up yourself, I'm too lazy
    let mut file = File::create(&path)?;
    scrn.write_ppm(&mut file)?;

    //imagemagick dependancy go yay
    Command::new("convert")
        .arg(&file_ppm)
        .arg(&file_png)
        .status()
        .expect("Couldn't fine convert and convert the ppm to a png");
    fs::remove_file(&path)?;

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
