#[derive(Debug, PartialEq)]
pub struct Matrix<T> {
    pub mat: Vec<T>,
    pub n_col: usize,
    pub n_row: usize,
}

impl<T: Copy> Matrix<T> {
    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.mat[(row - 1) * self.n_row + col - 1]
    }
}


