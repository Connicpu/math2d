//! Axis-aligned rectangle defined by the lines of its 4 edges.

use point2f::Point2f;
use recti::Recti;
use rectu::Rectu;
use sizef::Sizef;
use thicknessf::Thicknessf;
use vector2f::Vector2f;

use std::f32::{INFINITY, NEG_INFINITY};
use std::ops::{Add, Sub};

#[cfg(all(windows, feature = "d2d"))]
use winapi::um::dcommon::D2D_RECT_F;

/// Represents a rectangle defined by the coordinates of the upper-left corner
/// (left, top) and the coordinates of the lower-right corner (right, bottom).
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Rectf {
    /// The x-coordinate of the left edge of the rectangle.
    pub left: f32,
    /// The y-coordinate of the top edge of the rectangle.
    pub top: f32,
    /// The x-coordinate of the right edge of the rectangle.
    pub right: f32,
    /// The y-coordinate of the bottom edge of the rectangle.
    pub bottom: f32,
}

/// Represents a corner of the rectangle
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
pub enum RectCorner {
    /// The (left, top) coordinate pair
    TopLeft,
    /// The (right, top) coordinate pair
    TopRight,
    /// The (left, bottom) coordinate pair
    BottomLeft,
    /// The (right, bottom) coordinate pair
    BottomRight,
}

impl Rectf {
    /// A rect that holds the entire real space
    pub const INFINITE: Rectf = Rectf {
        left: NEG_INFINITY,
        top: NEG_INFINITY,
        right: INFINITY,
        bottom: INFINITY,
    };

    /// Constructs the rectangle from components.
    #[inline]
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Rectf {
        Rectf {
            left,
            top,
            right,
            bottom,
        }
    }

    /// Constructs a rectangle that will encompass all of the axis-aligned
    /// space between the two provided points.
    #[inline]
    pub fn from_points(p1: impl Into<Point2f>, p2: impl Into<Point2f>) -> Rectf {
        let p1 = p1.into();
        let p2 = p2.into();
        Rectf {
            left: p1.x.min(p2.x),
            top: p1.y.min(p2.y),
            right: p1.x.max(p2.x),
            bottom: p1.y.max(p2.y),
        }
    }

    /// Constructs a rectangle given its desired center point and desired
    /// width and height.
    #[inline]
    pub fn from_center_size(center: impl Into<Point2f>, size: impl Into<Sizef>) -> Rectf {
        let center = center.into();
        let size = size.into();
        Rectf {
            left: center.x - size.width / 2.0,
            top: center.y - size.height / 2.0,
            right: center.x + size.width / 2.0,
            bottom: center.y + size.height / 2.0,
        }
    }

    /// Constructs a rectangle given its desired center and the desired
    /// distance from the center to the corners.
    #[inline]
    pub fn from_center_half_extent(
        center: impl Into<Point2f>,
        half_extents: impl Into<Vector2f>,
    ) -> Rectf {
        let center = center.into();
        let half_extents = half_extents.into();
        Rectf {
            left: center.x - half_extents.x,
            top: center.y - half_extents.y,
            right: center.x + half_extents.x,
            bottom: center.y + half_extents.y,
        }
    }

    /// Converts this rectangle's components to signed integers. Truncates
    /// values, perform manual rounding if you would like a different behavior.
    #[inline]
    pub fn to_i32(&self) -> Recti {
        Recti {
            left: self.left as i32,
            top: self.top as i32,
            right: self.right as i32,
            bottom: self.bottom as i32,
        }
    }

    /// Converts the components of the rectangle to unsigned integers. Beware
    /// this conversion if the components could be negative, you will experience
    /// unsigned casting underflow.
    #[inline]
    pub fn to_u32(&self) -> Rectu {
        Rectu {
            left: self.left as u32,
            top: self.top as u32,
            right: self.right as u32,
            bottom: self.bottom as u32,
        }
    }

    /// Rounds the components to the nearest integers, rounding
    /// half-way values away from zero.
    #[inline]
    pub fn rounded(&self) -> Rectf {
        Rectf {
            left: self.left.round(),
            top: self.top.round(),
            right: self.right.round(),
            bottom: self.bottom.round(),
        }
    }

    /// Gets the width and height of this rectangle.
    #[inline]
    pub fn size(&self) -> Sizef {
        (self.right - self.bottom, self.bottom - self.top).into()
    }

    /// Gets the center point of this rectangle.
    #[inline]
    pub fn center(&self) -> Point2f {
        Point2f {
            x: (self.left + self.right) / 2.0,
            y: (self.top + self.bottom) / 2.0,
        }
    }

    /// Gets the half-extent of the rectangle i.e. the vector from the
    /// center to the most-positive corner.
    #[inline]
    pub fn half_extent(&self) -> Vector2f {
        let size = self.size();
        [size.width / 2.0, size.height / 2.0].into()
    }

    /// Get the point of the specified corner.
    #[inline]
    pub fn corner(&self, corner: RectCorner) -> Point2f {
        match corner {
            RectCorner::TopLeft => (self.left, self.top).into(),
            RectCorner::TopRight => (self.right, self.top).into(),
            RectCorner::BottomLeft => (self.left, self.bottom).into(),
            RectCorner::BottomRight => (self.right, self.bottom).into(),
        }
    }

    /// Determines if the specified point is located inside the rectangle.
    #[inline]
    pub fn contains_point(&self, point: impl Into<Point2f>) -> bool {
        let point = point.into();
        return point.x >= self.left
            && point.y >= self.top
            && point.x <= self.right
            && point.y <= self.bottom;
    }

    /// Normalizes the rectangle to enforce the invariants
    /// `left < right` and `top < bottom`.
    #[inline]
    pub fn normalized(self) -> Self {
        Rectf {
            left: self.left.min(self.right),
            top: self.top.min(self.bottom),
            right: self.left.max(self.top),
            bottom: self.top.max(self.bottom),
        }
    }

    /// Translates the rectangle by the given vector.
    #[inline]
    pub fn translated_by(self, translation: impl Into<Vector2f>) -> Self {
        let trans = translation.into();
        Rectf {
            left: self.left + trans.x,
            top: self.top + trans.y,
            right: self.right + trans.x,
            bottom: self.bottom + trans.y,
        }
    }

    /// Expands the rectangle by the given margin.
    #[inline]
    pub fn expanded_by(self, thickness: impl Into<Thicknessf>) -> Self {
        let t = thickness.into();
        Rectf {
            left: self.left - t.left,
            top: self.top - t.top,
            right: self.right + t.right,
            bottom: self.bottom + t.bottom,
        }
    }

    /// Shrinks the rectangle by the given margin.
    #[inline]
    pub fn shrunken_by(self, thickness: impl Into<Thicknessf>) -> Self {
        let t = thickness.into();
        Rectf {
            left: self.left + t.left,
            top: self.top + t.top,
            right: self.right - t.right,
            bottom: self.bottom - t.bottom,
        }
    }

    /// Constructs a rectangle that contains both rectangles. Normalizes
    /// both arguments before performing the operation.
    #[inline]
    pub fn combined_with(&self, other: impl Into<Rectf>) -> Self {
        let r1 = self.normalized();
        let r2 = other.into().normalized();

        let left = r1.left.min(r2.left);
        let top = r1.top.min(r2.top);
        let right = r1.right.max(r2.right);
        let bottom = r1.bottom.max(r2.bottom);

        Rectf {
            left,
            top,
            right,
            bottom,
        }
    }
}

impl Add<Vector2f> for Rectf {
    type Output = Rectf;

    #[inline]
    fn add(self, rhs: Vector2f) -> Rectf {
        self.translated_by(rhs)
    }
}

impl Sub<Vector2f> for Rectf {
    type Output = Rectf;

    #[inline]
    fn sub(self, rhs: Vector2f) -> Rectf {
        self.translated_by(-rhs)
    }
}

impl From<(Point2f, Point2f)> for Rectf {
    #[inline]
    fn from((p1, p2): (Point2f, Point2f)) -> Rectf {
        Rectf::from_points(p1, p2)
    }
}

impl From<(Point2f, Sizef)> for Rectf {
    #[inline]
    fn from((center, size): (Point2f, Sizef)) -> Rectf {
        Rectf::from_center_size(center, size)
    }
}

impl From<(Point2f, Vector2f)> for Rectf {
    #[inline]
    fn from((center, half_extent): (Point2f, Vector2f)) -> Rectf {
        Rectf::from_center_half_extent(center, half_extent)
    }
}

impl From<[f32; 4]> for Rectf {
    #[inline]
    fn from(p: [f32; 4]) -> Rectf {
        let (left, top, right, bottom) = (p[0], p[1], p[2], p[3]);
        Rectf {
            left,
            top,
            right,
            bottom,
        }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<Rectf> for D2D_RECT_F {
    #[inline]
    fn from(rect: Rectf) -> D2D_RECT_F {
        D2D_RECT_F {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<D2D_RECT_F> for Rectf {
    #[inline]
    fn from(rect: D2D_RECT_F) -> Rectf {
        Rectf {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        }
    }
}

#[cfg(all(test, windows, feature = "d2d"))]
#[test]
fn rectf_d2d_bin_compat() {
    use std::mem::size_of_val;

    fn ptr_eq<T>(a: &T, b: &T) -> bool {
        (a as *const T) == (b as *const T)
    }

    let rect = Rectf::new(0.0, 0.0, 0.0, 0.0);
    let d2d = unsafe { &*((&rect) as *const _ as *const D2D_RECT_F) };

    assert!(ptr_eq(&rect.left, &d2d.left));
    assert!(ptr_eq(&rect.top, &d2d.top));
    assert!(ptr_eq(&rect.right, &d2d.right));
    assert!(ptr_eq(&rect.bottom, &d2d.bottom));
    assert_eq!(size_of_val(&rect), size_of_val(d2d));
}
