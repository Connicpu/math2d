use crate::vector2i::Vector2i;

use std::ops::{Add, Div, Mul, Neg, Sub};

#[cfg(all(windows, feature = "d2d"))]
use winapi::um::dcommon::D2D_VECTOR_2F;

/// Mathematical point on the 2D (x, y) plane
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}

pub const ZERO: Vector2f = Vector2f::ZERO;
pub const ONE: Vector2f = Vector2f::ONE;

impl Vector2f {
    /// Mathematical origin point on the real number plane
    pub const ZERO: Vector2f = Vector2f { x: 0.0, y: 0.0 };
    pub const ONE: Vector2f = Vector2f { x: 1.0, y: 1.0 };

    /// Construct a vector from the components
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Vector2f { x, y }
    }

    #[inline]
    pub fn to_i32(self) -> Vector2i {
        Vector2i {
            x: self.x as i32,
            y: self.y as i32,
        }
    }

    #[inline]
    pub fn rounded(self) -> Vector2f {
        Vector2f {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    #[inline]
    pub fn dot(self, rhs: Vector2f) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    #[inline]
    pub fn len_squared(self) -> f32 {
        self.dot(self)
    }

    #[inline]
    pub fn len(self) -> f32 {
        self.len_squared().sqrt()
    }

    #[inline]
    pub fn is_approx_eq(self, other: impl Into<Vector2f>, epsilon: f32) -> bool {
        let other = other.into();
        return (self.x - other.x).abs() <= epsilon
            && (self.y - other.y).abs() <= epsilon;
    }
}

impl Add for Vector2f {
    type Output = Vector2f;

    #[inline]
    fn add(self, rhs: Vector2f) -> Vector2f {
        Vector2f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector2f {
    type Output = Vector2f;

    #[inline]
    fn sub(self, rhs: Vector2f) -> Vector2f {
        Vector2f {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Neg for Vector2f {
    type Output = Vector2f;

    #[inline]
    fn neg(self) -> Vector2f {
        Vector2f {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Mul<f32> for Vector2f {
    type Output = Vector2f;

    #[inline]
    fn mul(self, rhs: f32) -> Vector2f {
        Vector2f {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vector2f> for f32 {
    type Output = Vector2f;

    #[inline]
    fn mul(self, rhs: Vector2f) -> Vector2f {
        Vector2f {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Div<f32> for Vector2f {
    type Output = Vector2f;

    #[inline]
    fn div(self, rhs: f32) -> Vector2f {
        Vector2f {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Div<Vector2f> for f32 {
    type Output = Vector2f;

    #[inline]
    fn div(self, rhs: Vector2f) -> Vector2f {
        Vector2f {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
}

impl From<f32> for Vector2f {
    #[inline]
    fn from(s: f32) -> Vector2f {
        Vector2f::new(s, s)
    }
}

impl From<(f32, f32)> for Vector2f {
    #[inline]
    fn from((x, y): (f32, f32)) -> Vector2f {
        Vector2f::new(x, y)
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<Vector2f> for D2D_VECTOR_2F {
    #[inline]
    fn from(vec: Vector2f) -> D2D_VECTOR_2F {
        D2D_VECTOR_2F { x: vec.x, y: vec.y }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<D2D_VECTOR_2F> for Vector2f {
    #[inline]
    fn from(vec: D2D_VECTOR_2F) -> Vector2f {
        Vector2f { x: vec.x, y: vec.y }
    }
}
