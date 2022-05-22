use crate::{Modtrix, Color, Space, Screen};
pub struct Engine<T: Color> {
    stack: Vec<Modtrix>,
    space: Space<T>,
    screen: Screen<T>,
}

impl<T: Color> Engine<T> {

}
