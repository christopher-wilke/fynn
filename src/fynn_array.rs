use std::{fmt::Debug, usize};

pub struct FynnArray {
    pub matrix: Vec<Vec<f64>>,
}

pub trait FynnBehavior {
    fn to_fynn_array(self) -> FynnArray;
}

impl FynnBehavior for Vec<Vec<f64>> {
    fn to_fynn_array(self) -> FynnArray {
        FynnArray { matrix: Vec::from(self) }
    }
}

// 2-d
impl<T: Into<f64> + Copy, const N: usize> FynnBehavior for &[[T; N]] {
    fn to_fynn_array(self) -> FynnArray {
        let mut out = vec![];
        for row in self {
            let mut current = vec![];
            for item in row {
                let val: f64 = (*item).into();
                current.push(val);
            }
            out.push(current);
        }
        FynnArray { matrix: out }
    }
}

impl FynnArray {

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

// Overlading for `FynnArray`
impl std::ops::Add<&[f64]> for FynnArray {
    type Output = FynnArray;

    fn add(self, rhs: &[f64]) -> Self::Output {
        let mut matrix = vec![];
        for row in self.matrix {
            let mut new_row = vec![];
            for (idx, val) in row.iter().enumerate() {
                let value: f64 = format!("{:.5}", val + rhs[idx]).parse().unwrap();
                new_row.push(value);
            }
            matrix.push(new_row);
        }
        
        FynnArray {
            matrix
        }
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
