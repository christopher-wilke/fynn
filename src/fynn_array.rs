use std::{fmt::Debug, usize};

use rand::prelude::*;
use rand_distr::StandardNormal;

#[derive(Clone)]
pub struct FynnArray {
    pub matrix: Vec<Vec<f64>>,
}

pub trait FynnBehavior {
    fn to_fynn_array(self) -> FynnArray;
}

impl FynnBehavior for Vec<Vec<f64>> {
    fn to_fynn_array(self) -> FynnArray {
        FynnArray {
            matrix: Vec::from(self),
        }
    }
}

impl FynnBehavior for Vec<f64> {
    fn to_fynn_array(self) -> FynnArray {
        let mut out = vec![];
        for v in self {
            out.push(vec![v]);
        }
        FynnArray { matrix: out }
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

    pub fn new() -> Self {
        Self {
            matrix: vec![]
        }
    }
    
    pub fn zeros(h: usize, w: usize) -> Self {
        let mut matrix = vec![];
        for _ in 0..h {
            let mut row = vec![];
            for _ in 0..w {
                row.push(0.);
            }
            matrix.push(row)
        }
        Self { matrix }
    }

    pub fn randn(h: usize, w: usize) -> Self {
        let mut matrix: Vec<Vec<f64>> = vec![];
        for _ in 0..h {
            let mut row = vec![];
            for _ in 0..w {
                let val: f64 = thread_rng().sample(StandardNormal);
                row.push(val);
            }
            matrix.push(row);
        }
        Self { matrix }
    }

    /// Returns 2d-dimensions in (width, height)
    pub fn get_dim(&self) -> (usize, usize) {
        (*(&self.matrix[0].len()), *(&self.matrix.len()))
    }

    pub fn transpose(mut self) -> Self {
        assert!(!self.matrix.is_empty());
        let num_cols = self.matrix.first().unwrap().len();
        let mut row_iters: Vec<_> = self.matrix.into_iter().map(Vec::into_iter).collect();
        self.matrix = (0..num_cols)
            .map(|_| row_iters.iter_mut().map(|it| it.next().unwrap()).collect())
            .collect();
        self
    }
}

// Math Overloads for `FynnArray`
impl std::ops::Mul<FynnArray> for f64 {
    type Output = FynnArray;

    fn mul(self, rhs: FynnArray) -> Self::Output {
        rhs.matrix
            .iter()
            .map(|i| i.iter().map(|&val| 0.01 * val).collect())
            .collect::<Vec<Vec<f64>>>()
            .to_fynn_array()
    }
}

impl std::ops::Sub<&Vec<Vec<f64>>> for &FynnArray {
    type Output = FynnArray;

    fn sub(self, rhs: &Vec<Vec<f64>>) -> Self::Output {
        let mut matrix = Vec::new();

        for (i, v) in self.matrix
            .iter()
            .enumerate() 
        {
            let mut new_row = Vec::new();
            let su = rhs.get(i).unwrap().get(0).unwrap();
            for val in v {
                let m = val - su;
                new_row.push(m);
            }
            matrix.push(new_row);
        }

        FynnArray { matrix }
    }
}

impl std::ops::Div<&FynnArray> for &Vec<Vec<f64>> {
    type Output = FynnArray;

    fn div(self, rhs: &FynnArray) -> Self::Output {
        let mut fa = FynnArray::new();
    
        for (val, sum) in self
            .iter()
            .zip(rhs.matrix.iter()) 
        {
            let mut arr = vec![];
            for v in val {
                arr.push(*v/sum.get(0).unwrap());
            }
            fa.matrix.push(arr);
        }
    
        fa
    }
}

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

        FynnArray { matrix }
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
