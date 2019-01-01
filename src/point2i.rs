//! Mathematical point on the 2D (x, y) plane.

use crate::point2f::Point2f;
use crate::point2u::Point2u;
use crate::vector2i::Vector2i;

use std::ops::{Add, AddAssign, Sub, SubAssign};

#[cfg(all(windows, feature = "d2d"))]
use winapi::um::dcommon::D2D_POINT_2L;

/// Mathematical point on the 2D (x, y) plane.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Point2i {
    /// Horizontal component
    pub x: i32,
    /// Vertical component
    pub y: i32,
}

impl Point2i {
    /// Mathematical origin point
    pub const ORIGIN: Point2i = Point2i { x: 0, y: 0 };

    /// Construct a point from its components
    #[inline]
    pub fn new(x: i32, y: i32) -> Point2i {
        Point2i { x, y }
    }

    /// Convert this value to a floating point
    #[inline]
    pub fn to_f32(self) -> Point2f {
        Point2f {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    /// Convert this value to an unsigned point. It is the caller's duty
    /// to ensure the values are not negative to avoid casting underflow.
    #[inline]
    pub fn to_u32(self) -> Point2u {
        Point2u {
            x: self.x as u32,
            y: self.y as u32,
        }
    }
}

impl<V> Add<V> for Point2i
where
    V: Into<Vector2i>,
{
    type Output = Point2i;

    #[inline]
    fn add(self, rhs: V) -> Point2i {
        let rhs = rhs.into();
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

impl Add<(i32, i32)> for Vector2i {
    type Output = Point2i;

    #[inline]
    fn add(self, rhs: (i32, i32)) -> Point2i {
        Point2i::from(rhs) + self
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

impl Sub<(i32, i32)> for Point2i {
    type Output = Vector2i;

    #[inline]
    fn sub(self, rhs: (i32, i32)) -> Vector2i {
        Vector2i {
            x: self.x - rhs.0,
            y: self.y - rhs.0,
        }
    }
}

impl<V> Sub<V> for Point2i
where
    V: Into<Vector2i>,
{
    type Output = Point2i;

    #[inline]
    fn sub(self, rhs: V) -> Point2i {
        let rhs = rhs.into();
        Point2i {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<V> AddAssign<V> for Point2i
where
    Point2i: Add<V, Output = Point2i>,
{
    fn add_assign(&mut self, v: V) {
        *self = *self + v;
    }
}

impl<V> SubAssign<V> for Point2i
where
    Point2i: Sub<V, Output = Point2i>,
{
    fn sub_assign(&mut self, v: V) {
        *self = *self - v;
    }
}

impl From<(i32, i32)> for Point2i {
    #[inline]
    fn from((x, y): (i32, i32)) -> Point2i {
        Point2i { x, y }
    }
}

impl From<Point2i> for (i32, i32) {
    #[inline]
    fn from(p: Point2i) -> (i32, i32) {
        (p.x, p.y)
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

#[cfg(feature = "mint")]
impl From<Point2i> for mint::Point2<i32> {
    #[inline]
    fn from(p: Point2i) -> mint::Point2<i32> {
        mint::Point2 { x: p.x, y: p.y }
    }
}

#[cfg(feature = "mint")]
impl From<mint::Point2<i32>> for Point2i {
    #[inline]
    fn from(p: mint::Point2<i32>) -> Point2i {
        Point2i { x: p.x, y: p.y }
    }
}

#[cfg(all(test, windows, feature = "d2d"))]
#[test]
fn pt2i_d2d_bin_compat() {
    use std::mem::size_of_val;

    fn ptr_eq<T>(a: &T, b: &T) -> bool {
        (a as *const T) == (b as *const T)
    }

    let pt = Point2i::ORIGIN;
    let d2d = unsafe { &*((&pt) as *const _ as *const D2D_POINT_2L) };

    assert!(ptr_eq(&pt.x, &d2d.x));
    assert!(ptr_eq(&pt.y, &d2d.y));
    assert_eq!(size_of_val(&pt), size_of_val(d2d));
}
