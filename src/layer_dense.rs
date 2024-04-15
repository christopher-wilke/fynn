use crate::{helpers, FynnArray};
use crate::FynnBehavior;

#[derive(Debug)]
pub struct LayerDense {
    pub weights: FynnArray,
    pub biases: [f64; 3]
}

impl LayerDense {
    pub fn new() -> Self {
        let weights = 0.01*FynnArray::randn(2,3);
        let biases = [0., 0., 0.];
        
        Self { weights, biases }
    }

    pub fn fwd(self, inputs: &FynnArray) -> FynnArray {
        helpers::dot(&inputs, &self.weights) + &self.biases
    }
}
