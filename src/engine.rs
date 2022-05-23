use std::fs::File;

use crate::{Modtrix, Color, Space, Screen, Light};

pub struct Engine<T: Color> {
    stack: Vec<Modtrix>,
    space: Space<T>,
    screen: Screen<T>,
}

impl<T: Color> Engine<T> {
    ///creates an an engine housing a screen with given size
    pub fn with_screen_dims(screen_width: usize, screen_height: usize) -> Self {
        Self {
            stack: vec![Modtrix::IDENT],
            space: Space::new(),
            screen: Screen::<T>::with_size(screen_width, screen_height),
        }
    }

    ///sets the constants of the space, takes an array of nine f64 values:
    ///[Ka_r, Ka_g, Ka_b, Kd_r, Kd_g, Kd_b, Ks_r, Ks_g, Ks_b]
    pub fn set_constants(&mut self, constants: [f64; 9]) {
        self.space.set_ambient_reflection((constants[0], constants[1], constants[2]));
        self.space.set_diffuse_reflection((constants[3], constants[4], constants[5]));
        self.space.set_specular_reflection((constants[6], constants[7], constants[8]));
    }

    pub fn push_sys(&mut self) {
        self.stack.push(self.stack.last().unwrap().clone());
    }

    pub fn pop_sys(&mut self) {
        self.stack.pop();
    }

    pub fn move_sys(&mut self, x: f64, y: f64, z: f64) {
        let mm = crate::move_matrix!(x, y, z);
        Modtrix::multr(self.stack.last_mut().unwrap(), &mm);
    }

    pub fn rotate_sys(&mut self, axis: f64, theta: f64) {
        let rm = if axis == 0.0 {
            crate::rotx_matrix!(theta)
        } else if axis == 1.0 {
            crate::roty_matrix!(theta)
        } else if axis == 2.0 {
            crate::rotz_matrix!(theta)
        } else{
            panic!("attempt to rotate by invalid axis");
        };

        Modtrix::multr(self.stack.last_mut().unwrap(), &rm);
    }

    pub fn scale_sys(&mut self, x: f64, y: f64, z: f64) {
        let sm = crate::scale_matrix!(x, y, z);
        Modtrix::multr(self.stack.last_mut().unwrap(), &sm);
    }

    pub fn add_box(&mut self, (x, y, z): (f64, f64, f64), h: f64, w: f64, d: f64) {
        crate::add_box(x, y, z, w, h, d, &mut self.space);
    }

    pub fn add_sphere(&mut self, (x, y, z): (f64, f64, f64), r: f64) {
        crate::add_sphere(x, y, z, r, &mut self.space)
    }

    pub fn add_torus(&mut self, (x, y, z): (f64, f64, f64), r0: f64, r1: f64) {
        crate::add_torus(x, y, z, r0, r1, &mut self.space)
    }

    pub fn add_line(&mut self, p: (f64, f64, f64), q: (f64, f64, f64)) {
        crate::add_line(p, q, &mut self.space)
    }

    pub fn draw_space(&mut self) {
        crate::space::draw_space(&mut self.space, &mut self.screen);
    }

    pub fn clear_lines(&mut self) {
        self.space.clear_lines();
    }

    pub fn clear_tris(&mut self) {
        self.space.clear_tris();
    }

    pub fn apply_sys(&mut self) {
        self.space.apply(&mut self.stack.last().unwrap());
    }

    pub fn ppm_byte_vec(&self) -> Vec<u8> {
        self.screen.byte_vec()
    }

    pub fn write_binary_ppm(&self, file: &mut File) {
        self.screen.write_binary_ppm(file).expect("failed to write binary ppm");
    }

    pub fn add_light(&mut self, light: Light<T>) {
        self.space.add_light(light);
    }

    pub fn set_camera(&mut self, p: (f64, f64, f64)) {
        self.space.set_camera(p);
    }

    pub fn set_ambient_light(&mut self, color: T) {
        self.space.set_ambient_light(color);
    }
}
