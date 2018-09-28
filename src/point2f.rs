use crate::vector2f::Vector2f;
use crate::point2i::Point2i;

use std::ops::{Add, Sub};

#[cfg(all(windows, feature = "d2d"))]
use winapi::um::dcommon::D2D_POINT_2F;

/// Mathematical point on the 2D (x, y) plane
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
pub struct Point2f {
    pub x: f32,
    pub y: f32,
}

pub const ORIGIN: Point2f = Point2f::ORIGIN;

impl Point2f {
    /// Mathematical origin point on the real number plane
    pub const ORIGIN: Point2f = Point2f { x: 0.0, y: 0.0 };

    /// Construct a point from the components
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Point2f { x, y }
    }

    #[inline]
    pub fn to_i32(self) -> Point2i {
        Point2i {
            x: self.x as i32,
            y: self.y as i32,
        }
    }

    #[inline]
    pub fn rounded(self) -> Point2f {
        Point2f {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    #[inline]
    pub fn is_approx_eq(self, other: impl Into<Point2f>, epsilon: f32) -> bool {
        let other = other.into();
        return (self.x - other.x).abs() <= epsilon
            && (self.y - other.y).abs() <= epsilon;
    }
}

impl Add<Vector2f> for Point2f {
    type Output = Point2f;

    #[inline]
    fn add(self, rhs: Vector2f) -> Point2f {
        Point2f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Point2f> for Vector2f {
    type Output = Point2f;

    #[inline]
    fn add(self, rhs: Point2f) -> Point2f {
        rhs + self
    }
}

impl Sub for Point2f {
    type Output = Vector2f;
    
    #[inline]
    fn sub(self, rhs: Point2f) -> Vector2f {
        Vector2f {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<Vector2f> for Point2f {
    type Output = Point2f;

    #[inline]
    fn sub(self, rhs: Vector2f) -> Point2f {
        Point2f {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<(f32, f32)> for Point2f {
    #[inline]
    fn from((x, y): (f32, f32)) -> Point2f {
        Point2f { x, y }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<Point2f> for D2D_POINT_2F {
    #[inline]
    fn from(point: Point2f) -> D2D_POINT_2F {
        D2D_POINT_2F {
            x: point.x,
            y: point.y,
        }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<D2D_POINT_2F> for Point2f {
    #[inline]
    fn from(point: D2D_POINT_2F) -> Point2f {
        Point2f {
            x: point.x,
            y: point.y,
        }
    }
}
