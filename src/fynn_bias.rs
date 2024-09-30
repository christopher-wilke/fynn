#[derive(Debug, Clone)]
pub struct FynnBias {
    pub val: Vec<f64>,
}

impl std::ops::AddAssign for FynnBias {
    fn add_assign(&mut self, rhs: Self) {
        self.val = self
            .val
            .iter()
            .zip(rhs.val.iter())
            .map(|(x, y)| x + y)
            .collect();
    }
}

// impl std::ops::AddAssign for FynnArray {
//     fn add_assign(&mut self, rhs: Self) {
//         self.matrix = self.matrix.iter()
//             .zip(rhs.matrix.iter())
//             .map(|(v1, v2)| {
//                 v1.iter()
//                     .zip(v2.iter())
//                     .map(|(k1, k2)| k1 + k2)
//                     .collect()
//             })
//             .collect();
//     }
// }
