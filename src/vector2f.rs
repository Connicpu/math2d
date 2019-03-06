//! Mathematical vector on the 2D (x, y) plane.

use crate::point2f::Point2f;
use crate::sizef::Sizef;
use crate::vector2i::Vector2i;

use std::ops::{Add, Div, Mul, Neg, Sub};

#[cfg(all(windows, feature = "d2d"))]
use winapi::um::dcommon::D2D_VECTOR_2F;

/// Mathematical vector on the 2D (x, y) plane.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Vector2f {
    /// Horizontal component.
    pub x: f32,
    /// Vertical component.
    pub y: f32,
}

/// Zero vector, addition identity value.
pub const ZERO: Vector2f = Vector2f::ZERO;
/// One vector, multiplication identity value.
pub const ONE: Vector2f = Vector2f::ONE;

impl Vector2f {
    /// Zero vector, addition identity value.
    pub const ZERO: Vector2f = Vector2f { x: 0.0, y: 0.0 };
    /// One vector, multiplication identity value.
    pub const ONE: Vector2f = Vector2f { x: 1.0, y: 1.0 };

    /// Up vector in the top-left coordinate system common to
    /// 2D drawing systems.
    pub const UP: Vector2f = Vector2f { x: 0.0, y: -1.0 };
    /// Right vector in the top-left coordinate system common to
    /// 2D drawing systems.
    pub const RIGHT: Vector2f = Vector2f { x: 1.0, y: 0.0 };
    /// Down vector in the top-left coordinate system common to
    /// 2D drawing systems.
    pub const DOWN: Vector2f = Vector2f { x: 0.0, y: 1.0 };
    /// Left vector in the top-left coordinate system common to
    /// 2D drawing systems.
    pub const LEFT: Vector2f = Vector2f { x: -1.0, y: 0.0 };

    /// Construct a vector from the components.
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Vector2f { x, y }
    }

    /// Converts the vector to unsigned integer values. Truncates integers, if
    /// you want your components to be rounded you must do this manually first.
    #[inline]
    pub fn to_i32(self) -> Vector2i {
        Vector2i {
            x: self.x as i32,
            y: self.y as i32,
        }
    }

    #[inline]
    pub fn to_point(self) -> Point2f {
        Point2f::ORIGIN + self
    }

    /// Converts this vector to a size value with the x representing width
    /// and the y representing height.
    #[inline]
    pub fn to_size(self) -> Sizef {
        Sizef {
            width: self.x,
            height: self.y,
        }
    }

    /// Rounds the components of the vector to the nearest integer. Rounds
    /// half-way values away from 0.
    #[inline]
    pub fn rounded(self) -> Vector2f {
        Vector2f {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    /// Dot product of two vectors.
    #[inline]
    pub fn dot(self, rhs: Vector2f) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    /// The squared length of the vector
    #[inline]
    pub fn len_squared(self) -> f32 {
        self.dot(self)
    }

    /// The length of the vector. This requires performing a square root,
    /// so the squared length should be preferred where possible.
    #[inline]
    pub fn len(self) -> f32 {
        self.len_squared().sqrt()
    }

    /// Absolute value of the vector components.
    #[inline]
    pub fn abs(self) -> Self {
        Vector2f {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    /// Tests if two vectors are approximately equal to each other within a
    /// given epsilon. The epsilon is applied component-wise. If you would like
    /// to check that two vectors are within a specified distance of each
    /// other, you should subtract one from the other and check the length of
    /// the resulting distance vector between them.
    #[inline]
    pub fn is_approx_eq(self, other: impl Into<Vector2f>, epsilon: f32) -> bool {
        let other = other.into();
        return (self.x - other.x).abs() <= epsilon && (self.y - other.y).abs() <= epsilon;
    }
}

impl<V> Add<V> for Vector2f
where
    V: Into<Vector2f>,
{
    type Output = Vector2f;

    #[inline]
    fn add(self, rhs: V) -> Vector2f {
        let rhs = rhs.into();
        Vector2f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<V> Sub<V> for Vector2f
where
    V: Into<Vector2f>,
{
    type Output = Vector2f;

    #[inline]
    fn sub(self, rhs: V) -> Vector2f {
        let rhs = rhs.into();
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

impl<V> Mul<V> for Vector2f
where
    V: Into<Vector2f>,
{
    type Output = Vector2f;

    #[inline]
    fn mul(self, rhs: V) -> Self {
        let rhs = rhs.into();
        Vector2f {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
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
        let recip = 1.0 / rhs;
        Vector2f {
            x: self.x * recip,
            y: self.y * recip,
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

impl From<[f32; 2]> for Vector2f {
    #[inline]
    fn from(v: [f32; 2]) -> Vector2f {
        Vector2f::new(v[0], v[1])
    }
}

impl From<Vector2f> for [f32; 2] {
    #[inline]
    fn from(v: Vector2f) -> [f32; 2] {
        [v.x, v.y]
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

#[cfg(feature = "mint")]
impl From<Vector2f> for mint::Vector2<f32> {
    #[inline]
    fn from(p: Vector2f) -> mint::Vector2<f32> {
        mint::Vector2 { x: p.x, y: p.y }
    }
}

#[cfg(feature = "mint")]
impl From<mint::Vector2<f32>> for Vector2f {
    #[inline]
    fn from(p: mint::Vector2<f32>) -> Vector2f {
        Vector2f { x: p.x, y: p.y }
    }
}

#[cfg(feature = "kurbo")]
impl From<kurbo::Vec2> for Vector2f {
    #[inline]
    fn from(p: kurbo::Vec2) -> Vector2f {
        Vector2f {
            x: p.x as f32,
            y: p.y as f32,
        }
    }
}

#[cfg(all(test, windows, feature = "d2d"))]
#[test]
fn vec2f_d2d_bin_compat() {
    use std::mem::size_of_val;

    fn ptr_eq<T>(a: &T, b: &T) -> bool {
        (a as *const T) == (b as *const T)
    }

    let vec = Vector2f::ZERO;
    let d2d = unsafe { &*((&vec) as *const _ as *const D2D_VECTOR_2F) };

    assert!(ptr_eq(&vec.x, &d2d.x));
    assert!(ptr_eq(&vec.y, &d2d.y));
    assert_eq!(size_of_val(&vec), size_of_val(d2d));
}
