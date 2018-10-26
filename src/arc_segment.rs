//! ArcSegments represent a curved line following the path of an ellipse
//! and are designed to be part of a Path. See Direct2D, SVG, etc for
//! an overview of the Path concept.

use point2f::Point2f;
use sizef::Sizef;

#[cfg(all(windows, feature = "d2d"))]
use winapi::um::d2d1::D2D1_ARC_SEGMENT;

/// Describes an elliptical arc between two points. The starting point
/// is implicit when an ArcSegment is used as part of a Path, as it is a
/// continuation from the previous segment.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ArcSegment {
    /// The end point of the arc.
    pub point: Point2f,
    /// The x and y radius of the arc.
    pub size: Sizef,
    /// A value that specifies how many degrees in the clockwise direction the
    /// ellipse is rotated relative to the current coordinate system.
    pub rotation_angle: f32,
    /// A value that specifies whether the arc sweep is clockwise or
    /// counterclockwise.
    pub sweep_direction: SweepDirection,
    /// A value that specifies whether the given arc is larger than 180 degrees.
    pub arc_size: ArcSize,
}

impl ArcSegment {
    /// Constructs an ArcSegment from its parts, more conveniently allowing
    /// types that may be converted into Point and Size (such as tuples of floats)
    #[inline]
    pub fn new(
        point: impl Into<Point2f>,
        size: impl Into<Sizef>,
        rotation_angle: f32,
        sweep_direction: SweepDirection,
        arc_size: ArcSize,
    ) -> ArcSegment {
        ArcSegment {
            point: point.into(),
            size: size.into(),
            rotation_angle,
            sweep_direction,
            arc_size,
        }
    }
}

/// Defines the direction that an elliptical arc is drawn.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[repr(u32)]
pub enum SweepDirection {
    /// Arcs are drawn in a counterclockwise (negative-angle) direction.
    CounterClockwise = 0,
    /// Arcs are drawn in a clockwise (positive-angle) direction.
    Clockwise = 1,
}

impl Default for SweepDirection {
    #[inline]
    fn default() -> Self {
        SweepDirection::CounterClockwise
    }
}

/// Specifies whether an arc should be greater than 180 degrees.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[repr(u32)]
pub enum ArcSize {
    /// An arc's sweep should be 180 degrees or less.
    Small = 0,
    /// An arc's sweep should be 180 degrees or greater.
    Large = 1,
}

impl Default for ArcSize {
    #[inline]
    fn default() -> Self {
        ArcSize::Small
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<ArcSegment> for D2D1_ARC_SEGMENT {
    #[inline]
    fn from(seg: ArcSegment) -> D2D1_ARC_SEGMENT {
        D2D1_ARC_SEGMENT {
            point: seg.point.into(),
            size: seg.size.into(),
            rotationAngle: seg.rotation_angle,
            sweepDirection: seg.sweep_direction as u32,
            arcSize: seg.arc_size as u32,
        }
    }
}

#[cfg(all(test, windows, feature = "d2d"))]
#[test]
fn arc_d2d_bin_compat() {
    use std::mem::size_of_val;

    fn ptr_eq<T, U>(a: &T, b: &U) -> bool {
        assert_eq!(size_of_val(a), size_of_val(b));
        (a as *const T) == (b as *const U as *const T)
    }

    let arc = ArcSegment::new(
        (0.0, 0.0),
        (1.0, 1.0),
        90.0,
        SweepDirection::CounterClockwise,
        ArcSize::Small,
    );
    let d2d = unsafe { &*((&arc) as *const _ as *const D2D1_ARC_SEGMENT) };

    assert!(ptr_eq(&arc.point.x, &d2d.point.x));
    assert!(ptr_eq(&arc.point.y, &d2d.point.y));
    assert!(ptr_eq(&arc.size.width, &d2d.size.width));
    assert!(ptr_eq(&arc.size.height, &d2d.size.height));
    assert!(ptr_eq(&arc.rotation_angle, &d2d.rotationAngle));
    assert!(ptr_eq(&arc.sweep_direction, &d2d.sweepDirection));
    assert!(ptr_eq(&arc.arc_size, &d2d.arcSize));
    assert_eq!(size_of_val(&arc), size_of_val(d2d));
}
