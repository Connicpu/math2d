//! Axis-aligned rectangle defined by the lines of its 4 edges.

use rectf::Rectf;
use rectu::Rectu;

#[cfg(all(windows, feature = "d2d"))]
use winapi::um::dcommon::D2D_RECT_L;

/// Represents a rectangle defined by the coordinates of the upper-left corner
/// (left, top) and the coordinates of the lower-right corner (right, bottom).
#[derive(Copy, Clone, Debug, Default)]
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
