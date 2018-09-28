use crate::vector2f::Vector2f;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
pub struct Thicknessf {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl From<Vector2f> for Thicknessf {
    #[inline]
    fn from(vec: Vector2f) -> Thicknessf {
        (vec.x, vec.y).into()
    }
}

impl From<f32> for Thicknessf {
    #[inline]
    fn from(f: f32) -> Thicknessf {
        (f, f, f, f).into()
    }
}

impl From<(f32, f32)> for Thicknessf {
    #[inline]
    fn from((x, y): (f32, f32)) -> Thicknessf {
        (x, y, x, y).into()
    }
}

impl From<(f32, f32, f32, f32)> for Thicknessf {
    #[inline]
    fn from(values: (f32, f32, f32, f32)) -> Thicknessf {
        let (left, top, right, bottom) = values;
        Thicknessf {
            left,
            top,
            right,
            bottom,
        }
    }
}
