//! Floating point size descriptor.

#[cfg(all(windows, feature = "d2d"))]
use winapi::um::dcommon::D2D_SIZE_F;

/// Stores an ordered pair of floating-point values, typically the width
/// and height of a rectangle.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Sizef {
    /// Horizontal component.
    pub width: f32,
    /// Vertical component.
    pub height: f32,
}

impl Sizef {
    /// Constructs a size from the components.
    #[inline]
    pub fn new(width: f32, height: f32) -> Sizef {
        Sizef { width, height }
    }
}

impl From<f32> for Sizef {
    #[inline]
    fn from(size: f32) -> Sizef {
        Sizef {
            width: size,
            height: size,
        }
    }
}

impl From<(f32, f32)> for Sizef {
    #[inline]
    fn from((width, height): (f32, f32)) -> Sizef {
        Sizef { width, height }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<Sizef> for D2D_SIZE_F {
    #[inline]
    fn from(point: Sizef) -> D2D_SIZE_F {
        D2D_SIZE_F {
            width: point.width,
            height: point.height,
        }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<D2D_SIZE_F> for Sizef {
    #[inline]
    fn from(point: D2D_SIZE_F) -> Sizef {
        Sizef {
            width: point.width,
            height: point.height,
        }
    }
}

#[cfg(all(test, windows, feature = "d2d"))]
#[test]
fn sizef_d2d_bin_compat() {
    use std::mem::size_of_val;

    fn ptr_eq<T>(a: &T, b: &T) -> bool {
        (a as *const T) == (b as *const T)
    }

    let sz = Sizef::new(0.0, 0.0);
    let d2d = unsafe { &*((&sz) as *const _ as *const D2D_SIZE_F) };

    assert!(ptr_eq(&sz.width, &d2d.width));
    assert!(ptr_eq(&sz.height, &d2d.height));
    assert_eq!(size_of_val(&sz), size_of_val(d2d));
}
