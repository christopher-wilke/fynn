use std::{fmt::Debug, usize};

pub struct FynnArray {
    inner: Vec<Vec<f64>>,
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
        Self { inner: out }
    }

    pub fn transpose(&mut self) {
        if self.inner.len() > 0 {
            (0..self.inner[0].len())
                .map(|i| self.inner.iter().map(|inn|inn[i].clone()).collect::Vec<f64>())
                .collect();
        }
    }
}

impl Debug for FynnArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for val in &self.inner {
            out.push_str(format!("{:?}\n", val).as_str());
        }
        write!(f, "{}", out)
    }
}
