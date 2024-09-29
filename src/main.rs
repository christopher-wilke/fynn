mod fynn_array;
mod fynn_bias;
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

pub fn main() {
    env_logger::init();

    let (input, y_true) = Importer::from_files("py/out.txt", "py/out_Y.txt");

    // Testing Add Functionality
    let input = vec![
        [1., 2., 3., 2.5]
    ].to_fynn_array();
    let weight = vec![
        [0.2, 0.8, -0.5, 1.],
        [0.5, -0.91, 0.26, -0.5],
        [-0.26, -0.27, 0.17, 0.87],
        [0.3, 0.1, -0.41, 0.8]
    ].to_fynn_array();

    let res = MathHelpers::dot(&input, &weight);
    log::info!("res={res:?}");
    
    // Layer 1    
    // let mut dense1 = LayerDense::new(2, 3);
    // let out_dense_1 = dense1.fwd(&input);
    // log::info!("{out_dense_1:?}");
    // log::info!("{out_dense_1:?}");
    // let activation1 = ActivationReLU::forward(&out_dense_1);

    // // Layer 2
    // let mut dense2 = LayerDense::new(3, 3);
    // let out_dense_2 = dense2.fwd(&activation1);
    // let activation2 = ActivationSoftmax::forward(&out_dense_2);
    
    // // Helper Vars
    // let mut lowest_loss = 9999999.;
    // let mut best_dense1_weights = dense1.weights.clone();
    // let mut best_dense1_biases = dense1.biases.clone();
    // let mut best_dense2_weights = dense2.weights.clone();
    // let mut best_dense2_biases = dense2.biases.clone();

    // let fa_1 = dense1.fwd(&input);
    // log::info!("fa_1={:?}", fa_1);
    // let activation1 = ActivationReLU::forward(&fa_1);
    // let fa_2 = dense2.fwd(&activation1);
    // let mut activation2 = ActivationSoftmax::forward(&fa_2);

    // let loss = LossCategoricalCrossentropy::calculate(&mut activation2, y_true.clone());
    
    // for i in 0..100 {
    //     dense1.weights += 0.5*FynnArray::randn(2, 3);
    //     dense1.biases += MathHelpers::rand_biases(3);
    //     dense2.weights += 0.05*FynnArray::randn(3, 3);
    //     dense2.biases += MathHelpers::rand_biases(3);

    //     let fa_1 = dense1.fwd(&input);        
    //     let activation1 = ActivationReLU::forward(&fa_1);
    //     let fa_2 = dense2.fwd(&activation1);
    //     let mut activation2 = ActivationSoftmax::forward(&fa_2);

    //     let loss = LossCategoricalCrossentropy::calculate(&mut activation2, y_true.clone());
    //     let predictions = MathHelpers::argmax(activation2.clone()); 
    //     let accuracy = MathHelpers::mean(&predictions, &y_true);
    //     log::info!("New set of weights found! Iteration: {}, loss: {}, acc: {}", i, loss, accuracy);

    //     // if loss < lowest_loss {
    //     //     log::info!("New set of weights found! Iteration: {}, loss: {}, acc: {}", i, loss, accuracy);
    //     //     best_dense1_weights = dense1.weights.clone();
    //     //     best_dense1_biases = dense1.biases.clone();
    //     //     best_dense2_weights = dense2.weights.clone();
    //     //     best_dense2_biases = dense2.biases.clone();
    //     //     lowest_loss = loss;
    //     // }
    //     // else {
    //     //     dense1.weights = best_dense1_weights.clone();
    //     //     dense1.biases = best_dense1_biases.clone();
    //     //     dense2.weights = best_dense2_weights.clone();
    //     //     dense2.biases = best_dense2_biases.clone();
    //     // }
    // }
   
}
