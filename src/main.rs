//!This is my graphics class project.
//!It's done in rust so maybe I will learn something by the end.
//!Not meant to be useful but hopefully interesting in at least some way.

#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]

use std::error::Error;
use std::fs::{self, File};
use std::path::PathBuf;
use std::{process, env};

use lrlex::{lrlex_mod, DefaultLexeme};
use lrpar::{lrpar_mod, NonStreamingLexer, Span};
use dwscript_y::Expr;

mod draw;
mod fatrix;
mod screen;

use screen::color::RGB8Color;
use screen::Screen;
use fatrix::{Fatrix, Modtrix};

const FILE_NAME: &str = "graphics_out";

lrlex_mod!("dwscript.l");
lrpar_mod!("dwscript.y");

fn eval(lexer: &dyn NonStreamingLexer<DefaultLexeme, u32>, e: Expr, trans: &mut Modtrix, edges: &mut Fatrix) {

}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let script = fs::read_to_string(&args[1])?;

    let lexerdef = dwscript_l::lexerdef();
    let lexer = lexerdef.lexer(script.trim());
    let (res, errs) = dwscript_y::parse(&lexer);

    for e in errs {
        println!("{}", e.pp(&lexer, &dwscript_y::token_epp));
    }
    if let Some(Ok(r)) = res {
        let mut trans = Modtrix::IDENT.clone();
        let mut edges = Fatrix::new();
        eval(&lexer, r, &mut trans, &mut edges);
    }
    Ok(())
    /*

    let file_ppm = format!("{}.ppm", FILE_NAME);
    let path = PathBuf::from(&file_ppm);
    //the program just puts the file wherever it was run from because why not...
    //clean it up yourself, I'm too lazy
    let mut file = File::create(&path)?;
    scrn.write_binary_ppm(&mut file)?;
    Ok(())
    */
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
