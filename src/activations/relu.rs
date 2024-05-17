use super::Activation;
use crate::{FynnArray, FynnBehavior};

pub struct ActivationReLU;

impl Activation for ActivationReLU {
    fn forward(input: FynnArray) -> FynnArray {
        input.matrix
            .into_iter()
            .map(|a| {
                a.into_iter()
                    .map(|b| if b < 0. {0.} else {b})
                    .collect()
            })
            .collect::<Vec<Vec<f64>>>()
            .to_fynn_array()
    }
}
