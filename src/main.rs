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
use std::path::PathBuf;
use std::process::{self, Command};

use dwscript_y::Expr;
use lrlex::{lrlex_mod, DefaultLexeme};
use lrpar::{lrpar_mod, NonStreamingLexer, Span};

mod draw;
mod fatrix;
mod screen;

use fatrix::{Fatrix, Float, Modtrix};
use screen::color::RGB8Color;

const TMP_FILE_NAME: &str = "graphics_out";
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
    edges: &mut Fatrix,
) -> Result<&'a str, (Span, &'static str)> {
    match e {
        Expr::Expr {
            span: _span,
            lhs,
            rhs,
        } => {
            eval(lexer, *lhs, trans, edges)?;
            eval(lexer, *rhs, trans, edges)?;
            Ok("")
        }
        Expr::Command { span: _span, fun } => {
            eval(lexer, *fun, trans, edges)?;
            Ok("")
        }
        Expr::Function { span: _span, typ } => {
            eval(lexer, *typ, trans, edges)?;
            Ok("")
        }
        Expr::Line { span, args } => {
            let nums = args
                .into_iter()
                .map(|x| {
                    eval(lexer, *x, trans, edges)?
                        .parse()
                        .map_err(|_| (span, "input not a number"))
                })
                .collect::<Vec<Result<_, _>>>();
            let p1 = (nums[0]?, nums[1]?, nums[2]?);
            let p2 = (nums[3]?, nums[4]?, nums[5]?);
            edges.add_edge(p1, p2);
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
                    eval(lexer, *x, trans, edges)?
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
                    eval(lexer, *x, trans, edges)?
                        .parse()
                        .map_err(|_| (span, "input not a number"))
                })
                .collect::<Vec<Result<_, _>>>();
            let mm = move_matrix!(nums[0]?, nums[1]?, nums[2]?);
            Modtrix::mult(&mm, trans);
            Ok("")
        }
        Expr::Rotate { span, axis, deg } => {
            let a = eval(lexer, *axis, trans, edges)?;
            let t = eval(lexer, *deg, trans, edges)?
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
            edges.apply(trans);
            Ok("")
        }
        Expr::Display { span } => {
            let file_ppm = format!(".tmp_displayfilelhfgfhgf{}.ppm", TMP_FILE_NAME);
            let path = PathBuf::from(&file_ppm);
            let mut file = File::create(&path).map_err(|_| (span, "failed create file path"))?;
            let scrn = edges.screen::<RGB8Color>((255, 255, 255).into(), IMAGE_WIDTH, IMAGE_HEIGHT);
            scrn.write_binary_ppm(&mut file)
                .map_err(|_| (span, "failed to write ppm file"))?;
            //requires imagemagick
            Command::new("display")
                .arg(&file_ppm)
                .status()
                .map_err(|_| (span, "failed to display file"))?;

            fs::remove_file(&file_ppm).map_err(|_| (span, "couldn't remove tmp file"))?;
            Ok("")
        }
        Expr::Save { span, file } => {
            let file_name = eval(lexer, *file, trans, edges)?;
            let file_ppm = format!(".tmp_convertfilelhfgfhgf{}.ppm", file_name);
            let path = PathBuf::from(&file_ppm);
            let mut file = File::create(&path).map_err(|_| (span, "failed create file path"))?;
            let scrn = edges.screen::<RGB8Color>((255, 255, 255).into(), IMAGE_WIDTH, IMAGE_HEIGHT);
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
        Expr::Num { span } => Ok(lexer.span_str(span)),
        Expr::Axis { span } => Ok(lexer.span_str(span)),
        Expr::File { span } => Ok(lexer.span_str(span)),
    }
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
