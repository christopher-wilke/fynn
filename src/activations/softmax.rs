use crate::{FynnArray, MathHelpers, FynnBehavior};
use super::Activation;

pub struct Softmax;

impl Activation for Softmax {
    fn forward(&self, input: FynnArray) -> FynnArray {
        let exp_values = MathHelpers::exp(&input - &MathHelpers::max(&input));
        let sum = MathHelpers::sum(&exp_values).to_fynn_array();
        exp_values / sum    
    }
}
