//!a collection of structs dealing with spaces to put shapes in.

mod light;
mod modtrix;
mod space;

pub use light::Light;
pub use modtrix::{move_matrix, rotx_matrix, roty_matrix, rotz_matrix, scale_matrix, Modtrix};
pub use space::{draw_space, Space};

//when Float is updated, make sure to update the below three lines as well
pub type Float = f64;

///The point is stored (x, y, z)
pub type Point = (Float, Float, Float);
