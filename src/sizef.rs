#[cfg(all(windows, feature = "d2d"))]
use winapi::um::dcommon::D2D_SIZE_F;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
pub struct Sizef {
    pub width: f32,
    pub height: f32,
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
