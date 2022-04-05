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

lrlex_mod!("dwscript.l");
lrpar_mod!("dwscript.y");

///Evaluates the AST built by the parser
//FIXME: fix filename clashing
fn eval<'a>(
    lexer: &'a dyn NonStreamingLexer<DefaultLexeme, u32>,
    e: Expr,
    trans: &mut Modtrix,
    twoson: &mut Space,
) -> Result<&'a str, (Span, &'static str)> {
    match e {
        Expr::Expr { span: _span, cmds } => {
            for c in cmds {
                eval(lexer, c, trans, twoson)?;
            }
            Ok("")
        }
        Expr::Command { span: _span, fun } => {
            eval(lexer, *fun, trans, twoson)?;
            Ok("")
        }
        Expr::Function { span: _span, typ } => {
            eval(lexer, *typ, trans, twoson)?;
            Ok("")
        }
        Expr::Line { span, args } => {
            let nums = args
                .into_iter()
                .map(|x| {
                    eval(lexer, *x, trans, twoson)?
                        .parse()
                        .map_err(|_| (span, "input not a number"))
                })
                .collect::<Vec<Result<_, _>>>();
            let p1 = (nums[0]?, nums[1]?, nums[2]?);
            let p2 = (nums[3]?, nums[4]?, nums[5]?);
            twoson.add_line(p1, p2);
            Ok("")
        }
        Expr::Ident { span: _span } => {
            trans.ident();
            Ok("")
        }
        Expr::Scale { span, args } => {
            let nums = args
                .into_iter()
                .map(|x| {
                    eval(lexer, *x, trans, twoson)?
                        .parse()
                        .map_err(|_| (span, "input not a number"))
                })
                .collect::<Vec<Result<_, _>>>();
            let sm = scale_matrix!(nums[0]?, nums[1]?, nums[2]?);
            Modtrix::mult(&sm, trans);
            Ok("")
        }
        Expr::Move { span, args } => {
            let nums = args
                .into_iter()
                .map(|x| {
                    eval(lexer, *x, trans, twoson)?
                        .parse()
                        .map_err(|_| (span, "input not a number"))
                })
                .collect::<Vec<Result<_, _>>>();
            let mm = move_matrix!(nums[0]?, nums[1]?, nums[2]?);
            Modtrix::mult(&mm, trans);
            Ok("")
        }
        Expr::Rotate { span, axis, deg } => {
            let a = eval(lexer, *axis, trans, twoson)?;
            let t = eval(lexer, *deg, trans, twoson)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse angle of rotation"))?;
            let rm = match a {
                "x" => rotx_matrix!(t),
                "y" => roty_matrix!(t),
                "z" => rotz_matrix!(t),
                _ => return Err((span, "cannot parse axis")),
            };
            Modtrix::mult(&rm, trans);
            Ok("")
        }
        Expr::Apply { span: _span } => {
            twoson.apply(trans);
            Ok("")
        }
        Expr::Display { span } => {
            let mut scrn = Screen::<RGB8Color>::with_size(IMAGE_WIDTH, IMAGE_HEIGHT);
            Space::screen(twoson, (255, 255, 255).into(), &mut scrn);
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
            let file_name = eval(lexer, *file, trans, twoson)?;
            let file_ppm = format!(".tmp_convertfilelhfgfhgf{}.ppm", file_name);
            let path = PathBuf::from(&file_ppm);
            let mut file = File::create(&path).map_err(|_| (span, "failed create file path"))?;
            let mut scrn = Screen::<RGB8Color>::with_size(IMAGE_WIDTH, IMAGE_HEIGHT);
            Space::screen(twoson, (255, 255, 255).into(), &mut scrn);
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
            let cx = eval(lexer, *cx, trans, twoson)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let cy = eval(lexer, *cy, trans, twoson)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let cz = eval(lexer, *cz, trans, twoson)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let r = eval(lexer, *r, trans, twoson)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            draw::add_circle(cx, cy, cz, r, twoson);
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
            let x0 = eval(lexer, *x0, trans, twoson)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let y0 = eval(lexer, *y0, trans, twoson)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let x1 = eval(lexer, *x1, trans, twoson)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let y1 = eval(lexer, *y1, trans, twoson)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let rx0 = eval(lexer, *rx0, trans, twoson)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let ry0 = eval(lexer, *ry0, trans, twoson)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let rx1 = eval(lexer, *rx1, trans, twoson)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let ry1 = eval(lexer, *ry1, trans, twoson)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            draw::add_hermite(x0, y0, x1, y1, rx0, ry0, rx1, ry1, twoson);
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
            let x0 = eval(lexer, *x0, trans, twoson)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let y0 = eval(lexer, *y0, trans, twoson)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let x1 = eval(lexer, *x1, trans, twoson)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let y1 = eval(lexer, *y1, trans, twoson)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let x2 = eval(lexer, *x2, trans, twoson)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let y2 = eval(lexer, *y2, trans, twoson)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let x3 = eval(lexer, *x3, trans, twoson)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            let y3 = eval(lexer, *y3, trans, twoson)?
                .parse::<Float>()
                .map_err(|_| (span, "cannot parse num"))?;
            draw::add_bezier(x0, y0, x1, y1, x2, y2, x3, y3, twoson);
            Ok("")
        }
        Expr::Box { span, args } => {
            let args = args
                .into_iter()
                .map(|x| {
                    eval(lexer, *x, trans, twoson)?
                        .parse()
                        .map_err(|_| (span, "input not a number"))
                })
                .collect::<Vec<Result<_, _>>>();
            draw::add_box(
                args[0]?, args[1]?, args[2]?, args[3]?, args[4]?, args[5]?, twoson,
            );
            Ok("")
        }
        Expr::Sphere { span, args } => {
            let args = args
                .into_iter()
                .map(|x| {
                    eval(lexer, *x, trans, twoson)?
                        .parse()
                        .map_err(|_| (span, "input not a number"))
                })
                .collect::<Vec<Result<_, _>>>();
            draw::add_sphere(args[0]?, args[1]?, args[2]?, args[3]?, twoson);
            Ok("")
        }
        Expr::Torus { span, args } => {
            let args = args
                .into_iter()
                .map(|x| {
                    eval(lexer, *x, trans, twoson)?
                        .parse()
                        .map_err(|_| (span, "input not a number"))
                })
                .collect::<Vec<Result<_, _>>>();
            draw::add_torus(args[0]?, args[1]?, args[2]?, args[3]?, args[4]?, twoson);
            Ok("")
        }
        Expr::Clear { span: _span } => {
            twoson.clear();
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
        let mut trans = Modtrix::IDENT.clone();
        let mut edges = Space::new();
        if let Err((span, msg)) = eval(&lexer, r, &mut trans, &mut edges) {
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
