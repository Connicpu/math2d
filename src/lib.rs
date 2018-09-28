#[cfg(all(feature = "serde", feature = "serde_derive"))]
#[macro_use]
extern crate serde_derive;
#[cfg(all(feature = "serde", feature = "serde_derive"))]
extern crate serde;

#[cfg(all(windows, feature = "winapi"))]
extern crate winapi;

pub use crate::matrix3x2f::Matrix3x2f;
pub use crate::point2f::Point2f;
pub use crate::point2i::Point2i;
pub use crate::rectf::Rectf;
pub use crate::recti::Recti;
pub use crate::rectu::Rectu;
pub use crate::sizef::Sizef;
pub use crate::sizeu::Sizeu;
pub use crate::thicknessf::Thicknessf;
pub use crate::vector2f::Vector2f;
pub use crate::vector2i::Vector2i;

pub mod color;
pub mod matrix3x2f;
pub mod point2f;
pub mod point2i;
pub mod rectf;
pub mod recti;
pub mod rectu;
pub mod sizef;
pub mod sizeu;
pub mod thicknessf;
pub mod vector2f;
pub mod vector2i;

pub enum RectCorner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
