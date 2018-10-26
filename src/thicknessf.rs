//! Represents a margin around an axis-aligned rectangle.

use vector2f::Vector2f;

/// Represents a margin around an axis-aligned rectangle.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Thicknessf {
    /// Left x component
    pub left: f32,
    /// Top y component
    pub top: f32,
    /// Right x component
    pub right: f32,
    /// Bottom y component
    pub bottom: f32,
}

impl Thicknessf {
    /// Constructs the thickness from components.
    #[inline]
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Thicknessf {
        Thicknessf {
            left,
            top,
            right,
            bottom,
        }
    }
}

impl From<Vector2f> for Thicknessf {
    #[inline]
    fn from(vec: Vector2f) -> Thicknessf {
        (vec.x, vec.y).into()
    }
}

impl From<f32> for Thicknessf {
    #[inline]
    fn from(f: f32) -> Thicknessf {
        (f, f, f, f).into()
    }
}

impl From<(f32, f32)> for Thicknessf {
    #[inline]
    fn from((x, y): (f32, f32)) -> Thicknessf {
        (x, y, x, y).into()
    }
}

impl From<(f32, f32, f32, f32)> for Thicknessf {
    #[inline]
    fn from(values: (f32, f32, f32, f32)) -> Thicknessf {
        let (left, top, right, bottom) = values;
        Thicknessf {
            left,
            top,
            right,
            bottom,
        }
    }
}
