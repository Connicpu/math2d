//! BezierSegment represents a curved line in a Path shaped as a
//! cubic bezier segment i.e. a bezier line segment with 4 points,
//! the two center ones acting as control points.

use point2f::Point2f;

#[cfg(all(windows, feature = "d2d"))]
use winapi::um::d2d1::D2D1_BEZIER_SEGMENT;

/// Represents a cubic bezier segment drawn between two points. The first point
/// in the bezier segment is implicitly the end point of the previous segment.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct BezierSegment {
    /// The first control point
    pub p1: Point2f,
    /// The second control point
    pub p2: Point2f,
    /// The end point
    pub p3: Point2f,
}

impl BezierSegment {
    /// Construct the segment from its parts, conveniently converting
    /// types like float tuples into points.
    #[inline]
    pub fn new(
        p1: impl Into<Point2f>,
        p2: impl Into<Point2f>,
        p3: impl Into<Point2f>,
    ) -> BezierSegment {
        BezierSegment {
            p1: p1.into(),
            p2: p2.into(),
            p3: p3.into(),
        }
    }
}

impl<P1, P2, P3> From<(P1, P2, P3)> for BezierSegment
where
    P1: Into<Point2f>,
    P2: Into<Point2f>,
    P3: Into<Point2f>,
{
    #[inline]
    fn from((p1, p2, p3): (P1, P2, P3)) -> BezierSegment {
        BezierSegment {
            p1: p1.into(),
            p2: p2.into(),
            p3: p3.into(),
        }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<BezierSegment> for D2D1_BEZIER_SEGMENT {
    #[inline]
    fn from(seg: BezierSegment) -> D2D1_BEZIER_SEGMENT {
        D2D1_BEZIER_SEGMENT {
            point1: seg.p1.into(),
            point2: seg.p2.into(),
            point3: seg.p3.into(),
        }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<D2D1_BEZIER_SEGMENT> for BezierSegment {
    #[inline]
    fn from(seg: D2D1_BEZIER_SEGMENT) -> BezierSegment {
        BezierSegment {
            p1: seg.point1.into(),
            p2: seg.point2.into(),
            p3: seg.point3.into(),
        }
    }
}

#[cfg(all(test, windows, feature = "d2d"))]
#[test]
fn bezier_d2d_bin_compat() {
    use std::mem::size_of_val;

    fn ptr_eq<T>(a: &T, b: &T) -> bool {
        (a as *const T) == (b as *const T)
    }

    let bez = BezierSegment::new((0.0, 0.0), (1.0, 0.0), (1.0, 1.0));
    let d2d = unsafe { &*((&bez) as *const _ as *const D2D1_BEZIER_SEGMENT) };

    assert!(ptr_eq(&bez.p1.x, &d2d.point1.x));
    assert!(ptr_eq(&bez.p1.y, &d2d.point1.y));
    assert!(ptr_eq(&bez.p2.x, &d2d.point2.x));
    assert!(ptr_eq(&bez.p2.y, &d2d.point2.y));
    assert!(ptr_eq(&bez.p3.x, &d2d.point3.x));
    assert!(ptr_eq(&bez.p3.y, &d2d.point3.y));
    assert_eq!(size_of_val(&bez), size_of_val(d2d));
}
