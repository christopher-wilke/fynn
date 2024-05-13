use std::fmt::Debug;

use crate::{helpers, FynnArray, FynnBehavior};

#[derive(Debug)]
pub struct LayerDense {
    pub weights: FynnArray,
    pub biases: [f64; 3],
}

impl LayerDense {

    // this should get probably outsourced to a unit test
    pub fn demo_weights() -> Self {
        let weights = vec![
            vec![0.01764052,  0.00400157,  0.00978738],
            vec![0.02240893,  0.01867558, -0.00977278]        
        ].to_fynn_array();
        let biases = [0., 0., 0.];

        Self { weights, biases }
    }
    
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
