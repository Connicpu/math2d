//! Mathematical vector on the 2D (x, y) plane.

use sizeu::Sizeu;
use vector2f::Vector2f;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Mathematical vector on the 2D (x, y) plane.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Vector2i {
    /// Horizontal component.
    pub x: i32,
    /// Vertical component.
    pub y: i32,
}

impl Vector2i {
    /// The zero vector. Addition with the zero vector is the identity function.
    pub const ZERO: Vector2i = Vector2i { x: 0, y: 0 };

    /// Construct a vector from the components
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Vector2i { x, y }
    }

    /// Converts this vector to floating point components.
    #[inline]
    pub fn to_f32(self) -> Vector2f {
        Vector2f {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    /// Converts this vector to a size. Ensure the values are positive or you
    /// will experience casting underflow.
    #[inline]
    pub fn as_size(self) -> Sizeu {
        Sizeu {
            width: self.x as u32,
            height: self.y as u32,
        }
    }

    /// Returns the absolute values of the components.
    #[inline]
    pub fn abs(self) -> Vector2i {
        Vector2i::new(self.x.abs(), self.y.abs())
    }
}

impl<V> Add<V> for Vector2i
where
    V: Into<Vector2i>,
{
    type Output = Vector2i;

    #[inline]
    fn add(self, rhs: V) -> Vector2i {
        let rhs = rhs.into();
        Vector2i {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<V> Sub<V> for Vector2i
where
    V: Into<Vector2i>,
{
    type Output = Vector2i;

    #[inline]
    fn sub(self, rhs: V) -> Vector2i {
        let rhs = rhs.into();
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

impl<V> AddAssign<V> for Vector2i
where
    Vector2i: Add<V, Output = Vector2i>,
{
    fn add_assign(&mut self, v: V) {
        *self = *self + v;
    }
}

impl<V> SubAssign<V> for Vector2i
where
    Vector2i: Sub<V, Output = Vector2i>,
{
    fn sub_assign(&mut self, v: V) {
        *self = *self - v;
    }
}

impl<V> MulAssign<V> for Vector2i
where
    Vector2i: Mul<V, Output = Vector2i>,
{
    fn mul_assign(&mut self, v: V) {
        *self = *self * v;
    }
}

impl<V> DivAssign<V> for Vector2i
where
    Vector2i: Div<V, Output = Vector2i>,
{
    fn div_assign(&mut self, v: V) {
        *self = *self / v;
    }
}

impl From<[i32; 2]> for Vector2i {
    #[inline]
    fn from(v: [i32; 2]) -> Vector2i {
        Vector2i::new(v[0], v[1])
    }
}

impl From<Vector2i> for [i32; 2] {
    fn from(v: Vector2i) -> [i32; 2] {
        [v.x, v.y]
    }
}

#[cfg(feature = "mint")]
impl From<Vector2i> for mint::Vector2<i32> {
    #[inline]
    fn from(p: Vector2i) -> mint::Vector2<i32> {
        mint::Vector2 { x: p.x, y: p.y }
    }
}

#[cfg(feature = "mint")]
impl From<mint::Vector2<i32>> for Vector2i {
    #[inline]
    fn from(p: mint::Vector2<i32>) -> Vector2i {
        Vector2i { x: p.x, y: p.y }
    }
}

#[cfg(test)]
mod tests {
    use vector2i::Vector2i;

    #[test]
    fn addition() {
        let val = Vector2i::ZERO + [1, 2] + [3, 4];
        assert_eq!(val, Vector2i::new(4, 6));
    }

    #[test]
    fn subtraction() {
        let val = Vector2i::ZERO - [5, 3] + [2, 0] - [1, 1];
        assert_eq!(val, Vector2i::new(-4, -4));
    }
}
