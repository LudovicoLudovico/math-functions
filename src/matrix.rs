use std::fmt::Display;

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

impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        let length = 20;

        for (i, el) in self.mat.iter().enumerate() {
            if i % self.n_col == 0 {
                result += "|\n";
            }

            let el = el.to_string();
            result += "|";
            let new = format!("{:^width$}", el, width = length);
            result += &new;
        }
        result += "|";

        write!(f, "{}", result)
    }
}