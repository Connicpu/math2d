use crate::vector2f::Vector2f;

use std::ops::{Add, Div, Mul, Neg, Sub};

/// Mathematical point on the 2D (x, y) plane
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
pub struct Vector2i {
    pub x: i32,
    pub y: i32,
}

impl Vector2i {
    pub const ZERO: Vector2i = Vector2i { x: 0, y: 0 };

    #[inline]
    pub fn to_f32(self) -> Vector2f {
        Vector2f {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

impl Add for Vector2i {
    type Output = Vector2i;

    #[inline]
    fn add(self, rhs: Vector2i) -> Vector2i {
        Vector2i {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector2i {
    type Output = Vector2i;

    #[inline]
    fn sub(self, rhs: Vector2i) -> Vector2i {
        Vector2i {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Neg for Vector2i {
    type Output = Vector2i;

    #[inline]
    fn neg(self) -> Vector2i {
        Vector2i {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Mul<i32> for Vector2i {
    type Output = Vector2i;

    #[inline]
    fn mul(self, rhs: i32) -> Vector2i {
        Vector2i {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vector2i> for i32 {
    type Output = Vector2i;

    #[inline]
    fn mul(self, rhs: Vector2i) -> Vector2i {
        Vector2i {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Div<i32> for Vector2i {
    type Output = Vector2i;

    #[inline]
    fn div(self, rhs: i32) -> Vector2i {
        Vector2i {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Div<Vector2i> for i32 {
    type Output = Vector2i;

    #[inline]
    fn div(self, rhs: Vector2i) -> Vector2i {
        Vector2i {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
}
