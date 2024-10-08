#[derive(Debug, Clone)]
pub struct FynnBias {
    pub val: Vec<f64>,
}

impl std::ops::AddAssign for FynnBias {
    fn add_assign(&mut self, rhs: Self) {
        for (a, b) in self.val.iter_mut().zip(rhs.val.iter()) {
            *a += *b;
        }
    }
}
