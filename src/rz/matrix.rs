#![allow(unused_imports)]
use std::convert::{identity, From};
use std::f64::consts::PI;
use std::ops::{Index, IndexMut, Mul};

#[allow(unused_imports)]
use super::tuple::{point, vector, Tuple};

type F = f64;

#[derive(Debug, Copy, Clone)]
pub struct Matrix<const D: usize> {
    data: [[F; D]; D],
}

impl<const D: usize> From<[[F; D]; D]> for Matrix<D> {
    fn from(data: [[F; D]; D]) -> Self {
        Matrix { data }
    }
}

impl<const D: usize> Default for Matrix<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const D: usize> Matrix<D> {
    pub fn new() -> Matrix<D> {
        Matrix::from([[0.0; D]; D])
    }
}

impl<const D: usize> Index<usize> for Matrix<D> {
    type Output = [F; D];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const D: usize> IndexMut<usize> for Matrix<D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Matrix<4> {
    pub fn translate(self, x: F, y: F, z: F) -> Self {
        Self::translation(x, y, z) * self
    }

    pub fn scale(self, x: F, y: F, z: F) -> Self {
        Self::scaling(x, y, z) * self
    }

    pub fn rotate_x(self, r: F) -> Self {
        Self::rotation_x(r) * self
    }

    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn translation(x: F, y: F, z: F) -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn scaling(x: F, y: F, z: F) -> Self {
        Self {
            data: [
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotation_x(r: F) -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, r.cos(), -r.sin(), 0.0],
                [0.0, r.sin(), r.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotation_y(r: F) -> Self {
        Self {
            data: [
                [r.cos(), 0.0, r.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-r.sin(), 0.0, r.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotation_z(r: F) -> Self {
        Self {
            data: [
                [r.cos(), -r.sin(), 0.0, 0.0],
                [r.sin(), r.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn shear(xy: F, xz: F, yx: F, yz: F, zx: F, zy: F) -> Self {
        Self {
            data: [
                [1.0, xy, xz, 0.0],
                [yx, 1.0, yz, 0.0],
                [zx, zy, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn view(from: Tuple, to: Tuple, up: Tuple) -> Self {
        let forward = (to - from).normalized();
        let up = up.normalized();
        let left = forward.cross(up).normalized();
        let up = left.cross(forward);

        let orientation = Self {
            data: [
                [left.x, left.y, left.z, 0.0],
                [up.x, up.y, up.z, 0.0],
                [-forward.x, -forward.y, -forward.z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        orientation * Matrix::translation(-from.x, -from.y, -from.z)
    }

    pub fn transposed(&self) -> Self {
        let mut m = Matrix::new();

        for r in 0..4 {
            for c in 0..4 {
                m[r][c] = self[c][r]
            }
        }

        m
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix<3> {
        let r0 = match col {
            0 => [self[0][1], self[0][2], self[0][3]],
            1 => [self[0][0], self[0][2], self[0][3]],
            2 => [self[0][0], self[0][1], self[0][3]],
            _ => [self[0][0], self[0][1], self[0][2]],
        };

        let r1 = match col {
            0 => [self[1][1], self[1][2], self[1][3]],
            1 => [self[1][0], self[1][2], self[1][3]],
            2 => [self[1][0], self[1][1], self[1][3]],
            _ => [self[1][0], self[1][1], self[1][2]],
        };

        let r2 = match col {
            0 => [self[2][1], self[2][2], self[2][3]],
            1 => [self[2][0], self[2][2], self[2][3]],
            2 => [self[2][0], self[2][1], self[2][3]],
            _ => [self[2][0], self[2][1], self[2][2]],
        };

        let r3 = match col {
            0 => [self[3][1], self[3][2], self[3][3]],
            1 => [self[3][0], self[3][2], self[3][3]],
            2 => [self[3][0], self[3][1], self[3][3]],
            _ => [self[3][0], self[3][1], self[3][2]],
        };

        Matrix::from(match row {
            0 => [r1, r2, r3],
            1 => [r0, r2, r3],
            2 => [r0, r1, r3],
            _ => [r0, r1, r2],
        })
    }

    pub fn minor(&self, row: usize, col: usize) -> F {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> F {
        let sign = if (row + col) % 2 == 0 { 1.0 } else { -1.0 };

        sign * self.minor(row, col)
    }

    pub fn determinant(&self) -> F {
        let mut det = 0.0;

        for col in 0..4 {
            det += self[0][col] * self.cofactor(0, col);
        }

        det
    }

    pub fn inverse(&self) -> Self {
        let det = self.determinant();
        if det == 0.0 {
            panic!("Matrix is not invertable");
        }

        let mut m = Matrix::new();
        for row in 0..4 {
            for col in 0..4 {
                let c = self.cofactor(row, col);
                m[col][row] = c / det;
            }
        }

        m
    }
}

impl Matrix<3> {
    #[rustfmt::skip]
    pub fn identity3() -> Matrix<3> {
        Self {
            data: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix<2> {
        let r0 = match col {
            0 => [self[0][1], self[0][2]],
            1 => [self[0][0], self[0][2]],
            _ => [self[0][0], self[0][1]],
        };

        let r1 = match col {
            0 => [self[1][1], self[1][2]],
            1 => [self[1][0], self[1][2]],
            _ => [self[1][0], self[1][1]],
        };

        let r2 = match col {
            0 => [self[2][1], self[2][2]],
            1 => [self[2][0], self[2][2]],
            _ => [self[2][0], self[2][1]],
        };

        Matrix::from(match row {
            0 => [r1, r2],
            1 => [r0, r2],
            _ => [r0, r1],
        })
    }

    pub fn minor(&self, row: usize, col: usize) -> F {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> F {
        let sign = if (row + col) % 2 == 0 { 1.0 } else { -1.0 };

        sign * self.minor(row, col)
    }

    pub fn determinant(&self) -> F {
        let mut det = 0.0;

        for col in 0..3 {
            det += self[0][col] * self.cofactor(0, col);
        }

        det
    }
}

impl Matrix<2> {
    #[rustfmt::skip]
    pub fn identity2() -> Matrix<2> {
        Self {
            data: [
                [1.0, 0.0],
                [0.0, 1.0],
            ],
        }
    }

    pub fn determinant(&self) -> F {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl Mul for Matrix<4> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let mut m = Matrix::new();

        for r in 0..4 {
            for c in 0..4 {
                m[r][c] = self[r][0] * other[0][c]
                    + self[r][1] * other[1][c]
                    + self[r][2] * other[2][c]
                    + self[r][3] * other[3][c];
            }
        }

        m
    }
}

impl Mul<F> for Matrix<4> {
    type Output = Self;
    fn mul(self, other: F) -> Self::Output {
        let mut m = Matrix::new();

        for r in 0..4 {
            for c in 0..4 {
                m[r][c] *= other;
            }
        }

        m
    }
}

impl Mul<Tuple> for Matrix<4> {
    type Output = Tuple;
    fn mul(self, other: Tuple) -> Self::Output {
        let mut v = Tuple::new(0.0, 0.0, 0.0, 0.0);

        for r in 0..4 {
            for c in 0..4 {
                v[r] += self[r][c] * other[c];
            }
        }

        v
    }
}

impl<const D: usize> PartialEq<Self> for Matrix<D> {
    fn eq(&self, other: &Self) -> bool {
        for row in 0..D {
            for col in 0..D {
                if (self[row][col] - other[row][col]).abs() > 0.00001 {
                    return false;
                }
            }
        }

        true
    }
}

#[test]
fn constructing_a_4x4_matrix() {
    let m = Matrix::from([
        [1.0, 2.0, 3.0, 4.0],
        [5.5, 6.5, 7.5, 8.5],
        [9.0, 10.0, 11.0, 12.0],
        [13.5, 14.5, 15.5, 16.5],
    ]);

    assert_eq!(m[0][0], 1.0);
    assert_eq!(m[0][3], 4.0);
    assert_eq!(m[1][0], 5.5);
    assert_eq!(m[1][2], 7.5);
    assert_eq!(m[2][2], 11.0);
    assert_eq!(m[3][0], 13.5);
    assert_eq!(m[3][2], 15.5);
}

#[test]
fn constructing_a_3x3_matrix() {
    let m = Matrix::from([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

    assert_eq!(m[0][0], -3.0);
    assert_eq!(m[1][1], -2.0);
    assert_eq!(m[2][2], 1.0);
}

#[test]
fn constructing_a_2x2_matrix() {
    let m = Matrix::from([[-3.0, 5.0], [1.0, -2.0]]);

    assert_eq!(m[0][0], -3.0);
    assert_eq!(m[0][1], 5.0);
    assert_eq!(m[1][0], 1.0);
    assert_eq!(m[1][1], -2.0);
}

#[test]
fn multiplying_two_matrices() {
    let a = Matrix::from([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);

    let b = Matrix::from([
        [-2.0, 1.0, 2.0, 3.0],
        [3.0, 2.0, 1.0, -1.0],
        [4.0, 3.0, 6.0, 5.0],
        [1.0, 2.0, 7.0, 8.0],
    ]);

    let result = Matrix::from([
        [20.0, 22.0, 50.0, 48.0],
        [44.0, 54.0, 114.0, 108.0],
        [40.0, 58.0, 110.0, 102.0],
        [16.0, 26.0, 46.0, 42.0],
    ]);

    assert_eq!(a * b, result);
}

#[test]
fn matrix_multiplied_by_a_tuple() {
    let a = Matrix::from([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 4.0, 4.0, 2.0],
        [8.0, 6.0, 4.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    let t = point(1.0, 2.0, 3.0);

    assert_eq!(a * t, point(18.0, 24.0, 33.0));
}

#[test]
fn transposing_a_matrix() {
    let a = Matrix::from([
        [0.0, 9.0, 3.0, 0.0],
        [9.0, 8.0, 0.0, 8.0],
        [1.0, 8.0, 5.0, 3.0],
        [0.0, 0.0, 5.0, 8.0],
    ]);

    let b = Matrix::from([
        [0.0, 9.0, 1.0, 0.0],
        [9.0, 8.0, 8.0, 0.0],
        [3.0, 0.0, 5.0, 5.0],
        [0.0, 8.0, 3.0, 8.0],
    ]);

    assert_eq!(a.transposed(), b);
}

#[test]
#[rustfmt::skip]
fn determinant_of_a_2x2_matrix() {
    let m = Matrix::from([
        [1.0, 5.0],
        [-3.0, 2.0]
    ]);

    assert_eq!(m.determinant(), 17.0);
}

#[test]
#[rustfmt::skip]
fn submatrix_of_3x3_matrix() {
    let a = Matrix::from([
        [ 1.0, 5.0,  0.0],
        [-3.0, 2.0,  7.0],
        [ 0.0, 6.0, -3.0],
    ]);

    let b = Matrix::from([
        [-3.0, 2.0],
        [ 0.0, 6.0]
    ]);

    assert_eq!(a.submatrix(0, 2), b);
}

#[test]
#[rustfmt::skip]
fn submatrix_of_4x4_matrix() {
    let a = Matrix::from([
        [-6.0, 1.0,  1.0, 6.0],
        [-8.0, 5.0,  8.0, 6.0],
        [-1.0, 0.0,  8.0, 2.0],
        [-7.0, 1.0, -1.0, 1.0],
    ]);
    
    let b = Matrix::from([
        [-6.0,  1.0, 6.0],
        [-8.0,  8.0, 6.0],
        [-7.0, -1.0, 1.0],
    ]);

    assert_eq!(a.submatrix(2, 1), b);
}

#[test]
#[rustfmt::skip]
fn minor_of_a_3x3_matrix() {
    let a = Matrix::from([
        [3.0,  5.0,  0.0], 
        [2.0, -1.0, -7.0], 
        [6.0, -1.0,  5.0],
    ]);

    let b= a.submatrix(1, 0);

    assert_eq!(b.determinant(), 25.0);
    assert_eq!(a.minor(1, 0), 25.0);
}

#[test]
#[rustfmt::skip]
fn cofactor_of_a_3x3_matrix() {
    let m = Matrix::from([
        [3.0,  5.0,  0.0], 
        [2.0, -1.0, -7.0],
        [6.0, -1.0,  5.0],
    ]);

    assert_eq!(m.minor(0, 0), -12.0);
    assert_eq!(m.cofactor(0, 0), -12.0);
    assert_eq!(m.minor(1, 0), 25.0);
    assert_eq!(m.cofactor(1, 0),-25.0);
}

#[test]
fn determinant_of_a_3x3_matrix() {
    let m = Matrix::from([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

    assert_eq!(m.cofactor(0, 0), 56.0);
    assert_eq!(m.cofactor(0, 1), 12.0);
    assert_eq!(m.cofactor(0, 2), -46.0);
    assert_eq!(m.determinant(), -196.0);
}

#[test]
fn determinant_of_a_4x4_matrix() {
    let m = Matrix::from([
        [-2.0, -8.0, 3.0, 5.0],
        [-3.0, 1.0, 7.0, 3.0],
        [1.0, 2.0, -9.0, 6.0],
        [-6.0, 7.0, 7.0, -9.0],
    ]);

    assert_eq!(m.cofactor(0, 0), 690.0);
    assert_eq!(m.cofactor(0, 1), 447.0);
    assert_eq!(m.cofactor(0, 2), 210.0);
    assert_eq!(m.cofactor(0, 3), 51.0);
    assert_eq!(m.determinant(), -4071.0);
}

#[test]
fn is_matrix_invertable() {
    let a = Matrix::from([
        [6.0, 4.0, 4.0, 4.0],
        [5.0, 5.0, 7.0, 6.0],
        [4.0, -9.0, 3.0, -7.0],
        [9.0, 1.0, 7.0, -6.0],
    ]);

    assert_eq!(a.determinant(), -2120.0);

    let b = Matrix::from([
        [-4.0, 2.0, -2.0, -3.0],
        [9.0, 6.0, 2.0, 6.0],
        [0.0, -5.0, 1.0, -5.0],
        [0.0, 0.0, 0.0, 0.0],
    ]);

    assert_eq!(b.determinant(), 0.0);
}

#[test]
fn invert_a_matrix1() {
    let a = Matrix::from([
        [-5.0, 2.0, 6.0, -8.0],
        [1.0, -5.0, 1.0, 8.0],
        [7.0, 7.0, -6.0, -7.0],
        [1.0, -3.0, 7.0, 4.0],
    ]);

    let a2 = a.inverse();

    let b = Matrix::from([
        [0.21805, 0.45113, 0.24060, -0.04511],
        [-0.80827, -1.45677, -0.44361, 0.52068],
        [-0.07895, -0.22368, -0.05263, 0.19737],
        [-0.52256, -0.81391, -0.30075, 0.30639],
    ]);

    assert_eq!(a.determinant(), 532.0);
    assert_eq!(a.cofactor(2, 3), -160.0);
    assert_eq!(a2[2][3], 105.0 / 532.0);
    assert_eq!(a.cofactor(3, 2), 105.0);
    assert_eq!(a2[2][3], 105.0 / 532.0);
    assert_eq!(a2, b);
}

#[test]
fn multiplying_by_inverse() {
    let a = Matrix::from([
        [3.0, -9.0, 7.0, 3.0],
        [3.0, -8.0, 2.0, -9.0],
        [-4.0, 4.0, 4.0, 1.0],
        [-6.0, 5.0, -1.0, 1.0],
    ]);

    let b = Matrix::from([
        [8.0, 2.0, 2.0, 2.0],
        [3.0, -1.0, 7.0, 0.0],
        [7.0, 0.0, 5.0, 4.0],
        [6.0, -2.0, 0.0, 5.0],
    ]);

    assert_eq!(a * b * b.inverse(), a);
}

#[test]
fn multiplying_by_a_translation_matrix() {
    let m = Matrix::translation(5.0, -3.0, 2.0);
    let p = point(-3.0, 4.0, 5.0);

    assert_eq!(m * p, point(2.0, 1.0, 7.0));
}

#[test]
fn multiplying_by_inverst_of_a_translation_matrix() {
    let m = Matrix::translation(5.0, -3.0, 2.0);
    let im = m.inverse();
    let p = point(-3.0, 4.0, 5.0);

    assert_eq!(im * p, point(-8.0, 7.0, 3.0));
}

#[test]
fn translation_does_not_affect_vectors() {
    let m = Matrix::translation(5.0, -3.0, 2.0);
    let v = vector(-3.0, 4.0, 5.0);

    assert_eq!(m * v, v);
}

#[test]
fn scaling_applied_to_point() {
    let m = Matrix::scaling(2.0, 3.0, 4.0);
    let p = point(-4.0, 6.0, 8.0);

    assert_eq!(m * p, point(-8.0, 18.0, 32.0));
}

#[test]
fn scaling_applied_to_vector() {
    let m = Matrix::scaling(2.0, 3.0, 4.0);
    let p = vector(-4.0, 6.0, 8.0);

    assert_eq!(m * p, vector(-8.0, 18.0, 32.0));
}

#[test]
fn inverse_scaling_applied_to_point() {
    let m = Matrix::scaling(2.0, 3.0, 4.0);
    let im = m.inverse();
    let p = vector(-4.0, 6.0, 8.0);

    assert_eq!(im * p, vector(-2.0, 2.0, 2.0));
}

#[test]
fn reflection_by_scaling() {
    let m = Matrix::scaling(-1.0, 1.0, 1.0);
    let p = point(2.0, 3.0, 4.0);

    assert_eq!(m * p, point(-2.0, 3.0, 4.0));
}

#[test]
fn rotating_around_x_axis() {
    let p = point(0.0, 1.0, 0.0);
    let half_quarter = Matrix::rotation_x(PI / 4.0);
    let full_quarter = Matrix::rotation_x(PI / 2.0);

    let sqrt2 = (2.0 as F).sqrt();

    assert_eq!(half_quarter * p, point(0.0, sqrt2 / 2.0, sqrt2 / 2.0));
    assert_eq!(full_quarter * p, point(0.0, 0.0, 1.0));
}

#[test]
fn inverse_rotation_around_x_axis() {
    let p = point(0.0, 1.0, 0.0);
    let half_quarter = Matrix::rotation_x(PI / 4.0);
    let inv = half_quarter.inverse();

    let sqrt2 = (2.0 as F).sqrt();

    assert_eq!(inv * p, point(0.0, sqrt2 / 2.0, -sqrt2 / 2.0));
}

#[test]
fn inverse_rotating_around_x_axis() {
    let p = point(0.0, 1.0, 0.0);
    let half_quarter = Matrix::rotation_x(PI / 4.0);
    let inv = half_quarter.inverse();
    let sqrt2 = (2.0 as F).sqrt();

    assert_eq!(inv * p, point(0.0, sqrt2 / 2.0, -sqrt2 / 2.0));
}

#[test]
fn rotating_around_y_axis() {
    let p = point(0.0, 0.0, 1.0);
    let half_quarter = Matrix::rotation_y(PI / 4.0);
    let full_quarter = Matrix::rotation_y(PI / 2.0);
    let sqrt2 = (2.0 as F).sqrt();

    assert_eq!(half_quarter * p, point(sqrt2 / 2.0, 0.0, sqrt2 / 2.0));
    assert_eq!(full_quarter * p, point(1.0, 0.0, 0.0));
}

#[test]
fn rotating_around_z_axis() {
    let p = point(0.0, 1.0, 0.0);
    let half_quarter = Matrix::rotation_z(PI / 4.0);
    let full_quarter = Matrix::rotation_z(PI / 2.0);
    let sqrt2 = (2.0 as F).sqrt();

    assert_eq!(half_quarter * p, point(-sqrt2 / 2.0, sqrt2 / 2.0, 0.0));
    assert_eq!(full_quarter * p, point(-1.0, 0.0, 0.0));
}

#[test]
fn shearing_x_to_z() {
    let m = Matrix::shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    let p = point(2.0, 3.0, 4.0);
    assert_eq!(m * p, point(6.0, 3.0, 4.0));
}

#[test]
fn shearing_y_to_x() {
    let m = Matrix::shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    let p = point(2.0, 3.0, 4.0);
    assert_eq!(m * p, point(2.0, 5.0, 4.0));
}

#[test]
fn shearing_y_to_z() {
    let m = Matrix::shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let p = point(2.0, 3.0, 4.0);
    assert_eq!(m * p, point(2.0, 7.0, 4.0));
}

#[test]
fn shearing_z_to_x() {
    let m = Matrix::shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    let p = point(2.0, 3.0, 4.0);
    assert_eq!(m * p, point(2.0, 3.0, 6.0));
}

#[test]
fn combining_transforms() {
    let p = point(1.0, 0.0, 1.0);
    let ma = Matrix::rotation_x(PI / 2.0);
    let mb = Matrix::scaling(5.0, 5.0, 5.0);
    let mc = Matrix::translation(10.0, 5.0, 7.0);
    let p2 = ma * p;
    let p3 = mb * p2;
    let p4 = mc * p3;

    assert_eq!(p2, point(1.0, -1.0, 0.0));
    assert_eq!(mb * p2, point(5.0, -5.0, 0.0));
    assert_eq!(p4, point(15.0, 0.0, 7.0));
}

#[test]
fn reverse_multiplication_of_transforms() {
    let p = point(1.0, 0.0, 1.0);
    let rotation = Matrix::rotation_x(PI / 2.0);
    let scale = Matrix::scaling(5.0, 5.0, 5.0);
    let translation = Matrix::translation(10.0, 5.0, 7.0);
    assert_eq!(translation * scale * rotation * p, point(15.0, 0.0, 7.0));
}

#[test]
fn chaining_transforms() {
    let p = point(1.0, 0.0, 1.0);
    let transform = Matrix::identity()
        .rotate_x(PI / 2.0)
        .scale(5.0, 5.0, 5.0)
        .translate(10.0, 5.0, 7.0);

    assert_eq!(transform * p, point(15.0, 0.0, 7.0));
}

#[test]
fn matrix_for_default_orientation() {
    let m = Matrix::view(
        point(0.0, 0.0, 0.0),
        point(0.0, 0.0, -1.0),
        point(0.0, 1.0, 0.0),
    );

    assert_eq!(m, Matrix::identity());
}

#[test]
fn matrix_looking_in_positive_z() {
    let m = Matrix::view(
        point(0.0, 0.0, 0.0),
        point(0.0, 0.0, 1.0),
        vector(0.0, 1.0, 0.0),
    );

    assert_eq!(m, Matrix::scaling(-1.0, 1.0, -1.0));
}
