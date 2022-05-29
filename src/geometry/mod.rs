pub use direction::{Direction, DIR8, DIR9};
pub use point::Point;
pub use two_dim_direction::{ConvertError, TwoDimDirection};

pub mod circles;
mod direction;
mod point;
mod two_dim_direction;

pub type Vec2 = tetra::math::Vec2<f32>;
pub type Rect = tetra::math::Rect<f32, f32>;
