#[cfg(all(windows, feature = "d2d"))]
use winapi::um::dcommon::D2D_SIZE_U;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
pub struct Sizeu {
    pub width: u32,
    pub height: u32,
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
