use super::polynomials::Pol;
use crate::{F2D, F3D};
use std::fmt::Display;

#[derive(Debug, PartialEq)]
/// 2D Vector
pub struct Vec2<T> {
    /// x component
    pub x: T,
    /// y component
    pub y: T,
}

#[derive(Debug, PartialEq)]
/// 3D Vector
pub struct Vec3<T> {
    /// x component
    pub x: T,
    /// y component
    pub y: T,
    /// z component
    pub z: T,
}

#[derive(Debug, PartialEq)]
/// Matrix
pub struct Matrix<T> {
    mat: Vec<T>,
    n_col: usize,
    n_row: usize,
}

impl<T> Matrix<T> {
    /// Creates new matrix from vector
    pub fn new(mat: Vec<T>, n_row: usize, n_col: usize) -> Self {
        Self { mat, n_row, n_col }
    }

    /// Get the element from the i-th row and j-th column (starts from 1)
    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.mat[(row - 1) * self.n_row + col - 1]
    }
}

impl<T: std::ops::Add<Output = T> + Clone> Matrix<T> {
    /// Calculate the trace of the matrix
    pub fn trace(&self) -> T {
        let mut result = self.get(1, 1).clone();

        for i in 2..=self.n_row {
            result = result + (*self.get(i, i)).clone();
        }

        result
    }
}

impl<T: PartialEq> Matrix<T> {
    /// Check if the matrix is symmetric
    pub fn is_symmetric(&self) -> bool {
        for i in 1..=self.n_row {
            for j in (i + 1)..=self.n_col {
                if self.get(i, j) != self.get(j, i) {
                    return false;
                }
            }
        }

        true
    }
}

impl Matrix<f64> {
    /// Calculate the characteristic polynomial
    pub fn pol(&self) -> Pol {
        if self.n_col != self.n_row {
            panic!("No pol in non-square matrix");
        }

        let mut mat = Vec::with_capacity(self.mat.len());
        let mut next_diagonal = 0;

        for (i, el) in self.mat.iter().enumerate() {
            if i == next_diagonal {
                mat.push(Pol::new(vec![*el, -1.]));
                next_diagonal += 1 + self.n_col;
            } else {
                mat.push(Pol::new(vec![*el]));
            }
        }

        let mat_minus_identity = Matrix {
            mat,
            n_row: self.n_row,
            n_col: self.n_col,
        };

        mat_minus_identity.determinant()
    }
}

impl Matrix<Pol> {
    /// Computes determinant (2x2 and 3x3)
    pub fn determinant(&self) -> Pol {
        if self.n_row != self.n_col {
            panic!("Cant' calculate determinant of non-square matrix")
        }

        if self.n_row == 2 {
            (self.mat[0].clone() * self.mat[3].clone())
                - (self.mat[1].clone() * self.mat[2].clone())
        } else if self.n_row == 3 {
            self.get(1, 1) * self.get(2, 2) * self.get(3, 3)
                + self.get(1, 2) * self.get(2, 3) * self.get(3, 1)
                + self.get(1, 3) * self.get(2, 1) * self.get(3, 2)
                - self.get(3, 1) * self.get(2, 2) * self.get(1, 3)
                - self.get(3, 2) * self.get(2, 3) * self.get(1, 1)
                - self.get(3, 3) * self.get(2, 1) * self.get(1, 2)
        } else {
            panic!("Matrix size not supported")
        }
    }
}

impl Matrix<F2D> {
    /// Eval
    pub fn eval(&self, x: f64, y: f64) -> Matrix<f64> {
        Matrix {
            mat: self.mat.iter().map(|func| func.eval(x, y)).collect(),
            n_col: self.n_col,
            n_row: self.n_row,
        }
    }
}
impl Matrix<F3D> {
    /// Eval
    pub fn eval(&self, x: f64, y: f64, z: f64) -> Matrix<f64> {
        Matrix {
            mat: self.mat.iter().map(|func| func.eval(x, y, z)).collect(),
            n_col: self.n_col,
            n_row: self.n_row,
        }
    }
}

impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for (i, el) in self.mat.iter().enumerate() {
            if i % self.n_col == 0 && i != 0 {
                result += "|\n";
            }

            result += &format!("|{:^width$}", el.to_string(), width = 20);
        }

        write!(f, "{}|", result)
    }
}

impl<T: Display> Display for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
