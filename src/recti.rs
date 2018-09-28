use crate::rectf::Rectf;
use crate::rectu::Rectu;

#[cfg(all(windows, feature = "d2d"))]
use winapi::um::dcommon::D2D_RECT_L;

#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
pub struct Recti {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Recti {
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

