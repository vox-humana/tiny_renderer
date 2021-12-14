use crate::point::Vec3;
use std::ops::Mul;

pub(crate) struct ViewPort {
    pub(crate) x: u16,
    pub(crate) y: u16,
    pub(crate) width: u16,
    pub(crate) height: u16,
}

#[derive(Clone)]
pub(crate) struct Matrix {
    pub(crate) m: Vec<Vec<f32>>,
}

impl Matrix {
    pub(crate) fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            m: vec![vec![0.0; cols]; rows],
        }
    }

    pub(crate) fn new_identity(l: usize) -> Self {
        let mut m = Matrix::new(l, l);
        for i in 0..l {
            m.m[i][i] = 1.0;
        }
        return m;
    }

    fn cols(&self) -> usize {
        return match self.m.first() {
            None => 0,
            Some(row) => row.len(),
        };
    }

    fn rows(&self) -> usize {
        self.m.len()
    }

    pub(crate) fn to_vector(&self) -> Vec3<f32> {
        assert_eq!(self.cols(), 1);
        assert_eq!(self.rows(), 4);
        Vec3 {
            x: self.m[0][0] / self.m[3][0],
            y: self.m[1][0] / self.m[3][0],
            z: self.m[2][0] / self.m[3][0],
        }
    }
}

impl ViewPort {
    pub(crate) fn to_matrix(&self) -> Matrix {
        let depth: f32 = 255.0;

        let mut m = Matrix::new_identity(4);
        m.m[0][3] = self.x as f32 + self.width as f32 / 2.0;
        m.m[1][3] = self.y as f32 + self.height as f32 / 2.0;
        m.m[2][3] = depth / 2.0;

        m.m[0][0] = self.width as f32 / 2.0;
        m.m[1][1] = self.height as f32 / 2.0;
        m.m[2][2] = depth / 2.0;
        return m;
    }
}

impl Vec3<f32> {
    pub(crate) fn to_matrix(&self) -> Matrix {
        let mut m = Matrix::new(4, 1);
        m.m[0][0] = self.x;
        m.m[1][0] = self.y;
        m.m[2][0] = self.z;
        m.m[3][0] = 1.0;
        return m;
    }
}

impl Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.cols(), rhs.rows());
        let mut r = Matrix::new(self.rows(), rhs.cols());
        for i in 0..self.rows() {
            for j in 0..rhs.cols() {
                for k in 0..self.cols() {
                    r.m[i][j] += self.m[i][k] * rhs.m[k][j];
                }
            }
        }
        return r;
    }
}
