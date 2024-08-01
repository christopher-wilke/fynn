use crate::{FynnArray, MathHelpers, FynnBehavior};
use super::Activation;

pub struct ActivationSoftmax;

impl Activation for ActivationSoftmax {
    fn forward(input: &FynnArray) -> FynnArray {
        let max = MathHelpers::max(&input);
        let v =  input - &max; 
        let exp_values = MathHelpers::exp(&v);
        let sum = MathHelpers::sum(&exp_values).to_fynn_array();
        exp_values / sum
    }
}
