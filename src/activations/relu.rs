use super::Activation;
use crate::{FynnArray, FynnBehavior};

pub struct ActivationReLU;

impl ActivationReLU {
    // Py syntax is actually better here
    // E.g., dinputs[inputs <= 0] = 0
    pub fn bwd(dvalues: &FynnArray) -> FynnArray {
        let d: Vec<Vec<f64>> = dvalues.matrix
            .iter()
            .map(|row| {
                row.into_iter()
                    .map(|x| if x < &0. {0.} else {*x})
                    .collect()
            })
            .collect();

        d.to_fynn_array()
    }
}

impl Activation for ActivationReLU {
    fn forward(input: &FynnArray) -> FynnArray {
        input
            .matrix
            .iter()
            .map(|a| {
                a.into_iter()
                    .map(|b| if *b < 0. { 0. } else { *b })
                    .collect()
            })
            .collect::<Vec<Vec<f64>>>()
            .to_fynn_array()
    }
}
