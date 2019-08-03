//! Axis-aligned rectangle defined by the lines of its 4 edges.

use crate::rectf::Rectf;
use crate::recti::Recti;

#[cfg(all(windows, feature = "d2d"))]
use winapi::um::dcommon::D2D_RECT_U;

/// Represents a rectangle defined by the coordinates of the upper-left corner
/// (left, top) and the coordinates of the lower-right corner (right, bottom).
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Rectu {
    /// The x-coordinate of the upper-left corner of the rectangle.
    pub left: u32,
    /// The y-coordinate of the upper-left corner of the rectangle.
    pub top: u32,
    /// The x-coordinate of the lower-right corner of the rectangle.
    pub right: u32,
    /// The y-coordinate of the lower-right corner of the rectangle.
    pub bottom: u32,
}

impl Rectu {
    /// Constructs the rectangle from components.
    #[inline]
    pub fn new(left: u32, top: u32, right: u32, bottom: u32) -> Rectu {
        Rectu {
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

    /// Converts the rectangle to signed integer values. Beware of casting
    /// overflow if any of the components are greater than i32::MAX.
    #[inline]
    pub fn to_i32(&self) -> Recti {
        Recti {
            left: self.left as i32,
            top: self.top as i32,
            right: self.right as i32,
            bottom: self.bottom as i32,
        }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<Rectu> for D2D_RECT_U {
    #[inline]
    fn from(rect: Rectu) -> D2D_RECT_U {
        D2D_RECT_U {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<D2D_RECT_U> for Rectu {
    #[inline]
    fn from(rect: D2D_RECT_U) -> Rectu {
        Rectu {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        }
    }
}

#[cfg(all(test, windows, feature = "d2d"))]
#[test]
fn rectu_d2d_bin_compat() {
    use std::mem::size_of_val;

    fn ptr_eq<T>(a: &T, b: &T) -> bool {
        (a as *const T) == (b as *const T)
    }

    let rect = Rectu::new(0, 0, 0, 0);
    let d2d = unsafe { &*((&rect) as *const _ as *const D2D_RECT_U) };

    assert!(ptr_eq(&rect.left, &d2d.left));
    assert!(ptr_eq(&rect.top, &d2d.top));
    assert!(ptr_eq(&rect.right, &d2d.right));
    assert!(ptr_eq(&rect.bottom, &d2d.bottom));
    assert_eq!(size_of_val(&rect), size_of_val(d2d));
}
