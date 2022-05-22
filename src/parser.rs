use std::cmp::PartialEq;

use crate::{Color, Engine};
use binrw::{BinRead, NullString};

trait Run {
    fn run<T: Color>(self, engine: Engine<T>);
}

#[derive(BinRead, PartialEq, Debug)]
struct PushCommand { }

#[derive(BinRead, PartialEq, Debug)]
struct PopCommand { }

#[derive(BinRead, PartialEq, Debug)]
struct MoveCommand {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(BinRead, PartialEq, Debug)]
struct RotateCommand {
    axis: f64,
    theta: f64,
}

#[derive(BinRead, PartialEq, Debug)]
struct ScaleCommand {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(BinRead, PartialEq, Debug)]
struct BoxCommand {
    x: f64,
    y: f64,
    z: f64,
    h: f64,
    w: f64,
    d: f64,
    constants: [f64; 9],
}

#[derive(BinRead, PartialEq, Debug)]
struct SphereCommand {
    x: f64,
    y: f64,
    z: f64,
    r: f64,
    constants: [f64; 9],
}

#[derive(BinRead, PartialEq, Debug)]
struct TorusCommand {
    x: f64,
    y: f64,
    z: f64,
    r0: f64,
    r1: f64,
    constants: [f64; 9],
}

#[derive(BinRead, PartialEq, Debug)]
struct LineCommand {
    x0: f64,
    y0: f64,
    z0: f64,
    x1: f64,
    y1: f64,
    z1: f64,
}

#[derive(BinRead, PartialEq, Debug)]
struct SaveCommand {
    file: NullString,
}

#[derive(BinRead, PartialEq, Debug)]
struct DisplayCommand { }

#[derive(BinRead, PartialEq, Debug)]
enum Command {
    #[br(magic = 0x1u8)]
    Push(PushCommand),
    #[br(magic = 0x2u8)]
    Pop(PopCommand),
    #[br(magic = 0x3u8)]
    Move(MoveCommand),
    #[br(magic = 0x4u8)]
    Rotate(RotateCommand),
    #[br(magic = 0x5u8)]
    Scale(ScaleCommand),
    #[br(magic = 0x6u8)]
    Box(BoxCommand),
    #[br(magic = 0x7u8)]
    Sphere(SphereCommand),
    #[br(magic = 0x8u8)]
    Torus(TorusCommand),
    #[br(magic = 0x9u8)]
    Line(LineCommand),
    #[br(magic = 0xAu8)]
    Save(SaveCommand),
    #[br(magic = 0xBu8)]
    Display(DisplayCommand),
    #[br(magic = 0x0u8)]
    End,
}

#[derive(BinRead, Debug)]
pub struct Script {
    #[br(parse_with = binrw::until(|com| *com == Command::End))]
    commands: Vec<Command>,
}
