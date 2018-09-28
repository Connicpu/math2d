use crate::point2f::Point2f;
use crate::recti::Recti;
use crate::rectu::Rectu;
use crate::sizef::Sizef;
use crate::thicknessf::Thicknessf;
use crate::vector2f::Vector2f;
use crate::RectCorner;

use std::ops::{Add, Sub};

#[cfg(all(windows, feature = "d2d"))]
use winapi::um::dcommon::D2D_RECT_F;

#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
pub struct Rectf {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Rectf {
    #[inline]
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Rectf {
        Rectf {
            left,
            top,
            right,
            bottom,
        }
    }

    #[inline]
    pub fn from_points(p1: Point2f, p2: Point2f) -> Rectf {
        Rectf {
            left: p1.x.min(p2.x),
            top: p1.y.min(p2.y),
            right: p1.x.max(p2.x),
            bottom: p1.y.max(p2.y),
        }
    }

    #[inline]
    pub fn from_center_size(center: impl Into<Point2f>, size: impl Into<Sizef>) -> Rectf {
        let center = center.into();
        let size = size.into();
        Rectf {
            left: center.x - size.width / 2.0,
            top: center.y - size.height / 2.0,
            right: center.x + size.width / 2.0,
            bottom: center.y + size.height / 2.0,
        }
    }

    #[inline]
    pub fn from_center_half_extents(
        center: impl Into<Point2f>,
        half_extents: impl Into<Vector2f>,
    ) -> Rectf {
        let center = center.into();
        let half_extents = half_extents.into();
        Rectf {
            left: center.x - half_extents.x,
            top: center.y - half_extents.y,
            right: center.x + half_extents.x,
            bottom: center.y + half_extents.y,
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

    #[inline]
    pub fn to_u32(&self) -> Rectu {
        Rectu {
            left: self.left as u32,
            top: self.top as u32,
            right: self.right as u32,
            bottom: self.bottom as u32,
        }
    }

    #[inline]
    pub fn get_size(&self) -> Sizef {
        (self.right - self.bottom, self.bottom - self.top).into()
    }

    #[inline]
    pub fn get_center(&self) -> Point2f {
        Point2f {
            x: (self.left + self.right) / 2.0,
            y: (self.top + self.bottom) / 2.0,
        }
    }

    #[inline]
    pub fn get_corner(&self, corner: RectCorner) -> Point2f {
        match corner {
            RectCorner::TopLeft => (self.left, self.top).into(),
            RectCorner::TopRight => (self.right, self.top).into(),
            RectCorner::BottomLeft => (self.left, self.bottom).into(),
            RectCorner::BottomRight => (self.right, self.bottom).into(),
        }
    }

    #[inline]
    pub fn contains_point(&self, point: impl Into<Point2f>) -> bool {
        let point = point.into();
        return point.x >= self.left
            && point.y >= self.top
            && point.x <= self.right
            && point.y <= self.bottom;
    }

    #[inline]
    pub fn normalized(self) -> Self {
        Rectf {
            left: self.left.min(self.right),
            top: self.top.min(self.bottom),
            right: self.left.max(self.top),
            bottom: self.top.max(self.bottom),
        }
    }

    #[inline]
    pub fn translated_by(self, translation: impl Into<Vector2f>) -> Self {
        let trans = translation.into();
        Rectf {
            left: self.left + trans.x,
            top: self.top + trans.y,
            right: self.right + trans.x,
            bottom: self.bottom + trans.y,
        }
    }

    #[inline]
    pub fn expanded_by(self, thickness: impl Into<Thicknessf>) -> Self {
        let t = thickness.into();
        Rectf {
            left: self.left - t.left,
            top: self.top - t.top,
            right: self.right + t.right,
            bottom: self.bottom + t.bottom,
        }
    }

    #[inline]
    pub fn shrunken_by(self, thickness: impl Into<Thicknessf>) -> Self {
        let t = thickness.into();
        Rectf {
            left: self.left + t.left,
            top: self.top + t.top,
            right: self.right - t.right,
            bottom: self.bottom - t.bottom,
        }
    }

    #[inline]
    pub fn combined_with(self, other: impl Into<Rectf>) -> Self {
        let r1 = self.normalized();
        let r2 = other.into().normalized();

        let left = r1.left.min(r2.left);
        let top = r1.top.min(r2.top);
        let right = r1.right.max(r2.right);
        let bottom = r1.bottom.max(r2.bottom);

        Rectf {
            left,
            top,
            right,
            bottom,
        }
    }
}

impl Add<Vector2f> for Rectf {
    type Output = Rectf;

    #[inline]
    fn add(self, rhs: Vector2f) -> Rectf {
        self.translated_by(rhs)
    }
}

impl Sub<Vector2f> for Rectf {
    type Output = Rectf;

    #[inline]
    fn sub(self, rhs: Vector2f) -> Rectf {
        self.translated_by(-rhs)
    }
}

impl From<(Point2f, Point2f)> for Rectf {
    #[inline]
    fn from((p1, p2): (Point2f, Point2f)) -> Rectf {
        Rectf::from_points(p1, p2)
    }
}

impl From<(Point2f, Sizef)> for Rectf {
    #[inline]
    fn from((center, size): (Point2f, Sizef)) -> Rectf {
        Rectf::from_center_size(center, size)
    }
}

impl From<(Point2f, Vector2f)> for Rectf {
    #[inline]
    fn from((center, half_extents): (Point2f, Vector2f)) -> Rectf {
        Rectf::from_center_half_extents(center, half_extents)
    }
}

impl From<(f32, f32, f32, f32)> for Rectf {
    #[inline]
    fn from((left, top, right, bottom): (f32, f32, f32, f32)) -> Rectf {
        Rectf {
            left,
            top,
            right,
            bottom,
        }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<Rectf> for D2D_RECT_F {
    #[inline]
    fn from(rect: Rectf) -> D2D_RECT_F {
        D2D_RECT_F {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<D2D_RECT_F> for Rectf {
    #[inline]
    fn from(rect: D2D_RECT_F) -> Rectf {
        Rectf {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        }
    }
}
