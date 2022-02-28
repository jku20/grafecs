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

use draw::draw_line;
use screen::color::{Color, RGB8Color};
use screen::Screen;

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
    draw_line((0, 0), (xres - 1, yres - 1), c, &mut scrn);
    draw_line((0, 0), (xres - 1, yres / 2), c, &mut scrn);
    draw_line((xres - 1, yres - 1), (0, yres / 2), c, &mut scrn);

    //octants 8 and 4
    let c = (0, 255, 255).into();
    draw_line((0, yres - 1), (xres - 1, 0), c, &mut scrn);
    draw_line((0, yres - 1), (xres - 1, yres / 2), c, &mut scrn);
    draw_line((xres - 1, 0), (0, yres / 2), c, &mut scrn);

    //octants 2 and 6
    let c = (255, 0, 0).into();
    draw_line((0, 0), (xres / 2, yres - 1), c, &mut scrn);
    draw_line((xres - 1, yres - 1), (xres / 2, 0), c, &mut scrn);

    //octants 7 and 3
    let c = (255, 0, 255).into();
    draw_line((0, yres - 1), (xres / 2, 0), c, &mut scrn);
    draw_line((xres - 1, 0), (xres / 2, yres - 1), c, &mut scrn);

    //horizontal and vertical
    let c = (255, 255, 0).into();
    draw_line((0, yres / 2), (xres - 1, yres / 2), c, &mut scrn);
    draw_line((xres / 2, 0), (xres / 2, yres - 1), c, &mut scrn);

    scrn
}

//yeah my code to draw these images is pretty bad
//I swear the actual graphics engine code is better
fn is_prime(n: i32) -> bool {
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn smooth(s: &mut Screen<RGB8Color>) {
    for i in 0..s.width() {
        for j in 0..s.height() {
            let c = s[[i, j]];
            let (mut r, mut g, mut b) = (c.red(), c.green(), c.blue());
            for (dx, dy) in [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)] {
                let nx = (i as i32 + dx).min(s.width() as i32 - 1).max(0);
                let ny = (j as i32 + dy).min(s.height() as i32 - 1).max(0);
                let c = s[[nx as usize, ny as usize]];
                r += c.red();
                g += c.green();
                b += c.blue();
            }
            s[[i, j]] = (r as u8 / 5, g as u8 / 5, b as u8 / 5).into();
        }
    }
}

fn rot(s: &mut Screen<RGB8Color>) {
    let h = s.height();
    for i in 0..s.width() {
        for j in i + 1..s.height() {
            let t = s[[i, h - j]];
            s[[i, h - j]] = s[[i, j]];
            s[[i, j]] = t;
        }
    }
}

fn make_cool_screen() -> Screen<RGB8Color> {
    let res = 500;
    let mut s = Screen::with_size(res, res);

    let mut cur = 100000000;
    for i in 0..=1000 {
        cur += 1;
        let mut gp = 1;
        while !is_prime(cur) {
            gp += 1;
            cur += 1;
        }
        let nm = (gp) as u8;
        draw_line((gp, i), (i, gp), (nm, 50, (i / 3) as u8).into(), &mut s);
    }

    rot(&mut s);

    for _ in 0..15 {
        smooth(&mut s);
    }

    for i in 0..s.width() {
        for j in 0..s.height() {
            let c = s[[i, j]];
            let d = 240;
            s[[i, j]] = (c.blue() as u8 + d, c.green() as u8 + d, c.red() as u8 + d).into();
        }
    }

    s
}

fn run() -> Result<(), Box<dyn Error>> {
    //let scrn = dw_test();
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
