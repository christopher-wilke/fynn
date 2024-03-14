use std::{fmt::Debug, usize};

pub struct FynnArray {
    pub matrix: Vec<Vec<f64>>,
}

impl FynnArray {
    // Forcing to use 2-d arrays
    pub fn new<T: Into<f64> + Copy, const N: usize>(arr: &[[T; N]]) -> Self {
        let mut out = vec![];
        for row in arr {
            let mut current = vec![];
            for item in row {
                let val: f64 = (*item).into();
                current.push(val);
            }
            out.push(current);
        }
        Self { matrix: out }
    }

    // Returns 2d-dimensions in (width, height)
    pub fn get_dim(&self) -> (usize, usize) {
        (*(&self.matrix[0].len()), *(&self.matrix.len()))
    }

    pub fn transpose(mut self) -> Self  {
        assert!(!self.matrix.is_empty());
        let num_cols = self.matrix.first().unwrap().len();
        let mut row_iters: Vec<_> = self.matrix.into_iter().map(Vec::into_iter).collect();
        self.matrix = (0..num_cols)
            .map(|_| row_iters.iter_mut().map(|it| it.next().unwrap()).collect())
            .collect();
        self
    }

}

impl Debug for FynnArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for val in &self.matrix {
            out.push_str(format!("{:?}\n", val).as_str());
        }
        write!(f, "{}", out)
    }
}
