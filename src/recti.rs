//! Axis-aligned rectangle defined by the lines of its 4 edges.

use point2i::Point2i;
use rectf::Rectf;
use rectu::Rectu;

#[cfg(all(windows, feature = "d2d"))]
use winapi::um::dcommon::D2D_RECT_L;
#[cfg(all(windows, feature = "d2d"))]
use winapi::um::wincodec::WICRect;

/// Represents a rectangle defined by the coordinates of the upper-left corner
/// (left, top) and the coordinates of the lower-right corner (right, bottom).
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Recti {
    /// The x-coordinate of the upper-left corner of the rectangle.
    pub left: i32,
    /// The y-coordinate of the upper-left corner of the rectangle.
    pub top: i32,
    /// The x-coordinate of the lower-right corner of the rectangle.
    pub right: i32,
    /// The y-coordinate of the lower-right corner of the rectangle.
    pub bottom: i32,
}

impl Recti {
    /// An inversely empty rectangle that contains no points
    pub const EMPTY: Recti = Recti {
        left: std::i32::MAX,
        top: std::i32::MAX,
        right: std::i32::MIN,
        bottom: std::i32::MIN,
    };

    /// Constructs the rectangle from components.
    #[inline]
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Recti {
        Recti {
            left,
            top,
            right,
            bottom,
        }
    }

    #[inline]
    pub fn point(point: impl Into<Point2i>) -> Self {
        let point = point.into();
        Recti {
            left: point.x,
            right: point.x,
            top: point.y,
            bottom: point.y,
        }
    }

    /// Converts the rectangle to floating point values.
    #[inline]
    pub fn to_f32(&self) -> Rectf {
        Rectf {
            left: self.left as f32,
            top: self.top as f32,
            right: self.right as f32,
            bottom: self.bottom as f32,
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

    /// Determines if the specified point is located inside the rectangle.
    #[inline]
    pub fn contains_point(&self, point: impl Into<Point2i>) -> bool {
        let point = point.into();
        return point.x >= self.left
            && point.y >= self.top
            && point.x <= self.right
            && point.y <= self.bottom;
    }

    #[inline]
    pub fn is_on_edge(&self, point: impl Into<Point2i>) -> bool {
        let point = point.into();
        return point.x == self.left
            || point.x == self.right
            || point.y == self.top
            || point.y == self.bottom;
    }

    /// Normalizes the rectangle to enforce the invariants
    /// `left < right` and `top < bottom`.
    #[inline]
    pub fn normalized(self) -> Self {
        Recti {
            left: self.left.min(self.right),
            top: self.top.min(self.bottom),
            right: self.left.max(self.top),
            bottom: self.top.max(self.bottom),
        }
    }

    /// Constructs a rectangle that contains both rectangles. Normalizes
    /// both arguments before performing the operation.
    #[inline]
    pub fn combined_with(&self, other: impl Into<Recti>) -> Self {
        let r2 = other.into();

        let left = self.left.min(r2.left);
        let top = self.top.min(r2.top);
        let right = self.right.max(r2.right);
        let bottom = self.bottom.max(r2.bottom);

        Recti {
            left,
            top,
            right,
            bottom,
        }
    }

    #[inline]
    pub fn width(&self) -> i32 {
        self.right - self.left
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.bottom - self.top
    }

    #[inline]
    pub fn area(&self) -> i64 {
        let width = self.width() as i64;
        let height = self.height() as i64;
        width * height
    }

    #[inline]
    pub fn rows(&self) -> impl Iterator<Item = i32> {
        self.top..=self.bottom
    }

    #[inline]
    pub fn columns(&self) -> impl Iterator<Item = i32> {
        self.left..=self.right
    }

    #[inline]
    pub fn points(self) -> impl Iterator<Item = Point2i> {
        self.rows()
            .flat_map(move |row| self.columns().map(move |col| (col, row).into()))
    }
}

impl From<Point2i> for Recti {
    #[inline]
    fn from(point: Point2i) -> Recti {
        Recti::point(point)
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<Recti> for D2D_RECT_L {
    #[inline]
    fn from(rect: Recti) -> D2D_RECT_L {
        D2D_RECT_L {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<D2D_RECT_L> for Recti {
    #[inline]
    fn from(rect: D2D_RECT_L) -> Recti {
        Recti {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<WICRect> for Recti {
    #[inline]
    fn from(rect: WICRect) -> Recti {
        Recti {
            left: rect.X,
            top: rect.Y,
            right: rect.X + rect.Width,
            bottom: rect.Y + rect.Height,
        }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<Recti> for WICRect {
    #[inline]
    fn from(rect: Recti) -> WICRect {
        WICRect {
            X: rect.left,
            Y: rect.top,
            Width: rect.width(),
            Height: rect.height(),
        }
    }
}

#[cfg(all(test, windows, feature = "d2d"))]
#[test]
fn recti_d2d_bin_compat() {
    use std::mem::size_of_val;

    fn ptr_eq<T>(a: &T, b: &T) -> bool {
        (a as *const T) == (b as *const T)
    }

    let rect = Recti::new(0, 0, 0, 0);
    let d2d = unsafe { &*((&rect) as *const _ as *const D2D_RECT_L) };

    assert!(ptr_eq(&rect.left, &d2d.left));
    assert!(ptr_eq(&rect.top, &d2d.top));
    assert!(ptr_eq(&rect.right, &d2d.right));
    assert!(ptr_eq(&rect.bottom, &d2d.bottom));
    assert_eq!(size_of_val(&rect), size_of_val(d2d));
}
