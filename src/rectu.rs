use crate::rectf::Rectf;
use crate::recti::Recti;

#[cfg(all(windows, feature = "d2d"))]
use winapi::um::dcommon::D2D_RECT_U;

#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
pub struct Rectu {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

impl Rectu {
    #[inline]
    pub fn to_f32(&self) -> Rectf {
        Rectf {
            left: self.left as f32,
            top: self.top as f32,
            right: self.right as f32,
            bottom: self.bottom as f32,
        }
    }

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

