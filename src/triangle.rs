//! Represents a triangle described by its 3 corners.

use point2f::Point2f;

#[cfg(all(windows, feature = "d2d"))]
use winapi::um::d2d1::D2D1_TRIANGLE;

/// Represents a triangle described by its 3 corners.
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Triangle {
    /// The first point
    pub p1: Point2f,
    /// The second point
    pub p2: Point2f,
    /// The third point
    pub p3: Point2f,
}

impl<P1, P2, P3> From<(P1, P2, P3)> for Triangle
where
    P1: Into<Point2f>,
    P2: Into<Point2f>,
    P3: Into<Point2f>,
{
    #[inline]
    fn from((p1, p2, p3): (P1, P2, P3)) -> Triangle {
        Triangle {
            p1: p1.into(),
            p2: p2.into(),
            p3: p3.into(),
        }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<Triangle> for D2D1_TRIANGLE {
    #[inline]
    fn from(seg: Triangle) -> D2D1_TRIANGLE {
        D2D1_TRIANGLE {
            point1: seg.p1.into(),
            point2: seg.p2.into(),
            point3: seg.p3.into(),
        }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<D2D1_TRIANGLE> for Triangle {
    #[inline]
    fn from(seg: D2D1_TRIANGLE) -> Triangle {
        Triangle {
            p1: seg.point1.into(),
            p2: seg.point2.into(),
            p3: seg.point3.into(),
        }
    }
}
