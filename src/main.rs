//!This is my graphics class project.
//!It's done in rust so maybe I will learn something by the end.
//!Not meant to be useful but hopefully interesting in at least some way.

#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::process;

mod draw;
mod fatrix;
mod screen;

use screen::color::RGB8Color;
use screen::Screen;
use fatrix::{Fatrix, Modtrix};

const FILE_NAME: &str = "graphics_out";

fn dw_test() -> Screen<RGB8Color> {
    let xres = 500;
    let yres = 500;

    let mut m2 = Fatrix::new();
    m2.reserve(2);
    println!("Testing add_edge. Adding (1, 2, 3), (4, 5, 6) m2 =");
    m2.add_edge((1.0, 2.0, 3.0), (4.0, 5.0, 6.0));
    println!("{:?}", m2);

    println!("Testing ident. m1 = ");
    let m1 = Modtrix::IDENT;
    println!("{:?}", m1);

    println!("Testing Matrix mult. m1 * m2 =");
    println!("{:?}", m1 * m2.clone());

    println!("Testing Matrix mult. m1 =");
    let m1 = Modtrix::from([
        [1.00, 4.00, 7.00, 10.00],
        [2.00, 5.00, 8.00, 11.00],
        [3.00, 6.00, 9.00, 12.00],
        [1.00, 1.00, 1.00, 1.00],
    ]);
    println!("{:?}", m1);

    println!("Testing Matrix mult. m1 * m2=");
    println!("{:?}", m1 * m2);

    let mut edges = Fatrix::new();
    edges.add_edge((50.0, 450.0, 0.0), (100.0, 450.0, 0.0));
    edges.add_edge((50.0, 450.0, 0.0), (50.0, 400.0, 0.0));
    edges.add_edge((100.0, 450.0, 0.0), (100.0, 400.0, 0.0));
    edges.add_edge((100.0, 400.0, 0.0), (50.0, 400.0, 0.0));

    let p1 = (200.0, 450.0, 0.0);
    let p2 = (250.0, 450.0, 0.0);
    let p3 = (200.0, 400.0, 0.0);
    let p4 = (250.0, 400.0, 0.0);
    edges.add_edge(p1, p2);
    edges.add_edge(p1, p3);
    edges.add_edge(p2, p4);
    edges.add_edge(p4, p3);

    let p1 = (150.0, 400.0, 0.0);
    let p2 = (130.0, 360.0, 0.0);
    let p3 = (170.0, 360.0, 0.0);

    edges.add_edge(p1, p2);
    edges.add_edge(p1, p3);
    edges.add_edge(p2, p3);

    let p1 = (100.0, 340.0, 0.0);
    let p2 = (200.0, 340.0, 0.0);
    let p3 = (100.0, 320.0, 0.0);
    let p4 = (200.0, 320.0, 0.0);

    edges.add_edge(p1, p2);
    edges.add_edge(p3, p4);
    edges.add_edge(p1, p3);
    edges.add_edge(p2, p4);


    edges.screen::<RGB8Color>((100, 255, 255).into(), xres, yres)
}

fn run() -> Result<(), Box<dyn Error>> {
    let scrn = dw_test();
    //let scrn = make_cool_screen();

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
