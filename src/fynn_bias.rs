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

// Math Overloads for `FynnBias`
impl std::ops::Mul<FynnBias> for f64 {
    type Output = FynnBias;

    fn mul(self, rhs: FynnBias) -> Self::Output {
        let val = rhs.val
            .iter()
            .map(|i| i * self)
            .collect::<Vec<f64>>();

        FynnBias { val }
    }
}
