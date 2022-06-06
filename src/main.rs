//!This is my graphics class project.
//!It's done in rust so maybe I will learn something by the end.
//!Not meant to be useful but hopefully interesting in at least some way.
//!
//!The ues of this project is an interpreter for a language called MDL. The files
//!will have the extension `.mdl`.
//!
//!The interpreter stores a transformation matrix and edge matrix which are updated via commands in
//!the script file. The commands are the format specified src/parser/MDL.spec
//!
//!Currently not all of MDL is supported.

#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]

use std::env;
use std::error::Error;
use std::fs;
use std::process::{self, Command};

use binrw::io::Cursor;
use binrw::BinRead;

///default width of an image
pub const IMAGE_WIDTH: usize = 500;
///default height of an image
pub const IMAGE_HEIGHT: usize = 500;

use graphics::{Engine, Light, RGB8Color, Script};

fn run(script: Script) -> Result<(), Box<dyn Error>> {
    let mut eng = Engine::<RGB8Color>::with_screen_dims(IMAGE_WIDTH, IMAGE_HEIGHT);
    eng.set_ambient_light((50, 50, 50).into());
    //let light = Light::new((0.5, 0.75, 1.0), (0, 255, 255).into());
    let light = Light::new((5000.0, 7500.0, 10000.0), (255, 255, 255).into());
    eng.add_light(light);
    eng.set_camera((0.0, 0.0, 1.0));

    script.exec(&mut eng);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    Command::new("./deps/mdl")
        .arg(args.get(1).ok_or("No Input File Given")?)
        .status()
        .expect("failed to create intermediate with dw mdl parser");
    let script = fs::read("a.mdl_intermediate_language")?;
    let mut input = Cursor::new(script);
    let s = Script::read(&mut input).expect("could not read intermediate file");

    match run(s) {
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
        Ok(_) => {
            process::exit(0);
        }
    }
}
