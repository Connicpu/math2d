#![cfg_attr(feature = "docs", warn(missing_docs))]
//! 2D Mathematics library designed for use with 2D drawing applications.
//! Primarily designed for the needs of Direct2D, but this library should
//! be perfectly capable of filling in the needs of other libraries such
//! as Cairo. If you would like interoperability defitions added please feel
//! free to open a pull request on the [repository][1]
//! 
//! [1]: https://github.com/connicpu/math2d

#[cfg(all(feature = "serde", feature = "serde_derive"))]
#[macro_use]
extern crate serde_derive;
#[cfg(all(feature = "serde", feature = "serde_derive"))]
extern crate serde;

#[cfg(all(windows, feature = "winapi"))]
extern crate winapi;

#[cfg(feature = "mint")]
extern crate mint;

pub use arc_segment::{ArcSegment, ArcSize, SweepDirection};
pub use bezier_segment::BezierSegment;
pub use color::Color;
pub use ellipse::Ellipse;
pub use matrix3x2f::Matrix3x2f;
pub use point2f::Point2f;
pub use point2i::Point2i;
pub use point2u::Point2u;
pub use quad_bezier_segment::QuadBezierSegment;
pub use rectf::{RectCorner, Rectf};
pub use recti::Recti;
pub use rectu::Rectu;
pub use rounded_rect::RoundedRect;
pub use sizef::Sizef;
pub use sizeu::Sizeu;
pub use thicknessf::Thicknessf;
pub use triangle::Triangle;
pub use vector2f::Vector2f;
pub use vector2i::Vector2i;

pub mod arc_segment;
pub mod bezier_segment;
pub mod color;
pub mod ellipse;
pub mod matrix3x2f;
pub mod point2f;
pub mod point2i;
pub mod point2u;
pub mod quad_bezier_segment;
pub mod rectf;
pub mod recti;
pub mod rectu;
pub mod rounded_rect;
pub mod sizef;
pub mod sizeu;
pub mod thicknessf;
pub mod triangle;
pub mod vector2f;
pub mod vector2i;
