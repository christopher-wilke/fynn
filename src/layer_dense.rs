use crate::{helpers, FynnArray};

#[derive(Debug)]
pub struct LayerDense {
    pub weights: FynnArray,
    pub biases: [f64; 3],
}

impl LayerDense {
    pub fn new(n_inputs: usize, n_neurons: usize) -> Self {
        let weights = 0.01 * FynnArray::randn(n_inputs, n_neurons);
        let biases = [0., 0., 0.];

        Self { weights, biases }
    }

    pub fn fwd(&self, inputs: &FynnArray) -> FynnArray {
        log::trace!("input: \n{:?}\n weight: \n{:?}\n bias: \n{:?}", inputs, &self.weights, &self.biases);
        helpers::dot(&inputs, &self.weights) + &self.biases
    }
}
