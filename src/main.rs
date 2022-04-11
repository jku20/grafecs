//!This is my graphics class project.
//!It's done in rust so maybe I will learn something by the end.
//!Not meant to be useful but hopefully interesting in at least some way.
//!
//!The ues of this project is an interpreter for a language I lovingly call DWscript. The files
//!will have the extension `.dw`.
//!
//!The interpreter stores a transformation matrix and edge matrix which are updated via commands in
//!the script file. The commands are the format:
//!```
//!command
//!arg1 arg2 arg3...
//!command
//!arg1 arg2 arg3...
//!...
//!```
//!The commands are
//!
//!`line`: add a line to the point matrix - takes 6 arguemnts (x0, y0, z0, x1, y1, z1)
//!
//!`ident`: set the transform matrix to the identity matrix
//!
//!`scale`: create a scale matrix, then multiply the transform matrix by the scale matrix - takes 3 arguments (sx, sy, sz)
//!
//!`move`: create a translation matrix, then multiply the transform matrix by the translation matrix - takes 3 arguments (tx, ty, tz)
//!
//!`rotate`: create a rotation matrix, then multiply the transform matrix by the rotation matrix - takes 2 arguments (axis theta)
//!
//!`apply`: apply the current transformation matrix to the edge matrix
//!
//!`display`: clear the screen, draw the lines of the point matrix to the screen, display the screen
//!
//!`save`: clear the screen, draw the lines of the point matrix to the screen/frame save the screen/frame to a file - takes 1 argument (file name)

#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]

use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::{self, Command, Stdio};

use dwscript_y::Expr;
use lrlex::{lrlex_mod, DefaultLexeme};
use lrpar::{lrpar_mod, NonStreamingLexer, Span};

mod draw;
mod fatrix;
mod gmath;
mod screen;

use fatrix::{Float, Modtrix, Space};
use screen::{color::RGB8Color, Screen};

const IMAGE_WIDTH: usize = 500;
const IMAGE_HEIGHT: usize = 500;
const LINE_COLOR: RGB8Color = RGB8Color::WHITE;

lrlex_mod!("dwscript.l");
lrpar_mod!("dwscript.y");

///given a screen, a coordinate system/Modtrix, and a "good" draw function, the macro will draw
///the shape made with that draw function to the screen
macro_rules! draw_to_screen {
    ( $screen:expr, $sys:expr, $shape:expr, $color:expr ,$( $args:expr ),* ) => {
        {
            let mut f = Space::new();
            $shape($( $args ),*, &mut f);
            f.apply($sys);
            Space::draw_space(&f, $color, $screen);
        }
    };
}

///Evaluates the AST built by the parser
fn eval<'a>(
    lexer: &'a dyn NonStreamingLexer<DefaultLexeme, u32>,
    e: Expr,
    coord: &mut Vec<Modtrix>,
    scrn: &mut Screen<RGB8Color>,
) -> Result<&'a str, (Span, &'static str)> {
    match e {
        Expr::Expr { span: _span, cmds } => {
            for c in cmds {
                eval(lexer, c, coord, scrn)?;
            }
            Ok("")
        }
        Expr::Command { span: _span, fun } => {
            eval(lexer, *fun, coord, scrn)?;
            Ok("")
        }
        Expr::Function { span: _span, typ } => {
            eval(lexer, *typ, coord, scrn)?;
            Ok("")
        }
        Expr::Line { span, args } => {
            let nums = args
                .into_iter()
                .map(|x| {
                    eval(lexer, *x, coord, scrn)?
                        .parse()
                        .map_err(|_| (span, "input not a number"))
                })
                .collect::<Vec<Result<_, _>>>();
            let p1 = (nums[0]?, nums[1]?, nums[2]?);
            let p2 = (nums[3]?, nums[4]?, nums[5]?);
            draw_to_screen!(
                scrn,
                coord.last().unwrap(),
                draw::add_line,
                LINE_COLOR,
                p1,
                p2
            );
            Ok("")
        }
        Expr::Ident { span: _span } => Ok(""),
        Expr::Scale { span, args } => {
            let nums = args
                .into_iter()
                .map(|x| {
                    eval(lexer, *x, coord, scrn)?
                        .parse()
                        .map_err(|_| (span, "input not a number"))
                })
                .collect::<Vec<Result<_, _>>>();
            let sm = scale_matrix!(nums[0]?, nums[1]?, nums[2]?);
            Modtrix::multr(coord.last_mut().unwrap(), &sm);
            Ok("")
        }
        Expr::Move { span, args } => {
            let nums = args
                .into_iter()
                .map(|x| {
                    eval(lexer, *x, coord, scrn)?
                        .parse()
                        .map_err(|_| (span, "input not a number"))
                })
                .collect::<Vec<Result<_, _>>>();
            let mm = move_matrix!(nums[0]?, nums[1]?, nums[2]?);
            Modtrix::multr(coord.last_mut().unwrap(), &mm);
            Ok("")
        }
        Expr::Rotate { span, axis, deg } => {
            let a = eval(lexer, *axis, coord, scrn)?;
            let t = eval(lexer, *deg, coord, scrn)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse angle of rotation"))?;
            let rm = match a {
                "x" => rotx_matrix!(t),
                "y" => roty_matrix!(t),
                "z" => rotz_matrix!(t),
                _ => return Err((span, "cannot parse axis")),
            };
            Modtrix::multr(coord.last_mut().unwrap(), &rm);
            Ok("")
        }
        Expr::Apply { span: _span } => Ok(""),
        Expr::Display { span } => {
            let mut display_command = Command::new("display")
                .stdin(Stdio::piped())
                .spawn()
                .map_err(|_| (span, "failed to display image"))?;

            //hopefully unwrap won't fail
            display_command
                .stdin
                .as_mut()
                .ok_or((span, "failed to display image"))?
                .write_all(&scrn.byte_vec())
                .map_err(|_| (span, "failed to display image"))?;
            display_command
                .wait()
                .map_err(|_| (span, "failed to display image"))?;
            Ok("")
        }
        Expr::Save { span, file } => {
            let file_name = eval(lexer, *file, coord, scrn)?;
            let file_ppm = format!(".tmp_convertfilelhfgfhgf{}.ppm", file_name);
            let path = PathBuf::from(&file_ppm);
            let mut file = File::create(&path).map_err(|_| (span, "failed create file path"))?;
            scrn.write_binary_ppm(&mut file)
                .map_err(|_| (span, "failed to write ppm file"))?;
            //requires imagemgick
            Command::new("convert")
                .arg(&file_ppm)
                .arg(&file_name)
                .status()
                .map_err(|_| (span, "failed to convert file to png"))?;
            fs::remove_file(&file_ppm).map_err(|_| (span, "couldn't remove tmp file"))?;
            Ok("")
        }
        Expr::Circle {
            span,
            cx,
            cy,
            cz,
            r,
        } => {
            let cx = eval(lexer, *cx, coord, scrn)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let cy = eval(lexer, *cy, coord, scrn)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let cz = eval(lexer, *cz, coord, scrn)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let r = eval(lexer, *r, coord, scrn)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            draw_to_screen!(
                scrn,
                coord.last().unwrap(),
                draw::add_circle,
                LINE_COLOR,
                cx,
                cy,
                cz,
                r
            );
            Ok("")
        }
        Expr::Hermite {
            span,
            x0,
            y0,
            x1,
            y1,
            rx0,
            ry0,
            rx1,
            ry1,
        } => {
            let x0 = eval(lexer, *x0, coord, scrn)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let y0 = eval(lexer, *y0, coord, scrn)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let x1 = eval(lexer, *x1, coord, scrn)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let y1 = eval(lexer, *y1, coord, scrn)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let rx0 = eval(lexer, *rx0, coord, scrn)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let ry0 = eval(lexer, *ry0, coord, scrn)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let rx1 = eval(lexer, *rx1, coord, scrn)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let ry1 = eval(lexer, *ry1, coord, scrn)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            draw_to_screen!(
                scrn,
                coord.last().unwrap(),
                draw::add_hermite,
                LINE_COLOR,
                x0,
                y0,
                x1,
                y1,
                rx0,
                ry0,
                rx1,
                ry1
            );
            Ok("")
        }
        Expr::Bezier {
            span,
            x0,
            y0,
            x1,
            y1,
            x2,
            y2,
            x3,
            y3,
        } => {
            let x0 = eval(lexer, *x0, coord, scrn)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let y0 = eval(lexer, *y0, coord, scrn)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let x1 = eval(lexer, *x1, coord, scrn)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let y1 = eval(lexer, *y1, coord, scrn)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let x2 = eval(lexer, *x2, coord, scrn)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let y2 = eval(lexer, *y2, coord, scrn)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let x3 = eval(lexer, *x3, coord, scrn)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let y3 = eval(lexer, *y3, coord, scrn)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            draw_to_screen!(
                scrn,
                coord.last().unwrap(),
                draw::add_bezier,
                LINE_COLOR,
                x0,
                y0,
                x1,
                y1,
                x2,
                y2,
                x3,
                y3
            );
            Ok("")
        }
        Expr::Box { span, args } => {
            let args = args
                .into_iter()
                .map(|x| {
                    eval(lexer, *x, coord, scrn)?
                        .parse()
                        .map_err(|_| (span, "input not a number"))
                })
                .collect::<Vec<Result<_, _>>>();
            draw_to_screen!(
                scrn,
                coord.last().unwrap(),
                draw::add_box,
                LINE_COLOR,
                args[0]?,
                args[1]?,
                args[2]?,
                args[3]?,
                args[4]?,
                args[5]?
            );
            Ok("")
        }
        Expr::Sphere { span, args } => {
            let args = args
                .into_iter()
                .map(|x| {
                    eval(lexer, *x, coord, scrn)?
                        .parse()
                        .map_err(|_| (span, "input not a number"))
                })
                .collect::<Vec<Result<_, _>>>();
            draw_to_screen!(
                scrn,
                coord.last().unwrap(),
                draw::add_sphere,
                LINE_COLOR,
                args[0]?,
                args[1]?,
                args[2]?,
                args[3]?
            );
            Ok("")
        }
        Expr::Torus { span, args } => {
            let args = args
                .into_iter()
                .map(|x| {
                    eval(lexer, *x, coord, scrn)?
                        .parse()
                        .map_err(|_| (span, "input not a number"))
                })
                .collect::<Vec<Result<_, _>>>();
            draw_to_screen!(
                scrn,
                coord.last().unwrap(),
                draw::add_torus,
                LINE_COLOR,
                args[0]?,
                args[1]?,
                args[2]?,
                args[3]?,
                args[4]?
            );
            Ok("")
        }
        Expr::Clear { span: _span } => Ok(""),
        Expr::Push { span: _span } => {
            coord.push(coord.last().unwrap().clone());
            Ok("")
        }
        Expr::Pop { span: _span } => {
            coord.pop();
            Ok("")
        }
        Expr::Num { span } => Ok(lexer.span_str(span)),
        Expr::Axis { span } => Ok(lexer.span_str(span)),
        Expr::File { span } => Ok(lexer.span_str(span)),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let script = fs::read_to_string(&args.get(1).ok_or("No Input File Given")?)?;

    let lexerdef = dwscript_l::lexerdef();
    let lexer = lexerdef.lexer(script.trim());
    let (res, errs) = dwscript_y::parse(&lexer);

    for e in errs {
        println!("{}", e.pp(&lexer, &dwscript_y::token_epp));
    }
    if let Some(Ok(r)) = res {
        let mut coords = vec![Modtrix::IDENT];
        let mut scrn = Screen::<RGB8Color>::with_size(IMAGE_WIDTH, IMAGE_HEIGHT);

        if let Err((span, msg)) = eval(&lexer, r, &mut coords, &mut scrn) {
            let ((line, col), _) = lexer.line_col(span);
            eprintln!(
                "Error parsing scriptat line {} column {}, '{}' {}.",
                line,
                col,
                lexer.span_str(span),
                msg,
            );
        }
    }
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
