//! Quadratic version of the BezierSegment, uses 1 fewer control point than
//! the cubic variant.

use point2f::Point2f;

#[cfg(all(windows, feature = "d2d"))]
use winapi::um::d2d1::D2D1_QUADRATIC_BEZIER_SEGMENT;

/// Contains the control point and end point for a quadratic Bezier segment.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct QuadBezierSegment {
    /// The control point of the quadratic Bezier segment.
    pub p1: Point2f,
    /// The end point of the quadratic Bezier segment.
    pub p2: Point2f,
}

impl QuadBezierSegment {
    /// Constructs the bezier segment from its components
    #[inline]
    pub fn new(
        p1: impl Into<Point2f>,
        p2: impl Into<Point2f>,
    ) -> QuadBezierSegment {
        QuadBezierSegment {
            p1: p1.into(),
            p2: p2.into(),
        }
    }
}

impl<P1, P2> From<(P1, P2)> for QuadBezierSegment
where
    P1: Into<Point2f>,
    P2: Into<Point2f>,
{
    #[inline]
    fn from((p1, p2): (P1, P2)) -> QuadBezierSegment {
        QuadBezierSegment {
            p1: p1.into(),
            p2: p2.into(),
        }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<QuadBezierSegment> for D2D1_QUADRATIC_BEZIER_SEGMENT {
    #[inline]
    fn from(seg: QuadBezierSegment) -> D2D1_QUADRATIC_BEZIER_SEGMENT {
        D2D1_QUADRATIC_BEZIER_SEGMENT {
            point1: seg.p1.into(),
            point2: seg.p2.into(),
        }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<D2D1_QUADRATIC_BEZIER_SEGMENT> for QuadBezierSegment {
    #[inline]
    fn from(seg: D2D1_QUADRATIC_BEZIER_SEGMENT) -> QuadBezierSegment {
        QuadBezierSegment {
            p1: seg.point1.into(),
            p2: seg.point2.into(),
        }
    }
}

#[cfg(all(test, windows, feature = "d2d"))]
#[test]
fn qbezier_d2d_bin_compat() {
    use std::mem::size_of_val;

    fn ptr_eq<T>(a: &T, b: &T) -> bool {
        (a as *const T) == (b as *const T)
    }

    let bez = QuadBezierSegment::new((0.0, 0.0), (1.0, 0.0));
    let d2d = unsafe { &*((&bez) as *const _ as *const D2D1_QUADRATIC_BEZIER_SEGMENT) };

    assert!(ptr_eq(&bez.p1.x, &d2d.point1.x));
    assert!(ptr_eq(&bez.p1.y, &d2d.point1.y));
    assert!(ptr_eq(&bez.p2.x, &d2d.point2.x));
    assert!(ptr_eq(&bez.p2.y, &d2d.point2.y));
    assert_eq!(size_of_val(&bez), size_of_val(d2d));
}
