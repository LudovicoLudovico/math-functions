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

    pub(crate) fn get(&self, row: usize, col: usize) -> &T {
        &self.mat[(row - 1) * self.n_row + col - 1]
    }

    pub(crate) fn get_row(&self, row: usize) -> &[T] {
        &self.mat[(row - 1) * self.n_col..(row - 1 + self.n_col)]
    }

    pub(crate) fn get_rows(&self, from: usize, to: usize) -> &[T] {
        &self.mat[(from - 1) * self.n_col..to * self.n_col]
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

macro_rules! impl_determinant{
    (for $($t:ty),+) => {
        $(impl Matrix<$t> {
            /// Calculate determinant
            pub fn determinant(&self) -> $t {
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
                    let mut result: $t = Default::default();
                    for (idx, el) in self.get_row(1).iter().enumerate() {
                        let mut mat = self.get_rows(2, self.n_row).to_vec();
                        let mut index = 0;

                        mat.retain(|_| {
                            index += 1;
                            if index <= 2 {
                                index - 1 != idx
                            } else if index  > idx {
                                (index - 1 - idx) % self.n_col != 0
                            } else {
                                true
                            }
                        });

                        let sub_mat = Matrix::new(mat, self.n_col - 1, self.n_col - 1);
                        result += (-1_f64).powf((idx + 2) as f64) * el * sub_mat.determinant();
                    }

                    result
                }
            }
        })*
    }
}

impl_determinant!(for f64, Pol);

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

#[test]
fn test_mat() {
    let mat = Matrix::new(vec![1., 2., 3., 4., 5., 6., 7., 8., 9.], 3, 3);
    assert_eq!(3., *mat.get(1, 3));
    assert_eq!(9., *mat.get(3, 3));
    assert!(!mat.is_symmetric());
    assert_eq!(Pol::new(vec![0., 18., 15., -1.]), mat.pol());

    let mat = Matrix::new(vec![1, 2, 2, 1], 2, 2);
    assert!(mat.is_symmetric());
    assert_eq!(mat.trace(), 2);

    let mat = Matrix::new(
        vec![
            2., 3., -1., 0., 0., 1., 1., 1., 2., 3., 1., -1., 4., 1., 2., 0.,
        ],
        4,
        4,
    );
    assert_eq!(mat.determinant(), 38.);
    println!("{}", mat.determinant())
}
