use std::cmp::PartialEq;
use std::process;
use std::io::Write;
use std::fs::{self, File};
use std::path::PathBuf;

use crate::{Color, Engine, TransformConstants};
use binrw::{BinRead, NullString};

#[derive(BinRead, PartialEq, Debug)]
enum Knob {
    #[br(magic = 0x0u8)]
    Constant,
    #[br(magic = 0x1u8)]
    Name(NullString),
}

trait Run {
    fn run<T: Color>(&self, engine: &mut Engine<T>);
}

/// knobular in you can put a knob value on it
trait Knobular {
    fn go<T: Color>(&self, engine: &mut Engine<T>, frame: u32);
}

#[derive(BinRead, PartialEq, Debug)]
struct PushCommand { }

impl Run for PushCommand {
    fn run<T: Color>(&self, engine: &mut Engine<T>) {
        engine.push_sys();
    }
}

#[derive(BinRead, PartialEq, Debug)]
struct PopCommand { }

impl Run for PopCommand {
    fn run<T: Color>(&self, engine: &mut Engine<T>) {
        engine.pop_sys();
    }
}

#[derive(BinRead, PartialEq, Debug)]
struct MoveCommand {
    x: f64,
    y: f64,
    z: f64,
    knob: Knob,
}

impl Run for MoveCommand {
    fn run<T: Color>(&self, engine: &mut Engine<T>) {
        engine.move_sys(self.x, self.y, self.z);
    }
}

impl Knobular for MoveCommand {
    fn go<T: Color>(&self, engine: &mut Engine<T>, frame: u32) {
        let v = match self.knob {
            Knob::Constant => 1.0,
            Knob::Name(ref name) => engine.get_knob_val(name.to_string(), frame),
        };

        engine.move_sys(self.x * v, self.y * v, self.z * v);
    }
}

#[derive(BinRead, PartialEq, Debug)]
struct RotateCommand {
    axis: f64,
    theta: f64,
    knob: Knob,
}

impl Run for RotateCommand {
    fn run<T: Color>(&self, engine: &mut Engine<T>) {
        engine.rotate_sys(self.axis, self.theta);
    }
}

impl Knobular for RotateCommand {
    fn go<T: Color>(&self, engine: &mut Engine<T>, frame: u32) {
        let v = match self.knob {
            Knob::Constant => 1.0,
            Knob::Name(ref name) => engine.get_knob_val(name.to_string(), frame),
        };

        engine.rotate_sys(self.axis, self.theta * v);
    }
}

#[derive(BinRead, PartialEq, Debug)]
struct ScaleCommand {
    x: f64,
    y: f64,
    z: f64,
    knob: Knob,
}

impl Run for ScaleCommand {
    fn run<T: Color>(&self, engine: &mut Engine<T>) {
        engine.scale_sys(self.x, self.y, self.z);
    }
}

impl Knobular for ScaleCommand {
    fn go<T: Color>(&self, engine: &mut Engine<T>, frame: u32) {
        let v = match self.knob {
            Knob::Constant => 1.0,
            Knob::Name(ref name) => engine.get_knob_val(name.to_string(), frame),
        };

        engine.scale_sys(self.x * v, self.y * v, self.z * v);
    }
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

impl Run for BoxCommand {
    fn run<T: Color>(&self, engine: &mut Engine<T>) {
        engine.set_constants(self.constants);
        engine.add_box((self.x, self.y, self.z), self.h, self.w, self.d);
        engine.apply_sys();
        engine.draw_space();
        engine.clear_lines();
        engine.clear_tris();
    }
}

#[derive(BinRead, PartialEq, Debug)]
struct SphereCommand {
    x: f64,
    y: f64,
    z: f64,
    r: f64,
    constants: [f64; 9],
}

impl Run for SphereCommand {
    fn run<T: Color>(&self, engine: &mut Engine<T>) {
        engine.set_constants(self.constants);
        engine.add_sphere((self.x, self.y, self.z), self.r);
        engine.apply_sys();
        engine.draw_space();
        engine.clear_lines();
        engine.clear_tris();
    }
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

impl Run for TorusCommand {
    fn run<T: Color>(&self, engine: &mut Engine<T>) {
        engine.set_constants(self.constants);
        engine.add_torus((self.x, self.y, self.z), self.r0, self.r1);
        engine.apply_sys();
        engine.draw_space();
        engine.clear_lines();
        engine.clear_tris();
    }
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

impl Run for LineCommand {
    fn run<T: Color>(&self, engine: &mut Engine<T>) {
        engine.add_line((self.x0, self.y0, self.z0), (self.x1, self.y1, self.z1));
        engine.apply_sys();
        engine.draw_space();
        engine.clear_lines();
        engine.clear_tris();
    }
}

#[derive(BinRead, PartialEq, Debug)]
struct SaveCommand {
    file: NullString,
}

impl Run for SaveCommand {
    fn run<T: Color>(&self, engine: &mut Engine<T>) {
        let file_name = self.file.to_string();
        let file_ppm = format!(".tmp_convertfilelhfgfhgf{}.ppm", file_name);
        let path = PathBuf::from(&file_ppm);
        let mut file = File::create(&path).expect("failed to create file path");
        engine.write_binary_ppm(&mut file);
        process::Command::new("convert")
            .arg(&file_ppm)
            .arg(&file_name)
            .status()
            .expect("failed to convert tmp file to png");
        fs::remove_file(&file_ppm).expect("failed to remove tmp ppm file");
    }
}

#[derive(BinRead, PartialEq, Debug)]
struct DisplayCommand { }

impl Run for DisplayCommand {
    fn run<T: Color>(&self, engine: &mut Engine<T>) {
        let mut display_command = process::Command::new("display")
            .stdin(process::Stdio::piped())
            .spawn()
            .expect("failed to display image with image magick display");

        display_command
            .stdin
            .as_mut()
            .expect("failed to display image with image magick display")
            .write_all(&engine.ppm_byte_vec())
            .expect("failed to display image with image magick display");
        display_command.wait().expect("failed to display image with image magick display");
    }
}

#[derive(BinRead, PartialEq, Debug)]
struct BasenameCommand {
    basename: NullString,
}

#[derive(BinRead, PartialEq, Debug)]
struct FramesCommand {
    frames: u32,
}

#[derive(BinRead, PartialEq, Debug)]
struct VaryCommand {
    knob: NullString,
    start_frame: u32,
    end_frame: u32,
    start_val: f64,
    end_val: f64,
}

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
    #[br(magic = 0xCu8)]
    Basename(BasenameCommand),
    #[br(magic = 0xDu8)]
    Frames(FramesCommand),
    #[br(magic = 0xEu8)]
    Vary(VaryCommand),
    #[br(magic = 0x0u8)]
    End,
}

#[derive(BinRead, Debug)]
pub struct Script {
    #[br(parse_with = binrw::until(|com| *com == Command::End))]
    commands: Vec<Command>,
}

impl Script {
    pub fn exec<T: Color>(self, eng: &mut Engine<T>) {
        // if there are more than one frames/basenames commands, we take the last one stated
        let has_frames = self.commands.iter().rev().find(|x| matches!(x, Command::Frames(_)));
        let has_basename = self.commands.iter().rev().find(|x| matches!(x, Command::Basename(_)));
        let has_vary = self.commands.iter().any(|x| matches!(x, Command::Vary(_)));

        if has_vary && has_frames == None {
            panic!("this is all a terrible misunderstanding, 
            you need a frames command with your vary");
        }

        if has_frames != None && has_basename == None {
            println!("woah there, you either forgot to put a basename or want to use the default,
            be sure that is actually what you want buddy");
        }

        let basename = match has_basename {
            None => "generic_mdl_animation".to_string(),
            Some(s) => if let Command::Basename(BasenameCommand { basename }) = s {
                basename.to_string()
            } else {
                "generic_mdl_animation".to_string()
            }
        };

        if let Some(frames_command) = has_frames {
            //create an animation
            let frames;
            if let &Command::Frames(FramesCommand {frames: x}) = frames_command {
                frames = x;
            } else {
                panic!("bad bad very bad, I literly filtered for only frame commands");
            }
            for v in self.commands.iter().filter(|x| matches!(x, Command::Vary(_))) {
                if let Command::Vary(vc) = v {
                    let VaryCommand { knob, start_frame, end_frame, start_val, end_val} = vc;
                    eng.add_vary(knob.to_string(), *start_frame, *end_frame, *start_val, *end_val, frames);
                } else {
                    panic!("bad bad very bad, I literly filtered for only vary commands");
                }
            }

            for f in 0..frames {
                for com in self.commands.iter() {
                    match com {
                        Command::Push(c) => c.run(eng),
                        Command::Pop(c) => c.run(eng),
                        Command::Move(c) => c.go(eng, f),
                        Command::Rotate(c) => c.go(eng, f),
                        Command::Scale(c) => c.go(eng, f),
                        Command::Box(c) => c.run(eng),
                        Command::Sphere(c) => c.run(eng),
                        Command::Torus(c) => c.run(eng),
                        Command::Line(c) => c.run(eng),
                        Command::Save(_) => (),
                        Command::Display(_) => (),
                        Command::Basename(_) => (),
                        Command::Frames(_) => (),
                        Command::Vary(_) => (),
                        Command::End => (),
                    }
                }
                let file_name = format!("{}{:0>8}.png", basename, f);
                let file_ppm = format!(".tmp_convertfilelhfgfhgf{}.ppm", file_name);
                let path = PathBuf::from(&file_ppm);
                let mut file = File::create(&path).expect("failed to create file path");
                eng.write_binary_ppm(&mut file);
                process::Command::new("convert")
                    .arg(&file_ppm)
                    .arg(&file_name)
                    .status()
                    .expect("failed to convert tmp file to png");
                fs::remove_file(&file_ppm).expect("failed to remove tmp ppm file");

                eng.clear_tris();
                eng.clear_lines();
                eng.clear_screen();
                eng.clear_stack();
            }

            //TODO:make the fps not stuck at 60
            process::Command::new("convert")
                .arg(format!("{}*.png", basename))
                .arg(format!("{}.gif", basename))
                .arg("-delay 1.7")
                .status()
                .expect("failed to convert files to gif");
            fs::remove_file(format!("{}*.png", basename)).expect("failed to remove created files");
        } else {
            //create a still image
            for com in self.commands {
                match com {
                    Command::Push(c) => c.run(eng),
                    Command::Pop(c) => c.run(eng),
                    Command::Move(c) => c.run(eng),
                    Command::Rotate(c) => c.run(eng),
                    Command::Scale(c) => c.run(eng),
                    Command::Box(c) => c.run(eng),
                    Command::Sphere(c) => c.run(eng),
                    Command::Torus(c) => c.run(eng),
                    Command::Line(c) => c.run(eng),
                    Command::Save(c) => c.run(eng),
                    Command::Display(c) => c.run(eng),
                    Command::Basename(_) => println!("you sure you want a basename in this script?"),
                    Command::Frames(_) => panic!("bad bad very bad this should be impossible"),
                    Command::Vary(_) => panic!("bad bad very bad this should be impossible"),
                    Command::End => (),
                }
            }
        }
    }
}
