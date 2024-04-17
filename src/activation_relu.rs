use crate::FynnArray;

pub struct ActivationRelu;

impl ActivationRelu {
    pub fn fwd(mut inputs: FynnArray) -> Vec<Vec<f64>> {
        log::trace!("Transforming via Relu_activation: {inputs:?}");
        
        inputs.matrix
            .into_iter()
            .map(|a| {
                a.into_iter()
                    .map(|b| if b < 0. {0.} else {b})
                    .collect()
            }).collect()
    }
}
