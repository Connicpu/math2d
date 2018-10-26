use point2f::Point2f;
use vector2f::Vector2f;

use std::f32::EPSILON;
use std::ops::Mul;

use mint;
#[cfg(all(windows, feature = "d2d"))]
use winapi::um::dcommon::D2D_MATRIX_3X2_F;

    /// The 2D affine identity matrix.
pub const IDENTITY: Matrix3x2f = Matrix3x2f::IDENTITY;

/// 2D Affine Transformation Matrix.
///
/// Mathematically you can think of this matrix as if it were the following:
/// ```
/// # let (a,b,c,d,x,y) = (0,0,0,0,0,0);
/// # let _ =
/// [a, b, 0]
/// # ; let _ =
/// [c, d, 0]
/// # ; let _ =
/// [x, y, 1]
/// # ;
/// ```
///
/// ### Composing matrices
///
/// Affine transformations are performed in "Row-Major" order. What this means,
/// if you're familiar with linear algebra, is that when you compose multiple
/// affine transformations together, the matrix representing the set of operations
/// that should happen "first" must be the left hand operand of the multiplication
/// operator.
///
/// This is also why points and vectors are the left-hand operand when multiplied
/// with matrices.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Matrix3x2f {
    /// Horizontal scaling / cosine of rotation
    pub a: f32,
    /// Vertical shear / sine of rotation
    pub b: f32,
    /// Horizontal shear / negative sine of rotation
    pub c: f32,
    /// Vertical scaling / cosine of rotation
    pub d: f32,
    /// Horizontal translation (always orthogonal regardless of rotation)
    pub x: f32,
    /// Vertical translation (always orthogonal regardless of rotation)
    pub y: f32,
}

impl Matrix3x2f {
    /// The 2D affine identity matrix.
    pub const IDENTITY: Matrix3x2f = Matrix3x2f {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        x: 0.0,
        y: 0.0,
    };

    /// Construct the matrix from an array of the row values.
    #[inline]
    pub fn new(parts: [[f32; 2]; 3]) -> Matrix3x2f {
        Matrix3x2f {
            a: parts[0][0],
            b: parts[0][1],
            c: parts[1][0],
            d: parts[1][1],
            x: parts[2][0],
            y: parts[2][1],
        }
    }

    /// Constructs the matrix from a slice of 6 values as
    /// `[a, b, c, d, x, y]`.
    ///
    /// Panics if `values` does not contain exactly 6 elements.
    #[inline]
    pub fn from_slice(values: &[f32]) -> Matrix3x2f {
        assert_eq!(values.len(), 6);
        Matrix3x2f {
            a: values[0],
            b: values[1],
            c: values[2],
            d: values[3],
            x: values[4],
            y: values[5],
        }
    }

    /// Constructs the matrix from a tuple of 6 values as
    /// `(a, b, c, d, x, y)`.
    #[inline]
    pub fn from_tuple(values: (f32, f32, f32, f32, f32, f32)) -> Matrix3x2f {
        let (a, b, c, d, x, y) = values;
        Matrix3x2f { a, b, c, d, x, y }
    }

    /// Creates an affine translation matrix that translates points by the passed
    /// vector. The linear part of the matrix is the identity.
    ///
    /// ![Example Translation][1]
    ///
    /// [Read More][2]
    ///
    /// [1]: https://docs.microsoft.com/en-us/windows/desktop/Direct2D/images/translation-ovw.png
    /// [2]: https://docs.microsoft.com/en-us/windows/desktop/Direct2D/how-to-translate
    #[inline]
    pub fn translation(trans: impl Into<Vector2f>) -> Matrix3x2f {
        let trans = trans.into();

        Matrix3x2f {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            x: trans.x,
            y: trans.y,
        }
    }

    /// Creates a scaling matrix that performs scaling around a specified point
    /// of origin. This is equivalent to translating the center point back to
    /// the origin, scaling around the origin by the scaling value, and then
    /// translating the center point back to its original location.
    ///
    /// ![Example Scaling][1]
    ///
    /// [Read More][2]
    ///
    /// [1]: https://docs.microsoft.com/en-us/windows/desktop/Direct2D/images/scale-ovw.png
    /// [2]: https://docs.microsoft.com/en-us/windows/desktop/Direct2D/how-to-scale
    #[inline]
    pub fn scaling(scale: impl Into<Vector2f>, center: impl Into<Point2f>) -> Matrix3x2f {
        let scale = scale.into();
        let center = center.into();

        Matrix3x2f {
            a: scale.x,
            b: 0.0,
            c: 0.0,
            d: scale.y,
            x: center.x - scale.x * center.x,
            y: center.y - scale.y * center.y,
        }
    }

    /// Creates a rotation matrix that performs rotation around a specified point
    /// of origin. This is equivalent to translating the center point back to the
    /// origin, rotating around the origin by the specified angle, and then
    /// translating the center point back to its original location.
    ///
    /// ![Example Rotation][1]
    ///
    /// [Read More][2]
    ///
    /// [1]: https://docs.microsoft.com/en-us/windows/desktop/Direct2D/images/rotate-ovw.png
    /// [2]: https://docs.microsoft.com/en-us/windows/desktop/Direct2D/how-to-rotate
    #[inline]
    pub fn rotation(angle: f32, center: impl Into<Point2f>) -> Matrix3x2f {
        let center = center.into();
        let cos = angle.cos();
        let sin = angle.sin();

        Matrix3x2f {
            a: cos,
            b: sin,
            c: -sin,
            d: cos,
            x: center.x - cos * center.x - sin * center.y,
            y: center.y - cos * center.y - sin * center.x,
        }
    }

    /// Creates a matrix that skews an object by a tangent angle around the center point.
    ///
    /// ![Example Effect of Skewing][1]
    ///
    /// [Read More][2]
    ///
    /// [1]: https://docs.microsoft.com/en-us/windows/desktop/Direct2D/images/skew-ovw.png
    /// [2]: https://docs.microsoft.com/en-us/windows/desktop/Direct2D/how-to-skew
    #[inline]
    pub fn skew(angle_x: f32, angle_y: f32, center: impl Into<Point2f>) -> Matrix3x2f {
        let center = center.into();
        let tanx = angle_x.tan();
        let tany = angle_y.tan();

        Matrix3x2f {
            a: 1.0,
            b: tany,
            c: tanx,
            d: 1.0,
            x: -center.y * tany,
            y: -center.x * tanx,
        }
    }

    /// Returns the determinant of the matrix. Since this matrix is conceptually 3x3, and the
    /// bottom-right element is always 1, this value works out to be `a * d - b * c`.
    #[inline]
    pub fn determinant(&self) -> f32 {
        self.a * self.d - self.b * self.c
    }

    /// Determines if the `inverse` or `try_inverse` functions would succeed if called. A
    /// matrix is invertible if its determinant is nonzero. Since we're dealing with floats,
    /// we check that the absolute value of the determinant is greater than f32::EPSILON.
    #[inline]
    pub fn is_invertible(&self) -> bool {
        Matrix3x2f::det_shows_invertible(self.determinant())
    }

    /// Calculates the inverse of this matrix. Panics if the matrix is not invertible (see above).
    #[inline]
    pub fn inverse(&self) -> Matrix3x2f {
        let det = self.determinant();
        assert!(Matrix3x2f::det_shows_invertible(det));

        self.unchecked_inverse(det)
    }

    /// Calculates the inverse of the matrix. Returns None if the determinant is less than
    /// f32::EPSILON.
    #[inline]
    pub fn try_inverse(&self) -> Option<Matrix3x2f> {
        let det = self.determinant();
        if Matrix3x2f::det_shows_invertible(det) {
            Some(self.unchecked_inverse(det))
        } else {
            None
        }
    }

    /// Performs the inverse of the matrix without checking for invertibility.
    /// 
    /// *WARNING: If this matrix is not invertible, you may get NaN or INF!*
    #[inline]
    pub fn unchecked_inverse(&self, det: f32) -> Matrix3x2f {
        Matrix3x2f {
            a: self.d / det,
            b: self.b / -det,
            c: self.c / -det,
            d: self.a / det,
            x: (self.d * self.x - self.c * self.y) / -det,
            y: (self.b * self.x - self.a * self.y) / det,
        }
    }

    /// Compose a matrix from a scaling, rotation, and translation value
    /// (combined in that order).
    #[inline]
    pub fn compose(
        scaling: impl Into<Vector2f>,
        rotation: f32,
        translation: impl Into<Vector2f>,
    ) -> Matrix3x2f {
        let s = scaling.into();
        let cos = rotation.cos();
        let sin = rotation.sin();
        let trans = translation.into();

        Matrix3x2f {
            a: s.x * cos,
            b: s.y * sin,
            c: s.x * -sin,
            d: s.y * cos,
            x: trans.x,
            y: trans.y,
        }
    }

    /// Decomposes a simple affine transformation into its scaling, rotation, and
    /// translation parts.
    #[inline]
    pub fn decompose(&self) -> Decomposition {
        Decomposition {
            translation: [self.x, self.y].into(),
            scaling: Vector2f {
                x: (self.a * self.a + self.c * self.c).sqrt(),
                y: (self.b * self.b + self.d * self.d).sqrt(),
            },
            rotation: self.b.atan2(self.d),
        }
    }

    /// A more explicit way to do `point * matrix`, while also allowing any type
    /// that may be converted into a Point2F with a From/Into impl.
    #[inline]
    pub fn transform_point(&self, point: impl Into<Point2f>) -> Point2f {
        point.into() * *self
    }

    /// A more explicit way to do `vec * matrix`, while also allowing any type
    /// that may be converted into a Vector2F with a From/Into impl.
    #[inline]
    pub fn transform_vector(&self, vec: impl Into<Vector2f>) -> Vector2f {
        vec.into() * *self
    }

    /// Returns this matrix as a 3x3 float array using the mathematical form
    /// described above.
    #[inline]
    pub fn to_row_major(&self) -> [[f32; 3]; 3] {
        [
            [self.a, self.b, 0.0],
            [self.c, self.d, 0.0],
            [self.x, self.y, 1.0],
        ]
    }

    /// Returns the matrix as a 3x3 float array in column major form, i.e.
    /// the transpose of the row-major version.
    #[inline]
    pub fn to_column_major(&self) -> [[f32; 3]; 3] {
        [
            [self.a, self.c, self.x],
            [self.b, self.d, self.y],
            [0.0, 0.0, 1.0],
        ]
    }

    /// Checks if two matrices are approximately equal given an epsilon value.
    #[inline]
    pub fn is_approx_eq(&self, other: &Matrix3x2f, epsilon: f32) -> bool {
        return (self.a - other.a).abs() < epsilon
            && (self.b - other.b).abs() < epsilon
            && (self.c - other.c).abs() < epsilon
            && (self.d - other.d).abs() < epsilon
            && (self.x - other.x).abs() < epsilon
            && (self.y - other.y).abs() < epsilon;
    }

    /// Checks if this matrix is equal to the identity matrix within 1e-5
    #[inline]
    pub fn is_identity(&self) -> bool {
        self.is_approx_eq(&Matrix3x2f::IDENTITY, 1e-5)
    }

    #[inline]
    fn det_shows_invertible(det: f32) -> bool {
        det.abs() > EPSILON
    }
}

impl Mul for Matrix3x2f {
    type Output = Matrix3x2f;

    #[inline]
    fn mul(self, rhs: Matrix3x2f) -> Matrix3x2f {
        let lhs = self;

        Matrix3x2f {
            a: lhs.a * rhs.a + lhs.b * rhs.c,
            b: lhs.a * rhs.b + lhs.b * rhs.d,
            c: lhs.c * rhs.a + lhs.d * rhs.c,
            d: lhs.c * rhs.b + lhs.d * rhs.d,
            x: lhs.x * rhs.a + lhs.y * rhs.c + rhs.x,
            y: lhs.x * rhs.b + lhs.y * rhs.d + rhs.y,
        }
    }
}

impl Mul<Matrix3x2f> for Point2f {
    type Output = Point2f;

    #[inline]
    fn mul(self, m: Matrix3x2f) -> Point2f {
        Point2f {
            x: self.x * m.a + self.y * m.c + m.x,
            y: self.x * m.b + self.y * m.d + m.y,
        }
    }
}

impl Mul<Matrix3x2f> for Vector2f {
    type Output = Vector2f;

    #[inline]
    fn mul(self, m: Matrix3x2f) -> Vector2f {
        Vector2f {
            x: self.x * m.a + self.y * m.c,
            y: self.x * m.b + self.y * m.d,
        }
    }
}

impl From<[[f32; 2]; 3]> for Matrix3x2f {
    #[inline]
    fn from(parts: [[f32; 2]; 3]) -> Matrix3x2f {
        Matrix3x2f::new(parts)
    }
}

impl From<Matrix3x2f> for [[f32; 2]; 3] {
    #[inline]
    fn from(m: Matrix3x2f) -> [[f32; 2]; 3] {
        [[m.a, m.b], [m.c, m.d], [m.x, m.y]]
    }
}

impl From<Matrix3x2f> for [[f32; 3]; 3] {
    #[inline]
    fn from(m: Matrix3x2f) -> [[f32; 3]; 3] {
        m.to_row_major()
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<Matrix3x2f> for D2D_MATRIX_3X2_F {
    #[inline]
    fn from(m: Matrix3x2f) -> D2D_MATRIX_3X2_F {
        let matrix: [[f32; 2]; 3] = m.into();
        D2D_MATRIX_3X2_F { matrix }
    }
}

#[cfg(all(windows, feature = "d2d"))]
impl From<D2D_MATRIX_3X2_F> for Matrix3x2f {
    #[inline]
    fn from(m: D2D_MATRIX_3X2_F) -> Matrix3x2f {
        Matrix3x2f::new(m.matrix)
    }
}

impl Default for Matrix3x2f {
    #[inline]
    fn default() -> Self {
        Matrix3x2f::IDENTITY
    }
}

/// Represents a decomposition of a non-skewing matrix i.e. one made up of
/// only rotations, translations, and scalings.
pub struct Decomposition {
    /// Total scaling applied in the transformation. This operation is applied
    /// first if the decomposition is recomposed.
    pub scaling: Vector2f,
    /// Total rotation applied in the transformation. This operation is applied
    /// second if the decomposition is recomposed.
    pub rotation: f32,
    /// Total translation applied in the transformation. This operation is
    /// applied last if the decomposition is recomposed.
    pub translation: Vector2f,
}

impl From<Decomposition> for Matrix3x2f {
    #[inline]
    fn from(decomp: Decomposition) -> Matrix3x2f {
        Matrix3x2f::compose(decomp.scaling, decomp.rotation, decomp.translation)
    }
}

#[cfg(feature = "mint")]
impl From<Matrix3x2f> for mint::RowMatrix3x2<f32> {
    #[inline]
    fn from(mat: Matrix3x2f) -> mint::RowMatrix3x2<f32> {
        mint::RowMatrix3x2 {
            x: [mat.a, mat.b].into(),
            y: [mat.c, mat.d].into(),
            z: [mat.x, mat.y].into(),
        }
    }
}

#[cfg(all(test, windows, feature = "d2d"))]
#[test]
fn mat32_d2d_bin_compat() {
    use std::mem::size_of_val;

    fn ptr_eq<T>(a: &T, b: &T) -> bool {
        (a as *const T) == (b as *const T)
    }

    let mat = Matrix3x2f::IDENTITY;
    let d2d = unsafe { &*((&mat) as *const _ as *const D2D_MATRIX_3X2_F) };

    assert!(ptr_eq(&mat.a, &d2d.matrix[0][0]));
    assert!(ptr_eq(&mat.b, &d2d.matrix[0][1]));
    assert!(ptr_eq(&mat.c, &d2d.matrix[1][0]));
    assert!(ptr_eq(&mat.d, &d2d.matrix[1][1]));
    assert!(ptr_eq(&mat.x, &d2d.matrix[2][0]));
    assert!(ptr_eq(&mat.y, &d2d.matrix[2][1]));
    assert_eq!(size_of_val(&mat), size_of_val(d2d));
}
