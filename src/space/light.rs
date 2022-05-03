///A light (well technically a point linght source) but just called a light for simplicity becuase
///an ambient light just a single number and I don't think there are any other types of lights.
use std::fmt::Debug;
use super::Point;
use crate::screen::Color;

#[derive(Copy, Clone, Debug)]
pub struct Light<T: Color> {
    pub pos: Point,
    pub col: T,
}

impl<T: Color> Light<T> {
    pub fn new(pos: Point, col: T) -> Self {
        Self {
            pos,
            col,
        }
    }
}
