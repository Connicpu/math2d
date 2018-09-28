use crate::vector2i::Vector2i;
use crate::point2f::Point2f;

use std::ops::{Add, Sub};

#[cfg(all(windows, feature = "d2d"))]
use winapi::um::dcommon::D2D_POINT_2L;

#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
pub struct Point2i {
    pub x: i32,
    pub y: i32,
}

impl Point2i {
    pub const ORIGIN: Point2i = Point2i { x: 0, y: 0 };

    #[inline]
    pub fn new(x: i32, y: i32) -> Point2i {
        Point2i { x, y }
    }

    #[inline]
    pub fn to_f32(self) -> Point2f {
        Point2f {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

impl From<(i32, i32)> for Point2i {
    #[inline]
    fn from((x, y): (i32, i32)) -> Point2i {
        Point2i { x, y }
    }
}

impl Add<Vector2i> for Point2i {
    type Output = Point2i;

    #[inline]
    fn add(self, rhs: Vector2i) -> Point2i {
        Point2i {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Point2i> for Vector2i {
    type Output = Point2i;

    #[inline]
    fn add(self, rhs: Point2i) -> Point2i {
        rhs + self
    }
}

impl Sub for Point2i {
    type Output = Vector2i;
    
    #[inline]
    fn sub(self, rhs: Point2i) -> Vector2i {
        Vector2i {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<Vector2i> for Point2i {
    type Output = Point2i;

    #[inline]
    fn sub(self, rhs: Vector2i) -> Point2i {
        Point2i {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<Point2i> for D2D_POINT_2L {
    #[inline]
    fn from(point: Point2i) -> D2D_POINT_2L {
        D2D_POINT_2L {
            x: point.x,
            y: point.y,
        }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<D2D_POINT_2L> for Point2i {
    #[inline]
    fn from(point: D2D_POINT_2L) -> Point2i {
        Point2i {
            x: point.x,
            y: point.y,
        }
    }
}
