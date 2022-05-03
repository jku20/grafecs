//!a collection of structs dealing with spaces to put shapes in.

mod modtrix;
pub use modtrix::Modtrix;

mod space;
pub use space::{draw_space, Space};

mod light;
pub use light::Light;

//when Float is updated, make sure to update the below three lines as well
pub type Float = f64;

///The point is stored (x, y, z)
pub type Point = (Float, Float, Float);
