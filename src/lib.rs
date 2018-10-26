#![cfg_attr(feature = "docs", warn(missing_docs))]
//! 2D Mathematics library designed for use with 2D drawing applications.
//! 
//! Primarily designed for the needs of Direct2D, but this library should
//! be perfectly capable of filling in the needs of other libraries such
//! as Cairo. If you would like interoperability defitions added please feel
//! free to open a pull request on the [repository][1].
//! 
//! Currently compatible with:
//! - Direct2D (winapi types)
//! - [Mint][2]
//! 
//! [1]: https://github.com/connicpu/math2d
//! [2]: https://docs.rs/mint

#[cfg(all(feature = "serde", feature = "serde_derive"))]
#[macro_use]
extern crate serde_derive;
#[cfg(all(feature = "serde", feature = "serde_derive"))]
extern crate serde;

#[cfg(all(windows, feature = "winapi"))]
extern crate winapi;

#[cfg(feature = "mint")]
extern crate mint;

#[doc(inline)]
pub use arc_segment::{ArcSegment, ArcSize, SweepDirection};
#[doc(inline)]
pub use bezier_segment::BezierSegment;
pub use color::Color;
#[doc(inline)]
pub use ellipse::Ellipse;
#[doc(inline)]
pub use matrix3x2f::Matrix3x2f;
#[doc(inline)]
pub use point2f::Point2f;
#[doc(inline)]
pub use point2i::Point2i;
#[doc(inline)]
pub use point2u::Point2u;
#[doc(inline)]
pub use quad_bezier_segment::QuadBezierSegment;
#[doc(inline)]
pub use rectf::{RectCorner, Rectf};
#[doc(inline)]
pub use recti::Recti;
#[doc(inline)]
pub use rectu::Rectu;
#[doc(inline)]
pub use rounded_rect::RoundedRect;
#[doc(inline)]
pub use sizef::Sizef;
#[doc(inline)]
pub use sizeu::Sizeu;
#[doc(inline)]
pub use thicknessf::Thicknessf;
#[doc(inline)]
pub use triangle::Triangle;
#[doc(inline)]
pub use vector2f::Vector2f;
#[doc(inline)]
pub use vector2i::Vector2i;

#[doc(hidden)]
pub mod arc_segment;
#[doc(hidden)]
pub mod bezier_segment;
pub mod color;
#[doc(hidden)]
pub mod ellipse;
#[doc(hidden)]
pub mod matrix3x2f;
#[doc(hidden)]
pub mod point2f;
#[doc(hidden)]
pub mod point2i;
#[doc(hidden)]
pub mod point2u;
#[doc(hidden)]
pub mod quad_bezier_segment;
#[doc(hidden)]
pub mod rectf;
#[doc(hidden)]
pub mod recti;
#[doc(hidden)]
pub mod rectu;
#[doc(hidden)]
pub mod rounded_rect;
#[doc(hidden)]
pub mod sizef;
#[doc(hidden)]
pub mod sizeu;
#[doc(hidden)]
pub mod thicknessf;
#[doc(hidden)]
pub mod triangle;
#[doc(hidden)]
pub mod vector2f;
#[doc(hidden)]
pub mod vector2i;
