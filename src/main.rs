mod fynn_array;
mod importer;
mod layer_dense;
mod activations;
mod loss;
mod math_helpers;

use activations::{Activation, softmax::*, relu::*};
use fynn_array::*;
use layer_dense::LayerDense;
use importer::*;
use math_helpers::*;

use crate::loss::{categorical_crossentropy::LossCategoricalCrossentropy, Loss};

static INPUT: &str = "py/out.txt";

pub fn main() {
    env_logger::init();

    let (input, y_true) = Importer::from_files("py/out.txt", "py/out_Y.txt");
     
    // Sample Layer 1    
    let dense1 = LayerDense::new(2, 3);
    let out_dense_1 = dense1.fwd(&input);    
    let activation1 = ActivationReLU::forward(out_dense_1);

    // Sample Layer 2
    let dense2 = LayerDense::new(3, 3);
    let out_dense_2 = dense2.fwd(&activation1);
    let activation2 = ActivationSoftmax::forward(out_dense_2);
    
    let loss = LossCategoricalCrossentropy::calculate(activation2, y_true);
    log::debug!("CategoricalCrossEntropy Loss: {}", loss);
   
    // let content = Importer::from(INPUT);
    // let input = content
    //     .get_values()
    //     .iter()
    //     .take(10)
    //     .cloned()
    //     .collect::<Vec<Vec<f64>>>()
    //     .to_fynn_array();

    // let n_inputs = input.matrix.first().unwrap().len();
    // let n_neurons: usize = 3;

    // let layer_dense = LayerDense::new(n_inputs, n_neurons);
    // let output = layer_dense.fwd(&input);
    // let activation = ActivationRelu::fwd(output);
    
    // log::debug!("output={activation:?}");
}
