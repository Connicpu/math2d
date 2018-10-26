//! Mathematical point on the 2D (x, y) plane.

use point2f::Point2f;
use point2i::Point2i;

#[cfg(all(windows, feature = "d2d"))]
use winapi::um::dcommon::D2D_POINT_2U;

/// Mathematical point on the 2D (x, y) plane.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Point2u {
    /// Horizontal component
    pub x: u32,
    /// Vertical component
    pub y: u32,
}

impl Point2u {
    /// Mathematical origin point
    pub const ORIGIN: Point2u = Point2u { x: 0, y: 0 };

    /// Construct a point from its components
    #[inline]
    pub fn new(x: u32, y: u32) -> Point2u {
        Point2u { x, y }
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
    pub fn to_i32(self) -> Point2i {
        Point2i {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

impl From<(u32, u32)> for Point2u {
    #[inline]
    fn from((x, y): (u32, u32)) -> Point2u {
        Point2u { x, y }
    }
}

impl From<[u32; 2]> for Point2u {
    #[inline]
    fn from(p: [u32; 2]) -> Point2u {
        Point2u { x: p[0], y: p[0] }
    }
}

impl From<Point2u> for [u32; 2] {
    #[inline]
    fn from(p: Point2u) -> [u32; 2] {
        [p.x, p.y]
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<Point2u> for D2D_POINT_2U {
    #[inline]
    fn from(point: Point2u) -> D2D_POINT_2U {
        D2D_POINT_2U {
            x: point.x,
            y: point.y,
        }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<D2D_POINT_2U> for Point2u {
    #[inline]
    fn from(point: D2D_POINT_2U) -> Point2u {
        Point2u {
            x: point.x,
            y: point.y,
        }
    }
}

#[cfg(feature = "mint")]
impl From<Point2u> for mint::Point2<u32> {
    #[inline]
    fn from(p: Point2u) -> mint::Point2<u32> {
        mint::Point2 { x: p.x, y: p.y }
    }
}

#[cfg(feature = "mint")]
impl From<mint::Point2<u32>> for Point2u {
    #[inline]
    fn from(p: mint::Point2<u32>) -> Point2u {
        Point2u { x: p.x, y: p.y }
    }
}

#[cfg(all(test, windows, feature = "d2d"))]
#[test]
fn pt2u_d2d_bin_compat() {
    use std::mem::size_of_val;

    fn ptr_eq<T>(a: &T, b: &T) -> bool {
        (a as *const T) == (b as *const T)
    }

    let pt = Point2u::ORIGIN;
    let d2d = unsafe { &*((&pt) as *const _ as *const D2D_POINT_2U) };

    assert!(ptr_eq(&pt.x, &d2d.x));
    assert!(ptr_eq(&pt.y, &d2d.y));
    assert_eq!(size_of_val(&pt), size_of_val(d2d));
}
