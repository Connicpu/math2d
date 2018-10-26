//! Unsigned integer size descriptor e.g. the size of a bitmap.

#[cfg(all(windows, feature = "d2d"))]
use winapi::um::dcommon::D2D_SIZE_U;

/// Stores an ordered pair of unsigned integer values, typically the width
/// and height of a rectangle.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Sizeu {
    /// Horizontal component.
    pub width: u32,
    /// Vertical component.
    pub height: u32,
}

impl Sizeu {
    /// Constructs a size from the components.
    #[inline]
    pub fn new(width: u32, height: u32) -> Sizeu {
        Sizeu { width, height }
    }
}

impl From<u32> for Sizeu {
    #[inline]
    fn from(size: u32) -> Sizeu {
        Sizeu {
            width: size,
            height: size,
        }
    }
}

impl From<(u32, u32)> for Sizeu {
    #[inline]
    fn from((width, height): (u32, u32)) -> Sizeu {
        Sizeu { width, height }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<Sizeu> for D2D_SIZE_U {
    #[inline]
    fn from(point: Sizeu) -> D2D_SIZE_U {
        D2D_SIZE_U {
            width: point.width,
            height: point.height,
        }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<D2D_SIZE_U> for Sizeu {
    #[inline]
    fn from(point: D2D_SIZE_U) -> Sizeu {
        Sizeu {
            width: point.width,
            height: point.height,
        }
    }
}

#[cfg(all(test, windows, feature = "d2d"))]
#[test]
fn sizeu_d2d_bin_compat() {
    use std::mem::size_of_val;

    fn ptr_eq<T>(a: &T, b: &T) -> bool {
        (a as *const T) == (b as *const T)
    }

    let sz = Sizeu::new(0, 0);
    let d2d = unsafe { &*((&sz) as *const _ as *const D2D_SIZE_U) };

    assert!(ptr_eq(&sz.width, &d2d.width));
    assert!(ptr_eq(&sz.height, &d2d.height));
    assert_eq!(size_of_val(&sz), size_of_val(d2d));
}
