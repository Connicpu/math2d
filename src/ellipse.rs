//! Axis-aligned ellipse constructed from a center point and the x and y radii.

use matrix3x2f::Matrix3x2f;
use point2f::Point2f;

#[cfg(all(windows, feature = "d2d"))]
use winapi::um::d2d1::D2D1_ELLIPSE;

/// Contains the center point, x-radius, and y-radius of an ellipse.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Ellipse {
    /// The center point of the ellipse.
    pub center: Point2f,
    /// The X-radius of the ellipse.
    pub radius_x: f32,
    /// The Y-radius of the ellipse.
    pub radius_y: f32,
}

impl Ellipse {
    /// Constructs an ellipse from its components
    #[inline]
    pub fn new(center: impl Into<Point2f>, rx: f32, ry: f32) -> Ellipse {
        Ellipse {
            center: center.into(),
            radius_x: rx,
            radius_y: ry,
        }
    }

    /// Checks if an ellipse contains a point
    #[inline]
    pub fn contains_point(&self, point: impl Into<Point2f>) -> bool {
        let point = point.into();
        let px = point.x - self.center.x;
        let px2 = px * px;
        let py = point.y - self.center.y;
        let py2 = py * py;
        let rx2 = self.radius_x * self.radius_x;
        let ry2 = self.radius_y * self.radius_y;

        px2 / rx2 + py2 / ry2 <= 1.0
    }

    /// Determines if an ellipse which has a transform applied to it contains a specified
    /// (non- or pre-transformed) point.
    ///
    /// Will always return false if `!transform.is_invertible()`
    #[inline]
    pub fn contains_point_transformed(
        &self,
        transform: &Matrix3x2f,
        point: impl Into<Point2f>,
    ) -> bool {
        if let Some(inverse) = transform.try_inverse() {
            let point = point.into();
            let point = point * inverse;
            self.contains_point(point)
        } else {
            false
        }
    }
}

impl<P> From<(P, f32, f32)> for Ellipse
where
    P: Into<Point2f>,
{
    #[inline]
    fn from(data: (P, f32, f32)) -> Ellipse {
        Ellipse::new(data.0, data.1, data.2)
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<Ellipse> for D2D1_ELLIPSE {
    #[inline]
    fn from(e: Ellipse) -> D2D1_ELLIPSE {
        D2D1_ELLIPSE {
            point: e.center.into(),
            radiusX: e.radius_x,
            radiusY: e.radius_y,
        }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<D2D1_ELLIPSE> for Ellipse {
    #[inline]
    fn from(e: D2D1_ELLIPSE) -> Ellipse {
        Ellipse {
            center: e.point.into(),
            radius_x: e.radiusX,
            radius_y: e.radiusY,
        }
    }
}

#[cfg(all(test, windows, feature = "d2d"))]
#[test]
fn ellipse_d2d_bin_compat() {
    use std::mem::size_of_val;

    fn ptr_eq<T>(a: &T, b: &T) -> bool {
        (a as *const T) == (b as *const T)
    }

    let ellipse = Ellipse::new((0.0, 0.0), 1.0, 0.5);
    let d2d = unsafe { &*((&ellipse) as *const _ as *const D2D1_ELLIPSE) };

    assert!(ptr_eq(&ellipse.center.x, &d2d.point.x));
    assert!(ptr_eq(&ellipse.center.y, &d2d.point.y));
    assert!(ptr_eq(&ellipse.radius_x, &d2d.radiusX));
    assert!(ptr_eq(&ellipse.radius_y, &d2d.radiusY));
    assert_eq!(size_of_val(&ellipse), size_of_val(d2d));
}
