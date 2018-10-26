//! Mathematical point on the 2D (x, y) plane.

use point2i::Point2i;
use point2u::Point2u;
use vector2f::Vector2f;

use std::ops::{Add, Sub};

#[cfg(all(windows, feature = "d2d"))]
use winapi::um::dcommon::D2D_POINT_2F;

/// Mathematical point on the 2D (x, y) plane.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Point2f {
    /// Horizontal component
    pub x: f32,
    /// Vertical component
    pub y: f32,
}

/// Mathematical origin point on the real number plane.
pub const ORIGIN: Point2f = Point2f::ORIGIN;

impl Point2f {
    /// Mathematical origin point on the real number plane.
    pub const ORIGIN: Point2f = Point2f { x: 0.0, y: 0.0 };

    /// Construct a point from the components
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Point2f { x, y }
    }

    /// Casts the point to an integer point. Will truncate integers; if
    /// another behavior is desired it should be manually performed on
    /// the values before calling this function.
    #[inline]
    pub fn to_i32(self) -> Point2i {
        Point2i {
            x: self.x as i32,
            y: self.y as i32,
        }
    }

    /// Casts the point to an unsigned integer point. Will truncate integers;
    /// if another behavior is desired it should be manually performed on
    /// the values before calling this function. Beware this function will
    /// not produce reasonable values if the current value is negative.
    #[inline]
    pub fn to_u32(self) -> Point2u {
        Point2u {
            x: self.x as u32,
            y: self.y as u32,
        }
    }

    /// Rounds the values in the point to the nearest integer, rounding away
    /// from zero in the half-way case.
    /// 
    /// See [f32::round][1]
    /// 
    /// [1]: https://doc.rust-lang.org/std/primitive.f32.html#method.round
    #[inline]
    pub fn rounded(self) -> Point2f {
        Point2f {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    /// Determines if the components of two points are less than `epsilon`
    /// distance from each other. Be wary that this does not check the actual
    /// distance, but a component-wise distance check. If you desire a more
    /// precise distance check, consider subtracting one point from the other
    /// and comparing the length(_sq) of the resulting vector.
    #[inline]
    pub fn is_approx_eq(self, other: impl Into<Point2f>, epsilon: f32) -> bool {
        let other = other.into();
        return (self.x - other.x).abs() <= epsilon && (self.y - other.y).abs() <= epsilon;
    }
}

impl<V> Add<V> for Point2f
where
    V: Into<Vector2f>,
{
    type Output = Point2f;

    #[inline]
    fn add(self, rhs: V) -> Point2f {
        let rhs = rhs.into();
        Point2f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
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

impl Sub<(f32, f32)> for Point2f {
    type Output = Vector2f;

    #[inline]
    fn sub(self, rhs: (f32, f32)) -> Vector2f {
        Vector2f {
            x: self.x - rhs.0,
            y: self.y - rhs.1,
        }
    }
}

impl Sub<Point2f> for (f32, f32) {
    type Output = Vector2f;

    #[inline]
    fn sub(self, rhs: Point2f) -> Vector2f {
        Vector2f {
            x: self.0 - rhs.x,
            y: self.1 - rhs.y,
        }
    }
}

impl<V> Sub<V> for Point2f
where
    V: Into<Vector2f>,
{
    type Output = Point2f;

    #[inline]
    fn sub(self, rhs: V) -> Point2f {
        let rhs = rhs.into();
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

impl From<[f32; 2]> for Point2f {
    #[inline]
    fn from(p: [f32; 2]) -> Point2f {
        Point2f { x: p[0], y: p[0] }
    }
}

impl From<Point2f> for [f32; 2] {
    #[inline]
    fn from(p: Point2f) -> [f32; 2] {
        [p.x, p.y]
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

#[cfg(feature = "mint")]
impl From<Point2f> for mint::Point2<f32> {
    #[inline]
    fn from(p: Point2f) -> mint::Point2<f32> {
        mint::Point2 { x: p.x, y: p.y }
    }
}

#[cfg(feature = "mint")]
impl From<mint::Point2<f32>> for Point2f {
    #[inline]
    fn from(p: mint::Point2<f32>) -> Point2f {
        Point2f { x: p.x, y: p.y }
    }
}

#[cfg(all(test, windows, feature = "d2d"))]
#[test]
fn pt2f_d2d_bin_compat() {
    use std::mem::size_of_val;

    fn ptr_eq<T>(a: &T, b: &T) -> bool {
        (a as *const T) == (b as *const T)
    }

    let pt = Point2f::ORIGIN;
    let d2d = unsafe { &*((&pt) as *const _ as *const D2D_POINT_2F) };

    assert!(ptr_eq(&pt.x, &d2d.x));
    assert!(ptr_eq(&pt.y, &d2d.y));
    assert_eq!(size_of_val(&pt), size_of_val(d2d));
}
