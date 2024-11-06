use std::{borrow::BorrowMut, fmt::Debug};

use crate::{fynn_bias::FynnBias, FynnArray, MathHelpers};

#[derive(Debug)]
pub struct LayerDense {
    pub weights: FynnArray,
    pub biases: FynnBias,
}

impl LayerDense {
    pub fn new(n_inputs: usize, n_neurons: usize) -> Self {
        let weights = 0.01 * FynnArray::randn(n_inputs, n_neurons);
        let biases = FynnBias {
            val: vec![0., 0., 0.],
        };

        Self { weights, biases }
    }

    pub fn fwd(&self, inputs: &FynnArray) -> FynnArray {
        MathHelpers::dot(&inputs, &(self.weights)) + &self.biases
    }

    pub fn bwd(&self, drelu: &FynnArray) {
        log::info!("running backward calculation");
        let dweights = MathHelpers::dot(&self.weights, drelu);
        let dinputs = MathHelpers::dot(&drelu, &self.weights.clone().transpose());
        let dbiases = FynnBias { val: MathHelpers::sum(&drelu.matrix) };
        log::info!("dweights: {:?}", dweights);
        log::info!("dinputs: {:?}", dinputs);
        log::info!("dbiases: {:?}", dbiases);
    }
}
