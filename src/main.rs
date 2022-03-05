//!This is my graphics class project.
//!It's done in rust so maybe I will learn something by the end.
//!Not meant to be useful but hopefully interesting in at least some way.

#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]

use std::error::Error;
use std::fs::{self, File};
use std::path::PathBuf;
use std::env;
use std::process::{self, Command};

use lrlex::{lrlex_mod, DefaultLexeme};
use lrpar::{lrpar_mod, NonStreamingLexer, Span};
use dwscript_y::Expr;

mod draw;
mod fatrix;
mod screen;

use screen::color::RGB8Color;
use fatrix::{Fatrix, Modtrix, Float};

const FILE_NAME: &str = "graphics_out";
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
    edges: &mut Fatrix
) -> Result<&'a str, (Span, &'static str)> {
    match e {
        Expr::Expr { span, lhs, rhs } => {
            eval(lexer, *lhs, trans, edges)?;
            eval(lexer, *rhs, trans, edges)?;
            Ok("")
        },
        Expr::Command { span, fun } => {
            eval(lexer, *fun, trans, edges)?;
            Ok("")
        },
        Expr::Function { span, typ } => {
            eval(lexer, *typ, trans, edges)?;
            Ok("")
        },
        Expr::Line { span, args } => {
            let nums = args
                .into_iter()
                .map(|x| eval(lexer, *x, trans, edges)?
                     .parse()
                     .map_err(|_| (span, "input not a number"))
                )
                .collect::<Vec<Result<_, _>>>();
            let p1 = (nums[0]?, nums[1]?, nums[2]?);
            let p2 = (nums[3]?, nums[4]?, nums[5]?);
            edges.add_edge(p1, p2);
            Ok("")
        },
        Expr::Ident { span } => {
            trans.ident();
            Ok("")
        },
        Expr::Scale { span, args } => {
            let nums = args
                .into_iter()
                .map(|x| eval(lexer, *x, trans, edges)?
                     .parse()
                     .map_err(|_| (span, "input not a number"))
                )
                .collect::<Vec<Result<_, _>>>();
            let sm = scale_matrix!(nums[0]?, nums[1]?, nums[2]?);
            Modtrix::mult(&sm, trans);
            Ok("")
        },
        Expr::Move { span, args } => {
            let nums = args
                .into_iter()
                .map(|x| eval(lexer, *x, trans, edges)?
                     .parse()
                     .map_err(|_| (span, "input not a number"))
                )
                .collect::<Vec<Result<_, _>>>();
            let mm = move_matrix!(nums[0]?, nums[1]?, nums[2]?);
            Modtrix::mult(&mm, trans);
            Ok("")
        },
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
        },
        Expr::Apply { span } => {
            edges.apply(trans);
            Ok("")
        },
        Expr::Display { span } => {
            let file_ppm = format!(".tmp_displayfilelhfgfhgf{}.ppm", FILE_NAME);
            let path = PathBuf::from(&file_ppm);
            let mut file = File::create(&path).map_err(|_| (span, "failed create file path"))?;
            let scrn = edges.screen::<RGB8Color>((255, 255, 255).into(), IMAGE_WIDTH, IMAGE_HEIGHT);
            scrn.write_binary_ppm(&mut file).map_err(|_| (span, "failed to write ppm file"))?;
            //requires imagemagick
            Command::new("display")
                .arg(&file_ppm)
                .status()
                .map_err(|_| (span, "failed to display file"))?;

            fs::remove_file(&file_ppm)
                .map_err(|_| (span, "couldn't remove tmp file"))?;
            Ok("")
        },
        Expr::Save { span, file } => {
            let file_name = eval(lexer, *file, trans, edges)?;
            let file_ppm = format!(".tmp_convertfilelhfgfhgf{}.ppm", file_name);
            let path = PathBuf::from(&file_ppm);
            let mut file = File::create(&path).map_err(|_| (span, "failed create file path"))?;
            let scrn = edges.screen::<RGB8Color>((255, 255, 255).into(), IMAGE_WIDTH, IMAGE_HEIGHT);
            scrn.write_binary_ppm(&mut file).map_err(|_| (span, "failed to write ppm file"))?;
            //requires imagemgick
            Command::new("convert")
                .arg(&file_ppm)
                .arg(&file_name)
                .status()
                .map_err(|_| (span, "failed to convert file to png"))?;
            fs::remove_file(&file_ppm)
                .map_err(|_| (span, "couldn't remove tmp file"))?;
            Ok("")
        },
        Expr::Num { span } => {
            Ok(lexer.span_str(span))
        },
        Expr::Axis { span } => {
            Ok(lexer.span_str(span))
        },
        Expr::File { span } => {
            Ok(lexer.span_str(span))
        },
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
        match eval(&lexer, r, &mut trans, &mut edges) {
            Ok(_) => eprintln!("script ran"),
            Err((span, msg)) => {
                let ((line, col), _) = lexer.line_col(span);
                eprintln!("Error parsing scriptat line {} column {}, '{}' {}.",
                          line,
                          col,
                          lexer.span_str(span),
                          msg,
                          );
            }
        }
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
